#[doc = "Reader of register HPLR"]
pub type R = crate::R<u32, super::HPLR>;
#[doc = "Writer for register HPLR"]
pub type W = crate::W<u32, super::HPLR>;
#[doc = "Register HPLR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_WSL_A {
    #[doc = "0: Write access is allowed"]
    ZMK_WSL_0 = 0,
    #[doc = "1: Write access is not allowed"]
    ZMK_WSL_1 = 1,
}
impl From<ZMK_WSL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_WSL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_WSL`"]
pub type ZMK_WSL_R = crate::R<bool, ZMK_WSL_A>;
impl ZMK_WSL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_WSL_A {
        match self.bits {
            false => ZMK_WSL_A::ZMK_WSL_0,
            true => ZMK_WSL_A::ZMK_WSL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_WSL_0`"]
    #[inline(always)]
    pub fn is_zmk_wsl_0(&self) -> bool {
        *self == ZMK_WSL_A::ZMK_WSL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_WSL_1`"]
    #[inline(always)]
    pub fn is_zmk_wsl_1(&self) -> bool {
        *self == ZMK_WSL_A::ZMK_WSL_1
    }
}
#[doc = "Write proxy for field `ZMK_WSL`"]
pub struct ZMK_WSL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_WSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZMK_WSL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn zmk_wsl_0(self) -> &'a mut W {
        self.variant(ZMK_WSL_A::ZMK_WSL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn zmk_wsl_1(self) -> &'a mut W {
        self.variant(ZMK_WSL_A::ZMK_WSL_1)
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
#[doc = "Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_RSL_A {
    #[doc = "0: Read access is allowed (only in software Programming mode)"]
    ZMK_RSL_0 = 0,
    #[doc = "1: Read access is not allowed"]
    ZMK_RSL_1 = 1,
}
impl From<ZMK_RSL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_RSL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_RSL`"]
pub type ZMK_RSL_R = crate::R<bool, ZMK_RSL_A>;
impl ZMK_RSL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_RSL_A {
        match self.bits {
            false => ZMK_RSL_A::ZMK_RSL_0,
            true => ZMK_RSL_A::ZMK_RSL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_RSL_0`"]
    #[inline(always)]
    pub fn is_zmk_rsl_0(&self) -> bool {
        *self == ZMK_RSL_A::ZMK_RSL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_RSL_1`"]
    #[inline(always)]
    pub fn is_zmk_rsl_1(&self) -> bool {
        *self == ZMK_RSL_A::ZMK_RSL_1
    }
}
#[doc = "Write proxy for field `ZMK_RSL`"]
pub struct ZMK_RSL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_RSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZMK_RSL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access is allowed (only in software Programming mode)"]
    #[inline(always)]
    pub fn zmk_rsl_0(self) -> &'a mut W {
        self.variant(ZMK_RSL_A::ZMK_RSL_0)
    }
    #[doc = "Read access is not allowed"]
    #[inline(always)]
    pub fn zmk_rsl_1(self) -> &'a mut W {
        self.variant(ZMK_RSL_A::ZMK_RSL_1)
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
#[doc = "Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTC_SL_A {
    #[doc = "0: Write access is allowed"]
    SRTC_SL_0 = 0,
    #[doc = "1: Write access is not allowed"]
    SRTC_SL_1 = 1,
}
impl From<SRTC_SL_A> for bool {
    #[inline(always)]
    fn from(variant: SRTC_SL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRTC_SL`"]
pub type SRTC_SL_R = crate::R<bool, SRTC_SL_A>;
impl SRTC_SL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTC_SL_A {
        match self.bits {
            false => SRTC_SL_A::SRTC_SL_0,
            true => SRTC_SL_A::SRTC_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTC_SL_0`"]
    #[inline(always)]
    pub fn is_srtc_sl_0(&self) -> bool {
        *self == SRTC_SL_A::SRTC_SL_0
    }
    #[doc = "Checks if the value of the field is `SRTC_SL_1`"]
    #[inline(always)]
    pub fn is_srtc_sl_1(&self) -> bool {
        *self == SRTC_SL_A::SRTC_SL_1
    }
}
#[doc = "Write proxy for field `SRTC_SL`"]
pub struct SRTC_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTC_SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRTC_SL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn srtc_sl_0(self) -> &'a mut W {
        self.variant(SRTC_SL_A::SRTC_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn srtc_sl_1(self) -> &'a mut W {
        self.variant(SRTC_SL_A::SRTC_SL_1)
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
#[doc = "LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCALB_SL_A {
    #[doc = "0: Write access is allowed"]
    LPCALB_SL_0 = 0,
    #[doc = "1: Write access is not allowed"]
    LPCALB_SL_1 = 1,
}
impl From<LPCALB_SL_A> for bool {
    #[inline(always)]
    fn from(variant: LPCALB_SL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPCALB_SL`"]
pub type LPCALB_SL_R = crate::R<bool, LPCALB_SL_A>;
impl LPCALB_SL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCALB_SL_A {
        match self.bits {
            false => LPCALB_SL_A::LPCALB_SL_0,
            true => LPCALB_SL_A::LPCALB_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPCALB_SL_0`"]
    #[inline(always)]
    pub fn is_lpcalb_sl_0(&self) -> bool {
        *self == LPCALB_SL_A::LPCALB_SL_0
    }
    #[doc = "Checks if the value of the field is `LPCALB_SL_1`"]
    #[inline(always)]
    pub fn is_lpcalb_sl_1(&self) -> bool {
        *self == LPCALB_SL_A::LPCALB_SL_1
    }
}
#[doc = "Write proxy for field `LPCALB_SL`"]
pub struct LPCALB_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCALB_SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCALB_SL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn lpcalb_sl_0(self) -> &'a mut W {
        self.variant(LPCALB_SL_A::LPCALB_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn lpcalb_sl_1(self) -> &'a mut W {
        self.variant(LPCALB_SL_A::LPCALB_SL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MC_SL_A {
    #[doc = "0: Write access (increment) is allowed"]
    MC_SL_0 = 0,
    #[doc = "1: Write access (increment) is not allowed"]
    MC_SL_1 = 1,
}
impl From<MC_SL_A> for bool {
    #[inline(always)]
    fn from(variant: MC_SL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MC_SL`"]
pub type MC_SL_R = crate::R<bool, MC_SL_A>;
impl MC_SL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_SL_A {
        match self.bits {
            false => MC_SL_A::MC_SL_0,
            true => MC_SL_A::MC_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MC_SL_0`"]
    #[inline(always)]
    pub fn is_mc_sl_0(&self) -> bool {
        *self == MC_SL_A::MC_SL_0
    }
    #[doc = "Checks if the value of the field is `MC_SL_1`"]
    #[inline(always)]
    pub fn is_mc_sl_1(&self) -> bool {
        *self == MC_SL_A::MC_SL_1
    }
}
#[doc = "Write proxy for field `MC_SL`"]
pub struct MC_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_SL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access (increment) is allowed"]
    #[inline(always)]
    pub fn mc_sl_0(self) -> &'a mut W {
        self.variant(MC_SL_A::MC_SL_0)
    }
    #[doc = "Write access (increment) is not allowed"]
    #[inline(always)]
    pub fn mc_sl_1(self) -> &'a mut W {
        self.variant(MC_SL_A::MC_SL_1)
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
#[doc = "General Purpose Register Soft Lock When set, prevents any writes to the GPR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPR_SL_A {
    #[doc = "0: Write access is allowed"]
    GPR_SL_0 = 0,
    #[doc = "1: Write access is not allowed"]
    GPR_SL_1 = 1,
}
impl From<GPR_SL_A> for bool {
    #[inline(always)]
    fn from(variant: GPR_SL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPR_SL`"]
pub type GPR_SL_R = crate::R<bool, GPR_SL_A>;
impl GPR_SL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPR_SL_A {
        match self.bits {
            false => GPR_SL_A::GPR_SL_0,
            true => GPR_SL_A::GPR_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPR_SL_0`"]
    #[inline(always)]
    pub fn is_gpr_sl_0(&self) -> bool {
        *self == GPR_SL_A::GPR_SL_0
    }
    #[doc = "Checks if the value of the field is `GPR_SL_1`"]
    #[inline(always)]
    pub fn is_gpr_sl_1(&self) -> bool {
        *self == GPR_SL_A::GPR_SL_1
    }
}
#[doc = "Write proxy for field `GPR_SL`"]
pub struct GPR_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPR_SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPR_SL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn gpr_sl_0(self) -> &'a mut W {
        self.variant(GPR_SL_A::GPR_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn gpr_sl_1(self) -> &'a mut W {
        self.variant(GPR_SL_A::GPR_SL_1)
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
#[doc = "LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSVCR_SL_A {
    #[doc = "0: Write access is allowed"]
    LPSVCR_SL_0 = 0,
    #[doc = "1: Write access is not allowed"]
    LPSVCR_SL_1 = 1,
}
impl From<LPSVCR_SL_A> for bool {
    #[inline(always)]
    fn from(variant: LPSVCR_SL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSVCR_SL`"]
pub type LPSVCR_SL_R = crate::R<bool, LPSVCR_SL_A>;
impl LPSVCR_SL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSVCR_SL_A {
        match self.bits {
            false => LPSVCR_SL_A::LPSVCR_SL_0,
            true => LPSVCR_SL_A::LPSVCR_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSVCR_SL_0`"]
    #[inline(always)]
    pub fn is_lpsvcr_sl_0(&self) -> bool {
        *self == LPSVCR_SL_A::LPSVCR_SL_0
    }
    #[doc = "Checks if the value of the field is `LPSVCR_SL_1`"]
    #[inline(always)]
    pub fn is_lpsvcr_sl_1(&self) -> bool {
        *self == LPSVCR_SL_A::LPSVCR_SL_1
    }
}
#[doc = "Write proxy for field `LPSVCR_SL`"]
pub struct LPSVCR_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSVCR_SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSVCR_SL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn lpsvcr_sl_0(self) -> &'a mut W {
        self.variant(LPSVCR_SL_A::LPSVCR_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn lpsvcr_sl_1(self) -> &'a mut W {
        self.variant(LPSVCR_SL_A::LPSVCR_SL_1)
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
#[doc = "LP Tamper Detectors Configuration Register Soft Lock When set, prevents any writes to the LPTDCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTDCR_SL_A {
    #[doc = "0: Write access is allowed"]
    LPTDCR_SL_0 = 0,
    #[doc = "1: Write access is not allowed"]
    LPTDCR_SL_1 = 1,
}
impl From<LPTDCR_SL_A> for bool {
    #[inline(always)]
    fn from(variant: LPTDCR_SL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTDCR_SL`"]
pub type LPTDCR_SL_R = crate::R<bool, LPTDCR_SL_A>;
impl LPTDCR_SL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTDCR_SL_A {
        match self.bits {
            false => LPTDCR_SL_A::LPTDCR_SL_0,
            true => LPTDCR_SL_A::LPTDCR_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPTDCR_SL_0`"]
    #[inline(always)]
    pub fn is_lptdcr_sl_0(&self) -> bool {
        *self == LPTDCR_SL_A::LPTDCR_SL_0
    }
    #[doc = "Checks if the value of the field is `LPTDCR_SL_1`"]
    #[inline(always)]
    pub fn is_lptdcr_sl_1(&self) -> bool {
        *self == LPTDCR_SL_A::LPTDCR_SL_1
    }
}
#[doc = "Write proxy for field `LPTDCR_SL`"]
pub struct LPTDCR_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTDCR_SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTDCR_SL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn lptdcr_sl_0(self) -> &'a mut W {
        self.variant(LPTDCR_SL_A::LPTDCR_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn lptdcr_sl_1(self) -> &'a mut W {
        self.variant(LPTDCR_SL_A::LPTDCR_SL_1)
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
#[doc = "Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MKS_SL_A {
    #[doc = "0: Write access is allowed"]
    MKS_SL_0 = 0,
    #[doc = "1: Write access is not allowed"]
    MKS_SL_1 = 1,
}
impl From<MKS_SL_A> for bool {
    #[inline(always)]
    fn from(variant: MKS_SL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MKS_SL`"]
pub type MKS_SL_R = crate::R<bool, MKS_SL_A>;
impl MKS_SL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MKS_SL_A {
        match self.bits {
            false => MKS_SL_A::MKS_SL_0,
            true => MKS_SL_A::MKS_SL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MKS_SL_0`"]
    #[inline(always)]
    pub fn is_mks_sl_0(&self) -> bool {
        *self == MKS_SL_A::MKS_SL_0
    }
    #[doc = "Checks if the value of the field is `MKS_SL_1`"]
    #[inline(always)]
    pub fn is_mks_sl_1(&self) -> bool {
        *self == MKS_SL_A::MKS_SL_1
    }
}
#[doc = "Write proxy for field `MKS_SL`"]
pub struct MKS_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> MKS_SL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MKS_SL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn mks_sl_0(self) -> &'a mut W {
        self.variant(MKS_SL_A::MKS_SL_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn mks_sl_1(self) -> &'a mut W {
        self.variant(MKS_SL_A::MKS_SL_1)
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
#[doc = "HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPSVCR_L_A {
    #[doc = "0: Write access is allowed"]
    HPSVCR_L_0 = 0,
    #[doc = "1: Write access is not allowed"]
    HPSVCR_L_1 = 1,
}
impl From<HPSVCR_L_A> for bool {
    #[inline(always)]
    fn from(variant: HPSVCR_L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HPSVCR_L`"]
pub type HPSVCR_L_R = crate::R<bool, HPSVCR_L_A>;
impl HPSVCR_L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPSVCR_L_A {
        match self.bits {
            false => HPSVCR_L_A::HPSVCR_L_0,
            true => HPSVCR_L_A::HPSVCR_L_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPSVCR_L_0`"]
    #[inline(always)]
    pub fn is_hpsvcr_l_0(&self) -> bool {
        *self == HPSVCR_L_A::HPSVCR_L_0
    }
    #[doc = "Checks if the value of the field is `HPSVCR_L_1`"]
    #[inline(always)]
    pub fn is_hpsvcr_l_1(&self) -> bool {
        *self == HPSVCR_L_A::HPSVCR_L_1
    }
}
#[doc = "Write proxy for field `HPSVCR_L`"]
pub struct HPSVCR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> HPSVCR_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPSVCR_L_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn hpsvcr_l_0(self) -> &'a mut W {
        self.variant(HPSVCR_L_A::HPSVCR_L_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn hpsvcr_l_1(self) -> &'a mut W {
        self.variant(HPSVCR_L_A::HPSVCR_L_1)
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
#[doc = "HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPSICR_L_A {
    #[doc = "0: Write access is allowed"]
    HPSICR_L_0 = 0,
    #[doc = "1: Write access is not allowed"]
    HPSICR_L_1 = 1,
}
impl From<HPSICR_L_A> for bool {
    #[inline(always)]
    fn from(variant: HPSICR_L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HPSICR_L`"]
pub type HPSICR_L_R = crate::R<bool, HPSICR_L_A>;
impl HPSICR_L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPSICR_L_A {
        match self.bits {
            false => HPSICR_L_A::HPSICR_L_0,
            true => HPSICR_L_A::HPSICR_L_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPSICR_L_0`"]
    #[inline(always)]
    pub fn is_hpsicr_l_0(&self) -> bool {
        *self == HPSICR_L_A::HPSICR_L_0
    }
    #[doc = "Checks if the value of the field is `HPSICR_L_1`"]
    #[inline(always)]
    pub fn is_hpsicr_l_1(&self) -> bool {
        *self == HPSICR_L_A::HPSICR_L_1
    }
}
#[doc = "Write proxy for field `HPSICR_L`"]
pub struct HPSICR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> HPSICR_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPSICR_L_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn hpsicr_l_0(self) -> &'a mut W {
        self.variant(HPSICR_L_A::HPSICR_L_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn hpsicr_l_1(self) -> &'a mut W {
        self.variant(HPSICR_L_A::HPSICR_L_1)
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
#[doc = "High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAC_L_A {
    #[doc = "0: Write access is allowed"]
    HAC_L_0 = 0,
    #[doc = "1: Write access is not allowed"]
    HAC_L_1 = 1,
}
impl From<HAC_L_A> for bool {
    #[inline(always)]
    fn from(variant: HAC_L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HAC_L`"]
pub type HAC_L_R = crate::R<bool, HAC_L_A>;
impl HAC_L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAC_L_A {
        match self.bits {
            false => HAC_L_A::HAC_L_0,
            true => HAC_L_A::HAC_L_1,
        }
    }
    #[doc = "Checks if the value of the field is `HAC_L_0`"]
    #[inline(always)]
    pub fn is_hac_l_0(&self) -> bool {
        *self == HAC_L_A::HAC_L_0
    }
    #[doc = "Checks if the value of the field is `HAC_L_1`"]
    #[inline(always)]
    pub fn is_hac_l_1(&self) -> bool {
        *self == HAC_L_A::HAC_L_1
    }
}
#[doc = "Write proxy for field `HAC_L`"]
pub struct HAC_L_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HAC_L_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn hac_l_0(self) -> &'a mut W {
        self.variant(HAC_L_A::HAC_L_0)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn hac_l_1(self) -> &'a mut W {
        self.variant(HAC_L_A::HAC_L_1)
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
impl R {
    #[doc = "Bit 0 - Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_wsl(&self) -> ZMK_WSL_R {
        ZMK_WSL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_rsl(&self) -> ZMK_RSL_R {
        ZMK_RSL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub fn srtc_sl(&self) -> SRTC_SL_R {
        SRTC_SL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub fn lpcalb_sl(&self) -> LPCALB_SL_R {
        LPCALB_SL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub fn mc_sl(&self) -> MC_SL_R {
        MC_SL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub fn gpr_sl(&self) -> GPR_SL_R {
        GPR_SL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub fn lpsvcr_sl(&self) -> LPSVCR_SL_R {
        LPSVCR_SL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LP Tamper Detectors Configuration Register Soft Lock When set, prevents any writes to the LPTDCR"]
    #[inline(always)]
    pub fn lptdcr_sl(&self) -> LPTDCR_SL_R {
        LPTDCR_SL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline(always)]
    pub fn mks_sl(&self) -> MKS_SL_R {
        MKS_SL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline(always)]
    pub fn hpsvcr_l(&self) -> HPSVCR_L_R {
        HPSVCR_L_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline(always)]
    pub fn hpsicr_l(&self) -> HPSICR_L_R {
        HPSICR_L_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline(always)]
    pub fn hac_l(&self) -> HAC_L_R {
        HAC_L_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_wsl(&mut self) -> ZMK_WSL_W {
        ZMK_WSL_W { w: self }
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_rsl(&mut self) -> ZMK_RSL_W {
        ZMK_RSL_W { w: self }
    }
    #[doc = "Bit 2 - Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub fn srtc_sl(&mut self) -> SRTC_SL_W {
        SRTC_SL_W { w: self }
    }
    #[doc = "Bit 3 - LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub fn lpcalb_sl(&mut self) -> LPCALB_SL_W {
        LPCALB_SL_W { w: self }
    }
    #[doc = "Bit 4 - Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub fn mc_sl(&mut self) -> MC_SL_W {
        MC_SL_W { w: self }
    }
    #[doc = "Bit 5 - General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub fn gpr_sl(&mut self) -> GPR_SL_W {
        GPR_SL_W { w: self }
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub fn lpsvcr_sl(&mut self) -> LPSVCR_SL_W {
        LPSVCR_SL_W { w: self }
    }
    #[doc = "Bit 8 - LP Tamper Detectors Configuration Register Soft Lock When set, prevents any writes to the LPTDCR"]
    #[inline(always)]
    pub fn lptdcr_sl(&mut self) -> LPTDCR_SL_W {
        LPTDCR_SL_W { w: self }
    }
    #[doc = "Bit 9 - Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline(always)]
    pub fn mks_sl(&mut self) -> MKS_SL_W {
        MKS_SL_W { w: self }
    }
    #[doc = "Bit 16 - HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline(always)]
    pub fn hpsvcr_l(&mut self) -> HPSVCR_L_W {
        HPSVCR_L_W { w: self }
    }
    #[doc = "Bit 17 - HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline(always)]
    pub fn hpsicr_l(&mut self) -> HPSICR_L_W {
        HPSICR_L_W { w: self }
    }
    #[doc = "Bit 18 - High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline(always)]
    pub fn hac_l(&mut self) -> HAC_L_W {
        HAC_L_W { w: self }
    }
}
