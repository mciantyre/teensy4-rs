#[doc = "Reader of register LPSRTCMR"]
pub type R = crate::R<u32, super::LPSRTCMR>;
#[doc = "Writer for register LPSRTCMR"]
pub type W = crate::W<u32, super::LPSRTCMR>;
#[doc = "Register LPSRTCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPSRTCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRTC`"]
pub type SRTC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SRTC`"]
pub struct SRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    #[inline(always)]
    pub fn srtc(&self) -> SRTC_R {
        SRTC_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    #[inline(always)]
    pub fn srtc(&mut self) -> SRTC_W {
        SRTC_W { w: self }
    }
}
