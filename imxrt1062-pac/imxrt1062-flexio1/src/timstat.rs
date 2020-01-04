#[doc = "Reader of register TIMSTAT"]
pub type R = crate::R<u32, super::TIMSTAT>;
#[doc = "Writer for register TIMSTAT"]
pub type W = crate::W<u32, super::TIMSTAT>;
#[doc = "Register TIMSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSF`"]
pub type TSF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSF`"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer Status Flags"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer Status Flags"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
}
