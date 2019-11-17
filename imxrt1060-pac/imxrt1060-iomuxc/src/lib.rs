#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]

include!("../../generic.rs");

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 20usize],
    #[doc = "0x14 - SW_MUX_CTL_PAD_GPIO_EMC_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_00: SW_MUX_CTL_PAD_GPIO_EMC_00,
    #[doc = "0x18 - SW_MUX_CTL_PAD_GPIO_EMC_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_01: SW_MUX_CTL_PAD_GPIO_EMC_01,
    #[doc = "0x1c - SW_MUX_CTL_PAD_GPIO_EMC_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_02: SW_MUX_CTL_PAD_GPIO_EMC_02,
    #[doc = "0x20 - SW_MUX_CTL_PAD_GPIO_EMC_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_03: SW_MUX_CTL_PAD_GPIO_EMC_03,
    #[doc = "0x24 - SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_04: SW_MUX_CTL_PAD_GPIO_EMC_04,
    #[doc = "0x28 - SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_05: SW_MUX_CTL_PAD_GPIO_EMC_05,
    #[doc = "0x2c - SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_06: SW_MUX_CTL_PAD_GPIO_EMC_06,
    #[doc = "0x30 - SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_07: SW_MUX_CTL_PAD_GPIO_EMC_07,
    #[doc = "0x34 - SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_08: SW_MUX_CTL_PAD_GPIO_EMC_08,
    #[doc = "0x38 - SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_09: SW_MUX_CTL_PAD_GPIO_EMC_09,
    #[doc = "0x3c - SW_MUX_CTL_PAD_GPIO_EMC_10 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_10: SW_MUX_CTL_PAD_GPIO_EMC_10,
    #[doc = "0x40 - SW_MUX_CTL_PAD_GPIO_EMC_11 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_11: SW_MUX_CTL_PAD_GPIO_EMC_11,
    #[doc = "0x44 - SW_MUX_CTL_PAD_GPIO_EMC_12 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_12: SW_MUX_CTL_PAD_GPIO_EMC_12,
    #[doc = "0x48 - SW_MUX_CTL_PAD_GPIO_EMC_13 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_13: SW_MUX_CTL_PAD_GPIO_EMC_13,
    #[doc = "0x4c - SW_MUX_CTL_PAD_GPIO_EMC_14 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_14: SW_MUX_CTL_PAD_GPIO_EMC_14,
    #[doc = "0x50 - SW_MUX_CTL_PAD_GPIO_EMC_15 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_15: SW_MUX_CTL_PAD_GPIO_EMC_15,
    #[doc = "0x54 - SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_16: SW_MUX_CTL_PAD_GPIO_EMC_16,
    #[doc = "0x58 - SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_17: SW_MUX_CTL_PAD_GPIO_EMC_17,
    #[doc = "0x5c - SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_18: SW_MUX_CTL_PAD_GPIO_EMC_18,
    #[doc = "0x60 - SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_19: SW_MUX_CTL_PAD_GPIO_EMC_19,
    #[doc = "0x64 - SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_20: SW_MUX_CTL_PAD_GPIO_EMC_20,
    #[doc = "0x68 - SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_21: SW_MUX_CTL_PAD_GPIO_EMC_21,
    #[doc = "0x6c - SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_22: SW_MUX_CTL_PAD_GPIO_EMC_22,
    #[doc = "0x70 - SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_23: SW_MUX_CTL_PAD_GPIO_EMC_23,
    #[doc = "0x74 - SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_24: SW_MUX_CTL_PAD_GPIO_EMC_24,
    #[doc = "0x78 - SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_25: SW_MUX_CTL_PAD_GPIO_EMC_25,
    #[doc = "0x7c - SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_26: SW_MUX_CTL_PAD_GPIO_EMC_26,
    #[doc = "0x80 - SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_27: SW_MUX_CTL_PAD_GPIO_EMC_27,
    #[doc = "0x84 - SW_MUX_CTL_PAD_GPIO_EMC_28 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_28: SW_MUX_CTL_PAD_GPIO_EMC_28,
    #[doc = "0x88 - SW_MUX_CTL_PAD_GPIO_EMC_29 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_29: SW_MUX_CTL_PAD_GPIO_EMC_29,
    #[doc = "0x8c - SW_MUX_CTL_PAD_GPIO_EMC_30 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_30: SW_MUX_CTL_PAD_GPIO_EMC_30,
    #[doc = "0x90 - SW_MUX_CTL_PAD_GPIO_EMC_31 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_31: SW_MUX_CTL_PAD_GPIO_EMC_31,
    #[doc = "0x94 - SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_32: SW_MUX_CTL_PAD_GPIO_EMC_32,
    #[doc = "0x98 - SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_33: SW_MUX_CTL_PAD_GPIO_EMC_33,
    #[doc = "0x9c - SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_34: SW_MUX_CTL_PAD_GPIO_EMC_34,
    #[doc = "0xa0 - SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_35: SW_MUX_CTL_PAD_GPIO_EMC_35,
    #[doc = "0xa4 - SW_MUX_CTL_PAD_GPIO_EMC_36 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_36: SW_MUX_CTL_PAD_GPIO_EMC_36,
    #[doc = "0xa8 - SW_MUX_CTL_PAD_GPIO_EMC_37 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_37: SW_MUX_CTL_PAD_GPIO_EMC_37,
    #[doc = "0xac - SW_MUX_CTL_PAD_GPIO_EMC_38 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_38: SW_MUX_CTL_PAD_GPIO_EMC_38,
    #[doc = "0xb0 - SW_MUX_CTL_PAD_GPIO_EMC_39 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_39: SW_MUX_CTL_PAD_GPIO_EMC_39,
    #[doc = "0xb4 - SW_MUX_CTL_PAD_GPIO_EMC_40 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_40: SW_MUX_CTL_PAD_GPIO_EMC_40,
    #[doc = "0xb8 - SW_MUX_CTL_PAD_GPIO_EMC_41 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_emc_41: SW_MUX_CTL_PAD_GPIO_EMC_41,
    #[doc = "0xbc - SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_00: SW_MUX_CTL_PAD_GPIO_AD_B0_00,
    #[doc = "0xc0 - SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_01: SW_MUX_CTL_PAD_GPIO_AD_B0_01,
    #[doc = "0xc4 - SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_02: SW_MUX_CTL_PAD_GPIO_AD_B0_02,
    #[doc = "0xc8 - SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_03: SW_MUX_CTL_PAD_GPIO_AD_B0_03,
    #[doc = "0xcc - SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_04: SW_MUX_CTL_PAD_GPIO_AD_B0_04,
    #[doc = "0xd0 - SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_05: SW_MUX_CTL_PAD_GPIO_AD_B0_05,
    #[doc = "0xd4 - SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_06: SW_MUX_CTL_PAD_GPIO_AD_B0_06,
    #[doc = "0xd8 - SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_07: SW_MUX_CTL_PAD_GPIO_AD_B0_07,
    #[doc = "0xdc - SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_08: SW_MUX_CTL_PAD_GPIO_AD_B0_08,
    #[doc = "0xe0 - SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_09: SW_MUX_CTL_PAD_GPIO_AD_B0_09,
    #[doc = "0xe4 - SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_10: SW_MUX_CTL_PAD_GPIO_AD_B0_10,
    #[doc = "0xe8 - SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_11: SW_MUX_CTL_PAD_GPIO_AD_B0_11,
    #[doc = "0xec - SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_12: SW_MUX_CTL_PAD_GPIO_AD_B0_12,
    #[doc = "0xf0 - SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_13: SW_MUX_CTL_PAD_GPIO_AD_B0_13,
    #[doc = "0xf4 - SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_14: SW_MUX_CTL_PAD_GPIO_AD_B0_14,
    #[doc = "0xf8 - SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b0_15: SW_MUX_CTL_PAD_GPIO_AD_B0_15,
    #[doc = "0xfc - SW_MUX_CTL_PAD_GPIO_AD_B1_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_00: SW_MUX_CTL_PAD_GPIO_AD_B1_00,
    #[doc = "0x100 - SW_MUX_CTL_PAD_GPIO_AD_B1_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_01: SW_MUX_CTL_PAD_GPIO_AD_B1_01,
    #[doc = "0x104 - SW_MUX_CTL_PAD_GPIO_AD_B1_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_02: SW_MUX_CTL_PAD_GPIO_AD_B1_02,
    #[doc = "0x108 - SW_MUX_CTL_PAD_GPIO_AD_B1_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_03: SW_MUX_CTL_PAD_GPIO_AD_B1_03,
    #[doc = "0x10c - SW_MUX_CTL_PAD_GPIO_AD_B1_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_04: SW_MUX_CTL_PAD_GPIO_AD_B1_04,
    #[doc = "0x110 - SW_MUX_CTL_PAD_GPIO_AD_B1_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_05: SW_MUX_CTL_PAD_GPIO_AD_B1_05,
    #[doc = "0x114 - SW_MUX_CTL_PAD_GPIO_AD_B1_06 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_06: SW_MUX_CTL_PAD_GPIO_AD_B1_06,
    #[doc = "0x118 - SW_MUX_CTL_PAD_GPIO_AD_B1_07 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_07: SW_MUX_CTL_PAD_GPIO_AD_B1_07,
    #[doc = "0x11c - SW_MUX_CTL_PAD_GPIO_AD_B1_08 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_08: SW_MUX_CTL_PAD_GPIO_AD_B1_08,
    #[doc = "0x120 - SW_MUX_CTL_PAD_GPIO_AD_B1_09 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_09: SW_MUX_CTL_PAD_GPIO_AD_B1_09,
    #[doc = "0x124 - SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_10: SW_MUX_CTL_PAD_GPIO_AD_B1_10,
    #[doc = "0x128 - SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_11: SW_MUX_CTL_PAD_GPIO_AD_B1_11,
    #[doc = "0x12c - SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_12: SW_MUX_CTL_PAD_GPIO_AD_B1_12,
    #[doc = "0x130 - SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_13: SW_MUX_CTL_PAD_GPIO_AD_B1_13,
    #[doc = "0x134 - SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_14: SW_MUX_CTL_PAD_GPIO_AD_B1_14,
    #[doc = "0x138 - SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_ad_b1_15: SW_MUX_CTL_PAD_GPIO_AD_B1_15,
    #[doc = "0x13c - SW_MUX_CTL_PAD_GPIO_B0_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_00: SW_MUX_CTL_PAD_GPIO_B0_00,
    #[doc = "0x140 - SW_MUX_CTL_PAD_GPIO_B0_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_01: SW_MUX_CTL_PAD_GPIO_B0_01,
    #[doc = "0x144 - SW_MUX_CTL_PAD_GPIO_B0_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_02: SW_MUX_CTL_PAD_GPIO_B0_02,
    #[doc = "0x148 - SW_MUX_CTL_PAD_GPIO_B0_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_03: SW_MUX_CTL_PAD_GPIO_B0_03,
    #[doc = "0x14c - SW_MUX_CTL_PAD_GPIO_B0_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_04: SW_MUX_CTL_PAD_GPIO_B0_04,
    #[doc = "0x150 - SW_MUX_CTL_PAD_GPIO_B0_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_05: SW_MUX_CTL_PAD_GPIO_B0_05,
    #[doc = "0x154 - SW_MUX_CTL_PAD_GPIO_B0_06 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_06: SW_MUX_CTL_PAD_GPIO_B0_06,
    #[doc = "0x158 - SW_MUX_CTL_PAD_GPIO_B0_07 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_07: SW_MUX_CTL_PAD_GPIO_B0_07,
    #[doc = "0x15c - SW_MUX_CTL_PAD_GPIO_B0_08 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_08: SW_MUX_CTL_PAD_GPIO_B0_08,
    #[doc = "0x160 - SW_MUX_CTL_PAD_GPIO_B0_09 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_09: SW_MUX_CTL_PAD_GPIO_B0_09,
    #[doc = "0x164 - SW_MUX_CTL_PAD_GPIO_B0_10 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_10: SW_MUX_CTL_PAD_GPIO_B0_10,
    #[doc = "0x168 - SW_MUX_CTL_PAD_GPIO_B0_11 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_11: SW_MUX_CTL_PAD_GPIO_B0_11,
    #[doc = "0x16c - SW_MUX_CTL_PAD_GPIO_B0_12 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_12: SW_MUX_CTL_PAD_GPIO_B0_12,
    #[doc = "0x170 - SW_MUX_CTL_PAD_GPIO_B0_13 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_13: SW_MUX_CTL_PAD_GPIO_B0_13,
    #[doc = "0x174 - SW_MUX_CTL_PAD_GPIO_B0_14 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_14: SW_MUX_CTL_PAD_GPIO_B0_14,
    #[doc = "0x178 - SW_MUX_CTL_PAD_GPIO_B0_15 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b0_15: SW_MUX_CTL_PAD_GPIO_B0_15,
    #[doc = "0x17c - SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_00: SW_MUX_CTL_PAD_GPIO_B1_00,
    #[doc = "0x180 - SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_01: SW_MUX_CTL_PAD_GPIO_B1_01,
    #[doc = "0x184 - SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_02: SW_MUX_CTL_PAD_GPIO_B1_02,
    #[doc = "0x188 - SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_03: SW_MUX_CTL_PAD_GPIO_B1_03,
    #[doc = "0x18c - SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_04: SW_MUX_CTL_PAD_GPIO_B1_04,
    #[doc = "0x190 - SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_05: SW_MUX_CTL_PAD_GPIO_B1_05,
    #[doc = "0x194 - SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_06: SW_MUX_CTL_PAD_GPIO_B1_06,
    #[doc = "0x198 - SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_07: SW_MUX_CTL_PAD_GPIO_B1_07,
    #[doc = "0x19c - SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_08: SW_MUX_CTL_PAD_GPIO_B1_08,
    #[doc = "0x1a0 - SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_09: SW_MUX_CTL_PAD_GPIO_B1_09,
    #[doc = "0x1a4 - SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_10: SW_MUX_CTL_PAD_GPIO_B1_10,
    #[doc = "0x1a8 - SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_11: SW_MUX_CTL_PAD_GPIO_B1_11,
    #[doc = "0x1ac - SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_12: SW_MUX_CTL_PAD_GPIO_B1_12,
    #[doc = "0x1b0 - SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_13: SW_MUX_CTL_PAD_GPIO_B1_13,
    #[doc = "0x1b4 - SW_MUX_CTL_PAD_GPIO_B1_14 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_14: SW_MUX_CTL_PAD_GPIO_B1_14,
    #[doc = "0x1b8 - SW_MUX_CTL_PAD_GPIO_B1_15 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_b1_15: SW_MUX_CTL_PAD_GPIO_B1_15,
    #[doc = "0x1bc - SW_MUX_CTL_PAD_GPIO_SD_B0_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b0_00: SW_MUX_CTL_PAD_GPIO_SD_B0_00,
    #[doc = "0x1c0 - SW_MUX_CTL_PAD_GPIO_SD_B0_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b0_01: SW_MUX_CTL_PAD_GPIO_SD_B0_01,
    #[doc = "0x1c4 - SW_MUX_CTL_PAD_GPIO_SD_B0_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b0_02: SW_MUX_CTL_PAD_GPIO_SD_B0_02,
    #[doc = "0x1c8 - SW_MUX_CTL_PAD_GPIO_SD_B0_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b0_03: SW_MUX_CTL_PAD_GPIO_SD_B0_03,
    #[doc = "0x1cc - SW_MUX_CTL_PAD_GPIO_SD_B0_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b0_04: SW_MUX_CTL_PAD_GPIO_SD_B0_04,
    #[doc = "0x1d0 - SW_MUX_CTL_PAD_GPIO_SD_B0_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b0_05: SW_MUX_CTL_PAD_GPIO_SD_B0_05,
    #[doc = "0x1d4 - SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_00: SW_MUX_CTL_PAD_GPIO_SD_B1_00,
    #[doc = "0x1d8 - SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_01: SW_MUX_CTL_PAD_GPIO_SD_B1_01,
    #[doc = "0x1dc - SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_02: SW_MUX_CTL_PAD_GPIO_SD_B1_02,
    #[doc = "0x1e0 - SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_03: SW_MUX_CTL_PAD_GPIO_SD_B1_03,
    #[doc = "0x1e4 - SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_04: SW_MUX_CTL_PAD_GPIO_SD_B1_04,
    #[doc = "0x1e8 - SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_05: SW_MUX_CTL_PAD_GPIO_SD_B1_05,
    #[doc = "0x1ec - SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_06: SW_MUX_CTL_PAD_GPIO_SD_B1_06,
    #[doc = "0x1f0 - SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_07: SW_MUX_CTL_PAD_GPIO_SD_B1_07,
    #[doc = "0x1f4 - SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_08: SW_MUX_CTL_PAD_GPIO_SD_B1_08,
    #[doc = "0x1f8 - SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_09: SW_MUX_CTL_PAD_GPIO_SD_B1_09,
    #[doc = "0x1fc - SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_10: SW_MUX_CTL_PAD_GPIO_SD_B1_10,
    #[doc = "0x200 - SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_sd_b1_11: SW_MUX_CTL_PAD_GPIO_SD_B1_11,
    #[doc = "0x204 - SW_PAD_CTL_PAD_GPIO_EMC_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_00: SW_PAD_CTL_PAD_GPIO_EMC_00,
    #[doc = "0x208 - SW_PAD_CTL_PAD_GPIO_EMC_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_01: SW_PAD_CTL_PAD_GPIO_EMC_01,
    #[doc = "0x20c - SW_PAD_CTL_PAD_GPIO_EMC_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_02: SW_PAD_CTL_PAD_GPIO_EMC_02,
    #[doc = "0x210 - SW_PAD_CTL_PAD_GPIO_EMC_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_03: SW_PAD_CTL_PAD_GPIO_EMC_03,
    #[doc = "0x214 - SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_04: SW_PAD_CTL_PAD_GPIO_EMC_04,
    #[doc = "0x218 - SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_05: SW_PAD_CTL_PAD_GPIO_EMC_05,
    #[doc = "0x21c - SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_06: SW_PAD_CTL_PAD_GPIO_EMC_06,
    #[doc = "0x220 - SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_07: SW_PAD_CTL_PAD_GPIO_EMC_07,
    #[doc = "0x224 - SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_08: SW_PAD_CTL_PAD_GPIO_EMC_08,
    #[doc = "0x228 - SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_09: SW_PAD_CTL_PAD_GPIO_EMC_09,
    #[doc = "0x22c - SW_PAD_CTL_PAD_GPIO_EMC_10 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_10: SW_PAD_CTL_PAD_GPIO_EMC_10,
    #[doc = "0x230 - SW_PAD_CTL_PAD_GPIO_EMC_11 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_11: SW_PAD_CTL_PAD_GPIO_EMC_11,
    #[doc = "0x234 - SW_PAD_CTL_PAD_GPIO_EMC_12 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_12: SW_PAD_CTL_PAD_GPIO_EMC_12,
    #[doc = "0x238 - SW_PAD_CTL_PAD_GPIO_EMC_13 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_13: SW_PAD_CTL_PAD_GPIO_EMC_13,
    #[doc = "0x23c - SW_PAD_CTL_PAD_GPIO_EMC_14 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_14: SW_PAD_CTL_PAD_GPIO_EMC_14,
    #[doc = "0x240 - SW_PAD_CTL_PAD_GPIO_EMC_15 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_15: SW_PAD_CTL_PAD_GPIO_EMC_15,
    #[doc = "0x244 - SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_16: SW_PAD_CTL_PAD_GPIO_EMC_16,
    #[doc = "0x248 - SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_17: SW_PAD_CTL_PAD_GPIO_EMC_17,
    #[doc = "0x24c - SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_18: SW_PAD_CTL_PAD_GPIO_EMC_18,
    #[doc = "0x250 - SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_19: SW_PAD_CTL_PAD_GPIO_EMC_19,
    #[doc = "0x254 - SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_20: SW_PAD_CTL_PAD_GPIO_EMC_20,
    #[doc = "0x258 - SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_21: SW_PAD_CTL_PAD_GPIO_EMC_21,
    #[doc = "0x25c - SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_22: SW_PAD_CTL_PAD_GPIO_EMC_22,
    #[doc = "0x260 - SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_23: SW_PAD_CTL_PAD_GPIO_EMC_23,
    #[doc = "0x264 - SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_24: SW_PAD_CTL_PAD_GPIO_EMC_24,
    #[doc = "0x268 - SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_25: SW_PAD_CTL_PAD_GPIO_EMC_25,
    #[doc = "0x26c - SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_26: SW_PAD_CTL_PAD_GPIO_EMC_26,
    #[doc = "0x270 - SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_27: SW_PAD_CTL_PAD_GPIO_EMC_27,
    #[doc = "0x274 - SW_PAD_CTL_PAD_GPIO_EMC_28 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_28: SW_PAD_CTL_PAD_GPIO_EMC_28,
    #[doc = "0x278 - SW_PAD_CTL_PAD_GPIO_EMC_29 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_29: SW_PAD_CTL_PAD_GPIO_EMC_29,
    #[doc = "0x27c - SW_PAD_CTL_PAD_GPIO_EMC_30 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_30: SW_PAD_CTL_PAD_GPIO_EMC_30,
    #[doc = "0x280 - SW_PAD_CTL_PAD_GPIO_EMC_31 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_31: SW_PAD_CTL_PAD_GPIO_EMC_31,
    #[doc = "0x284 - SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_32: SW_PAD_CTL_PAD_GPIO_EMC_32,
    #[doc = "0x288 - SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_33: SW_PAD_CTL_PAD_GPIO_EMC_33,
    #[doc = "0x28c - SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_34: SW_PAD_CTL_PAD_GPIO_EMC_34,
    #[doc = "0x290 - SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_35: SW_PAD_CTL_PAD_GPIO_EMC_35,
    #[doc = "0x294 - SW_PAD_CTL_PAD_GPIO_EMC_36 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_36: SW_PAD_CTL_PAD_GPIO_EMC_36,
    #[doc = "0x298 - SW_PAD_CTL_PAD_GPIO_EMC_37 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_37: SW_PAD_CTL_PAD_GPIO_EMC_37,
    #[doc = "0x29c - SW_PAD_CTL_PAD_GPIO_EMC_38 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_38: SW_PAD_CTL_PAD_GPIO_EMC_38,
    #[doc = "0x2a0 - SW_PAD_CTL_PAD_GPIO_EMC_39 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_39: SW_PAD_CTL_PAD_GPIO_EMC_39,
    #[doc = "0x2a4 - SW_PAD_CTL_PAD_GPIO_EMC_40 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_40: SW_PAD_CTL_PAD_GPIO_EMC_40,
    #[doc = "0x2a8 - SW_PAD_CTL_PAD_GPIO_EMC_41 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_emc_41: SW_PAD_CTL_PAD_GPIO_EMC_41,
    #[doc = "0x2ac - SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_00: SW_PAD_CTL_PAD_GPIO_AD_B0_00,
    #[doc = "0x2b0 - SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_01: SW_PAD_CTL_PAD_GPIO_AD_B0_01,
    #[doc = "0x2b4 - SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_02: SW_PAD_CTL_PAD_GPIO_AD_B0_02,
    #[doc = "0x2b8 - SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_03: SW_PAD_CTL_PAD_GPIO_AD_B0_03,
    #[doc = "0x2bc - SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_04: SW_PAD_CTL_PAD_GPIO_AD_B0_04,
    #[doc = "0x2c0 - SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_05: SW_PAD_CTL_PAD_GPIO_AD_B0_05,
    #[doc = "0x2c4 - SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_06: SW_PAD_CTL_PAD_GPIO_AD_B0_06,
    #[doc = "0x2c8 - SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_07: SW_PAD_CTL_PAD_GPIO_AD_B0_07,
    #[doc = "0x2cc - SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_08: SW_PAD_CTL_PAD_GPIO_AD_B0_08,
    #[doc = "0x2d0 - SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_09: SW_PAD_CTL_PAD_GPIO_AD_B0_09,
    #[doc = "0x2d4 - SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_10: SW_PAD_CTL_PAD_GPIO_AD_B0_10,
    #[doc = "0x2d8 - SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_11: SW_PAD_CTL_PAD_GPIO_AD_B0_11,
    #[doc = "0x2dc - SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_12: SW_PAD_CTL_PAD_GPIO_AD_B0_12,
    #[doc = "0x2e0 - SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_13: SW_PAD_CTL_PAD_GPIO_AD_B0_13,
    #[doc = "0x2e4 - SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_14: SW_PAD_CTL_PAD_GPIO_AD_B0_14,
    #[doc = "0x2e8 - SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b0_15: SW_PAD_CTL_PAD_GPIO_AD_B0_15,
    #[doc = "0x2ec - SW_PAD_CTL_PAD_GPIO_AD_B1_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_00: SW_PAD_CTL_PAD_GPIO_AD_B1_00,
    #[doc = "0x2f0 - SW_PAD_CTL_PAD_GPIO_AD_B1_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_01: SW_PAD_CTL_PAD_GPIO_AD_B1_01,
    #[doc = "0x2f4 - SW_PAD_CTL_PAD_GPIO_AD_B1_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_02: SW_PAD_CTL_PAD_GPIO_AD_B1_02,
    #[doc = "0x2f8 - SW_PAD_CTL_PAD_GPIO_AD_B1_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_03: SW_PAD_CTL_PAD_GPIO_AD_B1_03,
    #[doc = "0x2fc - SW_PAD_CTL_PAD_GPIO_AD_B1_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_04: SW_PAD_CTL_PAD_GPIO_AD_B1_04,
    #[doc = "0x300 - SW_PAD_CTL_PAD_GPIO_AD_B1_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_05: SW_PAD_CTL_PAD_GPIO_AD_B1_05,
    #[doc = "0x304 - SW_PAD_CTL_PAD_GPIO_AD_B1_06 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_06: SW_PAD_CTL_PAD_GPIO_AD_B1_06,
    #[doc = "0x308 - SW_PAD_CTL_PAD_GPIO_AD_B1_07 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_07: SW_PAD_CTL_PAD_GPIO_AD_B1_07,
    #[doc = "0x30c - SW_PAD_CTL_PAD_GPIO_AD_B1_08 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_08: SW_PAD_CTL_PAD_GPIO_AD_B1_08,
    #[doc = "0x310 - SW_PAD_CTL_PAD_GPIO_AD_B1_09 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_09: SW_PAD_CTL_PAD_GPIO_AD_B1_09,
    #[doc = "0x314 - SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_10: SW_PAD_CTL_PAD_GPIO_AD_B1_10,
    #[doc = "0x318 - SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_11: SW_PAD_CTL_PAD_GPIO_AD_B1_11,
    #[doc = "0x31c - SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_12: SW_PAD_CTL_PAD_GPIO_AD_B1_12,
    #[doc = "0x320 - SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_13: SW_PAD_CTL_PAD_GPIO_AD_B1_13,
    #[doc = "0x324 - SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_14: SW_PAD_CTL_PAD_GPIO_AD_B1_14,
    #[doc = "0x328 - SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_ad_b1_15: SW_PAD_CTL_PAD_GPIO_AD_B1_15,
    #[doc = "0x32c - SW_PAD_CTL_PAD_GPIO_B0_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_00: SW_PAD_CTL_PAD_GPIO_B0_00,
    #[doc = "0x330 - SW_PAD_CTL_PAD_GPIO_B0_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_01: SW_PAD_CTL_PAD_GPIO_B0_01,
    #[doc = "0x334 - SW_PAD_CTL_PAD_GPIO_B0_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_02: SW_PAD_CTL_PAD_GPIO_B0_02,
    #[doc = "0x338 - SW_PAD_CTL_PAD_GPIO_B0_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_03: SW_PAD_CTL_PAD_GPIO_B0_03,
    #[doc = "0x33c - SW_PAD_CTL_PAD_GPIO_B0_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_04: SW_PAD_CTL_PAD_GPIO_B0_04,
    #[doc = "0x340 - SW_PAD_CTL_PAD_GPIO_B0_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_05: SW_PAD_CTL_PAD_GPIO_B0_05,
    #[doc = "0x344 - SW_PAD_CTL_PAD_GPIO_B0_06 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_06: SW_PAD_CTL_PAD_GPIO_B0_06,
    #[doc = "0x348 - SW_PAD_CTL_PAD_GPIO_B0_07 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_07: SW_PAD_CTL_PAD_GPIO_B0_07,
    #[doc = "0x34c - SW_PAD_CTL_PAD_GPIO_B0_08 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_08: SW_PAD_CTL_PAD_GPIO_B0_08,
    #[doc = "0x350 - SW_PAD_CTL_PAD_GPIO_B0_09 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_09: SW_PAD_CTL_PAD_GPIO_B0_09,
    #[doc = "0x354 - SW_PAD_CTL_PAD_GPIO_B0_10 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_10: SW_PAD_CTL_PAD_GPIO_B0_10,
    #[doc = "0x358 - SW_PAD_CTL_PAD_GPIO_B0_11 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_11: SW_PAD_CTL_PAD_GPIO_B0_11,
    #[doc = "0x35c - SW_PAD_CTL_PAD_GPIO_B0_12 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_12: SW_PAD_CTL_PAD_GPIO_B0_12,
    #[doc = "0x360 - SW_PAD_CTL_PAD_GPIO_B0_13 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_13: SW_PAD_CTL_PAD_GPIO_B0_13,
    #[doc = "0x364 - SW_PAD_CTL_PAD_GPIO_B0_14 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_14: SW_PAD_CTL_PAD_GPIO_B0_14,
    #[doc = "0x368 - SW_PAD_CTL_PAD_GPIO_B0_15 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b0_15: SW_PAD_CTL_PAD_GPIO_B0_15,
    #[doc = "0x36c - SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_00: SW_PAD_CTL_PAD_GPIO_B1_00,
    #[doc = "0x370 - SW_PAD_CTL_PAD_GPIO_B1_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_01: SW_PAD_CTL_PAD_GPIO_B1_01,
    #[doc = "0x374 - SW_PAD_CTL_PAD_GPIO_B1_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_02: SW_PAD_CTL_PAD_GPIO_B1_02,
    #[doc = "0x378 - SW_PAD_CTL_PAD_GPIO_B1_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_03: SW_PAD_CTL_PAD_GPIO_B1_03,
    #[doc = "0x37c - SW_PAD_CTL_PAD_GPIO_B1_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_04: SW_PAD_CTL_PAD_GPIO_B1_04,
    #[doc = "0x380 - SW_PAD_CTL_PAD_GPIO_B1_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_05: SW_PAD_CTL_PAD_GPIO_B1_05,
    #[doc = "0x384 - SW_PAD_CTL_PAD_GPIO_B1_06 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_06: SW_PAD_CTL_PAD_GPIO_B1_06,
    #[doc = "0x388 - SW_PAD_CTL_PAD_GPIO_B1_07 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_07: SW_PAD_CTL_PAD_GPIO_B1_07,
    #[doc = "0x38c - SW_PAD_CTL_PAD_GPIO_B1_08 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_08: SW_PAD_CTL_PAD_GPIO_B1_08,
    #[doc = "0x390 - SW_PAD_CTL_PAD_GPIO_B1_09 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_09: SW_PAD_CTL_PAD_GPIO_B1_09,
    #[doc = "0x394 - SW_PAD_CTL_PAD_GPIO_B1_10 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_10: SW_PAD_CTL_PAD_GPIO_B1_10,
    #[doc = "0x398 - SW_PAD_CTL_PAD_GPIO_B1_11 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_11: SW_PAD_CTL_PAD_GPIO_B1_11,
    #[doc = "0x39c - SW_PAD_CTL_PAD_GPIO_B1_12 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_12: SW_PAD_CTL_PAD_GPIO_B1_12,
    #[doc = "0x3a0 - SW_PAD_CTL_PAD_GPIO_B1_13 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_13: SW_PAD_CTL_PAD_GPIO_B1_13,
    #[doc = "0x3a4 - SW_PAD_CTL_PAD_GPIO_B1_14 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_14: SW_PAD_CTL_PAD_GPIO_B1_14,
    #[doc = "0x3a8 - SW_PAD_CTL_PAD_GPIO_B1_15 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_b1_15: SW_PAD_CTL_PAD_GPIO_B1_15,
    #[doc = "0x3ac - SW_PAD_CTL_PAD_GPIO_SD_B0_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b0_00: SW_PAD_CTL_PAD_GPIO_SD_B0_00,
    #[doc = "0x3b0 - SW_PAD_CTL_PAD_GPIO_SD_B0_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b0_01: SW_PAD_CTL_PAD_GPIO_SD_B0_01,
    #[doc = "0x3b4 - SW_PAD_CTL_PAD_GPIO_SD_B0_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b0_02: SW_PAD_CTL_PAD_GPIO_SD_B0_02,
    #[doc = "0x3b8 - SW_PAD_CTL_PAD_GPIO_SD_B0_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b0_03: SW_PAD_CTL_PAD_GPIO_SD_B0_03,
    #[doc = "0x3bc - SW_PAD_CTL_PAD_GPIO_SD_B0_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b0_04: SW_PAD_CTL_PAD_GPIO_SD_B0_04,
    #[doc = "0x3c0 - SW_PAD_CTL_PAD_GPIO_SD_B0_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b0_05: SW_PAD_CTL_PAD_GPIO_SD_B0_05,
    #[doc = "0x3c4 - SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_00: SW_PAD_CTL_PAD_GPIO_SD_B1_00,
    #[doc = "0x3c8 - SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_01: SW_PAD_CTL_PAD_GPIO_SD_B1_01,
    #[doc = "0x3cc - SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_02: SW_PAD_CTL_PAD_GPIO_SD_B1_02,
    #[doc = "0x3d0 - SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_03: SW_PAD_CTL_PAD_GPIO_SD_B1_03,
    #[doc = "0x3d4 - SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_04: SW_PAD_CTL_PAD_GPIO_SD_B1_04,
    #[doc = "0x3d8 - SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_05: SW_PAD_CTL_PAD_GPIO_SD_B1_05,
    #[doc = "0x3dc - SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_06: SW_PAD_CTL_PAD_GPIO_SD_B1_06,
    #[doc = "0x3e0 - SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_07: SW_PAD_CTL_PAD_GPIO_SD_B1_07,
    #[doc = "0x3e4 - SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_08: SW_PAD_CTL_PAD_GPIO_SD_B1_08,
    #[doc = "0x3e8 - SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_09: SW_PAD_CTL_PAD_GPIO_SD_B1_09,
    #[doc = "0x3ec - SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_10: SW_PAD_CTL_PAD_GPIO_SD_B1_10,
    #[doc = "0x3f0 - SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_sd_b1_11: SW_PAD_CTL_PAD_GPIO_SD_B1_11,
    #[doc = "0x3f4 - ANATOP_USB_OTG1_ID_SELECT_INPUT DAISY Register"]
    pub anatop_usb_otg1_id_select_input: ANATOP_USB_OTG1_ID_SELECT_INPUT,
    #[doc = "0x3f8 - ANATOP_USB_OTG2_ID_SELECT_INPUT DAISY Register"]
    pub anatop_usb_otg2_id_select_input: ANATOP_USB_OTG2_ID_SELECT_INPUT,
    #[doc = "0x3fc - CCM_PMIC_READY_SELECT_INPUT DAISY Register"]
    pub ccm_pmic_ready_select_input: CCM_PMIC_READY_SELECT_INPUT,
    #[doc = "0x400 - CSI_DATA02_SELECT_INPUT DAISY Register"]
    pub csi_data02_select_input: CSI_DATA02_SELECT_INPUT,
    #[doc = "0x404 - CSI_DATA03_SELECT_INPUT DAISY Register"]
    pub csi_data03_select_input: CSI_DATA03_SELECT_INPUT,
    #[doc = "0x408 - CSI_DATA04_SELECT_INPUT DAISY Register"]
    pub csi_data04_select_input: CSI_DATA04_SELECT_INPUT,
    #[doc = "0x40c - CSI_DATA05_SELECT_INPUT DAISY Register"]
    pub csi_data05_select_input: CSI_DATA05_SELECT_INPUT,
    #[doc = "0x410 - CSI_DATA06_SELECT_INPUT DAISY Register"]
    pub csi_data06_select_input: CSI_DATA06_SELECT_INPUT,
    #[doc = "0x414 - CSI_DATA07_SELECT_INPUT DAISY Register"]
    pub csi_data07_select_input: CSI_DATA07_SELECT_INPUT,
    #[doc = "0x418 - CSI_DATA08_SELECT_INPUT DAISY Register"]
    pub csi_data08_select_input: CSI_DATA08_SELECT_INPUT,
    #[doc = "0x41c - CSI_DATA09_SELECT_INPUT DAISY Register"]
    pub csi_data09_select_input: CSI_DATA09_SELECT_INPUT,
    #[doc = "0x420 - CSI_HSYNC_SELECT_INPUT DAISY Register"]
    pub csi_hsync_select_input: CSI_HSYNC_SELECT_INPUT,
    #[doc = "0x424 - CSI_PIXCLK_SELECT_INPUT DAISY Register"]
    pub csi_pixclk_select_input: CSI_PIXCLK_SELECT_INPUT,
    #[doc = "0x428 - CSI_VSYNC_SELECT_INPUT DAISY Register"]
    pub csi_vsync_select_input: CSI_VSYNC_SELECT_INPUT,
    #[doc = "0x42c - ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
    pub enet_ipg_clk_rmii_select_input: ENET_IPG_CLK_RMII_SELECT_INPUT,
    #[doc = "0x430 - ENET_MDIO_SELECT_INPUT DAISY Register"]
    pub enet_mdio_select_input: ENET_MDIO_SELECT_INPUT,
    #[doc = "0x434 - ENET0_RXDATA_SELECT_INPUT DAISY Register"]
    pub enet0_rxdata_select_input: ENET0_RXDATA_SELECT_INPUT,
    #[doc = "0x438 - ENET1_RXDATA_SELECT_INPUT DAISY Register"]
    pub enet1_rxdata_select_input: ENET1_RXDATA_SELECT_INPUT,
    #[doc = "0x43c - ENET_RXEN_SELECT_INPUT DAISY Register"]
    pub enet_rxen_select_input: ENET_RXEN_SELECT_INPUT,
    #[doc = "0x440 - ENET_RXERR_SELECT_INPUT DAISY Register"]
    pub enet_rxerr_select_input: ENET_RXERR_SELECT_INPUT,
    #[doc = "0x444 - ENET0_TIMER_SELECT_INPUT DAISY Register"]
    pub enet0_timer_select_input: ENET0_TIMER_SELECT_INPUT,
    #[doc = "0x448 - ENET_TXCLK_SELECT_INPUT DAISY Register"]
    pub enet_txclk_select_input: ENET_TXCLK_SELECT_INPUT,
    #[doc = "0x44c - FLEXCAN1_RX_SELECT_INPUT DAISY Register"]
    pub flexcan1_rx_select_input: FLEXCAN1_RX_SELECT_INPUT,
    #[doc = "0x450 - FLEXCAN2_RX_SELECT_INPUT DAISY Register"]
    pub flexcan2_rx_select_input: FLEXCAN2_RX_SELECT_INPUT,
    #[doc = "0x454 - FLEXPWM1_PWMA3_SELECT_INPUT DAISY Register"]
    pub flexpwm1_pwma3_select_input: FLEXPWM1_PWMA3_SELECT_INPUT,
    #[doc = "0x458 - FLEXPWM1_PWMA0_SELECT_INPUT DAISY Register"]
    pub flexpwm1_pwma0_select_input: FLEXPWM1_PWMA0_SELECT_INPUT,
    #[doc = "0x45c - FLEXPWM1_PWMA1_SELECT_INPUT DAISY Register"]
    pub flexpwm1_pwma1_select_input: FLEXPWM1_PWMA1_SELECT_INPUT,
    #[doc = "0x460 - FLEXPWM1_PWMA2_SELECT_INPUT DAISY Register"]
    pub flexpwm1_pwma2_select_input: FLEXPWM1_PWMA2_SELECT_INPUT,
    #[doc = "0x464 - FLEXPWM1_PWMB3_SELECT_INPUT DAISY Register"]
    pub flexpwm1_pwmb3_select_input: FLEXPWM1_PWMB3_SELECT_INPUT,
    #[doc = "0x468 - FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register"]
    pub flexpwm1_pwmb0_select_input: FLEXPWM1_PWMB0_SELECT_INPUT,
    #[doc = "0x46c - FLEXPWM1_PWMB1_SELECT_INPUT DAISY Register"]
    pub flexpwm1_pwmb1_select_input: FLEXPWM1_PWMB1_SELECT_INPUT,
    #[doc = "0x470 - FLEXPWM1_PWMB2_SELECT_INPUT DAISY Register"]
    pub flexpwm1_pwmb2_select_input: FLEXPWM1_PWMB2_SELECT_INPUT,
    #[doc = "0x474 - FLEXPWM2_PWMA3_SELECT_INPUT DAISY Register"]
    pub flexpwm2_pwma3_select_input: FLEXPWM2_PWMA3_SELECT_INPUT,
    #[doc = "0x478 - FLEXPWM2_PWMA0_SELECT_INPUT DAISY Register"]
    pub flexpwm2_pwma0_select_input: FLEXPWM2_PWMA0_SELECT_INPUT,
    #[doc = "0x47c - FLEXPWM2_PWMA1_SELECT_INPUT DAISY Register"]
    pub flexpwm2_pwma1_select_input: FLEXPWM2_PWMA1_SELECT_INPUT,
    #[doc = "0x480 - FLEXPWM2_PWMA2_SELECT_INPUT DAISY Register"]
    pub flexpwm2_pwma2_select_input: FLEXPWM2_PWMA2_SELECT_INPUT,
    #[doc = "0x484 - FLEXPWM2_PWMB3_SELECT_INPUT DAISY Register"]
    pub flexpwm2_pwmb3_select_input: FLEXPWM2_PWMB3_SELECT_INPUT,
    #[doc = "0x488 - FLEXPWM2_PWMB0_SELECT_INPUT DAISY Register"]
    pub flexpwm2_pwmb0_select_input: FLEXPWM2_PWMB0_SELECT_INPUT,
    #[doc = "0x48c - FLEXPWM2_PWMB1_SELECT_INPUT DAISY Register"]
    pub flexpwm2_pwmb1_select_input: FLEXPWM2_PWMB1_SELECT_INPUT,
    #[doc = "0x490 - FLEXPWM2_PWMB2_SELECT_INPUT DAISY Register"]
    pub flexpwm2_pwmb2_select_input: FLEXPWM2_PWMB2_SELECT_INPUT,
    #[doc = "0x494 - FLEXPWM4_PWMA0_SELECT_INPUT DAISY Register"]
    pub flexpwm4_pwma0_select_input: FLEXPWM4_PWMA0_SELECT_INPUT,
    #[doc = "0x498 - FLEXPWM4_PWMA1_SELECT_INPUT DAISY Register"]
    pub flexpwm4_pwma1_select_input: FLEXPWM4_PWMA1_SELECT_INPUT,
    #[doc = "0x49c - FLEXPWM4_PWMA2_SELECT_INPUT DAISY Register"]
    pub flexpwm4_pwma2_select_input: FLEXPWM4_PWMA2_SELECT_INPUT,
    #[doc = "0x4a0 - FLEXPWM4_PWMA3_SELECT_INPUT DAISY Register"]
    pub flexpwm4_pwma3_select_input: FLEXPWM4_PWMA3_SELECT_INPUT,
    #[doc = "0x4a4 - FLEXSPIA_DQS_SELECT_INPUT DAISY Register"]
    pub flexspia_dqs_select_input: FLEXSPIA_DQS_SELECT_INPUT,
    #[doc = "0x4a8 - FLEXSPIA_DATA0_SELECT_INPUT DAISY Register"]
    pub flexspia_data0_select_input: FLEXSPIA_DATA0_SELECT_INPUT,
    #[doc = "0x4ac - FLEXSPIA_DATA1_SELECT_INPUT DAISY Register"]
    pub flexspia_data1_select_input: FLEXSPIA_DATA1_SELECT_INPUT,
    #[doc = "0x4b0 - FLEXSPIA_DATA2_SELECT_INPUT DAISY Register"]
    pub flexspia_data2_select_input: FLEXSPIA_DATA2_SELECT_INPUT,
    #[doc = "0x4b4 - FLEXSPIA_DATA3_SELECT_INPUT DAISY Register"]
    pub flexspia_data3_select_input: FLEXSPIA_DATA3_SELECT_INPUT,
    #[doc = "0x4b8 - FLEXSPIB_DATA0_SELECT_INPUT DAISY Register"]
    pub flexspib_data0_select_input: FLEXSPIB_DATA0_SELECT_INPUT,
    #[doc = "0x4bc - FLEXSPIB_DATA1_SELECT_INPUT DAISY Register"]
    pub flexspib_data1_select_input: FLEXSPIB_DATA1_SELECT_INPUT,
    #[doc = "0x4c0 - FLEXSPIB_DATA2_SELECT_INPUT DAISY Register"]
    pub flexspib_data2_select_input: FLEXSPIB_DATA2_SELECT_INPUT,
    #[doc = "0x4c4 - FLEXSPIB_DATA3_SELECT_INPUT DAISY Register"]
    pub flexspib_data3_select_input: FLEXSPIB_DATA3_SELECT_INPUT,
    #[doc = "0x4c8 - FLEXSPIA_SCK_SELECT_INPUT DAISY Register"]
    pub flexspia_sck_select_input: FLEXSPIA_SCK_SELECT_INPUT,
    #[doc = "0x4cc - LPI2C1_SCL_SELECT_INPUT DAISY Register"]
    pub lpi2c1_scl_select_input: LPI2C1_SCL_SELECT_INPUT,
    #[doc = "0x4d0 - LPI2C1_SDA_SELECT_INPUT DAISY Register"]
    pub lpi2c1_sda_select_input: LPI2C1_SDA_SELECT_INPUT,
    #[doc = "0x4d4 - LPI2C2_SCL_SELECT_INPUT DAISY Register"]
    pub lpi2c2_scl_select_input: LPI2C2_SCL_SELECT_INPUT,
    #[doc = "0x4d8 - LPI2C2_SDA_SELECT_INPUT DAISY Register"]
    pub lpi2c2_sda_select_input: LPI2C2_SDA_SELECT_INPUT,
    #[doc = "0x4dc - LPI2C3_SCL_SELECT_INPUT DAISY Register"]
    pub lpi2c3_scl_select_input: LPI2C3_SCL_SELECT_INPUT,
    #[doc = "0x4e0 - LPI2C3_SDA_SELECT_INPUT DAISY Register"]
    pub lpi2c3_sda_select_input: LPI2C3_SDA_SELECT_INPUT,
    #[doc = "0x4e4 - LPI2C4_SCL_SELECT_INPUT DAISY Register"]
    pub lpi2c4_scl_select_input: LPI2C4_SCL_SELECT_INPUT,
    #[doc = "0x4e8 - LPI2C4_SDA_SELECT_INPUT DAISY Register"]
    pub lpi2c4_sda_select_input: LPI2C4_SDA_SELECT_INPUT,
    #[doc = "0x4ec - LPSPI1_PCS0_SELECT_INPUT DAISY Register"]
    pub lpspi1_pcs0_select_input: LPSPI1_PCS0_SELECT_INPUT,
    #[doc = "0x4f0 - LPSPI1_SCK_SELECT_INPUT DAISY Register"]
    pub lpspi1_sck_select_input: LPSPI1_SCK_SELECT_INPUT,
    #[doc = "0x4f4 - LPSPI1_SDI_SELECT_INPUT DAISY Register"]
    pub lpspi1_sdi_select_input: LPSPI1_SDI_SELECT_INPUT,
    #[doc = "0x4f8 - LPSPI1_SDO_SELECT_INPUT DAISY Register"]
    pub lpspi1_sdo_select_input: LPSPI1_SDO_SELECT_INPUT,
    #[doc = "0x4fc - LPSPI2_PCS0_SELECT_INPUT DAISY Register"]
    pub lpspi2_pcs0_select_input: LPSPI2_PCS0_SELECT_INPUT,
    #[doc = "0x500 - LPSPI2_SCK_SELECT_INPUT DAISY Register"]
    pub lpspi2_sck_select_input: LPSPI2_SCK_SELECT_INPUT,
    #[doc = "0x504 - LPSPI2_SDI_SELECT_INPUT DAISY Register"]
    pub lpspi2_sdi_select_input: LPSPI2_SDI_SELECT_INPUT,
    #[doc = "0x508 - LPSPI2_SDO_SELECT_INPUT DAISY Register"]
    pub lpspi2_sdo_select_input: LPSPI2_SDO_SELECT_INPUT,
    #[doc = "0x50c - LPSPI3_PCS0_SELECT_INPUT DAISY Register"]
    pub lpspi3_pcs0_select_input: LPSPI3_PCS0_SELECT_INPUT,
    #[doc = "0x510 - LPSPI3_SCK_SELECT_INPUT DAISY Register"]
    pub lpspi3_sck_select_input: LPSPI3_SCK_SELECT_INPUT,
    #[doc = "0x514 - LPSPI3_SDI_SELECT_INPUT DAISY Register"]
    pub lpspi3_sdi_select_input: LPSPI3_SDI_SELECT_INPUT,
    #[doc = "0x518 - LPSPI3_SDO_SELECT_INPUT DAISY Register"]
    pub lpspi3_sdo_select_input: LPSPI3_SDO_SELECT_INPUT,
    #[doc = "0x51c - LPSPI4_PCS0_SELECT_INPUT DAISY Register"]
    pub lpspi4_pcs0_select_input: LPSPI4_PCS0_SELECT_INPUT,
    #[doc = "0x520 - LPSPI4_SCK_SELECT_INPUT DAISY Register"]
    pub lpspi4_sck_select_input: LPSPI4_SCK_SELECT_INPUT,
    #[doc = "0x524 - LPSPI4_SDI_SELECT_INPUT DAISY Register"]
    pub lpspi4_sdi_select_input: LPSPI4_SDI_SELECT_INPUT,
    #[doc = "0x528 - LPSPI4_SDO_SELECT_INPUT DAISY Register"]
    pub lpspi4_sdo_select_input: LPSPI4_SDO_SELECT_INPUT,
    #[doc = "0x52c - LPUART2_RX_SELECT_INPUT DAISY Register"]
    pub lpuart2_rx_select_input: LPUART2_RX_SELECT_INPUT,
    #[doc = "0x530 - LPUART2_TX_SELECT_INPUT DAISY Register"]
    pub lpuart2_tx_select_input: LPUART2_TX_SELECT_INPUT,
    #[doc = "0x534 - LPUART3_CTS_B_SELECT_INPUT DAISY Register"]
    pub lpuart3_cts_b_select_input: LPUART3_CTS_B_SELECT_INPUT,
    #[doc = "0x538 - LPUART3_RX_SELECT_INPUT DAISY Register"]
    pub lpuart3_rx_select_input: LPUART3_RX_SELECT_INPUT,
    #[doc = "0x53c - LPUART3_TX_SELECT_INPUT DAISY Register"]
    pub lpuart3_tx_select_input: LPUART3_TX_SELECT_INPUT,
    #[doc = "0x540 - LPUART4_RX_SELECT_INPUT DAISY Register"]
    pub lpuart4_rx_select_input: LPUART4_RX_SELECT_INPUT,
    #[doc = "0x544 - LPUART4_TX_SELECT_INPUT DAISY Register"]
    pub lpuart4_tx_select_input: LPUART4_TX_SELECT_INPUT,
    #[doc = "0x548 - LPUART5_RX_SELECT_INPUT DAISY Register"]
    pub lpuart5_rx_select_input: LPUART5_RX_SELECT_INPUT,
    #[doc = "0x54c - LPUART5_TX_SELECT_INPUT DAISY Register"]
    pub lpuart5_tx_select_input: LPUART5_TX_SELECT_INPUT,
    #[doc = "0x550 - LPUART6_RX_SELECT_INPUT DAISY Register"]
    pub lpuart6_rx_select_input: LPUART6_RX_SELECT_INPUT,
    #[doc = "0x554 - LPUART6_TX_SELECT_INPUT DAISY Register"]
    pub lpuart6_tx_select_input: LPUART6_TX_SELECT_INPUT,
    #[doc = "0x558 - LPUART7_RX_SELECT_INPUT DAISY Register"]
    pub lpuart7_rx_select_input: LPUART7_RX_SELECT_INPUT,
    #[doc = "0x55c - LPUART7_TX_SELECT_INPUT DAISY Register"]
    pub lpuart7_tx_select_input: LPUART7_TX_SELECT_INPUT,
    #[doc = "0x560 - LPUART8_RX_SELECT_INPUT DAISY Register"]
    pub lpuart8_rx_select_input: LPUART8_RX_SELECT_INPUT,
    #[doc = "0x564 - LPUART8_TX_SELECT_INPUT DAISY Register"]
    pub lpuart8_tx_select_input: LPUART8_TX_SELECT_INPUT,
    #[doc = "0x568 - NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
    pub nmi_select_input: NMI_SELECT_INPUT,
    #[doc = "0x56c - QTIMER2_TIMER0_SELECT_INPUT DAISY Register"]
    pub qtimer2_timer0_select_input: QTIMER2_TIMER0_SELECT_INPUT,
    #[doc = "0x570 - QTIMER2_TIMER1_SELECT_INPUT DAISY Register"]
    pub qtimer2_timer1_select_input: QTIMER2_TIMER1_SELECT_INPUT,
    #[doc = "0x574 - QTIMER2_TIMER2_SELECT_INPUT DAISY Register"]
    pub qtimer2_timer2_select_input: QTIMER2_TIMER2_SELECT_INPUT,
    #[doc = "0x578 - QTIMER2_TIMER3_SELECT_INPUT DAISY Register"]
    pub qtimer2_timer3_select_input: QTIMER2_TIMER3_SELECT_INPUT,
    #[doc = "0x57c - QTIMER3_TIMER0_SELECT_INPUT DAISY Register"]
    pub qtimer3_timer0_select_input: QTIMER3_TIMER0_SELECT_INPUT,
    #[doc = "0x580 - QTIMER3_TIMER1_SELECT_INPUT DAISY Register"]
    pub qtimer3_timer1_select_input: QTIMER3_TIMER1_SELECT_INPUT,
    #[doc = "0x584 - QTIMER3_TIMER2_SELECT_INPUT DAISY Register"]
    pub qtimer3_timer2_select_input: QTIMER3_TIMER2_SELECT_INPUT,
    #[doc = "0x588 - QTIMER3_TIMER3_SELECT_INPUT DAISY Register"]
    pub qtimer3_timer3_select_input: QTIMER3_TIMER3_SELECT_INPUT,
    #[doc = "0x58c - SAI1_MCLK2_SELECT_INPUT DAISY Register"]
    pub sai1_mclk2_select_input: SAI1_MCLK2_SELECT_INPUT,
    #[doc = "0x590 - SAI1_RX_BCLK_SELECT_INPUT DAISY Register"]
    pub sai1_rx_bclk_select_input: SAI1_RX_BCLK_SELECT_INPUT,
    #[doc = "0x594 - SAI1_RX_DATA0_SELECT_INPUT DAISY Register"]
    pub sai1_rx_data0_select_input: SAI1_RX_DATA0_SELECT_INPUT,
    #[doc = "0x598 - SAI1_RX_DATA1_SELECT_INPUT DAISY Register"]
    pub sai1_rx_data1_select_input: SAI1_RX_DATA1_SELECT_INPUT,
    #[doc = "0x59c - SAI1_RX_DATA2_SELECT_INPUT DAISY Register"]
    pub sai1_rx_data2_select_input: SAI1_RX_DATA2_SELECT_INPUT,
    #[doc = "0x5a0 - SAI1_RX_DATA3_SELECT_INPUT DAISY Register"]
    pub sai1_rx_data3_select_input: SAI1_RX_DATA3_SELECT_INPUT,
    #[doc = "0x5a4 - SAI1_RX_SYNC_SELECT_INPUT DAISY Register"]
    pub sai1_rx_sync_select_input: SAI1_RX_SYNC_SELECT_INPUT,
    #[doc = "0x5a8 - SAI1_TX_BCLK_SELECT_INPUT DAISY Register"]
    pub sai1_tx_bclk_select_input: SAI1_TX_BCLK_SELECT_INPUT,
    #[doc = "0x5ac - SAI1_TX_SYNC_SELECT_INPUT DAISY Register"]
    pub sai1_tx_sync_select_input: SAI1_TX_SYNC_SELECT_INPUT,
    #[doc = "0x5b0 - SAI2_MCLK2_SELECT_INPUT DAISY Register"]
    pub sai2_mclk2_select_input: SAI2_MCLK2_SELECT_INPUT,
    #[doc = "0x5b4 - SAI2_RX_BCLK_SELECT_INPUT DAISY Register"]
    pub sai2_rx_bclk_select_input: SAI2_RX_BCLK_SELECT_INPUT,
    #[doc = "0x5b8 - SAI2_RX_DATA0_SELECT_INPUT DAISY Register"]
    pub sai2_rx_data0_select_input: SAI2_RX_DATA0_SELECT_INPUT,
    #[doc = "0x5bc - SAI2_RX_SYNC_SELECT_INPUT DAISY Register"]
    pub sai2_rx_sync_select_input: SAI2_RX_SYNC_SELECT_INPUT,
    #[doc = "0x5c0 - SAI2_TX_BCLK_SELECT_INPUT DAISY Register"]
    pub sai2_tx_bclk_select_input: SAI2_TX_BCLK_SELECT_INPUT,
    #[doc = "0x5c4 - SAI2_TX_SYNC_SELECT_INPUT DAISY Register"]
    pub sai2_tx_sync_select_input: SAI2_TX_SYNC_SELECT_INPUT,
    #[doc = "0x5c8 - SPDIF_IN_SELECT_INPUT DAISY Register"]
    pub spdif_in_select_input: SPDIF_IN_SELECT_INPUT,
    #[doc = "0x5cc - USB_OTG2_OC_SELECT_INPUT DAISY Register"]
    pub usb_otg2_oc_select_input: USB_OTG2_OC_SELECT_INPUT,
    #[doc = "0x5d0 - USB_OTG1_OC_SELECT_INPUT DAISY Register"]
    pub usb_otg1_oc_select_input: USB_OTG1_OC_SELECT_INPUT,
    #[doc = "0x5d4 - USDHC1_CD_B_SELECT_INPUT DAISY Register"]
    pub usdhc1_cd_b_select_input: USDHC1_CD_B_SELECT_INPUT,
    #[doc = "0x5d8 - USDHC1_WP_SELECT_INPUT DAISY Register"]
    pub usdhc1_wp_select_input: USDHC1_WP_SELECT_INPUT,
    #[doc = "0x5dc - USDHC2_CLK_SELECT_INPUT DAISY Register"]
    pub usdhc2_clk_select_input: USDHC2_CLK_SELECT_INPUT,
    #[doc = "0x5e0 - USDHC2_CD_B_SELECT_INPUT DAISY Register"]
    pub usdhc2_cd_b_select_input: USDHC2_CD_B_SELECT_INPUT,
    #[doc = "0x5e4 - USDHC2_CMD_SELECT_INPUT DAISY Register"]
    pub usdhc2_cmd_select_input: USDHC2_CMD_SELECT_INPUT,
    #[doc = "0x5e8 - USDHC2_DATA0_SELECT_INPUT DAISY Register"]
    pub usdhc2_data0_select_input: USDHC2_DATA0_SELECT_INPUT,
    #[doc = "0x5ec - USDHC2_DATA1_SELECT_INPUT DAISY Register"]
    pub usdhc2_data1_select_input: USDHC2_DATA1_SELECT_INPUT,
    #[doc = "0x5f0 - USDHC2_DATA2_SELECT_INPUT DAISY Register"]
    pub usdhc2_data2_select_input: USDHC2_DATA2_SELECT_INPUT,
    #[doc = "0x5f4 - USDHC2_DATA3_SELECT_INPUT DAISY Register"]
    pub usdhc2_data3_select_input: USDHC2_DATA3_SELECT_INPUT,
    #[doc = "0x5f8 - USDHC2_DATA4_SELECT_INPUT DAISY Register"]
    pub usdhc2_data4_select_input: USDHC2_DATA4_SELECT_INPUT,
    #[doc = "0x5fc - USDHC2_DATA5_SELECT_INPUT DAISY Register"]
    pub usdhc2_data5_select_input: USDHC2_DATA5_SELECT_INPUT,
    #[doc = "0x600 - USDHC2_DATA6_SELECT_INPUT DAISY Register"]
    pub usdhc2_data6_select_input: USDHC2_DATA6_SELECT_INPUT,
    #[doc = "0x604 - USDHC2_DATA7_SELECT_INPUT DAISY Register"]
    pub usdhc2_data7_select_input: USDHC2_DATA7_SELECT_INPUT,
    #[doc = "0x608 - USDHC2_WP_SELECT_INPUT DAISY Register"]
    pub usdhc2_wp_select_input: USDHC2_WP_SELECT_INPUT,
    #[doc = "0x60c - XBAR1_IN02_SELECT_INPUT DAISY Register"]
    pub xbar1_in02_select_input: XBAR1_IN02_SELECT_INPUT,
    #[doc = "0x610 - XBAR1_IN03_SELECT_INPUT DAISY Register"]
    pub xbar1_in03_select_input: XBAR1_IN03_SELECT_INPUT,
    #[doc = "0x614 - XBAR1_IN04_SELECT_INPUT DAISY Register"]
    pub xbar1_in04_select_input: XBAR1_IN04_SELECT_INPUT,
    #[doc = "0x618 - XBAR1_IN05_SELECT_INPUT DAISY Register"]
    pub xbar1_in05_select_input: XBAR1_IN05_SELECT_INPUT,
    #[doc = "0x61c - XBAR1_IN06_SELECT_INPUT DAISY Register"]
    pub xbar1_in06_select_input: XBAR1_IN06_SELECT_INPUT,
    #[doc = "0x620 - XBAR1_IN07_SELECT_INPUT DAISY Register"]
    pub xbar1_in07_select_input: XBAR1_IN07_SELECT_INPUT,
    #[doc = "0x624 - XBAR1_IN08_SELECT_INPUT DAISY Register"]
    pub xbar1_in08_select_input: XBAR1_IN08_SELECT_INPUT,
    #[doc = "0x628 - XBAR1_IN09_SELECT_INPUT DAISY Register"]
    pub xbar1_in09_select_input: XBAR1_IN09_SELECT_INPUT,
    #[doc = "0x62c - XBAR1_IN17_SELECT_INPUT DAISY Register"]
    pub xbar1_in17_select_input: XBAR1_IN17_SELECT_INPUT,
    #[doc = "0x630 - XBAR1_IN18_SELECT_INPUT DAISY Register"]
    pub xbar1_in18_select_input: XBAR1_IN18_SELECT_INPUT,
    #[doc = "0x634 - XBAR1_IN20_SELECT_INPUT DAISY Register"]
    pub xbar1_in20_select_input: XBAR1_IN20_SELECT_INPUT,
    #[doc = "0x638 - XBAR1_IN22_SELECT_INPUT DAISY Register"]
    pub xbar1_in22_select_input: XBAR1_IN22_SELECT_INPUT,
    #[doc = "0x63c - XBAR1_IN23_SELECT_INPUT DAISY Register"]
    pub xbar1_in23_select_input: XBAR1_IN23_SELECT_INPUT,
    #[doc = "0x640 - XBAR1_IN24_SELECT_INPUT DAISY Register"]
    pub xbar1_in24_select_input: XBAR1_IN24_SELECT_INPUT,
    #[doc = "0x644 - XBAR1_IN14_SELECT_INPUT DAISY Register"]
    pub xbar1_in14_select_input: XBAR1_IN14_SELECT_INPUT,
    #[doc = "0x648 - XBAR1_IN15_SELECT_INPUT DAISY Register"]
    pub xbar1_in15_select_input: XBAR1_IN15_SELECT_INPUT,
    #[doc = "0x64c - XBAR1_IN16_SELECT_INPUT DAISY Register"]
    pub xbar1_in16_select_input: XBAR1_IN16_SELECT_INPUT,
    #[doc = "0x650 - XBAR1_IN25_SELECT_INPUT DAISY Register"]
    pub xbar1_in25_select_input: XBAR1_IN25_SELECT_INPUT,
    #[doc = "0x654 - XBAR1_IN19_SELECT_INPUT DAISY Register"]
    pub xbar1_in19_select_input: XBAR1_IN19_SELECT_INPUT,
    #[doc = "0x658 - XBAR1_IN23_SELECT_INPUT DAISY Register"]
    pub xbar1_in21_select_input: XBAR1_IN21_SELECT_INPUT,
    #[doc = "0x65c - SW_MUX_CTL_PAD_GPIO_SPI_B0_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_00: SW_MUX_CTL_PAD_GPIO_SPI_B0_00,
    #[doc = "0x660 - SW_MUX_CTL_PAD_GPIO_SPI_B0_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_01: SW_MUX_CTL_PAD_GPIO_SPI_B0_01,
    #[doc = "0x664 - SW_MUX_CTL_PAD_GPIO_SPI_B0_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_02: SW_MUX_CTL_PAD_GPIO_SPI_B0_02,
    #[doc = "0x668 - SW_MUX_CTL_PAD_GPIO_SPI_B0_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_03: SW_MUX_CTL_PAD_GPIO_SPI_B0_03,
    #[doc = "0x66c - SW_MUX_CTL_PAD_GPIO_SPI_B0_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_04: SW_MUX_CTL_PAD_GPIO_SPI_B0_04,
    #[doc = "0x670 - SW_MUX_CTL_PAD_GPIO_SPI_B0_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_05: SW_MUX_CTL_PAD_GPIO_SPI_B0_05,
    #[doc = "0x674 - SW_MUX_CTL_PAD_GPIO_SPI_B0_06 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_06: SW_MUX_CTL_PAD_GPIO_SPI_B0_06,
    #[doc = "0x678 - SW_MUX_CTL_PAD_GPIO_SPI_B0_07 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_07: SW_MUX_CTL_PAD_GPIO_SPI_B0_07,
    #[doc = "0x67c - SW_MUX_CTL_PAD_GPIO_SPI_B0_08 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_08: SW_MUX_CTL_PAD_GPIO_SPI_B0_08,
    #[doc = "0x680 - SW_MUX_CTL_PAD_GPIO_SPI_B0_09 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_09: SW_MUX_CTL_PAD_GPIO_SPI_B0_09,
    #[doc = "0x684 - SW_MUX_CTL_PAD_GPIO_SPI_B0_10 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_10: SW_MUX_CTL_PAD_GPIO_SPI_B0_10,
    #[doc = "0x688 - SW_MUX_CTL_PAD_GPIO_SPI_B0_11 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_11: SW_MUX_CTL_PAD_GPIO_SPI_B0_11,
    #[doc = "0x68c - SW_MUX_CTL_PAD_GPIO_SPI_B0_12 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_12: SW_MUX_CTL_PAD_GPIO_SPI_B0_12,
    #[doc = "0x690 - SW_MUX_CTL_PAD_GPIO_SPI_B0_13 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b0_13: SW_MUX_CTL_PAD_GPIO_SPI_B0_13,
    #[doc = "0x694 - SW_MUX_CTL_PAD_GPIO_SPI_B1_00 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b1_00: SW_MUX_CTL_PAD_GPIO_SPI_B1_00,
    #[doc = "0x698 - SW_MUX_CTL_PAD_GPIO_SPI_B1_01 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b1_01: SW_MUX_CTL_PAD_GPIO_SPI_B1_01,
    #[doc = "0x69c - SW_MUX_CTL_PAD_GPIO_SPI_B1_02 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b1_02: SW_MUX_CTL_PAD_GPIO_SPI_B1_02,
    #[doc = "0x6a0 - SW_MUX_CTL_PAD_GPIO_SPI_B1_03 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b1_03: SW_MUX_CTL_PAD_GPIO_SPI_B1_03,
    #[doc = "0x6a4 - SW_MUX_CTL_PAD_GPIO_SPI_B1_04 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b1_04: SW_MUX_CTL_PAD_GPIO_SPI_B1_04,
    #[doc = "0x6a8 - SW_MUX_CTL_PAD_GPIO_SPI_B1_05 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b1_05: SW_MUX_CTL_PAD_GPIO_SPI_B1_05,
    #[doc = "0x6ac - SW_MUX_CTL_PAD_GPIO_SPI_B1_06 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b1_06: SW_MUX_CTL_PAD_GPIO_SPI_B1_06,
    #[doc = "0x6b0 - SW_MUX_CTL_PAD_GPIO_SPI_B1_07 SW MUX Control Register"]
    pub sw_mux_ctl_pad_gpio_spi_b1_07: SW_MUX_CTL_PAD_GPIO_SPI_B1_07,
    #[doc = "0x6b4 - SW_PAD_CTL_PAD_GPIO_SPI_B0_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_00: SW_PAD_CTL_PAD_GPIO_SPI_B0_00,
    #[doc = "0x6b8 - SW_PAD_CTL_PAD_GPIO_SPI_B0_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_01: SW_PAD_CTL_PAD_GPIO_SPI_B0_01,
    #[doc = "0x6bc - SW_PAD_CTL_PAD_GPIO_SPI_B0_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_02: SW_PAD_CTL_PAD_GPIO_SPI_B0_02,
    #[doc = "0x6c0 - SW_PAD_CTL_PAD_GPIO_SPI_B0_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_03: SW_PAD_CTL_PAD_GPIO_SPI_B0_03,
    #[doc = "0x6c4 - SW_PAD_CTL_PAD_GPIO_SPI_B0_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_04: SW_PAD_CTL_PAD_GPIO_SPI_B0_04,
    #[doc = "0x6c8 - SW_PAD_CTL_PAD_GPIO_SPI_B0_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_05: SW_PAD_CTL_PAD_GPIO_SPI_B0_05,
    #[doc = "0x6cc - SW_PAD_CTL_PAD_GPIO_SPI_B0_06 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_06: SW_PAD_CTL_PAD_GPIO_SPI_B0_06,
    #[doc = "0x6d0 - SW_PAD_CTL_PAD_GPIO_SPI_B0_07 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_07: SW_PAD_CTL_PAD_GPIO_SPI_B0_07,
    #[doc = "0x6d4 - SW_PAD_CTL_PAD_GPIO_SPI_B0_08 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_08: SW_PAD_CTL_PAD_GPIO_SPI_B0_08,
    #[doc = "0x6d8 - SW_PAD_CTL_PAD_GPIO_SPI_B0_09 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_09: SW_PAD_CTL_PAD_GPIO_SPI_B0_09,
    #[doc = "0x6dc - SW_PAD_CTL_PAD_GPIO_SPI_B0_10 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_10: SW_PAD_CTL_PAD_GPIO_SPI_B0_10,
    #[doc = "0x6e0 - SW_PAD_CTL_PAD_GPIO_SPI_B0_11 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_11: SW_PAD_CTL_PAD_GPIO_SPI_B0_11,
    #[doc = "0x6e4 - SW_PAD_CTL_PAD_GPIO_SPI_B0_12 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_12: SW_PAD_CTL_PAD_GPIO_SPI_B0_12,
    #[doc = "0x6e8 - SW_PAD_CTL_PAD_GPIO_SPI_B0_13 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b0_13: SW_PAD_CTL_PAD_GPIO_SPI_B0_13,
    #[doc = "0x6ec - SW_PAD_CTL_PAD_GPIO_SPI_B1_00 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b1_00: SW_PAD_CTL_PAD_GPIO_SPI_B1_00,
    #[doc = "0x6f0 - SW_PAD_CTL_PAD_GPIO_SPI_B1_01 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b1_01: SW_PAD_CTL_PAD_GPIO_SPI_B1_01,
    #[doc = "0x6f4 - SW_PAD_CTL_PAD_GPIO_SPI_B1_02 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b1_02: SW_PAD_CTL_PAD_GPIO_SPI_B1_02,
    #[doc = "0x6f8 - SW_PAD_CTL_PAD_GPIO_SPI_B1_03 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b1_03: SW_PAD_CTL_PAD_GPIO_SPI_B1_03,
    #[doc = "0x6fc - SW_PAD_CTL_PAD_GPIO_SPI_B1_04 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b1_04: SW_PAD_CTL_PAD_GPIO_SPI_B1_04,
    #[doc = "0x700 - SW_PAD_CTL_PAD_GPIO_SPI_B1_05 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b1_05: SW_PAD_CTL_PAD_GPIO_SPI_B1_05,
    #[doc = "0x704 - SW_PAD_CTL_PAD_GPIO_SPI_B1_06 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b1_06: SW_PAD_CTL_PAD_GPIO_SPI_B1_06,
    #[doc = "0x708 - SW_PAD_CTL_PAD_GPIO_SPI_B1_07 SW PAD Control Register"]
    pub sw_pad_ctl_pad_gpio_spi_b1_07: SW_PAD_CTL_PAD_GPIO_SPI_B1_07,
    #[doc = "0x70c - ENET2_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
    pub enet2_ipg_clk_rmii_select_input: ENET2_IPG_CLK_RMII_SELECT_INPUT,
    #[doc = "0x710 - ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT DAISY Register"]
    pub enet2_ipp_ind_mac0_mdio_select_input: ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT,
    #[doc = "0x714 - ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register"]
    pub enet2_ipp_ind_mac0_rxdata_select_input_0: ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0,
    #[doc = "0x718 - ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register"]
    pub enet2_ipp_ind_mac0_rxdata_select_input_1: ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1,
    #[doc = "0x71c - ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT DAISY Register"]
    pub enet2_ipp_ind_mac0_rxen_select_input: ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT,
    #[doc = "0x720 - ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT DAISY Register"]
    pub enet2_ipp_ind_mac0_rxerr_select_input: ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT,
    #[doc = "0x724 - ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 DAISY Register"]
    pub enet2_ipp_ind_mac0_timer_select_input_0: ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0,
    #[doc = "0x728 - ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT DAISY Register"]
    pub enet2_ipp_ind_mac0_txclk_select_input: ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT,
    #[doc = "0x72c - FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_dqs_fa_select_input: FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT,
    #[doc = "0x730 - FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_io_fa_bit0_select_input: FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT,
    #[doc = "0x734 - FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_io_fa_bit1_select_input: FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT,
    #[doc = "0x738 - FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_io_fa_bit2_select_input: FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT,
    #[doc = "0x73c - FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_io_fa_bit3_select_input: FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT,
    #[doc = "0x740 - FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_io_fb_bit0_select_input: FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT,
    #[doc = "0x744 - FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_io_fb_bit1_select_input: FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT,
    #[doc = "0x748 - FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_io_fb_bit2_select_input: FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT,
    #[doc = "0x74c - FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_io_fb_bit3_select_input: FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT,
    #[doc = "0x750 - FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_sck_fa_select_input: FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT,
    #[doc = "0x754 - FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT DAISY Register"]
    pub flexspi2_ipp_ind_sck_fb_select_input: FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT,
    #[doc = "0x758 - GPT1_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
    pub gpt1_ipp_ind_capin1_select_input: GPT1_IPP_IND_CAPIN1_SELECT_INPUT,
    #[doc = "0x75c - GPT1_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
    pub gpt1_ipp_ind_capin2_select_input: GPT1_IPP_IND_CAPIN2_SELECT_INPUT,
    #[doc = "0x760 - GPT1_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
    pub gpt1_ipp_ind_clkin_select_input: GPT1_IPP_IND_CLKIN_SELECT_INPUT,
    #[doc = "0x764 - GPT2_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
    pub gpt2_ipp_ind_capin1_select_input: GPT2_IPP_IND_CAPIN1_SELECT_INPUT,
    #[doc = "0x768 - GPT2_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
    pub gpt2_ipp_ind_capin2_select_input: GPT2_IPP_IND_CAPIN2_SELECT_INPUT,
    #[doc = "0x76c - GPT2_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
    pub gpt2_ipp_ind_clkin_select_input: GPT2_IPP_IND_CLKIN_SELECT_INPUT,
    #[doc = "0x770 - SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
    pub sai3_ipg_clk_sai_mclk_select_input_2: SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2,
    #[doc = "0x774 - SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
    pub sai3_ipp_ind_sai_rxbclk_select_input: SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT,
    #[doc = "0x778 - SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
    pub sai3_ipp_ind_sai_rxdata_select_input_0: SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0,
    #[doc = "0x77c - SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
    pub sai3_ipp_ind_sai_rxsync_select_input: SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT,
    #[doc = "0x780 - SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
    pub sai3_ipp_ind_sai_txbclk_select_input: SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT,
    #[doc = "0x784 - SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
    pub sai3_ipp_ind_sai_txsync_select_input: SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT,
    #[doc = "0x788 - SEMC_I_IPP_IND_DQS4_SELECT_INPUT DAISY Register"]
    pub semc_i_ipp_ind_dqs4_select_input: SEMC_I_IPP_IND_DQS4_SELECT_INPUT,
    #[doc = "0x78c - CANFD_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
    pub canfd_ipp_ind_canrx_select_input: CANFD_IPP_IND_CANRX_SELECT_INPUT,
}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_00](sw_mux_ctl_pad_gpio_emc_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_00::R](sw_mux_ctl_pad_gpio_emc_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_00::W](sw_mux_ctl_pad_gpio_emc_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_01](sw_mux_ctl_pad_gpio_emc_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_01::R](sw_mux_ctl_pad_gpio_emc_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_01::W](sw_mux_ctl_pad_gpio_emc_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_02](sw_mux_ctl_pad_gpio_emc_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_02::R](sw_mux_ctl_pad_gpio_emc_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_02::W](sw_mux_ctl_pad_gpio_emc_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_03](sw_mux_ctl_pad_gpio_emc_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_03::R](sw_mux_ctl_pad_gpio_emc_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_03::W](sw_mux_ctl_pad_gpio_emc_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_04](sw_mux_ctl_pad_gpio_emc_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_04::R](sw_mux_ctl_pad_gpio_emc_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_04::W](sw_mux_ctl_pad_gpio_emc_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_05](sw_mux_ctl_pad_gpio_emc_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_05::R](sw_mux_ctl_pad_gpio_emc_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_05::W](sw_mux_ctl_pad_gpio_emc_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_06](sw_mux_ctl_pad_gpio_emc_06) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_06 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_06;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_06::R](sw_mux_ctl_pad_gpio_emc_06::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_06 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_06::W](sw_mux_ctl_pad_gpio_emc_06::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_06 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_06 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_06;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_07](sw_mux_ctl_pad_gpio_emc_07) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_07 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_07;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_07::R](sw_mux_ctl_pad_gpio_emc_07::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_07 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_07::W](sw_mux_ctl_pad_gpio_emc_07::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_07 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_07 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_07;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_08](sw_mux_ctl_pad_gpio_emc_08) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_08 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_08;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_08::R](sw_mux_ctl_pad_gpio_emc_08::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_08 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_08::W](sw_mux_ctl_pad_gpio_emc_08::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_08 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_08 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_08;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_09](sw_mux_ctl_pad_gpio_emc_09) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_09 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_09;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_09::R](sw_mux_ctl_pad_gpio_emc_09::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_09 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_09::W](sw_mux_ctl_pad_gpio_emc_09::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_09 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_09 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_09;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_10 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_10](sw_mux_ctl_pad_gpio_emc_10) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_10 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_10;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_10::R](sw_mux_ctl_pad_gpio_emc_10::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_10 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_10::W](sw_mux_ctl_pad_gpio_emc_10::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_10 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_10 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_10;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_11 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_11](sw_mux_ctl_pad_gpio_emc_11) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_11 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_11;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_11::R](sw_mux_ctl_pad_gpio_emc_11::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_11 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_11::W](sw_mux_ctl_pad_gpio_emc_11::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_11 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_11 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_11;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_12 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_12](sw_mux_ctl_pad_gpio_emc_12) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_12 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_12;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_12::R](sw_mux_ctl_pad_gpio_emc_12::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_12 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_12::W](sw_mux_ctl_pad_gpio_emc_12::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_12 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_12 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_12;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_13 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_13](sw_mux_ctl_pad_gpio_emc_13) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_13 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_13;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_13::R](sw_mux_ctl_pad_gpio_emc_13::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_13 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_13::W](sw_mux_ctl_pad_gpio_emc_13::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_13 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_13 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_13;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_14 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_14](sw_mux_ctl_pad_gpio_emc_14) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_14 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_14;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_14::R](sw_mux_ctl_pad_gpio_emc_14::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_14 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_14::W](sw_mux_ctl_pad_gpio_emc_14::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_14 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_14 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_14;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_15 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_15](sw_mux_ctl_pad_gpio_emc_15) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_15 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_15;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_15::R](sw_mux_ctl_pad_gpio_emc_15::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_15 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_15::W](sw_mux_ctl_pad_gpio_emc_15::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_15 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_15 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_15;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_16](sw_mux_ctl_pad_gpio_emc_16) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_16 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_16;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_16::R](sw_mux_ctl_pad_gpio_emc_16::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_16 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_16::W](sw_mux_ctl_pad_gpio_emc_16::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_16 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_16 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_16;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_17](sw_mux_ctl_pad_gpio_emc_17) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_17 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_17;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_17::R](sw_mux_ctl_pad_gpio_emc_17::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_17 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_17::W](sw_mux_ctl_pad_gpio_emc_17::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_17 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_17 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_17;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_18](sw_mux_ctl_pad_gpio_emc_18) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_18 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_18;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_18::R](sw_mux_ctl_pad_gpio_emc_18::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_18 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_18::W](sw_mux_ctl_pad_gpio_emc_18::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_18 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_18 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_18;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_19](sw_mux_ctl_pad_gpio_emc_19) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_19 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_19;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_19::R](sw_mux_ctl_pad_gpio_emc_19::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_19 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_19::W](sw_mux_ctl_pad_gpio_emc_19::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_19 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_19 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_19;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_20](sw_mux_ctl_pad_gpio_emc_20) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_20 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_20;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_20::R](sw_mux_ctl_pad_gpio_emc_20::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_20 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_20::W](sw_mux_ctl_pad_gpio_emc_20::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_20 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_20 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_20;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_21](sw_mux_ctl_pad_gpio_emc_21) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_21 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_21;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_21::R](sw_mux_ctl_pad_gpio_emc_21::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_21 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_21::W](sw_mux_ctl_pad_gpio_emc_21::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_21 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_21 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_21;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_22](sw_mux_ctl_pad_gpio_emc_22) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_22 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_22;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_22::R](sw_mux_ctl_pad_gpio_emc_22::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_22 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_22::W](sw_mux_ctl_pad_gpio_emc_22::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_22 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_22 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_22;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_23](sw_mux_ctl_pad_gpio_emc_23) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_23 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_23;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_23::R](sw_mux_ctl_pad_gpio_emc_23::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_23 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_23::W](sw_mux_ctl_pad_gpio_emc_23::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_23 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_23 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_23;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_24](sw_mux_ctl_pad_gpio_emc_24) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_24 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_24;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_24::R](sw_mux_ctl_pad_gpio_emc_24::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_24 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_24::W](sw_mux_ctl_pad_gpio_emc_24::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_24 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_24 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_24;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_25](sw_mux_ctl_pad_gpio_emc_25) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_25 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_25;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_25::R](sw_mux_ctl_pad_gpio_emc_25::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_25 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_25::W](sw_mux_ctl_pad_gpio_emc_25::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_25 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_25 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_25;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_26](sw_mux_ctl_pad_gpio_emc_26) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_26 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_26;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_26::R](sw_mux_ctl_pad_gpio_emc_26::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_26 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_26::W](sw_mux_ctl_pad_gpio_emc_26::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_26 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_26 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_26;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_27](sw_mux_ctl_pad_gpio_emc_27) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_27 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_27;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_27::R](sw_mux_ctl_pad_gpio_emc_27::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_27 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_27::W](sw_mux_ctl_pad_gpio_emc_27::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_27 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_27 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_27;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_28 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_28](sw_mux_ctl_pad_gpio_emc_28) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_28 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_28;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_28::R](sw_mux_ctl_pad_gpio_emc_28::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_28 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_28::W](sw_mux_ctl_pad_gpio_emc_28::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_28 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_28 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_28;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_29 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_29](sw_mux_ctl_pad_gpio_emc_29) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_29 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_29;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_29::R](sw_mux_ctl_pad_gpio_emc_29::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_29 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_29::W](sw_mux_ctl_pad_gpio_emc_29::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_29 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_29 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_29;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_30 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_30](sw_mux_ctl_pad_gpio_emc_30) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_30 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_30;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_30::R](sw_mux_ctl_pad_gpio_emc_30::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_30 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_30::W](sw_mux_ctl_pad_gpio_emc_30::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_30 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_30 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_30;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_31 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_31](sw_mux_ctl_pad_gpio_emc_31) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_31 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_31;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_31::R](sw_mux_ctl_pad_gpio_emc_31::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_31 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_31::W](sw_mux_ctl_pad_gpio_emc_31::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_31 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_31 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_31;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_32](sw_mux_ctl_pad_gpio_emc_32) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_32 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_32;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_32::R](sw_mux_ctl_pad_gpio_emc_32::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_32 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_32::W](sw_mux_ctl_pad_gpio_emc_32::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_32 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_32 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_32;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_33](sw_mux_ctl_pad_gpio_emc_33) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_33 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_33;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_33::R](sw_mux_ctl_pad_gpio_emc_33::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_33 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_33::W](sw_mux_ctl_pad_gpio_emc_33::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_33 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_33 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_33;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_34](sw_mux_ctl_pad_gpio_emc_34) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_34 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_34;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_34::R](sw_mux_ctl_pad_gpio_emc_34::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_34 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_34::W](sw_mux_ctl_pad_gpio_emc_34::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_34 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_34 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_34;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_35](sw_mux_ctl_pad_gpio_emc_35) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_35 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_35;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_35::R](sw_mux_ctl_pad_gpio_emc_35::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_35 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_35::W](sw_mux_ctl_pad_gpio_emc_35::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_35 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_35 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_35;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_36 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_36](sw_mux_ctl_pad_gpio_emc_36) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_36 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_36;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_36::R](sw_mux_ctl_pad_gpio_emc_36::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_36 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_36::W](sw_mux_ctl_pad_gpio_emc_36::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_36 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_36 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_36;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_37 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_37](sw_mux_ctl_pad_gpio_emc_37) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_37 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_37;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_37::R](sw_mux_ctl_pad_gpio_emc_37::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_37 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_37::W](sw_mux_ctl_pad_gpio_emc_37::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_37 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_37 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_37;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_38 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_38](sw_mux_ctl_pad_gpio_emc_38) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_38 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_38;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_38::R](sw_mux_ctl_pad_gpio_emc_38::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_38 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_38::W](sw_mux_ctl_pad_gpio_emc_38::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_38 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_38 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_38;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_39 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_39](sw_mux_ctl_pad_gpio_emc_39) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_39 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_39;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_39::R](sw_mux_ctl_pad_gpio_emc_39::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_39 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_39::W](sw_mux_ctl_pad_gpio_emc_39::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_39 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_39 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_39;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_40 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_40](sw_mux_ctl_pad_gpio_emc_40) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_40 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_40;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_40::R](sw_mux_ctl_pad_gpio_emc_40::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_40 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_40::W](sw_mux_ctl_pad_gpio_emc_40::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_40 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_40 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_40;
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_41 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_emc_41](sw_mux_ctl_pad_gpio_emc_41) module"]
pub type SW_MUX_CTL_PAD_GPIO_EMC_41 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_EMC_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_EMC_41;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_emc_41::R](sw_mux_ctl_pad_gpio_emc_41::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_EMC_41 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_emc_41::W](sw_mux_ctl_pad_gpio_emc_41::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_EMC_41 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_EMC_41 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_emc_41;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_00](sw_mux_ctl_pad_gpio_ad_b0_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_00::R](sw_mux_ctl_pad_gpio_ad_b0_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_00::W](sw_mux_ctl_pad_gpio_ad_b0_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_01](sw_mux_ctl_pad_gpio_ad_b0_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_01::R](sw_mux_ctl_pad_gpio_ad_b0_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_01::W](sw_mux_ctl_pad_gpio_ad_b0_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_02](sw_mux_ctl_pad_gpio_ad_b0_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_02::R](sw_mux_ctl_pad_gpio_ad_b0_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_02::W](sw_mux_ctl_pad_gpio_ad_b0_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_03](sw_mux_ctl_pad_gpio_ad_b0_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_03::R](sw_mux_ctl_pad_gpio_ad_b0_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_03::W](sw_mux_ctl_pad_gpio_ad_b0_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_04](sw_mux_ctl_pad_gpio_ad_b0_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_04::R](sw_mux_ctl_pad_gpio_ad_b0_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_04::W](sw_mux_ctl_pad_gpio_ad_b0_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_05](sw_mux_ctl_pad_gpio_ad_b0_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_05::R](sw_mux_ctl_pad_gpio_ad_b0_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_05::W](sw_mux_ctl_pad_gpio_ad_b0_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_06](sw_mux_ctl_pad_gpio_ad_b0_06) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_06 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_06;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_06::R](sw_mux_ctl_pad_gpio_ad_b0_06::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_06 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_06::W](sw_mux_ctl_pad_gpio_ad_b0_06::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_06 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_06 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_06;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_07](sw_mux_ctl_pad_gpio_ad_b0_07) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_07 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_07;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_07::R](sw_mux_ctl_pad_gpio_ad_b0_07::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_07 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_07::W](sw_mux_ctl_pad_gpio_ad_b0_07::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_07 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_07 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_07;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_08](sw_mux_ctl_pad_gpio_ad_b0_08) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_08 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_08;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_08::R](sw_mux_ctl_pad_gpio_ad_b0_08::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_08 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_08::W](sw_mux_ctl_pad_gpio_ad_b0_08::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_08 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_08 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_08;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_09](sw_mux_ctl_pad_gpio_ad_b0_09) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_09 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_09;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_09::R](sw_mux_ctl_pad_gpio_ad_b0_09::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_09 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_09::W](sw_mux_ctl_pad_gpio_ad_b0_09::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_09 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_09 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_09;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_10](sw_mux_ctl_pad_gpio_ad_b0_10) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_10 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_10;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_10::R](sw_mux_ctl_pad_gpio_ad_b0_10::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_10 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_10::W](sw_mux_ctl_pad_gpio_ad_b0_10::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_10 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_10 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_10;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_11](sw_mux_ctl_pad_gpio_ad_b0_11) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_11 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_11;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_11::R](sw_mux_ctl_pad_gpio_ad_b0_11::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_11 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_11::W](sw_mux_ctl_pad_gpio_ad_b0_11::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_11 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_11 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_11;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_12](sw_mux_ctl_pad_gpio_ad_b0_12) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_12 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_12;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_12::R](sw_mux_ctl_pad_gpio_ad_b0_12::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_12 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_12::W](sw_mux_ctl_pad_gpio_ad_b0_12::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_12 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_12 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_12;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_13](sw_mux_ctl_pad_gpio_ad_b0_13) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_13 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_13;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_13::R](sw_mux_ctl_pad_gpio_ad_b0_13::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_13 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_13::W](sw_mux_ctl_pad_gpio_ad_b0_13::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_13 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_13 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_13;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_14](sw_mux_ctl_pad_gpio_ad_b0_14) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_14 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_14;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_14::R](sw_mux_ctl_pad_gpio_ad_b0_14::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_14 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_14::W](sw_mux_ctl_pad_gpio_ad_b0_14::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_14 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_14 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_14;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b0_15](sw_mux_ctl_pad_gpio_ad_b0_15) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B0_15 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B0_15;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_15::R](sw_mux_ctl_pad_gpio_ad_b0_15::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_15 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_15::W](sw_mux_ctl_pad_gpio_ad_b0_15::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_15 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_15 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b0_15;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_00](sw_mux_ctl_pad_gpio_ad_b1_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_00::R](sw_mux_ctl_pad_gpio_ad_b1_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_00::W](sw_mux_ctl_pad_gpio_ad_b1_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_01](sw_mux_ctl_pad_gpio_ad_b1_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_01::R](sw_mux_ctl_pad_gpio_ad_b1_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_01::W](sw_mux_ctl_pad_gpio_ad_b1_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_02](sw_mux_ctl_pad_gpio_ad_b1_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_02::R](sw_mux_ctl_pad_gpio_ad_b1_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_02::W](sw_mux_ctl_pad_gpio_ad_b1_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_03](sw_mux_ctl_pad_gpio_ad_b1_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_03::R](sw_mux_ctl_pad_gpio_ad_b1_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_03::W](sw_mux_ctl_pad_gpio_ad_b1_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_04](sw_mux_ctl_pad_gpio_ad_b1_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_04::R](sw_mux_ctl_pad_gpio_ad_b1_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_04::W](sw_mux_ctl_pad_gpio_ad_b1_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_05](sw_mux_ctl_pad_gpio_ad_b1_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_05::R](sw_mux_ctl_pad_gpio_ad_b1_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_05::W](sw_mux_ctl_pad_gpio_ad_b1_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_06 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_06](sw_mux_ctl_pad_gpio_ad_b1_06) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_06 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_06;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_06::R](sw_mux_ctl_pad_gpio_ad_b1_06::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_06 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_06::W](sw_mux_ctl_pad_gpio_ad_b1_06::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_06 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_06 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_06;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_07 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_07](sw_mux_ctl_pad_gpio_ad_b1_07) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_07 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_07;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_07::R](sw_mux_ctl_pad_gpio_ad_b1_07::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_07 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_07::W](sw_mux_ctl_pad_gpio_ad_b1_07::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_07 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_07 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_07;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_08 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_08](sw_mux_ctl_pad_gpio_ad_b1_08) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_08 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_08;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_08::R](sw_mux_ctl_pad_gpio_ad_b1_08::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_08 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_08::W](sw_mux_ctl_pad_gpio_ad_b1_08::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_08 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_08 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_08;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_09 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_09](sw_mux_ctl_pad_gpio_ad_b1_09) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_09 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_09;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_09::R](sw_mux_ctl_pad_gpio_ad_b1_09::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_09 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_09::W](sw_mux_ctl_pad_gpio_ad_b1_09::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_09 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_09 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_09;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_10](sw_mux_ctl_pad_gpio_ad_b1_10) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_10 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_10;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_10::R](sw_mux_ctl_pad_gpio_ad_b1_10::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_10 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_10::W](sw_mux_ctl_pad_gpio_ad_b1_10::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_10 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_10 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_10;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_11](sw_mux_ctl_pad_gpio_ad_b1_11) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_11 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_11;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_11::R](sw_mux_ctl_pad_gpio_ad_b1_11::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_11 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_11::W](sw_mux_ctl_pad_gpio_ad_b1_11::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_11 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_11 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_11;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_12](sw_mux_ctl_pad_gpio_ad_b1_12) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_12 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_12;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_12::R](sw_mux_ctl_pad_gpio_ad_b1_12::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_12 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_12::W](sw_mux_ctl_pad_gpio_ad_b1_12::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_12 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_12 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_12;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_13](sw_mux_ctl_pad_gpio_ad_b1_13) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_13 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_13;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_13::R](sw_mux_ctl_pad_gpio_ad_b1_13::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_13 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_13::W](sw_mux_ctl_pad_gpio_ad_b1_13::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_13 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_13 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_13;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_14](sw_mux_ctl_pad_gpio_ad_b1_14) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_14 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_14;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_14::R](sw_mux_ctl_pad_gpio_ad_b1_14::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_14 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_14::W](sw_mux_ctl_pad_gpio_ad_b1_14::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_14 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_14 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_14;
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_ad_b1_15](sw_mux_ctl_pad_gpio_ad_b1_15) module"]
pub type SW_MUX_CTL_PAD_GPIO_AD_B1_15 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_AD_B1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_AD_B1_15;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b1_15::R](sw_mux_ctl_pad_gpio_ad_b1_15::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B1_15 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b1_15::W](sw_mux_ctl_pad_gpio_ad_b1_15::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B1_15 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B1_15 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_ad_b1_15;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_00](sw_mux_ctl_pad_gpio_b0_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_00::R](sw_mux_ctl_pad_gpio_b0_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_00::W](sw_mux_ctl_pad_gpio_b0_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_01](sw_mux_ctl_pad_gpio_b0_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_01::R](sw_mux_ctl_pad_gpio_b0_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_01::W](sw_mux_ctl_pad_gpio_b0_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_02](sw_mux_ctl_pad_gpio_b0_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_02::R](sw_mux_ctl_pad_gpio_b0_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_02::W](sw_mux_ctl_pad_gpio_b0_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_03](sw_mux_ctl_pad_gpio_b0_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_03::R](sw_mux_ctl_pad_gpio_b0_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_03::W](sw_mux_ctl_pad_gpio_b0_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_04](sw_mux_ctl_pad_gpio_b0_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_04::R](sw_mux_ctl_pad_gpio_b0_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_04::W](sw_mux_ctl_pad_gpio_b0_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_05](sw_mux_ctl_pad_gpio_b0_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_05::R](sw_mux_ctl_pad_gpio_b0_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_05::W](sw_mux_ctl_pad_gpio_b0_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_06 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_06](sw_mux_ctl_pad_gpio_b0_06) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_06 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_06;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_06::R](sw_mux_ctl_pad_gpio_b0_06::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_06 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_06::W](sw_mux_ctl_pad_gpio_b0_06::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_06 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_06 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_06;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_07 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_07](sw_mux_ctl_pad_gpio_b0_07) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_07 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_07;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_07::R](sw_mux_ctl_pad_gpio_b0_07::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_07 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_07::W](sw_mux_ctl_pad_gpio_b0_07::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_07 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_07 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_07;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_08 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_08](sw_mux_ctl_pad_gpio_b0_08) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_08 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_08;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_08::R](sw_mux_ctl_pad_gpio_b0_08::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_08 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_08::W](sw_mux_ctl_pad_gpio_b0_08::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_08 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_08 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_08;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_09 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_09](sw_mux_ctl_pad_gpio_b0_09) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_09 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_09;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_09::R](sw_mux_ctl_pad_gpio_b0_09::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_09 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_09::W](sw_mux_ctl_pad_gpio_b0_09::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_09 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_09 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_09;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_10 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_10](sw_mux_ctl_pad_gpio_b0_10) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_10 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_10;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_10::R](sw_mux_ctl_pad_gpio_b0_10::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_10 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_10::W](sw_mux_ctl_pad_gpio_b0_10::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_10 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_10 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_10;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_11 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_11](sw_mux_ctl_pad_gpio_b0_11) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_11 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_11;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_11::R](sw_mux_ctl_pad_gpio_b0_11::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_11 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_11::W](sw_mux_ctl_pad_gpio_b0_11::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_11 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_11 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_11;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_12 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_12](sw_mux_ctl_pad_gpio_b0_12) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_12 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_12;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_12::R](sw_mux_ctl_pad_gpio_b0_12::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_12 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_12::W](sw_mux_ctl_pad_gpio_b0_12::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_12 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_12 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_12;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_13 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_13](sw_mux_ctl_pad_gpio_b0_13) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_13 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_13;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_13::R](sw_mux_ctl_pad_gpio_b0_13::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_13 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_13::W](sw_mux_ctl_pad_gpio_b0_13::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_13 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_13 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_13;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_14 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_14](sw_mux_ctl_pad_gpio_b0_14) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_14 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_14;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_14::R](sw_mux_ctl_pad_gpio_b0_14::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_14 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_14::W](sw_mux_ctl_pad_gpio_b0_14::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_14 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_14 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_14;
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_15 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b0_15](sw_mux_ctl_pad_gpio_b0_15) module"]
pub type SW_MUX_CTL_PAD_GPIO_B0_15 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B0_15;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b0_15::R](sw_mux_ctl_pad_gpio_b0_15::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B0_15 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b0_15::W](sw_mux_ctl_pad_gpio_b0_15::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B0_15 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B0_15 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b0_15;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_00](sw_mux_ctl_pad_gpio_b1_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_00::R](sw_mux_ctl_pad_gpio_b1_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_00::W](sw_mux_ctl_pad_gpio_b1_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_01](sw_mux_ctl_pad_gpio_b1_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_01::R](sw_mux_ctl_pad_gpio_b1_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_01::W](sw_mux_ctl_pad_gpio_b1_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_02](sw_mux_ctl_pad_gpio_b1_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_02::R](sw_mux_ctl_pad_gpio_b1_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_02::W](sw_mux_ctl_pad_gpio_b1_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_03](sw_mux_ctl_pad_gpio_b1_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_03::R](sw_mux_ctl_pad_gpio_b1_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_03::W](sw_mux_ctl_pad_gpio_b1_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_04](sw_mux_ctl_pad_gpio_b1_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_04::R](sw_mux_ctl_pad_gpio_b1_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_04::W](sw_mux_ctl_pad_gpio_b1_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_05](sw_mux_ctl_pad_gpio_b1_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_05::R](sw_mux_ctl_pad_gpio_b1_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_05::W](sw_mux_ctl_pad_gpio_b1_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_06](sw_mux_ctl_pad_gpio_b1_06) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_06 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_06;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_06::R](sw_mux_ctl_pad_gpio_b1_06::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_06 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_06::W](sw_mux_ctl_pad_gpio_b1_06::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_06 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_06 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_06;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_07](sw_mux_ctl_pad_gpio_b1_07) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_07 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_07;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_07::R](sw_mux_ctl_pad_gpio_b1_07::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_07 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_07::W](sw_mux_ctl_pad_gpio_b1_07::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_07 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_07 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_07;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_08](sw_mux_ctl_pad_gpio_b1_08) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_08 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_08;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_08::R](sw_mux_ctl_pad_gpio_b1_08::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_08 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_08::W](sw_mux_ctl_pad_gpio_b1_08::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_08 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_08 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_08;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_09](sw_mux_ctl_pad_gpio_b1_09) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_09 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_09;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_09::R](sw_mux_ctl_pad_gpio_b1_09::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_09 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_09::W](sw_mux_ctl_pad_gpio_b1_09::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_09 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_09 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_09;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_10](sw_mux_ctl_pad_gpio_b1_10) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_10 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_10;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_10::R](sw_mux_ctl_pad_gpio_b1_10::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_10 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_10::W](sw_mux_ctl_pad_gpio_b1_10::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_10 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_10 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_10;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_11](sw_mux_ctl_pad_gpio_b1_11) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_11 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_11;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_11::R](sw_mux_ctl_pad_gpio_b1_11::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_11 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_11::W](sw_mux_ctl_pad_gpio_b1_11::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_11 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_11 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_11;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_12](sw_mux_ctl_pad_gpio_b1_12) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_12 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_12;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_12::R](sw_mux_ctl_pad_gpio_b1_12::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_12 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_12::W](sw_mux_ctl_pad_gpio_b1_12::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_12 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_12 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_12;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_13](sw_mux_ctl_pad_gpio_b1_13) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_13 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_13;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_13::R](sw_mux_ctl_pad_gpio_b1_13::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_13 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_13::W](sw_mux_ctl_pad_gpio_b1_13::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_13 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_13 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_13;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_14 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_14](sw_mux_ctl_pad_gpio_b1_14) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_14 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_14;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_14::R](sw_mux_ctl_pad_gpio_b1_14::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_14 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_14::W](sw_mux_ctl_pad_gpio_b1_14::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_14 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_14 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_14;
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_15 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_b1_15](sw_mux_ctl_pad_gpio_b1_15) module"]
pub type SW_MUX_CTL_PAD_GPIO_B1_15 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_B1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_B1_15;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_b1_15::R](sw_mux_ctl_pad_gpio_b1_15::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_B1_15 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_b1_15::W](sw_mux_ctl_pad_gpio_b1_15::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_B1_15 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_B1_15 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_b1_15;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b0_00](sw_mux_ctl_pad_gpio_sd_b0_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B0_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B0_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B0_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b0_00::R](sw_mux_ctl_pad_gpio_sd_b0_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B0_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b0_00::W](sw_mux_ctl_pad_gpio_sd_b0_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B0_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b0_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b0_01](sw_mux_ctl_pad_gpio_sd_b0_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B0_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B0_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B0_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b0_01::R](sw_mux_ctl_pad_gpio_sd_b0_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B0_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b0_01::W](sw_mux_ctl_pad_gpio_sd_b0_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B0_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b0_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b0_02](sw_mux_ctl_pad_gpio_sd_b0_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B0_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B0_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B0_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b0_02::R](sw_mux_ctl_pad_gpio_sd_b0_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B0_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b0_02::W](sw_mux_ctl_pad_gpio_sd_b0_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B0_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b0_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b0_03](sw_mux_ctl_pad_gpio_sd_b0_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B0_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B0_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B0_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b0_03::R](sw_mux_ctl_pad_gpio_sd_b0_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B0_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b0_03::W](sw_mux_ctl_pad_gpio_sd_b0_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B0_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b0_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b0_04](sw_mux_ctl_pad_gpio_sd_b0_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B0_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B0_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B0_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b0_04::R](sw_mux_ctl_pad_gpio_sd_b0_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B0_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b0_04::W](sw_mux_ctl_pad_gpio_sd_b0_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B0_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b0_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b0_05](sw_mux_ctl_pad_gpio_sd_b0_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B0_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B0_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B0_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b0_05::R](sw_mux_ctl_pad_gpio_sd_b0_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B0_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b0_05::W](sw_mux_ctl_pad_gpio_sd_b0_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B0_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B0_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b0_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_00](sw_mux_ctl_pad_gpio_sd_b1_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_00::R](sw_mux_ctl_pad_gpio_sd_b1_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_00::W](sw_mux_ctl_pad_gpio_sd_b1_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_01](sw_mux_ctl_pad_gpio_sd_b1_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_01::R](sw_mux_ctl_pad_gpio_sd_b1_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_01::W](sw_mux_ctl_pad_gpio_sd_b1_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_02](sw_mux_ctl_pad_gpio_sd_b1_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_02::R](sw_mux_ctl_pad_gpio_sd_b1_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_02::W](sw_mux_ctl_pad_gpio_sd_b1_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_03](sw_mux_ctl_pad_gpio_sd_b1_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_03::R](sw_mux_ctl_pad_gpio_sd_b1_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_03::W](sw_mux_ctl_pad_gpio_sd_b1_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_04](sw_mux_ctl_pad_gpio_sd_b1_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_04::R](sw_mux_ctl_pad_gpio_sd_b1_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_04::W](sw_mux_ctl_pad_gpio_sd_b1_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_05](sw_mux_ctl_pad_gpio_sd_b1_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_05::R](sw_mux_ctl_pad_gpio_sd_b1_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_05::W](sw_mux_ctl_pad_gpio_sd_b1_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_06](sw_mux_ctl_pad_gpio_sd_b1_06) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_06 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_06;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_06::R](sw_mux_ctl_pad_gpio_sd_b1_06::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_06 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_06::W](sw_mux_ctl_pad_gpio_sd_b1_06::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_06 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_06 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_06;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_07](sw_mux_ctl_pad_gpio_sd_b1_07) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_07 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_07;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_07::R](sw_mux_ctl_pad_gpio_sd_b1_07::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_07 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_07::W](sw_mux_ctl_pad_gpio_sd_b1_07::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_07 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_07 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_07;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_08](sw_mux_ctl_pad_gpio_sd_b1_08) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_08 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_08;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_08::R](sw_mux_ctl_pad_gpio_sd_b1_08::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_08 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_08::W](sw_mux_ctl_pad_gpio_sd_b1_08::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_08 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_08 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_08;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_09](sw_mux_ctl_pad_gpio_sd_b1_09) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_09 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_09;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_09::R](sw_mux_ctl_pad_gpio_sd_b1_09::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_09 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_09::W](sw_mux_ctl_pad_gpio_sd_b1_09::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_09 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_09 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_09;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_10](sw_mux_ctl_pad_gpio_sd_b1_10) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_10 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_10;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_10::R](sw_mux_ctl_pad_gpio_sd_b1_10::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_10 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_10::W](sw_mux_ctl_pad_gpio_sd_b1_10::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_10 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_10 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_10;
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_sd_b1_11](sw_mux_ctl_pad_gpio_sd_b1_11) module"]
pub type SW_MUX_CTL_PAD_GPIO_SD_B1_11 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SD_B1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SD_B1_11;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_sd_b1_11::R](sw_mux_ctl_pad_gpio_sd_b1_11::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SD_B1_11 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_sd_b1_11::W](sw_mux_ctl_pad_gpio_sd_b1_11::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SD_B1_11 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SD_B1_11 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_sd_b1_11;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_00](sw_pad_ctl_pad_gpio_emc_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_00::R](sw_pad_ctl_pad_gpio_emc_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_00::W](sw_pad_ctl_pad_gpio_emc_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_01](sw_pad_ctl_pad_gpio_emc_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_01::R](sw_pad_ctl_pad_gpio_emc_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_01::W](sw_pad_ctl_pad_gpio_emc_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_02](sw_pad_ctl_pad_gpio_emc_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_02::R](sw_pad_ctl_pad_gpio_emc_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_02::W](sw_pad_ctl_pad_gpio_emc_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_03](sw_pad_ctl_pad_gpio_emc_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_03::R](sw_pad_ctl_pad_gpio_emc_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_03::W](sw_pad_ctl_pad_gpio_emc_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_04](sw_pad_ctl_pad_gpio_emc_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_04::R](sw_pad_ctl_pad_gpio_emc_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_04::W](sw_pad_ctl_pad_gpio_emc_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_05](sw_pad_ctl_pad_gpio_emc_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_05::R](sw_pad_ctl_pad_gpio_emc_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_05::W](sw_pad_ctl_pad_gpio_emc_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_06](sw_pad_ctl_pad_gpio_emc_06) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_06 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_06;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_06::R](sw_pad_ctl_pad_gpio_emc_06::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_06 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_06::W](sw_pad_ctl_pad_gpio_emc_06::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_06 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_06;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_07](sw_pad_ctl_pad_gpio_emc_07) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_07 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_07;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_07::R](sw_pad_ctl_pad_gpio_emc_07::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_07 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_07::W](sw_pad_ctl_pad_gpio_emc_07::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_07 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_07 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_07;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_08](sw_pad_ctl_pad_gpio_emc_08) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_08 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_08;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_08::R](sw_pad_ctl_pad_gpio_emc_08::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_08 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_08::W](sw_pad_ctl_pad_gpio_emc_08::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_08 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_08 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_08;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_09](sw_pad_ctl_pad_gpio_emc_09) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_09 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_09;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_09::R](sw_pad_ctl_pad_gpio_emc_09::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_09 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_09::W](sw_pad_ctl_pad_gpio_emc_09::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_09 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_09 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_09;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_10 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_10](sw_pad_ctl_pad_gpio_emc_10) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_10 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_10;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_10::R](sw_pad_ctl_pad_gpio_emc_10::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_10 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_10::W](sw_pad_ctl_pad_gpio_emc_10::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_10 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_10 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_10;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_11 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_11](sw_pad_ctl_pad_gpio_emc_11) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_11 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_11;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_11::R](sw_pad_ctl_pad_gpio_emc_11::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_11 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_11::W](sw_pad_ctl_pad_gpio_emc_11::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_11 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_11 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_11;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_12 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_12](sw_pad_ctl_pad_gpio_emc_12) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_12 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_12;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_12::R](sw_pad_ctl_pad_gpio_emc_12::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_12 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_12::W](sw_pad_ctl_pad_gpio_emc_12::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_12 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_12 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_12;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_13 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_13](sw_pad_ctl_pad_gpio_emc_13) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_13 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_13;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_13::R](sw_pad_ctl_pad_gpio_emc_13::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_13 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_13::W](sw_pad_ctl_pad_gpio_emc_13::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_13 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_13 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_13;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_14 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_14](sw_pad_ctl_pad_gpio_emc_14) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_14 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_14;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_14::R](sw_pad_ctl_pad_gpio_emc_14::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_14 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_14::W](sw_pad_ctl_pad_gpio_emc_14::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_14 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_14 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_14;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_15 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_15](sw_pad_ctl_pad_gpio_emc_15) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_15 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_15;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_15::R](sw_pad_ctl_pad_gpio_emc_15::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_15 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_15::W](sw_pad_ctl_pad_gpio_emc_15::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_15 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_15 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_15;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_16](sw_pad_ctl_pad_gpio_emc_16) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_16 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_16;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_16::R](sw_pad_ctl_pad_gpio_emc_16::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_16 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_16::W](sw_pad_ctl_pad_gpio_emc_16::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_16 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_16 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_16;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_17](sw_pad_ctl_pad_gpio_emc_17) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_17 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_17;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_17::R](sw_pad_ctl_pad_gpio_emc_17::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_17 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_17::W](sw_pad_ctl_pad_gpio_emc_17::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_17 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_17 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_17;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_18](sw_pad_ctl_pad_gpio_emc_18) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_18 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_18;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_18::R](sw_pad_ctl_pad_gpio_emc_18::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_18 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_18::W](sw_pad_ctl_pad_gpio_emc_18::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_18 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_18 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_18;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_19](sw_pad_ctl_pad_gpio_emc_19) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_19 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_19;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_19::R](sw_pad_ctl_pad_gpio_emc_19::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_19 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_19::W](sw_pad_ctl_pad_gpio_emc_19::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_19 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_19 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_19;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_20](sw_pad_ctl_pad_gpio_emc_20) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_20 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_20;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_20::R](sw_pad_ctl_pad_gpio_emc_20::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_20 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_20::W](sw_pad_ctl_pad_gpio_emc_20::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_20 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_20 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_20;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_21](sw_pad_ctl_pad_gpio_emc_21) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_21 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_21;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_21::R](sw_pad_ctl_pad_gpio_emc_21::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_21 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_21::W](sw_pad_ctl_pad_gpio_emc_21::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_21 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_21 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_21;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_22](sw_pad_ctl_pad_gpio_emc_22) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_22 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_22;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_22::R](sw_pad_ctl_pad_gpio_emc_22::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_22 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_22::W](sw_pad_ctl_pad_gpio_emc_22::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_22 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_22 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_22;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_23](sw_pad_ctl_pad_gpio_emc_23) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_23 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_23;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_23::R](sw_pad_ctl_pad_gpio_emc_23::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_23 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_23::W](sw_pad_ctl_pad_gpio_emc_23::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_23 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_23 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_23;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_24](sw_pad_ctl_pad_gpio_emc_24) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_24 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_24;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_24::R](sw_pad_ctl_pad_gpio_emc_24::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_24 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_24::W](sw_pad_ctl_pad_gpio_emc_24::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_24 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_24 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_24;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_25](sw_pad_ctl_pad_gpio_emc_25) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_25 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_25;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_25::R](sw_pad_ctl_pad_gpio_emc_25::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_25 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_25::W](sw_pad_ctl_pad_gpio_emc_25::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_25 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_25 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_25;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_26](sw_pad_ctl_pad_gpio_emc_26) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_26 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_26;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_26::R](sw_pad_ctl_pad_gpio_emc_26::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_26 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_26::W](sw_pad_ctl_pad_gpio_emc_26::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_26 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_26 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_26;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_27](sw_pad_ctl_pad_gpio_emc_27) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_27 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_27;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_27::R](sw_pad_ctl_pad_gpio_emc_27::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_27 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_27::W](sw_pad_ctl_pad_gpio_emc_27::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_27 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_27 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_27;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_28 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_28](sw_pad_ctl_pad_gpio_emc_28) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_28 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_28;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_28::R](sw_pad_ctl_pad_gpio_emc_28::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_28 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_28::W](sw_pad_ctl_pad_gpio_emc_28::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_28 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_28 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_28;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_29 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_29](sw_pad_ctl_pad_gpio_emc_29) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_29 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_29;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_29::R](sw_pad_ctl_pad_gpio_emc_29::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_29 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_29::W](sw_pad_ctl_pad_gpio_emc_29::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_29 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_29 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_29;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_30 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_30](sw_pad_ctl_pad_gpio_emc_30) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_30 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_30;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_30::R](sw_pad_ctl_pad_gpio_emc_30::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_30 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_30::W](sw_pad_ctl_pad_gpio_emc_30::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_30 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_30 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_30;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_31 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_31](sw_pad_ctl_pad_gpio_emc_31) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_31 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_31;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_31::R](sw_pad_ctl_pad_gpio_emc_31::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_31 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_31::W](sw_pad_ctl_pad_gpio_emc_31::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_31 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_31 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_31;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_32](sw_pad_ctl_pad_gpio_emc_32) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_32 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_32;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_32::R](sw_pad_ctl_pad_gpio_emc_32::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_32 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_32::W](sw_pad_ctl_pad_gpio_emc_32::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_32 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_32 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_32;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_33](sw_pad_ctl_pad_gpio_emc_33) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_33 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_33;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_33::R](sw_pad_ctl_pad_gpio_emc_33::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_33 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_33::W](sw_pad_ctl_pad_gpio_emc_33::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_33 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_33 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_33;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_34](sw_pad_ctl_pad_gpio_emc_34) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_34 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_34;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_34::R](sw_pad_ctl_pad_gpio_emc_34::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_34 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_34::W](sw_pad_ctl_pad_gpio_emc_34::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_34 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_34 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_34;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_35](sw_pad_ctl_pad_gpio_emc_35) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_35 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_35;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_35::R](sw_pad_ctl_pad_gpio_emc_35::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_35 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_35::W](sw_pad_ctl_pad_gpio_emc_35::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_35 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_35 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_35;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_36 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_36](sw_pad_ctl_pad_gpio_emc_36) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_36 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_36;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_36::R](sw_pad_ctl_pad_gpio_emc_36::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_36 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_36::W](sw_pad_ctl_pad_gpio_emc_36::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_36 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_36 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_36;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_37 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_37](sw_pad_ctl_pad_gpio_emc_37) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_37 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_37;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_37::R](sw_pad_ctl_pad_gpio_emc_37::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_37 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_37::W](sw_pad_ctl_pad_gpio_emc_37::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_37 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_37 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_37;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_38 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_38](sw_pad_ctl_pad_gpio_emc_38) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_38 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_38;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_38::R](sw_pad_ctl_pad_gpio_emc_38::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_38 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_38::W](sw_pad_ctl_pad_gpio_emc_38::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_38 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_38 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_38;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_39 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_39](sw_pad_ctl_pad_gpio_emc_39) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_39 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_39;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_39::R](sw_pad_ctl_pad_gpio_emc_39::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_39 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_39::W](sw_pad_ctl_pad_gpio_emc_39::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_39 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_39 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_39;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_40 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_40](sw_pad_ctl_pad_gpio_emc_40) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_40 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_40;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_40::R](sw_pad_ctl_pad_gpio_emc_40::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_40 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_40::W](sw_pad_ctl_pad_gpio_emc_40::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_40 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_40 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_40;
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_41 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_emc_41](sw_pad_ctl_pad_gpio_emc_41) module"]
pub type SW_PAD_CTL_PAD_GPIO_EMC_41 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_EMC_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_EMC_41;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_41::R](sw_pad_ctl_pad_gpio_emc_41::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_41 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_41::W](sw_pad_ctl_pad_gpio_emc_41::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_41 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_41 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_emc_41;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_00](sw_pad_ctl_pad_gpio_ad_b0_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_00::R](sw_pad_ctl_pad_gpio_ad_b0_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_00::W](sw_pad_ctl_pad_gpio_ad_b0_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_01](sw_pad_ctl_pad_gpio_ad_b0_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_01::R](sw_pad_ctl_pad_gpio_ad_b0_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_01::W](sw_pad_ctl_pad_gpio_ad_b0_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_02](sw_pad_ctl_pad_gpio_ad_b0_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_02::R](sw_pad_ctl_pad_gpio_ad_b0_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_02::W](sw_pad_ctl_pad_gpio_ad_b0_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_03](sw_pad_ctl_pad_gpio_ad_b0_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_03::R](sw_pad_ctl_pad_gpio_ad_b0_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_03::W](sw_pad_ctl_pad_gpio_ad_b0_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_04](sw_pad_ctl_pad_gpio_ad_b0_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_04::R](sw_pad_ctl_pad_gpio_ad_b0_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_04::W](sw_pad_ctl_pad_gpio_ad_b0_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_05](sw_pad_ctl_pad_gpio_ad_b0_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_05::R](sw_pad_ctl_pad_gpio_ad_b0_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_05::W](sw_pad_ctl_pad_gpio_ad_b0_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_06](sw_pad_ctl_pad_gpio_ad_b0_06) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_06 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_06;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_06::R](sw_pad_ctl_pad_gpio_ad_b0_06::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_06 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_06::W](sw_pad_ctl_pad_gpio_ad_b0_06::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_06 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_06 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_06;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_07](sw_pad_ctl_pad_gpio_ad_b0_07) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_07 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_07;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_07::R](sw_pad_ctl_pad_gpio_ad_b0_07::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_07 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_07::W](sw_pad_ctl_pad_gpio_ad_b0_07::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_07 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_07 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_07;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_08](sw_pad_ctl_pad_gpio_ad_b0_08) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_08 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_08;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_08::R](sw_pad_ctl_pad_gpio_ad_b0_08::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_08 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_08::W](sw_pad_ctl_pad_gpio_ad_b0_08::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_08 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_08 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_08;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_09](sw_pad_ctl_pad_gpio_ad_b0_09) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_09 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_09;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_09::R](sw_pad_ctl_pad_gpio_ad_b0_09::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_09 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_09::W](sw_pad_ctl_pad_gpio_ad_b0_09::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_09 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_09 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_09;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_10](sw_pad_ctl_pad_gpio_ad_b0_10) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_10 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_10;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_10::R](sw_pad_ctl_pad_gpio_ad_b0_10::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_10 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_10::W](sw_pad_ctl_pad_gpio_ad_b0_10::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_10 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_10 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_10;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_11](sw_pad_ctl_pad_gpio_ad_b0_11) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_11 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_11;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_11::R](sw_pad_ctl_pad_gpio_ad_b0_11::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_11 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_11::W](sw_pad_ctl_pad_gpio_ad_b0_11::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_11 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_11 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_11;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_12](sw_pad_ctl_pad_gpio_ad_b0_12) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_12 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_12;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_12::R](sw_pad_ctl_pad_gpio_ad_b0_12::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_12 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_12::W](sw_pad_ctl_pad_gpio_ad_b0_12::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_12 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_12 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_12;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_13](sw_pad_ctl_pad_gpio_ad_b0_13) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_13 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_13;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_13::R](sw_pad_ctl_pad_gpio_ad_b0_13::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_13 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_13::W](sw_pad_ctl_pad_gpio_ad_b0_13::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_13 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_13 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_13;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_14](sw_pad_ctl_pad_gpio_ad_b0_14) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_14 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_14;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_14::R](sw_pad_ctl_pad_gpio_ad_b0_14::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_14 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_14::W](sw_pad_ctl_pad_gpio_ad_b0_14::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_14 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_14 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_14;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b0_15](sw_pad_ctl_pad_gpio_ad_b0_15) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B0_15 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B0_15;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b0_15::R](sw_pad_ctl_pad_gpio_ad_b0_15::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B0_15 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b0_15::W](sw_pad_ctl_pad_gpio_ad_b0_15::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B0_15 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_15 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b0_15;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_00](sw_pad_ctl_pad_gpio_ad_b1_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_00::R](sw_pad_ctl_pad_gpio_ad_b1_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_00::W](sw_pad_ctl_pad_gpio_ad_b1_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_01](sw_pad_ctl_pad_gpio_ad_b1_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_01::R](sw_pad_ctl_pad_gpio_ad_b1_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_01::W](sw_pad_ctl_pad_gpio_ad_b1_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_02](sw_pad_ctl_pad_gpio_ad_b1_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_02::R](sw_pad_ctl_pad_gpio_ad_b1_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_02::W](sw_pad_ctl_pad_gpio_ad_b1_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_03](sw_pad_ctl_pad_gpio_ad_b1_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_03::R](sw_pad_ctl_pad_gpio_ad_b1_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_03::W](sw_pad_ctl_pad_gpio_ad_b1_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_04](sw_pad_ctl_pad_gpio_ad_b1_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_04::R](sw_pad_ctl_pad_gpio_ad_b1_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_04::W](sw_pad_ctl_pad_gpio_ad_b1_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_05](sw_pad_ctl_pad_gpio_ad_b1_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_05::R](sw_pad_ctl_pad_gpio_ad_b1_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_05::W](sw_pad_ctl_pad_gpio_ad_b1_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_06](sw_pad_ctl_pad_gpio_ad_b1_06) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_06 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_06;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_06::R](sw_pad_ctl_pad_gpio_ad_b1_06::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_06 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_06::W](sw_pad_ctl_pad_gpio_ad_b1_06::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_06 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_06 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_06;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_07 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_07](sw_pad_ctl_pad_gpio_ad_b1_07) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_07 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_07;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_07::R](sw_pad_ctl_pad_gpio_ad_b1_07::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_07 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_07::W](sw_pad_ctl_pad_gpio_ad_b1_07::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_07 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_07 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_07;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_08 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_08](sw_pad_ctl_pad_gpio_ad_b1_08) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_08 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_08;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_08::R](sw_pad_ctl_pad_gpio_ad_b1_08::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_08 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_08::W](sw_pad_ctl_pad_gpio_ad_b1_08::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_08 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_08 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_08;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_09 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_09](sw_pad_ctl_pad_gpio_ad_b1_09) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_09 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_09;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_09::R](sw_pad_ctl_pad_gpio_ad_b1_09::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_09 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_09::W](sw_pad_ctl_pad_gpio_ad_b1_09::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_09 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_09 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_09;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_10](sw_pad_ctl_pad_gpio_ad_b1_10) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_10 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_10;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_10::R](sw_pad_ctl_pad_gpio_ad_b1_10::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_10 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_10::W](sw_pad_ctl_pad_gpio_ad_b1_10::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_10 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_10 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_10;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_11](sw_pad_ctl_pad_gpio_ad_b1_11) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_11 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_11;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_11::R](sw_pad_ctl_pad_gpio_ad_b1_11::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_11 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_11::W](sw_pad_ctl_pad_gpio_ad_b1_11::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_11 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_11 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_11;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_12](sw_pad_ctl_pad_gpio_ad_b1_12) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_12 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_12;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_12::R](sw_pad_ctl_pad_gpio_ad_b1_12::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_12 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_12::W](sw_pad_ctl_pad_gpio_ad_b1_12::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_12 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_12 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_12;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_13](sw_pad_ctl_pad_gpio_ad_b1_13) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_13 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_13;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_13::R](sw_pad_ctl_pad_gpio_ad_b1_13::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_13 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_13::W](sw_pad_ctl_pad_gpio_ad_b1_13::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_13 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_13 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_13;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_14](sw_pad_ctl_pad_gpio_ad_b1_14) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_14 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_14;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_14::R](sw_pad_ctl_pad_gpio_ad_b1_14::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_14 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_14::W](sw_pad_ctl_pad_gpio_ad_b1_14::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_14 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_14 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_14;
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_ad_b1_15](sw_pad_ctl_pad_gpio_ad_b1_15) module"]
pub type SW_PAD_CTL_PAD_GPIO_AD_B1_15 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_AD_B1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_AD_B1_15;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_ad_b1_15::R](sw_pad_ctl_pad_gpio_ad_b1_15::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_AD_B1_15 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_ad_b1_15::W](sw_pad_ctl_pad_gpio_ad_b1_15::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_AD_B1_15 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B1_15 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_ad_b1_15;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_00](sw_pad_ctl_pad_gpio_b0_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_00::R](sw_pad_ctl_pad_gpio_b0_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_00::W](sw_pad_ctl_pad_gpio_b0_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_01](sw_pad_ctl_pad_gpio_b0_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_01::R](sw_pad_ctl_pad_gpio_b0_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_01::W](sw_pad_ctl_pad_gpio_b0_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_02](sw_pad_ctl_pad_gpio_b0_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_02::R](sw_pad_ctl_pad_gpio_b0_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_02::W](sw_pad_ctl_pad_gpio_b0_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_03](sw_pad_ctl_pad_gpio_b0_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_03::R](sw_pad_ctl_pad_gpio_b0_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_03::W](sw_pad_ctl_pad_gpio_b0_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_04](sw_pad_ctl_pad_gpio_b0_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_04::R](sw_pad_ctl_pad_gpio_b0_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_04::W](sw_pad_ctl_pad_gpio_b0_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_05](sw_pad_ctl_pad_gpio_b0_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_05::R](sw_pad_ctl_pad_gpio_b0_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_05::W](sw_pad_ctl_pad_gpio_b0_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_06](sw_pad_ctl_pad_gpio_b0_06) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_06 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_06;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_06::R](sw_pad_ctl_pad_gpio_b0_06::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_06 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_06::W](sw_pad_ctl_pad_gpio_b0_06::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_06 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_06 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_06;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_07 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_07](sw_pad_ctl_pad_gpio_b0_07) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_07 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_07;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_07::R](sw_pad_ctl_pad_gpio_b0_07::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_07 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_07::W](sw_pad_ctl_pad_gpio_b0_07::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_07 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_07 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_07;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_08 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_08](sw_pad_ctl_pad_gpio_b0_08) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_08 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_08;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_08::R](sw_pad_ctl_pad_gpio_b0_08::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_08 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_08::W](sw_pad_ctl_pad_gpio_b0_08::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_08 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_08 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_08;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_09 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_09](sw_pad_ctl_pad_gpio_b0_09) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_09 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_09;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_09::R](sw_pad_ctl_pad_gpio_b0_09::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_09 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_09::W](sw_pad_ctl_pad_gpio_b0_09::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_09 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_09 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_09;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_10 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_10](sw_pad_ctl_pad_gpio_b0_10) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_10 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_10;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_10::R](sw_pad_ctl_pad_gpio_b0_10::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_10 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_10::W](sw_pad_ctl_pad_gpio_b0_10::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_10 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_10 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_10;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_11 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_11](sw_pad_ctl_pad_gpio_b0_11) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_11 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_11;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_11::R](sw_pad_ctl_pad_gpio_b0_11::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_11 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_11::W](sw_pad_ctl_pad_gpio_b0_11::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_11 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_11 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_11;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_12 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_12](sw_pad_ctl_pad_gpio_b0_12) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_12 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_12;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_12::R](sw_pad_ctl_pad_gpio_b0_12::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_12 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_12::W](sw_pad_ctl_pad_gpio_b0_12::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_12 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_12 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_12;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_13 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_13](sw_pad_ctl_pad_gpio_b0_13) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_13 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_13;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_13::R](sw_pad_ctl_pad_gpio_b0_13::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_13 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_13::W](sw_pad_ctl_pad_gpio_b0_13::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_13 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_13 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_13;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_14 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_14](sw_pad_ctl_pad_gpio_b0_14) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_14 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_14;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_14::R](sw_pad_ctl_pad_gpio_b0_14::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_14 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_14::W](sw_pad_ctl_pad_gpio_b0_14::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_14 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_14 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_14;
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_15 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b0_15](sw_pad_ctl_pad_gpio_b0_15) module"]
pub type SW_PAD_CTL_PAD_GPIO_B0_15 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B0_15;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b0_15::R](sw_pad_ctl_pad_gpio_b0_15::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B0_15 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b0_15::W](sw_pad_ctl_pad_gpio_b0_15::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B0_15 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B0_15 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b0_15;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_00](sw_pad_ctl_pad_gpio_b1_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_00::R](sw_pad_ctl_pad_gpio_b1_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_00::W](sw_pad_ctl_pad_gpio_b1_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_01](sw_pad_ctl_pad_gpio_b1_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_01::R](sw_pad_ctl_pad_gpio_b1_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_01::W](sw_pad_ctl_pad_gpio_b1_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_02](sw_pad_ctl_pad_gpio_b1_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_02::R](sw_pad_ctl_pad_gpio_b1_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_02::W](sw_pad_ctl_pad_gpio_b1_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_03](sw_pad_ctl_pad_gpio_b1_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_03::R](sw_pad_ctl_pad_gpio_b1_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_03::W](sw_pad_ctl_pad_gpio_b1_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_04](sw_pad_ctl_pad_gpio_b1_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_04::R](sw_pad_ctl_pad_gpio_b1_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_04::W](sw_pad_ctl_pad_gpio_b1_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_05](sw_pad_ctl_pad_gpio_b1_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_05::R](sw_pad_ctl_pad_gpio_b1_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_05::W](sw_pad_ctl_pad_gpio_b1_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_06](sw_pad_ctl_pad_gpio_b1_06) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_06 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_06;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_06::R](sw_pad_ctl_pad_gpio_b1_06::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_06 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_06::W](sw_pad_ctl_pad_gpio_b1_06::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_06 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_06 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_06;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_07 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_07](sw_pad_ctl_pad_gpio_b1_07) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_07 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_07;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_07::R](sw_pad_ctl_pad_gpio_b1_07::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_07 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_07::W](sw_pad_ctl_pad_gpio_b1_07::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_07 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_07 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_07;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_08 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_08](sw_pad_ctl_pad_gpio_b1_08) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_08 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_08;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_08::R](sw_pad_ctl_pad_gpio_b1_08::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_08 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_08::W](sw_pad_ctl_pad_gpio_b1_08::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_08 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_08 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_08;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_09 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_09](sw_pad_ctl_pad_gpio_b1_09) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_09 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_09;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_09::R](sw_pad_ctl_pad_gpio_b1_09::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_09 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_09::W](sw_pad_ctl_pad_gpio_b1_09::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_09 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_09 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_09;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_10 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_10](sw_pad_ctl_pad_gpio_b1_10) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_10 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_10;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_10::R](sw_pad_ctl_pad_gpio_b1_10::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_10 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_10::W](sw_pad_ctl_pad_gpio_b1_10::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_10 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_10 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_10;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_11 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_11](sw_pad_ctl_pad_gpio_b1_11) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_11 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_11;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_11::R](sw_pad_ctl_pad_gpio_b1_11::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_11 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_11::W](sw_pad_ctl_pad_gpio_b1_11::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_11 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_11 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_11;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_12 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_12](sw_pad_ctl_pad_gpio_b1_12) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_12 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_12;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_12::R](sw_pad_ctl_pad_gpio_b1_12::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_12 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_12::W](sw_pad_ctl_pad_gpio_b1_12::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_12 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_12 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_12;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_13 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_13](sw_pad_ctl_pad_gpio_b1_13) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_13 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_13;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_13::R](sw_pad_ctl_pad_gpio_b1_13::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_13 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_13::W](sw_pad_ctl_pad_gpio_b1_13::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_13 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_13 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_13;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_14 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_14](sw_pad_ctl_pad_gpio_b1_14) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_14 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_14;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_14::R](sw_pad_ctl_pad_gpio_b1_14::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_14 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_14::W](sw_pad_ctl_pad_gpio_b1_14::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_14 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_14 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_14;
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_15 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_b1_15](sw_pad_ctl_pad_gpio_b1_15) module"]
pub type SW_PAD_CTL_PAD_GPIO_B1_15 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_B1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_B1_15;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_b1_15::R](sw_pad_ctl_pad_gpio_b1_15::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_B1_15 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_b1_15::W](sw_pad_ctl_pad_gpio_b1_15::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_B1_15 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_B1_15 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_b1_15;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b0_00](sw_pad_ctl_pad_gpio_sd_b0_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B0_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B0_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B0_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b0_00::R](sw_pad_ctl_pad_gpio_sd_b0_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B0_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b0_00::W](sw_pad_ctl_pad_gpio_sd_b0_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B0_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b0_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b0_01](sw_pad_ctl_pad_gpio_sd_b0_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B0_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B0_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B0_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b0_01::R](sw_pad_ctl_pad_gpio_sd_b0_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B0_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b0_01::W](sw_pad_ctl_pad_gpio_sd_b0_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B0_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b0_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b0_02](sw_pad_ctl_pad_gpio_sd_b0_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B0_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B0_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B0_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b0_02::R](sw_pad_ctl_pad_gpio_sd_b0_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B0_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b0_02::W](sw_pad_ctl_pad_gpio_sd_b0_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B0_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b0_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b0_03](sw_pad_ctl_pad_gpio_sd_b0_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B0_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B0_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B0_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b0_03::R](sw_pad_ctl_pad_gpio_sd_b0_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B0_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b0_03::W](sw_pad_ctl_pad_gpio_sd_b0_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B0_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b0_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b0_04](sw_pad_ctl_pad_gpio_sd_b0_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B0_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B0_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B0_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b0_04::R](sw_pad_ctl_pad_gpio_sd_b0_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B0_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b0_04::W](sw_pad_ctl_pad_gpio_sd_b0_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B0_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b0_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b0_05](sw_pad_ctl_pad_gpio_sd_b0_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B0_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B0_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B0_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b0_05::R](sw_pad_ctl_pad_gpio_sd_b0_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B0_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b0_05::W](sw_pad_ctl_pad_gpio_sd_b0_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B0_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B0_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b0_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_00](sw_pad_ctl_pad_gpio_sd_b1_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_00::R](sw_pad_ctl_pad_gpio_sd_b1_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_00::W](sw_pad_ctl_pad_gpio_sd_b1_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_01](sw_pad_ctl_pad_gpio_sd_b1_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_01::R](sw_pad_ctl_pad_gpio_sd_b1_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_01::W](sw_pad_ctl_pad_gpio_sd_b1_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_02](sw_pad_ctl_pad_gpio_sd_b1_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_02::R](sw_pad_ctl_pad_gpio_sd_b1_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_02::W](sw_pad_ctl_pad_gpio_sd_b1_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_03](sw_pad_ctl_pad_gpio_sd_b1_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_03::R](sw_pad_ctl_pad_gpio_sd_b1_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_03::W](sw_pad_ctl_pad_gpio_sd_b1_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_04](sw_pad_ctl_pad_gpio_sd_b1_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_04::R](sw_pad_ctl_pad_gpio_sd_b1_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_04::W](sw_pad_ctl_pad_gpio_sd_b1_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_05](sw_pad_ctl_pad_gpio_sd_b1_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_05::R](sw_pad_ctl_pad_gpio_sd_b1_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_05::W](sw_pad_ctl_pad_gpio_sd_b1_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_06](sw_pad_ctl_pad_gpio_sd_b1_06) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_06 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_06;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_06::R](sw_pad_ctl_pad_gpio_sd_b1_06::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_06 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_06::W](sw_pad_ctl_pad_gpio_sd_b1_06::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_06 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_06 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_06;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_07](sw_pad_ctl_pad_gpio_sd_b1_07) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_07 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_07;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_07::R](sw_pad_ctl_pad_gpio_sd_b1_07::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_07 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_07::W](sw_pad_ctl_pad_gpio_sd_b1_07::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_07 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_07 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_07;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_08](sw_pad_ctl_pad_gpio_sd_b1_08) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_08 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_08;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_08::R](sw_pad_ctl_pad_gpio_sd_b1_08::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_08 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_08::W](sw_pad_ctl_pad_gpio_sd_b1_08::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_08 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_08 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_08;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_09](sw_pad_ctl_pad_gpio_sd_b1_09) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_09 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_09;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_09::R](sw_pad_ctl_pad_gpio_sd_b1_09::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_09 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_09::W](sw_pad_ctl_pad_gpio_sd_b1_09::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_09 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_09 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_09;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_10](sw_pad_ctl_pad_gpio_sd_b1_10) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_10 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_10;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_10::R](sw_pad_ctl_pad_gpio_sd_b1_10::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_10 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_10::W](sw_pad_ctl_pad_gpio_sd_b1_10::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_10 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_10 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_10;
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_sd_b1_11](sw_pad_ctl_pad_gpio_sd_b1_11) module"]
pub type SW_PAD_CTL_PAD_GPIO_SD_B1_11 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SD_B1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SD_B1_11;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_sd_b1_11::R](sw_pad_ctl_pad_gpio_sd_b1_11::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SD_B1_11 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_sd_b1_11::W](sw_pad_ctl_pad_gpio_sd_b1_11::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SD_B1_11 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SD_B1_11 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_sd_b1_11;
#[doc = "ANATOP_USB_OTG1_ID_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anatop_usb_otg1_id_select_input](anatop_usb_otg1_id_select_input) module"]
pub type ANATOP_USB_OTG1_ID_SELECT_INPUT = crate::Reg<u32, _ANATOP_USB_OTG1_ID_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANATOP_USB_OTG1_ID_SELECT_INPUT;
#[doc = "`read()` method returns [anatop_usb_otg1_id_select_input::R](anatop_usb_otg1_id_select_input::R) reader structure"]
impl crate::Readable for ANATOP_USB_OTG1_ID_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [anatop_usb_otg1_id_select_input::W](anatop_usb_otg1_id_select_input::W) writer structure"]
impl crate::Writable for ANATOP_USB_OTG1_ID_SELECT_INPUT {}
#[doc = "ANATOP_USB_OTG1_ID_SELECT_INPUT DAISY Register"]
pub mod anatop_usb_otg1_id_select_input;
#[doc = "ANATOP_USB_OTG2_ID_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anatop_usb_otg2_id_select_input](anatop_usb_otg2_id_select_input) module"]
pub type ANATOP_USB_OTG2_ID_SELECT_INPUT = crate::Reg<u32, _ANATOP_USB_OTG2_ID_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANATOP_USB_OTG2_ID_SELECT_INPUT;
#[doc = "`read()` method returns [anatop_usb_otg2_id_select_input::R](anatop_usb_otg2_id_select_input::R) reader structure"]
impl crate::Readable for ANATOP_USB_OTG2_ID_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [anatop_usb_otg2_id_select_input::W](anatop_usb_otg2_id_select_input::W) writer structure"]
impl crate::Writable for ANATOP_USB_OTG2_ID_SELECT_INPUT {}
#[doc = "ANATOP_USB_OTG2_ID_SELECT_INPUT DAISY Register"]
pub mod anatop_usb_otg2_id_select_input;
#[doc = "CCM_PMIC_READY_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccm_pmic_ready_select_input](ccm_pmic_ready_select_input) module"]
pub type CCM_PMIC_READY_SELECT_INPUT = crate::Reg<u32, _CCM_PMIC_READY_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCM_PMIC_READY_SELECT_INPUT;
#[doc = "`read()` method returns [ccm_pmic_ready_select_input::R](ccm_pmic_ready_select_input::R) reader structure"]
impl crate::Readable for CCM_PMIC_READY_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [ccm_pmic_ready_select_input::W](ccm_pmic_ready_select_input::W) writer structure"]
impl crate::Writable for CCM_PMIC_READY_SELECT_INPUT {}
#[doc = "CCM_PMIC_READY_SELECT_INPUT DAISY Register"]
pub mod ccm_pmic_ready_select_input;
#[doc = "CSI_DATA02_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_data02_select_input](csi_data02_select_input) module"]
pub type CSI_DATA02_SELECT_INPUT = crate::Reg<u32, _CSI_DATA02_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_DATA02_SELECT_INPUT;
#[doc = "`read()` method returns [csi_data02_select_input::R](csi_data02_select_input::R) reader structure"]
impl crate::Readable for CSI_DATA02_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_data02_select_input::W](csi_data02_select_input::W) writer structure"]
impl crate::Writable for CSI_DATA02_SELECT_INPUT {}
#[doc = "CSI_DATA02_SELECT_INPUT DAISY Register"]
pub mod csi_data02_select_input;
#[doc = "CSI_DATA03_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_data03_select_input](csi_data03_select_input) module"]
pub type CSI_DATA03_SELECT_INPUT = crate::Reg<u32, _CSI_DATA03_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_DATA03_SELECT_INPUT;
#[doc = "`read()` method returns [csi_data03_select_input::R](csi_data03_select_input::R) reader structure"]
impl crate::Readable for CSI_DATA03_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_data03_select_input::W](csi_data03_select_input::W) writer structure"]
impl crate::Writable for CSI_DATA03_SELECT_INPUT {}
#[doc = "CSI_DATA03_SELECT_INPUT DAISY Register"]
pub mod csi_data03_select_input;
#[doc = "CSI_DATA04_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_data04_select_input](csi_data04_select_input) module"]
pub type CSI_DATA04_SELECT_INPUT = crate::Reg<u32, _CSI_DATA04_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_DATA04_SELECT_INPUT;
#[doc = "`read()` method returns [csi_data04_select_input::R](csi_data04_select_input::R) reader structure"]
impl crate::Readable for CSI_DATA04_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_data04_select_input::W](csi_data04_select_input::W) writer structure"]
impl crate::Writable for CSI_DATA04_SELECT_INPUT {}
#[doc = "CSI_DATA04_SELECT_INPUT DAISY Register"]
pub mod csi_data04_select_input;
#[doc = "CSI_DATA05_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_data05_select_input](csi_data05_select_input) module"]
pub type CSI_DATA05_SELECT_INPUT = crate::Reg<u32, _CSI_DATA05_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_DATA05_SELECT_INPUT;
#[doc = "`read()` method returns [csi_data05_select_input::R](csi_data05_select_input::R) reader structure"]
impl crate::Readable for CSI_DATA05_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_data05_select_input::W](csi_data05_select_input::W) writer structure"]
impl crate::Writable for CSI_DATA05_SELECT_INPUT {}
#[doc = "CSI_DATA05_SELECT_INPUT DAISY Register"]
pub mod csi_data05_select_input;
#[doc = "CSI_DATA06_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_data06_select_input](csi_data06_select_input) module"]
pub type CSI_DATA06_SELECT_INPUT = crate::Reg<u32, _CSI_DATA06_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_DATA06_SELECT_INPUT;
#[doc = "`read()` method returns [csi_data06_select_input::R](csi_data06_select_input::R) reader structure"]
impl crate::Readable for CSI_DATA06_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_data06_select_input::W](csi_data06_select_input::W) writer structure"]
impl crate::Writable for CSI_DATA06_SELECT_INPUT {}
#[doc = "CSI_DATA06_SELECT_INPUT DAISY Register"]
pub mod csi_data06_select_input;
#[doc = "CSI_DATA07_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_data07_select_input](csi_data07_select_input) module"]
pub type CSI_DATA07_SELECT_INPUT = crate::Reg<u32, _CSI_DATA07_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_DATA07_SELECT_INPUT;
#[doc = "`read()` method returns [csi_data07_select_input::R](csi_data07_select_input::R) reader structure"]
impl crate::Readable for CSI_DATA07_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_data07_select_input::W](csi_data07_select_input::W) writer structure"]
impl crate::Writable for CSI_DATA07_SELECT_INPUT {}
#[doc = "CSI_DATA07_SELECT_INPUT DAISY Register"]
pub mod csi_data07_select_input;
#[doc = "CSI_DATA08_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_data08_select_input](csi_data08_select_input) module"]
pub type CSI_DATA08_SELECT_INPUT = crate::Reg<u32, _CSI_DATA08_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_DATA08_SELECT_INPUT;
#[doc = "`read()` method returns [csi_data08_select_input::R](csi_data08_select_input::R) reader structure"]
impl crate::Readable for CSI_DATA08_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_data08_select_input::W](csi_data08_select_input::W) writer structure"]
impl crate::Writable for CSI_DATA08_SELECT_INPUT {}
#[doc = "CSI_DATA08_SELECT_INPUT DAISY Register"]
pub mod csi_data08_select_input;
#[doc = "CSI_DATA09_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_data09_select_input](csi_data09_select_input) module"]
pub type CSI_DATA09_SELECT_INPUT = crate::Reg<u32, _CSI_DATA09_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_DATA09_SELECT_INPUT;
#[doc = "`read()` method returns [csi_data09_select_input::R](csi_data09_select_input::R) reader structure"]
impl crate::Readable for CSI_DATA09_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_data09_select_input::W](csi_data09_select_input::W) writer structure"]
impl crate::Writable for CSI_DATA09_SELECT_INPUT {}
#[doc = "CSI_DATA09_SELECT_INPUT DAISY Register"]
pub mod csi_data09_select_input;
#[doc = "CSI_HSYNC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_hsync_select_input](csi_hsync_select_input) module"]
pub type CSI_HSYNC_SELECT_INPUT = crate::Reg<u32, _CSI_HSYNC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_HSYNC_SELECT_INPUT;
#[doc = "`read()` method returns [csi_hsync_select_input::R](csi_hsync_select_input::R) reader structure"]
impl crate::Readable for CSI_HSYNC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_hsync_select_input::W](csi_hsync_select_input::W) writer structure"]
impl crate::Writable for CSI_HSYNC_SELECT_INPUT {}
#[doc = "CSI_HSYNC_SELECT_INPUT DAISY Register"]
pub mod csi_hsync_select_input;
#[doc = "CSI_PIXCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_pixclk_select_input](csi_pixclk_select_input) module"]
pub type CSI_PIXCLK_SELECT_INPUT = crate::Reg<u32, _CSI_PIXCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_PIXCLK_SELECT_INPUT;
#[doc = "`read()` method returns [csi_pixclk_select_input::R](csi_pixclk_select_input::R) reader structure"]
impl crate::Readable for CSI_PIXCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_pixclk_select_input::W](csi_pixclk_select_input::W) writer structure"]
impl crate::Writable for CSI_PIXCLK_SELECT_INPUT {}
#[doc = "CSI_PIXCLK_SELECT_INPUT DAISY Register"]
pub mod csi_pixclk_select_input;
#[doc = "CSI_VSYNC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csi_vsync_select_input](csi_vsync_select_input) module"]
pub type CSI_VSYNC_SELECT_INPUT = crate::Reg<u32, _CSI_VSYNC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSI_VSYNC_SELECT_INPUT;
#[doc = "`read()` method returns [csi_vsync_select_input::R](csi_vsync_select_input::R) reader structure"]
impl crate::Readable for CSI_VSYNC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [csi_vsync_select_input::W](csi_vsync_select_input::W) writer structure"]
impl crate::Writable for CSI_VSYNC_SELECT_INPUT {}
#[doc = "CSI_VSYNC_SELECT_INPUT DAISY Register"]
pub mod csi_vsync_select_input;
#[doc = "ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet_ipg_clk_rmii_select_input](enet_ipg_clk_rmii_select_input) module"]
pub type ENET_IPG_CLK_RMII_SELECT_INPUT = crate::Reg<u32, _ENET_IPG_CLK_RMII_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET_IPG_CLK_RMII_SELECT_INPUT;
#[doc = "`read()` method returns [enet_ipg_clk_rmii_select_input::R](enet_ipg_clk_rmii_select_input::R) reader structure"]
impl crate::Readable for ENET_IPG_CLK_RMII_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet_ipg_clk_rmii_select_input::W](enet_ipg_clk_rmii_select_input::W) writer structure"]
impl crate::Writable for ENET_IPG_CLK_RMII_SELECT_INPUT {}
#[doc = "ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
pub mod enet_ipg_clk_rmii_select_input;
#[doc = "ENET_MDIO_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet_mdio_select_input](enet_mdio_select_input) module"]
pub type ENET_MDIO_SELECT_INPUT = crate::Reg<u32, _ENET_MDIO_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET_MDIO_SELECT_INPUT;
#[doc = "`read()` method returns [enet_mdio_select_input::R](enet_mdio_select_input::R) reader structure"]
impl crate::Readable for ENET_MDIO_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet_mdio_select_input::W](enet_mdio_select_input::W) writer structure"]
impl crate::Writable for ENET_MDIO_SELECT_INPUT {}
#[doc = "ENET_MDIO_SELECT_INPUT DAISY Register"]
pub mod enet_mdio_select_input;
#[doc = "ENET0_RXDATA_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet0_rxdata_select_input](enet0_rxdata_select_input) module"]
pub type ENET0_RXDATA_SELECT_INPUT = crate::Reg<u32, _ENET0_RXDATA_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET0_RXDATA_SELECT_INPUT;
#[doc = "`read()` method returns [enet0_rxdata_select_input::R](enet0_rxdata_select_input::R) reader structure"]
impl crate::Readable for ENET0_RXDATA_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet0_rxdata_select_input::W](enet0_rxdata_select_input::W) writer structure"]
impl crate::Writable for ENET0_RXDATA_SELECT_INPUT {}
#[doc = "ENET0_RXDATA_SELECT_INPUT DAISY Register"]
pub mod enet0_rxdata_select_input;
#[doc = "ENET1_RXDATA_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet1_rxdata_select_input](enet1_rxdata_select_input) module"]
pub type ENET1_RXDATA_SELECT_INPUT = crate::Reg<u32, _ENET1_RXDATA_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET1_RXDATA_SELECT_INPUT;
#[doc = "`read()` method returns [enet1_rxdata_select_input::R](enet1_rxdata_select_input::R) reader structure"]
impl crate::Readable for ENET1_RXDATA_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet1_rxdata_select_input::W](enet1_rxdata_select_input::W) writer structure"]
impl crate::Writable for ENET1_RXDATA_SELECT_INPUT {}
#[doc = "ENET1_RXDATA_SELECT_INPUT DAISY Register"]
pub mod enet1_rxdata_select_input;
#[doc = "ENET_RXEN_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet_rxen_select_input](enet_rxen_select_input) module"]
pub type ENET_RXEN_SELECT_INPUT = crate::Reg<u32, _ENET_RXEN_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET_RXEN_SELECT_INPUT;
#[doc = "`read()` method returns [enet_rxen_select_input::R](enet_rxen_select_input::R) reader structure"]
impl crate::Readable for ENET_RXEN_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet_rxen_select_input::W](enet_rxen_select_input::W) writer structure"]
impl crate::Writable for ENET_RXEN_SELECT_INPUT {}
#[doc = "ENET_RXEN_SELECT_INPUT DAISY Register"]
pub mod enet_rxen_select_input;
#[doc = "ENET_RXERR_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet_rxerr_select_input](enet_rxerr_select_input) module"]
pub type ENET_RXERR_SELECT_INPUT = crate::Reg<u32, _ENET_RXERR_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET_RXERR_SELECT_INPUT;
#[doc = "`read()` method returns [enet_rxerr_select_input::R](enet_rxerr_select_input::R) reader structure"]
impl crate::Readable for ENET_RXERR_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet_rxerr_select_input::W](enet_rxerr_select_input::W) writer structure"]
impl crate::Writable for ENET_RXERR_SELECT_INPUT {}
#[doc = "ENET_RXERR_SELECT_INPUT DAISY Register"]
pub mod enet_rxerr_select_input;
#[doc = "ENET0_TIMER_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet0_timer_select_input](enet0_timer_select_input) module"]
pub type ENET0_TIMER_SELECT_INPUT = crate::Reg<u32, _ENET0_TIMER_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET0_TIMER_SELECT_INPUT;
#[doc = "`read()` method returns [enet0_timer_select_input::R](enet0_timer_select_input::R) reader structure"]
impl crate::Readable for ENET0_TIMER_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet0_timer_select_input::W](enet0_timer_select_input::W) writer structure"]
impl crate::Writable for ENET0_TIMER_SELECT_INPUT {}
#[doc = "ENET0_TIMER_SELECT_INPUT DAISY Register"]
pub mod enet0_timer_select_input;
#[doc = "ENET_TXCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet_txclk_select_input](enet_txclk_select_input) module"]
pub type ENET_TXCLK_SELECT_INPUT = crate::Reg<u32, _ENET_TXCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET_TXCLK_SELECT_INPUT;
#[doc = "`read()` method returns [enet_txclk_select_input::R](enet_txclk_select_input::R) reader structure"]
impl crate::Readable for ENET_TXCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet_txclk_select_input::W](enet_txclk_select_input::W) writer structure"]
impl crate::Writable for ENET_TXCLK_SELECT_INPUT {}
#[doc = "ENET_TXCLK_SELECT_INPUT DAISY Register"]
pub mod enet_txclk_select_input;
#[doc = "FLEXCAN1_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexcan1_rx_select_input](flexcan1_rx_select_input) module"]
pub type FLEXCAN1_RX_SELECT_INPUT = crate::Reg<u32, _FLEXCAN1_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXCAN1_RX_SELECT_INPUT;
#[doc = "`read()` method returns [flexcan1_rx_select_input::R](flexcan1_rx_select_input::R) reader structure"]
impl crate::Readable for FLEXCAN1_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexcan1_rx_select_input::W](flexcan1_rx_select_input::W) writer structure"]
impl crate::Writable for FLEXCAN1_RX_SELECT_INPUT {}
#[doc = "FLEXCAN1_RX_SELECT_INPUT DAISY Register"]
pub mod flexcan1_rx_select_input;
#[doc = "FLEXCAN2_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexcan2_rx_select_input](flexcan2_rx_select_input) module"]
pub type FLEXCAN2_RX_SELECT_INPUT = crate::Reg<u32, _FLEXCAN2_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXCAN2_RX_SELECT_INPUT;
#[doc = "`read()` method returns [flexcan2_rx_select_input::R](flexcan2_rx_select_input::R) reader structure"]
impl crate::Readable for FLEXCAN2_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexcan2_rx_select_input::W](flexcan2_rx_select_input::W) writer structure"]
impl crate::Writable for FLEXCAN2_RX_SELECT_INPUT {}
#[doc = "FLEXCAN2_RX_SELECT_INPUT DAISY Register"]
pub mod flexcan2_rx_select_input;
#[doc = "FLEXPWM1_PWMA3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm1_pwma3_select_input](flexpwm1_pwma3_select_input) module"]
pub type FLEXPWM1_PWMA3_SELECT_INPUT = crate::Reg<u32, _FLEXPWM1_PWMA3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM1_PWMA3_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm1_pwma3_select_input::R](flexpwm1_pwma3_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMA3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwma3_select_input::W](flexpwm1_pwma3_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMA3_SELECT_INPUT {}
#[doc = "FLEXPWM1_PWMA3_SELECT_INPUT DAISY Register"]
pub mod flexpwm1_pwma3_select_input;
#[doc = "FLEXPWM1_PWMA0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm1_pwma0_select_input](flexpwm1_pwma0_select_input) module"]
pub type FLEXPWM1_PWMA0_SELECT_INPUT = crate::Reg<u32, _FLEXPWM1_PWMA0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM1_PWMA0_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm1_pwma0_select_input::R](flexpwm1_pwma0_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMA0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwma0_select_input::W](flexpwm1_pwma0_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMA0_SELECT_INPUT {}
#[doc = "FLEXPWM1_PWMA0_SELECT_INPUT DAISY Register"]
pub mod flexpwm1_pwma0_select_input;
#[doc = "FLEXPWM1_PWMA1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm1_pwma1_select_input](flexpwm1_pwma1_select_input) module"]
pub type FLEXPWM1_PWMA1_SELECT_INPUT = crate::Reg<u32, _FLEXPWM1_PWMA1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM1_PWMA1_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm1_pwma1_select_input::R](flexpwm1_pwma1_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMA1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwma1_select_input::W](flexpwm1_pwma1_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMA1_SELECT_INPUT {}
#[doc = "FLEXPWM1_PWMA1_SELECT_INPUT DAISY Register"]
pub mod flexpwm1_pwma1_select_input;
#[doc = "FLEXPWM1_PWMA2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm1_pwma2_select_input](flexpwm1_pwma2_select_input) module"]
pub type FLEXPWM1_PWMA2_SELECT_INPUT = crate::Reg<u32, _FLEXPWM1_PWMA2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM1_PWMA2_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm1_pwma2_select_input::R](flexpwm1_pwma2_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMA2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwma2_select_input::W](flexpwm1_pwma2_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMA2_SELECT_INPUT {}
#[doc = "FLEXPWM1_PWMA2_SELECT_INPUT DAISY Register"]
pub mod flexpwm1_pwma2_select_input;
#[doc = "FLEXPWM1_PWMB3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm1_pwmb3_select_input](flexpwm1_pwmb3_select_input) module"]
pub type FLEXPWM1_PWMB3_SELECT_INPUT = crate::Reg<u32, _FLEXPWM1_PWMB3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM1_PWMB3_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm1_pwmb3_select_input::R](flexpwm1_pwmb3_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMB3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwmb3_select_input::W](flexpwm1_pwmb3_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMB3_SELECT_INPUT {}
#[doc = "FLEXPWM1_PWMB3_SELECT_INPUT DAISY Register"]
pub mod flexpwm1_pwmb3_select_input;
#[doc = "FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm1_pwmb0_select_input](flexpwm1_pwmb0_select_input) module"]
pub type FLEXPWM1_PWMB0_SELECT_INPUT = crate::Reg<u32, _FLEXPWM1_PWMB0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM1_PWMB0_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm1_pwmb0_select_input::R](flexpwm1_pwmb0_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMB0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwmb0_select_input::W](flexpwm1_pwmb0_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMB0_SELECT_INPUT {}
#[doc = "FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register"]
pub mod flexpwm1_pwmb0_select_input;
#[doc = "FLEXPWM1_PWMB1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm1_pwmb1_select_input](flexpwm1_pwmb1_select_input) module"]
pub type FLEXPWM1_PWMB1_SELECT_INPUT = crate::Reg<u32, _FLEXPWM1_PWMB1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM1_PWMB1_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm1_pwmb1_select_input::R](flexpwm1_pwmb1_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMB1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwmb1_select_input::W](flexpwm1_pwmb1_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMB1_SELECT_INPUT {}
#[doc = "FLEXPWM1_PWMB1_SELECT_INPUT DAISY Register"]
pub mod flexpwm1_pwmb1_select_input;
#[doc = "FLEXPWM1_PWMB2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm1_pwmb2_select_input](flexpwm1_pwmb2_select_input) module"]
pub type FLEXPWM1_PWMB2_SELECT_INPUT = crate::Reg<u32, _FLEXPWM1_PWMB2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM1_PWMB2_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm1_pwmb2_select_input::R](flexpwm1_pwmb2_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMB2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwmb2_select_input::W](flexpwm1_pwmb2_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMB2_SELECT_INPUT {}
#[doc = "FLEXPWM1_PWMB2_SELECT_INPUT DAISY Register"]
pub mod flexpwm1_pwmb2_select_input;
#[doc = "FLEXPWM2_PWMA3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm2_pwma3_select_input](flexpwm2_pwma3_select_input) module"]
pub type FLEXPWM2_PWMA3_SELECT_INPUT = crate::Reg<u32, _FLEXPWM2_PWMA3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM2_PWMA3_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm2_pwma3_select_input::R](flexpwm2_pwma3_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM2_PWMA3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm2_pwma3_select_input::W](flexpwm2_pwma3_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM2_PWMA3_SELECT_INPUT {}
#[doc = "FLEXPWM2_PWMA3_SELECT_INPUT DAISY Register"]
pub mod flexpwm2_pwma3_select_input;
#[doc = "FLEXPWM2_PWMA0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm2_pwma0_select_input](flexpwm2_pwma0_select_input) module"]
pub type FLEXPWM2_PWMA0_SELECT_INPUT = crate::Reg<u32, _FLEXPWM2_PWMA0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM2_PWMA0_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm2_pwma0_select_input::R](flexpwm2_pwma0_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM2_PWMA0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm2_pwma0_select_input::W](flexpwm2_pwma0_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM2_PWMA0_SELECT_INPUT {}
#[doc = "FLEXPWM2_PWMA0_SELECT_INPUT DAISY Register"]
pub mod flexpwm2_pwma0_select_input;
#[doc = "FLEXPWM2_PWMA1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm2_pwma1_select_input](flexpwm2_pwma1_select_input) module"]
pub type FLEXPWM2_PWMA1_SELECT_INPUT = crate::Reg<u32, _FLEXPWM2_PWMA1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM2_PWMA1_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm2_pwma1_select_input::R](flexpwm2_pwma1_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM2_PWMA1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm2_pwma1_select_input::W](flexpwm2_pwma1_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM2_PWMA1_SELECT_INPUT {}
#[doc = "FLEXPWM2_PWMA1_SELECT_INPUT DAISY Register"]
pub mod flexpwm2_pwma1_select_input;
#[doc = "FLEXPWM2_PWMA2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm2_pwma2_select_input](flexpwm2_pwma2_select_input) module"]
pub type FLEXPWM2_PWMA2_SELECT_INPUT = crate::Reg<u32, _FLEXPWM2_PWMA2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM2_PWMA2_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm2_pwma2_select_input::R](flexpwm2_pwma2_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM2_PWMA2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm2_pwma2_select_input::W](flexpwm2_pwma2_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM2_PWMA2_SELECT_INPUT {}
#[doc = "FLEXPWM2_PWMA2_SELECT_INPUT DAISY Register"]
pub mod flexpwm2_pwma2_select_input;
#[doc = "FLEXPWM2_PWMB3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm2_pwmb3_select_input](flexpwm2_pwmb3_select_input) module"]
pub type FLEXPWM2_PWMB3_SELECT_INPUT = crate::Reg<u32, _FLEXPWM2_PWMB3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM2_PWMB3_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm2_pwmb3_select_input::R](flexpwm2_pwmb3_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM2_PWMB3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm2_pwmb3_select_input::W](flexpwm2_pwmb3_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM2_PWMB3_SELECT_INPUT {}
#[doc = "FLEXPWM2_PWMB3_SELECT_INPUT DAISY Register"]
pub mod flexpwm2_pwmb3_select_input;
#[doc = "FLEXPWM2_PWMB0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm2_pwmb0_select_input](flexpwm2_pwmb0_select_input) module"]
pub type FLEXPWM2_PWMB0_SELECT_INPUT = crate::Reg<u32, _FLEXPWM2_PWMB0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM2_PWMB0_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm2_pwmb0_select_input::R](flexpwm2_pwmb0_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM2_PWMB0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm2_pwmb0_select_input::W](flexpwm2_pwmb0_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM2_PWMB0_SELECT_INPUT {}
#[doc = "FLEXPWM2_PWMB0_SELECT_INPUT DAISY Register"]
pub mod flexpwm2_pwmb0_select_input;
#[doc = "FLEXPWM2_PWMB1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm2_pwmb1_select_input](flexpwm2_pwmb1_select_input) module"]
pub type FLEXPWM2_PWMB1_SELECT_INPUT = crate::Reg<u32, _FLEXPWM2_PWMB1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM2_PWMB1_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm2_pwmb1_select_input::R](flexpwm2_pwmb1_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM2_PWMB1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm2_pwmb1_select_input::W](flexpwm2_pwmb1_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM2_PWMB1_SELECT_INPUT {}
#[doc = "FLEXPWM2_PWMB1_SELECT_INPUT DAISY Register"]
pub mod flexpwm2_pwmb1_select_input;
#[doc = "FLEXPWM2_PWMB2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm2_pwmb2_select_input](flexpwm2_pwmb2_select_input) module"]
pub type FLEXPWM2_PWMB2_SELECT_INPUT = crate::Reg<u32, _FLEXPWM2_PWMB2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM2_PWMB2_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm2_pwmb2_select_input::R](flexpwm2_pwmb2_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM2_PWMB2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm2_pwmb2_select_input::W](flexpwm2_pwmb2_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM2_PWMB2_SELECT_INPUT {}
#[doc = "FLEXPWM2_PWMB2_SELECT_INPUT DAISY Register"]
pub mod flexpwm2_pwmb2_select_input;
#[doc = "FLEXPWM4_PWMA0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm4_pwma0_select_input](flexpwm4_pwma0_select_input) module"]
pub type FLEXPWM4_PWMA0_SELECT_INPUT = crate::Reg<u32, _FLEXPWM4_PWMA0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM4_PWMA0_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm4_pwma0_select_input::R](flexpwm4_pwma0_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM4_PWMA0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm4_pwma0_select_input::W](flexpwm4_pwma0_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM4_PWMA0_SELECT_INPUT {}
#[doc = "FLEXPWM4_PWMA0_SELECT_INPUT DAISY Register"]
pub mod flexpwm4_pwma0_select_input;
#[doc = "FLEXPWM4_PWMA1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm4_pwma1_select_input](flexpwm4_pwma1_select_input) module"]
pub type FLEXPWM4_PWMA1_SELECT_INPUT = crate::Reg<u32, _FLEXPWM4_PWMA1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM4_PWMA1_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm4_pwma1_select_input::R](flexpwm4_pwma1_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM4_PWMA1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm4_pwma1_select_input::W](flexpwm4_pwma1_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM4_PWMA1_SELECT_INPUT {}
#[doc = "FLEXPWM4_PWMA1_SELECT_INPUT DAISY Register"]
pub mod flexpwm4_pwma1_select_input;
#[doc = "FLEXPWM4_PWMA2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm4_pwma2_select_input](flexpwm4_pwma2_select_input) module"]
pub type FLEXPWM4_PWMA2_SELECT_INPUT = crate::Reg<u32, _FLEXPWM4_PWMA2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM4_PWMA2_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm4_pwma2_select_input::R](flexpwm4_pwma2_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM4_PWMA2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm4_pwma2_select_input::W](flexpwm4_pwma2_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM4_PWMA2_SELECT_INPUT {}
#[doc = "FLEXPWM4_PWMA2_SELECT_INPUT DAISY Register"]
pub mod flexpwm4_pwma2_select_input;
#[doc = "FLEXPWM4_PWMA3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexpwm4_pwma3_select_input](flexpwm4_pwma3_select_input) module"]
pub type FLEXPWM4_PWMA3_SELECT_INPUT = crate::Reg<u32, _FLEXPWM4_PWMA3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXPWM4_PWMA3_SELECT_INPUT;
#[doc = "`read()` method returns [flexpwm4_pwma3_select_input::R](flexpwm4_pwma3_select_input::R) reader structure"]
impl crate::Readable for FLEXPWM4_PWMA3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexpwm4_pwma3_select_input::W](flexpwm4_pwma3_select_input::W) writer structure"]
impl crate::Writable for FLEXPWM4_PWMA3_SELECT_INPUT {}
#[doc = "FLEXPWM4_PWMA3_SELECT_INPUT DAISY Register"]
pub mod flexpwm4_pwma3_select_input;
#[doc = "FLEXSPIA_DQS_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspia_dqs_select_input](flexspia_dqs_select_input) module"]
pub type FLEXSPIA_DQS_SELECT_INPUT = crate::Reg<u32, _FLEXSPIA_DQS_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIA_DQS_SELECT_INPUT;
#[doc = "`read()` method returns [flexspia_dqs_select_input::R](flexspia_dqs_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIA_DQS_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspia_dqs_select_input::W](flexspia_dqs_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIA_DQS_SELECT_INPUT {}
#[doc = "FLEXSPIA_DQS_SELECT_INPUT DAISY Register"]
pub mod flexspia_dqs_select_input;
#[doc = "FLEXSPIA_DATA0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspia_data0_select_input](flexspia_data0_select_input) module"]
pub type FLEXSPIA_DATA0_SELECT_INPUT = crate::Reg<u32, _FLEXSPIA_DATA0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIA_DATA0_SELECT_INPUT;
#[doc = "`read()` method returns [flexspia_data0_select_input::R](flexspia_data0_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIA_DATA0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspia_data0_select_input::W](flexspia_data0_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIA_DATA0_SELECT_INPUT {}
#[doc = "FLEXSPIA_DATA0_SELECT_INPUT DAISY Register"]
pub mod flexspia_data0_select_input;
#[doc = "FLEXSPIA_DATA1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspia_data1_select_input](flexspia_data1_select_input) module"]
pub type FLEXSPIA_DATA1_SELECT_INPUT = crate::Reg<u32, _FLEXSPIA_DATA1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIA_DATA1_SELECT_INPUT;
#[doc = "`read()` method returns [flexspia_data1_select_input::R](flexspia_data1_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIA_DATA1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspia_data1_select_input::W](flexspia_data1_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIA_DATA1_SELECT_INPUT {}
#[doc = "FLEXSPIA_DATA1_SELECT_INPUT DAISY Register"]
pub mod flexspia_data1_select_input;
#[doc = "FLEXSPIA_DATA2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspia_data2_select_input](flexspia_data2_select_input) module"]
pub type FLEXSPIA_DATA2_SELECT_INPUT = crate::Reg<u32, _FLEXSPIA_DATA2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIA_DATA2_SELECT_INPUT;
#[doc = "`read()` method returns [flexspia_data2_select_input::R](flexspia_data2_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIA_DATA2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspia_data2_select_input::W](flexspia_data2_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIA_DATA2_SELECT_INPUT {}
#[doc = "FLEXSPIA_DATA2_SELECT_INPUT DAISY Register"]
pub mod flexspia_data2_select_input;
#[doc = "FLEXSPIA_DATA3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspia_data3_select_input](flexspia_data3_select_input) module"]
pub type FLEXSPIA_DATA3_SELECT_INPUT = crate::Reg<u32, _FLEXSPIA_DATA3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIA_DATA3_SELECT_INPUT;
#[doc = "`read()` method returns [flexspia_data3_select_input::R](flexspia_data3_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIA_DATA3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspia_data3_select_input::W](flexspia_data3_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIA_DATA3_SELECT_INPUT {}
#[doc = "FLEXSPIA_DATA3_SELECT_INPUT DAISY Register"]
pub mod flexspia_data3_select_input;
#[doc = "FLEXSPIB_DATA0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspib_data0_select_input](flexspib_data0_select_input) module"]
pub type FLEXSPIB_DATA0_SELECT_INPUT = crate::Reg<u32, _FLEXSPIB_DATA0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIB_DATA0_SELECT_INPUT;
#[doc = "`read()` method returns [flexspib_data0_select_input::R](flexspib_data0_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIB_DATA0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspib_data0_select_input::W](flexspib_data0_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIB_DATA0_SELECT_INPUT {}
#[doc = "FLEXSPIB_DATA0_SELECT_INPUT DAISY Register"]
pub mod flexspib_data0_select_input;
#[doc = "FLEXSPIB_DATA1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspib_data1_select_input](flexspib_data1_select_input) module"]
pub type FLEXSPIB_DATA1_SELECT_INPUT = crate::Reg<u32, _FLEXSPIB_DATA1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIB_DATA1_SELECT_INPUT;
#[doc = "`read()` method returns [flexspib_data1_select_input::R](flexspib_data1_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIB_DATA1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspib_data1_select_input::W](flexspib_data1_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIB_DATA1_SELECT_INPUT {}
#[doc = "FLEXSPIB_DATA1_SELECT_INPUT DAISY Register"]
pub mod flexspib_data1_select_input;
#[doc = "FLEXSPIB_DATA2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspib_data2_select_input](flexspib_data2_select_input) module"]
pub type FLEXSPIB_DATA2_SELECT_INPUT = crate::Reg<u32, _FLEXSPIB_DATA2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIB_DATA2_SELECT_INPUT;
#[doc = "`read()` method returns [flexspib_data2_select_input::R](flexspib_data2_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIB_DATA2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspib_data2_select_input::W](flexspib_data2_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIB_DATA2_SELECT_INPUT {}
#[doc = "FLEXSPIB_DATA2_SELECT_INPUT DAISY Register"]
pub mod flexspib_data2_select_input;
#[doc = "FLEXSPIB_DATA3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspib_data3_select_input](flexspib_data3_select_input) module"]
pub type FLEXSPIB_DATA3_SELECT_INPUT = crate::Reg<u32, _FLEXSPIB_DATA3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIB_DATA3_SELECT_INPUT;
#[doc = "`read()` method returns [flexspib_data3_select_input::R](flexspib_data3_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIB_DATA3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspib_data3_select_input::W](flexspib_data3_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIB_DATA3_SELECT_INPUT {}
#[doc = "FLEXSPIB_DATA3_SELECT_INPUT DAISY Register"]
pub mod flexspib_data3_select_input;
#[doc = "FLEXSPIA_SCK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspia_sck_select_input](flexspia_sck_select_input) module"]
pub type FLEXSPIA_SCK_SELECT_INPUT = crate::Reg<u32, _FLEXSPIA_SCK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPIA_SCK_SELECT_INPUT;
#[doc = "`read()` method returns [flexspia_sck_select_input::R](flexspia_sck_select_input::R) reader structure"]
impl crate::Readable for FLEXSPIA_SCK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspia_sck_select_input::W](flexspia_sck_select_input::W) writer structure"]
impl crate::Writable for FLEXSPIA_SCK_SELECT_INPUT {}
#[doc = "FLEXSPIA_SCK_SELECT_INPUT DAISY Register"]
pub mod flexspia_sck_select_input;
#[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c1_scl_select_input](lpi2c1_scl_select_input) module"]
pub type LPI2C1_SCL_SELECT_INPUT = crate::Reg<u32, _LPI2C1_SCL_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C1_SCL_SELECT_INPUT;
#[doc = "`read()` method returns [lpi2c1_scl_select_input::R](lpi2c1_scl_select_input::R) reader structure"]
impl crate::Readable for LPI2C1_SCL_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpi2c1_scl_select_input::W](lpi2c1_scl_select_input::W) writer structure"]
impl crate::Writable for LPI2C1_SCL_SELECT_INPUT {}
#[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
pub mod lpi2c1_scl_select_input;
#[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c1_sda_select_input](lpi2c1_sda_select_input) module"]
pub type LPI2C1_SDA_SELECT_INPUT = crate::Reg<u32, _LPI2C1_SDA_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C1_SDA_SELECT_INPUT;
#[doc = "`read()` method returns [lpi2c1_sda_select_input::R](lpi2c1_sda_select_input::R) reader structure"]
impl crate::Readable for LPI2C1_SDA_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpi2c1_sda_select_input::W](lpi2c1_sda_select_input::W) writer structure"]
impl crate::Writable for LPI2C1_SDA_SELECT_INPUT {}
#[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
pub mod lpi2c1_sda_select_input;
#[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c2_scl_select_input](lpi2c2_scl_select_input) module"]
pub type LPI2C2_SCL_SELECT_INPUT = crate::Reg<u32, _LPI2C2_SCL_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C2_SCL_SELECT_INPUT;
#[doc = "`read()` method returns [lpi2c2_scl_select_input::R](lpi2c2_scl_select_input::R) reader structure"]
impl crate::Readable for LPI2C2_SCL_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpi2c2_scl_select_input::W](lpi2c2_scl_select_input::W) writer structure"]
impl crate::Writable for LPI2C2_SCL_SELECT_INPUT {}
#[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
pub mod lpi2c2_scl_select_input;
#[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c2_sda_select_input](lpi2c2_sda_select_input) module"]
pub type LPI2C2_SDA_SELECT_INPUT = crate::Reg<u32, _LPI2C2_SDA_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C2_SDA_SELECT_INPUT;
#[doc = "`read()` method returns [lpi2c2_sda_select_input::R](lpi2c2_sda_select_input::R) reader structure"]
impl crate::Readable for LPI2C2_SDA_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpi2c2_sda_select_input::W](lpi2c2_sda_select_input::W) writer structure"]
impl crate::Writable for LPI2C2_SDA_SELECT_INPUT {}
#[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
pub mod lpi2c2_sda_select_input;
#[doc = "LPI2C3_SCL_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c3_scl_select_input](lpi2c3_scl_select_input) module"]
pub type LPI2C3_SCL_SELECT_INPUT = crate::Reg<u32, _LPI2C3_SCL_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C3_SCL_SELECT_INPUT;
#[doc = "`read()` method returns [lpi2c3_scl_select_input::R](lpi2c3_scl_select_input::R) reader structure"]
impl crate::Readable for LPI2C3_SCL_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpi2c3_scl_select_input::W](lpi2c3_scl_select_input::W) writer structure"]
impl crate::Writable for LPI2C3_SCL_SELECT_INPUT {}
#[doc = "LPI2C3_SCL_SELECT_INPUT DAISY Register"]
pub mod lpi2c3_scl_select_input;
#[doc = "LPI2C3_SDA_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c3_sda_select_input](lpi2c3_sda_select_input) module"]
pub type LPI2C3_SDA_SELECT_INPUT = crate::Reg<u32, _LPI2C3_SDA_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C3_SDA_SELECT_INPUT;
#[doc = "`read()` method returns [lpi2c3_sda_select_input::R](lpi2c3_sda_select_input::R) reader structure"]
impl crate::Readable for LPI2C3_SDA_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpi2c3_sda_select_input::W](lpi2c3_sda_select_input::W) writer structure"]
impl crate::Writable for LPI2C3_SDA_SELECT_INPUT {}
#[doc = "LPI2C3_SDA_SELECT_INPUT DAISY Register"]
pub mod lpi2c3_sda_select_input;
#[doc = "LPI2C4_SCL_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c4_scl_select_input](lpi2c4_scl_select_input) module"]
pub type LPI2C4_SCL_SELECT_INPUT = crate::Reg<u32, _LPI2C4_SCL_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C4_SCL_SELECT_INPUT;
#[doc = "`read()` method returns [lpi2c4_scl_select_input::R](lpi2c4_scl_select_input::R) reader structure"]
impl crate::Readable for LPI2C4_SCL_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpi2c4_scl_select_input::W](lpi2c4_scl_select_input::W) writer structure"]
impl crate::Writable for LPI2C4_SCL_SELECT_INPUT {}
#[doc = "LPI2C4_SCL_SELECT_INPUT DAISY Register"]
pub mod lpi2c4_scl_select_input;
#[doc = "LPI2C4_SDA_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpi2c4_sda_select_input](lpi2c4_sda_select_input) module"]
pub type LPI2C4_SDA_SELECT_INPUT = crate::Reg<u32, _LPI2C4_SDA_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPI2C4_SDA_SELECT_INPUT;
#[doc = "`read()` method returns [lpi2c4_sda_select_input::R](lpi2c4_sda_select_input::R) reader structure"]
impl crate::Readable for LPI2C4_SDA_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpi2c4_sda_select_input::W](lpi2c4_sda_select_input::W) writer structure"]
impl crate::Writable for LPI2C4_SDA_SELECT_INPUT {}
#[doc = "LPI2C4_SDA_SELECT_INPUT DAISY Register"]
pub mod lpi2c4_sda_select_input;
#[doc = "LPSPI1_PCS0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi1_pcs0_select_input](lpspi1_pcs0_select_input) module"]
pub type LPSPI1_PCS0_SELECT_INPUT = crate::Reg<u32, _LPSPI1_PCS0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI1_PCS0_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi1_pcs0_select_input::R](lpspi1_pcs0_select_input::R) reader structure"]
impl crate::Readable for LPSPI1_PCS0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi1_pcs0_select_input::W](lpspi1_pcs0_select_input::W) writer structure"]
impl crate::Writable for LPSPI1_PCS0_SELECT_INPUT {}
#[doc = "LPSPI1_PCS0_SELECT_INPUT DAISY Register"]
pub mod lpspi1_pcs0_select_input;
#[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi1_sck_select_input](lpspi1_sck_select_input) module"]
pub type LPSPI1_SCK_SELECT_INPUT = crate::Reg<u32, _LPSPI1_SCK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI1_SCK_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi1_sck_select_input::R](lpspi1_sck_select_input::R) reader structure"]
impl crate::Readable for LPSPI1_SCK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi1_sck_select_input::W](lpspi1_sck_select_input::W) writer structure"]
impl crate::Writable for LPSPI1_SCK_SELECT_INPUT {}
#[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
pub mod lpspi1_sck_select_input;
#[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi1_sdi_select_input](lpspi1_sdi_select_input) module"]
pub type LPSPI1_SDI_SELECT_INPUT = crate::Reg<u32, _LPSPI1_SDI_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI1_SDI_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi1_sdi_select_input::R](lpspi1_sdi_select_input::R) reader structure"]
impl crate::Readable for LPSPI1_SDI_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi1_sdi_select_input::W](lpspi1_sdi_select_input::W) writer structure"]
impl crate::Writable for LPSPI1_SDI_SELECT_INPUT {}
#[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
pub mod lpspi1_sdi_select_input;
#[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi1_sdo_select_input](lpspi1_sdo_select_input) module"]
pub type LPSPI1_SDO_SELECT_INPUT = crate::Reg<u32, _LPSPI1_SDO_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI1_SDO_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi1_sdo_select_input::R](lpspi1_sdo_select_input::R) reader structure"]
impl crate::Readable for LPSPI1_SDO_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi1_sdo_select_input::W](lpspi1_sdo_select_input::W) writer structure"]
impl crate::Writable for LPSPI1_SDO_SELECT_INPUT {}
#[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
pub mod lpspi1_sdo_select_input;
#[doc = "LPSPI2_PCS0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi2_pcs0_select_input](lpspi2_pcs0_select_input) module"]
pub type LPSPI2_PCS0_SELECT_INPUT = crate::Reg<u32, _LPSPI2_PCS0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI2_PCS0_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi2_pcs0_select_input::R](lpspi2_pcs0_select_input::R) reader structure"]
impl crate::Readable for LPSPI2_PCS0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi2_pcs0_select_input::W](lpspi2_pcs0_select_input::W) writer structure"]
impl crate::Writable for LPSPI2_PCS0_SELECT_INPUT {}
#[doc = "LPSPI2_PCS0_SELECT_INPUT DAISY Register"]
pub mod lpspi2_pcs0_select_input;
#[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi2_sck_select_input](lpspi2_sck_select_input) module"]
pub type LPSPI2_SCK_SELECT_INPUT = crate::Reg<u32, _LPSPI2_SCK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI2_SCK_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi2_sck_select_input::R](lpspi2_sck_select_input::R) reader structure"]
impl crate::Readable for LPSPI2_SCK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi2_sck_select_input::W](lpspi2_sck_select_input::W) writer structure"]
impl crate::Writable for LPSPI2_SCK_SELECT_INPUT {}
#[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
pub mod lpspi2_sck_select_input;
#[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi2_sdi_select_input](lpspi2_sdi_select_input) module"]
pub type LPSPI2_SDI_SELECT_INPUT = crate::Reg<u32, _LPSPI2_SDI_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI2_SDI_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi2_sdi_select_input::R](lpspi2_sdi_select_input::R) reader structure"]
impl crate::Readable for LPSPI2_SDI_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi2_sdi_select_input::W](lpspi2_sdi_select_input::W) writer structure"]
impl crate::Writable for LPSPI2_SDI_SELECT_INPUT {}
#[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
pub mod lpspi2_sdi_select_input;
#[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi2_sdo_select_input](lpspi2_sdo_select_input) module"]
pub type LPSPI2_SDO_SELECT_INPUT = crate::Reg<u32, _LPSPI2_SDO_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI2_SDO_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi2_sdo_select_input::R](lpspi2_sdo_select_input::R) reader structure"]
impl crate::Readable for LPSPI2_SDO_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi2_sdo_select_input::W](lpspi2_sdo_select_input::W) writer structure"]
impl crate::Writable for LPSPI2_SDO_SELECT_INPUT {}
#[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
pub mod lpspi2_sdo_select_input;
#[doc = "LPSPI3_PCS0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi3_pcs0_select_input](lpspi3_pcs0_select_input) module"]
pub type LPSPI3_PCS0_SELECT_INPUT = crate::Reg<u32, _LPSPI3_PCS0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI3_PCS0_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi3_pcs0_select_input::R](lpspi3_pcs0_select_input::R) reader structure"]
impl crate::Readable for LPSPI3_PCS0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi3_pcs0_select_input::W](lpspi3_pcs0_select_input::W) writer structure"]
impl crate::Writable for LPSPI3_PCS0_SELECT_INPUT {}
#[doc = "LPSPI3_PCS0_SELECT_INPUT DAISY Register"]
pub mod lpspi3_pcs0_select_input;
#[doc = "LPSPI3_SCK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi3_sck_select_input](lpspi3_sck_select_input) module"]
pub type LPSPI3_SCK_SELECT_INPUT = crate::Reg<u32, _LPSPI3_SCK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI3_SCK_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi3_sck_select_input::R](lpspi3_sck_select_input::R) reader structure"]
impl crate::Readable for LPSPI3_SCK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi3_sck_select_input::W](lpspi3_sck_select_input::W) writer structure"]
impl crate::Writable for LPSPI3_SCK_SELECT_INPUT {}
#[doc = "LPSPI3_SCK_SELECT_INPUT DAISY Register"]
pub mod lpspi3_sck_select_input;
#[doc = "LPSPI3_SDI_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi3_sdi_select_input](lpspi3_sdi_select_input) module"]
pub type LPSPI3_SDI_SELECT_INPUT = crate::Reg<u32, _LPSPI3_SDI_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI3_SDI_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi3_sdi_select_input::R](lpspi3_sdi_select_input::R) reader structure"]
impl crate::Readable for LPSPI3_SDI_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi3_sdi_select_input::W](lpspi3_sdi_select_input::W) writer structure"]
impl crate::Writable for LPSPI3_SDI_SELECT_INPUT {}
#[doc = "LPSPI3_SDI_SELECT_INPUT DAISY Register"]
pub mod lpspi3_sdi_select_input;
#[doc = "LPSPI3_SDO_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi3_sdo_select_input](lpspi3_sdo_select_input) module"]
pub type LPSPI3_SDO_SELECT_INPUT = crate::Reg<u32, _LPSPI3_SDO_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI3_SDO_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi3_sdo_select_input::R](lpspi3_sdo_select_input::R) reader structure"]
impl crate::Readable for LPSPI3_SDO_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi3_sdo_select_input::W](lpspi3_sdo_select_input::W) writer structure"]
impl crate::Writable for LPSPI3_SDO_SELECT_INPUT {}
#[doc = "LPSPI3_SDO_SELECT_INPUT DAISY Register"]
pub mod lpspi3_sdo_select_input;
#[doc = "LPSPI4_PCS0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi4_pcs0_select_input](lpspi4_pcs0_select_input) module"]
pub type LPSPI4_PCS0_SELECT_INPUT = crate::Reg<u32, _LPSPI4_PCS0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI4_PCS0_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi4_pcs0_select_input::R](lpspi4_pcs0_select_input::R) reader structure"]
impl crate::Readable for LPSPI4_PCS0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi4_pcs0_select_input::W](lpspi4_pcs0_select_input::W) writer structure"]
impl crate::Writable for LPSPI4_PCS0_SELECT_INPUT {}
#[doc = "LPSPI4_PCS0_SELECT_INPUT DAISY Register"]
pub mod lpspi4_pcs0_select_input;
#[doc = "LPSPI4_SCK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi4_sck_select_input](lpspi4_sck_select_input) module"]
pub type LPSPI4_SCK_SELECT_INPUT = crate::Reg<u32, _LPSPI4_SCK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI4_SCK_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi4_sck_select_input::R](lpspi4_sck_select_input::R) reader structure"]
impl crate::Readable for LPSPI4_SCK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi4_sck_select_input::W](lpspi4_sck_select_input::W) writer structure"]
impl crate::Writable for LPSPI4_SCK_SELECT_INPUT {}
#[doc = "LPSPI4_SCK_SELECT_INPUT DAISY Register"]
pub mod lpspi4_sck_select_input;
#[doc = "LPSPI4_SDI_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi4_sdi_select_input](lpspi4_sdi_select_input) module"]
pub type LPSPI4_SDI_SELECT_INPUT = crate::Reg<u32, _LPSPI4_SDI_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI4_SDI_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi4_sdi_select_input::R](lpspi4_sdi_select_input::R) reader structure"]
impl crate::Readable for LPSPI4_SDI_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi4_sdi_select_input::W](lpspi4_sdi_select_input::W) writer structure"]
impl crate::Writable for LPSPI4_SDI_SELECT_INPUT {}
#[doc = "LPSPI4_SDI_SELECT_INPUT DAISY Register"]
pub mod lpspi4_sdi_select_input;
#[doc = "LPSPI4_SDO_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpspi4_sdo_select_input](lpspi4_sdo_select_input) module"]
pub type LPSPI4_SDO_SELECT_INPUT = crate::Reg<u32, _LPSPI4_SDO_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSPI4_SDO_SELECT_INPUT;
#[doc = "`read()` method returns [lpspi4_sdo_select_input::R](lpspi4_sdo_select_input::R) reader structure"]
impl crate::Readable for LPSPI4_SDO_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpspi4_sdo_select_input::W](lpspi4_sdo_select_input::W) writer structure"]
impl crate::Writable for LPSPI4_SDO_SELECT_INPUT {}
#[doc = "LPSPI4_SDO_SELECT_INPUT DAISY Register"]
pub mod lpspi4_sdo_select_input;
#[doc = "LPUART2_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart2_rx_select_input](lpuart2_rx_select_input) module"]
pub type LPUART2_RX_SELECT_INPUT = crate::Reg<u32, _LPUART2_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART2_RX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart2_rx_select_input::R](lpuart2_rx_select_input::R) reader structure"]
impl crate::Readable for LPUART2_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart2_rx_select_input::W](lpuart2_rx_select_input::W) writer structure"]
impl crate::Writable for LPUART2_RX_SELECT_INPUT {}
#[doc = "LPUART2_RX_SELECT_INPUT DAISY Register"]
pub mod lpuart2_rx_select_input;
#[doc = "LPUART2_TX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart2_tx_select_input](lpuart2_tx_select_input) module"]
pub type LPUART2_TX_SELECT_INPUT = crate::Reg<u32, _LPUART2_TX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART2_TX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart2_tx_select_input::R](lpuart2_tx_select_input::R) reader structure"]
impl crate::Readable for LPUART2_TX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart2_tx_select_input::W](lpuart2_tx_select_input::W) writer structure"]
impl crate::Writable for LPUART2_TX_SELECT_INPUT {}
#[doc = "LPUART2_TX_SELECT_INPUT DAISY Register"]
pub mod lpuart2_tx_select_input;
#[doc = "LPUART3_CTS_B_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart3_cts_b_select_input](lpuart3_cts_b_select_input) module"]
pub type LPUART3_CTS_B_SELECT_INPUT = crate::Reg<u32, _LPUART3_CTS_B_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART3_CTS_B_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart3_cts_b_select_input::R](lpuart3_cts_b_select_input::R) reader structure"]
impl crate::Readable for LPUART3_CTS_B_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart3_cts_b_select_input::W](lpuart3_cts_b_select_input::W) writer structure"]
impl crate::Writable for LPUART3_CTS_B_SELECT_INPUT {}
#[doc = "LPUART3_CTS_B_SELECT_INPUT DAISY Register"]
pub mod lpuart3_cts_b_select_input;
#[doc = "LPUART3_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart3_rx_select_input](lpuart3_rx_select_input) module"]
pub type LPUART3_RX_SELECT_INPUT = crate::Reg<u32, _LPUART3_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART3_RX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart3_rx_select_input::R](lpuart3_rx_select_input::R) reader structure"]
impl crate::Readable for LPUART3_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart3_rx_select_input::W](lpuart3_rx_select_input::W) writer structure"]
impl crate::Writable for LPUART3_RX_SELECT_INPUT {}
#[doc = "LPUART3_RX_SELECT_INPUT DAISY Register"]
pub mod lpuart3_rx_select_input;
#[doc = "LPUART3_TX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart3_tx_select_input](lpuart3_tx_select_input) module"]
pub type LPUART3_TX_SELECT_INPUT = crate::Reg<u32, _LPUART3_TX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART3_TX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart3_tx_select_input::R](lpuart3_tx_select_input::R) reader structure"]
impl crate::Readable for LPUART3_TX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart3_tx_select_input::W](lpuart3_tx_select_input::W) writer structure"]
impl crate::Writable for LPUART3_TX_SELECT_INPUT {}
#[doc = "LPUART3_TX_SELECT_INPUT DAISY Register"]
pub mod lpuart3_tx_select_input;
#[doc = "LPUART4_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart4_rx_select_input](lpuart4_rx_select_input) module"]
pub type LPUART4_RX_SELECT_INPUT = crate::Reg<u32, _LPUART4_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART4_RX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart4_rx_select_input::R](lpuart4_rx_select_input::R) reader structure"]
impl crate::Readable for LPUART4_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart4_rx_select_input::W](lpuart4_rx_select_input::W) writer structure"]
impl crate::Writable for LPUART4_RX_SELECT_INPUT {}
#[doc = "LPUART4_RX_SELECT_INPUT DAISY Register"]
pub mod lpuart4_rx_select_input;
#[doc = "LPUART4_TX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart4_tx_select_input](lpuart4_tx_select_input) module"]
pub type LPUART4_TX_SELECT_INPUT = crate::Reg<u32, _LPUART4_TX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART4_TX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart4_tx_select_input::R](lpuart4_tx_select_input::R) reader structure"]
impl crate::Readable for LPUART4_TX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart4_tx_select_input::W](lpuart4_tx_select_input::W) writer structure"]
impl crate::Writable for LPUART4_TX_SELECT_INPUT {}
#[doc = "LPUART4_TX_SELECT_INPUT DAISY Register"]
pub mod lpuart4_tx_select_input;
#[doc = "LPUART5_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart5_rx_select_input](lpuart5_rx_select_input) module"]
pub type LPUART5_RX_SELECT_INPUT = crate::Reg<u32, _LPUART5_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART5_RX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart5_rx_select_input::R](lpuart5_rx_select_input::R) reader structure"]
impl crate::Readable for LPUART5_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart5_rx_select_input::W](lpuart5_rx_select_input::W) writer structure"]
impl crate::Writable for LPUART5_RX_SELECT_INPUT {}
#[doc = "LPUART5_RX_SELECT_INPUT DAISY Register"]
pub mod lpuart5_rx_select_input;
#[doc = "LPUART5_TX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart5_tx_select_input](lpuart5_tx_select_input) module"]
pub type LPUART5_TX_SELECT_INPUT = crate::Reg<u32, _LPUART5_TX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART5_TX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart5_tx_select_input::R](lpuart5_tx_select_input::R) reader structure"]
impl crate::Readable for LPUART5_TX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart5_tx_select_input::W](lpuart5_tx_select_input::W) writer structure"]
impl crate::Writable for LPUART5_TX_SELECT_INPUT {}
#[doc = "LPUART5_TX_SELECT_INPUT DAISY Register"]
pub mod lpuart5_tx_select_input;
#[doc = "LPUART6_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart6_rx_select_input](lpuart6_rx_select_input) module"]
pub type LPUART6_RX_SELECT_INPUT = crate::Reg<u32, _LPUART6_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART6_RX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart6_rx_select_input::R](lpuart6_rx_select_input::R) reader structure"]
impl crate::Readable for LPUART6_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart6_rx_select_input::W](lpuart6_rx_select_input::W) writer structure"]
impl crate::Writable for LPUART6_RX_SELECT_INPUT {}
#[doc = "LPUART6_RX_SELECT_INPUT DAISY Register"]
pub mod lpuart6_rx_select_input;
#[doc = "LPUART6_TX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart6_tx_select_input](lpuart6_tx_select_input) module"]
pub type LPUART6_TX_SELECT_INPUT = crate::Reg<u32, _LPUART6_TX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART6_TX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart6_tx_select_input::R](lpuart6_tx_select_input::R) reader structure"]
impl crate::Readable for LPUART6_TX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart6_tx_select_input::W](lpuart6_tx_select_input::W) writer structure"]
impl crate::Writable for LPUART6_TX_SELECT_INPUT {}
#[doc = "LPUART6_TX_SELECT_INPUT DAISY Register"]
pub mod lpuart6_tx_select_input;
#[doc = "LPUART7_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart7_rx_select_input](lpuart7_rx_select_input) module"]
pub type LPUART7_RX_SELECT_INPUT = crate::Reg<u32, _LPUART7_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART7_RX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart7_rx_select_input::R](lpuart7_rx_select_input::R) reader structure"]
impl crate::Readable for LPUART7_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart7_rx_select_input::W](lpuart7_rx_select_input::W) writer structure"]
impl crate::Writable for LPUART7_RX_SELECT_INPUT {}
#[doc = "LPUART7_RX_SELECT_INPUT DAISY Register"]
pub mod lpuart7_rx_select_input;
#[doc = "LPUART7_TX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart7_tx_select_input](lpuart7_tx_select_input) module"]
pub type LPUART7_TX_SELECT_INPUT = crate::Reg<u32, _LPUART7_TX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART7_TX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart7_tx_select_input::R](lpuart7_tx_select_input::R) reader structure"]
impl crate::Readable for LPUART7_TX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart7_tx_select_input::W](lpuart7_tx_select_input::W) writer structure"]
impl crate::Writable for LPUART7_TX_SELECT_INPUT {}
#[doc = "LPUART7_TX_SELECT_INPUT DAISY Register"]
pub mod lpuart7_tx_select_input;
#[doc = "LPUART8_RX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart8_rx_select_input](lpuart8_rx_select_input) module"]
pub type LPUART8_RX_SELECT_INPUT = crate::Reg<u32, _LPUART8_RX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART8_RX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart8_rx_select_input::R](lpuart8_rx_select_input::R) reader structure"]
impl crate::Readable for LPUART8_RX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart8_rx_select_input::W](lpuart8_rx_select_input::W) writer structure"]
impl crate::Writable for LPUART8_RX_SELECT_INPUT {}
#[doc = "LPUART8_RX_SELECT_INPUT DAISY Register"]
pub mod lpuart8_rx_select_input;
#[doc = "LPUART8_TX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpuart8_tx_select_input](lpuart8_tx_select_input) module"]
pub type LPUART8_TX_SELECT_INPUT = crate::Reg<u32, _LPUART8_TX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPUART8_TX_SELECT_INPUT;
#[doc = "`read()` method returns [lpuart8_tx_select_input::R](lpuart8_tx_select_input::R) reader structure"]
impl crate::Readable for LPUART8_TX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [lpuart8_tx_select_input::W](lpuart8_tx_select_input::W) writer structure"]
impl crate::Writable for LPUART8_TX_SELECT_INPUT {}
#[doc = "LPUART8_TX_SELECT_INPUT DAISY Register"]
pub mod lpuart8_tx_select_input;
#[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nmi_select_input](nmi_select_input) module"]
pub type NMI_SELECT_INPUT = crate::Reg<u32, _NMI_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMI_SELECT_INPUT;
#[doc = "`read()` method returns [nmi_select_input::R](nmi_select_input::R) reader structure"]
impl crate::Readable for NMI_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [nmi_select_input::W](nmi_select_input::W) writer structure"]
impl crate::Writable for NMI_SELECT_INPUT {}
#[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
pub mod nmi_select_input;
#[doc = "QTIMER2_TIMER0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qtimer2_timer0_select_input](qtimer2_timer0_select_input) module"]
pub type QTIMER2_TIMER0_SELECT_INPUT = crate::Reg<u32, _QTIMER2_TIMER0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QTIMER2_TIMER0_SELECT_INPUT;
#[doc = "`read()` method returns [qtimer2_timer0_select_input::R](qtimer2_timer0_select_input::R) reader structure"]
impl crate::Readable for QTIMER2_TIMER0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [qtimer2_timer0_select_input::W](qtimer2_timer0_select_input::W) writer structure"]
impl crate::Writable for QTIMER2_TIMER0_SELECT_INPUT {}
#[doc = "QTIMER2_TIMER0_SELECT_INPUT DAISY Register"]
pub mod qtimer2_timer0_select_input;
#[doc = "QTIMER2_TIMER1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qtimer2_timer1_select_input](qtimer2_timer1_select_input) module"]
pub type QTIMER2_TIMER1_SELECT_INPUT = crate::Reg<u32, _QTIMER2_TIMER1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QTIMER2_TIMER1_SELECT_INPUT;
#[doc = "`read()` method returns [qtimer2_timer1_select_input::R](qtimer2_timer1_select_input::R) reader structure"]
impl crate::Readable for QTIMER2_TIMER1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [qtimer2_timer1_select_input::W](qtimer2_timer1_select_input::W) writer structure"]
impl crate::Writable for QTIMER2_TIMER1_SELECT_INPUT {}
#[doc = "QTIMER2_TIMER1_SELECT_INPUT DAISY Register"]
pub mod qtimer2_timer1_select_input;
#[doc = "QTIMER2_TIMER2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qtimer2_timer2_select_input](qtimer2_timer2_select_input) module"]
pub type QTIMER2_TIMER2_SELECT_INPUT = crate::Reg<u32, _QTIMER2_TIMER2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QTIMER2_TIMER2_SELECT_INPUT;
#[doc = "`read()` method returns [qtimer2_timer2_select_input::R](qtimer2_timer2_select_input::R) reader structure"]
impl crate::Readable for QTIMER2_TIMER2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [qtimer2_timer2_select_input::W](qtimer2_timer2_select_input::W) writer structure"]
impl crate::Writable for QTIMER2_TIMER2_SELECT_INPUT {}
#[doc = "QTIMER2_TIMER2_SELECT_INPUT DAISY Register"]
pub mod qtimer2_timer2_select_input;
#[doc = "QTIMER2_TIMER3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qtimer2_timer3_select_input](qtimer2_timer3_select_input) module"]
pub type QTIMER2_TIMER3_SELECT_INPUT = crate::Reg<u32, _QTIMER2_TIMER3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QTIMER2_TIMER3_SELECT_INPUT;
#[doc = "`read()` method returns [qtimer2_timer3_select_input::R](qtimer2_timer3_select_input::R) reader structure"]
impl crate::Readable for QTIMER2_TIMER3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [qtimer2_timer3_select_input::W](qtimer2_timer3_select_input::W) writer structure"]
impl crate::Writable for QTIMER2_TIMER3_SELECT_INPUT {}
#[doc = "QTIMER2_TIMER3_SELECT_INPUT DAISY Register"]
pub mod qtimer2_timer3_select_input;
#[doc = "QTIMER3_TIMER0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qtimer3_timer0_select_input](qtimer3_timer0_select_input) module"]
pub type QTIMER3_TIMER0_SELECT_INPUT = crate::Reg<u32, _QTIMER3_TIMER0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QTIMER3_TIMER0_SELECT_INPUT;
#[doc = "`read()` method returns [qtimer3_timer0_select_input::R](qtimer3_timer0_select_input::R) reader structure"]
impl crate::Readable for QTIMER3_TIMER0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [qtimer3_timer0_select_input::W](qtimer3_timer0_select_input::W) writer structure"]
impl crate::Writable for QTIMER3_TIMER0_SELECT_INPUT {}
#[doc = "QTIMER3_TIMER0_SELECT_INPUT DAISY Register"]
pub mod qtimer3_timer0_select_input;
#[doc = "QTIMER3_TIMER1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qtimer3_timer1_select_input](qtimer3_timer1_select_input) module"]
pub type QTIMER3_TIMER1_SELECT_INPUT = crate::Reg<u32, _QTIMER3_TIMER1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QTIMER3_TIMER1_SELECT_INPUT;
#[doc = "`read()` method returns [qtimer3_timer1_select_input::R](qtimer3_timer1_select_input::R) reader structure"]
impl crate::Readable for QTIMER3_TIMER1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [qtimer3_timer1_select_input::W](qtimer3_timer1_select_input::W) writer structure"]
impl crate::Writable for QTIMER3_TIMER1_SELECT_INPUT {}
#[doc = "QTIMER3_TIMER1_SELECT_INPUT DAISY Register"]
pub mod qtimer3_timer1_select_input;
#[doc = "QTIMER3_TIMER2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qtimer3_timer2_select_input](qtimer3_timer2_select_input) module"]
pub type QTIMER3_TIMER2_SELECT_INPUT = crate::Reg<u32, _QTIMER3_TIMER2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QTIMER3_TIMER2_SELECT_INPUT;
#[doc = "`read()` method returns [qtimer3_timer2_select_input::R](qtimer3_timer2_select_input::R) reader structure"]
impl crate::Readable for QTIMER3_TIMER2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [qtimer3_timer2_select_input::W](qtimer3_timer2_select_input::W) writer structure"]
impl crate::Writable for QTIMER3_TIMER2_SELECT_INPUT {}
#[doc = "QTIMER3_TIMER2_SELECT_INPUT DAISY Register"]
pub mod qtimer3_timer2_select_input;
#[doc = "QTIMER3_TIMER3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qtimer3_timer3_select_input](qtimer3_timer3_select_input) module"]
pub type QTIMER3_TIMER3_SELECT_INPUT = crate::Reg<u32, _QTIMER3_TIMER3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QTIMER3_TIMER3_SELECT_INPUT;
#[doc = "`read()` method returns [qtimer3_timer3_select_input::R](qtimer3_timer3_select_input::R) reader structure"]
impl crate::Readable for QTIMER3_TIMER3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [qtimer3_timer3_select_input::W](qtimer3_timer3_select_input::W) writer structure"]
impl crate::Writable for QTIMER3_TIMER3_SELECT_INPUT {}
#[doc = "QTIMER3_TIMER3_SELECT_INPUT DAISY Register"]
pub mod qtimer3_timer3_select_input;
#[doc = "SAI1_MCLK2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_mclk2_select_input](sai1_mclk2_select_input) module"]
pub type SAI1_MCLK2_SELECT_INPUT = crate::Reg<u32, _SAI1_MCLK2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_MCLK2_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_mclk2_select_input::R](sai1_mclk2_select_input::R) reader structure"]
impl crate::Readable for SAI1_MCLK2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_mclk2_select_input::W](sai1_mclk2_select_input::W) writer structure"]
impl crate::Writable for SAI1_MCLK2_SELECT_INPUT {}
#[doc = "SAI1_MCLK2_SELECT_INPUT DAISY Register"]
pub mod sai1_mclk2_select_input;
#[doc = "SAI1_RX_BCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_rx_bclk_select_input](sai1_rx_bclk_select_input) module"]
pub type SAI1_RX_BCLK_SELECT_INPUT = crate::Reg<u32, _SAI1_RX_BCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_RX_BCLK_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_rx_bclk_select_input::R](sai1_rx_bclk_select_input::R) reader structure"]
impl crate::Readable for SAI1_RX_BCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_rx_bclk_select_input::W](sai1_rx_bclk_select_input::W) writer structure"]
impl crate::Writable for SAI1_RX_BCLK_SELECT_INPUT {}
#[doc = "SAI1_RX_BCLK_SELECT_INPUT DAISY Register"]
pub mod sai1_rx_bclk_select_input;
#[doc = "SAI1_RX_DATA0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_rx_data0_select_input](sai1_rx_data0_select_input) module"]
pub type SAI1_RX_DATA0_SELECT_INPUT = crate::Reg<u32, _SAI1_RX_DATA0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_RX_DATA0_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_rx_data0_select_input::R](sai1_rx_data0_select_input::R) reader structure"]
impl crate::Readable for SAI1_RX_DATA0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_rx_data0_select_input::W](sai1_rx_data0_select_input::W) writer structure"]
impl crate::Writable for SAI1_RX_DATA0_SELECT_INPUT {}
#[doc = "SAI1_RX_DATA0_SELECT_INPUT DAISY Register"]
pub mod sai1_rx_data0_select_input;
#[doc = "SAI1_RX_DATA1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_rx_data1_select_input](sai1_rx_data1_select_input) module"]
pub type SAI1_RX_DATA1_SELECT_INPUT = crate::Reg<u32, _SAI1_RX_DATA1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_RX_DATA1_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_rx_data1_select_input::R](sai1_rx_data1_select_input::R) reader structure"]
impl crate::Readable for SAI1_RX_DATA1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_rx_data1_select_input::W](sai1_rx_data1_select_input::W) writer structure"]
impl crate::Writable for SAI1_RX_DATA1_SELECT_INPUT {}
#[doc = "SAI1_RX_DATA1_SELECT_INPUT DAISY Register"]
pub mod sai1_rx_data1_select_input;
#[doc = "SAI1_RX_DATA2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_rx_data2_select_input](sai1_rx_data2_select_input) module"]
pub type SAI1_RX_DATA2_SELECT_INPUT = crate::Reg<u32, _SAI1_RX_DATA2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_RX_DATA2_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_rx_data2_select_input::R](sai1_rx_data2_select_input::R) reader structure"]
impl crate::Readable for SAI1_RX_DATA2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_rx_data2_select_input::W](sai1_rx_data2_select_input::W) writer structure"]
impl crate::Writable for SAI1_RX_DATA2_SELECT_INPUT {}
#[doc = "SAI1_RX_DATA2_SELECT_INPUT DAISY Register"]
pub mod sai1_rx_data2_select_input;
#[doc = "SAI1_RX_DATA3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_rx_data3_select_input](sai1_rx_data3_select_input) module"]
pub type SAI1_RX_DATA3_SELECT_INPUT = crate::Reg<u32, _SAI1_RX_DATA3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_RX_DATA3_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_rx_data3_select_input::R](sai1_rx_data3_select_input::R) reader structure"]
impl crate::Readable for SAI1_RX_DATA3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_rx_data3_select_input::W](sai1_rx_data3_select_input::W) writer structure"]
impl crate::Writable for SAI1_RX_DATA3_SELECT_INPUT {}
#[doc = "SAI1_RX_DATA3_SELECT_INPUT DAISY Register"]
pub mod sai1_rx_data3_select_input;
#[doc = "SAI1_RX_SYNC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_rx_sync_select_input](sai1_rx_sync_select_input) module"]
pub type SAI1_RX_SYNC_SELECT_INPUT = crate::Reg<u32, _SAI1_RX_SYNC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_RX_SYNC_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_rx_sync_select_input::R](sai1_rx_sync_select_input::R) reader structure"]
impl crate::Readable for SAI1_RX_SYNC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_rx_sync_select_input::W](sai1_rx_sync_select_input::W) writer structure"]
impl crate::Writable for SAI1_RX_SYNC_SELECT_INPUT {}
#[doc = "SAI1_RX_SYNC_SELECT_INPUT DAISY Register"]
pub mod sai1_rx_sync_select_input;
#[doc = "SAI1_TX_BCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_tx_bclk_select_input](sai1_tx_bclk_select_input) module"]
pub type SAI1_TX_BCLK_SELECT_INPUT = crate::Reg<u32, _SAI1_TX_BCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_TX_BCLK_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_tx_bclk_select_input::R](sai1_tx_bclk_select_input::R) reader structure"]
impl crate::Readable for SAI1_TX_BCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_tx_bclk_select_input::W](sai1_tx_bclk_select_input::W) writer structure"]
impl crate::Writable for SAI1_TX_BCLK_SELECT_INPUT {}
#[doc = "SAI1_TX_BCLK_SELECT_INPUT DAISY Register"]
pub mod sai1_tx_bclk_select_input;
#[doc = "SAI1_TX_SYNC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai1_tx_sync_select_input](sai1_tx_sync_select_input) module"]
pub type SAI1_TX_SYNC_SELECT_INPUT = crate::Reg<u32, _SAI1_TX_SYNC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI1_TX_SYNC_SELECT_INPUT;
#[doc = "`read()` method returns [sai1_tx_sync_select_input::R](sai1_tx_sync_select_input::R) reader structure"]
impl crate::Readable for SAI1_TX_SYNC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai1_tx_sync_select_input::W](sai1_tx_sync_select_input::W) writer structure"]
impl crate::Writable for SAI1_TX_SYNC_SELECT_INPUT {}
#[doc = "SAI1_TX_SYNC_SELECT_INPUT DAISY Register"]
pub mod sai1_tx_sync_select_input;
#[doc = "SAI2_MCLK2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai2_mclk2_select_input](sai2_mclk2_select_input) module"]
pub type SAI2_MCLK2_SELECT_INPUT = crate::Reg<u32, _SAI2_MCLK2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI2_MCLK2_SELECT_INPUT;
#[doc = "`read()` method returns [sai2_mclk2_select_input::R](sai2_mclk2_select_input::R) reader structure"]
impl crate::Readable for SAI2_MCLK2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai2_mclk2_select_input::W](sai2_mclk2_select_input::W) writer structure"]
impl crate::Writable for SAI2_MCLK2_SELECT_INPUT {}
#[doc = "SAI2_MCLK2_SELECT_INPUT DAISY Register"]
pub mod sai2_mclk2_select_input;
#[doc = "SAI2_RX_BCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai2_rx_bclk_select_input](sai2_rx_bclk_select_input) module"]
pub type SAI2_RX_BCLK_SELECT_INPUT = crate::Reg<u32, _SAI2_RX_BCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI2_RX_BCLK_SELECT_INPUT;
#[doc = "`read()` method returns [sai2_rx_bclk_select_input::R](sai2_rx_bclk_select_input::R) reader structure"]
impl crate::Readable for SAI2_RX_BCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai2_rx_bclk_select_input::W](sai2_rx_bclk_select_input::W) writer structure"]
impl crate::Writable for SAI2_RX_BCLK_SELECT_INPUT {}
#[doc = "SAI2_RX_BCLK_SELECT_INPUT DAISY Register"]
pub mod sai2_rx_bclk_select_input;
#[doc = "SAI2_RX_DATA0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai2_rx_data0_select_input](sai2_rx_data0_select_input) module"]
pub type SAI2_RX_DATA0_SELECT_INPUT = crate::Reg<u32, _SAI2_RX_DATA0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI2_RX_DATA0_SELECT_INPUT;
#[doc = "`read()` method returns [sai2_rx_data0_select_input::R](sai2_rx_data0_select_input::R) reader structure"]
impl crate::Readable for SAI2_RX_DATA0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai2_rx_data0_select_input::W](sai2_rx_data0_select_input::W) writer structure"]
impl crate::Writable for SAI2_RX_DATA0_SELECT_INPUT {}
#[doc = "SAI2_RX_DATA0_SELECT_INPUT DAISY Register"]
pub mod sai2_rx_data0_select_input;
#[doc = "SAI2_RX_SYNC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai2_rx_sync_select_input](sai2_rx_sync_select_input) module"]
pub type SAI2_RX_SYNC_SELECT_INPUT = crate::Reg<u32, _SAI2_RX_SYNC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI2_RX_SYNC_SELECT_INPUT;
#[doc = "`read()` method returns [sai2_rx_sync_select_input::R](sai2_rx_sync_select_input::R) reader structure"]
impl crate::Readable for SAI2_RX_SYNC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai2_rx_sync_select_input::W](sai2_rx_sync_select_input::W) writer structure"]
impl crate::Writable for SAI2_RX_SYNC_SELECT_INPUT {}
#[doc = "SAI2_RX_SYNC_SELECT_INPUT DAISY Register"]
pub mod sai2_rx_sync_select_input;
#[doc = "SAI2_TX_BCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai2_tx_bclk_select_input](sai2_tx_bclk_select_input) module"]
pub type SAI2_TX_BCLK_SELECT_INPUT = crate::Reg<u32, _SAI2_TX_BCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI2_TX_BCLK_SELECT_INPUT;
#[doc = "`read()` method returns [sai2_tx_bclk_select_input::R](sai2_tx_bclk_select_input::R) reader structure"]
impl crate::Readable for SAI2_TX_BCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai2_tx_bclk_select_input::W](sai2_tx_bclk_select_input::W) writer structure"]
impl crate::Writable for SAI2_TX_BCLK_SELECT_INPUT {}
#[doc = "SAI2_TX_BCLK_SELECT_INPUT DAISY Register"]
pub mod sai2_tx_bclk_select_input;
#[doc = "SAI2_TX_SYNC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai2_tx_sync_select_input](sai2_tx_sync_select_input) module"]
pub type SAI2_TX_SYNC_SELECT_INPUT = crate::Reg<u32, _SAI2_TX_SYNC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI2_TX_SYNC_SELECT_INPUT;
#[doc = "`read()` method returns [sai2_tx_sync_select_input::R](sai2_tx_sync_select_input::R) reader structure"]
impl crate::Readable for SAI2_TX_SYNC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai2_tx_sync_select_input::W](sai2_tx_sync_select_input::W) writer structure"]
impl crate::Writable for SAI2_TX_SYNC_SELECT_INPUT {}
#[doc = "SAI2_TX_SYNC_SELECT_INPUT DAISY Register"]
pub mod sai2_tx_sync_select_input;
#[doc = "SPDIF_IN_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spdif_in_select_input](spdif_in_select_input) module"]
pub type SPDIF_IN_SELECT_INPUT = crate::Reg<u32, _SPDIF_IN_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIF_IN_SELECT_INPUT;
#[doc = "`read()` method returns [spdif_in_select_input::R](spdif_in_select_input::R) reader structure"]
impl crate::Readable for SPDIF_IN_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [spdif_in_select_input::W](spdif_in_select_input::W) writer structure"]
impl crate::Writable for SPDIF_IN_SELECT_INPUT {}
#[doc = "SPDIF_IN_SELECT_INPUT DAISY Register"]
pub mod spdif_in_select_input;
#[doc = "USB_OTG2_OC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_otg2_oc_select_input](usb_otg2_oc_select_input) module"]
pub type USB_OTG2_OC_SELECT_INPUT = crate::Reg<u32, _USB_OTG2_OC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_OTG2_OC_SELECT_INPUT;
#[doc = "`read()` method returns [usb_otg2_oc_select_input::R](usb_otg2_oc_select_input::R) reader structure"]
impl crate::Readable for USB_OTG2_OC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usb_otg2_oc_select_input::W](usb_otg2_oc_select_input::W) writer structure"]
impl crate::Writable for USB_OTG2_OC_SELECT_INPUT {}
#[doc = "USB_OTG2_OC_SELECT_INPUT DAISY Register"]
pub mod usb_otg2_oc_select_input;
#[doc = "USB_OTG1_OC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_otg1_oc_select_input](usb_otg1_oc_select_input) module"]
pub type USB_OTG1_OC_SELECT_INPUT = crate::Reg<u32, _USB_OTG1_OC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_OTG1_OC_SELECT_INPUT;
#[doc = "`read()` method returns [usb_otg1_oc_select_input::R](usb_otg1_oc_select_input::R) reader structure"]
impl crate::Readable for USB_OTG1_OC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usb_otg1_oc_select_input::W](usb_otg1_oc_select_input::W) writer structure"]
impl crate::Writable for USB_OTG1_OC_SELECT_INPUT {}
#[doc = "USB_OTG1_OC_SELECT_INPUT DAISY Register"]
pub mod usb_otg1_oc_select_input;
#[doc = "USDHC1_CD_B_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc1_cd_b_select_input](usdhc1_cd_b_select_input) module"]
pub type USDHC1_CD_B_SELECT_INPUT = crate::Reg<u32, _USDHC1_CD_B_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC1_CD_B_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc1_cd_b_select_input::R](usdhc1_cd_b_select_input::R) reader structure"]
impl crate::Readable for USDHC1_CD_B_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc1_cd_b_select_input::W](usdhc1_cd_b_select_input::W) writer structure"]
impl crate::Writable for USDHC1_CD_B_SELECT_INPUT {}
#[doc = "USDHC1_CD_B_SELECT_INPUT DAISY Register"]
pub mod usdhc1_cd_b_select_input;
#[doc = "USDHC1_WP_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc1_wp_select_input](usdhc1_wp_select_input) module"]
pub type USDHC1_WP_SELECT_INPUT = crate::Reg<u32, _USDHC1_WP_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC1_WP_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc1_wp_select_input::R](usdhc1_wp_select_input::R) reader structure"]
impl crate::Readable for USDHC1_WP_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc1_wp_select_input::W](usdhc1_wp_select_input::W) writer structure"]
impl crate::Writable for USDHC1_WP_SELECT_INPUT {}
#[doc = "USDHC1_WP_SELECT_INPUT DAISY Register"]
pub mod usdhc1_wp_select_input;
#[doc = "USDHC2_CLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_clk_select_input](usdhc2_clk_select_input) module"]
pub type USDHC2_CLK_SELECT_INPUT = crate::Reg<u32, _USDHC2_CLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_CLK_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_clk_select_input::R](usdhc2_clk_select_input::R) reader structure"]
impl crate::Readable for USDHC2_CLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_clk_select_input::W](usdhc2_clk_select_input::W) writer structure"]
impl crate::Writable for USDHC2_CLK_SELECT_INPUT {}
#[doc = "USDHC2_CLK_SELECT_INPUT DAISY Register"]
pub mod usdhc2_clk_select_input;
#[doc = "USDHC2_CD_B_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_cd_b_select_input](usdhc2_cd_b_select_input) module"]
pub type USDHC2_CD_B_SELECT_INPUT = crate::Reg<u32, _USDHC2_CD_B_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_CD_B_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_cd_b_select_input::R](usdhc2_cd_b_select_input::R) reader structure"]
impl crate::Readable for USDHC2_CD_B_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_cd_b_select_input::W](usdhc2_cd_b_select_input::W) writer structure"]
impl crate::Writable for USDHC2_CD_B_SELECT_INPUT {}
#[doc = "USDHC2_CD_B_SELECT_INPUT DAISY Register"]
pub mod usdhc2_cd_b_select_input;
#[doc = "USDHC2_CMD_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_cmd_select_input](usdhc2_cmd_select_input) module"]
pub type USDHC2_CMD_SELECT_INPUT = crate::Reg<u32, _USDHC2_CMD_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_CMD_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_cmd_select_input::R](usdhc2_cmd_select_input::R) reader structure"]
impl crate::Readable for USDHC2_CMD_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_cmd_select_input::W](usdhc2_cmd_select_input::W) writer structure"]
impl crate::Writable for USDHC2_CMD_SELECT_INPUT {}
#[doc = "USDHC2_CMD_SELECT_INPUT DAISY Register"]
pub mod usdhc2_cmd_select_input;
#[doc = "USDHC2_DATA0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_data0_select_input](usdhc2_data0_select_input) module"]
pub type USDHC2_DATA0_SELECT_INPUT = crate::Reg<u32, _USDHC2_DATA0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_DATA0_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_data0_select_input::R](usdhc2_data0_select_input::R) reader structure"]
impl crate::Readable for USDHC2_DATA0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_data0_select_input::W](usdhc2_data0_select_input::W) writer structure"]
impl crate::Writable for USDHC2_DATA0_SELECT_INPUT {}
#[doc = "USDHC2_DATA0_SELECT_INPUT DAISY Register"]
pub mod usdhc2_data0_select_input;
#[doc = "USDHC2_DATA1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_data1_select_input](usdhc2_data1_select_input) module"]
pub type USDHC2_DATA1_SELECT_INPUT = crate::Reg<u32, _USDHC2_DATA1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_DATA1_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_data1_select_input::R](usdhc2_data1_select_input::R) reader structure"]
impl crate::Readable for USDHC2_DATA1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_data1_select_input::W](usdhc2_data1_select_input::W) writer structure"]
impl crate::Writable for USDHC2_DATA1_SELECT_INPUT {}
#[doc = "USDHC2_DATA1_SELECT_INPUT DAISY Register"]
pub mod usdhc2_data1_select_input;
#[doc = "USDHC2_DATA2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_data2_select_input](usdhc2_data2_select_input) module"]
pub type USDHC2_DATA2_SELECT_INPUT = crate::Reg<u32, _USDHC2_DATA2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_DATA2_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_data2_select_input::R](usdhc2_data2_select_input::R) reader structure"]
impl crate::Readable for USDHC2_DATA2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_data2_select_input::W](usdhc2_data2_select_input::W) writer structure"]
impl crate::Writable for USDHC2_DATA2_SELECT_INPUT {}
#[doc = "USDHC2_DATA2_SELECT_INPUT DAISY Register"]
pub mod usdhc2_data2_select_input;
#[doc = "USDHC2_DATA3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_data3_select_input](usdhc2_data3_select_input) module"]
pub type USDHC2_DATA3_SELECT_INPUT = crate::Reg<u32, _USDHC2_DATA3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_DATA3_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_data3_select_input::R](usdhc2_data3_select_input::R) reader structure"]
impl crate::Readable for USDHC2_DATA3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_data3_select_input::W](usdhc2_data3_select_input::W) writer structure"]
impl crate::Writable for USDHC2_DATA3_SELECT_INPUT {}
#[doc = "USDHC2_DATA3_SELECT_INPUT DAISY Register"]
pub mod usdhc2_data3_select_input;
#[doc = "USDHC2_DATA4_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_data4_select_input](usdhc2_data4_select_input) module"]
pub type USDHC2_DATA4_SELECT_INPUT = crate::Reg<u32, _USDHC2_DATA4_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_DATA4_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_data4_select_input::R](usdhc2_data4_select_input::R) reader structure"]
impl crate::Readable for USDHC2_DATA4_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_data4_select_input::W](usdhc2_data4_select_input::W) writer structure"]
impl crate::Writable for USDHC2_DATA4_SELECT_INPUT {}
#[doc = "USDHC2_DATA4_SELECT_INPUT DAISY Register"]
pub mod usdhc2_data4_select_input;
#[doc = "USDHC2_DATA5_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_data5_select_input](usdhc2_data5_select_input) module"]
pub type USDHC2_DATA5_SELECT_INPUT = crate::Reg<u32, _USDHC2_DATA5_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_DATA5_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_data5_select_input::R](usdhc2_data5_select_input::R) reader structure"]
impl crate::Readable for USDHC2_DATA5_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_data5_select_input::W](usdhc2_data5_select_input::W) writer structure"]
impl crate::Writable for USDHC2_DATA5_SELECT_INPUT {}
#[doc = "USDHC2_DATA5_SELECT_INPUT DAISY Register"]
pub mod usdhc2_data5_select_input;
#[doc = "USDHC2_DATA6_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_data6_select_input](usdhc2_data6_select_input) module"]
pub type USDHC2_DATA6_SELECT_INPUT = crate::Reg<u32, _USDHC2_DATA6_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_DATA6_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_data6_select_input::R](usdhc2_data6_select_input::R) reader structure"]
impl crate::Readable for USDHC2_DATA6_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_data6_select_input::W](usdhc2_data6_select_input::W) writer structure"]
impl crate::Writable for USDHC2_DATA6_SELECT_INPUT {}
#[doc = "USDHC2_DATA6_SELECT_INPUT DAISY Register"]
pub mod usdhc2_data6_select_input;
#[doc = "USDHC2_DATA7_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_data7_select_input](usdhc2_data7_select_input) module"]
pub type USDHC2_DATA7_SELECT_INPUT = crate::Reg<u32, _USDHC2_DATA7_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_DATA7_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_data7_select_input::R](usdhc2_data7_select_input::R) reader structure"]
impl crate::Readable for USDHC2_DATA7_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_data7_select_input::W](usdhc2_data7_select_input::W) writer structure"]
impl crate::Writable for USDHC2_DATA7_SELECT_INPUT {}
#[doc = "USDHC2_DATA7_SELECT_INPUT DAISY Register"]
pub mod usdhc2_data7_select_input;
#[doc = "USDHC2_WP_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usdhc2_wp_select_input](usdhc2_wp_select_input) module"]
pub type USDHC2_WP_SELECT_INPUT = crate::Reg<u32, _USDHC2_WP_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USDHC2_WP_SELECT_INPUT;
#[doc = "`read()` method returns [usdhc2_wp_select_input::R](usdhc2_wp_select_input::R) reader structure"]
impl crate::Readable for USDHC2_WP_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [usdhc2_wp_select_input::W](usdhc2_wp_select_input::W) writer structure"]
impl crate::Writable for USDHC2_WP_SELECT_INPUT {}
#[doc = "USDHC2_WP_SELECT_INPUT DAISY Register"]
pub mod usdhc2_wp_select_input;
#[doc = "XBAR1_IN02_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in02_select_input](xbar1_in02_select_input) module"]
pub type XBAR1_IN02_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN02_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN02_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in02_select_input::R](xbar1_in02_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN02_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in02_select_input::W](xbar1_in02_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN02_SELECT_INPUT {}
#[doc = "XBAR1_IN02_SELECT_INPUT DAISY Register"]
pub mod xbar1_in02_select_input;
#[doc = "XBAR1_IN03_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in03_select_input](xbar1_in03_select_input) module"]
pub type XBAR1_IN03_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN03_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN03_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in03_select_input::R](xbar1_in03_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN03_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in03_select_input::W](xbar1_in03_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN03_SELECT_INPUT {}
#[doc = "XBAR1_IN03_SELECT_INPUT DAISY Register"]
pub mod xbar1_in03_select_input;
#[doc = "XBAR1_IN04_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in04_select_input](xbar1_in04_select_input) module"]
pub type XBAR1_IN04_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN04_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN04_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in04_select_input::R](xbar1_in04_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN04_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in04_select_input::W](xbar1_in04_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN04_SELECT_INPUT {}
#[doc = "XBAR1_IN04_SELECT_INPUT DAISY Register"]
pub mod xbar1_in04_select_input;
#[doc = "XBAR1_IN05_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in05_select_input](xbar1_in05_select_input) module"]
pub type XBAR1_IN05_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN05_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN05_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in05_select_input::R](xbar1_in05_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN05_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in05_select_input::W](xbar1_in05_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN05_SELECT_INPUT {}
#[doc = "XBAR1_IN05_SELECT_INPUT DAISY Register"]
pub mod xbar1_in05_select_input;
#[doc = "XBAR1_IN06_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in06_select_input](xbar1_in06_select_input) module"]
pub type XBAR1_IN06_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN06_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN06_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in06_select_input::R](xbar1_in06_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN06_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in06_select_input::W](xbar1_in06_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN06_SELECT_INPUT {}
#[doc = "XBAR1_IN06_SELECT_INPUT DAISY Register"]
pub mod xbar1_in06_select_input;
#[doc = "XBAR1_IN07_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in07_select_input](xbar1_in07_select_input) module"]
pub type XBAR1_IN07_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN07_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN07_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in07_select_input::R](xbar1_in07_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN07_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in07_select_input::W](xbar1_in07_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN07_SELECT_INPUT {}
#[doc = "XBAR1_IN07_SELECT_INPUT DAISY Register"]
pub mod xbar1_in07_select_input;
#[doc = "XBAR1_IN08_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in08_select_input](xbar1_in08_select_input) module"]
pub type XBAR1_IN08_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN08_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN08_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in08_select_input::R](xbar1_in08_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN08_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in08_select_input::W](xbar1_in08_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN08_SELECT_INPUT {}
#[doc = "XBAR1_IN08_SELECT_INPUT DAISY Register"]
pub mod xbar1_in08_select_input;
#[doc = "XBAR1_IN09_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in09_select_input](xbar1_in09_select_input) module"]
pub type XBAR1_IN09_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN09_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN09_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in09_select_input::R](xbar1_in09_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN09_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in09_select_input::W](xbar1_in09_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN09_SELECT_INPUT {}
#[doc = "XBAR1_IN09_SELECT_INPUT DAISY Register"]
pub mod xbar1_in09_select_input;
#[doc = "XBAR1_IN17_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in17_select_input](xbar1_in17_select_input) module"]
pub type XBAR1_IN17_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN17_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN17_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in17_select_input::R](xbar1_in17_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN17_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in17_select_input::W](xbar1_in17_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN17_SELECT_INPUT {}
#[doc = "XBAR1_IN17_SELECT_INPUT DAISY Register"]
pub mod xbar1_in17_select_input;
#[doc = "XBAR1_IN18_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in18_select_input](xbar1_in18_select_input) module"]
pub type XBAR1_IN18_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN18_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN18_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in18_select_input::R](xbar1_in18_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN18_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in18_select_input::W](xbar1_in18_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN18_SELECT_INPUT {}
#[doc = "XBAR1_IN18_SELECT_INPUT DAISY Register"]
pub mod xbar1_in18_select_input;
#[doc = "XBAR1_IN20_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in20_select_input](xbar1_in20_select_input) module"]
pub type XBAR1_IN20_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN20_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN20_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in20_select_input::R](xbar1_in20_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN20_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in20_select_input::W](xbar1_in20_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN20_SELECT_INPUT {}
#[doc = "XBAR1_IN20_SELECT_INPUT DAISY Register"]
pub mod xbar1_in20_select_input;
#[doc = "XBAR1_IN22_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in22_select_input](xbar1_in22_select_input) module"]
pub type XBAR1_IN22_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN22_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN22_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in22_select_input::R](xbar1_in22_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN22_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in22_select_input::W](xbar1_in22_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN22_SELECT_INPUT {}
#[doc = "XBAR1_IN22_SELECT_INPUT DAISY Register"]
pub mod xbar1_in22_select_input;
#[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in23_select_input](xbar1_in23_select_input) module"]
pub type XBAR1_IN23_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN23_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN23_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in23_select_input::R](xbar1_in23_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN23_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in23_select_input::W](xbar1_in23_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN23_SELECT_INPUT {}
#[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register"]
pub mod xbar1_in23_select_input;
#[doc = "XBAR1_IN24_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in24_select_input](xbar1_in24_select_input) module"]
pub type XBAR1_IN24_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN24_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN24_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in24_select_input::R](xbar1_in24_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN24_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in24_select_input::W](xbar1_in24_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN24_SELECT_INPUT {}
#[doc = "XBAR1_IN24_SELECT_INPUT DAISY Register"]
pub mod xbar1_in24_select_input;
#[doc = "XBAR1_IN14_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in14_select_input](xbar1_in14_select_input) module"]
pub type XBAR1_IN14_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN14_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN14_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in14_select_input::R](xbar1_in14_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN14_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in14_select_input::W](xbar1_in14_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN14_SELECT_INPUT {}
#[doc = "XBAR1_IN14_SELECT_INPUT DAISY Register"]
pub mod xbar1_in14_select_input;
#[doc = "XBAR1_IN15_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in15_select_input](xbar1_in15_select_input) module"]
pub type XBAR1_IN15_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN15_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN15_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in15_select_input::R](xbar1_in15_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN15_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in15_select_input::W](xbar1_in15_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN15_SELECT_INPUT {}
#[doc = "XBAR1_IN15_SELECT_INPUT DAISY Register"]
pub mod xbar1_in15_select_input;
#[doc = "XBAR1_IN16_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in16_select_input](xbar1_in16_select_input) module"]
pub type XBAR1_IN16_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN16_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN16_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in16_select_input::R](xbar1_in16_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN16_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in16_select_input::W](xbar1_in16_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN16_SELECT_INPUT {}
#[doc = "XBAR1_IN16_SELECT_INPUT DAISY Register"]
pub mod xbar1_in16_select_input;
#[doc = "XBAR1_IN25_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in25_select_input](xbar1_in25_select_input) module"]
pub type XBAR1_IN25_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN25_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN25_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in25_select_input::R](xbar1_in25_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN25_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in25_select_input::W](xbar1_in25_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN25_SELECT_INPUT {}
#[doc = "XBAR1_IN25_SELECT_INPUT DAISY Register"]
pub mod xbar1_in25_select_input;
#[doc = "XBAR1_IN19_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in19_select_input](xbar1_in19_select_input) module"]
pub type XBAR1_IN19_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN19_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN19_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in19_select_input::R](xbar1_in19_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN19_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in19_select_input::W](xbar1_in19_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN19_SELECT_INPUT {}
#[doc = "XBAR1_IN19_SELECT_INPUT DAISY Register"]
pub mod xbar1_in19_select_input;
#[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xbar1_in21_select_input](xbar1_in21_select_input) module"]
pub type XBAR1_IN21_SELECT_INPUT = crate::Reg<u32, _XBAR1_IN21_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XBAR1_IN21_SELECT_INPUT;
#[doc = "`read()` method returns [xbar1_in21_select_input::R](xbar1_in21_select_input::R) reader structure"]
impl crate::Readable for XBAR1_IN21_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [xbar1_in21_select_input::W](xbar1_in21_select_input::W) writer structure"]
impl crate::Writable for XBAR1_IN21_SELECT_INPUT {}
#[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register"]
pub mod xbar1_in21_select_input;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_00](sw_mux_ctl_pad_gpio_spi_b0_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_00::R](sw_mux_ctl_pad_gpio_spi_b0_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_00::W](sw_mux_ctl_pad_gpio_spi_b0_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_01](sw_mux_ctl_pad_gpio_spi_b0_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_01::R](sw_mux_ctl_pad_gpio_spi_b0_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_01::W](sw_mux_ctl_pad_gpio_spi_b0_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_02](sw_mux_ctl_pad_gpio_spi_b0_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_02::R](sw_mux_ctl_pad_gpio_spi_b0_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_02::W](sw_mux_ctl_pad_gpio_spi_b0_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_03](sw_mux_ctl_pad_gpio_spi_b0_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_03::R](sw_mux_ctl_pad_gpio_spi_b0_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_03::W](sw_mux_ctl_pad_gpio_spi_b0_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_04](sw_mux_ctl_pad_gpio_spi_b0_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_04::R](sw_mux_ctl_pad_gpio_spi_b0_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_04::W](sw_mux_ctl_pad_gpio_spi_b0_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_05](sw_mux_ctl_pad_gpio_spi_b0_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_05::R](sw_mux_ctl_pad_gpio_spi_b0_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_05::W](sw_mux_ctl_pad_gpio_spi_b0_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_06 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_06](sw_mux_ctl_pad_gpio_spi_b0_06) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_06 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_06;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_06::R](sw_mux_ctl_pad_gpio_spi_b0_06::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_06 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_06::W](sw_mux_ctl_pad_gpio_spi_b0_06::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_06 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_06 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_06;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_07 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_07](sw_mux_ctl_pad_gpio_spi_b0_07) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_07 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_07;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_07::R](sw_mux_ctl_pad_gpio_spi_b0_07::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_07 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_07::W](sw_mux_ctl_pad_gpio_spi_b0_07::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_07 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_07 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_07;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_08 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_08](sw_mux_ctl_pad_gpio_spi_b0_08) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_08 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_08;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_08::R](sw_mux_ctl_pad_gpio_spi_b0_08::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_08 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_08::W](sw_mux_ctl_pad_gpio_spi_b0_08::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_08 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_08 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_08;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_09 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_09](sw_mux_ctl_pad_gpio_spi_b0_09) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_09 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_09;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_09::R](sw_mux_ctl_pad_gpio_spi_b0_09::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_09 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_09::W](sw_mux_ctl_pad_gpio_spi_b0_09::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_09 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_09 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_09;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_10 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_10](sw_mux_ctl_pad_gpio_spi_b0_10) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_10 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_10;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_10::R](sw_mux_ctl_pad_gpio_spi_b0_10::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_10 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_10::W](sw_mux_ctl_pad_gpio_spi_b0_10::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_10 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_10 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_10;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_11 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_11](sw_mux_ctl_pad_gpio_spi_b0_11) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_11 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_11;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_11::R](sw_mux_ctl_pad_gpio_spi_b0_11::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_11 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_11::W](sw_mux_ctl_pad_gpio_spi_b0_11::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_11 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_11 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_11;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_12 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_12](sw_mux_ctl_pad_gpio_spi_b0_12) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_12 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_12;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_12::R](sw_mux_ctl_pad_gpio_spi_b0_12::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_12 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_12::W](sw_mux_ctl_pad_gpio_spi_b0_12::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_12 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_12 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_12;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_13 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b0_13](sw_mux_ctl_pad_gpio_spi_b0_13) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B0_13 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B0_13;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b0_13::R](sw_mux_ctl_pad_gpio_spi_b0_13::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B0_13 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b0_13::W](sw_mux_ctl_pad_gpio_spi_b0_13::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B0_13 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B0_13 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b0_13;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b1_00](sw_mux_ctl_pad_gpio_spi_b1_00) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B1_00 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B1_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B1_00;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b1_00::R](sw_mux_ctl_pad_gpio_spi_b1_00::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B1_00 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b1_00::W](sw_mux_ctl_pad_gpio_spi_b1_00::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B1_00 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_00 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b1_00;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_01 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b1_01](sw_mux_ctl_pad_gpio_spi_b1_01) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B1_01 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B1_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B1_01;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b1_01::R](sw_mux_ctl_pad_gpio_spi_b1_01::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B1_01 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b1_01::W](sw_mux_ctl_pad_gpio_spi_b1_01::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B1_01 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_01 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b1_01;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_02 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b1_02](sw_mux_ctl_pad_gpio_spi_b1_02) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B1_02 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B1_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B1_02;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b1_02::R](sw_mux_ctl_pad_gpio_spi_b1_02::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B1_02 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b1_02::W](sw_mux_ctl_pad_gpio_spi_b1_02::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B1_02 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_02 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b1_02;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_03 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b1_03](sw_mux_ctl_pad_gpio_spi_b1_03) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B1_03 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B1_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B1_03;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b1_03::R](sw_mux_ctl_pad_gpio_spi_b1_03::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B1_03 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b1_03::W](sw_mux_ctl_pad_gpio_spi_b1_03::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B1_03 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_03 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b1_03;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_04 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b1_04](sw_mux_ctl_pad_gpio_spi_b1_04) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B1_04 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B1_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B1_04;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b1_04::R](sw_mux_ctl_pad_gpio_spi_b1_04::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B1_04 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b1_04::W](sw_mux_ctl_pad_gpio_spi_b1_04::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B1_04 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_04 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b1_04;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_05 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b1_05](sw_mux_ctl_pad_gpio_spi_b1_05) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B1_05 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B1_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B1_05;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b1_05::R](sw_mux_ctl_pad_gpio_spi_b1_05::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B1_05 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b1_05::W](sw_mux_ctl_pad_gpio_spi_b1_05::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B1_05 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_05 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b1_05;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_06 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b1_06](sw_mux_ctl_pad_gpio_spi_b1_06) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B1_06 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B1_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B1_06;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b1_06::R](sw_mux_ctl_pad_gpio_spi_b1_06::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B1_06 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b1_06::W](sw_mux_ctl_pad_gpio_spi_b1_06::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B1_06 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_06 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b1_06;
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_07 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_mux_ctl_pad_gpio_spi_b1_07](sw_mux_ctl_pad_gpio_spi_b1_07) module"]
pub type SW_MUX_CTL_PAD_GPIO_SPI_B1_07 = crate::Reg<u32, _SW_MUX_CTL_PAD_GPIO_SPI_B1_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_GPIO_SPI_B1_07;
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_spi_b1_07::R](sw_mux_ctl_pad_gpio_spi_b1_07::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_SPI_B1_07 {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_spi_b1_07::W](sw_mux_ctl_pad_gpio_spi_b1_07::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_SPI_B1_07 {}
#[doc = "SW_MUX_CTL_PAD_GPIO_SPI_B1_07 SW MUX Control Register"]
pub mod sw_mux_ctl_pad_gpio_spi_b1_07;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_00](sw_pad_ctl_pad_gpio_spi_b0_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_00::R](sw_pad_ctl_pad_gpio_spi_b0_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_00::W](sw_pad_ctl_pad_gpio_spi_b0_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_01](sw_pad_ctl_pad_gpio_spi_b0_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_01::R](sw_pad_ctl_pad_gpio_spi_b0_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_01::W](sw_pad_ctl_pad_gpio_spi_b0_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_02](sw_pad_ctl_pad_gpio_spi_b0_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_02::R](sw_pad_ctl_pad_gpio_spi_b0_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_02::W](sw_pad_ctl_pad_gpio_spi_b0_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_03](sw_pad_ctl_pad_gpio_spi_b0_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_03::R](sw_pad_ctl_pad_gpio_spi_b0_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_03::W](sw_pad_ctl_pad_gpio_spi_b0_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_04](sw_pad_ctl_pad_gpio_spi_b0_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_04::R](sw_pad_ctl_pad_gpio_spi_b0_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_04::W](sw_pad_ctl_pad_gpio_spi_b0_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_05](sw_pad_ctl_pad_gpio_spi_b0_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_05::R](sw_pad_ctl_pad_gpio_spi_b0_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_05::W](sw_pad_ctl_pad_gpio_spi_b0_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_06](sw_pad_ctl_pad_gpio_spi_b0_06) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_06 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_06;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_06::R](sw_pad_ctl_pad_gpio_spi_b0_06::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_06 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_06::W](sw_pad_ctl_pad_gpio_spi_b0_06::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_06 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_06 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_06;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_07 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_07](sw_pad_ctl_pad_gpio_spi_b0_07) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_07 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_07;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_07::R](sw_pad_ctl_pad_gpio_spi_b0_07::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_07 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_07::W](sw_pad_ctl_pad_gpio_spi_b0_07::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_07 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_07 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_07;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_08 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_08](sw_pad_ctl_pad_gpio_spi_b0_08) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_08 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_08>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_08;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_08::R](sw_pad_ctl_pad_gpio_spi_b0_08::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_08 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_08::W](sw_pad_ctl_pad_gpio_spi_b0_08::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_08 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_08 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_08;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_09 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_09](sw_pad_ctl_pad_gpio_spi_b0_09) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_09 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_09>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_09;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_09::R](sw_pad_ctl_pad_gpio_spi_b0_09::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_09 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_09::W](sw_pad_ctl_pad_gpio_spi_b0_09::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_09 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_09 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_09;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_10 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_10](sw_pad_ctl_pad_gpio_spi_b0_10) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_10 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_10;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_10::R](sw_pad_ctl_pad_gpio_spi_b0_10::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_10 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_10::W](sw_pad_ctl_pad_gpio_spi_b0_10::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_10 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_10 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_10;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_11 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_11](sw_pad_ctl_pad_gpio_spi_b0_11) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_11 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_11;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_11::R](sw_pad_ctl_pad_gpio_spi_b0_11::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_11 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_11::W](sw_pad_ctl_pad_gpio_spi_b0_11::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_11 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_11 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_11;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_12 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_12](sw_pad_ctl_pad_gpio_spi_b0_12) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_12 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_12;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_12::R](sw_pad_ctl_pad_gpio_spi_b0_12::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_12 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_12::W](sw_pad_ctl_pad_gpio_spi_b0_12::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_12 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_12 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_12;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_13 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b0_13](sw_pad_ctl_pad_gpio_spi_b0_13) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B0_13 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B0_13;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b0_13::R](sw_pad_ctl_pad_gpio_spi_b0_13::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B0_13 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b0_13::W](sw_pad_ctl_pad_gpio_spi_b0_13::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B0_13 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B0_13 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b0_13;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_00 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b1_00](sw_pad_ctl_pad_gpio_spi_b1_00) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B1_00 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B1_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B1_00;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b1_00::R](sw_pad_ctl_pad_gpio_spi_b1_00::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B1_00 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b1_00::W](sw_pad_ctl_pad_gpio_spi_b1_00::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B1_00 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_00 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b1_00;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_01 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b1_01](sw_pad_ctl_pad_gpio_spi_b1_01) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B1_01 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B1_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B1_01;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b1_01::R](sw_pad_ctl_pad_gpio_spi_b1_01::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B1_01 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b1_01::W](sw_pad_ctl_pad_gpio_spi_b1_01::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B1_01 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_01 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b1_01;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_02 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b1_02](sw_pad_ctl_pad_gpio_spi_b1_02) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B1_02 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B1_02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B1_02;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b1_02::R](sw_pad_ctl_pad_gpio_spi_b1_02::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B1_02 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b1_02::W](sw_pad_ctl_pad_gpio_spi_b1_02::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B1_02 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_02 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b1_02;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_03 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b1_03](sw_pad_ctl_pad_gpio_spi_b1_03) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B1_03 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B1_03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B1_03;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b1_03::R](sw_pad_ctl_pad_gpio_spi_b1_03::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B1_03 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b1_03::W](sw_pad_ctl_pad_gpio_spi_b1_03::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B1_03 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_03 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b1_03;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_04 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b1_04](sw_pad_ctl_pad_gpio_spi_b1_04) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B1_04 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B1_04>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B1_04;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b1_04::R](sw_pad_ctl_pad_gpio_spi_b1_04::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B1_04 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b1_04::W](sw_pad_ctl_pad_gpio_spi_b1_04::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B1_04 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_04 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b1_04;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_05 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b1_05](sw_pad_ctl_pad_gpio_spi_b1_05) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B1_05 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B1_05>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B1_05;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b1_05::R](sw_pad_ctl_pad_gpio_spi_b1_05::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B1_05 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b1_05::W](sw_pad_ctl_pad_gpio_spi_b1_05::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B1_05 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_05 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b1_05;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b1_06](sw_pad_ctl_pad_gpio_spi_b1_06) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B1_06 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B1_06>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B1_06;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b1_06::R](sw_pad_ctl_pad_gpio_spi_b1_06::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B1_06 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b1_06::W](sw_pad_ctl_pad_gpio_spi_b1_06::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B1_06 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_06 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b1_06;
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_07 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sw_pad_ctl_pad_gpio_spi_b1_07](sw_pad_ctl_pad_gpio_spi_b1_07) module"]
pub type SW_PAD_CTL_PAD_GPIO_SPI_B1_07 = crate::Reg<u32, _SW_PAD_CTL_PAD_GPIO_SPI_B1_07>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_GPIO_SPI_B1_07;
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_spi_b1_07::R](sw_pad_ctl_pad_gpio_spi_b1_07::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_SPI_B1_07 {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_spi_b1_07::W](sw_pad_ctl_pad_gpio_spi_b1_07::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_SPI_B1_07 {}
#[doc = "SW_PAD_CTL_PAD_GPIO_SPI_B1_07 SW PAD Control Register"]
pub mod sw_pad_ctl_pad_gpio_spi_b1_07;
#[doc = "ENET2_IPG_CLK_RMII_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet2_ipg_clk_rmii_select_input](enet2_ipg_clk_rmii_select_input) module"]
pub type ENET2_IPG_CLK_RMII_SELECT_INPUT = crate::Reg<u32, _ENET2_IPG_CLK_RMII_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET2_IPG_CLK_RMII_SELECT_INPUT;
#[doc = "`read()` method returns [enet2_ipg_clk_rmii_select_input::R](enet2_ipg_clk_rmii_select_input::R) reader structure"]
impl crate::Readable for ENET2_IPG_CLK_RMII_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet2_ipg_clk_rmii_select_input::W](enet2_ipg_clk_rmii_select_input::W) writer structure"]
impl crate::Writable for ENET2_IPG_CLK_RMII_SELECT_INPUT {}
#[doc = "ENET2_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
pub mod enet2_ipg_clk_rmii_select_input;
#[doc = "ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet2_ipp_ind_mac0_mdio_select_input](enet2_ipp_ind_mac0_mdio_select_input) module"]
pub type ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT =
    crate::Reg<u32, _ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT;
#[doc = "`read()` method returns [enet2_ipp_ind_mac0_mdio_select_input::R](enet2_ipp_ind_mac0_mdio_select_input::R) reader structure"]
impl crate::Readable for ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet2_ipp_ind_mac0_mdio_select_input::W](enet2_ipp_ind_mac0_mdio_select_input::W) writer structure"]
impl crate::Writable for ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT {}
#[doc = "ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT DAISY Register"]
pub mod enet2_ipp_ind_mac0_mdio_select_input;
#[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet2_ipp_ind_mac0_rxdata_select_input_0](enet2_ipp_ind_mac0_rxdata_select_input_0) module"]
pub type ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 =
    crate::Reg<u32, _ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0;
