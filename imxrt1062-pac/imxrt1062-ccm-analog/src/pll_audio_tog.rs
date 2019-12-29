#[doc = "Reader of register PLL_AUDIO_TOG"]
pub type R = crate::R<u32, super::PLL_AUDIO_TOG>;
#[doc = "Writer for register PLL_AUDIO_TOG"]
pub type W = crate::W<u32, super::PLL_AUDIO_TOG>;
#[doc = "Register PLL_AUDIO_TOG `reset()`'s with value 0x0001_1006"]
impl crate::ResetValue for super::PLL_AUDIO_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_1006
    }
}
#[doc = "Reader of field `DIV_SELECT`"]
pub type DIV_SELECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_SELECT`"]
pub struct DIV_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `POWERDOWN`"]
pub type POWERDOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWERDOWN`"]
pub struct POWERDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWERDOWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Determines the bypass source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_CLK_SRC_A {
    #[doc = "0: Select the 24MHz oscillator as source."]
    REF_CLK_24M,
    #[doc = "1: Select the CLK1_N / CLK1_P as source."]
    CLK1,
}
impl From<BYPASS_CLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_CLK_SRC_A) -> Self {
        match variant {
            BYPASS_CLK_SRC_A::REF_CLK_24M => 0,
            BYPASS_CLK_SRC_A::CLK1 => 1,
        }
    }
}
#[doc = "Reader of field `BYPASS_CLK_SRC`"]
pub type BYPASS_CLK_SRC_R = crate::R<u8, BYPASS_CLK_SRC_A>;
impl BYPASS_CLK_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BYPASS_CLK_SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BYPASS_CLK_SRC_A::REF_CLK_24M),
            1 => Val(BYPASS_CLK_SRC_A::CLK1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REF_CLK_24M`"]
    #[inline(always)]
    pub fn is_ref_clk_24m(&self) -> bool {
        *self == BYPASS_CLK_SRC_A::REF_CLK_24M
    }
    #[doc = "Checks if the value of the field is `CLK1`"]
    #[inline(always)]
    pub fn is_clk1(&self) -> bool {
        *self == BYPASS_CLK_SRC_A::CLK1
    }
}
#[doc = "Write proxy for field `BYPASS_CLK_SRC`"]
pub struct BYPASS_CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_CLK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_CLK_SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select the 24MHz oscillator as source."]
    #[inline(always)]
    pub fn ref_clk_24m(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRC_A::REF_CLK_24M)
    }
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    #[inline(always)]
    pub fn clk1(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRC_A::CLK1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POST_DIV_SELECT_A {
    #[doc = "0: Divide by 4."]
    POST_DIV_SELECT_0,
    #[doc = "1: Divide by 2."]
    POST_DIV_SELECT_1,
    #[doc = "2: Divide by 1."]
    POST_DIV_SELECT_2,
}
impl From<POST_DIV_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_DIV_SELECT_A) -> Self {
        match variant {
            POST_DIV_SELECT_A::POST_DIV_SELECT_0 => 0,
            POST_DIV_SELECT_A::POST_DIV_SELECT_1 => 1,
            POST_DIV_SELECT_A::POST_DIV_SELECT_2 => 2,
        }
    }
}
#[doc = "Reader of field `POST_DIV_SELECT`"]
pub type POST_DIV_SELECT_R = crate::R<u8, POST_DIV_SELECT_A>;
impl POST_DIV_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, POST_DIV_SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(POST_DIV_SELECT_A::POST_DIV_SELECT_0),
            1 => Val(POST_DIV_SELECT_A::POST_DIV_SELECT_1),
            2 => Val(POST_DIV_SELECT_A::POST_DIV_SELECT_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POST_DIV_SELECT_0`"]
    #[inline(always)]
    pub fn is_post_div_select_0(&self) -> bool {
        *self == POST_DIV_SELECT_A::POST_DIV_SELECT_0
    }
    #[doc = "Checks if the value of the field is `POST_DIV_SELECT_1`"]
    #[inline(always)]
    pub fn is_post_div_select_1(&self) -> bool {
        *self == POST_DIV_SELECT_A::POST_DIV_SELECT_1
    }
    #[doc = "Checks if the value of the field is `POST_DIV_SELECT_2`"]
    #[inline(always)]
    pub fn is_post_div_select_2(&self) -> bool {
        *self == POST_DIV_SELECT_A::POST_DIV_SELECT_2
    }
}
#[doc = "Write proxy for field `POST_DIV_SELECT`"]
pub struct POST_DIV_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> POST_DIV_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POST_DIV_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn post_div_select_0(self) -> &'a mut W {
        self.variant(POST_DIV_SELECT_A::POST_DIV_SELECT_0)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn post_div_select_1(self) -> &'a mut W {
        self.variant(POST_DIV_SELECT_A::POST_DIV_SELECT_1)
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn post_div_select_2(self) -> &'a mut W {
        self.variant(POST_DIV_SELECT_A::POST_DIV_SELECT_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:6 - This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub fn div_select(&self) -> DIV_SELECT_R {
        DIV_SELECT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Powers down the PLL."]
    #[inline(always)]
    pub fn powerdown(&self) -> POWERDOWN_R {
        POWERDOWN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable PLL output"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline(always)]
    pub fn bypass_clk_src(&self) -> BYPASS_CLK_SRC_R {
        BYPASS_CLK_SRC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub fn post_div_select(&self) -> POST_DIV_SELECT_R {
        POST_DIV_SELECT_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 31 - 1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub fn div_select(&mut self) -> DIV_SELECT_W {
        DIV_SELECT_W { w: self }
    }
    #[doc = "Bit 12 - Powers down the PLL."]
    #[inline(always)]
    pub fn powerdown(&mut self) -> POWERDOWN_W {
        POWERDOWN_W { w: self }
    }
    #[doc = "Bit 13 - Enable PLL output"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline(always)]
    pub fn bypass_clk_src(&mut self) -> BYPASS_CLK_SRC_W {
        BYPASS_CLK_SRC_W { w: self }
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bits 19:20 - These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub fn post_div_select(&mut self) -> POST_DIV_SELECT_W {
        POST_DIV_SELECT_W { w: self }
    }
}
