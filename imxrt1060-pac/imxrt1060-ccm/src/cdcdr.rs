#[doc = "Reader of register CDCDR"]
pub type R = crate::R<u32, super::CDCDR>;
#[doc = "Writer for register CDCDR"]
pub type W = crate::W<u32, super::CDCDR>;
#[doc = "Register CDCDR `reset()`'s with value 0x33f7_1f92"]
impl crate::ResetValue for super::CDCDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x33f7_1f92
    }
}
#[doc = "Selector for flexio1 clock multiplexer\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_CLK_SEL_A {
    #[doc = "0: derive clock from PLL4"]
    FLEXIO1_CLK_SEL_0,
    #[doc = "1: derive clock from PLL3 PFD2"]
    FLEXIO1_CLK_SEL_1,
    #[doc = "2: derive clock from PLL5"]
    FLEXIO1_CLK_SEL_2,
    #[doc = "3: derive clock from pll3_sw_clk"]
    FLEXIO1_CLK_SEL_3,
}
impl From<FLEXIO1_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXIO1_CLK_SEL_A) -> Self {
        match variant {
            FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_0 => 0,
            FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_1 => 1,
            FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_2 => 2,
            FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `FLEXIO1_CLK_SEL`"]
pub type FLEXIO1_CLK_SEL_R = crate::R<u8, FLEXIO1_CLK_SEL_A>;
impl FLEXIO1_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_CLK_SEL_A {
        match self.bits {
            0 => FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_0,
            1 => FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_1,
            2 => FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_2,
            3 => FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_flexio1_clk_sel_0(&self) -> bool {
        *self == FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_flexio1_clk_sel_1(&self) -> bool {
        *self == FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_flexio1_clk_sel_2(&self) -> bool {
        *self == FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_flexio1_clk_sel_3(&self) -> bool {
        *self == FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_3
    }
}
#[doc = "Write proxy for field `FLEXIO1_CLK_SEL`"]
pub struct FLEXIO1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO1_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO1_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "derive clock from PLL4"]
    #[inline(always)]
    pub fn flexio1_clk_sel_0(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline(always)]
    pub fn flexio1_clk_sel_1(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL5"]
    #[inline(always)]
    pub fn flexio1_clk_sel_2(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_2)
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline(always)]
    pub fn flexio1_clk_sel_3(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_SEL_A::FLEXIO1_CLK_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Divider for flexio1 clock podf. Divider should be updated when output clock is gated.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_CLK_PODF_A {
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
    #[doc = "7: Divide by 8"]
    DIVIDE_8,
}
impl From<FLEXIO1_CLK_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXIO1_CLK_PODF_A) -> Self {
        match variant {
            FLEXIO1_CLK_PODF_A::DIVIDE_1 => 0,
            FLEXIO1_CLK_PODF_A::DIVIDE_2 => 1,
            FLEXIO1_CLK_PODF_A::DIVIDE_3 => 2,
            FLEXIO1_CLK_PODF_A::DIVIDE_4 => 3,
            FLEXIO1_CLK_PODF_A::DIVIDE_5 => 4,
            FLEXIO1_CLK_PODF_A::DIVIDE_6 => 5,
            FLEXIO1_CLK_PODF_A::DIVIDE_7 => 6,
            FLEXIO1_CLK_PODF_A::DIVIDE_8 => 7,
        }
    }
}
#[doc = "Reader of field `FLEXIO1_CLK_PODF`"]
pub type FLEXIO1_CLK_PODF_R = crate::R<u8, FLEXIO1_CLK_PODF_A>;
impl FLEXIO1_CLK_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_CLK_PODF_A {
        match self.bits {
            0 => FLEXIO1_CLK_PODF_A::DIVIDE_1,
            1 => FLEXIO1_CLK_PODF_A::DIVIDE_2,
            2 => FLEXIO1_CLK_PODF_A::DIVIDE_3,
            3 => FLEXIO1_CLK_PODF_A::DIVIDE_4,
            4 => FLEXIO1_CLK_PODF_A::DIVIDE_5,
            5 => FLEXIO1_CLK_PODF_A::DIVIDE_6,
            6 => FLEXIO1_CLK_PODF_A::DIVIDE_7,
            7 => FLEXIO1_CLK_PODF_A::DIVIDE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == FLEXIO1_CLK_PODF_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == FLEXIO1_CLK_PODF_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == FLEXIO1_CLK_PODF_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == FLEXIO1_CLK_PODF_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == FLEXIO1_CLK_PODF_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == FLEXIO1_CLK_PODF_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == FLEXIO1_CLK_PODF_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == FLEXIO1_CLK_PODF_A::DIVIDE_8
    }
}
#[doc = "Write proxy for field `FLEXIO1_CLK_PODF`"]
pub struct FLEXIO1_CLK_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO1_CLK_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO1_CLK_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODF_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODF_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODF_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODF_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODF_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODF_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODF_A::DIVIDE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PODF_A::DIVIDE_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Divider for flexio1 clock pred. Divider should be updated when output clock is gated.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_CLK_PRED_A {
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
    #[doc = "7: Divide by 8"]
    DIVIDE_8,
}
impl From<FLEXIO1_CLK_PRED_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXIO1_CLK_PRED_A) -> Self {
        match variant {
            FLEXIO1_CLK_PRED_A::DIVIDE_1 => 0,
            FLEXIO1_CLK_PRED_A::DIVIDE_2 => 1,
            FLEXIO1_CLK_PRED_A::DIVIDE_3 => 2,
            FLEXIO1_CLK_PRED_A::DIVIDE_4 => 3,
            FLEXIO1_CLK_PRED_A::DIVIDE_5 => 4,
            FLEXIO1_CLK_PRED_A::DIVIDE_6 => 5,
            FLEXIO1_CLK_PRED_A::DIVIDE_7 => 6,
            FLEXIO1_CLK_PRED_A::DIVIDE_8 => 7,
        }
    }
}
#[doc = "Reader of field `FLEXIO1_CLK_PRED`"]
pub type FLEXIO1_CLK_PRED_R = crate::R<u8, FLEXIO1_CLK_PRED_A>;
impl FLEXIO1_CLK_PRED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_CLK_PRED_A {
        match self.bits {
            0 => FLEXIO1_CLK_PRED_A::DIVIDE_1,
            1 => FLEXIO1_CLK_PRED_A::DIVIDE_2,
            2 => FLEXIO1_CLK_PRED_A::DIVIDE_3,
            3 => FLEXIO1_CLK_PRED_A::DIVIDE_4,
            4 => FLEXIO1_CLK_PRED_A::DIVIDE_5,
            5 => FLEXIO1_CLK_PRED_A::DIVIDE_6,
            6 => FLEXIO1_CLK_PRED_A::DIVIDE_7,
            7 => FLEXIO1_CLK_PRED_A::DIVIDE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == FLEXIO1_CLK_PRED_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == FLEXIO1_CLK_PRED_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == FLEXIO1_CLK_PRED_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == FLEXIO1_CLK_PRED_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == FLEXIO1_CLK_PRED_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == FLEXIO1_CLK_PRED_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == FLEXIO1_CLK_PRED_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == FLEXIO1_CLK_PRED_A::DIVIDE_8
    }
}
#[doc = "Write proxy for field `FLEXIO1_CLK_PRED`"]
pub struct FLEXIO1_CLK_PRED_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO1_CLK_PRED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO1_CLK_PRED_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PRED_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PRED_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PRED_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PRED_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PRED_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PRED_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PRED_A::DIVIDE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(FLEXIO1_CLK_PRED_A::DIVIDE_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Selector for spdif0 clock multiplexer\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIF0_CLK_SEL_A {
    #[doc = "0: derive clock from PLL4"]
    SPDIF0_CLK_SEL_0,
    #[doc = "1: derive clock from PLL3 PFD2"]
    SPDIF0_CLK_SEL_1,
    #[doc = "2: derive clock from PLL5"]
    SPDIF0_CLK_SEL_2,
    #[doc = "3: derive clock from pll3_sw_clk"]
    SPDIF0_CLK_SEL_3,
}
impl From<SPDIF0_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIF0_CLK_SEL_A) -> Self {
        match variant {
            SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_0 => 0,
            SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_1 => 1,
            SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_2 => 2,
            SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `SPDIF0_CLK_SEL`"]
pub type SPDIF0_CLK_SEL_R = crate::R<u8, SPDIF0_CLK_SEL_A>;
impl SPDIF0_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIF0_CLK_SEL_A {
        match self.bits {
            0 => SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_0,
            1 => SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_1,
            2 => SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_2,
            3 => SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_spdif0_clk_sel_0(&self) -> bool {
        *self == SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_spdif0_clk_sel_1(&self) -> bool {
        *self == SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_spdif0_clk_sel_2(&self) -> bool {
        *self == SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_spdif0_clk_sel_3(&self) -> bool {
        *self == SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_3
    }
}
#[doc = "Write proxy for field `SPDIF0_CLK_SEL`"]
pub struct SPDIF0_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIF0_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIF0_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "derive clock from PLL4"]
    #[inline(always)]
    pub fn spdif0_clk_sel_0(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline(always)]
    pub fn spdif0_clk_sel_1(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL5"]
    #[inline(always)]
    pub fn spdif0_clk_sel_2(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_2)
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline(always)]
    pub fn spdif0_clk_sel_3(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Divider for spdif0 clock podf. Divider should be updated when output clock is gated.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIF0_CLK_PODF_A {
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
    #[doc = "7: Divide by 8"]
    DIVIDE_8,
}
impl From<SPDIF0_CLK_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIF0_CLK_PODF_A) -> Self {
        match variant {
            SPDIF0_CLK_PODF_A::DIVIDE_1 => 0,
            SPDIF0_CLK_PODF_A::DIVIDE_2 => 1,
            SPDIF0_CLK_PODF_A::DIVIDE_3 => 2,
            SPDIF0_CLK_PODF_A::DIVIDE_4 => 3,
            SPDIF0_CLK_PODF_A::DIVIDE_5 => 4,
            SPDIF0_CLK_PODF_A::DIVIDE_6 => 5,
            SPDIF0_CLK_PODF_A::DIVIDE_7 => 6,
            SPDIF0_CLK_PODF_A::DIVIDE_8 => 7,
        }
    }
}
#[doc = "Reader of field `SPDIF0_CLK_PODF`"]
pub type SPDIF0_CLK_PODF_R = crate::R<u8, SPDIF0_CLK_PODF_A>;
impl SPDIF0_CLK_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIF0_CLK_PODF_A {
        match self.bits {
            0 => SPDIF0_CLK_PODF_A::DIVIDE_1,
            1 => SPDIF0_CLK_PODF_A::DIVIDE_2,
            2 => SPDIF0_CLK_PODF_A::DIVIDE_3,
            3 => SPDIF0_CLK_PODF_A::DIVIDE_4,
            4 => SPDIF0_CLK_PODF_A::DIVIDE_5,
            5 => SPDIF0_CLK_PODF_A::DIVIDE_6,
            6 => SPDIF0_CLK_PODF_A::DIVIDE_7,
            7 => SPDIF0_CLK_PODF_A::DIVIDE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_8
    }
}
#[doc = "Write proxy for field `SPDIF0_CLK_PODF`"]
pub struct SPDIF0_CLK_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIF0_CLK_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIF0_CLK_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Divider for spdif0 clock pred. Divider should be updated when output clock is gated.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIF0_CLK_PRED_A {
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
    #[doc = "7: Divide by 8"]
    DIVIDE_8,
}
impl From<SPDIF0_CLK_PRED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIF0_CLK_PRED_A) -> Self {
        match variant {
            SPDIF0_CLK_PRED_A::DIVIDE_1 => 0,
            SPDIF0_CLK_PRED_A::DIVIDE_2 => 1,
            SPDIF0_CLK_PRED_A::DIVIDE_3 => 2,
            SPDIF0_CLK_PRED_A::DIVIDE_4 => 3,
            SPDIF0_CLK_PRED_A::DIVIDE_5 => 4,
            SPDIF0_CLK_PRED_A::DIVIDE_6 => 5,
            SPDIF0_CLK_PRED_A::DIVIDE_7 => 6,
            SPDIF0_CLK_PRED_A::DIVIDE_8 => 7,
        }
    }
}
#[doc = "Reader of field `SPDIF0_CLK_PRED`"]
pub type SPDIF0_CLK_PRED_R = crate::R<u8, SPDIF0_CLK_PRED_A>;
impl SPDIF0_CLK_PRED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIF0_CLK_PRED_A {
        match self.bits {
            0 => SPDIF0_CLK_PRED_A::DIVIDE_1,
            1 => SPDIF0_CLK_PRED_A::DIVIDE_2,
            2 => SPDIF0_CLK_PRED_A::DIVIDE_3,
            3 => SPDIF0_CLK_PRED_A::DIVIDE_4,
            4 => SPDIF0_CLK_PRED_A::DIVIDE_5,
            5 => SPDIF0_CLK_PRED_A::DIVIDE_6,
            6 => SPDIF0_CLK_PRED_A::DIVIDE_7,
            7 => SPDIF0_CLK_PRED_A::DIVIDE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_8
    }
}
#[doc = "Write proxy for field `SPDIF0_CLK_PRED`"]
pub struct SPDIF0_CLK_PRED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIF0_CLK_PRED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIF0_CLK_PRED_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:8 - Selector for flexio1 clock multiplexer"]
    #[inline(always)]
    pub fn flexio1_clk_sel(&self) -> FLEXIO1_CLK_SEL_R {
        FLEXIO1_CLK_SEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:11 - Divider for flexio1 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn flexio1_clk_podf(&self) -> FLEXIO1_CLK_PODF_R {
        FLEXIO1_CLK_PODF_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Divider for flexio1 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn flexio1_clk_pred(&self) -> FLEXIO1_CLK_PRED_R {
        FLEXIO1_CLK_PRED_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 20:21 - Selector for spdif0 clock multiplexer"]
    #[inline(always)]
    pub fn spdif0_clk_sel(&self) -> SPDIF0_CLK_SEL_R {
        SPDIF0_CLK_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:24 - Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn spdif0_clk_podf(&self) -> SPDIF0_CLK_PODF_R {
        SPDIF0_CLK_PODF_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn spdif0_clk_pred(&self) -> SPDIF0_CLK_PRED_R {
        SPDIF0_CLK_PRED_R::new(((self.bits >> 25) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 7:8 - Selector for flexio1 clock multiplexer"]
    #[inline(always)]
    pub fn flexio1_clk_sel(&mut self) -> FLEXIO1_CLK_SEL_W {
        FLEXIO1_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 9:11 - Divider for flexio1 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn flexio1_clk_podf(&mut self) -> FLEXIO1_CLK_PODF_W {
        FLEXIO1_CLK_PODF_W { w: self }
    }
    #[doc = "Bits 12:14 - Divider for flexio1 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn flexio1_clk_pred(&mut self) -> FLEXIO1_CLK_PRED_W {
        FLEXIO1_CLK_PRED_W { w: self }
    }
    #[doc = "Bits 20:21 - Selector for spdif0 clock multiplexer"]
    #[inline(always)]
    pub fn spdif0_clk_sel(&mut self) -> SPDIF0_CLK_SEL_W {
        SPDIF0_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 22:24 - Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn spdif0_clk_podf(&mut self) -> SPDIF0_CLK_PODF_W {
        SPDIF0_CLK_PODF_W { w: self }
    }
    #[doc = "Bits 25:27 - Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn spdif0_clk_pred(&mut self) -> SPDIF0_CLK_PRED_W {
        SPDIF0_CLK_PRED_W { w: self }
    }
}
