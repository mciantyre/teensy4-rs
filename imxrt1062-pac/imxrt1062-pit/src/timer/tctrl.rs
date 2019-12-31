#[doc = "Reader of register TCTRL"]
pub type R = crate::R<u32, super::TCTRL>;
#[doc = "Writer for register TCTRL"]
pub type W = crate::W<u32, super::TCTRL>;
#[doc = "Register TCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
    #[doc = "0: Timer n is disabled."]
    TEN_0 = 0,
    #[doc = "1: Timer n is enabled."]
    TEN_1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEN`"]
pub type TEN_R = crate::R<bool, TEN_A>;
impl TEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::TEN_0,
            true => TEN_A::TEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEN_0`"]
    #[inline(always)]
    pub fn is_ten_0(&self) -> bool {
        *self == TEN_A::TEN_0
    }
    #[doc = "Checks if the value of the field is `TEN_1`"]
    #[inline(always)]
    pub fn is_ten_1(&self) -> bool {
        *self == TEN_A::TEN_1
    }
}
#[doc = "Write proxy for field `TEN`"]
pub struct TEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer n is disabled."]
    #[inline(always)]
    pub fn ten_0(self) -> &'a mut W {
        self.variant(TEN_A::TEN_0)
    }
    #[doc = "Timer n is enabled."]
    #[inline(always)]
    pub fn ten_1(self) -> &'a mut W {
        self.variant(TEN_A::TEN_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Interrupt requests from Timer n are disabled."]
    TIE_0 = 0,
    #[doc = "1: Interrupt will be requested whenever TIF is set."]
    TIE_1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::TIE_0,
            true => TIE_A::TIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE_0`"]
    #[inline(always)]
    pub fn is_tie_0(&self) -> bool {
        *self == TIE_A::TIE_0
    }
    #[doc = "Checks if the value of the field is `TIE_1`"]
    #[inline(always)]
    pub fn is_tie_1(&self) -> bool {
        *self == TIE_A::TIE_1
    }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt requests from Timer n are disabled."]
    #[inline(always)]
    pub fn tie_0(self) -> &'a mut W {
        self.variant(TIE_A::TIE_0)
    }
    #[doc = "Interrupt will be requested whenever TIF is set."]
    #[inline(always)]
    pub fn tie_1(self) -> &'a mut W {
        self.variant(TIE_A::TIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Chain Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHN_A {
    #[doc = "0: Timer is not chained."]
    CHN_0 = 0,
    #[doc = "1: Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    CHN_1 = 1,
}
impl From<CHN_A> for bool {
    #[inline(always)]
    fn from(variant: CHN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHN`"]
pub type CHN_R = crate::R<bool, CHN_A>;
impl CHN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHN_A {
        match self.bits {
            false => CHN_A::CHN_0,
            true => CHN_A::CHN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CHN_0`"]
    #[inline(always)]
    pub fn is_chn_0(&self) -> bool {
        *self == CHN_A::CHN_0
    }
    #[doc = "Checks if the value of the field is `CHN_1`"]
    #[inline(always)]
    pub fn is_chn_1(&self) -> bool {
        *self == CHN_A::CHN_1
    }
}
#[doc = "Write proxy for field `CHN`"]
pub struct CHN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer is not chained."]
    #[inline(always)]
    pub fn chn_0(self) -> &'a mut W {
        self.variant(CHN_A::CHN_0)
    }
    #[doc = "Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    #[inline(always)]
    pub fn chn_1(self) -> &'a mut W {
        self.variant(CHN_A::CHN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W { w: self }
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline(always)]
    pub fn chn(&mut self) -> CHN_W {
        CHN_W { w: self }
    }
}
