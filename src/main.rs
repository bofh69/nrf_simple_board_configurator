use rusb::{Context, DeviceHandle, UsbContext};
use std::env;
use std::time::Duration;

const VENDOR_ID: u16 = 0x1366;
const PRODUCT_ID: u16 = 0x1068;
const TIMEOUT: Duration = Duration::from_millis(1000);
const INTERFACE_ID: u8 = 5;

fn usage(progname: &str) {
    eprintln!("Usage: {} on|off", progname);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage(&args[0]);
        std::process::exit(1);
    }

    let turn_on = args[1] == "on";
    let turn_off = args[1] == "off";
    if !(turn_on || turn_off) {
        usage(&args[0]);
        std::process::exit(1);
    }

    let context = match Context::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Failed to initialize rusb: {:?}", e);
            std::process::exit(1);
        }
    };

    let mut device_handle = match context.open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID) {
        Some(handle) => handle,
        None => {
            eprintln!("Failed to open device");
            std::process::exit(1);
        }
    };

    if let Ok(active) = device_handle.kernel_driver_active(INTERFACE_ID) {
        if active {
            if let Err(e) = device_handle.detach_kernel_driver(INTERFACE_ID) {
                eprintln!("Failed to detach kernel driver: {:?}", e);
                std::process::exit(1);
            }
        }
    }

    if let Err(e) = device_handle.claim_interface(INTERFACE_ID) {
        eprintln!("Failed to claim interface: {:?}", e);
        std::process::exit(1);
    }

    let data_on: [u8; 64] = [
        0x02, 0x00, 0x00, 0x15, 0x00, 0x40, 0x00, 0x00, 0x82, 0x8c, 0x06, 0xf5, 0x14, 0xf5, 0x16,
        0xf5, 0x17, 0xf5, 0x18, 0x2a, 0xf5, 0x18, 0x2d, 0xf5, 0x82, 0x01, 0x19, 0x0c, 0xe4, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];
    let data_off: [u8; 64] = [
        0x02, 0x00, 0x00, 0x15, 0x00, 0x40, 0x00, 0x00, 0x82, 0x8c, 0x06, 0xf4, 0x14, 0xf4, 0x16,
        0xf4, 0x17, 0xf4, 0x18, 0x2a, 0xf4, 0x18, 0x2d, 0xf4, 0x82, 0x01, 0x19, 0x0c, 0xe4, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let data = if turn_on { &data_on } else { &data_off };

    match device_handle.write_interrupt(0x04, data, TIMEOUT) {
        Ok(transferred) => {
            println!("Data sent successfully, {} bytes transferred", transferred);
        }
        Err(e) => {
            eprintln!("Failed to send data: {:?}", e);
            cleanup(&mut device_handle);
            std::process::exit(1);
        }
    }

    let mut response = [0u8; 64];
    match device_handle.read_bulk(0x80 | 0x06, &mut response, TIMEOUT) {
        Ok(transferred) => {
            println!("Response received ({} bytes):", transferred);
            for byte in &response[..transferred] {
                print!("{:02x} ", byte);
            }
            println!();
        }
        Err(e) => {
            eprintln!("Failed to read response: {:?}", e);
            cleanup(&mut device_handle);
            std::process::exit(1);
        }
    }

    cleanup(&mut device_handle);
}

fn cleanup<T: UsbContext>(device_handle: &mut DeviceHandle<T>) {
    if let Err(e) = device_handle.release_interface(INTERFACE_ID) {
        eprintln!("Failed to release interface: {:?}", e);
    }

    if let Err(e) = device_handle.attach_kernel_driver(INTERFACE_ID) {
        eprintln!("Failed to reattach kernel driver: {:?}", e);
    }
}
