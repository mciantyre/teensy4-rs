#[doc = "Reader of register MEGA_SR"]
pub type R = crate::R<u32, super::MEGA_SR>;
#[doc = "Writer for register MEGA_SR"]
pub type W = crate::W<u32, super::MEGA_SR>;
#[doc = "Register MEGA_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::MEGA_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR_A {
    #[doc = "0: The target subsystem was not powered down for the previous power-down request."]
    PSR_0 = 0,
    #[doc = "1: The target subsystem was powered down for the previous power-down request."]
    PSR_1 = 1,
}
impl From<PSR_A> for bool {
    #[inline(always)]
    fn from(variant: PSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSR`"]
pub type PSR_R = crate::R<bool, PSR_A>;
impl PSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSR_A {
        match self.bits {
            false => PSR_A::PSR_0,
            true => PSR_A::PSR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSR_0`"]
    #[inline(always)]
    pub fn is_psr_0(&self) -> bool {
        *self == PSR_A::PSR_0
    }
    #[doc = "Checks if the value of the field is `PSR_1`"]
    #[inline(always)]
    pub fn is_psr_1(&self) -> bool {
        *self == PSR_A::PSR_1
    }
}
#[doc = "Write proxy for field `PSR`"]
pub struct PSR_W<'a> {
    w: &'a mut W,
}
impl<'a> PSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The target subsystem was not powered down for the previous power-down request."]
    #[inline(always)]
    pub fn psr_0(self) -> &'a mut W {
        self.variant(PSR_A::PSR_0)
    }
    #[doc = "The target subsystem was powered down for the previous power-down request."]
    #[inline(always)]
    pub fn psr_1(self) -> &'a mut W {
        self.variant(PSR_A::PSR_1)
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
impl R {
    #[doc = "Bit 0 - Power status"]
    #[inline(always)]
    pub fn psr(&self) -> PSR_R {
        PSR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power status"]
    #[inline(always)]
    pub fn psr(&mut self) -> PSR_W {
        PSR_W { w: self }
    }
}
