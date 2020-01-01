#[doc = "Reader of register SRPC"]
pub type R = crate::R<u32, super::SRPC>;
#[doc = "Writer for register SRPC"]
pub type W = crate::W<u32, super::SRPC>;
#[doc = "Register SRPC `reset()`'s with value 0"]
impl crate::ResetValue for super::SRPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Gain selection:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINSEL_A {
    #[doc = "0: 24*(2**10)"]
    GAINSEL_0 = 0,
    #[doc = "1: 16*(2**10)"]
    GAINSEL_1 = 1,
    #[doc = "2: 12*(2**10)"]
    GAINSEL_2 = 2,
    #[doc = "3: 8*(2**10)"]
    GAINSEL_3 = 3,
    #[doc = "4: 6*(2**10)"]
    GAINSEL_4 = 4,
    #[doc = "5: 4*(2**10)"]
    GAINSEL_5 = 5,
    #[doc = "6: 3*(2**10)"]
    GAINSEL_6 = 6,
}
impl From<GAINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GainSel`"]
pub type GAINSEL_R = crate::R<u8, GAINSEL_A>;
impl GAINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAINSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAINSEL_A::GAINSEL_0),
            1 => Val(GAINSEL_A::GAINSEL_1),
            2 => Val(GAINSEL_A::GAINSEL_2),
            3 => Val(GAINSEL_A::GAINSEL_3),
            4 => Val(GAINSEL_A::GAINSEL_4),
            5 => Val(GAINSEL_A::GAINSEL_5),
            6 => Val(GAINSEL_A::GAINSEL_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GAINSEL_0`"]
    #[inline(always)]
    pub fn is_gain_sel_0(&self) -> bool {
        *self == GAINSEL_A::GAINSEL_0
    }
    #[doc = "Checks if the value of the field is `GAINSEL_1`"]
    #[inline(always)]
    pub fn is_gain_sel_1(&self) -> bool {
        *self == GAINSEL_A::GAINSEL_1
    }
    #[doc = "Checks if the value of the field is `GAINSEL_2`"]
    #[inline(always)]
    pub fn is_gain_sel_2(&self) -> bool {
        *self == GAINSEL_A::GAINSEL_2
    }
    #[doc = "Checks if the value of the field is `GAINSEL_3`"]
    #[inline(always)]
    pub fn is_gain_sel_3(&self) -> bool {
        *self == GAINSEL_A::GAINSEL_3
    }
    #[doc = "Checks if the value of the field is `GAINSEL_4`"]
    #[inline(always)]
    pub fn is_gain_sel_4(&self) -> bool {
        *self == GAINSEL_A::GAINSEL_4
    }
    #[doc = "Checks if the value of the field is `GAINSEL_5`"]
    #[inline(always)]
    pub fn is_gain_sel_5(&self) -> bool {
        *self == GAINSEL_A::GAINSEL_5
    }
    #[doc = "Checks if the value of the field is `GAINSEL_6`"]
    #[inline(always)]
    pub fn is_gain_sel_6(&self) -> bool {
        *self == GAINSEL_A::GAINSEL_6
    }
}
#[doc = "Write proxy for field `GainSel`"]
pub struct GAINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAINSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "24*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_0(self) -> &'a mut W {
        self.variant(GAINSEL_A::GAINSEL_0)
    }
    #[doc = "16*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_1(self) -> &'a mut W {
        self.variant(GAINSEL_A::GAINSEL_1)
    }
    #[doc = "12*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_2(self) -> &'a mut W {
        self.variant(GAINSEL_A::GAINSEL_2)
    }
    #[doc = "8*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_3(self) -> &'a mut W {
        self.variant(GAINSEL_A::GAINSEL_3)
    }
    #[doc = "6*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_4(self) -> &'a mut W {
        self.variant(GAINSEL_A::GAINSEL_4)
    }
    #[doc = "4*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_5(self) -> &'a mut W {
        self.variant(GAINSEL_A::GAINSEL_5)
    }
    #[doc = "3*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_6(self) -> &'a mut W {
        self.variant(GAINSEL_A::GAINSEL_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Clock source selection, all other settings not shown are reserved:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSRC_SEL_A {
    #[doc = "0: if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    CLKSRC_SEL_0 = 0,
    #[doc = "1: if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    CLKSRC_SEL_1 = 1,
    #[doc = "3: if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    CLKSRC_SEL_3 = 3,
    #[doc = "5: REF_CLK_32K (XTALOSC)"]
    CLKSRC_SEL_5 = 5,
    #[doc = "6: tx_clk (SPDIF0_CLK_ROOT)"]
    CLKSRC_SEL_6 = 6,
    #[doc = "8: SPDIF_EXT_CLK"]
    CLKSRC_SEL_8 = 8,
}
impl From<CLKSRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ClkSrc_Sel`"]
pub type CLKSRC_SEL_R = crate::R<u8, CLKSRC_SEL_A>;
impl CLKSRC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKSRC_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSRC_SEL_A::CLKSRC_SEL_0),
            1 => Val(CLKSRC_SEL_A::CLKSRC_SEL_1),
            3 => Val(CLKSRC_SEL_A::CLKSRC_SEL_3),
            5 => Val(CLKSRC_SEL_A::CLKSRC_SEL_5),
            6 => Val(CLKSRC_SEL_A::CLKSRC_SEL_6),
            8 => Val(CLKSRC_SEL_A::CLKSRC_SEL_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_0`"]
    #[inline(always)]
    pub fn is_clk_src_sel_0(&self) -> bool {
        *self == CLKSRC_SEL_A::CLKSRC_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_1`"]
    #[inline(always)]
    pub fn is_clk_src_sel_1(&self) -> bool {
        *self == CLKSRC_SEL_A::CLKSRC_SEL_1
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_3`"]
    #[inline(always)]
    pub fn is_clk_src_sel_3(&self) -> bool {
        *self == CLKSRC_SEL_A::CLKSRC_SEL_3
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_5`"]
    #[inline(always)]
    pub fn is_clk_src_sel_5(&self) -> bool {
        *self == CLKSRC_SEL_A::CLKSRC_SEL_5
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_6`"]
    #[inline(always)]
    pub fn is_clk_src_sel_6(&self) -> bool {
        *self == CLKSRC_SEL_A::CLKSRC_SEL_6
    }
    #[doc = "Checks if the value of the field is `CLKSRC_SEL_8`"]
    #[inline(always)]
    pub fn is_clk_src_sel_8(&self) -> bool {
        *self == CLKSRC_SEL_A::CLKSRC_SEL_8
    }
}
#[doc = "Write proxy for field `ClkSrc_Sel`"]
pub struct CLKSRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    #[inline(always)]
    pub fn clk_src_sel_0(self) -> &'a mut W {
        self.variant(CLKSRC_SEL_A::CLKSRC_SEL_0)
    }
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    #[inline(always)]
    pub fn clk_src_sel_1(self) -> &'a mut W {
        self.variant(CLKSRC_SEL_A::CLKSRC_SEL_1)
    }
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    #[inline(always)]
    pub fn clk_src_sel_3(self) -> &'a mut W {
        self.variant(CLKSRC_SEL_A::CLKSRC_SEL_3)
    }
    #[doc = "REF_CLK_32K (XTALOSC)"]
    #[inline(always)]
    pub fn clk_src_sel_5(self) -> &'a mut W {
        self.variant(CLKSRC_SEL_A::CLKSRC_SEL_5)
    }
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
    #[inline(always)]
    pub fn clk_src_sel_6(self) -> &'a mut W {
        self.variant(CLKSRC_SEL_A::CLKSRC_SEL_6)
    }
    #[doc = "SPDIF_EXT_CLK"]
    #[inline(always)]
    pub fn clk_src_sel_8(self) -> &'a mut W {
        self.variant(CLKSRC_SEL_A::CLKSRC_SEL_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - Gain selection:"]
    #[inline(always)]
    pub fn gain_sel(&self) -> GAINSEL_R {
        GAINSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - LOCK bit to show that the internal DPLL is locked, read only"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Clock source selection, all other settings not shown are reserved:"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLKSRC_SEL_R {
        CLKSRC_SEL_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - Gain selection:"]
    #[inline(always)]
    pub fn gain_sel(&mut self) -> GAINSEL_W {
        GAINSEL_W { w: self }
    }
    #[doc = "Bits 7:10 - Clock source selection, all other settings not shown are reserved:"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLKSRC_SEL_W {
        CLKSRC_SEL_W { w: self }
    }
}
