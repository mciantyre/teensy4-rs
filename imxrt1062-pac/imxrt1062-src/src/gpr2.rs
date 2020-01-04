#[doc = "Reader of register GPR2"]
pub type R = crate::R<u32, super::GPR2>;
#[doc = "Writer for register GPR2"]
pub type W = crate::W<u32, super::GPR2>;
#[doc = "Register GPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERSISTENT_ARG0`"]
pub type PERSISTENT_ARG0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERSISTENT_ARG0`"]
pub struct PERSISTENT_ARG0_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSISTENT_ARG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Holds argument of entry function for core0 for waking-up from low power mode"]
    #[inline(always)]
    pub fn persistent_arg0(&self) -> PERSISTENT_ARG0_R {
        PERSISTENT_ARG0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Holds argument of entry function for core0 for waking-up from low power mode"]
    #[inline(always)]
    pub fn persistent_arg0(&mut self) -> PERSISTENT_ARG0_W {
        PERSISTENT_ARG0_W { w: self }
    }
}
