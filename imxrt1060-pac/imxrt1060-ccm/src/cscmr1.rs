#[doc = "Reader of register CSCMR1"]
pub type R = crate::R<u32, super::CSCMR1>;
#[doc = "Writer for register CSCMR1"]
pub type W = crate::W<u32, super::CSCMR1>;
#[doc = "Register CSCMR1 `reset()`'s with value 0x0490_0000"]
impl crate::ResetValue for super::CSCMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0490_0000
    }
}
#[doc = "Divider for perclk podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERCLK_PODF_A {
    #[doc = "0: Divide by 1"]
    DIVIDE_1,
    #[doc = "1: Divide by 2"]
    DIVIDE_2,
    #[doc = "2: Divide by 3"]
    DIVIDE_3,
    #[doc = "3: Divide by 4"]
    DIVIDE_4,
    #[doc = "4: Divide by 5"]
    DIVIDE_5,
    #[doc = "5: Divide by 6"]
    DIVIDE_6,
    #[doc = "6: Divide by 7"]
    DIVIDE_7,
    #[doc = "63: Divide by 64"]
    DIVIDE_64,
}
impl From<PERCLK_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: PERCLK_PODF_A) -> Self {
        match variant {
            PERCLK_PODF_A::DIVIDE_1 => 0b000000,
            PERCLK_PODF_A::DIVIDE_2 => 0b000001,
            PERCLK_PODF_A::DIVIDE_3 => 0b000010,
            PERCLK_PODF_A::DIVIDE_4 => 0b000011,
            PERCLK_PODF_A::DIVIDE_5 => 0b000100,
            PERCLK_PODF_A::DIVIDE_6 => 0b000101,
            PERCLK_PODF_A::DIVIDE_7 => 0b000110,
            PERCLK_PODF_A::DIVIDE_64 => 0b111111,
        }
    }
}
#[doc = "Reader of field `PERCLK_PODF`"]
pub type PERCLK_PODF_R = crate::R<u8, PERCLK_PODF_A>;
impl PERCLK_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERCLK_PODF_A {
        match self.bits {
            0b000000 => PERCLK_PODF_A::DIVIDE_1,
            0b000001 => PERCLK_PODF_A::DIVIDE_2,
            0b000010 => PERCLK_PODF_A::DIVIDE_3,
            0b000011 => PERCLK_PODF_A::DIVIDE_4,
            0b000100 => PERCLK_PODF_A::DIVIDE_5,
            0b000101 => PERCLK_PODF_A::DIVIDE_6,
            0b000110 => PERCLK_PODF_A::DIVIDE_7,
            0b111111 => PERCLK_PODF_A::DIVIDE_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == PERCLK_PODF_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == PERCLK_PODF_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == PERCLK_PODF_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == PERCLK_PODF_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == PERCLK_PODF_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == PERCLK_PODF_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == PERCLK_PODF_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_64`"]
    #[inline(always)]
    pub fn is_divide_64(&self) -> bool {
        *self == PERCLK_PODF_A::DIVIDE_64
    }
}
#[doc = "Write proxy for field `PERCLK_PODF`"]
pub struct PERCLK_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERCLK_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERCLK_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(PERCLK_PODF_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(PERCLK_PODF_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(PERCLK_PODF_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(PERCLK_PODF_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(PERCLK_PODF_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(PERCLK_PODF_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(PERCLK_PODF_A::DIVIDE_7)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn divide_64(self) -> &'a mut W {
        self.variant(PERCLK_PODF_A::DIVIDE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Selector for the perclk clock multiplexor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERCLK_CLK_SEL_A {
    #[doc = "0: derive clock from ipg clk root"]
    PERCLK_CLK_SEL_0,
    #[doc = "1: derive clock from osc_clk"]
    PERCLK_CLK_SEL_1,
}
impl From<PERCLK_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PERCLK_CLK_SEL_A) -> Self {
        match variant {
            PERCLK_CLK_SEL_A::PERCLK_CLK_SEL_0 => false,
            PERCLK_CLK_SEL_A::PERCLK_CLK_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `PERCLK_CLK_SEL`"]
pub type PERCLK_CLK_SEL_R = crate::R<bool, PERCLK_CLK_SEL_A>;
impl PERCLK_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERCLK_CLK_SEL_A {
        match self.bits {
            false => PERCLK_CLK_SEL_A::PERCLK_CLK_SEL_0,
            true => PERCLK_CLK_SEL_A::PERCLK_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERCLK_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_perclk_clk_sel_0(&self) -> bool {
        *self == PERCLK_CLK_SEL_A::PERCLK_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PERCLK_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_perclk_clk_sel_1(&self) -> bool {
        *self == PERCLK_CLK_SEL_A::PERCLK_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `PERCLK_CLK_SEL`"]
pub struct PERCLK_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERCLK_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERCLK_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "derive clock from ipg clk root"]
    #[inline(always)]
    pub fn perclk_clk_sel_0(self) -> &'a mut W {
        self.variant(PERCLK_CLK_SEL_A::PERCLK_CLK_SEL_0)
    }
    #[doc = "derive clock from osc_clk"]
    #[inline(always)]
    pub fn perclk_clk_sel_1(self) -> &'a mut W {
        self.variant(PERCLK_CLK_SEL_A::PERCLK_CLK_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Selector for sai1 clock multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_CLK_SEL_A {
    #[doc = "0: derive clock from PLL3 PFD2"]
    SAI1_CLK_SEL_0,
    #[doc = "1: derive clock from PLL5"]
    SAI1_CLK_SEL_1,
    #[doc = "2: derive clock from PLL4"]
    SAI1_CLK_SEL_2,
}
impl From<SAI1_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1_CLK_SEL_A) -> Self {
        match variant {
            SAI1_CLK_SEL_A::SAI1_CLK_SEL_0 => 0,
            SAI1_CLK_SEL_A::SAI1_CLK_SEL_1 => 1,
            SAI1_CLK_SEL_A::SAI1_CLK_SEL_2 => 2,
        }
    }
}
#[doc = "Reader of field `SAI1_CLK_SEL`"]
pub type SAI1_CLK_SEL_R = crate::R<u8, SAI1_CLK_SEL_A>;
impl SAI1_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI1_CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI1_CLK_SEL_A::SAI1_CLK_SEL_0),
            1 => Val(SAI1_CLK_SEL_A::SAI1_CLK_SEL_1),
            2 => Val(SAI1_CLK_SEL_A::SAI1_CLK_SEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_sai1_clk_sel_0(&self) -> bool {
        *self == SAI1_CLK_SEL_A::SAI1_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_sai1_clk_sel_1(&self) -> bool {
        *self == SAI1_CLK_SEL_A::SAI1_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_sai1_clk_sel_2(&self) -> bool {
        *self == SAI1_CLK_SEL_A::SAI1_CLK_SEL_2
    }
}
#[doc = "Write proxy for field `SAI1_CLK_SEL`"]
pub struct SAI1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1_CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline(always)]
    pub fn sai1_clk_sel_0(self) -> &'a mut W {
        self.variant(SAI1_CLK_SEL_A::SAI1_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL5"]
    #[inline(always)]
    pub fn sai1_clk_sel_1(self) -> &'a mut W {
        self.variant(SAI1_CLK_SEL_A::SAI1_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL4"]
    #[inline(always)]
    pub fn sai1_clk_sel_2(self) -> &'a mut W {
        self.variant(SAI1_CLK_SEL_A::SAI1_CLK_SEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Selector for sai2 clock multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_CLK_SEL_A {
    #[doc = "0: derive clock from PLL3 PFD2"]
    SAI2_CLK_SEL_0,
    #[doc = "1: derive clock from PLL5"]
    SAI2_CLK_SEL_1,
    #[doc = "2: derive clock from PLL4"]
    SAI2_CLK_SEL_2,
}
impl From<SAI2_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2_CLK_SEL_A) -> Self {
        match variant {
            SAI2_CLK_SEL_A::SAI2_CLK_SEL_0 => 0,
            SAI2_CLK_SEL_A::SAI2_CLK_SEL_1 => 1,
            SAI2_CLK_SEL_A::SAI2_CLK_SEL_2 => 2,
        }
    }
}
#[doc = "Reader of field `SAI2_CLK_SEL`"]
pub type SAI2_CLK_SEL_R = crate::R<u8, SAI2_CLK_SEL_A>;
impl SAI2_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI2_CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI2_CLK_SEL_A::SAI2_CLK_SEL_0),
            1 => Val(SAI2_CLK_SEL_A::SAI2_CLK_SEL_1),
            2 => Val(SAI2_CLK_SEL_A::SAI2_CLK_SEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_sai2_clk_sel_0(&self) -> bool {
        *self == SAI2_CLK_SEL_A::SAI2_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_sai2_clk_sel_1(&self) -> bool {
        *self == SAI2_CLK_SEL_A::SAI2_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_sai2_clk_sel_2(&self) -> bool {
        *self == SAI2_CLK_SEL_A::SAI2_CLK_SEL_2
    }
}
#[doc = "Write proxy for field `SAI2_CLK_SEL`"]
pub struct SAI2_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2_CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline(always)]
    pub fn sai2_clk_sel_0(self) -> &'a mut W {
        self.variant(SAI2_CLK_SEL_A::SAI2_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL5"]
    #[inline(always)]
    pub fn sai2_clk_sel_1(self) -> &'a mut W {
        self.variant(SAI2_CLK_SEL_A::SAI2_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL4"]
    #[inline(always)]
    pub fn sai2_clk_sel_2(self) -> &'a mut W {
        self.variant(SAI2_CLK_SEL_A::SAI2_CLK_SEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Selector for sai3/adc1/adc2 clock multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_CLK_SEL_A {
    #[doc = "0: derive clock from PLL3 PFD2"]
    SAI3_CLK_SEL_0,
    #[doc = "1: derive clock from PLL5"]
    SAI3_CLK_SEL_1,
    #[doc = "2: derive clock from PLL4"]
    SAI3_CLK_SEL_2,
}
impl From<SAI3_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI3_CLK_SEL_A) -> Self {
        match variant {
            SAI3_CLK_SEL_A::SAI3_CLK_SEL_0 => 0,
            SAI3_CLK_SEL_A::SAI3_CLK_SEL_1 => 1,
            SAI3_CLK_SEL_A::SAI3_CLK_SEL_2 => 2,
        }
    }
}
#[doc = "Reader of field `SAI3_CLK_SEL`"]
pub type SAI3_CLK_SEL_R = crate::R<u8, SAI3_CLK_SEL_A>;
impl SAI3_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI3_CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI3_CLK_SEL_A::SAI3_CLK_SEL_0),
            1 => Val(SAI3_CLK_SEL_A::SAI3_CLK_SEL_1),
            2 => Val(SAI3_CLK_SEL_A::SAI3_CLK_SEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_sai3_clk_sel_0(&self) -> bool {
        *self == SAI3_CLK_SEL_A::SAI3_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_sai3_clk_sel_1(&self) -> bool {
        *self == SAI3_CLK_SEL_A::SAI3_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI3_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_sai3_clk_sel_2(&self) -> bool {
        *self == SAI3_CLK_SEL_A::SAI3_CLK_SEL_2
    }
}
#[doc = "Write proxy for field `SAI3_CLK_SEL`"]
pub struct SAI3_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI3_CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline(always)]
    pub fn sai3_clk_sel_0(self) -> &'a mut W {
        self.variant(SAI3_CLK_SEL_A::SAI3_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL5"]
    #[inline(always)]
    pub fn sai3_clk_sel_1(self) -> &'a mut W {
        self.variant(SAI3_CLK_SEL_A::SAI3_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL4"]
    #[inline(always)]
    pub fn sai3_clk_sel_2(self) -> &'a mut W {
        self.variant(SAI3_CLK_SEL_A::SAI3_CLK_SEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Selector for usdhc1 clock multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USDHC1_CLK_SEL_A {
    #[doc = "0: derive clock from PLL2 PFD2"]
    USDHC1_CLK_SEL_0,
    #[doc = "1: derive clock from PLL2 PFD0"]
    USDHC1_CLK_SEL_1,
}
impl From<USDHC1_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: USDHC1_CLK_SEL_A) -> Self {
        match variant {
            USDHC1_CLK_SEL_A::USDHC1_CLK_SEL_0 => false,
            USDHC1_CLK_SEL_A::USDHC1_CLK_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `USDHC1_CLK_SEL`"]
pub type USDHC1_CLK_SEL_R = crate::R<bool, USDHC1_CLK_SEL_A>;
impl USDHC1_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USDHC1_CLK_SEL_A {
        match self.bits {
            false => USDHC1_CLK_SEL_A::USDHC1_CLK_SEL_0,
            true => USDHC1_CLK_SEL_A::USDHC1_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `USDHC1_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_usdhc1_clk_sel_0(&self) -> bool {
        *self == USDHC1_CLK_SEL_A::USDHC1_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `USDHC1_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_usdhc1_clk_sel_1(&self) -> bool {
        *self == USDHC1_CLK_SEL_A::USDHC1_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `USDHC1_CLK_SEL`"]
pub struct USDHC1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USDHC1_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USDHC1_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn usdhc1_clk_sel_0(self) -> &'a mut W {
        self.variant(USDHC1_CLK_SEL_A::USDHC1_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline(always)]
    pub fn usdhc1_clk_sel_1(self) -> &'a mut W {
        self.variant(USDHC1_CLK_SEL_A::USDHC1_CLK_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Selector for usdhc2 clock multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USDHC2_CLK_SEL_A {
    #[doc = "0: derive clock from PLL2 PFD2"]
    USDHC2_CLK_SEL_0,
    #[doc = "1: derive clock from PLL2 PFD0"]
    USDHC2_CLK_SEL_1,
}
impl From<USDHC2_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: USDHC2_CLK_SEL_A) -> Self {
        match variant {
            USDHC2_CLK_SEL_A::USDHC2_CLK_SEL_0 => false,
            USDHC2_CLK_SEL_A::USDHC2_CLK_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `USDHC2_CLK_SEL`"]
pub type USDHC2_CLK_SEL_R = crate::R<bool, USDHC2_CLK_SEL_A>;
impl USDHC2_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USDHC2_CLK_SEL_A {
        match self.bits {
            false => USDHC2_CLK_SEL_A::USDHC2_CLK_SEL_0,
            true => USDHC2_CLK_SEL_A::USDHC2_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `USDHC2_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_usdhc2_clk_sel_0(&self) -> bool {
        *self == USDHC2_CLK_SEL_A::USDHC2_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `USDHC2_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_usdhc2_clk_sel_1(&self) -> bool {
        *self == USDHC2_CLK_SEL_A::USDHC2_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `USDHC2_CLK_SEL`"]
pub struct USDHC2_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USDHC2_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USDHC2_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn usdhc2_clk_sel_0(self) -> &'a mut W {
        self.variant(USDHC2_CLK_SEL_A::USDHC2_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline(always)]
    pub fn usdhc2_clk_sel_1(self) -> &'a mut W {
        self.variant(USDHC2_CLK_SEL_A::USDHC2_CLK_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Divider for flexspi clock root.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI_PODF_A {
    #[doc = "0: divide by 1"]
    FLEXSPI_PODF_0,
    #[doc = "1: divide by 2"]
    FLEXSPI_PODF_1,
    #[doc = "2: divide by 3"]
    FLEXSPI_PODF_2,
    #[doc = "3: divide by 4"]
    FLEXSPI_PODF_3,
    #[doc = "4: divide by 5"]
    FLEXSPI_PODF_4,
    #[doc = "5: divide by 6"]
    FLEXSPI_PODF_5,
    #[doc = "6: divide by 7"]
    FLEXSPI_PODF_6,
    #[doc = "7: divide by 8"]
    FLEXSPI_PODF_7,
}
impl From<FLEXSPI_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXSPI_PODF_A) -> Self {
        match variant {
            FLEXSPI_PODF_A::FLEXSPI_PODF_0 => 0,
            FLEXSPI_PODF_A::FLEXSPI_PODF_1 => 1,
            FLEXSPI_PODF_A::FLEXSPI_PODF_2 => 2,
            FLEXSPI_PODF_A::FLEXSPI_PODF_3 => 3,
            FLEXSPI_PODF_A::FLEXSPI_PODF_4 => 4,
            FLEXSPI_PODF_A::FLEXSPI_PODF_5 => 5,
            FLEXSPI_PODF_A::FLEXSPI_PODF_6 => 6,
            FLEXSPI_PODF_A::FLEXSPI_PODF_7 => 7,
        }
    }
}
#[doc = "Reader of field `FLEXSPI_PODF`"]
pub type FLEXSPI_PODF_R = crate::R<u8, FLEXSPI_PODF_A>;
impl FLEXSPI_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI_PODF_A {
        match self.bits {
            0 => FLEXSPI_PODF_A::FLEXSPI_PODF_0,
            1 => FLEXSPI_PODF_A::FLEXSPI_PODF_1,
            2 => FLEXSPI_PODF_A::FLEXSPI_PODF_2,
            3 => FLEXSPI_PODF_A::FLEXSPI_PODF_3,
            4 => FLEXSPI_PODF_A::FLEXSPI_PODF_4,
            5 => FLEXSPI_PODF_A::FLEXSPI_PODF_5,
            6 => FLEXSPI_PODF_A::FLEXSPI_PODF_6,
            7 => FLEXSPI_PODF_A::FLEXSPI_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_0`"]
    #[inline(always)]
    pub fn is_flexspi_podf_0(&self) -> bool {
        *self == FLEXSPI_PODF_A::FLEXSPI_PODF_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_1`"]
    #[inline(always)]
    pub fn is_flexspi_podf_1(&self) -> bool {
        *self == FLEXSPI_PODF_A::FLEXSPI_PODF_1
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_2`"]
    #[inline(always)]
    pub fn is_flexspi_podf_2(&self) -> bool {
        *self == FLEXSPI_PODF_A::FLEXSPI_PODF_2
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_3`"]
    #[inline(always)]
    pub fn is_flexspi_podf_3(&self) -> bool {
        *self == FLEXSPI_PODF_A::FLEXSPI_PODF_3
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_4`"]
    #[inline(always)]
    pub fn is_flexspi_podf_4(&self) -> bool {
        *self == FLEXSPI_PODF_A::FLEXSPI_PODF_4
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_5`"]
    #[inline(always)]
    pub fn is_flexspi_podf_5(&self) -> bool {
        *self == FLEXSPI_PODF_A::FLEXSPI_PODF_5
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_6`"]
    #[inline(always)]
    pub fn is_flexspi_podf_6(&self) -> bool {
        *self == FLEXSPI_PODF_A::FLEXSPI_PODF_6
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_PODF_7`"]
    #[inline(always)]
    pub fn is_flexspi_podf_7(&self) -> bool {
        *self == FLEXSPI_PODF_A::FLEXSPI_PODF_7
    }
}
#[doc = "Write proxy for field `FLEXSPI_PODF`"]
pub struct FLEXSPI_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXSPI_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXSPI_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn flexspi_podf_0(self) -> &'a mut W {
        self.variant(FLEXSPI_PODF_A::FLEXSPI_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn flexspi_podf_1(self) -> &'a mut W {
        self.variant(FLEXSPI_PODF_A::FLEXSPI_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn flexspi_podf_2(self) -> &'a mut W {
        self.variant(FLEXSPI_PODF_A::FLEXSPI_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn flexspi_podf_3(self) -> &'a mut W {
        self.variant(FLEXSPI_PODF_A::FLEXSPI_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn flexspi_podf_4(self) -> &'a mut W {
        self.variant(FLEXSPI_PODF_A::FLEXSPI_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn flexspi_podf_5(self) -> &'a mut W {
        self.variant(FLEXSPI_PODF_A::FLEXSPI_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn flexspi_podf_6(self) -> &'a mut W {
        self.variant(FLEXSPI_PODF_A::FLEXSPI_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn flexspi_podf_7(self) -> &'a mut W {
        self.variant(FLEXSPI_PODF_A::FLEXSPI_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Selector for flexspi clock multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI_CLK_SEL_A {
    #[doc = "0: derive clock from semc_clk_root_pre"]
    FLEXSPI_CLK_SEL_0,
    #[doc = "1: derive clock from pll3_sw_clk"]
    FLEXSPI_CLK_SEL_1,
    #[doc = "2: derive clock from PLL2 PFD2"]
    FLEXSPI_CLK_SEL_2,
    #[doc = "3: derive clock from PLL3 PFD0"]
    FLEXSPI_CLK_SEL_3,
}
impl From<FLEXSPI_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXSPI_CLK_SEL_A) -> Self {
        match variant {
            FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_0 => 0,
            FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_1 => 1,
            FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_2 => 2,
            FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `FLEXSPI_CLK_SEL`"]
pub type FLEXSPI_CLK_SEL_R = crate::R<u8, FLEXSPI_CLK_SEL_A>;
impl FLEXSPI_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI_CLK_SEL_A {
        match self.bits {
            0 => FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_0,
            1 => FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_1,
            2 => FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_2,
            3 => FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_flexspi_clk_sel_0(&self) -> bool {
        *self == FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_flexspi_clk_sel_1(&self) -> bool {
        *self == FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_flexspi_clk_sel_2(&self) -> bool {
        *self == FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_flexspi_clk_sel_3(&self) -> bool {
        *self == FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_3
    }
}
#[doc = "Write proxy for field `FLEXSPI_CLK_SEL`"]
pub struct FLEXSPI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXSPI_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXSPI_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "derive clock from semc_clk_root_pre"]
    #[inline(always)]
    pub fn flexspi_clk_sel_0(self) -> &'a mut W {
        self.variant(FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_0)
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline(always)]
    pub fn flexspi_clk_sel_1(self) -> &'a mut W {
        self.variant(FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn flexspi_clk_sel_2(self) -> &'a mut W {
        self.variant(FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL3 PFD0"]
    #[inline(always)]
    pub fn flexspi_clk_sel_3(self) -> &'a mut W {
        self.variant(FLEXSPI_CLK_SEL_A::FLEXSPI_CLK_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Divider for perclk podf."]
    #[inline(always)]
    pub fn perclk_podf(&self) -> PERCLK_PODF_R {
        PERCLK_PODF_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Selector for the perclk clock multiplexor"]
    #[inline(always)]
    pub fn perclk_clk_sel(&self) -> PERCLK_CLK_SEL_R {
        PERCLK_CLK_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Selector for sai1 clock multiplexer"]
    #[inline(always)]
    pub fn sai1_clk_sel(&self) -> SAI1_CLK_SEL_R {
        SAI1_CLK_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Selector for sai2 clock multiplexer"]
    #[inline(always)]
    pub fn sai2_clk_sel(&self) -> SAI2_CLK_SEL_R {
        SAI2_CLK_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Selector for sai3/adc1/adc2 clock multiplexer"]
    #[inline(always)]
    pub fn sai3_clk_sel(&self) -> SAI3_CLK_SEL_R {
        SAI3_CLK_SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Selector for usdhc1 clock multiplexer"]
    #[inline(always)]
    pub fn usdhc1_clk_sel(&self) -> USDHC1_CLK_SEL_R {
        USDHC1_CLK_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Selector for usdhc2 clock multiplexer"]
    #[inline(always)]
    pub fn usdhc2_clk_sel(&self) -> USDHC2_CLK_SEL_R {
        USDHC2_CLK_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 23:25 - Divider for flexspi clock root."]
    #[inline(always)]
    pub fn flexspi_podf(&self) -> FLEXSPI_PODF_R {
        FLEXSPI_PODF_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 29:30 - Selector for flexspi clock multiplexer"]
    #[inline(always)]
    pub fn flexspi_clk_sel(&self) -> FLEXSPI_CLK_SEL_R {
        FLEXSPI_CLK_SEL_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Divider for perclk podf."]
    #[inline(always)]
    pub fn perclk_podf(&mut self) -> PERCLK_PODF_W {
        PERCLK_PODF_W { w: self }
    }
    #[doc = "Bit 6 - Selector for the perclk clock multiplexor"]
    #[inline(always)]
    pub fn perclk_clk_sel(&mut self) -> PERCLK_CLK_SEL_W {
        PERCLK_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - Selector for sai1 clock multiplexer"]
    #[inline(always)]
    pub fn sai1_clk_sel(&mut self) -> SAI1_CLK_SEL_W {
        SAI1_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - Selector for sai2 clock multiplexer"]
    #[inline(always)]
    pub fn sai2_clk_sel(&mut self) -> SAI2_CLK_SEL_W {
        SAI2_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Selector for sai3/adc1/adc2 clock multiplexer"]
    #[inline(always)]
    pub fn sai3_clk_sel(&mut self) -> SAI3_CLK_SEL_W {
        SAI3_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 16 - Selector for usdhc1 clock multiplexer"]
    #[inline(always)]
    pub fn usdhc1_clk_sel(&mut self) -> USDHC1_CLK_SEL_W {
        USDHC1_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 17 - Selector for usdhc2 clock multiplexer"]
    #[inline(always)]
    pub fn usdhc2_clk_sel(&mut self) -> USDHC2_CLK_SEL_W {
        USDHC2_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 23:25 - Divider for flexspi clock root."]
    #[inline(always)]
    pub fn flexspi_podf(&mut self) -> FLEXSPI_PODF_W {
        FLEXSPI_PODF_W { w: self }
    }
    #[doc = "Bits 29:30 - Selector for flexspi clock multiplexer"]
    #[inline(always)]
    pub fn flexspi_clk_sel(&mut self) -> FLEXSPI_CLK_SEL_W {
        FLEXSPI_CLK_SEL_W { w: self }
    }
}
