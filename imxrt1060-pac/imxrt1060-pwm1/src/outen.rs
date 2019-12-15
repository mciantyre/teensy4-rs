#[doc = "Reader of register OUTEN"]
pub type R = crate::R<u16, super::OUTEN>;
#[doc = "Writer for register OUTEN"]
pub type W = crate::W<u16, super::OUTEN>;
#[doc = "Register OUTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWM_X Output Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWMX_EN_A {
    #[doc = "0: PWM_X output disabled."]
    PWMX_EN_0 = 0,
    #[doc = "1: PWM_X output enabled."]
    PWMX_EN_1 = 1,
}
impl From<PWMX_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMX_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWMX_EN`"]
pub type PWMX_EN_R = crate::R<u8, PWMX_EN_A>;
impl PWMX_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWMX_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWMX_EN_A::PWMX_EN_0),
            1 => Val(PWMX_EN_A::PWMX_EN_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWMX_EN_0`"]
    #[inline(always)]
    pub fn is_pwmx_en_0(&self) -> bool {
        *self == PWMX_EN_A::PWMX_EN_0
    }
    #[doc = "Checks if the value of the field is `PWMX_EN_1`"]
    #[inline(always)]
    pub fn is_pwmx_en_1(&self) -> bool {
        *self == PWMX_EN_A::PWMX_EN_1
    }
}
#[doc = "Write proxy for field `PWMX_EN`"]
pub struct PWMX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMX_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMX_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM_X output disabled."]
    #[inline(always)]
    pub fn pwmx_en_0(self) -> &'a mut W {
        self.variant(PWMX_EN_A::PWMX_EN_0)
    }
    #[doc = "PWM_X output enabled."]
    #[inline(always)]
    pub fn pwmx_en_1(self) -> &'a mut W {
        self.variant(PWMX_EN_A::PWMX_EN_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "PWM_B Output Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWMB_EN_A {
    #[doc = "0: PWM_B output disabled."]
    PWMB_EN_0 = 0,
    #[doc = "1: PWM_B output enabled."]
    PWMB_EN_1 = 1,
}
impl From<PWMB_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMB_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWMB_EN`"]
pub type PWMB_EN_R = crate::R<u8, PWMB_EN_A>;
impl PWMB_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWMB_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWMB_EN_A::PWMB_EN_0),
            1 => Val(PWMB_EN_A::PWMB_EN_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWMB_EN_0`"]
    #[inline(always)]
    pub fn is_pwmb_en_0(&self) -> bool {
        *self == PWMB_EN_A::PWMB_EN_0
    }
    #[doc = "Checks if the value of the field is `PWMB_EN_1`"]
    #[inline(always)]
    pub fn is_pwmb_en_1(&self) -> bool {
        *self == PWMB_EN_A::PWMB_EN_1
    }
}
#[doc = "Write proxy for field `PWMB_EN`"]
pub struct PWMB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMB_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMB_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM_B output disabled."]
    #[inline(always)]
    pub fn pwmb_en_0(self) -> &'a mut W {
        self.variant(PWMB_EN_A::PWMB_EN_0)
    }
    #[doc = "PWM_B output enabled."]
    #[inline(always)]
    pub fn pwmb_en_1(self) -> &'a mut W {
        self.variant(PWMB_EN_A::PWMB_EN_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "PWM_A Output Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWMA_EN_A {
    #[doc = "0: PWM_A output disabled."]
    PWMA_EN_0 = 0,
    #[doc = "1: PWM_A output enabled."]
    PWMA_EN_1 = 1,
}
impl From<PWMA_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMA_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWMA_EN`"]
pub type PWMA_EN_R = crate::R<u8, PWMA_EN_A>;
impl PWMA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWMA_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWMA_EN_A::PWMA_EN_0),
            1 => Val(PWMA_EN_A::PWMA_EN_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWMA_EN_0`"]
    #[inline(always)]
    pub fn is_pwma_en_0(&self) -> bool {
        *self == PWMA_EN_A::PWMA_EN_0
    }
    #[doc = "Checks if the value of the field is `PWMA_EN_1`"]
    #[inline(always)]
    pub fn is_pwma_en_1(&self) -> bool {
        *self == PWMA_EN_A::PWMA_EN_1
    }
}
#[doc = "Write proxy for field `PWMA_EN`"]
pub struct PWMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMA_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM_A output disabled."]
    #[inline(always)]
    pub fn pwma_en_0(self) -> &'a mut W {
        self.variant(PWMA_EN_A::PWMA_EN_0)
    }
    #[doc = "PWM_A output enabled."]
    #[inline(always)]
    pub fn pwma_en_1(self) -> &'a mut W {
        self.variant(PWMA_EN_A::PWMA_EN_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM_X Output Enables"]
    #[inline(always)]
    pub fn pwmx_en(&self) -> PWMX_EN_R {
        PWMX_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PWM_B Output Enables"]
    #[inline(always)]
    pub fn pwmb_en(&self) -> PWMB_EN_R {
        PWMB_EN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PWM_A Output Enables"]
    #[inline(always)]
    pub fn pwma_en(&self) -> PWMA_EN_R {
        PWMA_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_X Output Enables"]
    #[inline(always)]
    pub fn pwmx_en(&mut self) -> PWMX_EN_W {
        PWMX_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - PWM_B Output Enables"]
    #[inline(always)]
    pub fn pwmb_en(&mut self) -> PWMB_EN_W {
        PWMB_EN_W { w: self }
    }
    #[doc = "Bits 8:11 - PWM_A Output Enables"]
    #[inline(always)]
    pub fn pwma_en(&mut self) -> PWMA_EN_W {
        PWMA_EN_W { w: self }
    }
}
