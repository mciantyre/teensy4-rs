#[doc = "Reader of register SW_MUX_CTL_PAD_PMIC_ON_REQ"]
pub type R = crate::R<u32, super::SW_MUX_CTL_PAD_PMIC_ON_REQ>;
#[doc = "Writer for register SW_MUX_CTL_PAD_PMIC_ON_REQ"]
pub type W = crate::W<u32, super::SW_MUX_CTL_PAD_PMIC_ON_REQ>;
#[doc = "Register SW_MUX_CTL_PAD_PMIC_ON_REQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_MUX_CTL_PAD_PMIC_ON_REQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MUX Mode Select Field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_MODE_A {
    #[doc = "0: Select mux mode: ALT0 mux port: SNVS_LP_PMIC_ON_REQ of instance: snvs_lp"]
    ALT0 = 0,
    #[doc = "5: Select mux mode: ALT5 mux port: GPIO5_IO01 of instance: gpio5"]
    ALT5 = 5,
}
impl From<MUX_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUX_MODE`"]
pub type MUX_MODE_R = crate::R<u8, MUX_MODE_A>;
impl MUX_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUX_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUX_MODE_A::ALT0),
            5 => Val(MUX_MODE_A::ALT5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == MUX_MODE_A::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == MUX_MODE_A::ALT5
    }
}
#[doc = "Write proxy for field `MUX_MODE`"]
pub struct MUX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select mux mode: ALT0 mux port: SNVS_LP_PMIC_ON_REQ of instance: snvs_lp"]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT0)
    }
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO01 of instance: gpio5"]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Software Input On Field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SION_A {
    #[doc = "0: Input Path is determined by functionality"]
    DISABLED = 0,
    #[doc = "1: Force input path of pad PMIC_ON_REQ"]
    ENABLED = 1,
}
impl From<SION_A> for bool {
    #[inline(always)]
    fn from(variant: SION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SION`"]
pub type SION_R = crate::R<bool, SION_A>;
impl SION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SION_A {
        match self.bits {
            false => SION_A::DISABLED,
            true => SION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SION_A::ENABLED
    }
}
#[doc = "Write proxy for field `SION`"]
pub struct SION_W<'a> {
    w: &'a mut W,
}
impl<'a> SION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input Path is determined by functionality"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SION_A::DISABLED)
    }
    #[doc = "Force input path of pad PMIC_ON_REQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SION_A::ENABLED)
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
impl R {
    #[doc = "Bits 0:2 - MUX Mode Select Field."]
    #[inline(always)]
    pub fn mux_mode(&self) -> MUX_MODE_R {
        MUX_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Software Input On Field."]
    #[inline(always)]
    pub fn sion(&self) -> SION_R {
        SION_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MUX Mode Select Field."]
    #[inline(always)]
    pub fn mux_mode(&mut self) -> MUX_MODE_W {
        MUX_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Software Input On Field."]
    #[inline(always)]
    pub fn sion(&mut self) -> SION_W {
        SION_W { w: self }
    }
}
