#[doc = "Reader of register IMASK2"]
pub type R = crate::R<u32, super::IMASK2>;
#[doc = "Writer for register IMASK2"]
pub type W = crate::W<u32, super::IMASK2>;
#[doc = "Register IMASK2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMASK2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BUFHM_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled"]
    BUFHM_0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled"]
    BUFHM_1 = 1,
}
impl From<BUFHM_A> for u32 {
    #[inline(always)]
    fn from(variant: BUFHM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BUFHM`"]
pub type BUFHM_R = crate::R<u32, BUFHM_A>;
impl BUFHM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BUFHM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BUFHM_A::BUFHM_0),
            1 => Val(BUFHM_A::BUFHM_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUFHM_0`"]
    #[inline(always)]
    pub fn is_bufhm_0(&self) -> bool {
        *self == BUFHM_A::BUFHM_0
    }
    #[doc = "Checks if the value of the field is `BUFHM_1`"]
    #[inline(always)]
    pub fn is_bufhm_1(&self) -> bool {
        *self == BUFHM_A::BUFHM_1
    }
}
#[doc = "Write proxy for field `BUFHM`"]
pub struct BUFHM_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFHM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFHM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding buffer Interrupt is disabled"]
    #[inline(always)]
    pub fn bufhm_0(self) -> &'a mut W {
        self.variant(BUFHM_A::BUFHM_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled"]
    #[inline(always)]
    pub fn bufhm_1(self) -> &'a mut W {
        self.variant(BUFHM_A::BUFHM_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    #[inline(always)]
    pub fn bufhm(&self) -> BUFHM_R {
        BUFHM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    #[inline(always)]
    pub fn bufhm(&mut self) -> BUFHM_W {
        BUFHM_W { w: self }
    }
}
