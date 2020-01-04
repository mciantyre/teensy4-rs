#[doc = "Reader of register DEBUG_MODE"]
pub type R = crate::R<u32, super::DEBUG_MODE>;
#[doc = "Writer for register DEBUG_MODE"]
pub type W = crate::W<u32, super::DEBUG_MODE>;
#[doc = "Register DEBUG_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUG_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_CONV_VALUE`"]
pub type ADC_CONV_VALUE_R = crate::R<u16, u16>;
#[doc = "Reader of field `ADC_COCO`"]
pub type ADC_COCO_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXT_HWTS`"]
pub type EXT_HWTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXT_HWTS`"]
pub struct EXT_HWTS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_HWTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGER_A {
    #[doc = "0: No hardware trigger signal"]
    TRIGGER_0 = 0,
    #[doc = "1: Hardware trigger signal, the signal must last at least 1 ips clock period"]
    TRIGGER_1 = 1,
}
impl From<TRIGGER_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGER`"]
pub type TRIGGER_R = crate::R<bool, TRIGGER_A>;
impl TRIGGER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGER_A {
        match self.bits {
            false => TRIGGER_A::TRIGGER_0,
            true => TRIGGER_A::TRIGGER_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGER_0`"]
    #[inline(always)]
    pub fn is_trigger_0(&self) -> bool {
        *self == TRIGGER_A::TRIGGER_0
    }
    #[doc = "Checks if the value of the field is `TRIGGER_1`"]
    #[inline(always)]
    pub fn is_trigger_1(&self) -> bool {
        *self == TRIGGER_A::TRIGGER_1
    }
}
#[doc = "Write proxy for field `TRIGGER`"]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No hardware trigger signal"]
    #[inline(always)]
    pub fn trigger_0(self) -> &'a mut W {
        self.variant(TRIGGER_A::TRIGGER_0)
    }
    #[doc = "Hardware trigger signal, the signal must last at least 1 ips clock period"]
    #[inline(always)]
    pub fn trigger_1(self) -> &'a mut W {
        self.variant(TRIGGER_A::TRIGGER_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "ADC Coco Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_COCO_CLEAR_A {
    #[doc = "0: No ADC COCO clear"]
    ADC_COCO_CLEAR_0 = 0,
    #[doc = "1: Set ADC COCO clear"]
    ADC_COCO_CLEAR_1 = 1,
}
impl From<ADC_COCO_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_COCO_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_COCO_CLEAR`"]
pub type ADC_COCO_CLEAR_R = crate::R<bool, ADC_COCO_CLEAR_A>;
impl ADC_COCO_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_COCO_CLEAR_A {
        match self.bits {
            false => ADC_COCO_CLEAR_A::ADC_COCO_CLEAR_0,
            true => ADC_COCO_CLEAR_A::ADC_COCO_CLEAR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_COCO_CLEAR_0`"]
    #[inline(always)]
    pub fn is_adc_coco_clear_0(&self) -> bool {
        *self == ADC_COCO_CLEAR_A::ADC_COCO_CLEAR_0
    }
    #[doc = "Checks if the value of the field is `ADC_COCO_CLEAR_1`"]
    #[inline(always)]
    pub fn is_adc_coco_clear_1(&self) -> bool {
        *self == ADC_COCO_CLEAR_A::ADC_COCO_CLEAR_1
    }
}
#[doc = "Write proxy for field `ADC_COCO_CLEAR`"]
pub struct ADC_COCO_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_COCO_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_COCO_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ADC COCO clear"]
    #[inline(always)]
    pub fn adc_coco_clear_0(self) -> &'a mut W {
        self.variant(ADC_COCO_CLEAR_A::ADC_COCO_CLEAR_0)
    }
    #[doc = "Set ADC COCO clear"]
    #[inline(always)]
    pub fn adc_coco_clear_1(self) -> &'a mut W {
        self.variant(ADC_COCO_CLEAR_A::ADC_COCO_CLEAR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "ADC COCO Clear Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_COCO_CLEAR_DISABLE_A {
    #[doc = "0: Allow TSC hardware generates ADC COCO clear"]
    ADC_COCO_CLEAR_DISABLE_0 = 0,
    #[doc = "1: Prevent TSC from generate ADC COCO clear signal"]
    ADC_COCO_CLEAR_DISABLE_1 = 1,
}
impl From<ADC_COCO_CLEAR_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_COCO_CLEAR_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_COCO_CLEAR_DISABLE`"]
pub type ADC_COCO_CLEAR_DISABLE_R = crate::R<bool, ADC_COCO_CLEAR_DISABLE_A>;
impl ADC_COCO_CLEAR_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_COCO_CLEAR_DISABLE_A {
        match self.bits {
            false => ADC_COCO_CLEAR_DISABLE_A::ADC_COCO_CLEAR_DISABLE_0,
            true => ADC_COCO_CLEAR_DISABLE_A::ADC_COCO_CLEAR_DISABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_COCO_CLEAR_DISABLE_0`"]
    #[inline(always)]
    pub fn is_adc_coco_clear_disable_0(&self) -> bool {
        *self == ADC_COCO_CLEAR_DISABLE_A::ADC_COCO_CLEAR_DISABLE_0
    }
    #[doc = "Checks if the value of the field is `ADC_COCO_CLEAR_DISABLE_1`"]
    #[inline(always)]
    pub fn is_adc_coco_clear_disable_1(&self) -> bool {
        *self == ADC_COCO_CLEAR_DISABLE_A::ADC_COCO_CLEAR_DISABLE_1
    }
}
#[doc = "Write proxy for field `ADC_COCO_CLEAR_DISABLE`"]
pub struct ADC_COCO_CLEAR_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_COCO_CLEAR_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_COCO_CLEAR_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow TSC hardware generates ADC COCO clear"]
    #[inline(always)]
    pub fn adc_coco_clear_disable_0(self) -> &'a mut W {
        self.variant(ADC_COCO_CLEAR_DISABLE_A::ADC_COCO_CLEAR_DISABLE_0)
    }
    #[doc = "Prevent TSC from generate ADC COCO clear signal"]
    #[inline(always)]
    pub fn adc_coco_clear_disable_1(self) -> &'a mut W {
        self.variant(ADC_COCO_CLEAR_DISABLE_A::ADC_COCO_CLEAR_DISABLE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_EN_A {
    #[doc = "0: Enable debug mode"]
    DEBUG_EN_0 = 0,
    #[doc = "1: Disable debug mode"]
    DEBUG_EN_1 = 1,
}
impl From<DEBUG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEBUG_EN`"]
pub type DEBUG_EN_R = crate::R<bool, DEBUG_EN_A>;
impl DEBUG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUG_EN_A {
        match self.bits {
            false => DEBUG_EN_A::DEBUG_EN_0,
            true => DEBUG_EN_A::DEBUG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBUG_EN_0`"]
    #[inline(always)]
    pub fn is_debug_en_0(&self) -> bool {
        *self == DEBUG_EN_A::DEBUG_EN_0
    }
    #[doc = "Checks if the value of the field is `DEBUG_EN_1`"]
    #[inline(always)]
    pub fn is_debug_en_1(&self) -> bool {
        *self == DEBUG_EN_A::DEBUG_EN_1
    }
}
#[doc = "Write proxy for field `DEBUG_EN`"]
pub struct DEBUG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUG_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable debug mode"]
    #[inline(always)]
    pub fn debug_en_0(self) -> &'a mut W {
        self.variant(DEBUG_EN_A::DEBUG_EN_0)
    }
    #[doc = "Disable debug mode"]
    #[inline(always)]
    pub fn debug_en_1(self) -> &'a mut W {
        self.variant(DEBUG_EN_A::DEBUG_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADC Conversion Value"]
    #[inline(always)]
    pub fn adc_conv_value(&self) -> ADC_CONV_VALUE_R {
        ADC_CONV_VALUE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - ADC COCO Signal"]
    #[inline(always)]
    pub fn adc_coco(&self) -> ADC_COCO_R {
        ADC_COCO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Hardware Trigger Select Signal"]
    #[inline(always)]
    pub fn ext_hwts(&self) -> EXT_HWTS_R {
        EXT_HWTS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Trigger"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADC Coco Clear"]
    #[inline(always)]
    pub fn adc_coco_clear(&self) -> ADC_COCO_CLEAR_R {
        ADC_COCO_CLEAR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ADC COCO Clear Disable"]
    #[inline(always)]
    pub fn adc_coco_clear_disable(&self) -> ADC_COCO_CLEAR_DISABLE_R {
        ADC_COCO_CLEAR_DISABLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Debug Enable"]
    #[inline(always)]
    pub fn debug_en(&self) -> DEBUG_EN_R {
        DEBUG_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - Hardware Trigger Select Signal"]
    #[inline(always)]
    pub fn ext_hwts(&mut self) -> EXT_HWTS_W {
        EXT_HWTS_W { w: self }
    }
    #[doc = "Bit 24 - Trigger"]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
    #[doc = "Bit 25 - ADC Coco Clear"]
    #[inline(always)]
    pub fn adc_coco_clear(&mut self) -> ADC_COCO_CLEAR_W {
        ADC_COCO_CLEAR_W { w: self }
    }
    #[doc = "Bit 26 - ADC COCO Clear Disable"]
    #[inline(always)]
    pub fn adc_coco_clear_disable(&mut self) -> ADC_COCO_CLEAR_DISABLE_W {
        ADC_COCO_CLEAR_DISABLE_W { w: self }
    }
    #[doc = "Bit 28 - Debug Enable"]
    #[inline(always)]
    pub fn debug_en(&mut self) -> DEBUG_EN_W {
        DEBUG_EN_W { w: self }
    }
}