#[doc = "`read()` method returns [enet2_ipp_ind_mac0_rxdata_select_input_0::R](enet2_ipp_ind_mac0_rxdata_select_input_0::R) reader structure"]
impl crate::Readable for ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 {}
#[doc = "`write(|w| ..)` method takes [enet2_ipp_ind_mac0_rxdata_select_input_0::W](enet2_ipp_ind_mac0_rxdata_select_input_0::W) writer structure"]
impl crate::Writable for ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 {}
#[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register"]
pub mod enet2_ipp_ind_mac0_rxdata_select_input_0;
#[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet2_ipp_ind_mac0_rxdata_select_input_1](enet2_ipp_ind_mac0_rxdata_select_input_1) module"]
pub type ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 =
    crate::Reg<u32, _ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1;
#[doc = "`read()` method returns [enet2_ipp_ind_mac0_rxdata_select_input_1::R](enet2_ipp_ind_mac0_rxdata_select_input_1::R) reader structure"]
impl crate::Readable for ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 {}
#[doc = "`write(|w| ..)` method takes [enet2_ipp_ind_mac0_rxdata_select_input_1::W](enet2_ipp_ind_mac0_rxdata_select_input_1::W) writer structure"]
impl crate::Writable for ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 {}
#[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register"]
pub mod enet2_ipp_ind_mac0_rxdata_select_input_1;
#[doc = "ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet2_ipp_ind_mac0_rxen_select_input](enet2_ipp_ind_mac0_rxen_select_input) module"]
pub type ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT =
    crate::Reg<u32, _ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT;
