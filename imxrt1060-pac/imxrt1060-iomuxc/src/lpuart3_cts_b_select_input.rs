#[doc = "Reader of register LPUART3_CTS_B_SELECT_INPUT"]
pub type R = crate::R<u32, super::LPUART3_CTS_B_SELECT_INPUT>;
#[doc = "Writer for register LPUART3_CTS_B_SELECT_INPUT"]
pub type W = crate::W<u32, super::LPUART3_CTS_B_SELECT_INPUT>;
#[doc = "Register LPUART3_CTS_B_SELECT_INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::LPUART3_CTS_B_SELECT_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_EMC_15 for Mode: ALT2"]
    GPIO_EMC_15_ALT2,
    #[doc = "1: Selecting Pad: GPIO_AD_B1_04 for Mode: ALT2"]
    GPIO_AD_B1_04_ALT2,
}
impl From<DAISY_A> for bool {
    #[inline(always)]
    fn from(variant: DAISY_A) -> Self {
        match variant {
            DAISY_A::GPIO_EMC_15_ALT2 => false,
            DAISY_A::GPIO_AD_B1_04_ALT2 => true,
        }
    }
}
#[doc = "Reader of field `DAISY`"]
pub type DAISY_R = crate::R<bool, DAISY_A>;
impl DAISY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAISY_A {
        match self.bits {
            false => DAISY_A::GPIO_EMC_15_ALT2,
            true => DAISY_A::GPIO_AD_B1_04_ALT2,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_15_ALT2`"]
    #[inline(always)]
    pub fn is_gpio_emc_15_alt2(&self) -> bool {
        *self == DAISY_A::GPIO_EMC_15_ALT2
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B1_04_ALT2`"]
    #[inline(always)]
    pub fn is_gpio_ad_b1_04_alt2(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B1_04_ALT2
    }
}
#[doc = "Write proxy for field `DAISY`"]
pub struct DAISY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAISY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAISY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selecting Pad: GPIO_EMC_15 for Mode: ALT2"]
    #[inline(always)]
    pub fn gpio_emc_15_alt2(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_EMC_15_ALT2)
    }
    #[doc = "Selecting Pad: GPIO_AD_B1_04 for Mode: ALT2"]
    #[inline(always)]
    pub fn gpio_ad_b1_04_alt2(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B1_04_ALT2)
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
    #[doc = "Bit 0 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&self) -> DAISY_R {
        DAISY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&mut self) -> DAISY_W {
        DAISY_W { w: self }
    }
}
