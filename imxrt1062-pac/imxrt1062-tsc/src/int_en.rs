#[doc = "Reader of register INT_EN"]
pub type R = crate::R<u32, super::INT_EN>;
#[doc = "Writer for register INT_EN"]
pub type W = crate::W<u32, super::INT_EN>;
#[doc = "Register INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Measure Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEASURE_INT_EN_A {
    #[doc = "0: Disable measure interrupt"]
    MEASURE_INT_EN_0 = 0,
    #[doc = "1: Enable measure interrupt"]
    MEASURE_INT_EN_1 = 1,
}
impl From<MEASURE_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MEASURE_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEASURE_INT_EN`"]
pub type MEASURE_INT_EN_R = crate::R<bool, MEASURE_INT_EN_A>;
impl MEASURE_INT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEASURE_INT_EN_A {
        match self.bits {
            false => MEASURE_INT_EN_A::MEASURE_INT_EN_0,
            true => MEASURE_INT_EN_A::MEASURE_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEASURE_INT_EN_0`"]
    #[inline(always)]
    pub fn is_measure_int_en_0(&self) -> bool {
        *self == MEASURE_INT_EN_A::MEASURE_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `MEASURE_INT_EN_1`"]
    #[inline(always)]
    pub fn is_measure_int_en_1(&self) -> bool {
        *self == MEASURE_INT_EN_A::MEASURE_INT_EN_1
    }
}
#[doc = "Write proxy for field `MEASURE_INT_EN`"]
pub struct MEASURE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEASURE_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEASURE_INT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable measure interrupt"]
    #[inline(always)]
    pub fn measure_int_en_0(self) -> &'a mut W {
        self.variant(MEASURE_INT_EN_A::MEASURE_INT_EN_0)
    }
    #[doc = "Enable measure interrupt"]
    #[inline(always)]
    pub fn measure_int_en_1(self) -> &'a mut W {
        self.variant(MEASURE_INT_EN_A::MEASURE_INT_EN_1)
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
#[doc = "Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_INT_EN_A {
    #[doc = "0: Disable detect interrupt"]
    DETECT_INT_EN_0 = 0,
    #[doc = "1: Enable detect interrupt"]
    DETECT_INT_EN_1 = 1,
}
impl From<DETECT_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DETECT_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETECT_INT_EN`"]
pub type DETECT_INT_EN_R = crate::R<bool, DETECT_INT_EN_A>;
impl DETECT_INT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECT_INT_EN_A {
        match self.bits {
            false => DETECT_INT_EN_A::DETECT_INT_EN_0,
            true => DETECT_INT_EN_A::DETECT_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_INT_EN_0`"]
    #[inline(always)]
    pub fn is_detect_int_en_0(&self) -> bool {
        *self == DETECT_INT_EN_A::DETECT_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `DETECT_INT_EN_1`"]
    #[inline(always)]
    pub fn is_detect_int_en_1(&self) -> bool {
        *self == DETECT_INT_EN_A::DETECT_INT_EN_1
    }
}
#[doc = "Write proxy for field `DETECT_INT_EN`"]
pub struct DETECT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECT_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DETECT_INT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable detect interrupt"]
    #[inline(always)]
    pub fn detect_int_en_0(self) -> &'a mut W {
        self.variant(DETECT_INT_EN_A::DETECT_INT_EN_0)
    }
    #[doc = "Enable detect interrupt"]
    #[inline(always)]
    pub fn detect_int_en_1(self) -> &'a mut W {
        self.variant(DETECT_INT_EN_A::DETECT_INT_EN_1)
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
#[doc = "Idle Software Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_SW_INT_EN_A {
    #[doc = "0: Disable idle software interrupt"]
    IDLE_SW_INT_EN_0 = 0,
    #[doc = "1: Enable idle software interrupt"]
    IDLE_SW_INT_EN_1 = 1,
}
impl From<IDLE_SW_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_SW_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLE_SW_INT_EN`"]
pub type IDLE_SW_INT_EN_R = crate::R<bool, IDLE_SW_INT_EN_A>;
impl IDLE_SW_INT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_SW_INT_EN_A {
        match self.bits {
            false => IDLE_SW_INT_EN_A::IDLE_SW_INT_EN_0,
            true => IDLE_SW_INT_EN_A::IDLE_SW_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_INT_EN_0`"]
    #[inline(always)]
    pub fn is_idle_sw_int_en_0(&self) -> bool {
        *self == IDLE_SW_INT_EN_A::IDLE_SW_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `IDLE_SW_INT_EN_1`"]
    #[inline(always)]
    pub fn is_idle_sw_int_en_1(&self) -> bool {
        *self == IDLE_SW_INT_EN_A::IDLE_SW_INT_EN_1
    }
}
#[doc = "Write proxy for field `IDLE_SW_INT_EN`"]
pub struct IDLE_SW_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_SW_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_SW_INT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable idle software interrupt"]
    #[inline(always)]
    pub fn idle_sw_int_en_0(self) -> &'a mut W {
        self.variant(IDLE_SW_INT_EN_A::IDLE_SW_INT_EN_0)
    }
    #[doc = "Enable idle software interrupt"]
    #[inline(always)]
    pub fn idle_sw_int_en_1(self) -> &'a mut W {
        self.variant(IDLE_SW_INT_EN_A::IDLE_SW_INT_EN_1)
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
    #[doc = "Bit 0 - Measure Interrupt Enable"]
    #[inline(always)]
    pub fn measure_int_en(&self) -> MEASURE_INT_EN_R {
        MEASURE_INT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Detect Interrupt Enable"]
    #[inline(always)]
    pub fn detect_int_en(&self) -> DETECT_INT_EN_R {
        DETECT_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Idle Software Interrupt Enable"]
    #[inline(always)]
    pub fn idle_sw_int_en(&self) -> IDLE_SW_INT_EN_R {
        IDLE_SW_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Measure Interrupt Enable"]
    #[inline(always)]
    pub fn measure_int_en(&mut self) -> MEASURE_INT_EN_W {
        MEASURE_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Detect Interrupt Enable"]
    #[inline(always)]
    pub fn detect_int_en(&mut self) -> DETECT_INT_EN_W {
        DETECT_INT_EN_W { w: self }
    }
    #[doc = "Bit 12 - Idle Software Interrupt Enable"]
    #[inline(always)]
    pub fn idle_sw_int_en(&mut self) -> IDLE_SW_INT_EN_W {
        IDLE_SW_INT_EN_W { w: self }
    }
}
