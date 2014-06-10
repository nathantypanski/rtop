PROGRAM_NAME = rtop

all: $(PROGRAM_NAME)

.PHONY: clean $(PROGRAM_NAME)

$(PROGRAM_NAME): $(PROGRAM_NAME).rs
	rustc -Lncurses-rs/lib $(PROGRAM_NAME).rs

clean :
	$(RM) $(PROGRAM_NAME)

run: ${PROGRAM_NAME}
	./${PROGRAM_NAME}
