#[doc = "Reader of register CMEOR"]
pub type R = crate::R<u32, super::CMEOR>;
#[doc = "Writer for register CMEOR"]
pub type W = crate::W<u32, super::CMEOR>;
#[doc = "Register CMEOR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CMEOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_GPT_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_GPT_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_GPT_1 = 1,
}
impl From<MOD_EN_OV_GPT_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_GPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_EN_OV_GPT`"]
pub type MOD_EN_OV_GPT_R = crate::R<bool, MOD_EN_OV_GPT_A>;
impl MOD_EN_OV_GPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_GPT_A {
        match self.bits {
            false => MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_0,
            true => MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_GPT_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_gpt_0(&self) -> bool {
        *self == MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_GPT_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_gpt_1(&self) -> bool {
        *self == MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_1
    }
}
#[doc = "Write proxy for field `MOD_EN_OV_GPT`"]
pub struct MOD_EN_OV_GPT_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_EN_OV_GPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_EN_OV_GPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_gpt_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_gpt_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_PIT_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_PIT_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_PIT_1 = 1,
}
impl From<MOD_EN_OV_PIT_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_PIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_EN_OV_PIT`"]
pub type MOD_EN_OV_PIT_R = crate::R<bool, MOD_EN_OV_PIT_A>;
impl MOD_EN_OV_PIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_PIT_A {
        match self.bits {
            false => MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_0,
            true => MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_PIT_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_pit_0(&self) -> bool {
        *self == MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_PIT_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_pit_1(&self) -> bool {
        *self == MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_1
    }
}
#[doc = "Write proxy for field `MOD_EN_OV_PIT`"]
pub struct MOD_EN_OV_PIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_EN_OV_PIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_EN_OV_PIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_pit_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_pit_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "overide clock enable signal from USDHC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_USDHC_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_USDHC_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_USDHC_1 = 1,
}
impl From<MOD_EN_USDHC_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_USDHC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_EN_USDHC`"]
pub type MOD_EN_USDHC_R = crate::R<bool, MOD_EN_USDHC_A>;
impl MOD_EN_USDHC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_USDHC_A {
        match self.bits {
            false => MOD_EN_USDHC_A::MOD_EN_USDHC_0,
            true => MOD_EN_USDHC_A::MOD_EN_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_USDHC_0`"]
    #[inline(always)]
    pub fn is_mod_en_usdhc_0(&self) -> bool {
        *self == MOD_EN_USDHC_A::MOD_EN_USDHC_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_USDHC_1`"]
    #[inline(always)]
    pub fn is_mod_en_usdhc_1(&self) -> bool {
        *self == MOD_EN_USDHC_A::MOD_EN_USDHC_1
    }
}
#[doc = "Write proxy for field `MOD_EN_USDHC`"]
pub struct MOD_EN_USDHC_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_EN_USDHC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_EN_USDHC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_usdhc_0(self) -> &'a mut W {
        self.variant(MOD_EN_USDHC_A::MOD_EN_USDHC_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_usdhc_1(self) -> &'a mut W {
        self.variant(MOD_EN_USDHC_A::MOD_EN_USDHC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Overide clock enable signal from TRNG\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_TRNG_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_TRNG_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_TRNG_1 = 1,
}
impl From<MOD_EN_OV_TRNG_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_TRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_EN_OV_TRNG`"]
pub type MOD_EN_OV_TRNG_R = crate::R<bool, MOD_EN_OV_TRNG_A>;
impl MOD_EN_OV_TRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_TRNG_A {
        match self.bits {
            false => MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_0,
            true => MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_TRNG_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_trng_0(&self) -> bool {
        *self == MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_TRNG_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_trng_1(&self) -> bool {
        *self == MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_1
    }
}
#[doc = "Write proxy for field `MOD_EN_OV_TRNG`"]
pub struct MOD_EN_OV_TRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_EN_OV_TRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_EN_OV_TRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_trng_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_trng_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Overide clock enable signal from FlexCAN3(CANFD) - clock will not be gated based on CAN's signal 'enable_clk_cpi'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_CANFD_CPI_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_CANFD_CPI_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_CANFD_CPI_1 = 1,
}
impl From<MOD_EN_OV_CANFD_CPI_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_CANFD_CPI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_EN_OV_CANFD_CPI`"]
pub type MOD_EN_OV_CANFD_CPI_R = crate::R<bool, MOD_EN_OV_CANFD_CPI_A>;
impl MOD_EN_OV_CANFD_CPI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_CANFD_CPI_A {
        match self.bits {
            false => MOD_EN_OV_CANFD_CPI_A::MOD_EN_OV_CANFD_CPI_0,
            true => MOD_EN_OV_CANFD_CPI_A::MOD_EN_OV_CANFD_CPI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CANFD_CPI_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_canfd_cpi_0(&self) -> bool {
        *self == MOD_EN_OV_CANFD_CPI_A::MOD_EN_OV_CANFD_CPI_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CANFD_CPI_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_canfd_cpi_1(&self) -> bool {
        *self == MOD_EN_OV_CANFD_CPI_A::MOD_EN_OV_CANFD_CPI_1
    }
}
#[doc = "Write proxy for field `MOD_EN_OV_CANFD_CPI`"]
pub struct MOD_EN_OV_CANFD_CPI_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_EN_OV_CANFD_CPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_EN_OV_CANFD_CPI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_canfd_cpi_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CANFD_CPI_A::MOD_EN_OV_CANFD_CPI_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_canfd_cpi_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CANFD_CPI_A::MOD_EN_OV_CANFD_CPI_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_CAN2_CPI_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_CAN2_CPI_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_CAN2_CPI_1 = 1,
}
impl From<MOD_EN_OV_CAN2_CPI_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_CAN2_CPI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_EN_OV_CAN2_CPI`"]
pub type MOD_EN_OV_CAN2_CPI_R = crate::R<bool, MOD_EN_OV_CAN2_CPI_A>;
impl MOD_EN_OV_CAN2_CPI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_CAN2_CPI_A {
        match self.bits {
            false => MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_0,
            true => MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN2_CPI_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_can2_cpi_0(&self) -> bool {
        *self == MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN2_CPI_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_can2_cpi_1(&self) -> bool {
        *self == MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_1
    }
}
#[doc = "Write proxy for field `MOD_EN_OV_CAN2_CPI`"]
pub struct MOD_EN_OV_CAN2_CPI_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_EN_OV_CAN2_CPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_EN_OV_CAN2_CPI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_can2_cpi_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_can2_cpi_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_1)
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
#[doc = "Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_EN_OV_CAN1_CPI_A {
    #[doc = "0: don't overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_0 = 0,
    #[doc = "1: overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_1 = 1,
}
impl From<MOD_EN_OV_CAN1_CPI_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_CAN1_CPI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOD_EN_OV_CAN1_CPI`"]
pub type MOD_EN_OV_CAN1_CPI_R = crate::R<bool, MOD_EN_OV_CAN1_CPI_A>;
impl MOD_EN_OV_CAN1_CPI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_CAN1_CPI_A {
        match self.bits {
            false => MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_0,
            true => MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN1_CPI_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_can1_cpi_0(&self) -> bool {
        *self == MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN1_CPI_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_can1_cpi_1(&self) -> bool {
        *self == MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_1
    }
}
#[doc = "Write proxy for field `MOD_EN_OV_CAN1_CPI`"]
pub struct MOD_EN_OV_CAN1_CPI_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_EN_OV_CAN1_CPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_EN_OV_CAN1_CPI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't overide module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_can1_cpi_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_0)
    }
    #[doc = "overide module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_can1_cpi_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub fn mod_en_ov_gpt(&self) -> MOD_EN_OV_GPT_R {
        MOD_EN_OV_GPT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub fn mod_en_ov_pit(&self) -> MOD_EN_OV_PIT_R {
        MOD_EN_OV_PIT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - overide clock enable signal from USDHC."]
    #[inline(always)]
    pub fn mod_en_usdhc(&self) -> MOD_EN_USDHC_R {
        MOD_EN_USDHC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Overide clock enable signal from TRNG"]
    #[inline(always)]
    pub fn mod_en_ov_trng(&self) -> MOD_EN_OV_TRNG_R {
        MOD_EN_OV_TRNG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overide clock enable signal from FlexCAN3(CANFD) - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub fn mod_en_ov_canfd_cpi(&self) -> MOD_EN_OV_CANFD_CPI_R {
        MOD_EN_OV_CANFD_CPI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub fn mod_en_ov_can2_cpi(&self) -> MOD_EN_OV_CAN2_CPI_R {
        MOD_EN_OV_CAN2_CPI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub fn mod_en_ov_can1_cpi(&self) -> MOD_EN_OV_CAN1_CPI_R {
        MOD_EN_OV_CAN1_CPI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub fn mod_en_ov_gpt(&mut self) -> MOD_EN_OV_GPT_W {
        MOD_EN_OV_GPT_W { w: self }
    }
    #[doc = "Bit 6 - Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub fn mod_en_ov_pit(&mut self) -> MOD_EN_OV_PIT_W {
        MOD_EN_OV_PIT_W { w: self }
    }
    #[doc = "Bit 7 - overide clock enable signal from USDHC."]
    #[inline(always)]
    pub fn mod_en_usdhc(&mut self) -> MOD_EN_USDHC_W {
        MOD_EN_USDHC_W { w: self }
    }
    #[doc = "Bit 9 - Overide clock enable signal from TRNG"]
    #[inline(always)]
    pub fn mod_en_ov_trng(&mut self) -> MOD_EN_OV_TRNG_W {
        MOD_EN_OV_TRNG_W { w: self }
    }
    #[doc = "Bit 10 - Overide clock enable signal from FlexCAN3(CANFD) - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub fn mod_en_ov_canfd_cpi(&mut self) -> MOD_EN_OV_CANFD_CPI_W {
        MOD_EN_OV_CANFD_CPI_W { w: self }
    }
    #[doc = "Bit 28 - Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub fn mod_en_ov_can2_cpi(&mut self) -> MOD_EN_OV_CAN2_CPI_W {
        MOD_EN_OV_CAN2_CPI_W { w: self }
    }
    #[doc = "Bit 30 - Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub fn mod_en_ov_can1_cpi(&mut self) -> MOD_EN_OV_CAN1_CPI_W {
        MOD_EN_OV_CAN1_CPI_W { w: self }
    }
}
