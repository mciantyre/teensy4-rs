#[doc = "Reader of register CISR"]
pub type R = crate::R<u32, super::CISR>;
#[doc = "Writer for register CISR"]
pub type W = crate::W<u32, super::CISR>;
#[doc = "Register CISR `reset()`'s with value 0"]
impl crate::ResetValue for super::CISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRF_PLL_A {
    #[doc = "0: interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_0,
    #[doc = "1: interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_1,
}
impl From<LRF_PLL_A> for bool {
    #[inline(always)]
    fn from(variant: LRF_PLL_A) -> Self {
        match variant {
            LRF_PLL_A::LRF_PLL_0 => false,
            LRF_PLL_A::LRF_PLL_1 => true,
        }
    }
}
#[doc = "Reader of field `LRF_PLL`"]
pub type LRF_PLL_R = crate::R<bool, LRF_PLL_A>;
impl LRF_PLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRF_PLL_A {
        match self.bits {
            false => LRF_PLL_A::LRF_PLL_0,
            true => LRF_PLL_A::LRF_PLL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LRF_PLL_0`"]
    #[inline(always)]
    pub fn is_lrf_pll_0(&self) -> bool {
        *self == LRF_PLL_A::LRF_PLL_0
    }
    #[doc = "Checks if the value of the field is `LRF_PLL_1`"]
    #[inline(always)]
    pub fn is_lrf_pll_1(&self) -> bool {
        *self == LRF_PLL_A::LRF_PLL_1
    }
}
#[doc = "Write proxy for field `LRF_PLL`"]
pub struct LRF_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> LRF_PLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRF_PLL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub fn lrf_pll_0(self) -> &'a mut W {
        self.variant(LRF_PLL_A::LRF_PLL_0)
    }
    #[doc = "interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub fn lrf_pll_1(self) -> &'a mut W {
        self.variant(LRF_PLL_A::LRF_PLL_1)
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
#[doc = "CCM interrupt request 2 generated due to on board oscillator ready, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_READY_A {
    #[doc = "0: interrupt is not generated due to on board oscillator ready"]
    COSC_READY_0,
    #[doc = "1: interrupt generated due to on board oscillator ready"]
    COSC_READY_1,
}
impl From<COSC_READY_A> for bool {
    #[inline(always)]
    fn from(variant: COSC_READY_A) -> Self {
        match variant {
            COSC_READY_A::COSC_READY_0 => false,
            COSC_READY_A::COSC_READY_1 => true,
        }
    }
}
#[doc = "Reader of field `COSC_READY`"]
pub type COSC_READY_R = crate::R<bool, COSC_READY_A>;
impl COSC_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSC_READY_A {
        match self.bits {
            false => COSC_READY_A::COSC_READY_0,
            true => COSC_READY_A::COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_READY_0`"]
    #[inline(always)]
    pub fn is_cosc_ready_0(&self) -> bool {
        *self == COSC_READY_A::COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `COSC_READY_1`"]
    #[inline(always)]
    pub fn is_cosc_ready_1(&self) -> bool {
        *self == COSC_READY_A::COSC_READY_1
    }
}
#[doc = "Write proxy for field `COSC_READY`"]
pub struct COSC_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> COSC_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COSC_READY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt is not generated due to on board oscillator ready"]
    #[inline(always)]
    pub fn cosc_ready_0(self) -> &'a mut W {
        self.variant(COSC_READY_A::COSC_READY_0)
    }
    #[doc = "interrupt generated due to on board oscillator ready"]
    #[inline(always)]
    pub fn cosc_ready_1(self) -> &'a mut W {
        self.variant(COSC_READY_A::COSC_READY_1)
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
#[doc = "CCM interrupt request 1 generated due to frequency change of semc_podf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_PODF_LOADED_A {
    #[doc = "0: interrupt is not generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_0,
    #[doc = "1: interrupt generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_1,
}
impl From<SEMC_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_PODF_LOADED_A) -> Self {
        match variant {
            SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_0 => false,
            SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `SEMC_PODF_LOADED`"]
pub type SEMC_PODF_LOADED_R = crate::R<bool, SEMC_PODF_LOADED_A>;
impl SEMC_PODF_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_PODF_LOADED_A {
        match self.bits {
            false => SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_0,
            true => SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_semc_podf_loaded_0(&self) -> bool {
        *self == SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_semc_podf_loaded_1(&self) -> bool {
        *self == SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_1
    }
}
#[doc = "Write proxy for field `SEMC_PODF_LOADED`"]
pub struct SEMC_PODF_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMC_PODF_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEMC_PODF_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt is not generated due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn semc_podf_loaded_0(self) -> &'a mut W {
        self.variant(SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn semc_podf_loaded_1(self) -> &'a mut W {
        self.variant(SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "CCM interrupt request 1 generated due to frequency change of periph2_clk_sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH2_CLK_SEL_LOADED_A {
    #[doc = "0: interrupt is not generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_0,
    #[doc = "1: interrupt generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_1,
}
impl From<PERIPH2_CLK_SEL_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH2_CLK_SEL_LOADED_A) -> Self {
        match variant {
            PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_0 => false,
            PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `PERIPH2_CLK_SEL_LOADED`"]
pub type PERIPH2_CLK_SEL_LOADED_R = crate::R<bool, PERIPH2_CLK_SEL_LOADED_A>;
impl PERIPH2_CLK_SEL_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH2_CLK_SEL_LOADED_A {
        match self.bits {
            false => PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_0,
            true => PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_LOADED_0`"]
    #[inline(always)]
    pub fn is_periph2_clk_sel_loaded_0(&self) -> bool {
        *self == PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_LOADED_1`"]
    #[inline(always)]
    pub fn is_periph2_clk_sel_loaded_1(&self) -> bool {
        *self == PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_1
    }
}
#[doc = "Write proxy for field `PERIPH2_CLK_SEL_LOADED`"]
pub struct PERIPH2_CLK_SEL_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPH2_CLK_SEL_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPH2_CLK_SEL_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt is not generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    pub fn periph2_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    pub fn periph2_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "CCM interrupt request 1 generated due to frequency change of ahb_podf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_PODF_LOADED_A {
    #[doc = "0: interrupt is not generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_0,
    #[doc = "1: interrupt generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_1,
}
impl From<AHB_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_PODF_LOADED_A) -> Self {
        match variant {
            AHB_PODF_LOADED_A::AHB_PODF_LOADED_0 => false,
            AHB_PODF_LOADED_A::AHB_PODF_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `AHB_PODF_LOADED`"]
pub type AHB_PODF_LOADED_R = crate::R<bool, AHB_PODF_LOADED_A>;
impl AHB_PODF_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_PODF_LOADED_A {
        match self.bits {
            false => AHB_PODF_LOADED_A::AHB_PODF_LOADED_0,
            true => AHB_PODF_LOADED_A::AHB_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_ahb_podf_loaded_0(&self) -> bool {
        *self == AHB_PODF_LOADED_A::AHB_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_ahb_podf_loaded_1(&self) -> bool {
        *self == AHB_PODF_LOADED_A::AHB_PODF_LOADED_1
    }
}
#[doc = "Write proxy for field `AHB_PODF_LOADED`"]
pub struct AHB_PODF_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_PODF_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_PODF_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt is not generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn ahb_podf_loaded_0(self) -> &'a mut W {
        self.variant(AHB_PODF_LOADED_A::AHB_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn ahb_podf_loaded_1(self) -> &'a mut W {
        self.variant(AHB_PODF_LOADED_A::AHB_PODF_LOADED_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "CCM interrupt request 1 generated due to update of periph_clk_sel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH_CLK_SEL_LOADED_A {
    #[doc = "0: interrupt is not generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_0,
    #[doc = "1: interrupt generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_1,
}
impl From<PERIPH_CLK_SEL_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH_CLK_SEL_LOADED_A) -> Self {
        match variant {
            PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_0 => false,
            PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `PERIPH_CLK_SEL_LOADED`"]
pub type PERIPH_CLK_SEL_LOADED_R = crate::R<bool, PERIPH_CLK_SEL_LOADED_A>;
impl PERIPH_CLK_SEL_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH_CLK_SEL_LOADED_A {
        match self.bits {
            false => PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_0,
            true => PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_LOADED_0`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_loaded_0(&self) -> bool {
        *self == PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_LOADED_1`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_loaded_1(&self) -> bool {
        *self == PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_1
    }
}
#[doc = "Write proxy for field `PERIPH_CLK_SEL_LOADED`"]
pub struct PERIPH_CLK_SEL_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPH_CLK_SEL_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPH_CLK_SEL_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt is not generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn periph_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_0)
    }
    #[doc = "interrupt generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn periph_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "CCM interrupt request 1 generated due to frequency change of arm_podf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_PODF_LOADED_A {
    #[doc = "0: interrupt is not generated due to frequency change of arm_podf"]
    ARM_PODF_LOADED_0,
    #[doc = "1: interrupt generated due to frequency change of arm_podf"]
    ARM_PODF_LOADED_1,
}
impl From<ARM_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_PODF_LOADED_A) -> Self {
        match variant {
            ARM_PODF_LOADED_A::ARM_PODF_LOADED_0 => false,
            ARM_PODF_LOADED_A::ARM_PODF_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `ARM_PODF_LOADED`"]
pub type ARM_PODF_LOADED_R = crate::R<bool, ARM_PODF_LOADED_A>;
impl ARM_PODF_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_PODF_LOADED_A {
        match self.bits {
            false => ARM_PODF_LOADED_A::ARM_PODF_LOADED_0,
            true => ARM_PODF_LOADED_A::ARM_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_arm_podf_loaded_0(&self) -> bool {
        *self == ARM_PODF_LOADED_A::ARM_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_arm_podf_loaded_1(&self) -> bool {
        *self == ARM_PODF_LOADED_A::ARM_PODF_LOADED_1
    }
}
#[doc = "Write proxy for field `ARM_PODF_LOADED`"]
pub struct ARM_PODF_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> ARM_PODF_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARM_PODF_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt is not generated due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded_0(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADED_A::ARM_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded_1(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADED_A::ARM_PODF_LOADED_1)
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
impl R {
    #[doc = "Bit 0 - CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub fn lrf_pll(&self) -> LRF_PLL_R {
        LRF_PLL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 6 - CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline(always)]
    pub fn cosc_ready(&self) -> COSC_READY_R {
        COSC_READY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CCM interrupt request 1 generated due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn semc_podf_loaded(&self) -> SEMC_PODF_LOADED_R {
        SEMC_PODF_LOADED_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    pub fn periph2_clk_sel_loaded(&self) -> PERIPH2_CLK_SEL_LOADED_R {
        PERIPH2_CLK_SEL_LOADED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn ahb_podf_loaded(&self) -> AHB_PODF_LOADED_R {
        AHB_PODF_LOADED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn periph_clk_sel_loaded(&self) -> PERIPH_CLK_SEL_LOADED_R {
        PERIPH_CLK_SEL_LOADED_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CCM interrupt request 1 generated due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded(&self) -> ARM_PODF_LOADED_R {
        ARM_PODF_LOADED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub fn lrf_pll(&mut self) -> LRF_PLL_W {
        LRF_PLL_W { w: self }
    }
    #[doc = "Bit 6 - CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline(always)]
    pub fn cosc_ready(&mut self) -> COSC_READY_W {
        COSC_READY_W { w: self }
    }
    #[doc = "Bit 17 - CCM interrupt request 1 generated due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn semc_podf_loaded(&mut self) -> SEMC_PODF_LOADED_W {
        SEMC_PODF_LOADED_W { w: self }
    }
    #[doc = "Bit 19 - CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    pub fn periph2_clk_sel_loaded(&mut self) -> PERIPH2_CLK_SEL_LOADED_W {
        PERIPH2_CLK_SEL_LOADED_W { w: self }
    }
    #[doc = "Bit 20 - CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn ahb_podf_loaded(&mut self) -> AHB_PODF_LOADED_W {
        AHB_PODF_LOADED_W { w: self }
    }
    #[doc = "Bit 22 - CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn periph_clk_sel_loaded(&mut self) -> PERIPH_CLK_SEL_LOADED_W {
        PERIPH_CLK_SEL_LOADED_W { w: self }
    }
    #[doc = "Bit 26 - CCM interrupt request 1 generated due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded(&mut self) -> ARM_PODF_LOADED_W {
        ARM_PODF_LOADED_W { w: self }
    }
}