#[doc = "`read()` method returns [enet2_ipp_ind_mac0_rxen_select_input::R](enet2_ipp_ind_mac0_rxen_select_input::R) reader structure"]
impl crate::Readable for ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet2_ipp_ind_mac0_rxen_select_input::W](enet2_ipp_ind_mac0_rxen_select_input::W) writer structure"]
impl crate::Writable for ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT {}
#[doc = "ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT DAISY Register"]
pub mod enet2_ipp_ind_mac0_rxen_select_input;
#[doc = "ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet2_ipp_ind_mac0_rxerr_select_input](enet2_ipp_ind_mac0_rxerr_select_input) module"]
pub type ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT =
    crate::Reg<u32, _ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT;
#[doc = "`read()` method returns [enet2_ipp_ind_mac0_rxerr_select_input::R](enet2_ipp_ind_mac0_rxerr_select_input::R) reader structure"]
impl crate::Readable for ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet2_ipp_ind_mac0_rxerr_select_input::W](enet2_ipp_ind_mac0_rxerr_select_input::W) writer structure"]
impl crate::Writable for ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT {}
#[doc = "ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT DAISY Register"]
pub mod enet2_ipp_ind_mac0_rxerr_select_input;
#[doc = "ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet2_ipp_ind_mac0_timer_select_input_0](enet2_ipp_ind_mac0_timer_select_input_0) module"]
pub type ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 =
    crate::Reg<u32, _ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0;
