#[doc = "Reader of register MIX_CTRL"]
pub type R = crate::R<u32, super::MIX_CTRL>;
#[doc = "Writer for register MIX_CTRL"]
pub type W = crate::W<u32, super::MIX_CTRL>;
#[doc = "Register MIX_CTRL `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::MIX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Disable"]
    DMAEN_0 = 0,
    #[doc = "1: Enable"]
    DMAEN_1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DMAEN_0,
            true => DMAEN_A::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        *self == DMAEN_A::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        *self == DMAEN_A::DMAEN_1
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_1)
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
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCEN_A {
    #[doc = "0: Disable"]
    BCEN_0 = 0,
    #[doc = "1: Enable"]
    BCEN_1 = 1,
}
impl From<BCEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCEN`"]
pub type BCEN_R = crate::R<bool, BCEN_A>;
impl BCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCEN_A {
        match self.bits {
            false => BCEN_A::BCEN_0,
            true => BCEN_A::BCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCEN_0`"]
    #[inline(always)]
    pub fn is_bcen_0(&self) -> bool {
        *self == BCEN_A::BCEN_0
    }
    #[doc = "Checks if the value of the field is `BCEN_1`"]
    #[inline(always)]
    pub fn is_bcen_1(&self) -> bool {
        *self == BCEN_A::BCEN_1
    }
}
#[doc = "Write proxy for field `BCEN`"]
pub struct BCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn bcen_0(self) -> &'a mut W {
        self.variant(BCEN_A::BCEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn bcen_1(self) -> &'a mut W {
        self.variant(BCEN_A::BCEN_1)
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
#[doc = "Auto CMD12 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EN_A {
    #[doc = "0: Disable"]
    AC12EN_0 = 0,
    #[doc = "1: Enable"]
    AC12EN_1 = 1,
}
impl From<AC12EN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AC12EN`"]
pub type AC12EN_R = crate::R<bool, AC12EN_A>;
impl AC12EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EN_A {
        match self.bits {
            false => AC12EN_A::AC12EN_0,
            true => AC12EN_A::AC12EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12EN_0`"]
    #[inline(always)]
    pub fn is_ac12en_0(&self) -> bool {
        *self == AC12EN_A::AC12EN_0
    }
    #[doc = "Checks if the value of the field is `AC12EN_1`"]
    #[inline(always)]
    pub fn is_ac12en_1(&self) -> bool {
        *self == AC12EN_A::AC12EN_1
    }
}
#[doc = "Write proxy for field `AC12EN`"]
pub struct AC12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AC12EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn ac12en_0(self) -> &'a mut W {
        self.variant(AC12EN_A::AC12EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn ac12en_1(self) -> &'a mut W {
        self.variant(AC12EN_A::AC12EN_1)
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
#[doc = "Reader of field `DDR_EN`"]
pub type DDR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDR_EN`"]
pub struct DDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Data Transfer Direction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDSEL_A {
    #[doc = "0: Write (Host to Card)"]
    DTDSEL_0 = 0,
    #[doc = "1: Read (Card to Host)"]
    DTDSEL_1 = 1,
}
impl From<DTDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DTDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTDSEL`"]
pub type DTDSEL_R = crate::R<bool, DTDSEL_A>;
impl DTDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDSEL_A {
        match self.bits {
            false => DTDSEL_A::DTDSEL_0,
            true => DTDSEL_A::DTDSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTDSEL_0`"]
    #[inline(always)]
    pub fn is_dtdsel_0(&self) -> bool {
        *self == DTDSEL_A::DTDSEL_0
    }
    #[doc = "Checks if the value of the field is `DTDSEL_1`"]
    #[inline(always)]
    pub fn is_dtdsel_1(&self) -> bool {
        *self == DTDSEL_A::DTDSEL_1
    }
}
#[doc = "Write proxy for field `DTDSEL`"]
pub struct DTDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn dtdsel_0(self) -> &'a mut W {
        self.variant(DTDSEL_A::DTDSEL_0)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn dtdsel_1(self) -> &'a mut W {
        self.variant(DTDSEL_A::DTDSEL_1)
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
#[doc = "Multi / Single Block Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBSEL_A {
    #[doc = "0: Single Block"]
    MSBSEL_0 = 0,
    #[doc = "1: Multiple Blocks"]
    MSBSEL_1 = 1,
}
impl From<MSBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSBSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSBSEL`"]
pub type MSBSEL_R = crate::R<bool, MSBSEL_A>;
impl MSBSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBSEL_A {
        match self.bits {
            false => MSBSEL_A::MSBSEL_0,
            true => MSBSEL_A::MSBSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MSBSEL_0`"]
    #[inline(always)]
    pub fn is_msbsel_0(&self) -> bool {
        *self == MSBSEL_A::MSBSEL_0
    }
    #[doc = "Checks if the value of the field is `MSBSEL_1`"]
    #[inline(always)]
    pub fn is_msbsel_1(&self) -> bool {
        *self == MSBSEL_A::MSBSEL_1
    }
}
#[doc = "Write proxy for field `MSBSEL`"]
pub struct MSBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSBSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn msbsel_0(self) -> &'a mut W {
        self.variant(MSBSEL_A::MSBSEL_0)
    }
    #[doc = "Multiple Blocks"]
    #[inline(always)]
    pub fn msbsel_1(self) -> &'a mut W {
        self.variant(MSBSEL_A::MSBSEL_1)
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
#[doc = "Reader of field `NIBBLE_POS`"]
pub type NIBBLE_POS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIBBLE_POS`"]
pub struct NIBBLE_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIBBLE_POS_W<'a> {
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
#[doc = "Reader of field `AC23EN`"]
pub type AC23EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC23EN`"]
pub struct AC23EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AC23EN_W<'a> {
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
#[doc = "Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXE_TUNE_A {
    #[doc = "0: Not Tuned or Tuning Completed"]
    EXE_TUNE_0 = 0,
    #[doc = "1: Execute Tuning"]
    EXE_TUNE_1 = 1,
}
impl From<EXE_TUNE_A> for bool {
    #[inline(always)]
    fn from(variant: EXE_TUNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXE_TUNE`"]
pub type EXE_TUNE_R = crate::R<bool, EXE_TUNE_A>;
impl EXE_TUNE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXE_TUNE_A {
        match self.bits {
            false => EXE_TUNE_A::EXE_TUNE_0,
            true => EXE_TUNE_A::EXE_TUNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXE_TUNE_0`"]
    #[inline(always)]
    pub fn is_exe_tune_0(&self) -> bool {
        *self == EXE_TUNE_A::EXE_TUNE_0
    }
    #[doc = "Checks if the value of the field is `EXE_TUNE_1`"]
    #[inline(always)]
    pub fn is_exe_tune_1(&self) -> bool {
        *self == EXE_TUNE_A::EXE_TUNE_1
    }
}
#[doc = "Write proxy for field `EXE_TUNE`"]
pub struct EXE_TUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXE_TUNE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXE_TUNE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn exe_tune_0(self) -> &'a mut W {
        self.variant(EXE_TUNE_A::EXE_TUNE_0)
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn exe_tune_1(self) -> &'a mut W {
        self.variant(EXE_TUNE_A::EXE_TUNE_1)
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
#[doc = "SMP_CLK_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMP_CLK_SEL_A {
    #[doc = "0: Fixed clock is used to sample data / cmd"]
    SMP_CLK_SEL_0 = 0,
    #[doc = "1: Tuned clock is used to sample data / cmd"]
    SMP_CLK_SEL_1 = 1,
}
impl From<SMP_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMP_CLK_SEL`"]
pub type SMP_CLK_SEL_R = crate::R<bool, SMP_CLK_SEL_A>;
impl SMP_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_CLK_SEL_A {
        match self.bits {
            false => SMP_CLK_SEL_A::SMP_CLK_SEL_0,
            true => SMP_CLK_SEL_A::SMP_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_smp_clk_sel_0(&self) -> bool {
        *self == SMP_CLK_SEL_A::SMP_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_smp_clk_sel_1(&self) -> bool {
        *self == SMP_CLK_SEL_A::SMP_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `SMP_CLK_SEL`"]
pub struct SMP_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fixed clock is used to sample data / cmd"]
    #[inline(always)]
    pub fn smp_clk_sel_0(self) -> &'a mut W {
        self.variant(SMP_CLK_SEL_A::SMP_CLK_SEL_0)
    }
    #[doc = "Tuned clock is used to sample data / cmd"]
    #[inline(always)]
    pub fn smp_clk_sel_1(self) -> &'a mut W {
        self.variant(SMP_CLK_SEL_A::SMP_CLK_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_TUNE_EN_A {
    #[doc = "0: Disable auto tuning"]
    AUTO_TUNE_EN_0 = 0,
    #[doc = "1: Enable auto tuning"]
    AUTO_TUNE_EN_1 = 1,
}
impl From<AUTO_TUNE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_TUNE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTO_TUNE_EN`"]
pub type AUTO_TUNE_EN_R = crate::R<bool, AUTO_TUNE_EN_A>;
impl AUTO_TUNE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_TUNE_EN_A {
        match self.bits {
            false => AUTO_TUNE_EN_A::AUTO_TUNE_EN_0,
            true => AUTO_TUNE_EN_A::AUTO_TUNE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_TUNE_EN_0`"]
    #[inline(always)]
    pub fn is_auto_tune_en_0(&self) -> bool {
        *self == AUTO_TUNE_EN_A::AUTO_TUNE_EN_0
    }
    #[doc = "Checks if the value of the field is `AUTO_TUNE_EN_1`"]
    #[inline(always)]
    pub fn is_auto_tune_en_1(&self) -> bool {
        *self == AUTO_TUNE_EN_A::AUTO_TUNE_EN_1
    }
}
#[doc = "Write proxy for field `AUTO_TUNE_EN`"]
pub struct AUTO_TUNE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_TUNE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTO_TUNE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable auto tuning"]
    #[inline(always)]
    pub fn auto_tune_en_0(self) -> &'a mut W {
        self.variant(AUTO_TUNE_EN_A::AUTO_TUNE_EN_0)
    }
    #[doc = "Enable auto tuning"]
    #[inline(always)]
    pub fn auto_tune_en_1(self) -> &'a mut W {
        self.variant(AUTO_TUNE_EN_A::AUTO_TUNE_EN_1)
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
#[doc = "Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBCLK_SEL_A {
    #[doc = "0: Feedback clock comes from the loopback CLK"]
    FBCLK_SEL_0 = 0,
    #[doc = "1: Feedback clock comes from the ipp_card_clk_out"]
    FBCLK_SEL_1 = 1,
}
impl From<FBCLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FBCLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FBCLK_SEL`"]
pub type FBCLK_SEL_R = crate::R<bool, FBCLK_SEL_A>;
impl FBCLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBCLK_SEL_A {
        match self.bits {
            false => FBCLK_SEL_A::FBCLK_SEL_0,
            true => FBCLK_SEL_A::FBCLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FBCLK_SEL_0`"]
    #[inline(always)]
    pub fn is_fbclk_sel_0(&self) -> bool {
        *self == FBCLK_SEL_A::FBCLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FBCLK_SEL_1`"]
    #[inline(always)]
    pub fn is_fbclk_sel_1(&self) -> bool {
        *self == FBCLK_SEL_A::FBCLK_SEL_1
    }
}
#[doc = "Write proxy for field `FBCLK_SEL`"]
pub struct FBCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBCLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Feedback clock comes from the loopback CLK"]
    #[inline(always)]
    pub fn fbclk_sel_0(self) -> &'a mut W {
        self.variant(FBCLK_SEL_A::FBCLK_SEL_0)
    }
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    #[inline(always)]
    pub fn fbclk_sel_1(self) -> &'a mut W {
        self.variant(FBCLK_SEL_A::FBCLK_SEL_1)
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
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BCEN_R {
        BCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline(always)]
    pub fn ac12en(&self) -> AC12EN_R {
        AC12EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dual Data Rate mode selection"]
    #[inline(always)]
    pub fn ddr_en(&self) -> DDR_EN_R {
        DDR_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DTDSEL_R {
        DTDSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline(always)]
    pub fn msbsel(&self) -> MSBSEL_R {
        MSBSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NIBBLE_POS"]
    #[inline(always)]
    pub fn nibble_pos(&self) -> NIBBLE_POS_R {
        NIBBLE_POS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Auto CMD23 Enable"]
    #[inline(always)]
    pub fn ac23en(&self) -> AC23EN_R {
        AC23EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn exe_tune(&self) -> EXE_TUNE_R {
        EXE_TUNE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SMP_CLK_SEL"]
    #[inline(always)]
    pub fn smp_clk_sel(&self) -> SMP_CLK_SEL_R {
        SMP_CLK_SEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn auto_tune_en(&self) -> AUTO_TUNE_EN_R {
        AUTO_TUNE_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn fbclk_sel(&self) -> FBCLK_SEL_R {
        FBCLK_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&mut self) -> BCEN_W {
        BCEN_W { w: self }
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline(always)]
    pub fn ac12en(&mut self) -> AC12EN_W {
        AC12EN_W { w: self }
    }
    #[doc = "Bit 3 - Dual Data Rate mode selection"]
    #[inline(always)]
    pub fn ddr_en(&mut self) -> DDR_EN_W {
        DDR_EN_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn dtdsel(&mut self) -> DTDSEL_W {
        DTDSEL_W { w: self }
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline(always)]
    pub fn msbsel(&mut self) -> MSBSEL_W {
        MSBSEL_W { w: self }
    }
    #[doc = "Bit 6 - NIBBLE_POS"]
    #[inline(always)]
    pub fn nibble_pos(&mut self) -> NIBBLE_POS_W {
        NIBBLE_POS_W { w: self }
    }
    #[doc = "Bit 7 - Auto CMD23 Enable"]
    #[inline(always)]
    pub fn ac23en(&mut self) -> AC23EN_W {
        AC23EN_W { w: self }
    }
    #[doc = "Bit 22 - Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn exe_tune(&mut self) -> EXE_TUNE_W {
        EXE_TUNE_W { w: self }
    }
    #[doc = "Bit 23 - SMP_CLK_SEL"]
    #[inline(always)]
    pub fn smp_clk_sel(&mut self) -> SMP_CLK_SEL_W {
        SMP_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 24 - Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn auto_tune_en(&mut self) -> AUTO_TUNE_EN_W {
        AUTO_TUNE_EN_W { w: self }
    }
    #[doc = "Bit 25 - Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn fbclk_sel(&mut self) -> FBCLK_SEL_W {
        FBCLK_SEL_W { w: self }
    }
}
