#[doc = "Reader of register TEMPSENSE2_CLR"]
pub type R = crate::R<u32, super::TEMPSENSE2_CLR>;
#[doc = "Writer for register TEMPSENSE2_CLR"]
pub type W = crate::W<u32, super::TEMPSENSE2_CLR>;
#[doc = "Register TEMPSENSE2_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::TEMPSENSE2_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOW_ALARM_VALUE`"]
pub type LOW_ALARM_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LOW_ALARM_VALUE`"]
pub struct LOW_ALARM_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_ALARM_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `PANIC_ALARM_VALUE`"]
pub type PANIC_ALARM_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PANIC_ALARM_VALUE`"]
pub struct PANIC_ALARM_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PANIC_ALARM_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub fn low_alarm_value(&self) -> LOW_ALARM_VALUE_R {
        LOW_ALARM_VALUE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub fn panic_alarm_value(&self) -> PANIC_ALARM_VALUE_R {
        PANIC_ALARM_VALUE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub fn low_alarm_value(&mut self) -> LOW_ALARM_VALUE_W {
        LOW_ALARM_VALUE_W { w: self }
    }
    #[doc = "Bits 16:27 - This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub fn panic_alarm_value(&mut self) -> PANIC_ALARM_VALUE_W {
        PANIC_ALARM_VALUE_W { w: self }
    }
}
