#[doc = "Reader of register SMOCTRL"]
pub type R = crate::R<u16, super::SMOCTRL>;
#[doc = "Writer for register SMOCTRL"]
pub type W = crate::W<u16, super::SMOCTRL>;
#[doc = "Register SMOCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SMOCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWM_X Fault State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWMXFS_A {
    #[doc = "0: Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMXFS_0 = 0,
    #[doc = "1: Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMXFS_1 = 1,
    #[doc = "2: Output is tristated."]
    PWMXFS_2 = 2,
    #[doc = "3: Output is tristated."]
    PWMXFS_3 = 3,
}
impl From<PWMXFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMXFS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWMXFS`"]
pub type PWMXFS_R = crate::R<u8, PWMXFS_A>;
impl PWMXFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMXFS_A {
        match self.bits {
            0 => PWMXFS_A::PWMXFS_0,
            1 => PWMXFS_A::PWMXFS_1,
            2 => PWMXFS_A::PWMXFS_2,
            3 => PWMXFS_A::PWMXFS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMXFS_0`"]
    #[inline(always)]
    pub fn is_pwmxfs_0(&self) -> bool {
        *self == PWMXFS_A::PWMXFS_0
    }
    #[doc = "Checks if the value of the field is `PWMXFS_1`"]
    #[inline(always)]
    pub fn is_pwmxfs_1(&self) -> bool {
        *self == PWMXFS_A::PWMXFS_1
    }
    #[doc = "Checks if the value of the field is `PWMXFS_2`"]
    #[inline(always)]
    pub fn is_pwmxfs_2(&self) -> bool {
        *self == PWMXFS_A::PWMXFS_2
    }
    #[doc = "Checks if the value of the field is `PWMXFS_3`"]
    #[inline(always)]
    pub fn is_pwmxfs_3(&self) -> bool {
        *self == PWMXFS_A::PWMXFS_3
    }
}
#[doc = "Write proxy for field `PWMXFS`"]
pub struct PWMXFS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMXFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMXFS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn pwmxfs_0(self) -> &'a mut W {
        self.variant(PWMXFS_A::PWMXFS_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn pwmxfs_1(self) -> &'a mut W {
        self.variant(PWMXFS_A::PWMXFS_1)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn pwmxfs_2(self) -> &'a mut W {
        self.variant(PWMXFS_A::PWMXFS_2)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn pwmxfs_3(self) -> &'a mut W {
        self.variant(PWMXFS_A::PWMXFS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "PWM_B Fault State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWMBFS_A {
    #[doc = "0: Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMBFS_0 = 0,
    #[doc = "1: Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMBFS_1 = 1,
    #[doc = "2: Output is tristated."]
    PWMBFS_2 = 2,
    #[doc = "3: Output is tristated."]
    PWMBFS_3 = 3,
}
impl From<PWMBFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMBFS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWMBFS`"]
pub type PWMBFS_R = crate::R<u8, PWMBFS_A>;
impl PWMBFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMBFS_A {
        match self.bits {
            0 => PWMBFS_A::PWMBFS_0,
            1 => PWMBFS_A::PWMBFS_1,
            2 => PWMBFS_A::PWMBFS_2,
            3 => PWMBFS_A::PWMBFS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMBFS_0`"]
    #[inline(always)]
    pub fn is_pwmbfs_0(&self) -> bool {
        *self == PWMBFS_A::PWMBFS_0
    }
    #[doc = "Checks if the value of the field is `PWMBFS_1`"]
    #[inline(always)]
    pub fn is_pwmbfs_1(&self) -> bool {
        *self == PWMBFS_A::PWMBFS_1
    }
    #[doc = "Checks if the value of the field is `PWMBFS_2`"]
    #[inline(always)]
    pub fn is_pwmbfs_2(&self) -> bool {
        *self == PWMBFS_A::PWMBFS_2
    }
    #[doc = "Checks if the value of the field is `PWMBFS_3`"]
    #[inline(always)]
    pub fn is_pwmbfs_3(&self) -> bool {
        *self == PWMBFS_A::PWMBFS_3
    }
}
#[doc = "Write proxy for field `PWMBFS`"]
pub struct PWMBFS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMBFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMBFS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn pwmbfs_0(self) -> &'a mut W {
        self.variant(PWMBFS_A::PWMBFS_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn pwmbfs_1(self) -> &'a mut W {
        self.variant(PWMBFS_A::PWMBFS_1)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn pwmbfs_2(self) -> &'a mut W {
        self.variant(PWMBFS_A::PWMBFS_2)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn pwmbfs_3(self) -> &'a mut W {
        self.variant(PWMBFS_A::PWMBFS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "PWM_A Fault State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWMAFS_A {
    #[doc = "0: Output is forced to logic 0 state prior to consideration of output polarity control."]
    PWMAFS_0 = 0,
    #[doc = "1: Output is forced to logic 1 state prior to consideration of output polarity control."]
    PWMAFS_1 = 1,
    #[doc = "2: Output is tristated."]
    PWMAFS_2 = 2,
    #[doc = "3: Output is tristated."]
    PWMAFS_3 = 3,
}
impl From<PWMAFS_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMAFS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWMAFS`"]
pub type PWMAFS_R = crate::R<u8, PWMAFS_A>;
impl PWMAFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMAFS_A {
        match self.bits {
            0 => PWMAFS_A::PWMAFS_0,
            1 => PWMAFS_A::PWMAFS_1,
            2 => PWMAFS_A::PWMAFS_2,
            3 => PWMAFS_A::PWMAFS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMAFS_0`"]
    #[inline(always)]
    pub fn is_pwmafs_0(&self) -> bool {
        *self == PWMAFS_A::PWMAFS_0
    }
    #[doc = "Checks if the value of the field is `PWMAFS_1`"]
    #[inline(always)]
    pub fn is_pwmafs_1(&self) -> bool {
        *self == PWMAFS_A::PWMAFS_1
    }
    #[doc = "Checks if the value of the field is `PWMAFS_2`"]
    #[inline(always)]
    pub fn is_pwmafs_2(&self) -> bool {
        *self == PWMAFS_A::PWMAFS_2
    }
    #[doc = "Checks if the value of the field is `PWMAFS_3`"]
    #[inline(always)]
    pub fn is_pwmafs_3(&self) -> bool {
        *self == PWMAFS_A::PWMAFS_3
    }
}
#[doc = "Write proxy for field `PWMAFS`"]
pub struct PWMAFS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMAFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMAFS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn pwmafs_0(self) -> &'a mut W {
        self.variant(PWMAFS_A::PWMAFS_0)
    }
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    #[inline(always)]
    pub fn pwmafs_1(self) -> &'a mut W {
        self.variant(PWMAFS_A::PWMAFS_1)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn pwmafs_2(self) -> &'a mut W {
        self.variant(PWMAFS_A::PWMAFS_2)
    }
    #[doc = "Output is tristated."]
    #[inline(always)]
    pub fn pwmafs_3(self) -> &'a mut W {
        self.variant(PWMAFS_A::PWMAFS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "PWM_X Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLX_A {
    #[doc = "0: PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    POLX_0 = 0,
    #[doc = "1: PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    POLX_1 = 1,
}
impl From<POLX_A> for bool {
    #[inline(always)]
    fn from(variant: POLX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POLX`"]
pub type POLX_R = crate::R<bool, POLX_A>;
impl POLX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLX_A {
        match self.bits {
            false => POLX_A::POLX_0,
            true => POLX_A::POLX_1,
        }
    }
    #[doc = "Checks if the value of the field is `POLX_0`"]
    #[inline(always)]
    pub fn is_polx_0(&self) -> bool {
        *self == POLX_A::POLX_0
    }
    #[doc = "Checks if the value of the field is `POLX_1`"]
    #[inline(always)]
    pub fn is_polx_1(&self) -> bool {
        *self == POLX_A::POLX_1
    }
}
#[doc = "Write proxy for field `POLX`"]
pub struct POLX_W<'a> {
    w: &'a mut W,
}
impl<'a> POLX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn polx_0(self) -> &'a mut W {
        self.variant(POLX_A::POLX_0)
    }
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn polx_1(self) -> &'a mut W {
        self.variant(POLX_A::POLX_1)
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
#[doc = "PWM_B Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLB_A {
    #[doc = "0: PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    POLB_0 = 0,
    #[doc = "1: PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    POLB_1 = 1,
}
impl From<POLB_A> for bool {
    #[inline(always)]
    fn from(variant: POLB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POLB`"]
pub type POLB_R = crate::R<bool, POLB_A>;
impl POLB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLB_A {
        match self.bits {
            false => POLB_A::POLB_0,
            true => POLB_A::POLB_1,
        }
    }
    #[doc = "Checks if the value of the field is `POLB_0`"]
    #[inline(always)]
    pub fn is_polb_0(&self) -> bool {
        *self == POLB_A::POLB_0
    }
    #[doc = "Checks if the value of the field is `POLB_1`"]
    #[inline(always)]
    pub fn is_polb_1(&self) -> bool {
        *self == POLB_A::POLB_1
    }
}
#[doc = "Write proxy for field `POLB`"]
pub struct POLB_W<'a> {
    w: &'a mut W,
}
impl<'a> POLB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn polb_0(self) -> &'a mut W {
        self.variant(POLB_A::POLB_0)
    }
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn polb_1(self) -> &'a mut W {
        self.variant(POLB_A::POLB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "PWM_A Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLA_A {
    #[doc = "0: PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    POLA_0 = 0,
    #[doc = "1: PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    POLA_1 = 1,
}
impl From<POLA_A> for bool {
    #[inline(always)]
    fn from(variant: POLA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POLA`"]
pub type POLA_R = crate::R<bool, POLA_A>;
impl POLA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLA_A {
        match self.bits {
            false => POLA_A::POLA_0,
            true => POLA_A::POLA_1,
        }
    }
    #[doc = "Checks if the value of the field is `POLA_0`"]
    #[inline(always)]
    pub fn is_pola_0(&self) -> bool {
        *self == POLA_A::POLA_0
    }
    #[doc = "Checks if the value of the field is `POLA_1`"]
    #[inline(always)]
    pub fn is_pola_1(&self) -> bool {
        *self == POLA_A::POLA_1
    }
}
#[doc = "Write proxy for field `POLA`"]
pub struct POLA_W<'a> {
    w: &'a mut W,
}
impl<'a> POLA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn pola_0(self) -> &'a mut W {
        self.variant(POLA_A::POLA_0)
    }
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    #[inline(always)]
    pub fn pola_1(self) -> &'a mut W {
        self.variant(POLA_A::POLA_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PWMX_IN`"]
pub type PWMX_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMB_IN`"]
pub type PWMB_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWMA_IN`"]
pub type PWMA_IN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - PWM_X Fault State"]
    #[inline(always)]
    pub fn pwmxfs(&self) -> PWMXFS_R {
        PWMXFS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - PWM_B Fault State"]
    #[inline(always)]
    pub fn pwmbfs(&self) -> PWMBFS_R {
        PWMBFS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PWM_A Fault State"]
    #[inline(always)]
    pub fn pwmafs(&self) -> PWMAFS_R {
        PWMAFS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - PWM_X Output Polarity"]
    #[inline(always)]
    pub fn polx(&self) -> POLX_R {
        POLX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWM_B Output Polarity"]
    #[inline(always)]
    pub fn polb(&self) -> POLB_R {
        POLB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM_A Output Polarity"]
    #[inline(always)]
    pub fn pola(&self) -> POLA_R {
        POLA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PWM_X Input"]
    #[inline(always)]
    pub fn pwmx_in(&self) -> PWMX_IN_R {
        PWMX_IN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PWM_B Input"]
    #[inline(always)]
    pub fn pwmb_in(&self) -> PWMB_IN_R {
        PWMB_IN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PWM_A Input"]
    #[inline(always)]
    pub fn pwma_in(&self) -> PWMA_IN_R {
        PWMA_IN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM_X Fault State"]
    #[inline(always)]
    pub fn pwmxfs(&mut self) -> PWMXFS_W {
        PWMXFS_W { w: self }
    }
    #[doc = "Bits 2:3 - PWM_B Fault State"]
    #[inline(always)]
    pub fn pwmbfs(&mut self) -> PWMBFS_W {
        PWMBFS_W { w: self }
    }
    #[doc = "Bits 4:5 - PWM_A Fault State"]
    #[inline(always)]
    pub fn pwmafs(&mut self) -> PWMAFS_W {
        PWMAFS_W { w: self }
    }
    #[doc = "Bit 8 - PWM_X Output Polarity"]
    #[inline(always)]
    pub fn polx(&mut self) -> POLX_W {
        POLX_W { w: self }
    }
    #[doc = "Bit 9 - PWM_B Output Polarity"]
    #[inline(always)]
    pub fn polb(&mut self) -> POLB_W {
        POLB_W { w: self }
    }
    #[doc = "Bit 10 - PWM_A Output Polarity"]
    #[inline(always)]
    pub fn pola(&mut self) -> POLA_W {
        POLA_W { w: self }
    }
}
