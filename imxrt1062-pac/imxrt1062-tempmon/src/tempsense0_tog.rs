#[doc = "Reader of register TEMPSENSE0_TOG"]
pub type R = crate::R<u32, super::TEMPSENSE0_TOG>;
#[doc = "Writer for register TEMPSENSE0_TOG"]
pub type W = crate::W<u32, super::TEMPSENSE0_TOG>;
#[doc = "Register TEMPSENSE0_TOG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::TEMPSENSE0_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "This bit powers down the temperature sensor.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POWER_DOWN_A {
    #[doc = "0: Enable power to the temperature sensor."]
    POWER_UP = 0,
    #[doc = "1: Power down the temperature sensor."]
    POWER_DOWN = 1,
}
impl From<POWER_DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: POWER_DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POWER_DOWN`"]
pub type POWER_DOWN_R = crate::R<bool, POWER_DOWN_A>;
impl POWER_DOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_DOWN_A {
        match self.bits {
            false => POWER_DOWN_A::POWER_UP,
            true => POWER_DOWN_A::POWER_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == POWER_DOWN_A::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == POWER_DOWN_A::POWER_DOWN
    }
}
#[doc = "Write proxy for field `POWER_DOWN`"]
pub struct POWER_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_DOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POWER_DOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable power to the temperature sensor."]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(POWER_DOWN_A::POWER_UP)
    }
    #[doc = "Power down the temperature sensor."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(POWER_DOWN_A::POWER_DOWN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Starts the measurement process\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEASURE_TEMP_A {
    #[doc = "0: Do not start the measurement process."]
    STOP = 0,
    #[doc = "1: Start the measurement process."]
    START = 1,
}
impl From<MEASURE_TEMP_A> for bool {
    #[inline(always)]
    fn from(variant: MEASURE_TEMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEASURE_TEMP`"]
pub type MEASURE_TEMP_R = crate::R<bool, MEASURE_TEMP_A>;
impl MEASURE_TEMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEASURE_TEMP_A {
        match self.bits {
            false => MEASURE_TEMP_A::STOP,
            true => MEASURE_TEMP_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MEASURE_TEMP_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == MEASURE_TEMP_A::START
    }
}
#[doc = "Write proxy for field `MEASURE_TEMP`"]
pub struct MEASURE_TEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> MEASURE_TEMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEASURE_TEMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not start the measurement process."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MEASURE_TEMP_A::STOP)
    }
    #[doc = "Start the measurement process."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(MEASURE_TEMP_A::START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Indicates that the latest temp is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINISHED_A {
    #[doc = "0: Last measurement is not ready yet."]
    INVALID = 0,
    #[doc = "1: Last measurement is valid."]
    VALID = 1,
}
impl From<FINISHED_A> for bool {
    #[inline(always)]
    fn from(variant: FINISHED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FINISHED`"]
pub type FINISHED_R = crate::R<bool, FINISHED_A>;
impl FINISHED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINISHED_A {
        match self.bits {
            false => FINISHED_A::INVALID,
            true => FINISHED_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == FINISHED_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == FINISHED_A::VALID
    }
}
#[doc = "Reader of field `TEMP_CNT`"]
pub type TEMP_CNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `ALARM_VALUE`"]
pub type ALARM_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ALARM_VALUE`"]
pub struct ALARM_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit powers down the temperature sensor."]
    #[inline(always)]
    pub fn power_down(&self) -> POWER_DOWN_R {
        POWER_DOWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Starts the measurement process"]
    #[inline(always)]
    pub fn measure_temp(&self) -> MEASURE_TEMP_R {
        MEASURE_TEMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates that the latest temp is valid"]
    #[inline(always)]
    pub fn finished(&self) -> FINISHED_R {
        FINISHED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:19 - This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub fn temp_cnt(&self) -> TEMP_CNT_R {
        TEMP_CNT_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub fn alarm_value(&self) -> ALARM_VALUE_R {
        ALARM_VALUE_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This bit powers down the temperature sensor."]
    #[inline(always)]
    pub fn power_down(&mut self) -> POWER_DOWN_W {
        POWER_DOWN_W { w: self }
    }
    #[doc = "Bit 1 - Starts the measurement process"]
    #[inline(always)]
    pub fn measure_temp(&mut self) -> MEASURE_TEMP_W {
        MEASURE_TEMP_W { w: self }
    }
    #[doc = "Bits 20:31 - This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub fn alarm_value(&mut self) -> ALARM_VALUE_W {
        ALARM_VALUE_W { w: self }
    }
}
