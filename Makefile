PROGRAM := sbc
SRC = simple_board_configurator.c

$(PROGRAM): $(SRC)
	gcc -o $@ $< -lusb-1.0

clean:
	-$(RM) $(PROGRAM)

fmt:
	-uncrustify-0.75.1 -q --no-backup -c .uncrustify.cfg --replace $(SRC)

.PHONY: clean fmt
