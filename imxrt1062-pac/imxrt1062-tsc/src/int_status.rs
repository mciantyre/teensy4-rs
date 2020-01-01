#[doc = "Reader of register INT_STATUS"]
pub type R = crate::R<u32, super::INT_STATUS>;
#[doc = "Writer for register INT_STATUS"]
pub type W = crate::W<u32, super::INT_STATUS>;
#[doc = "Register INT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Measure Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEASURE_A {
    #[doc = "0: Does not exist a measure signal"]
    MEASURE_0 = 0,
    #[doc = "1: Exist a measure signal"]
    MEASURE_1 = 1,
}
impl From<MEASURE_A> for bool {
    #[inline(always)]
    fn from(variant: MEASURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEASURE`"]
pub type MEASURE_R = crate::R<bool, MEASURE_A>;
impl MEASURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEASURE_A {
        match self.bits {
            false => MEASURE_A::MEASURE_0,
            true => MEASURE_A::MEASURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEASURE_0`"]
    #[inline(always)]
    pub fn is_measure_0(&self) -> bool {
        *self == MEASURE_A::MEASURE_0
    }
    #[doc = "Checks if the value of the field is `MEASURE_1`"]
    #[inline(always)]
    pub fn is_measure_1(&self) -> bool {
        *self == MEASURE_A::MEASURE_1
    }
}
#[doc = "Write proxy for field `MEASURE`"]
pub struct MEASURE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEASURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEASURE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Does not exist a measure signal"]
    #[inline(always)]
    pub fn measure_0(self) -> &'a mut W {
        self.variant(MEASURE_A::MEASURE_0)
    }
    #[doc = "Exist a measure signal"]
    #[inline(always)]
    pub fn measure_1(self) -> &'a mut W {
        self.variant(MEASURE_A::MEASURE_1)
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
#[doc = "Detect Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_A {
    #[doc = "0: Does not exist a detect signal"]
    DETECT_0 = 0,
    #[doc = "1: Exist detect signal"]
    DETECT_1 = 1,
}
impl From<DETECT_A> for bool {
    #[inline(always)]
    fn from(variant: DETECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETECT`"]
pub type DETECT_R = crate::R<bool, DETECT_A>;
impl DETECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECT_A {
        match self.bits {
            false => DETECT_A::DETECT_0,
            true => DETECT_A::DETECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_0`"]
    #[inline(always)]
    pub fn is_detect_0(&self) -> bool {
        *self == DETECT_A::DETECT_0
    }
    #[doc = "Checks if the value of the field is `DETECT_1`"]
    #[inline(always)]
    pub fn is_detect_1(&self) -> bool {
        *self == DETECT_A::DETECT_1
    }
}
#[doc = "Write proxy for field `DETECT`"]
pub struct DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DETECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Does not exist a detect signal"]
    #[inline(always)]
    pub fn detect_0(self) -> &'a mut W {
        self.variant(DETECT_A::DETECT_0)
    }
    #[doc = "Exist detect signal"]
    #[inline(always)]
    pub fn detect_1(self) -> &'a mut W {
        self.variant(DETECT_A::DETECT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Valid Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_A {
    #[doc = "0: There is no touch detected after measurement, indicates that the measured value is not valid"]
    VALID_0 = 0,
    #[doc = "1: There is touch detection after measurement, indicates that the measure is valid"]
    VALID_1 = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, VALID_A>;
impl VALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::VALID_0,
            true => VALID_A::VALID_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_0`"]
    #[inline(always)]
    pub fn is_valid_0(&self) -> bool {
        *self == VALID_A::VALID_0
    }
    #[doc = "Checks if the value of the field is `VALID_1`"]
    #[inline(always)]
    pub fn is_valid_1(&self) -> bool {
        *self == VALID_A::VALID_1
    }
}
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "There is no touch detected after measurement, indicates that the measured value is not valid"]
    #[inline(always)]
    pub fn valid_0(self) -> &'a mut W {
        self.variant(VALID_A::VALID_0)
    }
    #[doc = "There is touch detection after measurement, indicates that the measure is valid"]
    #[inline(always)]
    pub fn valid_1(self) -> &'a mut W {
        self.variant(VALID_A::VALID_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Idle Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_SW_A {
    #[doc = "0: Haven't return to idle status"]
    IDLE_SW_0 = 0,
    #[doc = "1: Already return to idle status"]
    IDLE_SW_1 = 1,
}
impl From<IDLE_SW_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLE_SW`"]
pub type IDLE_SW_R = crate::R<bool, IDLE_SW_A>;
impl IDLE_SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_SW_A {
        match self.bits {
            false => IDLE_SW_A::IDLE_SW_0,
            true => IDLE_SW_A::IDLE_SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_0`"]
    #[inline(always)]
    pub fn is_idle_sw_0(&self) -> bool {
        *self == IDLE_SW_A::IDLE_SW_0
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_1`"]
    #[inline(always)]
    pub fn is_idle_sw_1(&self) -> bool {
        *self == IDLE_SW_A::IDLE_SW_1
    }
}
#[doc = "Write proxy for field `IDLE_SW`"]
pub struct IDLE_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Haven't return to idle status"]
    #[inline(always)]
    pub fn idle_sw_0(self) -> &'a mut W {
        self.variant(IDLE_SW_A::IDLE_SW_0)
    }
    #[doc = "Already return to idle status"]
    #[inline(always)]
    pub fn idle_sw_1(self) -> &'a mut W {
        self.variant(IDLE_SW_A::IDLE_SW_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Measure Signal"]
    #[inline(always)]
    pub fn measure(&self) -> MEASURE_R {
        MEASURE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Detect Signal"]
    #[inline(always)]
    pub fn detect(&self) -> DETECT_R {
        DETECT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Valid Signal"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Idle Software"]
    #[inline(always)]
    pub fn idle_sw(&self) -> IDLE_SW_R {
        IDLE_SW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Measure Signal"]
    #[inline(always)]
    pub fn measure(&mut self) -> MEASURE_W {
        MEASURE_W { w: self }
    }
    #[doc = "Bit 4 - Detect Signal"]
    #[inline(always)]
    pub fn detect(&mut self) -> DETECT_W {
        DETECT_W { w: self }
    }
    #[doc = "Bit 8 - Valid Signal"]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
    #[doc = "Bit 12 - Idle Software"]
    #[inline(always)]
    pub fn idle_sw(&mut self) -> IDLE_SW_W {
        IDLE_SW_W { w: self }
    }
}
