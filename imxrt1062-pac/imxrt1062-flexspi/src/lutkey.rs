#[doc = "Reader of register LUTKEY"]
pub type R = crate::R<u32, super::LUTKEY>;
#[doc = "Writer for register LUTKEY"]
pub type W = crate::W<u32, super::LUTKEY>;
#[doc = "Register LUTKEY `reset()`'s with value 0x5af0_5af0"]
impl crate::ResetValue for super::LUTKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5af0_5af0
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The Key to lock or unlock LUT."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Key to lock or unlock LUT."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
