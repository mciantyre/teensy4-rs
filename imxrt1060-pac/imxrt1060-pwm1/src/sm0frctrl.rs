#[doc = "Reader of register SM0FRCTRL"]
pub type R = crate::R<u16, super::SM0FRCTRL>;
#[doc = "Writer for register SM0FRCTRL"]
pub type W = crate::W<u16, super::SM0FRCTRL>;
#[doc = "Register SM0FRCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0FRCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fractional Cycle PWM Period Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC1_EN_A {
    #[doc = "0: Disable fractional cycle length for the PWM period."]
    FRAC1_EN_0,
    #[doc = "1: Enable fractional cycle length for the PWM period."]
    FRAC1_EN_1,
}
impl From<FRAC1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC1_EN_A) -> Self {
        match variant {
            FRAC1_EN_A::FRAC1_EN_0 => false,
            FRAC1_EN_A::FRAC1_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `FRAC1_EN`"]
pub type FRAC1_EN_R = crate::R<bool, FRAC1_EN_A>;
impl FRAC1_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAC1_EN_A {
        match self.bits {
            false => FRAC1_EN_A::FRAC1_EN_0,
            true => FRAC1_EN_A::FRAC1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC1_EN_0`"]
    #[inline(always)]
    pub fn is_frac1_en_0(&self) -> bool {
        *self == FRAC1_EN_A::FRAC1_EN_0
    }
    #[doc = "Checks if the value of the field is `FRAC1_EN_1`"]
    #[inline(always)]
    pub fn is_frac1_en_1(&self) -> bool {
        *self == FRAC1_EN_A::FRAC1_EN_1
    }
}
#[doc = "Write proxy for field `FRAC1_EN`"]
pub struct FRAC1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAC1_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable fractional cycle length for the PWM period."]
    #[inline(always)]
    pub fn frac1_en_0(self) -> &'a mut W {
        self.variant(FRAC1_EN_A::FRAC1_EN_0)
    }
    #[doc = "Enable fractional cycle length for the PWM period."]
    #[inline(always)]
    pub fn frac1_en_1(self) -> &'a mut W {
        self.variant(FRAC1_EN_A::FRAC1_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Fractional Cycle Placement Enable for PWM_A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC23_EN_A {
    #[doc = "0: Disable fractional cycle placement for PWM_A."]
    FRAC23_EN_0,
    #[doc = "1: Enable fractional cycle placement for PWM_A."]
    FRAC23_EN_1,
}
impl From<FRAC23_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC23_EN_A) -> Self {
        match variant {
            FRAC23_EN_A::FRAC23_EN_0 => false,
            FRAC23_EN_A::FRAC23_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `FRAC23_EN`"]
pub type FRAC23_EN_R = crate::R<bool, FRAC23_EN_A>;
impl FRAC23_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAC23_EN_A {
        match self.bits {
            false => FRAC23_EN_A::FRAC23_EN_0,
            true => FRAC23_EN_A::FRAC23_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC23_EN_0`"]
    #[inline(always)]
    pub fn is_frac23_en_0(&self) -> bool {
        *self == FRAC23_EN_A::FRAC23_EN_0
    }
    #[doc = "Checks if the value of the field is `FRAC23_EN_1`"]
    #[inline(always)]
    pub fn is_frac23_en_1(&self) -> bool {
        *self == FRAC23_EN_A::FRAC23_EN_1
    }
}
#[doc = "Write proxy for field `FRAC23_EN`"]
pub struct FRAC23_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC23_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAC23_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable fractional cycle placement for PWM_A."]
    #[inline(always)]
    pub fn frac23_en_0(self) -> &'a mut W {
        self.variant(FRAC23_EN_A::FRAC23_EN_0)
    }
    #[doc = "Enable fractional cycle placement for PWM_A."]
    #[inline(always)]
    pub fn frac23_en_1(self) -> &'a mut W {
        self.variant(FRAC23_EN_A::FRAC23_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Fractional Cycle Placement Enable for PWM_B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC45_EN_A {
    #[doc = "0: Disable fractional cycle placement for PWM_B."]
    FRAC45_EN_0,
    #[doc = "1: Enable fractional cycle placement for PWM_B."]
    FRAC45_EN_1,
}
impl From<FRAC45_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC45_EN_A) -> Self {
        match variant {
            FRAC45_EN_A::FRAC45_EN_0 => false,
            FRAC45_EN_A::FRAC45_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `FRAC45_EN`"]
pub type FRAC45_EN_R = crate::R<bool, FRAC45_EN_A>;
impl FRAC45_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAC45_EN_A {
        match self.bits {
            false => FRAC45_EN_A::FRAC45_EN_0,
            true => FRAC45_EN_A::FRAC45_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC45_EN_0`"]
    #[inline(always)]
    pub fn is_frac45_en_0(&self) -> bool {
        *self == FRAC45_EN_A::FRAC45_EN_0
    }
    #[doc = "Checks if the value of the field is `FRAC45_EN_1`"]
    #[inline(always)]
    pub fn is_frac45_en_1(&self) -> bool {
        *self == FRAC45_EN_A::FRAC45_EN_1
    }
}
#[doc = "Write proxy for field `FRAC45_EN`"]
pub struct FRAC45_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC45_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAC45_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable fractional cycle placement for PWM_B."]
    #[inline(always)]
    pub fn frac45_en_0(self) -> &'a mut W {
        self.variant(FRAC45_EN_A::FRAC45_EN_0)
    }
    #[doc = "Enable fractional cycle placement for PWM_B."]
    #[inline(always)]
    pub fn frac45_en_1(self) -> &'a mut W {
        self.variant(FRAC45_EN_A::FRAC45_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Fractional Delay Circuit Power Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAC_PU_A {
    #[doc = "0: Turn off fractional delay logic."]
    FRAC_PU_0,
    #[doc = "1: Power up fractional delay logic."]
    FRAC_PU_1,
}
impl From<FRAC_PU_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC_PU_A) -> Self {
        match variant {
            FRAC_PU_A::FRAC_PU_0 => false,
            FRAC_PU_A::FRAC_PU_1 => true,
        }
    }
}
#[doc = "Reader of field `FRAC_PU`"]
pub type FRAC_PU_R = crate::R<bool, FRAC_PU_A>;
impl FRAC_PU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAC_PU_A {
        match self.bits {
            false => FRAC_PU_A::FRAC_PU_0,
            true => FRAC_PU_A::FRAC_PU_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC_PU_0`"]
    #[inline(always)]
    pub fn is_frac_pu_0(&self) -> bool {
        *self == FRAC_PU_A::FRAC_PU_0
    }
    #[doc = "Checks if the value of the field is `FRAC_PU_1`"]
    #[inline(always)]
    pub fn is_frac_pu_1(&self) -> bool {
        *self == FRAC_PU_A::FRAC_PU_1
    }
}
#[doc = "Write proxy for field `FRAC_PU`"]
pub struct FRAC_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAC_PU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Turn off fractional delay logic."]
    #[inline(always)]
    pub fn frac_pu_0(self) -> &'a mut W {
        self.variant(FRAC_PU_A::FRAC_PU_0)
    }
    #[doc = "Power up fractional delay logic."]
    #[inline(always)]
    pub fn frac_pu_1(self) -> &'a mut W {
        self.variant(FRAC_PU_A::FRAC_PU_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TEST`"]
pub type TEST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub fn frac1_en(&self) -> FRAC1_EN_R {
        FRAC1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub fn frac23_en(&self) -> FRAC23_EN_R {
        FRAC23_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub fn frac45_en(&self) -> FRAC45_EN_R {
        FRAC45_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fractional Delay Circuit Power Up"]
    #[inline(always)]
    pub fn frac_pu(&self) -> FRAC_PU_R {
        FRAC_PU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Test Status Bit"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub fn frac1_en(&mut self) -> FRAC1_EN_W {
        FRAC1_EN_W { w: self }
    }
    #[doc = "Bit 2 - Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub fn frac23_en(&mut self) -> FRAC23_EN_W {
        FRAC23_EN_W { w: self }
    }
    #[doc = "Bit 4 - Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub fn frac45_en(&mut self) -> FRAC45_EN_W {
        FRAC45_EN_W { w: self }
    }
    #[doc = "Bit 8 - Fractional Delay Circuit Power Up"]
    #[inline(always)]
    pub fn frac_pu(&mut self) -> FRAC_PU_W {
        FRAC_PU_W { w: self }
    }
}
