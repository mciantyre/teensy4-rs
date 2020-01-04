#[doc = "Reader of register TRIG1_COUNTER"]
pub type R = crate::R<u32, super::TRIG1_COUNTER>;
#[doc = "Writer for register TRIG1_COUNTER"]
pub type W = crate::W<u32, super::TRIG1_COUNTER>;
#[doc = "Register TRIG1_COUNTER `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIG1_COUNTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT_DELAY`"]
pub type INIT_DELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INIT_DELAY`"]
pub struct INIT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `SAMPLE_INTERVAL`"]
pub type SAMPLE_INTERVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAMPLE_INTERVAL`"]
pub struct SAMPLE_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TRIGGER initial delay counter"]
    #[inline(always)]
    pub fn init_delay(&self) -> INIT_DELAY_R {
        INIT_DELAY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TRIGGER sampling interval counter"]
    #[inline(always)]
    pub fn sample_interval(&self) -> SAMPLE_INTERVAL_R {
        SAMPLE_INTERVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TRIGGER initial delay counter"]
    #[inline(always)]
    pub fn init_delay(&mut self) -> INIT_DELAY_W {
        INIT_DELAY_W { w: self }
    }
    #[doc = "Bits 16:31 - TRIGGER sampling interval counter"]
    #[inline(always)]
    pub fn sample_interval(&mut self) -> SAMPLE_INTERVAL_W {
        SAMPLE_INTERVAL_W { w: self }
    }
}