#[doc = "`read()` method returns [enet2_ipp_ind_mac0_timer_select_input_0::R](enet2_ipp_ind_mac0_timer_select_input_0::R) reader structure"]
impl crate::Readable for ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 {}
#[doc = "`write(|w| ..)` method takes [enet2_ipp_ind_mac0_timer_select_input_0::W](enet2_ipp_ind_mac0_timer_select_input_0::W) writer structure"]
impl crate::Writable for ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 {}
#[doc = "ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 DAISY Register"]
pub mod enet2_ipp_ind_mac0_timer_select_input_0;
#[doc = "ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enet2_ipp_ind_mac0_txclk_select_input](enet2_ipp_ind_mac0_txclk_select_input) module"]
pub type ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT =
    crate::Reg<u32, _ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT;
#[doc = "`read()` method returns [enet2_ipp_ind_mac0_txclk_select_input::R](enet2_ipp_ind_mac0_txclk_select_input::R) reader structure"]
impl crate::Readable for ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [enet2_ipp_ind_mac0_txclk_select_input::W](enet2_ipp_ind_mac0_txclk_select_input::W) writer structure"]
impl crate::Writable for ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT {}
#[doc = "ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT DAISY Register"]
pub mod enet2_ipp_ind_mac0_txclk_select_input;
#[doc = "FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_dqs_fa_select_input](flexspi2_ipp_ind_dqs_fa_select_input) module"]
pub type FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_dqs_fa_select_input::R](flexspi2_ipp_ind_dqs_fa_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_dqs_fa_select_input::W](flexspi2_ipp_ind_dqs_fa_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_DQS_FA_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_dqs_fa_select_input;
#[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_io_fa_bit0_select_input](flexspi2_ipp_ind_io_fa_bit0_select_input) module"]
pub type FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_io_fa_bit0_select_input::R](flexspi2_ipp_ind_io_fa_bit0_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_io_fa_bit0_select_input::W](flexspi2_ipp_ind_io_fa_bit0_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT0_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_io_fa_bit0_select_input;
#[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_io_fa_bit1_select_input](flexspi2_ipp_ind_io_fa_bit1_select_input) module"]
pub type FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_io_fa_bit1_select_input::R](flexspi2_ipp_ind_io_fa_bit1_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_io_fa_bit1_select_input::W](flexspi2_ipp_ind_io_fa_bit1_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT1_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_io_fa_bit1_select_input;
#[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_io_fa_bit2_select_input](flexspi2_ipp_ind_io_fa_bit2_select_input) module"]
pub type FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_io_fa_bit2_select_input::R](flexspi2_ipp_ind_io_fa_bit2_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_io_fa_bit2_select_input::W](flexspi2_ipp_ind_io_fa_bit2_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT2_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_io_fa_bit2_select_input;
#[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_io_fa_bit3_select_input](flexspi2_ipp_ind_io_fa_bit3_select_input) module"]
pub type FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_io_fa_bit3_select_input::R](flexspi2_ipp_ind_io_fa_bit3_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_io_fa_bit3_select_input::W](flexspi2_ipp_ind_io_fa_bit3_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_IO_FA_BIT3_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_io_fa_bit3_select_input;
#[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_io_fb_bit0_select_input](flexspi2_ipp_ind_io_fb_bit0_select_input) module"]
pub type FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_io_fb_bit0_select_input::R](flexspi2_ipp_ind_io_fb_bit0_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_io_fb_bit0_select_input::W](flexspi2_ipp_ind_io_fb_bit0_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT0_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_io_fb_bit0_select_input;
#[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_io_fb_bit1_select_input](flexspi2_ipp_ind_io_fb_bit1_select_input) module"]
pub type FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_io_fb_bit1_select_input::R](flexspi2_ipp_ind_io_fb_bit1_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_io_fb_bit1_select_input::W](flexspi2_ipp_ind_io_fb_bit1_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT1_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_io_fb_bit1_select_input;
#[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_io_fb_bit2_select_input](flexspi2_ipp_ind_io_fb_bit2_select_input) module"]
pub type FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_io_fb_bit2_select_input::R](flexspi2_ipp_ind_io_fb_bit2_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_io_fb_bit2_select_input::W](flexspi2_ipp_ind_io_fb_bit2_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT2_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_io_fb_bit2_select_input;
#[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_io_fb_bit3_select_input](flexspi2_ipp_ind_io_fb_bit3_select_input) module"]
pub type FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_io_fb_bit3_select_input::R](flexspi2_ipp_ind_io_fb_bit3_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_io_fb_bit3_select_input::W](flexspi2_ipp_ind_io_fb_bit3_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_IO_FB_BIT3_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_io_fb_bit3_select_input;
#[doc = "FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_sck_fa_select_input](flexspi2_ipp_ind_sck_fa_select_input) module"]
pub type FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_sck_fa_select_input::R](flexspi2_ipp_ind_sck_fa_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_sck_fa_select_input::W](flexspi2_ipp_ind_sck_fa_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_SCK_FA_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_sck_fa_select_input;
#[doc = "FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexspi2_ipp_ind_sck_fb_select_input](flexspi2_ipp_ind_sck_fb_select_input) module"]
pub type FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT =
    crate::Reg<u32, _FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT;
#[doc = "`read()` method returns [flexspi2_ipp_ind_sck_fb_select_input::R](flexspi2_ipp_ind_sck_fb_select_input::R) reader structure"]
impl crate::Readable for FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [flexspi2_ipp_ind_sck_fb_select_input::W](flexspi2_ipp_ind_sck_fb_select_input::W) writer structure"]
impl crate::Writable for FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT {}
#[doc = "FLEXSPI2_IPP_IND_SCK_FB_SELECT_INPUT DAISY Register"]
pub mod flexspi2_ipp_ind_sck_fb_select_input;
#[doc = "GPT1_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt1_ipp_ind_capin1_select_input](gpt1_ipp_ind_capin1_select_input) module"]
pub type GPT1_IPP_IND_CAPIN1_SELECT_INPUT = crate::Reg<u32, _GPT1_IPP_IND_CAPIN1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1_IPP_IND_CAPIN1_SELECT_INPUT;
#[doc = "`read()` method returns [gpt1_ipp_ind_capin1_select_input::R](gpt1_ipp_ind_capin1_select_input::R) reader structure"]
impl crate::Readable for GPT1_IPP_IND_CAPIN1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [gpt1_ipp_ind_capin1_select_input::W](gpt1_ipp_ind_capin1_select_input::W) writer structure"]
impl crate::Writable for GPT1_IPP_IND_CAPIN1_SELECT_INPUT {}
#[doc = "GPT1_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
pub mod gpt1_ipp_ind_capin1_select_input;
#[doc = "GPT1_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt1_ipp_ind_capin2_select_input](gpt1_ipp_ind_capin2_select_input) module"]
pub type GPT1_IPP_IND_CAPIN2_SELECT_INPUT = crate::Reg<u32, _GPT1_IPP_IND_CAPIN2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1_IPP_IND_CAPIN2_SELECT_INPUT;
#[doc = "`read()` method returns [gpt1_ipp_ind_capin2_select_input::R](gpt1_ipp_ind_capin2_select_input::R) reader structure"]
impl crate::Readable for GPT1_IPP_IND_CAPIN2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [gpt1_ipp_ind_capin2_select_input::W](gpt1_ipp_ind_capin2_select_input::W) writer structure"]
impl crate::Writable for GPT1_IPP_IND_CAPIN2_SELECT_INPUT {}
#[doc = "GPT1_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
pub mod gpt1_ipp_ind_capin2_select_input;
#[doc = "GPT1_IPP_IND_CLKIN_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt1_ipp_ind_clkin_select_input](gpt1_ipp_ind_clkin_select_input) module"]
pub type GPT1_IPP_IND_CLKIN_SELECT_INPUT = crate::Reg<u32, _GPT1_IPP_IND_CLKIN_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1_IPP_IND_CLKIN_SELECT_INPUT;
#[doc = "`read()` method returns [gpt1_ipp_ind_clkin_select_input::R](gpt1_ipp_ind_clkin_select_input::R) reader structure"]
impl crate::Readable for GPT1_IPP_IND_CLKIN_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [gpt1_ipp_ind_clkin_select_input::W](gpt1_ipp_ind_clkin_select_input::W) writer structure"]
impl crate::Writable for GPT1_IPP_IND_CLKIN_SELECT_INPUT {}
#[doc = "GPT1_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
pub mod gpt1_ipp_ind_clkin_select_input;
#[doc = "GPT2_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt2_ipp_ind_capin1_select_input](gpt2_ipp_ind_capin1_select_input) module"]
pub type GPT2_IPP_IND_CAPIN1_SELECT_INPUT = crate::Reg<u32, _GPT2_IPP_IND_CAPIN1_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2_IPP_IND_CAPIN1_SELECT_INPUT;
#[doc = "`read()` method returns [gpt2_ipp_ind_capin1_select_input::R](gpt2_ipp_ind_capin1_select_input::R) reader structure"]
impl crate::Readable for GPT2_IPP_IND_CAPIN1_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [gpt2_ipp_ind_capin1_select_input::W](gpt2_ipp_ind_capin1_select_input::W) writer structure"]
impl crate::Writable for GPT2_IPP_IND_CAPIN1_SELECT_INPUT {}
#[doc = "GPT2_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
pub mod gpt2_ipp_ind_capin1_select_input;
#[doc = "GPT2_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt2_ipp_ind_capin2_select_input](gpt2_ipp_ind_capin2_select_input) module"]
pub type GPT2_IPP_IND_CAPIN2_SELECT_INPUT = crate::Reg<u32, _GPT2_IPP_IND_CAPIN2_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2_IPP_IND_CAPIN2_SELECT_INPUT;
#[doc = "`read()` method returns [gpt2_ipp_ind_capin2_select_input::R](gpt2_ipp_ind_capin2_select_input::R) reader structure"]
impl crate::Readable for GPT2_IPP_IND_CAPIN2_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [gpt2_ipp_ind_capin2_select_input::W](gpt2_ipp_ind_capin2_select_input::W) writer structure"]
impl crate::Writable for GPT2_IPP_IND_CAPIN2_SELECT_INPUT {}
#[doc = "GPT2_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
pub mod gpt2_ipp_ind_capin2_select_input;
#[doc = "GPT2_IPP_IND_CLKIN_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt2_ipp_ind_clkin_select_input](gpt2_ipp_ind_clkin_select_input) module"]
pub type GPT2_IPP_IND_CLKIN_SELECT_INPUT = crate::Reg<u32, _GPT2_IPP_IND_CLKIN_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2_IPP_IND_CLKIN_SELECT_INPUT;
#[doc = "`read()` method returns [gpt2_ipp_ind_clkin_select_input::R](gpt2_ipp_ind_clkin_select_input::R) reader structure"]
impl crate::Readable for GPT2_IPP_IND_CLKIN_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [gpt2_ipp_ind_clkin_select_input::W](gpt2_ipp_ind_clkin_select_input::W) writer structure"]
impl crate::Writable for GPT2_IPP_IND_CLKIN_SELECT_INPUT {}
#[doc = "GPT2_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
pub mod gpt2_ipp_ind_clkin_select_input;
#[doc = "SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai3_ipg_clk_sai_mclk_select_input_2](sai3_ipg_clk_sai_mclk_select_input_2) module"]
pub type SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 =
    crate::Reg<u32, _SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2;
#[doc = "`read()` method returns [sai3_ipg_clk_sai_mclk_select_input_2::R](sai3_ipg_clk_sai_mclk_select_input_2::R) reader structure"]
impl crate::Readable for SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {}
#[doc = "`write(|w| ..)` method takes [sai3_ipg_clk_sai_mclk_select_input_2::W](sai3_ipg_clk_sai_mclk_select_input_2::W) writer structure"]
impl crate::Writable for SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 {}
#[doc = "SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
pub mod sai3_ipg_clk_sai_mclk_select_input_2;
#[doc = "SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai3_ipp_ind_sai_rxbclk_select_input](sai3_ipp_ind_sai_rxbclk_select_input) module"]
pub type SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT =
    crate::Reg<u32, _SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT;
#[doc = "`read()` method returns [sai3_ipp_ind_sai_rxbclk_select_input::R](sai3_ipp_ind_sai_rxbclk_select_input::R) reader structure"]
impl crate::Readable for SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai3_ipp_ind_sai_rxbclk_select_input::W](sai3_ipp_ind_sai_rxbclk_select_input::W) writer structure"]
impl crate::Writable for SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT {}
#[doc = "SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
pub mod sai3_ipp_ind_sai_rxbclk_select_input;
#[doc = "SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai3_ipp_ind_sai_rxdata_select_input_0](sai3_ipp_ind_sai_rxdata_select_input_0) module"]
pub type SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 =
    crate::Reg<u32, _SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0;
