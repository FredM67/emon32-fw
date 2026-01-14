CC = cc
CFLAGS = -Wall -Wpedantic -g
INCLUDES = ../src/ -I./
DEFINES = -DHOSTED
LIBS = -lm

cm: OBJS = test_cm.c ../src/emon_CM.c ../src/board_def.c
eeprom: OBJS = test_eeprom.c ../src/eeprom.c
asm_math: OBJS = test_asm_math.c

.PHONY: clean all

all: cm eeprom asm_math

eeprom:
	${CC} ${CFLAGS} $(DEFINES) -I$(INCLUDES) -o $@.test $(OBJS) ${LIBS}
cm:
	${CC} ${CFLAGS} $(DEFINES) -I$(INCLUDES) -o $@.test $(OBJS) ${LIBS}
asm_math:
	${CC} ${CFLAGS} $(DEFINES) -I$(INCLUDES) -o $@.test $(OBJS) ${LIBS}
clean:
	rm *.test
	rm -rf *dSYM/
