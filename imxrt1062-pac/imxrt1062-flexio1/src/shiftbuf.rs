#[doc = "Reader of register SHIFTBUF[%s]"]
pub type R = crate::R<u32, super::SHIFTBUF>;
#[doc = "Writer for register SHIFTBUF[%s]"]
pub type W = crate::W<u32, super::SHIFTBUF>;
#[doc = "Register SHIFTBUF[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTBUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHIFTBUF`"]
pub type SHIFTBUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SHIFTBUF`"]
pub struct SHIFTBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbuf(&self) -> SHIFTBUF_R {
        SHIFTBUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbuf(&mut self) -> SHIFTBUF_W {
        SHIFTBUF_W { w: self }
    }
}
