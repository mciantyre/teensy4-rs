#![doc = "Peripheral access API for MIMXRT1062 microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
extern crate vcell;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;

extern "C" {
    fn DMA0_DMA16();
    fn DMA1_DMA17();
    fn DMA2_DMA18();
    fn DMA3_DMA19();
    fn DMA4_DMA20();
    fn DMA5_DMA21();
    fn DMA6_DMA22();
    fn DMA7_DMA23();
    fn DMA8_DMA24();
    fn DMA9_DMA25();
    fn DMA10_DMA26();
    fn DMA11_DMA27();
    fn DMA12_DMA28();
    fn DMA13_DMA29();
    fn DMA14_DMA30();
    fn DMA15_DMA31();
    fn DMA_ERROR();
    fn CTI0_ERROR();
    fn CTI1_ERROR();
    fn CORE();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn LPUART5();
    fn LPUART6();
    fn LPUART7();
    fn LPUART8();
    fn LPI2C1();
    fn LPI2C2();
    fn LPI2C3();
    fn LPI2C4();
    fn LPSPI1();
    fn LPSPI2();
    fn LPSPI3();
    fn LPSPI4();
    fn CAN1();
    fn CAN2();
    fn FLEXRAM();
    fn KPP();
    fn TSC_DIG();
    fn GPR_IRQ();
    fn LCDIF();
    fn CSI();
    fn PXP();
    fn WDOG2();
    fn SNVS_HP_WRAPPER();
    fn SNVS_HP_WRAPPER_TZ();
    fn SNVS_LP_WRAPPER();
    fn CSU();
    fn DCP();
    fn DCP_VMI();
    fn TRNG();
    fn SJC();
    fn BEE();
    fn SAI1();
    fn SAI2();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SPDIF();
    fn PMU_EVENT();
    fn TEMP_LOW_HIGH();
    fn TEMP_PANIC();
    fn USB_PHY1();
    fn USB_PHY2();
    fn ADC1();
    fn ADC2();
    fn DCDC();
    fn GPIO1_INT0();
    fn GPIO1_INT1();
    fn GPIO1_INT2();
    fn GPIO1_INT3();
    fn GPIO1_INT4();
    fn GPIO1_INT5();
    fn GPIO1_INT6();
    fn GPIO1_INT7();
    fn GPIO1_COMBINED_0_15();
    fn GPIO1_COMBINED_16_31();
    fn GPIO2_COMBINED_0_15();
    fn GPIO2_COMBINED_16_31();
    fn GPIO3_COMBINED_0_15();
    fn GPIO3_COMBINED_16_31();
    fn GPIO4_COMBINED_0_15();
    fn GPIO4_COMBINED_16_31();
    fn GPIO5_COMBINED_0_15();
    fn GPIO5_COMBINED_16_31();
    fn FLEXIO1();
    fn FLEXIO2();
    fn WDOG1();
    fn RTWDOG();
    fn EWM();
    fn CCM_1();
    fn CCM_2();
    fn GPC();
    fn SRC();
    fn GPT1();
    fn GPT2();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
    fn FLEXSPI2();
    fn FLEXSPI();
    fn SEMC();
    fn USDHC1();
    fn USDHC2();
    fn USB_OTG2();
    fn USB_OTG1();
    fn ENET();
    fn ENET_1588_TIMER();
    fn XBAR1_IRQ_0_1();
    fn XBAR1_IRQ_2_3();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_ERROR_IRQ();
    fn PIT();
    fn ACMP1();
    fn ACMP2();
    fn ACMP3();
    fn ACMP4();
    fn ENC1();
    fn ENC2();
    fn ENC3();
    fn ENC4();
    fn TMR1();
    fn TMR2();
    fn TMR3();
    fn TMR4();
    fn PWM2_0();
    fn PWM2_1();
    fn PWM2_2();
    fn PWM2_3();
    fn PWM2_FAULT();
    fn PWM3_0();
    fn PWM3_1();
    fn PWM3_2();
    fn PWM3_3();
    fn PWM3_FAULT();
    fn PWM4_0();
    fn PWM4_1();
    fn PWM4_2();
    fn PWM4_3();
    fn PWM4_FAULT();
    fn ENET2();
    fn ENET2_1588_TIMER();
    fn CAN3();
    fn FLEXIO3();
    fn GPIO6_7_8_9();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 158] = [
    Vector {
        _handler: DMA0_DMA16,
    },
    Vector {
        _handler: DMA1_DMA17,
    },
    Vector {
        _handler: DMA2_DMA18,
    },
    Vector {
        _handler: DMA3_DMA19,
    },
    Vector {
        _handler: DMA4_DMA20,
    },
    Vector {
        _handler: DMA5_DMA21,
    },
    Vector {
        _handler: DMA6_DMA22,
    },
    Vector {
        _handler: DMA7_DMA23,
    },
    Vector {
        _handler: DMA8_DMA24,
    },
    Vector {
        _handler: DMA9_DMA25,
    },
    Vector {
        _handler: DMA10_DMA26,
    },
    Vector {
        _handler: DMA11_DMA27,
    },
    Vector {
        _handler: DMA12_DMA28,
    },
    Vector {
        _handler: DMA13_DMA29,
    },
    Vector {
        _handler: DMA14_DMA30,
    },
    Vector {
        _handler: DMA15_DMA31,
    },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector {
        _handler: CTI0_ERROR,
    },
    Vector {
        _handler: CTI1_ERROR,
    },
    Vector { _handler: CORE },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: LPUART5 },
    Vector { _handler: LPUART6 },
    Vector { _handler: LPUART7 },
    Vector { _handler: LPUART8 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPI2C3 },
    Vector { _handler: LPI2C4 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: LPSPI3 },
    Vector { _handler: LPSPI4 },
    Vector { _handler: CAN1 },
    Vector { _handler: CAN2 },
    Vector { _handler: FLEXRAM },
    Vector { _handler: KPP },
    Vector { _handler: TSC_DIG },
    Vector { _handler: GPR_IRQ },
    Vector { _handler: LCDIF },
    Vector { _handler: CSI },
    Vector { _handler: PXP },
    Vector { _handler: WDOG2 },
    Vector {
        _handler: SNVS_HP_WRAPPER,
    },
    Vector {
        _handler: SNVS_HP_WRAPPER_TZ,
    },
    Vector {
        _handler: SNVS_LP_WRAPPER,
    },
    Vector { _handler: CSU },
    Vector { _handler: DCP },
    Vector { _handler: DCP_VMI },
    Vector { _reserved: 0 },
    Vector { _handler: TRNG },
    Vector { _handler: SJC },
    Vector { _handler: BEE },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SPDIF },
    Vector {
        _handler: PMU_EVENT,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: TEMP_LOW_HIGH,
    },
    Vector {
        _handler: TEMP_PANIC,
    },
    Vector { _handler: USB_PHY1 },
    Vector { _handler: USB_PHY2 },
    Vector { _handler: ADC1 },
    Vector { _handler: ADC2 },
    Vector { _handler: DCDC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: GPIO1_INT0,
    },
    Vector {
        _handler: GPIO1_INT1,
    },
    Vector {
        _handler: GPIO1_INT2,
    },
    Vector {
        _handler: GPIO1_INT3,
    },
    Vector {
        _handler: GPIO1_INT4,
    },
    Vector {
        _handler: GPIO1_INT5,
    },
    Vector {
        _handler: GPIO1_INT6,
    },
    Vector {
        _handler: GPIO1_INT7,
    },
    Vector {
        _handler: GPIO1_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO1_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO2_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO2_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO3_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO3_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO4_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO4_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO5_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO5_COMBINED_16_31,
    },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: FLEXIO2 },
    Vector { _handler: WDOG1 },
    Vector { _handler: RTWDOG },
    Vector { _handler: EWM },
    Vector { _handler: CCM_1 },
    Vector { _handler: CCM_2 },
    Vector { _handler: GPC },
    Vector { _handler: SRC },
    Vector { _reserved: 0 },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector { _handler: FLEXSPI2 },
    Vector { _handler: FLEXSPI },
    Vector { _handler: SEMC },
    Vector { _handler: USDHC1 },
    Vector { _handler: USDHC2 },
    Vector { _handler: USB_OTG2 },
    Vector { _handler: USB_OTG1 },
    Vector { _handler: ENET },
    Vector {
        _handler: ENET_1588_TIMER,
    },
    Vector {
        _handler: XBAR1_IRQ_0_1,
    },
    Vector {
        _handler: XBAR1_IRQ_2_3,
    },
    Vector {
        _handler: ADC_ETC_IRQ0,
    },
    Vector {
        _handler: ADC_ETC_IRQ1,
    },
    Vector {
        _handler: ADC_ETC_IRQ2,
    },
    Vector {
        _handler: ADC_ETC_ERROR_IRQ,
    },
    Vector { _handler: PIT },
    Vector { _handler: ACMP1 },
    Vector { _handler: ACMP2 },
    Vector { _handler: ACMP3 },
    Vector { _handler: ACMP4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ENC1 },
    Vector { _handler: ENC2 },
    Vector { _handler: ENC3 },
    Vector { _handler: ENC4 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR3 },
    Vector { _handler: TMR4 },
    Vector { _handler: PWM2_0 },
    Vector { _handler: PWM2_1 },
    Vector { _handler: PWM2_2 },
    Vector { _handler: PWM2_3 },
    Vector {
        _handler: PWM2_FAULT,
    },
    Vector { _handler: PWM3_0 },
    Vector { _handler: PWM3_1 },
    Vector { _handler: PWM3_2 },
    Vector { _handler: PWM3_3 },
    Vector {
        _handler: PWM3_FAULT,
    },
    Vector { _handler: PWM4_0 },
    Vector { _handler: PWM4_1 },
    Vector { _handler: PWM4_2 },
    Vector { _handler: PWM4_3 },
    Vector {
        _handler: PWM4_FAULT,
    },
    Vector { _handler: ENET2 },
    Vector {
        _handler: ENET2_1588_TIMER,
    },
    Vector { _handler: CAN3 },
    Vector { _reserved: 0 },
    Vector { _handler: FLEXIO3 },
    Vector {
        _handler: GPIO6_7_8_9,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - DMA0_DMA16"]
    DMA0_DMA16 = 0,
    #[doc = "1 - DMA1_DMA17"]
    DMA1_DMA17 = 1,
    #[doc = "2 - DMA2_DMA18"]
    DMA2_DMA18 = 2,
    #[doc = "3 - DMA3_DMA19"]
    DMA3_DMA19 = 3,
    #[doc = "4 - DMA4_DMA20"]
    DMA4_DMA20 = 4,
    #[doc = "5 - DMA5_DMA21"]
    DMA5_DMA21 = 5,
    #[doc = "6 - DMA6_DMA22"]
    DMA6_DMA22 = 6,
    #[doc = "7 - DMA7_DMA23"]
    DMA7_DMA23 = 7,
    #[doc = "8 - DMA8_DMA24"]
    DMA8_DMA24 = 8,
    #[doc = "9 - DMA9_DMA25"]
    DMA9_DMA25 = 9,
    #[doc = "10 - DMA10_DMA26"]
    DMA10_DMA26 = 10,
    #[doc = "11 - DMA11_DMA27"]
    DMA11_DMA27 = 11,
    #[doc = "12 - DMA12_DMA28"]
    DMA12_DMA28 = 12,
    #[doc = "13 - DMA13_DMA29"]
    DMA13_DMA29 = 13,
    #[doc = "14 - DMA14_DMA30"]
    DMA14_DMA30 = 14,
    #[doc = "15 - DMA15_DMA31"]
    DMA15_DMA31 = 15,
    #[doc = "16 - DMA_ERROR"]
    DMA_ERROR = 16,
    #[doc = "17 - CTI0_ERROR"]
    CTI0_ERROR = 17,
    #[doc = "18 - CTI1_ERROR"]
    CTI1_ERROR = 18,
    #[doc = "19 - CORE"]
    CORE = 19,
    #[doc = "20 - LPUART1"]
    LPUART1 = 20,
    #[doc = "21 - LPUART2"]
    LPUART2 = 21,
    #[doc = "22 - LPUART3"]
    LPUART3 = 22,
    #[doc = "23 - LPUART4"]
    LPUART4 = 23,
    #[doc = "24 - LPUART5"]
    LPUART5 = 24,
    #[doc = "25 - LPUART6"]
    LPUART6 = 25,
    #[doc = "26 - LPUART7"]
    LPUART7 = 26,
    #[doc = "27 - LPUART8"]
    LPUART8 = 27,
    #[doc = "28 - LPI2C1"]
    LPI2C1 = 28,
    #[doc = "29 - LPI2C2"]
    LPI2C2 = 29,
    #[doc = "30 - LPI2C3"]
    LPI2C3 = 30,
    #[doc = "31 - LPI2C4"]
    LPI2C4 = 31,
    #[doc = "32 - LPSPI1"]
    LPSPI1 = 32,
    #[doc = "33 - LPSPI2"]
    LPSPI2 = 33,
    #[doc = "34 - LPSPI3"]
    LPSPI3 = 34,
    #[doc = "35 - LPSPI4"]
    LPSPI4 = 35,
    #[doc = "36 - CAN1"]
    CAN1 = 36,
    #[doc = "37 - CAN2"]
    CAN2 = 37,
    #[doc = "38 - FLEXRAM"]
    FLEXRAM = 38,
    #[doc = "39 - KPP"]
    KPP = 39,
    #[doc = "40 - TSC_DIG"]
    TSC_DIG = 40,
    #[doc = "41 - GPR_IRQ"]
    GPR_IRQ = 41,
    #[doc = "42 - LCDIF"]
    LCDIF = 42,
    #[doc = "43 - CSI"]
    CSI = 43,
    #[doc = "44 - PXP"]
    PXP = 44,
    #[doc = "45 - WDOG2"]
    WDOG2 = 45,
    #[doc = "46 - SNVS_HP_WRAPPER"]
    SNVS_HP_WRAPPER = 46,
    #[doc = "47 - SNVS_HP_WRAPPER_TZ"]
    SNVS_HP_WRAPPER_TZ = 47,
    #[doc = "48 - SNVS_LP_WRAPPER"]
    SNVS_LP_WRAPPER = 48,
    #[doc = "49 - CSU"]
    CSU = 49,
    #[doc = "50 - DCP"]
    DCP = 50,
    #[doc = "51 - DCP_VMI"]
    DCP_VMI = 51,
    #[doc = "52 - Reserved68"]
    RESERVED68 = 52,
    #[doc = "53 - TRNG"]
    TRNG = 53,
    #[doc = "54 - SJC"]
    SJC = 54,
    #[doc = "55 - BEE"]
    BEE = 55,
    #[doc = "56 - SAI1"]
    SAI1 = 56,
    #[doc = "57 - SAI2"]
    SAI2 = 57,
    #[doc = "58 - SAI3_RX"]
    SAI3_RX = 58,
    #[doc = "59 - SAI3_TX"]
    SAI3_TX = 59,
    #[doc = "60 - SPDIF"]
    SPDIF = 60,
    #[doc = "61 - PMU_EVENT"]
    PMU_EVENT = 61,
    #[doc = "62 - Reserved78"]
    RESERVED78 = 62,
    #[doc = "63 - TEMP_LOW_HIGH"]
    TEMP_LOW_HIGH = 63,
    #[doc = "64 - TEMP_PANIC"]
    TEMP_PANIC = 64,
    #[doc = "65 - USB_PHY1"]
    USB_PHY1 = 65,
    #[doc = "66 - USB_PHY2"]
    USB_PHY2 = 66,
    #[doc = "67 - ADC1"]
    ADC1 = 67,
    #[doc = "68 - ADC2"]
    ADC2 = 68,
    #[doc = "69 - DCDC"]
    DCDC = 69,
    #[doc = "70 - Reserved86"]
    RESERVED86 = 70,
    #[doc = "71 - Reserved87"]
    RESERVED87 = 71,
    #[doc = "72 - GPIO1_INT0"]
    GPIO1_INT0 = 72,
    #[doc = "73 - GPIO1_INT1"]
    GPIO1_INT1 = 73,
    #[doc = "74 - GPIO1_INT2"]
    GPIO1_INT2 = 74,
    #[doc = "75 - GPIO1_INT3"]
    GPIO1_INT3 = 75,
    #[doc = "76 - GPIO1_INT4"]
    GPIO1_INT4 = 76,
    #[doc = "77 - GPIO1_INT5"]
    GPIO1_INT5 = 77,
    #[doc = "78 - GPIO1_INT6"]
    GPIO1_INT6 = 78,
    #[doc = "79 - GPIO1_INT7"]
    GPIO1_INT7 = 79,
    #[doc = "80 - GPIO1_Combined_0_15"]
    GPIO1_COMBINED_0_15 = 80,
    #[doc = "81 - GPIO1_Combined_16_31"]
    GPIO1_COMBINED_16_31 = 81,
    #[doc = "82 - GPIO2_Combined_0_15"]
    GPIO2_COMBINED_0_15 = 82,
    #[doc = "83 - GPIO2_Combined_16_31"]
    GPIO2_COMBINED_16_31 = 83,
    #[doc = "84 - GPIO3_Combined_0_15"]
    GPIO3_COMBINED_0_15 = 84,
    #[doc = "85 - GPIO3_Combined_16_31"]
    GPIO3_COMBINED_16_31 = 85,
    #[doc = "86 - GPIO4_Combined_0_15"]
    GPIO4_COMBINED_0_15 = 86,
    #[doc = "87 - GPIO4_Combined_16_31"]
    GPIO4_COMBINED_16_31 = 87,
    #[doc = "88 - GPIO5_Combined_0_15"]
    GPIO5_COMBINED_0_15 = 88,
    #[doc = "89 - GPIO5_Combined_16_31"]
    GPIO5_COMBINED_16_31 = 89,
    #[doc = "90 - FLEXIO1"]
    FLEXIO1 = 90,
    #[doc = "91 - FLEXIO2"]
    FLEXIO2 = 91,
    #[doc = "92 - WDOG1"]
    WDOG1 = 92,
    #[doc = "93 - RTWDOG"]
    RTWDOG = 93,
    #[doc = "94 - EWM"]
    EWM = 94,
    #[doc = "95 - CCM_1"]
    CCM_1 = 95,
    #[doc = "96 - CCM_2"]
    CCM_2 = 96,
    #[doc = "97 - GPC"]
    GPC = 97,
    #[doc = "98 - SRC"]
    SRC = 98,
    #[doc = "99 - Reserved115"]
    RESERVED115 = 99,
    #[doc = "100 - GPT1"]
    GPT1 = 100,
    #[doc = "101 - GPT2"]
    GPT2 = 101,
    #[doc = "102 - PWM1_0"]
    PWM1_0 = 102,
    #[doc = "103 - PWM1_1"]
    PWM1_1 = 103,
    #[doc = "104 - PWM1_2"]
    PWM1_2 = 104,
    #[doc = "105 - PWM1_3"]
    PWM1_3 = 105,
    #[doc = "106 - PWM1_FAULT"]
    PWM1_FAULT = 106,
    #[doc = "107 - FLEXSPI2"]
    FLEXSPI2 = 107,
    #[doc = "108 - FLEXSPI"]
    FLEXSPI = 108,
    #[doc = "109 - SEMC"]
    SEMC = 109,
    #[doc = "110 - USDHC1"]
    USDHC1 = 110,
    #[doc = "111 - USDHC2"]
    USDHC2 = 111,
    #[doc = "112 - USB_OTG2"]
    USB_OTG2 = 112,
    #[doc = "113 - USB_OTG1"]
    USB_OTG1 = 113,
    #[doc = "114 - ENET"]
    ENET = 114,
    #[doc = "115 - ENET_1588_Timer"]
    ENET_1588_TIMER = 115,
    #[doc = "116 - XBAR1_IRQ_0_1"]
    XBAR1_IRQ_0_1 = 116,
    #[doc = "117 - XBAR1_IRQ_2_3"]
    XBAR1_IRQ_2_3 = 117,
    #[doc = "118 - ADC_ETC_IRQ0"]
    ADC_ETC_IRQ0 = 118,
    #[doc = "119 - ADC_ETC_IRQ1"]
    ADC_ETC_IRQ1 = 119,
    #[doc = "120 - ADC_ETC_IRQ2"]
    ADC_ETC_IRQ2 = 120,
    #[doc = "121 - ADC_ETC_ERROR_IRQ"]
    ADC_ETC_ERROR_IRQ = 121,
    #[doc = "122 - PIT"]
    PIT = 122,
    #[doc = "123 - ACMP1"]
    ACMP1 = 123,
    #[doc = "124 - ACMP2"]
    ACMP2 = 124,
    #[doc = "125 - ACMP3"]
    ACMP3 = 125,
    #[doc = "126 - ACMP4"]
    ACMP4 = 126,
    #[doc = "127 - Reserved143"]
    RESERVED143 = 127,
    #[doc = "128 - Reserved144"]
    RESERVED144 = 128,
    #[doc = "129 - ENC1"]
    ENC1 = 129,
    #[doc = "130 - ENC2"]
    ENC2 = 130,
    #[doc = "131 - ENC3"]
    ENC3 = 131,
    #[doc = "132 - ENC4"]
    ENC4 = 132,
    #[doc = "133 - TMR1"]
    TMR1 = 133,
    #[doc = "134 - TMR2"]
    TMR2 = 134,
    #[doc = "135 - TMR3"]
    TMR3 = 135,
    #[doc = "136 - TMR4"]
    TMR4 = 136,
    #[doc = "137 - PWM2_0"]
    PWM2_0 = 137,
    #[doc = "138 - PWM2_1"]
    PWM2_1 = 138,
    #[doc = "139 - PWM2_2"]
    PWM2_2 = 139,
    #[doc = "140 - PWM2_3"]
    PWM2_3 = 140,
    #[doc = "141 - PWM2_FAULT"]
    PWM2_FAULT = 141,
    #[doc = "142 - PWM3_0"]
    PWM3_0 = 142,
    #[doc = "143 - PWM3_1"]
    PWM3_1 = 143,
    #[doc = "144 - PWM3_2"]
    PWM3_2 = 144,
    #[doc = "145 - PWM3_3"]
    PWM3_3 = 145,
    #[doc = "146 - PWM3_FAULT"]
    PWM3_FAULT = 146,
    #[doc = "147 - PWM4_0"]
    PWM4_0 = 147,
    #[doc = "148 - PWM4_1"]
    PWM4_1 = 148,
    #[doc = "149 - PWM4_2"]
    PWM4_2 = 149,
    #[doc = "150 - PWM4_3"]
    PWM4_3 = 150,
    #[doc = "151 - PWM4_FAULT"]
    PWM4_FAULT = 151,
    #[doc = "152 - ENET2"]
    ENET2 = 152,
    #[doc = "153 - ENET2_1588_Timer"]
    ENET2_1588_TIMER = 153,
    #[doc = "154 - CAN3"]
    CAN3 = 154,
    #[doc = "155 - Reserved171"]
    RESERVED171 = 155,
    #[doc = "156 - FLEXIO3"]
    FLEXIO3 = 156,
    #[doc = "157 - GPIO6_7_8_9"]
    GPIO6_7_8_9 = 157,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
