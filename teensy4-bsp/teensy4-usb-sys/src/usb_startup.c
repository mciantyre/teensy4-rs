#include "debug/printf.h"
#include "imxrt.h"

FLASHMEM void usb_pll_start(void)
{
	while (1) {
		uint32_t n = CCM_ANALOG_PLL_USB1; // pg 759
		printf("CCM_ANALOG_PLL_USB1=%08lX\n", n);
		if (n & CCM_ANALOG_PLL_USB1_DIV_SELECT) {
			printf("  ERROR, 528 MHz mode!\n"); // never supposed to use this mode!
			CCM_ANALOG_PLL_USB1_CLR = 0xC000;			// bypass 24 MHz
			CCM_ANALOG_PLL_USB1_SET = CCM_ANALOG_PLL_USB1_BYPASS;	// bypass
			CCM_ANALOG_PLL_USB1_CLR = CCM_ANALOG_PLL_USB1_POWER |	// power down
				CCM_ANALOG_PLL_USB1_DIV_SELECT |		// use 480 MHz
				CCM_ANALOG_PLL_USB1_ENABLE |			// disable
				CCM_ANALOG_PLL_USB1_EN_USB_CLKS;		// disable usb
			continue;
		}
		if (!(n & CCM_ANALOG_PLL_USB1_ENABLE)) {
			printf("  enable PLL\n");
			// TODO: should this be done so early, or later??
			CCM_ANALOG_PLL_USB1_SET = CCM_ANALOG_PLL_USB1_ENABLE;
			continue;
		}
		if (!(n & CCM_ANALOG_PLL_USB1_POWER)) {
			printf("  power up PLL\n");
			CCM_ANALOG_PLL_USB1_SET = CCM_ANALOG_PLL_USB1_POWER;
			continue;
		}
		if (!(n & CCM_ANALOG_PLL_USB1_LOCK)) {
			printf("  wait for lock\n");
			continue;
		}
		if (n & CCM_ANALOG_PLL_USB1_BYPASS) {
			printf("  turn off bypass\n");
			CCM_ANALOG_PLL_USB1_CLR = CCM_ANALOG_PLL_USB1_BYPASS;
			continue;
		}
		if (!(n & CCM_ANALOG_PLL_USB1_EN_USB_CLKS)) {
			printf("  enable USB clocks\n");
			CCM_ANALOG_PLL_USB1_SET = CCM_ANALOG_PLL_USB1_EN_USB_CLKS;
			continue;
		}
		return; // everything is as it should be  :-)
	}
}