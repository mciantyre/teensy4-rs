#[doc = "Reader of register SHIFTERR"]
pub type R = crate::R<u32, super::SHIFTERR>;
#[doc = "Writer for register SHIFTERR"]
pub type W = crate::W<u32, super::SHIFTERR>;
#[doc = "Register SHIFTERR `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTERR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEF`"]
pub type SEF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEF`"]
pub struct SEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W {
        SEF_W { w: self }
    }
}
