#[doc = "Reader of register CGPR"]
pub type R = crate::R<u32, super::CGPR>;
#[doc = "Writer for register CGPR"]
pub type W = crate::W<u32, super::CGPR>;
#[doc = "Register CGPR `reset()`'s with value 0xfe62"]
impl crate::ResetValue for super::CGPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfe62
    }
}
#[doc = "Defines clock dividion of clock for stby_count (pmic delay counter)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMIC_DELAY_SCALER_A {
    #[doc = "0: clock is not divided"]
    PMIC_DELAY_SCALER_0 = 0,
    #[doc = "1: clock is divided /8"]
    PMIC_DELAY_SCALER_1 = 1,
}
impl From<PMIC_DELAY_SCALER_A> for bool {
    #[inline(always)]
    fn from(variant: PMIC_DELAY_SCALER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMIC_DELAY_SCALER`"]
pub type PMIC_DELAY_SCALER_R = crate::R<bool, PMIC_DELAY_SCALER_A>;
impl PMIC_DELAY_SCALER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMIC_DELAY_SCALER_A {
        match self.bits {
            false => PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_0,
            true => PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMIC_DELAY_SCALER_0`"]
    #[inline(always)]
    pub fn is_pmic_delay_scaler_0(&self) -> bool {
        *self == PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_0
    }
    #[doc = "Checks if the value of the field is `PMIC_DELAY_SCALER_1`"]
    #[inline(always)]
    pub fn is_pmic_delay_scaler_1(&self) -> bool {
        *self == PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_1
    }
}
#[doc = "Write proxy for field `PMIC_DELAY_SCALER`"]
pub struct PMIC_DELAY_SCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PMIC_DELAY_SCALER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMIC_DELAY_SCALER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "clock is not divided"]
    #[inline(always)]
    pub fn pmic_delay_scaler_0(self) -> &'a mut W {
        self.variant(PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_0)
    }
    #[doc = "clock is divided /8"]
    #[inline(always)]
    pub fn pmic_delay_scaler_1(self) -> &'a mut W {
        self.variant(PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_1)
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
#[doc = "Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFUSE_PROG_SUPPLY_GATE_A {
    #[doc = "0: fuse programing supply voltage is gated off to the efuse module"]
    EFUSE_PROG_SUPPLY_GATE_0 = 0,
    #[doc = "1: allow fuse programing."]
    EFUSE_PROG_SUPPLY_GATE_1 = 1,
}
impl From<EFUSE_PROG_SUPPLY_GATE_A> for bool {
    #[inline(always)]
    fn from(variant: EFUSE_PROG_SUPPLY_GATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EFUSE_PROG_SUPPLY_GATE`"]
pub type EFUSE_PROG_SUPPLY_GATE_R = crate::R<bool, EFUSE_PROG_SUPPLY_GATE_A>;
impl EFUSE_PROG_SUPPLY_GATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFUSE_PROG_SUPPLY_GATE_A {
        match self.bits {
            false => EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_0,
            true => EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EFUSE_PROG_SUPPLY_GATE_0`"]
    #[inline(always)]
    pub fn is_efuse_prog_supply_gate_0(&self) -> bool {
        *self == EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_0
    }
    #[doc = "Checks if the value of the field is `EFUSE_PROG_SUPPLY_GATE_1`"]
    #[inline(always)]
    pub fn is_efuse_prog_supply_gate_1(&self) -> bool {
        *self == EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_1
    }
}
#[doc = "Write proxy for field `EFUSE_PROG_SUPPLY_GATE`"]
pub struct EFUSE_PROG_SUPPLY_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PROG_SUPPLY_GATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EFUSE_PROG_SUPPLY_GATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fuse programing supply voltage is gated off to the efuse module"]
    #[inline(always)]
    pub fn efuse_prog_supply_gate_0(self) -> &'a mut W {
        self.variant(EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_0)
    }
    #[doc = "allow fuse programing."]
    #[inline(always)]
    pub fn efuse_prog_supply_gate_1(self) -> &'a mut W {
        self.variant(EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_1)
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
#[doc = "System memory DS control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYS_MEM_DS_CTRL_A {
    #[doc = "0: Disable memory DS mode always"]
    SYS_MEM_DS_CTRL_0 = 0,
    #[doc = "1: Enable memory (outside ARM platform) DS mode when system STOP and PLL are disabled"]
    SYS_MEM_DS_CTRL_1 = 1,
    #[doc = "2: enable memory (outside ARM platform) DS mode when system is in STOP mode"]
    SYS_MEM_DS_CTRL_2 = 2,
}
impl From<SYS_MEM_DS_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYS_MEM_DS_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYS_MEM_DS_CTRL`"]
pub type SYS_MEM_DS_CTRL_R = crate::R<u8, SYS_MEM_DS_CTRL_A>;
impl SYS_MEM_DS_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYS_MEM_DS_CTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_0),
            1 => Val(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_1),
            2 => Val(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_0`"]
    #[inline(always)]
    pub fn is_sys_mem_ds_ctrl_0(&self) -> bool {
        *self == SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_0
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_1`"]
    #[inline(always)]
    pub fn is_sys_mem_ds_ctrl_1(&self) -> bool {
        *self == SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_1
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_2`"]
    #[inline(always)]
    pub fn is_sys_mem_ds_ctrl_2(&self) -> bool {
        *self == SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_2
    }
}
#[doc = "Write proxy for field `SYS_MEM_DS_CTRL`"]
pub struct SYS_MEM_DS_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_MEM_DS_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_MEM_DS_CTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable memory DS mode always"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl_0(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_0)
    }
    #[doc = "Enable memory (outside ARM platform) DS mode when system STOP and PLL are disabled"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl_1(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_1)
    }
    #[doc = "enable memory (outside ARM platform) DS mode when system is in STOP mode"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl_2(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Fast PLL enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPL_A {
    #[doc = "0: Engage PLL enable default way."]
    FPL_0 = 0,
    #[doc = "1: Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    FPL_1 = 1,
}
impl From<FPL_A> for bool {
    #[inline(always)]
    fn from(variant: FPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FPL`"]
pub type FPL_R = crate::R<bool, FPL_A>;
impl FPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPL_A {
        match self.bits {
            false => FPL_A::FPL_0,
            true => FPL_A::FPL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FPL_0`"]
    #[inline(always)]
    pub fn is_fpl_0(&self) -> bool {
        *self == FPL_A::FPL_0
    }
    #[doc = "Checks if the value of the field is `FPL_1`"]
    #[inline(always)]
    pub fn is_fpl_1(&self) -> bool {
        *self == FPL_A::FPL_1
    }
}
#[doc = "Write proxy for field `FPL`"]
pub struct FPL_W<'a> {
    w: &'a mut W,
}
impl<'a> FPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Engage PLL enable default way."]
    #[inline(always)]
    pub fn fpl_0(self) -> &'a mut W {
        self.variant(FPL_A::FPL_0)
    }
    #[doc = "Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    #[inline(always)]
    pub fn fpl_1(self) -> &'a mut W {
        self.variant(FPL_A::FPL_1)
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
#[doc = "Control for the Deep Sleep signal to the ARM Platform memories with additional control logic based on the ARM WFI signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_MEM_CLK_LPM_A {
    #[doc = "0: Disable the clock to the ARM platform memories when entering Low Power Mode"]
    INT_MEM_CLK_LPM_0 = 0,
    #[doc = "1: Keep the clocks to the ARM platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
    INT_MEM_CLK_LPM_1 = 1,
}
impl From<INT_MEM_CLK_LPM_A> for bool {
    #[inline(always)]
    fn from(variant: INT_MEM_CLK_LPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INT_MEM_CLK_LPM`"]
pub type INT_MEM_CLK_LPM_R = crate::R<bool, INT_MEM_CLK_LPM_A>;
impl INT_MEM_CLK_LPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_MEM_CLK_LPM_A {
        match self.bits {
            false => INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_0,
            true => INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_MEM_CLK_LPM_0`"]
    #[inline(always)]
    pub fn is_int_mem_clk_lpm_0(&self) -> bool {
        *self == INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_0
    }
    #[doc = "Checks if the value of the field is `INT_MEM_CLK_LPM_1`"]
    #[inline(always)]
    pub fn is_int_mem_clk_lpm_1(&self) -> bool {
        *self == INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_1
    }
}
#[doc = "Write proxy for field `INT_MEM_CLK_LPM`"]
pub struct INT_MEM_CLK_LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MEM_CLK_LPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_MEM_CLK_LPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the clock to the ARM platform memories when entering Low Power Mode"]
    #[inline(always)]
    pub fn int_mem_clk_lpm_0(self) -> &'a mut W {
        self.variant(INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_0)
    }
    #[doc = "Keep the clocks to the ARM platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
    #[inline(always)]
    pub fn int_mem_clk_lpm_1(self) -> &'a mut W {
        self.variant(INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_1)
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
impl R {
    #[doc = "Bit 0 - Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline(always)]
    pub fn pmic_delay_scaler(&self) -> PMIC_DELAY_SCALER_R {
        PMIC_DELAY_SCALER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    #[inline(always)]
    pub fn efuse_prog_supply_gate(&self) -> EFUSE_PROG_SUPPLY_GATE_R {
        EFUSE_PROG_SUPPLY_GATE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - System memory DS control"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl(&self) -> SYS_MEM_DS_CTRL_R {
        SYS_MEM_DS_CTRL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Fast PLL enable."]
    #[inline(always)]
    pub fn fpl(&self) -> FPL_R {
        FPL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Control for the Deep Sleep signal to the ARM Platform memories with additional control logic based on the ARM WFI signal"]
    #[inline(always)]
    pub fn int_mem_clk_lpm(&self) -> INT_MEM_CLK_LPM_R {
        INT_MEM_CLK_LPM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline(always)]
    pub fn pmic_delay_scaler(&mut self) -> PMIC_DELAY_SCALER_W {
        PMIC_DELAY_SCALER_W { w: self }
    }
    #[doc = "Bit 4 - Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    #[inline(always)]
    pub fn efuse_prog_supply_gate(&mut self) -> EFUSE_PROG_SUPPLY_GATE_W {
        EFUSE_PROG_SUPPLY_GATE_W { w: self }
    }
    #[doc = "Bits 14:15 - System memory DS control"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl(&mut self) -> SYS_MEM_DS_CTRL_W {
        SYS_MEM_DS_CTRL_W { w: self }
    }
    #[doc = "Bit 16 - Fast PLL enable."]
    #[inline(always)]
    pub fn fpl(&mut self) -> FPL_W {
        FPL_W { w: self }
    }
    #[doc = "Bit 17 - Control for the Deep Sleep signal to the ARM Platform memories with additional control logic based on the ARM WFI signal"]
    #[inline(always)]
    pub fn int_mem_clk_lpm(&mut self) -> INT_MEM_CLK_LPM_W {
        INT_MEM_CLK_LPM_W { w: self }
    }
}
