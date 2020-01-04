#[doc = "Reader of register CPU_CTRL"]
pub type R = crate::R<u32, super::CPU_CTRL>;
#[doc = "Writer for register CPU_CTRL"]
pub type W = crate::W<u32, super::CPU_CTRL>;
#[doc = "Register CPU_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CPU_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCR_A {
    #[doc = "0: Do not switch off power even if pdn_req is asserted."]
    PCR_0 = 0,
    #[doc = "1: Switch off power when pdn_req is asserted."]
    PCR_1 = 1,
}
impl From<PCR_A> for bool {
    #[inline(always)]
    fn from(variant: PCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PCR`"]
pub type PCR_R = crate::R<bool, PCR_A>;
impl PCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCR_A {
        match self.bits {
            false => PCR_A::PCR_0,
            true => PCR_A::PCR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCR_0`"]
    #[inline(always)]
    pub fn is_pcr_0(&self) -> bool {
        *self == PCR_A::PCR_0
    }
    #[doc = "Checks if the value of the field is `PCR_1`"]
    #[inline(always)]
    pub fn is_pcr_1(&self) -> bool {
        *self == PCR_A::PCR_1
    }
}
#[doc = "Write proxy for field `PCR`"]
pub struct PCR_W<'a> {
    w: &'a mut W,
}
impl<'a> PCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not switch off power even if pdn_req is asserted."]
    #[inline(always)]
    pub fn pcr_0(self) -> &'a mut W {
        self.variant(PCR_A::PCR_0)
    }
    #[doc = "Switch off power when pdn_req is asserted."]
    #[inline(always)]
    pub fn pcr_1(self) -> &'a mut W {
        self.variant(PCR_A::PCR_1)
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
    #[doc = "Bit 0 - Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub fn pcr(&mut self) -> PCR_W {
        PCR_W { w: self }
    }
}
