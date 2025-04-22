#include <libusb-1.0/libusb.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#define VENDOR_ID  0x1366
#define PRODUCT_ID 0x1068
#define TIMEOUT    1000  // Timeout in milliseconds
#define INTERFACE_ID 5

static void usage(const char *progname)
{
    fprintf(stderr, "Usage: %s on|off\n", progname);
}

int main(int argc, const char *argv[]) {

    if (argc != 2) {
        usage(argv[0]);
        return 1;
    }

    bool turn_on = !strcmp(argv[1], "on");
    bool turn_off = !strcmp(argv[1], "off");
    if (!(turn_on || turn_off)) {
        usage(argv[0]);
        return 1;
    }

    libusb_context *ctx = NULL;
    libusb_device_handle *dev_handle = NULL;
    int rc;

    rc = libusb_init(&ctx);
    if (rc < 0) {
        fprintf(stderr, "Failed to initialize libusb: %s\n", libusb_error_name(rc));
        return 1;
    }

    dev_handle = libusb_open_device_with_vid_pid(ctx, VENDOR_ID, PRODUCT_ID);
    if (!dev_handle) {
        fprintf(stderr, "Failed to open device\n");
        libusb_exit(ctx);
        return 1;
    }

    if (libusb_kernel_driver_active(dev_handle, INTERFACE_ID)) {
        int detach_res = libusb_detach_kernel_driver(dev_handle, INTERFACE_ID);
        if (detach_res < 0) {
            fprintf(stderr, "Failed to detach kernel driver: %s\n", libusb_error_name(detach_res));
            libusb_close(dev_handle);
            libusb_exit(ctx);
            return -1;
        }
    }


    rc = libusb_claim_interface(dev_handle, INTERFACE_ID);
    if (rc < 0) {
        fprintf(stderr, "Failed to claim interface: %s\n", libusb_error_name(rc));
        libusb_close(dev_handle);
        libusb_exit(ctx);
        return 1;
    }

    // Data to send, extracted from the usbmon log
    unsigned char data_on[64] = {
        0x02, 0x00, 0x00, 0x15, 0x00, 0x40, 0x00, 0x00,
        0x82, 0x8c, 0x06, 0xf5, 0x14, 0xf5, 0x16, 0xf5,
        0x17, 0xf5, 0x18, 0x2a, 0xf5, 0x18, 0x2d, 0xf5,
        0x82, 0x01, 0x19, 0x0c, 0xe4, 0x00, 0x00, 0x00
    };
    unsigned char data_off[64] = {
        0x02, 0x00, 0x00, 0x15, 0x00, 0x40, 0x00, 0x00,
        0x82, 0x8c, 0x06, 0xf4, 0x14, 0xf4, 0x16, 0xf4,
        0x17, 0xf4, 0x18, 0x2a, 0xf4, 0x18, 0x2d, 0xf4,
        0x82, 0x01, 0x19, 0x0c, 0xe4, 0x00, 0x00, 0x00,
    };

    unsigned char *data = data_on;
    if (turn_off) {
        data = data_off;
    }

    // Send data to endpoint 0x04 (OUT)
    int transferred;
    rc = libusb_interrupt_transfer(dev_handle, 0x04, data, sizeof(data_on), &transferred, TIMEOUT);
    if (rc < 0) {
        fprintf(stderr, "Failed to send data: %s\n", libusb_error_name(rc));
        libusb_release_interface(dev_handle, INTERFACE_ID);
        libusb_close(dev_handle);
        libusb_exit(ctx);
        return 1;
    }
    printf("Data sent successfully, %d bytes transferred\n", transferred);

    // Read response from endpoint 0x6 (IN)
    unsigned char response[64];
    rc = libusb_bulk_transfer(dev_handle, 0x80|0x06, response, sizeof(response), &transferred, TIMEOUT);
    if (rc < 0) {
        fprintf(stderr, "Failed to read response: %s\n", libusb_error_name(rc));
    } else {
        printf("Response received (%d bytes):\n", transferred);
        for (int i = 0; i < transferred; i++) {
            printf("%02x ", response[i]);
        }
        printf("\n");
    }

    libusb_release_interface(dev_handle, INTERFACE_ID);
    libusb_close(dev_handle);
    libusb_exit(ctx);
    return 0;
}
