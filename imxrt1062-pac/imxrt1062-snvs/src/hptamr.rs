#[doc = "Reader of register HPTAMR"]
pub type R = crate::R<u32, super::HPTAMR>;
#[doc = "Writer for register HPTAMR"]
pub type W = crate::W<u32, super::HPTAMR>;
#[doc = "Register HPTAMR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPTAMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPTA_MS`"]
pub type HPTA_MS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HPTA_MS`"]
pub struct HPTA_MS_W<'a> {
    w: &'a mut W,
}
impl<'a> HPTA_MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - HP Time Alarm, most-significant 15 bits"]
    #[inline(always)]
    pub fn hpta_ms(&self) -> HPTA_MS_R {
        HPTA_MS_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - HP Time Alarm, most-significant 15 bits"]
    #[inline(always)]
    pub fn hpta_ms(&mut self) -> HPTA_MS_W {
        HPTA_MS_W { w: self }
    }
}
