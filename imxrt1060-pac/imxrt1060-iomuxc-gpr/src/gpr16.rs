#[doc = "Reader of register GPR16"]
pub type R = crate::R<u32, super::GPR16>;
#[doc = "Writer for register GPR16"]
pub type W = crate::W<u32, super::GPR16>;
#[doc = "Register GPR16 `reset()`'s with value 0x0020_0003"]
impl crate::ResetValue for super::GPR16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0020_0003
    }
}
#[doc = "ITCM enable initialization out of reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_ITCM_EN_A {
    #[doc = "0: ITCM is disabled"]
    INIT_ITCM_EN_0,
    #[doc = "1: ITCM is enabled"]
    INIT_ITCM_EN_1,
}
impl From<INIT_ITCM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_ITCM_EN_A) -> Self {
        match variant {
            INIT_ITCM_EN_A::INIT_ITCM_EN_0 => false,
            INIT_ITCM_EN_A::INIT_ITCM_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `INIT_ITCM_EN`"]
pub type INIT_ITCM_EN_R = crate::R<bool, INIT_ITCM_EN_A>;
impl INIT_ITCM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_ITCM_EN_A {
        match self.bits {
            false => INIT_ITCM_EN_A::INIT_ITCM_EN_0,
            true => INIT_ITCM_EN_A::INIT_ITCM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INIT_ITCM_EN_0`"]
    #[inline(always)]
    pub fn is_init_itcm_en_0(&self) -> bool {
        *self == INIT_ITCM_EN_A::INIT_ITCM_EN_0
    }
    #[doc = "Checks if the value of the field is `INIT_ITCM_EN_1`"]
    #[inline(always)]
    pub fn is_init_itcm_en_1(&self) -> bool {
        *self == INIT_ITCM_EN_A::INIT_ITCM_EN_1
    }
}
#[doc = "Write proxy for field `INIT_ITCM_EN`"]
pub struct INIT_ITCM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_ITCM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_ITCM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ITCM is disabled"]
    #[inline(always)]
    pub fn init_itcm_en_0(self) -> &'a mut W {
        self.variant(INIT_ITCM_EN_A::INIT_ITCM_EN_0)
    }
    #[doc = "ITCM is enabled"]
    #[inline(always)]
    pub fn init_itcm_en_1(self) -> &'a mut W {
        self.variant(INIT_ITCM_EN_A::INIT_ITCM_EN_1)
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
#[doc = "DTCM enable initialization out of reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_DTCM_EN_A {
    #[doc = "0: DTCM is disabled"]
    INIT_DTCM_EN_0,
    #[doc = "1: DTCM is enabled"]
    INIT_DTCM_EN_1,
}
impl From<INIT_DTCM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_DTCM_EN_A) -> Self {
        match variant {
            INIT_DTCM_EN_A::INIT_DTCM_EN_0 => false,
            INIT_DTCM_EN_A::INIT_DTCM_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `INIT_DTCM_EN`"]
pub type INIT_DTCM_EN_R = crate::R<bool, INIT_DTCM_EN_A>;
impl INIT_DTCM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_DTCM_EN_A {
        match self.bits {
            false => INIT_DTCM_EN_A::INIT_DTCM_EN_0,
            true => INIT_DTCM_EN_A::INIT_DTCM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INIT_DTCM_EN_0`"]
    #[inline(always)]
    pub fn is_init_dtcm_en_0(&self) -> bool {
        *self == INIT_DTCM_EN_A::INIT_DTCM_EN_0
    }
    #[doc = "Checks if the value of the field is `INIT_DTCM_EN_1`"]
    #[inline(always)]
    pub fn is_init_dtcm_en_1(&self) -> bool {
        *self == INIT_DTCM_EN_A::INIT_DTCM_EN_1
    }
}
#[doc = "Write proxy for field `INIT_DTCM_EN`"]
pub struct INIT_DTCM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_DTCM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_DTCM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DTCM is disabled"]
    #[inline(always)]
    pub fn init_dtcm_en_0(self) -> &'a mut W {
        self.variant(INIT_DTCM_EN_A::INIT_DTCM_EN_0)
    }
    #[doc = "DTCM is enabled"]
    #[inline(always)]
    pub fn init_dtcm_en_1(self) -> &'a mut W {
        self.variant(INIT_DTCM_EN_A::INIT_DTCM_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "FlexRAM bank config source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXRAM_BANK_CFG_SEL_A {
    #[doc = "0: use fuse value to config"]
    FLEXRAM_BANK_CFG_SEL_0,
    #[doc = "1: use FLEXRAM_BANK_CFG to config"]
    FLEXRAM_BANK_CFG_SEL_1,
}
impl From<FLEXRAM_BANK_CFG_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXRAM_BANK_CFG_SEL_A) -> Self {
        match variant {
            FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_0 => false,
            FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXRAM_BANK_CFG_SEL`"]
pub type FLEXRAM_BANK_CFG_SEL_R = crate::R<bool, FLEXRAM_BANK_CFG_SEL_A>;
impl FLEXRAM_BANK_CFG_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXRAM_BANK_CFG_SEL_A {
        match self.bits {
            false => FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_0,
            true => FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXRAM_BANK_CFG_SEL_0`"]
    #[inline(always)]
    pub fn is_flexram_bank_cfg_sel_0(&self) -> bool {
        *self == FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXRAM_BANK_CFG_SEL_1`"]
    #[inline(always)]
    pub fn is_flexram_bank_cfg_sel_1(&self) -> bool {
        *self == FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_1
    }
}
#[doc = "Write proxy for field `FLEXRAM_BANK_CFG_SEL`"]
pub struct FLEXRAM_BANK_CFG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXRAM_BANK_CFG_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXRAM_BANK_CFG_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "use fuse value to config"]
    #[inline(always)]
    pub fn flexram_bank_cfg_sel_0(self) -> &'a mut W {
        self.variant(FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_0)
    }
    #[doc = "use FLEXRAM_BANK_CFG to config"]
    #[inline(always)]
    pub fn flexram_bank_cfg_sel_1(self) -> &'a mut W {
        self.variant(FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ITCM enable initialization out of reset"]
    #[inline(always)]
    pub fn init_itcm_en(&self) -> INIT_ITCM_EN_R {
        INIT_ITCM_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTCM enable initialization out of reset"]
    #[inline(always)]
    pub fn init_dtcm_en(&self) -> INIT_DTCM_EN_R {
        INIT_DTCM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FlexRAM bank config source select"]
    #[inline(always)]
    pub fn flexram_bank_cfg_sel(&self) -> FLEXRAM_BANK_CFG_SEL_R {
        FLEXRAM_BANK_CFG_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITCM enable initialization out of reset"]
    #[inline(always)]
    pub fn init_itcm_en(&mut self) -> INIT_ITCM_EN_W {
        INIT_ITCM_EN_W { w: self }
    }
    #[doc = "Bit 1 - DTCM enable initialization out of reset"]
    #[inline(always)]
    pub fn init_dtcm_en(&mut self) -> INIT_DTCM_EN_W {
        INIT_DTCM_EN_W { w: self }
    }
    #[doc = "Bit 2 - FlexRAM bank config source select"]
    #[inline(always)]
    pub fn flexram_bank_cfg_sel(&mut self) -> FLEXRAM_BANK_CFG_SEL_W {
        FLEXRAM_BANK_CFG_SEL_W { w: self }
    }
}
