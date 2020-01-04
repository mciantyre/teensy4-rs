#[doc = "Reader of register TEMPSENSE1"]
pub type R = crate::R<u32, super::TEMPSENSE1>;
#[doc = "Writer for register TEMPSENSE1"]
pub type W = crate::W<u32, super::TEMPSENSE1>;
#[doc = "Register TEMPSENSE1 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::TEMPSENSE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `MEASURE_FREQ`"]
pub type MEASURE_FREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEASURE_FREQ`"]
pub struct MEASURE_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MEASURE_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub fn measure_freq(&self) -> MEASURE_FREQ_R {
        MEASURE_FREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub fn measure_freq(&mut self) -> MEASURE_FREQ_W {
        MEASURE_FREQ_W { w: self }
    }
}
