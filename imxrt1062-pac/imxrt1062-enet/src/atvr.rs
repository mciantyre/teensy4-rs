#[doc = "Reader of register ATVR"]
pub type R = crate::R<u32, super::ATVR>;
#[doc = "Writer for register ATVR"]
pub type W = crate::W<u32, super::ATVR>;
#[doc = "Register ATVR `reset()`'s with value 0"]
impl crate::ResetValue for super::ATVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATIME`"]
pub type ATIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ATIME`"]
pub struct ATIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ATIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - A write sets the timer"]
    #[inline(always)]
    pub fn atime(&self) -> ATIME_R {
        ATIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - A write sets the timer"]
    #[inline(always)]
    pub fn atime(&mut self) -> ATIME_W {
        ATIME_W { w: self }
    }
}
