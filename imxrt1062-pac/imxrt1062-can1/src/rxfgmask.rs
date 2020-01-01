#[doc = "Reader of register RXFGMASK"]
pub type R = crate::R<u32, super::RXFGMASK>;
#[doc = "Writer for register RXFGMASK"]
pub type W = crate::W<u32, super::RXFGMASK>;
#[doc = "Register RXFGMASK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RXFGMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "These bits mask the ID Filter Table elements bits in a perfect alignment\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FGM_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care\""]
    FGM_0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    FGM_1 = 1,
}
impl From<FGM_A> for u32 {
    #[inline(always)]
    fn from(variant: FGM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FGM`"]
pub type FGM_R = crate::R<u32, FGM_A>;
impl FGM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FGM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FGM_A::FGM_0),
            1 => Val(FGM_A::FGM_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FGM_0`"]
    #[inline(always)]
    pub fn is_fgm_0(&self) -> bool {
        *self == FGM_A::FGM_0
    }
    #[doc = "Checks if the value of the field is `FGM_1`"]
    #[inline(always)]
    pub fn is_fgm_1(&self) -> bool {
        *self == FGM_A::FGM_1
    }
}
#[doc = "Write proxy for field `FGM`"]
pub struct FGM_W<'a> {
    w: &'a mut W,
}
impl<'a> FGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FGM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn fgm_0(self) -> &'a mut W {
        self.variant(FGM_A::FGM_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn fgm_1(self) -> &'a mut W {
        self.variant(FGM_A::FGM_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - These bits mask the ID Filter Table elements bits in a perfect alignment"]
    #[inline(always)]
    pub fn fgm(&self) -> FGM_R {
        FGM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits mask the ID Filter Table elements bits in a perfect alignment"]
    #[inline(always)]
    pub fn fgm(&mut self) -> FGM_W {
        FGM_W { w: self }
    }
}
