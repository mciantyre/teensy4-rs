#[doc = "Reader of register TCD11_SOFF"]
pub type R = crate::R<u16, super::TCD11_SOFF>;
#[doc = "Writer for register TCD11_SOFF"]
pub type W = crate::W<u16, super::TCD11_SOFF>;
#[doc = "Register TCD11_SOFF `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD11_SOFF {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFF`"]
pub type SOFF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SOFF`"]
pub struct SOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Source address signed offset"]
    #[inline(always)]
    pub fn soff(&self) -> SOFF_R {
        SOFF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source address signed offset"]
    #[inline(always)]
    pub fn soff(&mut self) -> SOFF_W {
        SOFF_W { w: self }
    }
}
