#[doc = "Reader of register INT_SIG_EN"]
pub type R = crate::R<u32, super::INT_SIG_EN>;
#[doc = "Writer for register INT_SIG_EN"]
pub type W = crate::W<u32, super::INT_SIG_EN>;
#[doc = "Register INT_SIG_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_SIG_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEASURE_SIG_EN`"]
pub type MEASURE_SIG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEASURE_SIG_EN`"]
pub struct MEASURE_SIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEASURE_SIG_EN_W<'a> {
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
#[doc = "Detect Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_SIG_EN_A {
    #[doc = "0: Disable detect signal"]
    DETECT_SIG_EN_0 = 0,
    #[doc = "1: Enable detect signal"]
    DETECT_SIG_EN_1 = 1,
}
impl From<DETECT_SIG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DETECT_SIG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETECT_SIG_EN`"]
pub type DETECT_SIG_EN_R = crate::R<bool, DETECT_SIG_EN_A>;
impl DETECT_SIG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECT_SIG_EN_A {
        match self.bits {
            false => DETECT_SIG_EN_A::DETECT_SIG_EN_0,
            true => DETECT_SIG_EN_A::DETECT_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_SIG_EN_0`"]
    #[inline(always)]
    pub fn is_detect_sig_en_0(&self) -> bool {
        *self == DETECT_SIG_EN_A::DETECT_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `DETECT_SIG_EN_1`"]
    #[inline(always)]
    pub fn is_detect_sig_en_1(&self) -> bool {
        *self == DETECT_SIG_EN_A::DETECT_SIG_EN_1
    }
}
#[doc = "Write proxy for field `DETECT_SIG_EN`"]
pub struct DETECT_SIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECT_SIG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DETECT_SIG_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable detect signal"]
    #[inline(always)]
    pub fn detect_sig_en_0(self) -> &'a mut W {
        self.variant(DETECT_SIG_EN_A::DETECT_SIG_EN_0)
    }
    #[doc = "Enable detect signal"]
    #[inline(always)]
    pub fn detect_sig_en_1(self) -> &'a mut W {
        self.variant(DETECT_SIG_EN_A::DETECT_SIG_EN_1)
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
#[doc = "Valid Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_SIG_EN_A {
    #[doc = "0: Disable valid signal"]
    VALID_SIG_EN_0 = 0,
    #[doc = "1: Enable valid signal"]
    VALID_SIG_EN_1 = 1,
}
impl From<VALID_SIG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_SIG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VALID_SIG_EN`"]
pub type VALID_SIG_EN_R = crate::R<bool, VALID_SIG_EN_A>;
impl VALID_SIG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_SIG_EN_A {
        match self.bits {
            false => VALID_SIG_EN_A::VALID_SIG_EN_0,
            true => VALID_SIG_EN_A::VALID_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_SIG_EN_0`"]
    #[inline(always)]
    pub fn is_valid_sig_en_0(&self) -> bool {
        *self == VALID_SIG_EN_A::VALID_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `VALID_SIG_EN_1`"]
    #[inline(always)]
    pub fn is_valid_sig_en_1(&self) -> bool {
        *self == VALID_SIG_EN_A::VALID_SIG_EN_1
    }
}
#[doc = "Write proxy for field `VALID_SIG_EN`"]
pub struct VALID_SIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_SIG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALID_SIG_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable valid signal"]
    #[inline(always)]
    pub fn valid_sig_en_0(self) -> &'a mut W {
        self.variant(VALID_SIG_EN_A::VALID_SIG_EN_0)
    }
    #[doc = "Enable valid signal"]
    #[inline(always)]
    pub fn valid_sig_en_1(self) -> &'a mut W {
        self.variant(VALID_SIG_EN_A::VALID_SIG_EN_1)
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
#[doc = "Idle Software Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_SW_SIG_EN_A {
    #[doc = "0: Disable idle software signal"]
    IDLE_SW_SIG_EN_0 = 0,
    #[doc = "1: Enable idle software signal"]
    IDLE_SW_SIG_EN_1 = 1,
}
impl From<IDLE_SW_SIG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_SW_SIG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLE_SW_SIG_EN`"]
pub type IDLE_SW_SIG_EN_R = crate::R<bool, IDLE_SW_SIG_EN_A>;
impl IDLE_SW_SIG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_SW_SIG_EN_A {
        match self.bits {
            false => IDLE_SW_SIG_EN_A::IDLE_SW_SIG_EN_0,
            true => IDLE_SW_SIG_EN_A::IDLE_SW_SIG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_SIG_EN_0`"]
    #[inline(always)]
    pub fn is_idle_sw_sig_en_0(&self) -> bool {
        *self == IDLE_SW_SIG_EN_A::IDLE_SW_SIG_EN_0
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_SIG_EN_1`"]
    #[inline(always)]
    pub fn is_idle_sw_sig_en_1(&self) -> bool {
        *self == IDLE_SW_SIG_EN_A::IDLE_SW_SIG_EN_1
    }
}
#[doc = "Write proxy for field `IDLE_SW_SIG_EN`"]
pub struct IDLE_SW_SIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_SW_SIG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_SW_SIG_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable idle software signal"]
    #[inline(always)]
    pub fn idle_sw_sig_en_0(self) -> &'a mut W {
        self.variant(IDLE_SW_SIG_EN_A::IDLE_SW_SIG_EN_0)
    }
    #[doc = "Enable idle software signal"]
    #[inline(always)]
    pub fn idle_sw_sig_en_1(self) -> &'a mut W {
        self.variant(IDLE_SW_SIG_EN_A::IDLE_SW_SIG_EN_1)
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
    #[doc = "Bit 0 - Measure Signal Enable"]
    #[inline(always)]
    pub fn measure_sig_en(&self) -> MEASURE_SIG_EN_R {
        MEASURE_SIG_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Detect Signal Enable"]
    #[inline(always)]
    pub fn detect_sig_en(&self) -> DETECT_SIG_EN_R {
        DETECT_SIG_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Valid Signal Enable"]
    #[inline(always)]
    pub fn valid_sig_en(&self) -> VALID_SIG_EN_R {
        VALID_SIG_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Idle Software Signal Enable"]
    #[inline(always)]
    pub fn idle_sw_sig_en(&self) -> IDLE_SW_SIG_EN_R {
        IDLE_SW_SIG_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Measure Signal Enable"]
    #[inline(always)]
    pub fn measure_sig_en(&mut self) -> MEASURE_SIG_EN_W {
        MEASURE_SIG_EN_W { w: self }
    }
    #[doc = "Bit 4 - Detect Signal Enable"]
    #[inline(always)]
    pub fn detect_sig_en(&mut self) -> DETECT_SIG_EN_W {
        DETECT_SIG_EN_W { w: self }
    }
    #[doc = "Bit 8 - Valid Signal Enable"]
    #[inline(always)]
    pub fn valid_sig_en(&mut self) -> VALID_SIG_EN_W {
        VALID_SIG_EN_W { w: self }
    }
    #[doc = "Bit 12 - Idle Software Signal Enable"]
    #[inline(always)]
    pub fn idle_sw_sig_en(&mut self) -> IDLE_SW_SIG_EN_W {
        IDLE_SW_SIG_EN_W { w: self }
    }
}
