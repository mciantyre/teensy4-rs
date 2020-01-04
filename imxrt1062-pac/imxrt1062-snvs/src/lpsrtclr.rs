#[doc = "Reader of register LPSRTCLR"]
pub type R = crate::R<u32, super::LPSRTCLR>;
#[doc = "Writer for register LPSRTCLR"]
pub type W = crate::W<u32, super::LPSRTCLR>;
#[doc = "Register LPSRTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPSRTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRTC`"]
pub type SRTC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SRTC`"]
pub struct SRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
    #[inline(always)]
    pub fn srtc(&self) -> SRTC_R {
        SRTC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
    #[inline(always)]
    pub fn srtc(&mut self) -> SRTC_W {
        SRTC_W { w: self }
    }
}
