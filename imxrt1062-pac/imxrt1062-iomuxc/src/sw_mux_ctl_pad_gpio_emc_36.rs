#[doc = "Reader of register SW_MUX_CTL_PAD_GPIO_EMC_36"]
pub type R = crate::R<u32, super::SW_MUX_CTL_PAD_GPIO_EMC_36>;
#[doc = "Writer for register SW_MUX_CTL_PAD_GPIO_EMC_36"]
pub type W = crate::W<u32, super::SW_MUX_CTL_PAD_GPIO_EMC_36>;
#[doc = "Register SW_MUX_CTL_PAD_GPIO_EMC_36 `reset()`'s with value 0x05"]
impl crate::ResetValue for super::SW_MUX_CTL_PAD_GPIO_EMC_36 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05
    }
}
#[doc = "MUX Mode Select Field.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_MODE_A {
    #[doc = "0: Select mux mode: ALT0 mux port: SEMC_DATA14 of instance: semc"]
    ALT0 = 0,
    #[doc = "1: Select mux mode: ALT1 mux port: XBAR1_IN22 of instance: xbar1"]
    ALT1 = 1,
    #[doc = "2: Select mux mode: ALT2 mux port: GPT1_COMPARE2 of instance: gpt1"]
    ALT2 = 2,
    #[doc = "3: Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: sai3"]
    ALT3 = 3,
    #[doc = "4: Select mux mode: ALT4 mux port: CSI_DATA17 of instance: csi"]
    ALT4 = 4,
    #[doc = "5: Select mux mode: ALT5 mux port: GPIO3_IO22 of instance: gpio3"]
    ALT5 = 5,
    #[doc = "6: Select mux mode: ALT6 mux port: USDHC1_WP of instance: usdhc1"]
    ALT6 = 6,
    #[doc = "8: Select mux mode: ALT8 mux port: ENET2_RDATA01 of instance: enet2"]
    ALT8 = 8,
    #[doc = "9: Select mux mode: ALT9 mux port: FLEXCAN3_TX of instance: flexcan3/canfd"]
    ALT9 = 9,
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
            1 => Val(MUX_MODE_A::ALT1),
            2 => Val(MUX_MODE_A::ALT2),
            3 => Val(MUX_MODE_A::ALT3),
            4 => Val(MUX_MODE_A::ALT4),
            5 => Val(MUX_MODE_A::ALT5),
            6 => Val(MUX_MODE_A::ALT6),
            8 => Val(MUX_MODE_A::ALT8),
            9 => Val(MUX_MODE_A::ALT9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == MUX_MODE_A::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == MUX_MODE_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == MUX_MODE_A::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == MUX_MODE_A::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == MUX_MODE_A::ALT4
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == MUX_MODE_A::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT6`"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == MUX_MODE_A::ALT6
    }
    #[doc = "Checks if the value of the field is `ALT8`"]
    #[inline(always)]
    pub fn is_alt8(&self) -> bool {
        *self == MUX_MODE_A::ALT8
    }
    #[doc = "Checks if the value of the field is `ALT9`"]
    #[inline(always)]
    pub fn is_alt9(&self) -> bool {
        *self == MUX_MODE_A::ALT9
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
    #[doc = "Select mux mode: ALT0 mux port: SEMC_DATA14 of instance: semc"]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT0)
    }
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_IN22 of instance: xbar1"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT1)
    }
    #[doc = "Select mux mode: ALT2 mux port: GPT1_COMPARE2 of instance: gpt1"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT2)
    }
    #[doc = "Select mux mode: ALT3 mux port: SAI3_TX_DATA of instance: sai3"]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT3)
    }
    #[doc = "Select mux mode: ALT4 mux port: CSI_DATA17 of instance: csi"]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT4)
    }
    #[doc = "Select mux mode: ALT5 mux port: GPIO3_IO22 of instance: gpio3"]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT5)
    }
    #[doc = "Select mux mode: ALT6 mux port: USDHC1_WP of instance: usdhc1"]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT6)
    }
    #[doc = "Select mux mode: ALT8 mux port: ENET2_RDATA01 of instance: enet2"]
    #[inline(always)]
    pub fn alt8(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT8)
    }
    #[doc = "Select mux mode: ALT9 mux port: FLEXCAN3_TX of instance: flexcan3/canfd"]
    #[inline(always)]
    pub fn alt9(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Software Input On Field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SION_A {
    #[doc = "0: Input Path is determined by functionality"]
    DISABLED = 0,
    #[doc = "1: Force input path of pad GPIO_EMC_36"]
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
    #[doc = "Force input path of pad GPIO_EMC_36"]
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
    #[doc = "Bits 0:3 - MUX Mode Select Field."]
    #[inline(always)]
    pub fn mux_mode(&self) -> MUX_MODE_R {
        MUX_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Software Input On Field."]
    #[inline(always)]
    pub fn sion(&self) -> SION_R {
        SION_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MUX Mode Select Field."]
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
