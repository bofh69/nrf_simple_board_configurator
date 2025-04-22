PROGRAM=sbc

$(PROGRAM): simple_board_configurator.c
	gcc -o $@ $< -lusb-1.0

clean:
	-$(RM) $(PROGRAM)
