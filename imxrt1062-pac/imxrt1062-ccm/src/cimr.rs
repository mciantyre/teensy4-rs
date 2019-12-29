#[doc = "Reader of register CIMR"]
pub type R = crate::R<u32, super::CIMR>;
#[doc = "Writer for register CIMR"]
pub type W = crate::W<u32, super::CIMR>;
#[doc = "Register CIMR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "mask interrupt generation due to lrf of PLLs\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_LRF_PLL_A {
    #[doc = "0: don't mask interrupt due to lrf of PLLs - interrupt will be created"]
    MASK_LRF_PLL_0,
    #[doc = "1: mask interrupt due to lrf of PLLs"]
    MASK_LRF_PLL_1,
}
impl From<MASK_LRF_PLL_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_LRF_PLL_A) -> Self {
        match variant {
            MASK_LRF_PLL_A::MASK_LRF_PLL_0 => false,
            MASK_LRF_PLL_A::MASK_LRF_PLL_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_LRF_PLL`"]
pub type MASK_LRF_PLL_R = crate::R<bool, MASK_LRF_PLL_A>;
impl MASK_LRF_PLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_LRF_PLL_A {
        match self.bits {
            false => MASK_LRF_PLL_A::MASK_LRF_PLL_0,
            true => MASK_LRF_PLL_A::MASK_LRF_PLL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_LRF_PLL_0`"]
    #[inline(always)]
    pub fn is_mask_lrf_pll_0(&self) -> bool {
        *self == MASK_LRF_PLL_A::MASK_LRF_PLL_0
    }
    #[doc = "Checks if the value of the field is `MASK_LRF_PLL_1`"]
    #[inline(always)]
    pub fn is_mask_lrf_pll_1(&self) -> bool {
        *self == MASK_LRF_PLL_A::MASK_LRF_PLL_1
    }
}
#[doc = "Write proxy for field `MASK_LRF_PLL`"]
pub struct MASK_LRF_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_LRF_PLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_LRF_PLL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't mask interrupt due to lrf of PLLs - interrupt will be created"]
    #[inline(always)]
    pub fn mask_lrf_pll_0(self) -> &'a mut W {
        self.variant(MASK_LRF_PLL_A::MASK_LRF_PLL_0)
    }
    #[doc = "mask interrupt due to lrf of PLLs"]
    #[inline(always)]
    pub fn mask_lrf_pll_1(self) -> &'a mut W {
        self.variant(MASK_LRF_PLL_A::MASK_LRF_PLL_1)
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
#[doc = "mask interrupt generation due to on board oscillator ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_COSC_READY_A {
    #[doc = "0: don't mask interrupt due to on board oscillator ready - interrupt will be created"]
    MASK_COSC_READY_0,
    #[doc = "1: mask interrupt due to on board oscillator ready"]
    MASK_COSC_READY_1,
}
impl From<MASK_COSC_READY_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_COSC_READY_A) -> Self {
        match variant {
            MASK_COSC_READY_A::MASK_COSC_READY_0 => false,
            MASK_COSC_READY_A::MASK_COSC_READY_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_COSC_READY`"]
pub type MASK_COSC_READY_R = crate::R<bool, MASK_COSC_READY_A>;
impl MASK_COSC_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_COSC_READY_A {
        match self.bits {
            false => MASK_COSC_READY_A::MASK_COSC_READY_0,
            true => MASK_COSC_READY_A::MASK_COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_COSC_READY_0`"]
    #[inline(always)]
    pub fn is_mask_cosc_ready_0(&self) -> bool {
        *self == MASK_COSC_READY_A::MASK_COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `MASK_COSC_READY_1`"]
    #[inline(always)]
    pub fn is_mask_cosc_ready_1(&self) -> bool {
        *self == MASK_COSC_READY_A::MASK_COSC_READY_1
    }
}
#[doc = "Write proxy for field `MASK_COSC_READY`"]
pub struct MASK_COSC_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_COSC_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_COSC_READY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't mask interrupt due to on board oscillator ready - interrupt will be created"]
    #[inline(always)]
    pub fn mask_cosc_ready_0(self) -> &'a mut W {
        self.variant(MASK_COSC_READY_A::MASK_COSC_READY_0)
    }
    #[doc = "mask interrupt due to on board oscillator ready"]
    #[inline(always)]
    pub fn mask_cosc_ready_1(self) -> &'a mut W {
        self.variant(MASK_COSC_READY_A::MASK_COSC_READY_1)
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
#[doc = "mask interrupt generation due to frequency change of semc_podf\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_SEMC_PODF_LOADED_A {
    #[doc = "0: don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
    MASK_SEMC_PODF_LOADED_0,
    #[doc = "1: mask interrupt due to frequency change of semc_podf"]
    MASK_SEMC_PODF_LOADED_1,
}
impl From<MASK_SEMC_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_SEMC_PODF_LOADED_A) -> Self {
        match variant {
            MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_0 => false,
            MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_SEMC_PODF_LOADED`"]
pub type MASK_SEMC_PODF_LOADED_R = crate::R<bool, MASK_SEMC_PODF_LOADED_A>;
impl MASK_SEMC_PODF_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_SEMC_PODF_LOADED_A {
        match self.bits {
            false => MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_0,
            true => MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_SEMC_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_mask_semc_podf_loaded_0(&self) -> bool {
        *self == MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_SEMC_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_mask_semc_podf_loaded_1(&self) -> bool {
        *self == MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_1
    }
}
#[doc = "Write proxy for field `MASK_SEMC_PODF_LOADED`"]
pub struct MASK_SEMC_PODF_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_SEMC_PODF_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_SEMC_PODF_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
    #[inline(always)]
    pub fn mask_semc_podf_loaded_0(self) -> &'a mut W {
        self.variant(MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn mask_semc_podf_loaded_1(self) -> &'a mut W {
        self.variant(MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_1)
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
#[doc = "mask interrupt generation due to update of periph2_clk_sel.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_PERIPH2_CLK_SEL_LOADED_A {
    #[doc = "0: don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
    MASK_PERIPH2_CLK_SEL_LOADED_0,
    #[doc = "1: mask interrupt due to update of periph2_clk_sel"]
    MASK_PERIPH2_CLK_SEL_LOADED_1,
}
impl From<MASK_PERIPH2_CLK_SEL_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_PERIPH2_CLK_SEL_LOADED_A) -> Self {
        match variant {
            MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_0 => false,
            MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_PERIPH2_CLK_SEL_LOADED`"]
pub type MASK_PERIPH2_CLK_SEL_LOADED_R = crate::R<bool, MASK_PERIPH2_CLK_SEL_LOADED_A>;
impl MASK_PERIPH2_CLK_SEL_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_PERIPH2_CLK_SEL_LOADED_A {
        match self.bits {
            false => MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_0,
            true => MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH2_CLK_SEL_LOADED_0`"]
    #[inline(always)]
    pub fn is_mask_periph2_clk_sel_loaded_0(&self) -> bool {
        *self == MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH2_CLK_SEL_LOADED_1`"]
    #[inline(always)]
    pub fn is_mask_periph2_clk_sel_loaded_1(&self) -> bool {
        *self == MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_1
    }
}
#[doc = "Write proxy for field `MASK_PERIPH2_CLK_SEL_LOADED`"]
pub struct MASK_PERIPH2_CLK_SEL_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_PERIPH2_CLK_SEL_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_PERIPH2_CLK_SEL_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
    #[inline(always)]
    pub fn mask_periph2_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_0)
    }
    #[doc = "mask interrupt due to update of periph2_clk_sel"]
    #[inline(always)]
    pub fn mask_periph2_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_1)
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
#[doc = "mask interrupt generation due to frequency change of ahb_podf\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_AHB_PODF_LOADED_A {
    #[doc = "0: don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
    MASK_AHB_PODF_LOADED_0,
    #[doc = "1: mask interrupt due to frequency change of ahb_podf"]
    MASK_AHB_PODF_LOADED_1,
}
impl From<MASK_AHB_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_AHB_PODF_LOADED_A) -> Self {
        match variant {
            MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_0 => false,
            MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_AHB_PODF_LOADED`"]
pub type MASK_AHB_PODF_LOADED_R = crate::R<bool, MASK_AHB_PODF_LOADED_A>;
impl MASK_AHB_PODF_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_AHB_PODF_LOADED_A {
        match self.bits {
            false => MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_0,
            true => MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_AHB_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_mask_ahb_podf_loaded_0(&self) -> bool {
        *self == MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_AHB_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_mask_ahb_podf_loaded_1(&self) -> bool {
        *self == MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_1
    }
}
#[doc = "Write proxy for field `MASK_AHB_PODF_LOADED`"]
pub struct MASK_AHB_PODF_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_AHB_PODF_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_AHB_PODF_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
    #[inline(always)]
    pub fn mask_ahb_podf_loaded_0(self) -> &'a mut W {
        self.variant(MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn mask_ahb_podf_loaded_1(self) -> &'a mut W {
        self.variant(MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_1)
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
#[doc = "mask interrupt generation due to update of periph_clk_sel.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_PERIPH_CLK_SEL_LOADED_A {
    #[doc = "0: don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
    MASK_PERIPH_CLK_SEL_LOADED_0,
    #[doc = "1: mask interrupt due to update of periph_clk_sel"]
    MASK_PERIPH_CLK_SEL_LOADED_1,
}
impl From<MASK_PERIPH_CLK_SEL_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_PERIPH_CLK_SEL_LOADED_A) -> Self {
        match variant {
            MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_0 => false,
            MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_PERIPH_CLK_SEL_LOADED`"]
pub type MASK_PERIPH_CLK_SEL_LOADED_R = crate::R<bool, MASK_PERIPH_CLK_SEL_LOADED_A>;
impl MASK_PERIPH_CLK_SEL_LOADED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_PERIPH_CLK_SEL_LOADED_A {
        match self.bits {
            false => MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_0,
            true => MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH_CLK_SEL_LOADED_0`"]
    #[inline(always)]
    pub fn is_mask_periph_clk_sel_loaded_0(&self) -> bool {
        *self == MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH_CLK_SEL_LOADED_1`"]
    #[inline(always)]
    pub fn is_mask_periph_clk_sel_loaded_1(&self) -> bool {
        *self == MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_1
    }
}
#[doc = "Write proxy for field `MASK_PERIPH_CLK_SEL_LOADED`"]
pub struct MASK_PERIPH_CLK_SEL_LOADED_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_PERIPH_CLK_SEL_LOADED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_PERIPH_CLK_SEL_LOADED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
    #[inline(always)]
    pub fn mask_periph_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_0)
    }
    #[doc = "mask interrupt due to update of periph_clk_sel"]
    #[inline(always)]
    pub fn mask_periph_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_1)
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
#[doc = "mask interrupt generation due to frequency change of arm_podf\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_PODF_LOADED_A {
    #[doc = "0: don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
    ARM_PODF_LOADED_0,
    #[doc = "1: mask interrupt due to frequency change of arm_podf"]
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
    #[doc = "don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
    #[inline(always)]
    pub fn arm_podf_loaded_0(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADED_A::ARM_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of arm_podf"]
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
    #[doc = "Bit 0 - mask interrupt generation due to lrf of PLLs"]
    #[inline(always)]
    pub fn mask_lrf_pll(&self) -> MASK_LRF_PLL_R {
        MASK_LRF_PLL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 6 - mask interrupt generation due to on board oscillator ready"]
    #[inline(always)]
    pub fn mask_cosc_ready(&self) -> MASK_COSC_READY_R {
        MASK_COSC_READY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - mask interrupt generation due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn mask_semc_podf_loaded(&self) -> MASK_SEMC_PODF_LOADED_R {
        MASK_SEMC_PODF_LOADED_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - mask interrupt generation due to update of periph2_clk_sel."]
    #[inline(always)]
    pub fn mask_periph2_clk_sel_loaded(&self) -> MASK_PERIPH2_CLK_SEL_LOADED_R {
        MASK_PERIPH2_CLK_SEL_LOADED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - mask interrupt generation due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn mask_ahb_podf_loaded(&self) -> MASK_AHB_PODF_LOADED_R {
        MASK_AHB_PODF_LOADED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - mask interrupt generation due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn mask_periph_clk_sel_loaded(&self) -> MASK_PERIPH_CLK_SEL_LOADED_R {
        MASK_PERIPH_CLK_SEL_LOADED_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - mask interrupt generation due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded(&self) -> ARM_PODF_LOADED_R {
        ARM_PODF_LOADED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mask interrupt generation due to lrf of PLLs"]
    #[inline(always)]
    pub fn mask_lrf_pll(&mut self) -> MASK_LRF_PLL_W {
        MASK_LRF_PLL_W { w: self }
    }
    #[doc = "Bit 6 - mask interrupt generation due to on board oscillator ready"]
    #[inline(always)]
    pub fn mask_cosc_ready(&mut self) -> MASK_COSC_READY_W {
        MASK_COSC_READY_W { w: self }
    }
    #[doc = "Bit 17 - mask interrupt generation due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn mask_semc_podf_loaded(&mut self) -> MASK_SEMC_PODF_LOADED_W {
        MASK_SEMC_PODF_LOADED_W { w: self }
    }
    #[doc = "Bit 19 - mask interrupt generation due to update of periph2_clk_sel."]
    #[inline(always)]
    pub fn mask_periph2_clk_sel_loaded(&mut self) -> MASK_PERIPH2_CLK_SEL_LOADED_W {
        MASK_PERIPH2_CLK_SEL_LOADED_W { w: self }
    }
    #[doc = "Bit 20 - mask interrupt generation due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn mask_ahb_podf_loaded(&mut self) -> MASK_AHB_PODF_LOADED_W {
        MASK_AHB_PODF_LOADED_W { w: self }
    }
    #[doc = "Bit 22 - mask interrupt generation due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn mask_periph_clk_sel_loaded(&mut self) -> MASK_PERIPH_CLK_SEL_LOADED_W {
        MASK_PERIPH_CLK_SEL_LOADED_W { w: self }
    }
    #[doc = "Bit 26 - mask interrupt generation due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded(&mut self) -> ARM_PODF_LOADED_W {
        ARM_PODF_LOADED_W { w: self }
    }
}