#[doc = "`read()` method returns [sai3_ipp_ind_sai_rxdata_select_input_0::R](sai3_ipp_ind_sai_rxdata_select_input_0::R) reader structure"]
impl crate::Readable for SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {}
#[doc = "`write(|w| ..)` method takes [sai3_ipp_ind_sai_rxdata_select_input_0::W](sai3_ipp_ind_sai_rxdata_select_input_0::W) writer structure"]
impl crate::Writable for SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 {}
#[doc = "SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
pub mod sai3_ipp_ind_sai_rxdata_select_input_0;
#[doc = "SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai3_ipp_ind_sai_rxsync_select_input](sai3_ipp_ind_sai_rxsync_select_input) module"]
pub type SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT =
    crate::Reg<u32, _SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT;
#[doc = "`read()` method returns [sai3_ipp_ind_sai_rxsync_select_input::R](sai3_ipp_ind_sai_rxsync_select_input::R) reader structure"]
impl crate::Readable for SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai3_ipp_ind_sai_rxsync_select_input::W](sai3_ipp_ind_sai_rxsync_select_input::W) writer structure"]
impl crate::Writable for SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT {}
#[doc = "SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
pub mod sai3_ipp_ind_sai_rxsync_select_input;
#[doc = "SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai3_ipp_ind_sai_txbclk_select_input](sai3_ipp_ind_sai_txbclk_select_input) module"]
pub type SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT =
    crate::Reg<u32, _SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT;
