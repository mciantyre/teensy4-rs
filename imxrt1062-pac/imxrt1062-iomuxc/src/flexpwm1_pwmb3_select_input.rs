#[doc = "Reader of register FLEXPWM1_PWMB3_SELECT_INPUT"]
pub type R = crate::R<u32, super::FLEXPWM1_PWMB3_SELECT_INPUT>;
#[doc = "Writer for register FLEXPWM1_PWMB3_SELECT_INPUT"]
pub type W = crate::W<u32, super::FLEXPWM1_PWMB3_SELECT_INPUT>;
#[doc = "Register FLEXPWM1_PWMB3_SELECT_INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::FLEXPWM1_PWMB3_SELECT_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    GPIO_SD_B1_01_ALT2,
    #[doc = "1: Selecting Pad: GPIO_EMC_13 for Mode: ALT4"]
    GPIO_EMC_13_ALT4,
    #[doc = "2: Selecting Pad: GPIO_EMC_39 for Mode: ALT1"]
    GPIO_EMC_39_ALT1,
    #[doc = "3: Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1"]
    GPIO_AD_B0_11_ALT1,
    #[doc = "4: Selecting Pad: GPIO_B1_01 for Mode: ALT6"]
    GPIO_B1_01_ALT6,
}
impl From<DAISY_A> for u8 {
    #[inline(always)]
    fn from(variant: DAISY_A) -> Self {
        match variant {
            DAISY_A::GPIO_SD_B1_01_ALT2 => 0,
            DAISY_A::GPIO_EMC_13_ALT4 => 1,
            DAISY_A::GPIO_EMC_39_ALT1 => 2,
            DAISY_A::GPIO_AD_B0_11_ALT1 => 3,
            DAISY_A::GPIO_B1_01_ALT6 => 4,
        }
    }
}
#[doc = "Reader of field `DAISY`"]
pub type DAISY_R = crate::R<u8, DAISY_A>;
impl DAISY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DAISY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DAISY_A::GPIO_SD_B1_01_ALT2),
            1 => Val(DAISY_A::GPIO_EMC_13_ALT4),
            2 => Val(DAISY_A::GPIO_EMC_39_ALT1),
            3 => Val(DAISY_A::GPIO_AD_B0_11_ALT1),
            4 => Val(DAISY_A::GPIO_B1_01_ALT6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_SD_B1_01_ALT2`"]
    #[inline(always)]
    pub fn is_gpio_sd_b1_01_alt2(&self) -> bool {
        *self == DAISY_A::GPIO_SD_B1_01_ALT2
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_13_ALT4`"]
    #[inline(always)]
    pub fn is_gpio_emc_13_alt4(&self) -> bool {
        *self == DAISY_A::GPIO_EMC_13_ALT4
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_39_ALT1`"]
    #[inline(always)]
    pub fn is_gpio_emc_39_alt1(&self) -> bool {
        *self == DAISY_A::GPIO_EMC_39_ALT1
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_11_ALT1`"]
    #[inline(always)]
    pub fn is_gpio_ad_b0_11_alt1(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B0_11_ALT1
    }
    #[doc = "Checks if the value of the field is `GPIO_B1_01_ALT6`"]
    #[inline(always)]
    pub fn is_gpio_b1_01_alt6(&self) -> bool {
        *self == DAISY_A::GPIO_B1_01_ALT6
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    #[inline(always)]
    pub fn gpio_sd_b1_01_alt2(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_SD_B1_01_ALT2)
    }
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT4"]
    #[inline(always)]
    pub fn gpio_emc_13_alt4(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_EMC_13_ALT4)
    }
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT1"]
    #[inline(always)]
    pub fn gpio_emc_39_alt1(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_EMC_39_ALT1)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1"]
    #[inline(always)]
    pub fn gpio_ad_b0_11_alt1(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B0_11_ALT1)
    }
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT6"]
    #[inline(always)]
    pub fn gpio_b1_01_alt6(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_B1_01_ALT6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&self) -> DAISY_R {
        DAISY_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&mut self) -> DAISY_W {
        DAISY_W { w: self }
    }
}
