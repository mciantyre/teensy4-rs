#[doc = "Reader of register LPLR"]
pub type R = crate::R<u32, super::LPLR>;
#[doc = "Writer for register LPLR"]
pub type W = crate::W<u32, super::LPLR>;
#[doc = "Register LPLR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_WHL_A {
    #[doc = "0: Write access is allowed."]
    ZMK_WHL_0 = 0,
    #[doc = "1: Write access is not allowed."]
    ZMK_WHL_1 = 1,
}
impl From<ZMK_WHL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_WHL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_WHL`"]
pub type ZMK_WHL_R = crate::R<bool, ZMK_WHL_A>;
impl ZMK_WHL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_WHL_A {
        match self.bits {
            false => ZMK_WHL_A::ZMK_WHL_0,
            true => ZMK_WHL_A::ZMK_WHL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_WHL_0`"]
    #[inline(always)]
    pub fn is_zmk_whl_0(&self) -> bool {
        *self == ZMK_WHL_A::ZMK_WHL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_WHL_1`"]
    #[inline(always)]
    pub fn is_zmk_whl_1(&self) -> bool {
        *self == ZMK_WHL_A::ZMK_WHL_1
    }
}
#[doc = "Write proxy for field `ZMK_WHL`"]
pub struct ZMK_WHL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_WHL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZMK_WHL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn zmk_whl_0(self) -> &'a mut W {
        self.variant(ZMK_WHL_A::ZMK_WHL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn zmk_whl_1(self) -> &'a mut W {
        self.variant(ZMK_WHL_A::ZMK_WHL_1)
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
#[doc = "Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_RHL_A {
    #[doc = "0: Read access is allowed (only in software programming mode)."]
    ZMK_RHL_0 = 0,
    #[doc = "1: Read access is not allowed."]
    ZMK_RHL_1 = 1,
}
impl From<ZMK_RHL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_RHL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_RHL`"]
pub type ZMK_RHL_R = crate::R<bool, ZMK_RHL_A>;
impl ZMK_RHL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_RHL_A {
        match self.bits {
            false => ZMK_RHL_A::ZMK_RHL_0,
            true => ZMK_RHL_A::ZMK_RHL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_RHL_0`"]
    #[inline(always)]
    pub fn is_zmk_rhl_0(&self) -> bool {
        *self == ZMK_RHL_A::ZMK_RHL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_RHL_1`"]
    #[inline(always)]
    pub fn is_zmk_rhl_1(&self) -> bool {
        *self == ZMK_RHL_A::ZMK_RHL_1
    }
}
#[doc = "Write proxy for field `ZMK_RHL`"]
pub struct ZMK_RHL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_RHL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZMK_RHL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access is allowed (only in software programming mode)."]
    #[inline(always)]
    pub fn zmk_rhl_0(self) -> &'a mut W {
        self.variant(ZMK_RHL_A::ZMK_RHL_0)
    }
    #[doc = "Read access is not allowed."]
    #[inline(always)]
    pub fn zmk_rhl_1(self) -> &'a mut W {
        self.variant(ZMK_RHL_A::ZMK_RHL_1)
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
#[doc = "Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTC_HL_A {
    #[doc = "0: Write access is allowed."]
    SRTC_HL_0 = 0,
    #[doc = "1: Write access is not allowed."]
    SRTC_HL_1 = 1,
}
impl From<SRTC_HL_A> for bool {
    #[inline(always)]
    fn from(variant: SRTC_HL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRTC_HL`"]
pub type SRTC_HL_R = crate::R<bool, SRTC_HL_A>;
impl SRTC_HL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTC_HL_A {
        match self.bits {
            false => SRTC_HL_A::SRTC_HL_0,
            true => SRTC_HL_A::SRTC_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTC_HL_0`"]
    #[inline(always)]
    pub fn is_srtc_hl_0(&self) -> bool {
        *self == SRTC_HL_A::SRTC_HL_0
    }
    #[doc = "Checks if the value of the field is `SRTC_HL_1`"]
    #[inline(always)]
    pub fn is_srtc_hl_1(&self) -> bool {
        *self == SRTC_HL_A::SRTC_HL_1
    }
}
#[doc = "Write proxy for field `SRTC_HL`"]
pub struct SRTC_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTC_HL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRTC_HL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn srtc_hl_0(self) -> &'a mut W {
        self.variant(SRTC_HL_A::SRTC_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn srtc_hl_1(self) -> &'a mut W {
        self.variant(SRTC_HL_A::SRTC_HL_1)
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
#[doc = "LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCALB_HL_A {
    #[doc = "0: Write access is allowed."]
    LPCALB_HL_0 = 0,
    #[doc = "1: Write access is not allowed."]
    LPCALB_HL_1 = 1,
}
impl From<LPCALB_HL_A> for bool {
    #[inline(always)]
    fn from(variant: LPCALB_HL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPCALB_HL`"]
pub type LPCALB_HL_R = crate::R<bool, LPCALB_HL_A>;
impl LPCALB_HL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCALB_HL_A {
        match self.bits {
            false => LPCALB_HL_A::LPCALB_HL_0,
            true => LPCALB_HL_A::LPCALB_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPCALB_HL_0`"]
    #[inline(always)]
    pub fn is_lpcalb_hl_0(&self) -> bool {
        *self == LPCALB_HL_A::LPCALB_HL_0
    }
    #[doc = "Checks if the value of the field is `LPCALB_HL_1`"]
    #[inline(always)]
    pub fn is_lpcalb_hl_1(&self) -> bool {
        *self == LPCALB_HL_A::LPCALB_HL_1
    }
}
#[doc = "Write proxy for field `LPCALB_HL`"]
pub struct LPCALB_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCALB_HL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCALB_HL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn lpcalb_hl_0(self) -> &'a mut W {
        self.variant(LPCALB_HL_A::LPCALB_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn lpcalb_hl_1(self) -> &'a mut W {
        self.variant(LPCALB_HL_A::LPCALB_HL_1)
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
#[doc = "Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MC_HL_A {
    #[doc = "0: Write access (increment) is allowed."]
    MC_HL_0 = 0,
    #[doc = "1: Write access (increment) is not allowed."]
    MC_HL_1 = 1,
}
impl From<MC_HL_A> for bool {
    #[inline(always)]
    fn from(variant: MC_HL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MC_HL`"]
pub type MC_HL_R = crate::R<bool, MC_HL_A>;
impl MC_HL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_HL_A {
        match self.bits {
            false => MC_HL_A::MC_HL_0,
            true => MC_HL_A::MC_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MC_HL_0`"]
    #[inline(always)]
    pub fn is_mc_hl_0(&self) -> bool {
        *self == MC_HL_A::MC_HL_0
    }
    #[doc = "Checks if the value of the field is `MC_HL_1`"]
    #[inline(always)]
    pub fn is_mc_hl_1(&self) -> bool {
        *self == MC_HL_A::MC_HL_1
    }
}
#[doc = "Write proxy for field `MC_HL`"]
pub struct MC_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_HL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_HL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access (increment) is allowed."]
    #[inline(always)]
    pub fn mc_hl_0(self) -> &'a mut W {
        self.variant(MC_HL_A::MC_HL_0)
    }
    #[doc = "Write access (increment) is not allowed."]
    #[inline(always)]
    pub fn mc_hl_1(self) -> &'a mut W {
        self.variant(MC_HL_A::MC_HL_1)
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
#[doc = "General Purpose Register Hard Lock When set, prevents any writes to the GPR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPR_HL_A {
    #[doc = "0: Write access is allowed."]
    GPR_HL_0 = 0,
    #[doc = "1: Write access is not allowed."]
    GPR_HL_1 = 1,
}
impl From<GPR_HL_A> for bool {
    #[inline(always)]
    fn from(variant: GPR_HL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPR_HL`"]
pub type GPR_HL_R = crate::R<bool, GPR_HL_A>;
impl GPR_HL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPR_HL_A {
        match self.bits {
            false => GPR_HL_A::GPR_HL_0,
            true => GPR_HL_A::GPR_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPR_HL_0`"]
    #[inline(always)]
    pub fn is_gpr_hl_0(&self) -> bool {
        *self == GPR_HL_A::GPR_HL_0
    }
    #[doc = "Checks if the value of the field is `GPR_HL_1`"]
    #[inline(always)]
    pub fn is_gpr_hl_1(&self) -> bool {
        *self == GPR_HL_A::GPR_HL_1
    }
}
#[doc = "Write proxy for field `GPR_HL`"]
pub struct GPR_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPR_HL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPR_HL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn gpr_hl_0(self) -> &'a mut W {
        self.variant(GPR_HL_A::GPR_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn gpr_hl_1(self) -> &'a mut W {
        self.variant(GPR_HL_A::GPR_HL_1)
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
#[doc = "LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSVCR_HL_A {
    #[doc = "0: Write access is allowed."]
    LPSVCR_HL_0 = 0,
    #[doc = "1: Write access is not allowed."]
    LPSVCR_HL_1 = 1,
}
impl From<LPSVCR_HL_A> for bool {
    #[inline(always)]
    fn from(variant: LPSVCR_HL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSVCR_HL`"]
pub type LPSVCR_HL_R = crate::R<bool, LPSVCR_HL_A>;
impl LPSVCR_HL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSVCR_HL_A {
        match self.bits {
            false => LPSVCR_HL_A::LPSVCR_HL_0,
            true => LPSVCR_HL_A::LPSVCR_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSVCR_HL_0`"]
    #[inline(always)]
    pub fn is_lpsvcr_hl_0(&self) -> bool {
        *self == LPSVCR_HL_A::LPSVCR_HL_0
    }
    #[doc = "Checks if the value of the field is `LPSVCR_HL_1`"]
    #[inline(always)]
    pub fn is_lpsvcr_hl_1(&self) -> bool {
        *self == LPSVCR_HL_A::LPSVCR_HL_1
    }
}
#[doc = "Write proxy for field `LPSVCR_HL`"]
pub struct LPSVCR_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSVCR_HL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSVCR_HL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn lpsvcr_hl_0(self) -> &'a mut W {
        self.variant(LPSVCR_HL_A::LPSVCR_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn lpsvcr_hl_1(self) -> &'a mut W {
        self.variant(LPSVCR_HL_A::LPSVCR_HL_1)
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
#[doc = "LP Tamper Detectors Configuration Register Hard Lock When set, prevents any writes to the LPTDCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTDCR_HL_A {
    #[doc = "0: Write access is allowed."]
    LPTDCR_HL_0 = 0,
    #[doc = "1: Write access is not allowed."]
    LPTDCR_HL_1 = 1,
}
impl From<LPTDCR_HL_A> for bool {
    #[inline(always)]
    fn from(variant: LPTDCR_HL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTDCR_HL`"]
pub type LPTDCR_HL_R = crate::R<bool, LPTDCR_HL_A>;
impl LPTDCR_HL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTDCR_HL_A {
        match self.bits {
            false => LPTDCR_HL_A::LPTDCR_HL_0,
            true => LPTDCR_HL_A::LPTDCR_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPTDCR_HL_0`"]
    #[inline(always)]
    pub fn is_lptdcr_hl_0(&self) -> bool {
        *self == LPTDCR_HL_A::LPTDCR_HL_0
    }
    #[doc = "Checks if the value of the field is `LPTDCR_HL_1`"]
    #[inline(always)]
    pub fn is_lptdcr_hl_1(&self) -> bool {
        *self == LPTDCR_HL_A::LPTDCR_HL_1
    }
}
#[doc = "Write proxy for field `LPTDCR_HL`"]
pub struct LPTDCR_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTDCR_HL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTDCR_HL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn lptdcr_hl_0(self) -> &'a mut W {
        self.variant(LPTDCR_HL_A::LPTDCR_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn lptdcr_hl_1(self) -> &'a mut W {
        self.variant(LPTDCR_HL_A::LPTDCR_HL_1)
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
#[doc = "Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MKS_HL_A {
    #[doc = "0: Write access is allowed."]
    MKS_HL_0 = 0,
    #[doc = "1: Write access is not allowed."]
    MKS_HL_1 = 1,
}
impl From<MKS_HL_A> for bool {
    #[inline(always)]
    fn from(variant: MKS_HL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MKS_HL`"]
pub type MKS_HL_R = crate::R<bool, MKS_HL_A>;
impl MKS_HL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MKS_HL_A {
        match self.bits {
            false => MKS_HL_A::MKS_HL_0,
            true => MKS_HL_A::MKS_HL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MKS_HL_0`"]
    #[inline(always)]
    pub fn is_mks_hl_0(&self) -> bool {
        *self == MKS_HL_A::MKS_HL_0
    }
    #[doc = "Checks if the value of the field is `MKS_HL_1`"]
    #[inline(always)]
    pub fn is_mks_hl_1(&self) -> bool {
        *self == MKS_HL_A::MKS_HL_1
    }
}
#[doc = "Write proxy for field `MKS_HL`"]
pub struct MKS_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> MKS_HL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MKS_HL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn mks_hl_0(self) -> &'a mut W {
        self.variant(MKS_HL_A::MKS_HL_0)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn mks_hl_1(self) -> &'a mut W {
        self.variant(MKS_HL_A::MKS_HL_1)
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
impl R {
    #[doc = "Bit 0 - Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_whl(&self) -> ZMK_WHL_R {
        ZMK_WHL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_rhl(&self) -> ZMK_RHL_R {
        ZMK_RHL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub fn srtc_hl(&self) -> SRTC_HL_R {
        SRTC_HL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub fn lpcalb_hl(&self) -> LPCALB_HL_R {
        LPCALB_HL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub fn mc_hl(&self) -> MC_HL_R {
        MC_HL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub fn gpr_hl(&self) -> GPR_HL_R {
        GPR_HL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub fn lpsvcr_hl(&self) -> LPSVCR_HL_R {
        LPSVCR_HL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LP Tamper Detectors Configuration Register Hard Lock When set, prevents any writes to the LPTDCR"]
    #[inline(always)]
    pub fn lptdcr_hl(&self) -> LPTDCR_HL_R {
        LPTDCR_HL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline(always)]
    pub fn mks_hl(&self) -> MKS_HL_R {
        MKS_HL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_whl(&mut self) -> ZMK_WHL_W {
        ZMK_WHL_W { w: self }
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_rhl(&mut self) -> ZMK_RHL_W {
        ZMK_RHL_W { w: self }
    }
    #[doc = "Bit 2 - Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub fn srtc_hl(&mut self) -> SRTC_HL_W {
        SRTC_HL_W { w: self }
    }
    #[doc = "Bit 3 - LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub fn lpcalb_hl(&mut self) -> LPCALB_HL_W {
        LPCALB_HL_W { w: self }
    }
    #[doc = "Bit 4 - Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub fn mc_hl(&mut self) -> MC_HL_W {
        MC_HL_W { w: self }
    }
    #[doc = "Bit 5 - General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub fn gpr_hl(&mut self) -> GPR_HL_W {
        GPR_HL_W { w: self }
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub fn lpsvcr_hl(&mut self) -> LPSVCR_HL_W {
        LPSVCR_HL_W { w: self }
    }
    #[doc = "Bit 8 - LP Tamper Detectors Configuration Register Hard Lock When set, prevents any writes to the LPTDCR"]
    #[inline(always)]
    pub fn lptdcr_hl(&mut self) -> LPTDCR_HL_W {
        LPTDCR_HL_W { w: self }
    }
    #[doc = "Bit 9 - Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline(always)]
    pub fn mks_hl(&mut self) -> MKS_HL_W {
        MKS_HL_W { w: self }
    }
}
