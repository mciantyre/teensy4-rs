#[doc = "Reader of register TCD14_DOFF"]
pub type R = crate::R<u16, super::TCD14_DOFF>;
#[doc = "Writer for register TCD14_DOFF"]
pub type W = crate::W<u16, super::TCD14_DOFF>;
#[doc = "Register TCD14_DOFF `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD14_DOFF {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOFF`"]
pub type DOFF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOFF`"]
pub struct DOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Destination Address Signed Offset"]
    #[inline(always)]
    pub fn doff(&self) -> DOFF_R {
        DOFF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Destination Address Signed Offset"]
    #[inline(always)]
    pub fn doff(&mut self) -> DOFF_W {
        DOFF_W { w: self }
    }
}
