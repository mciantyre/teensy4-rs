#[doc = "Reader of register ENBL"]
pub type R = crate::R<u16, super::ENBL>;
#[doc = "Writer for register ENBL"]
pub type W = crate::W<u16, super::ENBL>;
#[doc = "Register ENBL `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::ENBL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Timer Channel Enable\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENBL_A {
    #[doc = "0: Timer channel is disabled."]
    ENBL_0 = 0,
    #[doc = "1: Timer channel is enabled. (default)"]
    ENBL_1 = 1,
}
impl From<ENBL_A> for u8 {
    #[inline(always)]
    fn from(variant: ENBL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENBL`"]
pub type ENBL_R = crate::R<u8, ENBL_A>;
impl ENBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENBL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENBL_A::ENBL_0),
            1 => Val(ENBL_A::ENBL_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENBL_0`"]
    #[inline(always)]
    pub fn is_enbl_0(&self) -> bool {
        *self == ENBL_A::ENBL_0
    }
    #[doc = "Checks if the value of the field is `ENBL_1`"]
    #[inline(always)]
    pub fn is_enbl_1(&self) -> bool {
        *self == ENBL_A::ENBL_1
    }
}
#[doc = "Write proxy for field `ENBL`"]
pub struct ENBL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENBL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer channel is disabled."]
    #[inline(always)]
    pub fn enbl_0(self) -> &'a mut W {
        self.variant(ENBL_A::ENBL_0)
    }
    #[doc = "Timer channel is enabled. (default)"]
    #[inline(always)]
    pub fn enbl_1(self) -> &'a mut W {
        self.variant(ENBL_A::ENBL_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> ENBL_R {
        ENBL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer Channel Enable"]
    #[inline(always)]
    pub fn enbl(&mut self) -> ENBL_W {
        ENBL_W { w: self }
    }
}
