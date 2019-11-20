CC = arm-none-eabi-gcc
AR = arm-none-eabi-gcc-ar
CFLAGS = -c -Wall -MMD -g -O2 -ffunction-sections -fdata-sections -mcpu=cortex-m7 -mthumb -mfloat-abi=hard -mfpu=fpv5-d16
FILES := $(wildcard *.c)

OBJS := $(FILES:.c=.o)
all:
	$(CC) $(CFLAGS) $(FILES)
	$(AR) rcs boot.a $(OBJS)
	rm -f *.o *.d

clean:
	rm -f *.a
