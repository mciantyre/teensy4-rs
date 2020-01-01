#[doc = "Reader of register SHIFTSTAT"]
pub type R = crate::R<u32, super::SHIFTSTAT>;
#[doc = "Writer for register SHIFTSTAT"]
pub type W = crate::W<u32, super::SHIFTSTAT>;
#[doc = "Register SHIFTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSF`"]
pub type SSF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSF`"]
pub struct SSF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&mut self) -> SSF_W {
        SSF_W { w: self }
    }
}
