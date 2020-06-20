CC=arm-none-eabi-gcc
AR=arm-none-eabi-gcc-ar
CFLAGS= -Wall -MMD -g -O2 -ffunction-sections -fdata-sections -mcpu=cortex-m7 -mthumb -mfloat-abi=hard -mfpu=fpv5-d16 -std=gnu11

SRCS=$(wildcard *.c)
OBJS=$(SRCS:.c=.o)

libt4boot.a: $(OBJS)
	$(AR) -rcs $@ $(OBJS)

.PHONY: clean
clean:
	rm -f libt4boot.a *.o *.d