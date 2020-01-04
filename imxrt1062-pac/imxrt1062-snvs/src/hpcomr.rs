#[doc = "Reader of register HPCOMR"]
pub type R = crate::R<u32, super::HPCOMR>;
#[doc = "Writer for register HPCOMR"]
pub type W = crate::W<u32, super::HPCOMR>;
#[doc = "Register HPCOMR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPCOMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SSM_ST`"]
pub struct SSM_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SSM_ST_W<'a> {
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
#[doc = "SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSM_ST_DIS_A {
    #[doc = "0: Secure to Trusted State transition is enabled"]
    SSM_ST_DIS_0 = 0,
    #[doc = "1: Secure to Trusted State transition is disabled"]
    SSM_ST_DIS_1 = 1,
}
impl From<SSM_ST_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_ST_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSM_ST_DIS`"]
pub type SSM_ST_DIS_R = crate::R<bool, SSM_ST_DIS_A>;
impl SSM_ST_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSM_ST_DIS_A {
        match self.bits {
            false => SSM_ST_DIS_A::SSM_ST_DIS_0,
            true => SSM_ST_DIS_A::SSM_ST_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSM_ST_DIS_0`"]
    #[inline(always)]
    pub fn is_ssm_st_dis_0(&self) -> bool {
        *self == SSM_ST_DIS_A::SSM_ST_DIS_0
    }
    #[doc = "Checks if the value of the field is `SSM_ST_DIS_1`"]
    #[inline(always)]
    pub fn is_ssm_st_dis_1(&self) -> bool {
        *self == SSM_ST_DIS_A::SSM_ST_DIS_1
    }
}
#[doc = "Write proxy for field `SSM_ST_DIS`"]
pub struct SSM_ST_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSM_ST_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSM_ST_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure to Trusted State transition is enabled"]
    #[inline(always)]
    pub fn ssm_st_dis_0(self) -> &'a mut W {
        self.variant(SSM_ST_DIS_A::SSM_ST_DIS_0)
    }
    #[doc = "Secure to Trusted State transition is disabled"]
    #[inline(always)]
    pub fn ssm_st_dis_1(self) -> &'a mut W {
        self.variant(SSM_ST_DIS_A::SSM_ST_DIS_1)
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
#[doc = "SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSM_SFNS_DIS_A {
    #[doc = "0: Soft Fail to Non-Secure State transition is enabled"]
    SSM_SFNS_DIS_0 = 0,
    #[doc = "1: Soft Fail to Non-Secure State transition is disabled"]
    SSM_SFNS_DIS_1 = 1,
}
impl From<SSM_SFNS_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_SFNS_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSM_SFNS_DIS`"]
pub type SSM_SFNS_DIS_R = crate::R<bool, SSM_SFNS_DIS_A>;
impl SSM_SFNS_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSM_SFNS_DIS_A {
        match self.bits {
            false => SSM_SFNS_DIS_A::SSM_SFNS_DIS_0,
            true => SSM_SFNS_DIS_A::SSM_SFNS_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSM_SFNS_DIS_0`"]
    #[inline(always)]
    pub fn is_ssm_sfns_dis_0(&self) -> bool {
        *self == SSM_SFNS_DIS_A::SSM_SFNS_DIS_0
    }
    #[doc = "Checks if the value of the field is `SSM_SFNS_DIS_1`"]
    #[inline(always)]
    pub fn is_ssm_sfns_dis_1(&self) -> bool {
        *self == SSM_SFNS_DIS_A::SSM_SFNS_DIS_1
    }
}
#[doc = "Write proxy for field `SSM_SFNS_DIS`"]
pub struct SSM_SFNS_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSM_SFNS_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSM_SFNS_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Soft Fail to Non-Secure State transition is enabled"]
    #[inline(always)]
    pub fn ssm_sfns_dis_0(self) -> &'a mut W {
        self.variant(SSM_SFNS_DIS_A::SSM_SFNS_DIS_0)
    }
    #[doc = "Soft Fail to Non-Secure State transition is disabled"]
    #[inline(always)]
    pub fn ssm_sfns_dis_1(self) -> &'a mut W {
        self.variant(SSM_SFNS_DIS_A::SSM_SFNS_DIS_1)
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
#[doc = "LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP_SWR_AW {
    #[doc = "0: No Action"]
    LP_SWR_0 = 0,
    #[doc = "1: Reset LP section"]
    LP_SWR_1 = 1,
}
impl From<LP_SWR_AW> for bool {
    #[inline(always)]
    fn from(variant: LP_SWR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LP_SWR`"]
pub struct LP_SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_SWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LP_SWR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn lp_swr_0(self) -> &'a mut W {
        self.variant(LP_SWR_AW::LP_SWR_0)
    }
    #[doc = "Reset LP section"]
    #[inline(always)]
    pub fn lp_swr_1(self) -> &'a mut W {
        self.variant(LP_SWR_AW::LP_SWR_1)
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
#[doc = "LP Software Reset Disable When set, disables the LP software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP_SWR_DIS_A {
    #[doc = "0: LP software reset is enabled"]
    LP_SWR_DIS_0 = 0,
    #[doc = "1: LP software reset is disabled"]
    LP_SWR_DIS_1 = 1,
}
impl From<LP_SWR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: LP_SWR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LP_SWR_DIS`"]
pub type LP_SWR_DIS_R = crate::R<bool, LP_SWR_DIS_A>;
impl LP_SWR_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_SWR_DIS_A {
        match self.bits {
            false => LP_SWR_DIS_A::LP_SWR_DIS_0,
            true => LP_SWR_DIS_A::LP_SWR_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LP_SWR_DIS_0`"]
    #[inline(always)]
    pub fn is_lp_swr_dis_0(&self) -> bool {
        *self == LP_SWR_DIS_A::LP_SWR_DIS_0
    }
    #[doc = "Checks if the value of the field is `LP_SWR_DIS_1`"]
    #[inline(always)]
    pub fn is_lp_swr_dis_1(&self) -> bool {
        *self == LP_SWR_DIS_A::LP_SWR_DIS_1
    }
}
#[doc = "Write proxy for field `LP_SWR_DIS`"]
pub struct LP_SWR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_SWR_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LP_SWR_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LP software reset is enabled"]
    #[inline(always)]
    pub fn lp_swr_dis_0(self) -> &'a mut W {
        self.variant(LP_SWR_DIS_A::LP_SWR_DIS_0)
    }
    #[doc = "LP software reset is disabled"]
    #[inline(always)]
    pub fn lp_swr_dis_1(self) -> &'a mut W {
        self.variant(LP_SWR_DIS_A::LP_SWR_DIS_1)
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
#[doc = "Reader of field `SW_SV`"]
pub type SW_SV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SV`"]
pub struct SW_SV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SW_FSV`"]
pub type SW_FSV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_FSV`"]
pub struct SW_FSV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_FSV_W<'a> {
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
#[doc = "Reader of field `SW_LPSV`"]
pub type SW_LPSV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_LPSV`"]
pub struct SW_LPSV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_LPSV_W<'a> {
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
#[doc = "Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROG_ZMK_AW {
    #[doc = "0: No Action"]
    PROG_ZMK_0 = 0,
    #[doc = "1: Activate hardware key programming mechanism"]
    PROG_ZMK_1 = 1,
}
impl From<PROG_ZMK_AW> for bool {
    #[inline(always)]
    fn from(variant: PROG_ZMK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PROG_ZMK`"]
pub struct PROG_ZMK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_ZMK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROG_ZMK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn prog_zmk_0(self) -> &'a mut W {
        self.variant(PROG_ZMK_AW::PROG_ZMK_0)
    }
    #[doc = "Activate hardware key programming mechanism"]
    #[inline(always)]
    pub fn prog_zmk_1(self) -> &'a mut W {
        self.variant(PROG_ZMK_AW::PROG_ZMK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MKS_EN_A {
    #[doc = "0: OTP master key is selected as an SNVS master key"]
    MKS_EN_0 = 0,
    #[doc = "1: SNVS master key is selected according to the setting of the MASTER_KEY_SEL field of LPMKCR"]
    MKS_EN_1 = 1,
}
impl From<MKS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MKS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MKS_EN`"]
pub type MKS_EN_R = crate::R<bool, MKS_EN_A>;
impl MKS_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MKS_EN_A {
        match self.bits {
            false => MKS_EN_A::MKS_EN_0,
            true => MKS_EN_A::MKS_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MKS_EN_0`"]
    #[inline(always)]
    pub fn is_mks_en_0(&self) -> bool {
        *self == MKS_EN_A::MKS_EN_0
    }
    #[doc = "Checks if the value of the field is `MKS_EN_1`"]
    #[inline(always)]
    pub fn is_mks_en_1(&self) -> bool {
        *self == MKS_EN_A::MKS_EN_1
    }
}
#[doc = "Write proxy for field `MKS_EN`"]
pub struct MKS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MKS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MKS_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OTP master key is selected as an SNVS master key"]
    #[inline(always)]
    pub fn mks_en_0(self) -> &'a mut W {
        self.variant(MKS_EN_A::MKS_EN_0)
    }
    #[doc = "SNVS master key is selected according to the setting of the MASTER_KEY_SEL field of LPMKCR"]
    #[inline(always)]
    pub fn mks_en_1(self) -> &'a mut W {
        self.variant(MKS_EN_A::MKS_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAC_EN_A {
    #[doc = "0: High Assurance Counter is disabled"]
    HAC_EN_0 = 0,
    #[doc = "1: High Assurance Counter is enabled"]
    HAC_EN_1 = 1,
}
impl From<HAC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HAC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HAC_EN`"]
pub type HAC_EN_R = crate::R<bool, HAC_EN_A>;
impl HAC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAC_EN_A {
        match self.bits {
            false => HAC_EN_A::HAC_EN_0,
            true => HAC_EN_A::HAC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HAC_EN_0`"]
    #[inline(always)]
    pub fn is_hac_en_0(&self) -> bool {
        *self == HAC_EN_A::HAC_EN_0
    }
    #[doc = "Checks if the value of the field is `HAC_EN_1`"]
    #[inline(always)]
    pub fn is_hac_en_1(&self) -> bool {
        *self == HAC_EN_A::HAC_EN_1
    }
}
#[doc = "Write proxy for field `HAC_EN`"]
pub struct HAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HAC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High Assurance Counter is disabled"]
    #[inline(always)]
    pub fn hac_en_0(self) -> &'a mut W {
        self.variant(HAC_EN_A::HAC_EN_0)
    }
    #[doc = "High Assurance Counter is enabled"]
    #[inline(always)]
    pub fn hac_en_1(self) -> &'a mut W {
        self.variant(HAC_EN_A::HAC_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAC_LOAD_AW {
    #[doc = "0: No Action"]
    HAC_LOAD_0 = 0,
    #[doc = "1: Load the HAC"]
    HAC_LOAD_1 = 1,
}
impl From<HAC_LOAD_AW> for bool {
    #[inline(always)]
    fn from(variant: HAC_LOAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `HAC_LOAD`"]
pub struct HAC_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_LOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HAC_LOAD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn hac_load_0(self) -> &'a mut W {
        self.variant(HAC_LOAD_AW::HAC_LOAD_0)
    }
    #[doc = "Load the HAC"]
    #[inline(always)]
    pub fn hac_load_1(self) -> &'a mut W {
        self.variant(HAC_LOAD_AW::HAC_LOAD_1)
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
#[doc = "High Assurance Counter Clear When set, it clears the High Assurance Counter Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAC_CLEAR_AW {
    #[doc = "0: No Action"]
    HAC_CLEAR_0 = 0,
    #[doc = "1: Clear the HAC"]
    HAC_CLEAR_1 = 1,
}
impl From<HAC_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: HAC_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `HAC_CLEAR`"]
pub struct HAC_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HAC_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn hac_clear_0(self) -> &'a mut W {
        self.variant(HAC_CLEAR_AW::HAC_CLEAR_0)
    }
    #[doc = "Clear the HAC"]
    #[inline(always)]
    pub fn hac_clear_1(self) -> &'a mut W {
        self.variant(HAC_CLEAR_AW::HAC_CLEAR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `HAC_STOP`"]
pub type HAC_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HAC_STOP`"]
pub struct HAC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_STOP_W<'a> {
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
#[doc = "Reader of field `NPSWA_EN`"]
pub type NPSWA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NPSWA_EN`"]
pub struct NPSWA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NPSWA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline(always)]
    pub fn ssm_st_dis(&self) -> SSM_ST_DIS_R {
        SSM_ST_DIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline(always)]
    pub fn ssm_sfns_dis(&self) -> SSM_SFNS_DIS_R {
        SSM_SFNS_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LP Software Reset Disable When set, disables the LP software reset"]
    #[inline(always)]
    pub fn lp_swr_dis(&self) -> LP_SWR_DIS_R {
        LP_SWR_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline(always)]
    pub fn sw_sv(&self) -> SW_SV_R {
        SW_SV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline(always)]
    pub fn sw_fsv(&self) -> SW_FSV_R {
        SW_FSV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline(always)]
    pub fn sw_lpsv(&self) -> SW_LPSV_R {
        SW_LPSV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline(always)]
    pub fn mks_en(&self) -> MKS_EN_R {
        MKS_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline(always)]
    pub fn hac_en(&self) -> HAC_EN_R {
        HAC_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline(always)]
    pub fn hac_stop(&self) -> HAC_STOP_R {
        HAC_STOP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline(always)]
    pub fn npswa_en(&self) -> NPSWA_EN_R {
        NPSWA_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSM State Transition Transition state of the system security monitor"]
    #[inline(always)]
    pub fn ssm_st(&mut self) -> SSM_ST_W {
        SSM_ST_W { w: self }
    }
    #[doc = "Bit 1 - SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline(always)]
    pub fn ssm_st_dis(&mut self) -> SSM_ST_DIS_W {
        SSM_ST_DIS_W { w: self }
    }
    #[doc = "Bit 2 - SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline(always)]
    pub fn ssm_sfns_dis(&mut self) -> SSM_SFNS_DIS_W {
        SSM_SFNS_DIS_W { w: self }
    }
    #[doc = "Bit 4 - LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set"]
    #[inline(always)]
    pub fn lp_swr(&mut self) -> LP_SWR_W {
        LP_SWR_W { w: self }
    }
    #[doc = "Bit 5 - LP Software Reset Disable When set, disables the LP software reset"]
    #[inline(always)]
    pub fn lp_swr_dis(&mut self) -> LP_SWR_DIS_W {
        LP_SWR_DIS_W { w: self }
    }
    #[doc = "Bit 8 - Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline(always)]
    pub fn sw_sv(&mut self) -> SW_SV_W {
        SW_SV_W { w: self }
    }
    #[doc = "Bit 9 - Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline(always)]
    pub fn sw_fsv(&mut self) -> SW_FSV_W {
        SW_FSV_W { w: self }
    }
    #[doc = "Bit 10 - LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline(always)]
    pub fn sw_lpsv(&mut self) -> SW_LPSV_W {
        SW_LPSV_W { w: self }
    }
    #[doc = "Bit 12 - Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
    #[inline(always)]
    pub fn prog_zmk(&mut self) -> PROG_ZMK_W {
        PROG_ZMK_W { w: self }
    }
    #[doc = "Bit 13 - Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline(always)]
    pub fn mks_en(&mut self) -> MKS_EN_W {
        MKS_EN_W { w: self }
    }
    #[doc = "Bit 16 - High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline(always)]
    pub fn hac_en(&mut self) -> HAC_EN_W {
        HAC_EN_W { w: self }
    }
    #[doc = "Bit 17 - High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
    #[inline(always)]
    pub fn hac_load(&mut self) -> HAC_LOAD_W {
        HAC_LOAD_W { w: self }
    }
    #[doc = "Bit 18 - High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
    #[inline(always)]
    pub fn hac_clear(&mut self) -> HAC_CLEAR_W {
        HAC_CLEAR_W { w: self }
    }
    #[doc = "Bit 19 - High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline(always)]
    pub fn hac_stop(&mut self) -> HAC_STOP_W {
        HAC_STOP_W { w: self }
    }
    #[doc = "Bit 31 - Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline(always)]
    pub fn npswa_en(&mut self) -> NPSWA_EN_W {
        NPSWA_EN_W { w: self }
    }
}
