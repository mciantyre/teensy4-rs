#[doc = "Reader of register SM0TCTRL"]
pub type R = crate::R<u16, super::SM0TCTRL>;
#[doc = "Writer for register SM0TCTRL"]
pub type W = crate::W<u16, super::SM0TCTRL>;
#[doc = "Register SM0TCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0TCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output Trigger Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT_TRIG_EN_A {
    #[doc = "0: PWM_OUT_TRIGx will not set when the counter value matches the VALx value."]
    OUT_TRIG_EN_0,
    #[doc = "1: PWM_OUT_TRIGx will set when the counter value matches the VALx value."]
    OUT_TRIG_EN_1,
}
impl From<OUT_TRIG_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_TRIG_EN_A) -> Self {
        match variant {
            OUT_TRIG_EN_A::OUT_TRIG_EN_0 => 0,
            OUT_TRIG_EN_A::OUT_TRIG_EN_1 => 1,
        }
    }
}
#[doc = "Reader of field `OUT_TRIG_EN`"]
pub type OUT_TRIG_EN_R = crate::R<u8, OUT_TRIG_EN_A>;
impl OUT_TRIG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OUT_TRIG_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OUT_TRIG_EN_A::OUT_TRIG_EN_0),
            1 => Val(OUT_TRIG_EN_A::OUT_TRIG_EN_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT_TRIG_EN_0`"]
    #[inline(always)]
    pub fn is_out_trig_en_0(&self) -> bool {
        *self == OUT_TRIG_EN_A::OUT_TRIG_EN_0
    }
    #[doc = "Checks if the value of the field is `OUT_TRIG_EN_1`"]
    #[inline(always)]
    pub fn is_out_trig_en_1(&self) -> bool {
        *self == OUT_TRIG_EN_A::OUT_TRIG_EN_1
    }
}
#[doc = "Write proxy for field `OUT_TRIG_EN`"]
pub struct OUT_TRIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TRIG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT_TRIG_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM_OUT_TRIGx will not set when the counter value matches the VALx value."]
    #[inline(always)]
    pub fn out_trig_en_0(self) -> &'a mut W {
        self.variant(OUT_TRIG_EN_A::OUT_TRIG_EN_0)
    }
    #[doc = "PWM_OUT_TRIGx will set when the counter value matches the VALx value."]
    #[inline(always)]
    pub fn out_trig_en_1(self) -> &'a mut W {
        self.variant(OUT_TRIG_EN_A::OUT_TRIG_EN_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Trigger frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGFRQ_A {
    #[doc = "0: Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    TRGFRQ_0,
    #[doc = "1: Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    TRGFRQ_1,
}
impl From<TRGFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRGFRQ_A) -> Self {
        match variant {
            TRGFRQ_A::TRGFRQ_0 => false,
            TRGFRQ_A::TRGFRQ_1 => true,
        }
    }
}
#[doc = "Reader of field `TRGFRQ`"]
pub type TRGFRQ_R = crate::R<bool, TRGFRQ_A>;
impl TRGFRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGFRQ_A {
        match self.bits {
            false => TRGFRQ_A::TRGFRQ_0,
            true => TRGFRQ_A::TRGFRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRGFRQ_0`"]
    #[inline(always)]
    pub fn is_trgfrq_0(&self) -> bool {
        *self == TRGFRQ_A::TRGFRQ_0
    }
    #[doc = "Checks if the value of the field is `TRGFRQ_1`"]
    #[inline(always)]
    pub fn is_trgfrq_1(&self) -> bool {
        *self == TRGFRQ_A::TRGFRQ_1
    }
}
#[doc = "Write proxy for field `TRGFRQ`"]
pub struct TRGFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGFRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGFRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    #[inline(always)]
    pub fn trgfrq_0(self) -> &'a mut W {
        self.variant(TRGFRQ_A::TRGFRQ_0)
    }
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    #[inline(always)]
    pub fn trgfrq_1(self) -> &'a mut W {
        self.variant(TRGFRQ_A::TRGFRQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Output Trigger 1 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWBOT1_A {
    #[doc = "0: Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port."]
    PWBOT1_0,
    #[doc = "1: Route the PWMB output to the PWM_OUT_TRIG1 port."]
    PWBOT1_1,
}
impl From<PWBOT1_A> for bool {
    #[inline(always)]
    fn from(variant: PWBOT1_A) -> Self {
        match variant {
            PWBOT1_A::PWBOT1_0 => false,
            PWBOT1_A::PWBOT1_1 => true,
        }
    }
}
#[doc = "Reader of field `PWBOT1`"]
pub type PWBOT1_R = crate::R<bool, PWBOT1_A>;
impl PWBOT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWBOT1_A {
        match self.bits {
            false => PWBOT1_A::PWBOT1_0,
            true => PWBOT1_A::PWBOT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWBOT1_0`"]
    #[inline(always)]
    pub fn is_pwbot1_0(&self) -> bool {
        *self == PWBOT1_A::PWBOT1_0
    }
    #[doc = "Checks if the value of the field is `PWBOT1_1`"]
    #[inline(always)]
    pub fn is_pwbot1_1(&self) -> bool {
        *self == PWBOT1_A::PWBOT1_1
    }
}
#[doc = "Write proxy for field `PWBOT1`"]
pub struct PWBOT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWBOT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWBOT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port."]
    #[inline(always)]
    pub fn pwbot1_0(self) -> &'a mut W {
        self.variant(PWBOT1_A::PWBOT1_0)
    }
    #[doc = "Route the PWMB output to the PWM_OUT_TRIG1 port."]
    #[inline(always)]
    pub fn pwbot1_1(self) -> &'a mut W {
        self.variant(PWBOT1_A::PWBOT1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Output Trigger 0 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWAOT0_A {
    #[doc = "0: Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port."]
    PWAOT0_0,
    #[doc = "1: Route the PWMA output to the PWM_OUT_TRIG0 port."]
    PWAOT0_1,
}
impl From<PWAOT0_A> for bool {
    #[inline(always)]
    fn from(variant: PWAOT0_A) -> Self {
        match variant {
            PWAOT0_A::PWAOT0_0 => false,
            PWAOT0_A::PWAOT0_1 => true,
        }
    }
}
#[doc = "Reader of field `PWAOT0`"]
pub type PWAOT0_R = crate::R<bool, PWAOT0_A>;
impl PWAOT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWAOT0_A {
        match self.bits {
            false => PWAOT0_A::PWAOT0_0,
            true => PWAOT0_A::PWAOT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWAOT0_0`"]
    #[inline(always)]
    pub fn is_pwaot0_0(&self) -> bool {
        *self == PWAOT0_A::PWAOT0_0
    }
    #[doc = "Checks if the value of the field is `PWAOT0_1`"]
    #[inline(always)]
    pub fn is_pwaot0_1(&self) -> bool {
        *self == PWAOT0_A::PWAOT0_1
    }
}
#[doc = "Write proxy for field `PWAOT0`"]
pub struct PWAOT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWAOT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWAOT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port."]
    #[inline(always)]
    pub fn pwaot0_0(self) -> &'a mut W {
        self.variant(PWAOT0_A::PWAOT0_0)
    }
    #[doc = "Route the PWMA output to the PWM_OUT_TRIG0 port."]
    #[inline(always)]
    pub fn pwaot0_1(self) -> &'a mut W {
        self.variant(PWAOT0_A::PWAOT0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Output Trigger Enables"]
    #[inline(always)]
    pub fn out_trig_en(&self) -> OUT_TRIG_EN_R {
        OUT_TRIG_EN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Trigger frequency"]
    #[inline(always)]
    pub fn trgfrq(&self) -> TRGFRQ_R {
        TRGFRQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Output Trigger 1 Source Select"]
    #[inline(always)]
    pub fn pwbot1(&self) -> PWBOT1_R {
        PWBOT1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Output Trigger 0 Source Select"]
    #[inline(always)]
    pub fn pwaot0(&self) -> PWAOT0_R {
        PWAOT0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Output Trigger Enables"]
    #[inline(always)]
    pub fn out_trig_en(&mut self) -> OUT_TRIG_EN_W {
        OUT_TRIG_EN_W { w: self }
    }
    #[doc = "Bit 12 - Trigger frequency"]
    #[inline(always)]
    pub fn trgfrq(&mut self) -> TRGFRQ_W {
        TRGFRQ_W { w: self }
    }
    #[doc = "Bit 14 - Output Trigger 1 Source Select"]
    #[inline(always)]
    pub fn pwbot1(&mut self) -> PWBOT1_W {
        PWBOT1_W { w: self }
    }
    #[doc = "Bit 15 - Output Trigger 0 Source Select"]
    #[inline(always)]
    pub fn pwaot0(&mut self) -> PWAOT0_W {
        PWAOT0_W { w: self }
    }
}