#[doc = "`read()` method returns [sai3_ipp_ind_sai_txbclk_select_input::R](sai3_ipp_ind_sai_txbclk_select_input::R) reader structure"]
impl crate::Readable for SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai3_ipp_ind_sai_txbclk_select_input::W](sai3_ipp_ind_sai_txbclk_select_input::W) writer structure"]
impl crate::Writable for SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT {}
#[doc = "SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
pub mod sai3_ipp_ind_sai_txbclk_select_input;
#[doc = "SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sai3_ipp_ind_sai_txsync_select_input](sai3_ipp_ind_sai_txsync_select_input) module"]
pub type SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT =
    crate::Reg<u32, _SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT;
#[doc = "`read()` method returns [sai3_ipp_ind_sai_txsync_select_input::R](sai3_ipp_ind_sai_txsync_select_input::R) reader structure"]
impl crate::Readable for SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [sai3_ipp_ind_sai_txsync_select_input::W](sai3_ipp_ind_sai_txsync_select_input::W) writer structure"]
impl crate::Writable for SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT {}
#[doc = "SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
pub mod sai3_ipp_ind_sai_txsync_select_input;
#[doc = "SEMC_I_IPP_IND_DQS4_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [semc_i_ipp_ind_dqs4_select_input](semc_i_ipp_ind_dqs4_select_input) module"]
pub type SEMC_I_IPP_IND_DQS4_SELECT_INPUT = crate::Reg<u32, _SEMC_I_IPP_IND_DQS4_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMC_I_IPP_IND_DQS4_SELECT_INPUT;
#[doc = "`read()` method returns [semc_i_ipp_ind_dqs4_select_input::R](semc_i_ipp_ind_dqs4_select_input::R) reader structure"]
impl crate::Readable for SEMC_I_IPP_IND_DQS4_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [semc_i_ipp_ind_dqs4_select_input::W](semc_i_ipp_ind_dqs4_select_input::W) writer structure"]
impl crate::Writable for SEMC_I_IPP_IND_DQS4_SELECT_INPUT {}
#[doc = "SEMC_I_IPP_IND_DQS4_SELECT_INPUT DAISY Register"]
pub mod semc_i_ipp_ind_dqs4_select_input;
#[doc = "CANFD_IPP_IND_CANRX_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [canfd_ipp_ind_canrx_select_input](canfd_ipp_ind_canrx_select_input) module"]
pub type CANFD_IPP_IND_CANRX_SELECT_INPUT = crate::Reg<u32, _CANFD_IPP_IND_CANRX_SELECT_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANFD_IPP_IND_CANRX_SELECT_INPUT;
#[doc = "`read()` method returns [canfd_ipp_ind_canrx_select_input::R](canfd_ipp_ind_canrx_select_input::R) reader structure"]
impl crate::Readable for CANFD_IPP_IND_CANRX_SELECT_INPUT {}
#[doc = "`write(|w| ..)` method takes [canfd_ipp_ind_canrx_select_input::W](canfd_ipp_ind_canrx_select_input::W) writer structure"]
impl crate::Writable for CANFD_IPP_IND_CANRX_SELECT_INPUT {}
#[doc = "CANFD_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
pub mod canfd_ipp_ind_canrx_select_input;
