#[doc = "Reader of register GS"]
pub type R = crate::R<u32, super::GS>;
#[doc = "Writer for register GS"]
pub type W = crate::W<u32, super::GS>;
#[doc = "Register GS `reset()`'s with value 0"]
impl crate::ResetValue for super::GS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Conversion Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACT_A {
    #[doc = "0: Conversion not in progress."]
    ADACT_0 = 0,
    #[doc = "1: Conversion in progress."]
    ADACT_1 = 1,
}
impl From<ADACT_A> for bool {
    #[inline(always)]
    fn from(variant: ADACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADACT`"]
pub type ADACT_R = crate::R<bool, ADACT_A>;
impl ADACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACT_A {
        match self.bits {
            false => ADACT_A::ADACT_0,
            true => ADACT_A::ADACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADACT_0`"]
    #[inline(always)]
    pub fn is_adact_0(&self) -> bool {
        *self == ADACT_A::ADACT_0
    }
    #[doc = "Checks if the value of the field is `ADACT_1`"]
    #[inline(always)]
    pub fn is_adact_1(&self) -> bool {
        *self == ADACT_A::ADACT_1
    }
}
#[doc = "Calibration Failed Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALF_A {
    #[doc = "0: Calibration completed normally."]
    CALF_0 = 0,
    #[doc = "1: Calibration failed. ADC accuracy specifications are not guaranteed."]
    CALF_1 = 1,
}
impl From<CALF_A> for bool {
    #[inline(always)]
    fn from(variant: CALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALF`"]
pub type CALF_R = crate::R<bool, CALF_A>;
impl CALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALF_A {
        match self.bits {
            false => CALF_A::CALF_0,
            true => CALF_A::CALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALF_0`"]
    #[inline(always)]
    pub fn is_calf_0(&self) -> bool {
        *self == CALF_A::CALF_0
    }
    #[doc = "Checks if the value of the field is `CALF_1`"]
    #[inline(always)]
    pub fn is_calf_1(&self) -> bool {
        *self == CALF_A::CALF_1
    }
}
#[doc = "Write proxy for field `CALF`"]
pub struct CALF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calibration completed normally."]
    #[inline(always)]
    pub fn calf_0(self) -> &'a mut W {
        self.variant(CALF_A::CALF_0)
    }
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    #[inline(always)]
    pub fn calf_1(self) -> &'a mut W {
        self.variant(CALF_A::CALF_1)
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
#[doc = "Asynchronous wakeup interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWKST_A {
    #[doc = "0: No asynchronous interrupt."]
    AWKST_0 = 0,
    #[doc = "1: Asynchronous wake up interrupt occurred in stop mode."]
    AWKST_1 = 1,
}
impl From<AWKST_A> for bool {
    #[inline(always)]
    fn from(variant: AWKST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWKST`"]
pub type AWKST_R = crate::R<bool, AWKST_A>;
impl AWKST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWKST_A {
        match self.bits {
            false => AWKST_A::AWKST_0,
            true => AWKST_A::AWKST_1,
        }
    }
    #[doc = "Checks if the value of the field is `AWKST_0`"]
    #[inline(always)]
    pub fn is_awkst_0(&self) -> bool {
        *self == AWKST_A::AWKST_0
    }
    #[doc = "Checks if the value of the field is `AWKST_1`"]
    #[inline(always)]
    pub fn is_awkst_1(&self) -> bool {
        *self == AWKST_A::AWKST_1
    }
}
#[doc = "Write proxy for field `AWKST`"]
pub struct AWKST_W<'a> {
    w: &'a mut W,
}
impl<'a> AWKST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWKST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No asynchronous interrupt."]
    #[inline(always)]
    pub fn awkst_0(self) -> &'a mut W {
        self.variant(AWKST_A::AWKST_0)
    }
    #[doc = "Asynchronous wake up interrupt occurred in stop mode."]
    #[inline(always)]
    pub fn awkst_1(self) -> &'a mut W {
        self.variant(AWKST_A::AWKST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Conversion Active"]
    #[inline(always)]
    pub fn adact(&self) -> ADACT_R {
        ADACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Calibration Failed Flag"]
    #[inline(always)]
    pub fn calf(&self) -> CALF_R {
        CALF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Asynchronous wakeup interrupt status"]
    #[inline(always)]
    pub fn awkst(&self) -> AWKST_R {
        AWKST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Calibration Failed Flag"]
    #[inline(always)]
    pub fn calf(&mut self) -> CALF_W {
        CALF_W { w: self }
    }
    #[doc = "Bit 2 - Asynchronous wakeup interrupt status"]
    #[inline(always)]
    pub fn awkst(&mut self) -> AWKST_W {
        AWKST_W { w: self }
    }
}
