#[doc = "Reader of register CCOSR"]
pub type R = crate::R<u32, super::CCOSR>;
#[doc = "Writer for register CCOSR"]
pub type W = crate::W<u32, super::CCOSR>;
#[doc = "Register CCOSR `reset()`'s with value 0x000a_0001"]
impl crate::ResetValue for super::CCOSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000a_0001
    }
}
#[doc = "Selection of the clock to be generated on CCM_CLKO1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO1_SEL_A {
    #[doc = "0: USB1 PLL clock (divided by 2)"]
    CLKO1_SEL_0,
    #[doc = "1: SYS PLL clock (divided by 2)"]
    CLKO1_SEL_1,
    #[doc = "3: VIDEO PLL clock (divided by 2)"]
    CLKO1_SEL_3,
    #[doc = "5: semc_clk_root"]
    CLKO1_SEL_5,
    #[doc = "10: lcdif_pix_clk_root"]
    CLKO1_SEL_10,
    #[doc = "11: ahb_clk_root"]
    CLKO1_SEL_11,
    #[doc = "12: ipg_clk_root"]
    CLKO1_SEL_12,
    #[doc = "13: perclk_root"]
    CLKO1_SEL_13,
    #[doc = "14: ckil_sync_clk_root"]
    CLKO1_SEL_14,
    #[doc = "15: pll4_main_clk"]
    CLKO1_SEL_15,
}
impl From<CLKO1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKO1_SEL_A) -> Self {
        match variant {
            CLKO1_SEL_A::CLKO1_SEL_0 => 0,
            CLKO1_SEL_A::CLKO1_SEL_1 => 1,
            CLKO1_SEL_A::CLKO1_SEL_3 => 3,
            CLKO1_SEL_A::CLKO1_SEL_5 => 5,
            CLKO1_SEL_A::CLKO1_SEL_10 => 10,
            CLKO1_SEL_A::CLKO1_SEL_11 => 11,
            CLKO1_SEL_A::CLKO1_SEL_12 => 12,
            CLKO1_SEL_A::CLKO1_SEL_13 => 13,
            CLKO1_SEL_A::CLKO1_SEL_14 => 14,
            CLKO1_SEL_A::CLKO1_SEL_15 => 15,
        }
    }
}
#[doc = "Reader of field `CLKO1_SEL`"]
pub type CLKO1_SEL_R = crate::R<u8, CLKO1_SEL_A>;
impl CLKO1_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKO1_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKO1_SEL_A::CLKO1_SEL_0),
            1 => Val(CLKO1_SEL_A::CLKO1_SEL_1),
            3 => Val(CLKO1_SEL_A::CLKO1_SEL_3),
            5 => Val(CLKO1_SEL_A::CLKO1_SEL_5),
            10 => Val(CLKO1_SEL_A::CLKO1_SEL_10),
            11 => Val(CLKO1_SEL_A::CLKO1_SEL_11),
            12 => Val(CLKO1_SEL_A::CLKO1_SEL_12),
            13 => Val(CLKO1_SEL_A::CLKO1_SEL_13),
            14 => Val(CLKO1_SEL_A::CLKO1_SEL_14),
            15 => Val(CLKO1_SEL_A::CLKO1_SEL_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_0`"]
    #[inline(always)]
    pub fn is_clko1_sel_0(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_1`"]
    #[inline(always)]
    pub fn is_clko1_sel_1(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_1
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_3`"]
    #[inline(always)]
    pub fn is_clko1_sel_3(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_3
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_5`"]
    #[inline(always)]
    pub fn is_clko1_sel_5(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_5
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_10`"]
    #[inline(always)]
    pub fn is_clko1_sel_10(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_10
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_11`"]
    #[inline(always)]
    pub fn is_clko1_sel_11(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_11
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_12`"]
    #[inline(always)]
    pub fn is_clko1_sel_12(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_12
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_13`"]
    #[inline(always)]
    pub fn is_clko1_sel_13(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_13
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_14`"]
    #[inline(always)]
    pub fn is_clko1_sel_14(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_14
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_15`"]
    #[inline(always)]
    pub fn is_clko1_sel_15(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_15
    }
}
#[doc = "Write proxy for field `CLKO1_SEL`"]
pub struct CLKO1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO1_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO1_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "USB1 PLL clock (divided by 2)"]
    #[inline(always)]
    pub fn clko1_sel_0(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_0)
    }
    #[doc = "SYS PLL clock (divided by 2)"]
    #[inline(always)]
    pub fn clko1_sel_1(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_1)
    }
    #[doc = "VIDEO PLL clock (divided by 2)"]
    #[inline(always)]
    pub fn clko1_sel_3(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_3)
    }
    #[doc = "semc_clk_root"]
    #[inline(always)]
    pub fn clko1_sel_5(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_5)
    }
    #[doc = "lcdif_pix_clk_root"]
    #[inline(always)]
    pub fn clko1_sel_10(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_10)
    }
    #[doc = "ahb_clk_root"]
    #[inline(always)]
    pub fn clko1_sel_11(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_11)
    }
    #[doc = "ipg_clk_root"]
    #[inline(always)]
    pub fn clko1_sel_12(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_12)
    }
    #[doc = "perclk_root"]
    #[inline(always)]
    pub fn clko1_sel_13(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_13)
    }
    #[doc = "ckil_sync_clk_root"]
    #[inline(always)]
    pub fn clko1_sel_14(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_14)
    }
    #[doc = "pll4_main_clk"]
    #[inline(always)]
    pub fn clko1_sel_15(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Setting the divider of CCM_CLKO1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO1_DIV_A {
    #[doc = "0: divide by 1"]
    CLKO1_DIV_0,
    #[doc = "1: divide by 2"]
    CLKO1_DIV_1,
    #[doc = "2: divide by 3"]
    CLKO1_DIV_2,
    #[doc = "3: divide by 4"]
    CLKO1_DIV_3,
    #[doc = "4: divide by 5"]
    CLKO1_DIV_4,
    #[doc = "5: divide by 6"]
    CLKO1_DIV_5,
    #[doc = "6: divide by 7"]
    CLKO1_DIV_6,
    #[doc = "7: divide by 8"]
    CLKO1_DIV_7,
}
impl From<CLKO1_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKO1_DIV_A) -> Self {
        match variant {
            CLKO1_DIV_A::CLKO1_DIV_0 => 0,
            CLKO1_DIV_A::CLKO1_DIV_1 => 1,
            CLKO1_DIV_A::CLKO1_DIV_2 => 2,
            CLKO1_DIV_A::CLKO1_DIV_3 => 3,
            CLKO1_DIV_A::CLKO1_DIV_4 => 4,
            CLKO1_DIV_A::CLKO1_DIV_5 => 5,
            CLKO1_DIV_A::CLKO1_DIV_6 => 6,
            CLKO1_DIV_A::CLKO1_DIV_7 => 7,
        }
    }
}
#[doc = "Reader of field `CLKO1_DIV`"]
pub type CLKO1_DIV_R = crate::R<u8, CLKO1_DIV_A>;
impl CLKO1_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO1_DIV_A {
        match self.bits {
            0 => CLKO1_DIV_A::CLKO1_DIV_0,
            1 => CLKO1_DIV_A::CLKO1_DIV_1,
            2 => CLKO1_DIV_A::CLKO1_DIV_2,
            3 => CLKO1_DIV_A::CLKO1_DIV_3,
            4 => CLKO1_DIV_A::CLKO1_DIV_4,
            5 => CLKO1_DIV_A::CLKO1_DIV_5,
            6 => CLKO1_DIV_A::CLKO1_DIV_6,
            7 => CLKO1_DIV_A::CLKO1_DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_0`"]
    #[inline(always)]
    pub fn is_clko1_div_0(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_1`"]
    #[inline(always)]
    pub fn is_clko1_div_1(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_2`"]
    #[inline(always)]
    pub fn is_clko1_div_2(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_3`"]
    #[inline(always)]
    pub fn is_clko1_div_3(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_4`"]
    #[inline(always)]
    pub fn is_clko1_div_4(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_5`"]
    #[inline(always)]
    pub fn is_clko1_div_5(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_6`"]
    #[inline(always)]
    pub fn is_clko1_div_6(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_6
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_7`"]
    #[inline(always)]
    pub fn is_clko1_div_7(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_7
    }
}
#[doc = "Write proxy for field `CLKO1_DIV`"]
pub struct CLKO1_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO1_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO1_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn clko1_div_0(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn clko1_div_1(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn clko1_div_2(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn clko1_div_3(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn clko1_div_4(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn clko1_div_5(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn clko1_div_6(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn clko1_div_7(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Enable of CCM_CLKO1 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO1_EN_A {
    #[doc = "0: CCM_CLKO1 disabled."]
    CLKO1_EN_0,
    #[doc = "1: CCM_CLKO1 enabled."]
    CLKO1_EN_1,
}
impl From<CLKO1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKO1_EN_A) -> Self {
        match variant {
            CLKO1_EN_A::CLKO1_EN_0 => false,
            CLKO1_EN_A::CLKO1_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `CLKO1_EN`"]
pub type CLKO1_EN_R = crate::R<bool, CLKO1_EN_A>;
impl CLKO1_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO1_EN_A {
        match self.bits {
            false => CLKO1_EN_A::CLKO1_EN_0,
            true => CLKO1_EN_A::CLKO1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_EN_0`"]
    #[inline(always)]
    pub fn is_clko1_en_0(&self) -> bool {
        *self == CLKO1_EN_A::CLKO1_EN_0
    }
    #[doc = "Checks if the value of the field is `CLKO1_EN_1`"]
    #[inline(always)]
    pub fn is_clko1_en_1(&self) -> bool {
        *self == CLKO1_EN_A::CLKO1_EN_1
    }
}
#[doc = "Write proxy for field `CLKO1_EN`"]
pub struct CLKO1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO1_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CCM_CLKO1 disabled."]
    #[inline(always)]
    pub fn clko1_en_0(self) -> &'a mut W {
        self.variant(CLKO1_EN_A::CLKO1_EN_0)
    }
    #[doc = "CCM_CLKO1 enabled."]
    #[inline(always)]
    pub fn clko1_en_1(self) -> &'a mut W {
        self.variant(CLKO1_EN_A::CLKO1_EN_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_OUT_SEL_A {
    #[doc = "0: CCM_CLKO1 output drives CCM_CLKO1 clock"]
    CLK_OUT_SEL_0,
    #[doc = "1: CCM_CLKO1 output drives CCM_CLKO2 clock"]
    CLK_OUT_SEL_1,
}
impl From<CLK_OUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_OUT_SEL_A) -> Self {
        match variant {
            CLK_OUT_SEL_A::CLK_OUT_SEL_0 => false,
            CLK_OUT_SEL_A::CLK_OUT_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `CLK_OUT_SEL`"]
pub type CLK_OUT_SEL_R = crate::R<bool, CLK_OUT_SEL_A>;
impl CLK_OUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_OUT_SEL_A {
        match self.bits {
            false => CLK_OUT_SEL_A::CLK_OUT_SEL_0,
            true => CLK_OUT_SEL_A::CLK_OUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_OUT_SEL_0`"]
    #[inline(always)]
    pub fn is_clk_out_sel_0(&self) -> bool {
        *self == CLK_OUT_SEL_A::CLK_OUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLK_OUT_SEL_1`"]
    #[inline(always)]
    pub fn is_clk_out_sel_1(&self) -> bool {
        *self == CLK_OUT_SEL_A::CLK_OUT_SEL_1
    }
}
#[doc = "Write proxy for field `CLK_OUT_SEL`"]
pub struct CLK_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_OUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_OUT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CCM_CLKO1 output drives CCM_CLKO1 clock"]
    #[inline(always)]
    pub fn clk_out_sel_0(self) -> &'a mut W {
        self.variant(CLK_OUT_SEL_A::CLK_OUT_SEL_0)
    }
    #[doc = "CCM_CLKO1 output drives CCM_CLKO2 clock"]
    #[inline(always)]
    pub fn clk_out_sel_1(self) -> &'a mut W {
        self.variant(CLK_OUT_SEL_A::CLK_OUT_SEL_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Selection of the clock to be generated on CCM_CLKO2\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO2_SEL_A {
    #[doc = "3: usdhc1_clk_root"]
    CLKO2_SEL_3,
    #[doc = "6: lpi2c_clk_root"]
    CLKO2_SEL_6,
    #[doc = "11: csi_clk_root"]
    CLKO2_SEL_11,
    #[doc = "14: osc_clk"]
    CLKO2_SEL_14,
    #[doc = "17: usdhc2_clk_root"]
    CLKO2_SEL_17,
    #[doc = "18: sai1_clk_root"]
    CLKO2_SEL_18,
    #[doc = "19: sai2_clk_root"]
    CLKO2_SEL_19,
    #[doc = "20: sai3_clk_root (shared with ADC1 and ADC2 alt_clk root)"]
    CLKO2_SEL_20,
    #[doc = "23: can_clk_root (FlexCAN, shared with CANFD)"]
    CLKO2_SEL_23,
    #[doc = "27: flexspi_clk_root"]
    CLKO2_SEL_27,
    #[doc = "28: uart_clk_root"]
    CLKO2_SEL_28,
    #[doc = "29: spdif0_clk_root"]
    CLKO2_SEL_29,
}
impl From<CLKO2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKO2_SEL_A) -> Self {
        match variant {
            CLKO2_SEL_A::CLKO2_SEL_3 => 3,
            CLKO2_SEL_A::CLKO2_SEL_6 => 6,
            CLKO2_SEL_A::CLKO2_SEL_11 => 11,
            CLKO2_SEL_A::CLKO2_SEL_14 => 14,
            CLKO2_SEL_A::CLKO2_SEL_17 => 17,
            CLKO2_SEL_A::CLKO2_SEL_18 => 18,
            CLKO2_SEL_A::CLKO2_SEL_19 => 19,
            CLKO2_SEL_A::CLKO2_SEL_20 => 20,
            CLKO2_SEL_A::CLKO2_SEL_23 => 23,
            CLKO2_SEL_A::CLKO2_SEL_27 => 27,
            CLKO2_SEL_A::CLKO2_SEL_28 => 28,
            CLKO2_SEL_A::CLKO2_SEL_29 => 29,
        }
    }
}
#[doc = "Reader of field `CLKO2_SEL`"]
pub type CLKO2_SEL_R = crate::R<u8, CLKO2_SEL_A>;
impl CLKO2_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKO2_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(CLKO2_SEL_A::CLKO2_SEL_3),
            6 => Val(CLKO2_SEL_A::CLKO2_SEL_6),
            11 => Val(CLKO2_SEL_A::CLKO2_SEL_11),
            14 => Val(CLKO2_SEL_A::CLKO2_SEL_14),
            17 => Val(CLKO2_SEL_A::CLKO2_SEL_17),
            18 => Val(CLKO2_SEL_A::CLKO2_SEL_18),
            19 => Val(CLKO2_SEL_A::CLKO2_SEL_19),
            20 => Val(CLKO2_SEL_A::CLKO2_SEL_20),
            23 => Val(CLKO2_SEL_A::CLKO2_SEL_23),
            27 => Val(CLKO2_SEL_A::CLKO2_SEL_27),
            28 => Val(CLKO2_SEL_A::CLKO2_SEL_28),
            29 => Val(CLKO2_SEL_A::CLKO2_SEL_29),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_3`"]
    #[inline(always)]
    pub fn is_clko2_sel_3(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_3
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_6`"]
    #[inline(always)]
    pub fn is_clko2_sel_6(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_6
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_11`"]
    #[inline(always)]
    pub fn is_clko2_sel_11(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_11
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_14`"]
    #[inline(always)]
    pub fn is_clko2_sel_14(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_14
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_17`"]
    #[inline(always)]
    pub fn is_clko2_sel_17(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_17
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_18`"]
    #[inline(always)]
    pub fn is_clko2_sel_18(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_18
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_19`"]
    #[inline(always)]
    pub fn is_clko2_sel_19(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_19
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_20`"]
    #[inline(always)]
    pub fn is_clko2_sel_20(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_20
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_23`"]
    #[inline(always)]
    pub fn is_clko2_sel_23(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_23
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_27`"]
    #[inline(always)]
    pub fn is_clko2_sel_27(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_27
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_28`"]
    #[inline(always)]
    pub fn is_clko2_sel_28(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_28
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_29`"]
    #[inline(always)]
    pub fn is_clko2_sel_29(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_29
    }
}
#[doc = "Write proxy for field `CLKO2_SEL`"]
pub struct CLKO2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO2_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO2_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "usdhc1_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_3(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_3)
    }
    #[doc = "lpi2c_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_6(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_6)
    }
    #[doc = "csi_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_11(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_11)
    }
    #[doc = "osc_clk"]
    #[inline(always)]
    pub fn clko2_sel_14(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_14)
    }
    #[doc = "usdhc2_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_17(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_17)
    }
    #[doc = "sai1_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_18(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_18)
    }
    #[doc = "sai2_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_19(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_19)
    }
    #[doc = "sai3_clk_root (shared with ADC1 and ADC2 alt_clk root)"]
    #[inline(always)]
    pub fn clko2_sel_20(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_20)
    }
    #[doc = "can_clk_root (FlexCAN, shared with CANFD)"]
    #[inline(always)]
    pub fn clko2_sel_23(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_23)
    }
    #[doc = "flexspi_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_27(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_27)
    }
    #[doc = "uart_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_28(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_28)
    }
    #[doc = "spdif0_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_29(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_29)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Setting the divider of CCM_CLKO2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO2_DIV_A {
    #[doc = "0: divide by 1"]
    CLKO2_DIV_0,
    #[doc = "1: divide by 2"]
    CLKO2_DIV_1,
    #[doc = "2: divide by 3"]
    CLKO2_DIV_2,
    #[doc = "3: divide by 4"]
    CLKO2_DIV_3,
    #[doc = "4: divide by 5"]
    CLKO2_DIV_4,
    #[doc = "5: divide by 6"]
    CLKO2_DIV_5,
    #[doc = "6: divide by 7"]
    CLKO2_DIV_6,
    #[doc = "7: divide by 8"]
    CLKO2_DIV_7,
}
impl From<CLKO2_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKO2_DIV_A) -> Self {
        match variant {
            CLKO2_DIV_A::CLKO2_DIV_0 => 0,
            CLKO2_DIV_A::CLKO2_DIV_1 => 1,
            CLKO2_DIV_A::CLKO2_DIV_2 => 2,
            CLKO2_DIV_A::CLKO2_DIV_3 => 3,
            CLKO2_DIV_A::CLKO2_DIV_4 => 4,
            CLKO2_DIV_A::CLKO2_DIV_5 => 5,
            CLKO2_DIV_A::CLKO2_DIV_6 => 6,
            CLKO2_DIV_A::CLKO2_DIV_7 => 7,
        }
    }
}
#[doc = "Reader of field `CLKO2_DIV`"]
pub type CLKO2_DIV_R = crate::R<u8, CLKO2_DIV_A>;
impl CLKO2_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO2_DIV_A {
        match self.bits {
            0 => CLKO2_DIV_A::CLKO2_DIV_0,
            1 => CLKO2_DIV_A::CLKO2_DIV_1,
            2 => CLKO2_DIV_A::CLKO2_DIV_2,
            3 => CLKO2_DIV_A::CLKO2_DIV_3,
            4 => CLKO2_DIV_A::CLKO2_DIV_4,
            5 => CLKO2_DIV_A::CLKO2_DIV_5,
            6 => CLKO2_DIV_A::CLKO2_DIV_6,
            7 => CLKO2_DIV_A::CLKO2_DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_0`"]
    #[inline(always)]
    pub fn is_clko2_div_0(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_1`"]
    #[inline(always)]
    pub fn is_clko2_div_1(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_2`"]
    #[inline(always)]
    pub fn is_clko2_div_2(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_3`"]
    #[inline(always)]
    pub fn is_clko2_div_3(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_4`"]
    #[inline(always)]
    pub fn is_clko2_div_4(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_5`"]
    #[inline(always)]
    pub fn is_clko2_div_5(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_6`"]
    #[inline(always)]
    pub fn is_clko2_div_6(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_6
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_7`"]
    #[inline(always)]
    pub fn is_clko2_div_7(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_7
    }
}
#[doc = "Write proxy for field `CLKO2_DIV`"]
pub struct CLKO2_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO2_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO2_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn clko2_div_0(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn clko2_div_1(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn clko2_div_2(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn clko2_div_3(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn clko2_div_4(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn clko2_div_5(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn clko2_div_6(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn clko2_div_7(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Enable of CCM_CLKO2 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO2_EN_A {
    #[doc = "0: CCM_CLKO2 disabled."]
    CLKO2_EN_0,
    #[doc = "1: CCM_CLKO2 enabled."]
    CLKO2_EN_1,
}
impl From<CLKO2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKO2_EN_A) -> Self {
        match variant {
            CLKO2_EN_A::CLKO2_EN_0 => false,
            CLKO2_EN_A::CLKO2_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `CLKO2_EN`"]
pub type CLKO2_EN_R = crate::R<bool, CLKO2_EN_A>;
impl CLKO2_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO2_EN_A {
        match self.bits {
            false => CLKO2_EN_A::CLKO2_EN_0,
            true => CLKO2_EN_A::CLKO2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_EN_0`"]
    #[inline(always)]
    pub fn is_clko2_en_0(&self) -> bool {
        *self == CLKO2_EN_A::CLKO2_EN_0
    }
    #[doc = "Checks if the value of the field is `CLKO2_EN_1`"]
    #[inline(always)]
    pub fn is_clko2_en_1(&self) -> bool {
        *self == CLKO2_EN_A::CLKO2_EN_1
    }
}
#[doc = "Write proxy for field `CLKO2_EN`"]
pub struct CLKO2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO2_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CCM_CLKO2 disabled."]
    #[inline(always)]
    pub fn clko2_en_0(self) -> &'a mut W {
        self.variant(CLKO2_EN_A::CLKO2_EN_0)
    }
    #[doc = "CCM_CLKO2 enabled."]
    #[inline(always)]
    pub fn clko2_en_1(self) -> &'a mut W {
        self.variant(CLKO2_EN_A::CLKO2_EN_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Selection of the clock to be generated on CCM_CLKO1"]
    #[inline(always)]
    pub fn clko1_sel(&self) -> CLKO1_SEL_R {
        CLKO1_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Setting the divider of CCM_CLKO1"]
    #[inline(always)]
    pub fn clko1_div(&self) -> CLKO1_DIV_R {
        CLKO1_DIV_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Enable of CCM_CLKO1 clock"]
    #[inline(always)]
    pub fn clko1_en(&self) -> CLKO1_EN_R {
        CLKO1_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline(always)]
    pub fn clk_out_sel(&self) -> CLK_OUT_SEL_R {
        CLK_OUT_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Selection of the clock to be generated on CCM_CLKO2"]
    #[inline(always)]
    pub fn clko2_sel(&self) -> CLKO2_SEL_R {
        CLKO2_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Setting the divider of CCM_CLKO2"]
    #[inline(always)]
    pub fn clko2_div(&self) -> CLKO2_DIV_R {
        CLKO2_DIV_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Enable of CCM_CLKO2 clock"]
    #[inline(always)]
    pub fn clko2_en(&self) -> CLKO2_EN_R {
        CLKO2_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selection of the clock to be generated on CCM_CLKO1"]
    #[inline(always)]
    pub fn clko1_sel(&mut self) -> CLKO1_SEL_W {
        CLKO1_SEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Setting the divider of CCM_CLKO1"]
    #[inline(always)]
    pub fn clko1_div(&mut self) -> CLKO1_DIV_W {
        CLKO1_DIV_W { w: self }
    }
    #[doc = "Bit 7 - Enable of CCM_CLKO1 clock"]
    #[inline(always)]
    pub fn clko1_en(&mut self) -> CLKO1_EN_W {
        CLKO1_EN_W { w: self }
    }
    #[doc = "Bit 8 - CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline(always)]
    pub fn clk_out_sel(&mut self) -> CLK_OUT_SEL_W {
        CLK_OUT_SEL_W { w: self }
    }
    #[doc = "Bits 16:20 - Selection of the clock to be generated on CCM_CLKO2"]
    #[inline(always)]
    pub fn clko2_sel(&mut self) -> CLKO2_SEL_W {
        CLKO2_SEL_W { w: self }
    }
    #[doc = "Bits 21:23 - Setting the divider of CCM_CLKO2"]
    #[inline(always)]
    pub fn clko2_div(&mut self) -> CLKO2_DIV_W {
        CLKO2_DIV_W { w: self }
    }
    #[doc = "Bit 24 - Enable of CCM_CLKO2 clock"]
    #[inline(always)]
    pub fn clko2_en(&mut self) -> CLKO2_EN_W {
        CLKO2_EN_W { w: self }
    }
}
