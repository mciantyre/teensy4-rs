#[doc = "Reader of register AS_CLRKEYHIGH"]
pub type R = crate::R<u32, super::AS_CLRKEYHIGH>;
#[doc = "Writer for register AS_CLRKEYHIGH"]
pub type W = crate::W<u32, super::AS_CLRKEYHIGH>;
#[doc = "Register AS_CLRKEYHIGH `reset()`'s with value 0"]
impl crate::ResetValue for super::AS_CLRKEYHIGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIXEL`"]
pub type PIXEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PIXEL`"]
pub struct PIXEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:23 - High range of RGB color key applied to AS buffer. Each overlay has an independent colorkey enable."]
    #[inline(always)]
    pub fn pixel(&self) -> PIXEL_R {
        PIXEL_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - High range of RGB color key applied to AS buffer. Each overlay has an independent colorkey enable."]
    #[inline(always)]
    pub fn pixel(&mut self) -> PIXEL_W {
        PIXEL_W { w: self }
    }
}
