CC=arm-none-eabi-gcc
AR=arm-none-eabi-gcc-ar
CFLAGS=-Wall -MMD -g -O2 -ffunction-sections -fdata-sections -mcpu=cortex-m7 -mthumb -mfloat-abi=hard -mfpu=fpv5-d16 -std=gnu11
CPPFLAGS= -DUSB_SERIAL -D__IMXRT1062__ -DFLASHMEM="__attribute__((section(\".flashmem\")))" -DPROGMEM="__attribute__((section(\".progmem\")))" -DDMAMEM="__attribute__ ((section(\".dmabuffers\"), used))"

all: libt4usb.a libt4start.a

SRCS_USB=$(wildcard usb*.c) nonstd.c
OBJS_USB=$(SRCS_USB:.c=.o)

libt4usb.a: $(OBJS_USB)
	$(AR) -rcs $@ $(OBJS_USB)

SRCS_START=start.c
OBJS_START=$(SRCS_START:.c=.o)

libt4start.a: $(OBJS_START)
	$(AR) -rcs $@ $(OBJS_START)

.PHONY: clean
clean:
	rm -f libt4start.a libt4usb.a *.o *.d