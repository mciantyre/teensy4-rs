#[doc = "Reader of register TMR"]
pub type R = crate::R<u32, super::TMR>;
#[doc = "Writer for register TMR"]
pub type W = crate::W<u32, super::TMR>;
#[doc = "Register TMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TWM_A {
    #[doc = "0: Word N is enabled."]
    TWM_0 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    TWM_1 = 1,
}
impl From<TWM_A> for u32 {
    #[inline(always)]
    fn from(variant: TWM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TWM`"]
pub type TWM_R = crate::R<u32, TWM_A>;
impl TWM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TWM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TWM_A::TWM_0),
            1 => Val(TWM_A::TWM_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TWM_0`"]
    #[inline(always)]
    pub fn is_twm_0(&self) -> bool {
        *self == TWM_A::TWM_0
    }
    #[doc = "Checks if the value of the field is `TWM_1`"]
    #[inline(always)]
    pub fn is_twm_1(&self) -> bool {
        *self == TWM_A::TWM_1
    }
}
#[doc = "Write proxy for field `TWM`"]
pub struct TWM_W<'a> {
    w: &'a mut W,
}
impl<'a> TWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn twm_0(self) -> &'a mut W {
        self.variant(TWM_A::TWM_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    #[inline(always)]
    pub fn twm_1(self) -> &'a mut W {
        self.variant(TWM_A::TWM_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm(&self) -> TWM_R {
        TWM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm(&mut self) -> TWM_W {
        TWM_W { w: self }
    }
}
