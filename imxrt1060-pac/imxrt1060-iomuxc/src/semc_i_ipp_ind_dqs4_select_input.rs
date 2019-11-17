#[doc = "Reader of register SEMC_I_IPP_IND_DQS4_SELECT_INPUT"]
pub type R = crate::R<u32, super::SEMC_I_IPP_IND_DQS4_SELECT_INPUT>;
#[doc = "Writer for register SEMC_I_IPP_IND_DQS4_SELECT_INPUT"]
pub type W = crate::W<u32, super::SEMC_I_IPP_IND_DQS4_SELECT_INPUT>;
#[doc = "Register SEMC_I_IPP_IND_DQS4_SELECT_INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::SEMC_I_IPP_IND_DQS4_SELECT_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_SD_B0_00 for Mode: ALT9"]
    GPIO_SD_B0_00_ALT9,
    #[doc = "1: Selecting Pad: GPIO_EMC_39 for Mode: ALT9"]
    GPIO_EMC_39_ALT9,
    #[doc = "2: Selecting Pad: GPIO_AD_B0_09 for Mode: ALT9"]
    GPIO_AD_B0_09_ALT9,
    #[doc = "3: Selecting Pad: GPIO_B1_13 for Mode: ALT8"]
    GPIO_B1_13_ALT8,
}
impl From<DAISY_A> for u8 {
    #[inline(always)]
    fn from(variant: DAISY_A) -> Self {
        match variant {
            DAISY_A::GPIO_SD_B0_00_ALT9 => 0,
            DAISY_A::GPIO_EMC_39_ALT9 => 1,
            DAISY_A::GPIO_AD_B0_09_ALT9 => 2,
            DAISY_A::GPIO_B1_13_ALT8 => 3,
        }
    }
}
#[doc = "Reader of field `DAISY`"]
pub type DAISY_R = crate::R<u8, DAISY_A>;
impl DAISY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAISY_A {
        match self.bits {
            0 => DAISY_A::GPIO_SD_B0_00_ALT9,
            1 => DAISY_A::GPIO_EMC_39_ALT9,
            2 => DAISY_A::GPIO_AD_B0_09_ALT9,
            3 => DAISY_A::GPIO_B1_13_ALT8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_SD_B0_00_ALT9`"]
    #[inline(always)]
    pub fn is_gpio_sd_b0_00_alt9(&self) -> bool {
        *self == DAISY_A::GPIO_SD_B0_00_ALT9
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_39_ALT9`"]
    #[inline(always)]
    pub fn is_gpio_emc_39_alt9(&self) -> bool {
        *self == DAISY_A::GPIO_EMC_39_ALT9
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_09_ALT9`"]
    #[inline(always)]
    pub fn is_gpio_ad_b0_09_alt9(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B0_09_ALT9
    }
    #[doc = "Checks if the value of the field is `GPIO_B1_13_ALT8`"]
    #[inline(always)]
    pub fn is_gpio_b1_13_alt8(&self) -> bool {
        *self == DAISY_A::GPIO_B1_13_ALT8
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
            self.bits(variant.into())
        }
    }
    #[doc = "Selecting Pad: GPIO_SD_B0_00 for Mode: ALT9"]
    #[inline(always)]
    pub fn gpio_sd_b0_00_alt9(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_SD_B0_00_ALT9)
    }
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT9"]
    #[inline(always)]
    pub fn gpio_emc_39_alt9(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_EMC_39_ALT9)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_09 for Mode: ALT9"]
    #[inline(always)]
    pub fn gpio_ad_b0_09_alt9(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B0_09_ALT9)
    }
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT8"]
    #[inline(always)]
    pub fn gpio_b1_13_alt8(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_B1_13_ALT8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&self) -> DAISY_R {
        DAISY_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&mut self) -> DAISY_W {
        DAISY_W { w: self }
    }
}
