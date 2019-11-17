#[doc = "Reader of register CLPCR"]
pub type R = crate::R<u32, super::CLPCR>;
#[doc = "Writer for register CLPCR"]
pub type W = crate::W<u32, super::CLPCR>;
#[doc = "Register CLPCR `reset()`'s with value 0x79"]
impl crate::ResetValue for super::CLPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x79
    }
}
#[doc = "Setting the low power mode that system will enter on next assertion of dsm_request signal.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_A {
    #[doc = "0: Remain in run mode"]
    LPM_0,
    #[doc = "1: Transfer to wait mode"]
    LPM_1,
    #[doc = "2: Transfer to stop mode"]
    LPM_2,
}
impl From<LPM_A> for u8 {
    #[inline(always)]
    fn from(variant: LPM_A) -> Self {
        match variant {
            LPM_A::LPM_0 => 0,
            LPM_A::LPM_1 => 1,
            LPM_A::LPM_2 => 2,
        }
    }
}
#[doc = "Reader of field `LPM`"]
pub type LPM_R = crate::R<u8, LPM_A>;
impl LPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPM_A::LPM_0),
            1 => Val(LPM_A::LPM_1),
            2 => Val(LPM_A::LPM_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPM_0`"]
    #[inline(always)]
    pub fn is_lpm_0(&self) -> bool {
        *self == LPM_A::LPM_0
    }
    #[doc = "Checks if the value of the field is `LPM_1`"]
    #[inline(always)]
    pub fn is_lpm_1(&self) -> bool {
        *self == LPM_A::LPM_1
    }
    #[doc = "Checks if the value of the field is `LPM_2`"]
    #[inline(always)]
    pub fn is_lpm_2(&self) -> bool {
        *self == LPM_A::LPM_2
    }
}
#[doc = "Write proxy for field `LPM`"]
pub struct LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Remain in run mode"]
    #[inline(always)]
    pub fn lpm_0(self) -> &'a mut W {
        self.variant(LPM_A::LPM_0)
    }
    #[doc = "Transfer to wait mode"]
    #[inline(always)]
    pub fn lpm_1(self) -> &'a mut W {
        self.variant(LPM_A::LPM_1)
    }
    #[doc = "Transfer to stop mode"]
    #[inline(always)]
    pub fn lpm_2(self) -> &'a mut W {
        self.variant(LPM_A::LPM_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Define if ARM clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_CLK_DIS_ON_LPM_A {
    #[doc = "0: ARM clock enabled on wait mode."]
    ARM_CLK_DIS_ON_LPM_0,
    #[doc = "1: ARM clock disabled on wait mode. ."]
    ARM_CLK_DIS_ON_LPM_1,
}
impl From<ARM_CLK_DIS_ON_LPM_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_CLK_DIS_ON_LPM_A) -> Self {
        match variant {
            ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_0 => false,
            ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_1 => true,
        }
    }
}
#[doc = "Reader of field `ARM_CLK_DIS_ON_LPM`"]
pub type ARM_CLK_DIS_ON_LPM_R = crate::R<bool, ARM_CLK_DIS_ON_LPM_A>;
impl ARM_CLK_DIS_ON_LPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_CLK_DIS_ON_LPM_A {
        match self.bits {
            false => ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_0,
            true => ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_CLK_DIS_ON_LPM_0`"]
    #[inline(always)]
    pub fn is_arm_clk_dis_on_lpm_0(&self) -> bool {
        *self == ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_0
    }
    #[doc = "Checks if the value of the field is `ARM_CLK_DIS_ON_LPM_1`"]
    #[inline(always)]
    pub fn is_arm_clk_dis_on_lpm_1(&self) -> bool {
        *self == ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_1
    }
}
#[doc = "Write proxy for field `ARM_CLK_DIS_ON_LPM`"]
pub struct ARM_CLK_DIS_ON_LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ARM_CLK_DIS_ON_LPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARM_CLK_DIS_ON_LPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ARM clock enabled on wait mode."]
    #[inline(always)]
    pub fn arm_clk_dis_on_lpm_0(self) -> &'a mut W {
        self.variant(ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_0)
    }
    #[doc = "ARM clock disabled on wait mode. ."]
    #[inline(always)]
    pub fn arm_clk_dis_on_lpm_1(self) -> &'a mut W {
        self.variant(ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_1)
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
#[doc = "Standby clock oscillator bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBYOS_A {
    #[doc = "0: On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
    SBYOS_0,
    #[doc = "1: On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    SBYOS_1,
}
impl From<SBYOS_A> for bool {
    #[inline(always)]
    fn from(variant: SBYOS_A) -> Self {
        match variant {
            SBYOS_A::SBYOS_0 => false,
            SBYOS_A::SBYOS_1 => true,
        }
    }
}
#[doc = "Reader of field `SBYOS`"]
pub type SBYOS_R = crate::R<bool, SBYOS_A>;
impl SBYOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBYOS_A {
        match self.bits {
            false => SBYOS_A::SBYOS_0,
            true => SBYOS_A::SBYOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBYOS_0`"]
    #[inline(always)]
    pub fn is_sbyos_0(&self) -> bool {
        *self == SBYOS_A::SBYOS_0
    }
    #[doc = "Checks if the value of the field is `SBYOS_1`"]
    #[inline(always)]
    pub fn is_sbyos_1(&self) -> bool {
        *self == SBYOS_A::SBYOS_1
    }
}
#[doc = "Write proxy for field `SBYOS`"]
pub struct SBYOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBYOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBYOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
    #[inline(always)]
    pub fn sbyos_0(self) -> &'a mut W {
        self.variant(SBYOS_A::SBYOS_0)
    }
    #[doc = "On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    #[inline(always)]
    pub fn sbyos_1(self) -> &'a mut W {
        self.variant(SBYOS_A::SBYOS_1)
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
#[doc = "dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_REF_OSC_A {
    #[doc = "0: external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    DIS_REF_OSC_0,
    #[doc = "1: external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
    DIS_REF_OSC_1,
}
impl From<DIS_REF_OSC_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_REF_OSC_A) -> Self {
        match variant {
            DIS_REF_OSC_A::DIS_REF_OSC_0 => false,
            DIS_REF_OSC_A::DIS_REF_OSC_1 => true,
        }
    }
}
#[doc = "Reader of field `DIS_REF_OSC`"]
pub type DIS_REF_OSC_R = crate::R<bool, DIS_REF_OSC_A>;
impl DIS_REF_OSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_REF_OSC_A {
        match self.bits {
            false => DIS_REF_OSC_A::DIS_REF_OSC_0,
            true => DIS_REF_OSC_A::DIS_REF_OSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_REF_OSC_0`"]
    #[inline(always)]
    pub fn is_dis_ref_osc_0(&self) -> bool {
        *self == DIS_REF_OSC_A::DIS_REF_OSC_0
    }
    #[doc = "Checks if the value of the field is `DIS_REF_OSC_1`"]
    #[inline(always)]
    pub fn is_dis_ref_osc_1(&self) -> bool {
        *self == DIS_REF_OSC_A::DIS_REF_OSC_1
    }
}
#[doc = "Write proxy for field `DIS_REF_OSC`"]
pub struct DIS_REF_OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_REF_OSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_REF_OSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    #[inline(always)]
    pub fn dis_ref_osc_0(self) -> &'a mut W {
        self.variant(DIS_REF_OSC_A::DIS_REF_OSC_0)
    }
    #[doc = "external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
    #[inline(always)]
    pub fn dis_ref_osc_1(self) -> &'a mut W {
        self.variant(DIS_REF_OSC_A::DIS_REF_OSC_1)
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
#[doc = "Voltage standby request bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSTBY_A {
    #[doc = "0: Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
    VSTBY_0,
    #[doc = "1: Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    VSTBY_1,
}
impl From<VSTBY_A> for bool {
    #[inline(always)]
    fn from(variant: VSTBY_A) -> Self {
        match variant {
            VSTBY_A::VSTBY_0 => false,
            VSTBY_A::VSTBY_1 => true,
        }
    }
}
#[doc = "Reader of field `VSTBY`"]
pub type VSTBY_R = crate::R<bool, VSTBY_A>;
impl VSTBY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSTBY_A {
        match self.bits {
            false => VSTBY_A::VSTBY_0,
            true => VSTBY_A::VSTBY_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSTBY_0`"]
    #[inline(always)]
    pub fn is_vstby_0(&self) -> bool {
        *self == VSTBY_A::VSTBY_0
    }
    #[doc = "Checks if the value of the field is `VSTBY_1`"]
    #[inline(always)]
    pub fn is_vstby_1(&self) -> bool {
        *self == VSTBY_A::VSTBY_1
    }
}
#[doc = "Write proxy for field `VSTBY`"]
pub struct VSTBY_W<'a> {
    w: &'a mut W,
}
impl<'a> VSTBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VSTBY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
    #[inline(always)]
    pub fn vstby_0(self) -> &'a mut W {
        self.variant(VSTBY_A::VSTBY_0)
    }
    #[doc = "Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    #[inline(always)]
    pub fn vstby_1(self) -> &'a mut W {
        self.variant(VSTBY_A::VSTBY_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Standby counter definition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBY_COUNT_A {
    #[doc = "0: CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_0,
    #[doc = "1: CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_1,
    #[doc = "2: CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_2,
    #[doc = "3: CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_3,
}
impl From<STBY_COUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: STBY_COUNT_A) -> Self {
        match variant {
            STBY_COUNT_A::STBY_COUNT_0 => 0,
            STBY_COUNT_A::STBY_COUNT_1 => 1,
            STBY_COUNT_A::STBY_COUNT_2 => 2,
            STBY_COUNT_A::STBY_COUNT_3 => 3,
        }
    }
}
#[doc = "Reader of field `STBY_COUNT`"]
pub type STBY_COUNT_R = crate::R<u8, STBY_COUNT_A>;
impl STBY_COUNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBY_COUNT_A {
        match self.bits {
            0 => STBY_COUNT_A::STBY_COUNT_0,
            1 => STBY_COUNT_A::STBY_COUNT_1,
            2 => STBY_COUNT_A::STBY_COUNT_2,
            3 => STBY_COUNT_A::STBY_COUNT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_0`"]
    #[inline(always)]
    pub fn is_stby_count_0(&self) -> bool {
        *self == STBY_COUNT_A::STBY_COUNT_0
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_1`"]
    #[inline(always)]
    pub fn is_stby_count_1(&self) -> bool {
        *self == STBY_COUNT_A::STBY_COUNT_1
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_2`"]
    #[inline(always)]
    pub fn is_stby_count_2(&self) -> bool {
        *self == STBY_COUNT_A::STBY_COUNT_2
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_3`"]
    #[inline(always)]
    pub fn is_stby_count_3(&self) -> bool {
        *self == STBY_COUNT_A::STBY_COUNT_3
    }
}
#[doc = "Write proxy for field `STBY_COUNT`"]
pub struct STBY_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> STBY_COUNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STBY_COUNT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline(always)]
    pub fn stby_count_0(self) -> &'a mut W {
        self.variant(STBY_COUNT_A::STBY_COUNT_0)
    }
    #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline(always)]
    pub fn stby_count_1(self) -> &'a mut W {
        self.variant(STBY_COUNT_A::STBY_COUNT_1)
    }
    #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline(always)]
    pub fn stby_count_2(self) -> &'a mut W {
        self.variant(STBY_COUNT_A::STBY_COUNT_2)
    }
    #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline(always)]
    pub fn stby_count_3(self) -> &'a mut W {
        self.variant(STBY_COUNT_A::STBY_COUNT_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "In run mode, software can manually control powering down of on chip oscillator, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_PWRDOWN_A {
    #[doc = "0: On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    COSC_PWRDOWN_0,
    #[doc = "1: On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    COSC_PWRDOWN_1,
}
impl From<COSC_PWRDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: COSC_PWRDOWN_A) -> Self {
        match variant {
            COSC_PWRDOWN_A::COSC_PWRDOWN_0 => false,
            COSC_PWRDOWN_A::COSC_PWRDOWN_1 => true,
        }
    }
}
#[doc = "Reader of field `COSC_PWRDOWN`"]
pub type COSC_PWRDOWN_R = crate::R<bool, COSC_PWRDOWN_A>;
impl COSC_PWRDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSC_PWRDOWN_A {
        match self.bits {
            false => COSC_PWRDOWN_A::COSC_PWRDOWN_0,
            true => COSC_PWRDOWN_A::COSC_PWRDOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_PWRDOWN_0`"]
    #[inline(always)]
    pub fn is_cosc_pwrdown_0(&self) -> bool {
        *self == COSC_PWRDOWN_A::COSC_PWRDOWN_0
    }
    #[doc = "Checks if the value of the field is `COSC_PWRDOWN_1`"]
    #[inline(always)]
    pub fn is_cosc_pwrdown_1(&self) -> bool {
        *self == COSC_PWRDOWN_A::COSC_PWRDOWN_1
    }
}
#[doc = "Write proxy for field `COSC_PWRDOWN`"]
pub struct COSC_PWRDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> COSC_PWRDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COSC_PWRDOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    #[inline(always)]
    pub fn cosc_pwrdown_0(self) -> &'a mut W {
        self.variant(COSC_PWRDOWN_A::COSC_PWRDOWN_0)
    }
    #[doc = "On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    #[inline(always)]
    pub fn cosc_pwrdown_1(self) -> &'a mut W {
        self.variant(COSC_PWRDOWN_A::COSC_PWRDOWN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `BYPASS_LPM_HS1`"]
pub type BYPASS_LPM_HS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS_LPM_HS1`"]
pub struct BYPASS_LPM_HS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_LPM_HS1_W<'a> {
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
#[doc = "Reader of field `BYPASS_LPM_HS0`"]
pub type BYPASS_LPM_HS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS_LPM_HS0`"]
pub struct BYPASS_LPM_HS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_LPM_HS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_CORE0_WFI_A {
    #[doc = "0: WFI of core0 is not masked"]
    MASK_CORE0_WFI_0,
    #[doc = "1: WFI of core0 is masked"]
    MASK_CORE0_WFI_1,
}
impl From<MASK_CORE0_WFI_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_CORE0_WFI_A) -> Self {
        match variant {
            MASK_CORE0_WFI_A::MASK_CORE0_WFI_0 => false,
            MASK_CORE0_WFI_A::MASK_CORE0_WFI_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_CORE0_WFI`"]
pub type MASK_CORE0_WFI_R = crate::R<bool, MASK_CORE0_WFI_A>;
impl MASK_CORE0_WFI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_CORE0_WFI_A {
        match self.bits {
            false => MASK_CORE0_WFI_A::MASK_CORE0_WFI_0,
            true => MASK_CORE0_WFI_A::MASK_CORE0_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_CORE0_WFI_0`"]
    #[inline(always)]
    pub fn is_mask_core0_wfi_0(&self) -> bool {
        *self == MASK_CORE0_WFI_A::MASK_CORE0_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_CORE0_WFI_1`"]
    #[inline(always)]
    pub fn is_mask_core0_wfi_1(&self) -> bool {
        *self == MASK_CORE0_WFI_A::MASK_CORE0_WFI_1
    }
}
#[doc = "Write proxy for field `MASK_CORE0_WFI`"]
pub struct MASK_CORE0_WFI_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_CORE0_WFI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_CORE0_WFI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "WFI of core0 is not masked"]
    #[inline(always)]
    pub fn mask_core0_wfi_0(self) -> &'a mut W {
        self.variant(MASK_CORE0_WFI_A::MASK_CORE0_WFI_0)
    }
    #[doc = "WFI of core0 is masked"]
    #[inline(always)]
    pub fn mask_core0_wfi_1(self) -> &'a mut W {
        self.variant(MASK_CORE0_WFI_A::MASK_CORE0_WFI_1)
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
#[doc = "Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_SCU_IDLE_A {
    #[doc = "0: SCU IDLE is not masked"]
    MASK_SCU_IDLE_0,
    #[doc = "1: SCU IDLE is masked"]
    MASK_SCU_IDLE_1,
}
impl From<MASK_SCU_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_SCU_IDLE_A) -> Self {
        match variant {
            MASK_SCU_IDLE_A::MASK_SCU_IDLE_0 => false,
            MASK_SCU_IDLE_A::MASK_SCU_IDLE_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_SCU_IDLE`"]
pub type MASK_SCU_IDLE_R = crate::R<bool, MASK_SCU_IDLE_A>;
impl MASK_SCU_IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_SCU_IDLE_A {
        match self.bits {
            false => MASK_SCU_IDLE_A::MASK_SCU_IDLE_0,
            true => MASK_SCU_IDLE_A::MASK_SCU_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_SCU_IDLE_0`"]
    #[inline(always)]
    pub fn is_mask_scu_idle_0(&self) -> bool {
        *self == MASK_SCU_IDLE_A::MASK_SCU_IDLE_0
    }
    #[doc = "Checks if the value of the field is `MASK_SCU_IDLE_1`"]
    #[inline(always)]
    pub fn is_mask_scu_idle_1(&self) -> bool {
        *self == MASK_SCU_IDLE_A::MASK_SCU_IDLE_1
    }
}
#[doc = "Write proxy for field `MASK_SCU_IDLE`"]
pub struct MASK_SCU_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_SCU_IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_SCU_IDLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCU IDLE is not masked"]
    #[inline(always)]
    pub fn mask_scu_idle_0(self) -> &'a mut W {
        self.variant(MASK_SCU_IDLE_A::MASK_SCU_IDLE_0)
    }
    #[doc = "SCU IDLE is masked"]
    #[inline(always)]
    pub fn mask_scu_idle_1(self) -> &'a mut W {
        self.variant(MASK_SCU_IDLE_A::MASK_SCU_IDLE_1)
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
#[doc = "Mask L2CC IDLE for entering low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_L2CC_IDLE_A {
    #[doc = "0: L2CC IDLE is not masked"]
    MASK_L2CC_IDLE_0,
    #[doc = "1: L2CC IDLE is masked"]
    MASK_L2CC_IDLE_1,
}
impl From<MASK_L2CC_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_L2CC_IDLE_A) -> Self {
        match variant {
            MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_0 => false,
            MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_1 => true,
        }
    }
}
#[doc = "Reader of field `MASK_L2CC_IDLE`"]
pub type MASK_L2CC_IDLE_R = crate::R<bool, MASK_L2CC_IDLE_A>;
impl MASK_L2CC_IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_L2CC_IDLE_A {
        match self.bits {
            false => MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_0,
            true => MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_L2CC_IDLE_0`"]
    #[inline(always)]
    pub fn is_mask_l2cc_idle_0(&self) -> bool {
        *self == MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_0
    }
    #[doc = "Checks if the value of the field is `MASK_L2CC_IDLE_1`"]
    #[inline(always)]
    pub fn is_mask_l2cc_idle_1(&self) -> bool {
        *self == MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_1
    }
}
#[doc = "Write proxy for field `MASK_L2CC_IDLE`"]
pub struct MASK_L2CC_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_L2CC_IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_L2CC_IDLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "L2CC IDLE is not masked"]
    #[inline(always)]
    pub fn mask_l2cc_idle_0(self) -> &'a mut W {
        self.variant(MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_0)
    }
    #[doc = "L2CC IDLE is masked"]
    #[inline(always)]
    pub fn mask_l2cc_idle_1(self) -> &'a mut W {
        self.variant(MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 5 - Define if ARM clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline(always)]
    pub fn arm_clk_dis_on_lpm(&self) -> ARM_CLK_DIS_ON_LPM_R {
        ARM_CLK_DIS_ON_LPM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Standby clock oscillator bit"]
    #[inline(always)]
    pub fn sbyos(&self) -> SBYOS_R {
        SBYOS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline(always)]
    pub fn dis_ref_osc(&self) -> DIS_REF_OSC_R {
        DIS_REF_OSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Voltage standby request bit"]
    #[inline(always)]
    pub fn vstby(&self) -> VSTBY_R {
        VSTBY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Standby counter definition"]
    #[inline(always)]
    pub fn stby_count(&self) -> STBY_COUNT_R {
        STBY_COUNT_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline(always)]
    pub fn cosc_pwrdown(&self) -> COSC_PWRDOWN_R {
        COSC_PWRDOWN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    pub fn bypass_lpm_hs1(&self) -> BYPASS_LPM_HS1_R {
        BYPASS_LPM_HS1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    pub fn bypass_lpm_hs0(&self) -> BYPASS_LPM_HS0_R {
        BYPASS_LPM_HS0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub fn mask_core0_wfi(&self) -> MASK_CORE0_WFI_R {
        MASK_CORE0_WFI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub fn mask_scu_idle(&self) -> MASK_SCU_IDLE_R {
        MASK_SCU_IDLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Mask L2CC IDLE for entering low power mode"]
    #[inline(always)]
    pub fn mask_l2cc_idle(&self) -> MASK_L2CC_IDLE_R {
        MASK_L2CC_IDLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W {
        LPM_W { w: self }
    }
    #[doc = "Bit 5 - Define if ARM clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline(always)]
    pub fn arm_clk_dis_on_lpm(&mut self) -> ARM_CLK_DIS_ON_LPM_W {
        ARM_CLK_DIS_ON_LPM_W { w: self }
    }
    #[doc = "Bit 6 - Standby clock oscillator bit"]
    #[inline(always)]
    pub fn sbyos(&mut self) -> SBYOS_W {
        SBYOS_W { w: self }
    }
    #[doc = "Bit 7 - dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline(always)]
    pub fn dis_ref_osc(&mut self) -> DIS_REF_OSC_W {
        DIS_REF_OSC_W { w: self }
    }
    #[doc = "Bit 8 - Voltage standby request bit"]
    #[inline(always)]
    pub fn vstby(&mut self) -> VSTBY_W {
        VSTBY_W { w: self }
    }
    #[doc = "Bits 9:10 - Standby counter definition"]
    #[inline(always)]
    pub fn stby_count(&mut self) -> STBY_COUNT_W {
        STBY_COUNT_W { w: self }
    }
    #[doc = "Bit 11 - In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline(always)]
    pub fn cosc_pwrdown(&mut self) -> COSC_PWRDOWN_W {
        COSC_PWRDOWN_W { w: self }
    }
    #[doc = "Bit 19 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    pub fn bypass_lpm_hs1(&mut self) -> BYPASS_LPM_HS1_W {
        BYPASS_LPM_HS1_W { w: self }
    }
    #[doc = "Bit 21 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    pub fn bypass_lpm_hs0(&mut self) -> BYPASS_LPM_HS0_W {
        BYPASS_LPM_HS0_W { w: self }
    }
    #[doc = "Bit 22 - Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub fn mask_core0_wfi(&mut self) -> MASK_CORE0_WFI_W {
        MASK_CORE0_WFI_W { w: self }
    }
    #[doc = "Bit 26 - Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\] will generate low power mode request"]
    #[inline(always)]
    pub fn mask_scu_idle(&mut self) -> MASK_SCU_IDLE_W {
        MASK_SCU_IDLE_W { w: self }
    }
    #[doc = "Bit 27 - Mask L2CC IDLE for entering low power mode"]
    #[inline(always)]
    pub fn mask_l2cc_idle(&mut self) -> MASK_L2CC_IDLE_W {
        MASK_L2CC_IDLE_W { w: self }
    }
}
