#[doc = "Reader of register PS_BACKGROUND"]
pub type R = crate::R<u32, super::PS_BACKGROUND>;
#[doc = "Writer for register PS_BACKGROUND"]
pub type W = crate::W<u32, super::PS_BACKGROUND>;
#[doc = "Register PS_BACKGROUND `reset()`'s with value 0"]
impl crate::ResetValue for super::PS_BACKGROUND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COLOR`"]
pub type COLOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COLOR`"]
pub struct COLOR_W<'a> {
    w: &'a mut W,
}
impl<'a> COLOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `RSVD`"]
pub type RSVD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:23 - Background color (in 24bpp format) for any pixels not within the buffer range specified by the PS ULC/LRC"]
    #[inline(always)]
    pub fn color(&self) -> COLOR_R {
        COLOR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Background color (in 24bpp format) for any pixels not within the buffer range specified by the PS ULC/LRC"]
    #[inline(always)]
    pub fn color(&mut self) -> COLOR_W {
        COLOR_W { w: self }
    }
}
