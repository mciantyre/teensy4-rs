#[doc = "Reader of register LPSR"]
pub type R = crate::R<u32, super::LPSR>;
#[doc = "Writer for register LPSR"]
pub type W = crate::W<u32, super::LPSR>;
#[doc = "Register LPSR `reset()`'s with value 0x08"]
impl crate::ResetValue for super::LPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "LP Time Alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTA_A {
    #[doc = "0: No time alarm interrupt occurred."]
    LPTA_0 = 0,
    #[doc = "1: A time alarm interrupt occurred."]
    LPTA_1 = 1,
}
impl From<LPTA_A> for bool {
    #[inline(always)]
    fn from(variant: LPTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTA`"]
pub type LPTA_R = crate::R<bool, LPTA_A>;
impl LPTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTA_A {
        match self.bits {
            false => LPTA_A::LPTA_0,
            true => LPTA_A::LPTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPTA_0`"]
    #[inline(always)]
    pub fn is_lpta_0(&self) -> bool {
        *self == LPTA_A::LPTA_0
    }
    #[doc = "Checks if the value of the field is `LPTA_1`"]
    #[inline(always)]
    pub fn is_lpta_1(&self) -> bool {
        *self == LPTA_A::LPTA_1
    }
}
#[doc = "Write proxy for field `LPTA`"]
pub struct LPTA_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time alarm interrupt occurred."]
    #[inline(always)]
    pub fn lpta_0(self) -> &'a mut W {
        self.variant(LPTA_A::LPTA_0)
    }
    #[doc = "A time alarm interrupt occurred."]
    #[inline(always)]
    pub fn lpta_1(self) -> &'a mut W {
        self.variant(LPTA_A::LPTA_1)
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
#[doc = "Secure Real Time Counter Rollover\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTCR_A {
    #[doc = "0: SRTC has not reached its maximum value."]
    SRTCR_0 = 0,
    #[doc = "1: SRTC has reached its maximum value."]
    SRTCR_1 = 1,
}
impl From<SRTCR_A> for bool {
    #[inline(always)]
    fn from(variant: SRTCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRTCR`"]
pub type SRTCR_R = crate::R<bool, SRTCR_A>;
impl SRTCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTCR_A {
        match self.bits {
            false => SRTCR_A::SRTCR_0,
            true => SRTCR_A::SRTCR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTCR_0`"]
    #[inline(always)]
    pub fn is_srtcr_0(&self) -> bool {
        *self == SRTCR_A::SRTCR_0
    }
    #[doc = "Checks if the value of the field is `SRTCR_1`"]
    #[inline(always)]
    pub fn is_srtcr_1(&self) -> bool {
        *self == SRTCR_A::SRTCR_1
    }
}
#[doc = "Write proxy for field `SRTCR`"]
pub struct SRTCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRTCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRTC has not reached its maximum value."]
    #[inline(always)]
    pub fn srtcr_0(self) -> &'a mut W {
        self.variant(SRTCR_A::SRTCR_0)
    }
    #[doc = "SRTC has reached its maximum value."]
    #[inline(always)]
    pub fn srtcr_1(self) -> &'a mut W {
        self.variant(SRTCR_A::SRTCR_1)
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
#[doc = "Monotonic Counter Rollover\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCR_A {
    #[doc = "0: MC has not reached its maximum value."]
    MCR_0 = 0,
    #[doc = "1: MC has reached its maximum value."]
    MCR_1 = 1,
}
impl From<MCR_A> for bool {
    #[inline(always)]
    fn from(variant: MCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCR`"]
pub type MCR_R = crate::R<bool, MCR_A>;
impl MCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCR_A {
        match self.bits {
            false => MCR_A::MCR_0,
            true => MCR_A::MCR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCR_0`"]
    #[inline(always)]
    pub fn is_mcr_0(&self) -> bool {
        *self == MCR_A::MCR_0
    }
    #[doc = "Checks if the value of the field is `MCR_1`"]
    #[inline(always)]
    pub fn is_mcr_1(&self) -> bool {
        *self == MCR_A::MCR_1
    }
}
#[doc = "Write proxy for field `MCR`"]
pub struct MCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MC has not reached its maximum value."]
    #[inline(always)]
    pub fn mcr_0(self) -> &'a mut W {
        self.variant(MCR_A::MCR_0)
    }
    #[doc = "MC has reached its maximum value."]
    #[inline(always)]
    pub fn mcr_1(self) -> &'a mut W {
        self.variant(MCR_A::MCR_1)
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
#[doc = "Reader of field `PGD`"]
pub type PGD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGD`"]
pub struct PGD_W<'a> {
    w: &'a mut W,
}
impl<'a> PGD_W<'a> {
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
#[doc = "External Tampering 1 Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET1D_A {
    #[doc = "0: External tampering 1 not detected."]
    ET1D_0 = 0,
    #[doc = "1: External tampering 1 detected."]
    ET1D_1 = 1,
}
impl From<ET1D_A> for bool {
    #[inline(always)]
    fn from(variant: ET1D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ET1D`"]
pub type ET1D_R = crate::R<bool, ET1D_A>;
impl ET1D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ET1D_A {
        match self.bits {
            false => ET1D_A::ET1D_0,
            true => ET1D_A::ET1D_1,
        }
    }
    #[doc = "Checks if the value of the field is `ET1D_0`"]
    #[inline(always)]
    pub fn is_et1d_0(&self) -> bool {
        *self == ET1D_A::ET1D_0
    }
    #[doc = "Checks if the value of the field is `ET1D_1`"]
    #[inline(always)]
    pub fn is_et1d_1(&self) -> bool {
        *self == ET1D_A::ET1D_1
    }
}
#[doc = "Write proxy for field `ET1D`"]
pub struct ET1D_W<'a> {
    w: &'a mut W,
}
impl<'a> ET1D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ET1D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External tampering 1 not detected."]
    #[inline(always)]
    pub fn et1d_0(self) -> &'a mut W {
        self.variant(ET1D_A::ET1D_0)
    }
    #[doc = "External tampering 1 detected."]
    #[inline(always)]
    pub fn et1d_1(self) -> &'a mut W {
        self.variant(ET1D_A::ET1D_1)
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
#[doc = "External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESVD_A {
    #[doc = "0: No external security violation."]
    ESVD_0 = 0,
    #[doc = "1: External security violation is detected."]
    ESVD_1 = 1,
}
impl From<ESVD_A> for bool {
    #[inline(always)]
    fn from(variant: ESVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ESVD`"]
pub type ESVD_R = crate::R<bool, ESVD_A>;
impl ESVD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESVD_A {
        match self.bits {
            false => ESVD_A::ESVD_0,
            true => ESVD_A::ESVD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESVD_0`"]
    #[inline(always)]
    pub fn is_esvd_0(&self) -> bool {
        *self == ESVD_A::ESVD_0
    }
    #[doc = "Checks if the value of the field is `ESVD_1`"]
    #[inline(always)]
    pub fn is_esvd_1(&self) -> bool {
        *self == ESVD_A::ESVD_1
    }
}
#[doc = "Write proxy for field `ESVD`"]
pub struct ESVD_W<'a> {
    w: &'a mut W,
}
impl<'a> ESVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESVD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No external security violation."]
    #[inline(always)]
    pub fn esvd_0(self) -> &'a mut W {
        self.variant(ESVD_A::ESVD_0)
    }
    #[doc = "External security violation is detected."]
    #[inline(always)]
    pub fn esvd_1(self) -> &'a mut W {
        self.variant(ESVD_A::ESVD_1)
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
#[doc = "Emergency Off This bit is set when a power off is requested.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EO_A {
    #[doc = "0: Emergency off was not detected."]
    EO_0 = 0,
    #[doc = "1: Emergency off was detected."]
    EO_1 = 1,
}
impl From<EO_A> for bool {
    #[inline(always)]
    fn from(variant: EO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EO`"]
pub type EO_R = crate::R<bool, EO_A>;
impl EO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EO_A {
        match self.bits {
            false => EO_A::EO_0,
            true => EO_A::EO_1,
        }
    }
    #[doc = "Checks if the value of the field is `EO_0`"]
    #[inline(always)]
    pub fn is_eo_0(&self) -> bool {
        *self == EO_A::EO_0
    }
    #[doc = "Checks if the value of the field is `EO_1`"]
    #[inline(always)]
    pub fn is_eo_1(&self) -> bool {
        *self == EO_A::EO_1
    }
}
#[doc = "Write proxy for field `EO`"]
pub struct EO_W<'a> {
    w: &'a mut W,
}
impl<'a> EO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Emergency off was not detected."]
    #[inline(always)]
    pub fn eo_0(self) -> &'a mut W {
        self.variant(EO_A::EO_0)
    }
    #[doc = "Emergency off was detected."]
    #[inline(always)]
    pub fn eo_1(self) -> &'a mut W {
        self.variant(EO_A::EO_1)
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
#[doc = "Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPO_A {
    #[doc = "0: Set Power Off was not detected."]
    SPO_0 = 0,
    #[doc = "1: Set Power Off was detected."]
    SPO_1 = 1,
}
impl From<SPO_A> for bool {
    #[inline(always)]
    fn from(variant: SPO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPO`"]
pub type SPO_R = crate::R<bool, SPO_A>;
impl SPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPO_A {
        match self.bits {
            false => SPO_A::SPO_0,
            true => SPO_A::SPO_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPO_0`"]
    #[inline(always)]
    pub fn is_spo_0(&self) -> bool {
        *self == SPO_A::SPO_0
    }
    #[doc = "Checks if the value of the field is `SPO_1`"]
    #[inline(always)]
    pub fn is_spo_1(&self) -> bool {
        *self == SPO_A::SPO_1
    }
}
#[doc = "Write proxy for field `SPO`"]
pub struct SPO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set Power Off was not detected."]
    #[inline(always)]
    pub fn spo_0(self) -> &'a mut W {
        self.variant(SPO_A::SPO_0)
    }
    #[doc = "Set Power Off was detected."]
    #[inline(always)]
    pub fn spo_1(self) -> &'a mut W {
        self.variant(SPO_A::SPO_1)
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
#[doc = "Scan Exit Detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SED_A {
    #[doc = "0: Scan exit was not detected."]
    SED_0 = 0,
    #[doc = "1: Scan exit was detected."]
    SED_1 = 1,
}
impl From<SED_A> for bool {
    #[inline(always)]
    fn from(variant: SED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SED`"]
pub type SED_R = crate::R<bool, SED_A>;
impl SED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SED_A {
        match self.bits {
            false => SED_A::SED_0,
            true => SED_A::SED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SED_0`"]
    #[inline(always)]
    pub fn is_sed_0(&self) -> bool {
        *self == SED_A::SED_0
    }
    #[doc = "Checks if the value of the field is `SED_1`"]
    #[inline(always)]
    pub fn is_sed_1(&self) -> bool {
        *self == SED_A::SED_1
    }
}
#[doc = "Write proxy for field `SED`"]
pub struct SED_W<'a> {
    w: &'a mut W,
}
impl<'a> SED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Scan exit was not detected."]
    #[inline(always)]
    pub fn sed_0(self) -> &'a mut W {
        self.variant(SED_A::SED_0)
    }
    #[doc = "Scan exit was detected."]
    #[inline(always)]
    pub fn sed_1(self) -> &'a mut W {
        self.variant(SED_A::SED_1)
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
#[doc = "LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPNS_A {
    #[doc = "0: LP section was not programmed in the non-secure state."]
    LPNS_0 = 0,
    #[doc = "1: LP section was programmed in the non-secure state."]
    LPNS_1 = 1,
}
impl From<LPNS_A> for bool {
    #[inline(always)]
    fn from(variant: LPNS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPNS`"]
pub type LPNS_R = crate::R<bool, LPNS_A>;
impl LPNS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPNS_A {
        match self.bits {
            false => LPNS_A::LPNS_0,
            true => LPNS_A::LPNS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPNS_0`"]
    #[inline(always)]
    pub fn is_lpns_0(&self) -> bool {
        *self == LPNS_A::LPNS_0
    }
    #[doc = "Checks if the value of the field is `LPNS_1`"]
    #[inline(always)]
    pub fn is_lpns_1(&self) -> bool {
        *self == LPNS_A::LPNS_1
    }
}
#[doc = "LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPS_A {
    #[doc = "0: LP section was not programmed in secure or trusted state."]
    LPS_0 = 0,
    #[doc = "1: LP section was programmed in secure or trusted state."]
    LPS_1 = 1,
}
impl From<LPS_A> for bool {
    #[inline(always)]
    fn from(variant: LPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPS`"]
pub type LPS_R = crate::R<bool, LPS_A>;
impl LPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPS_A {
        match self.bits {
            false => LPS_A::LPS_0,
            true => LPS_A::LPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPS_0`"]
    #[inline(always)]
    pub fn is_lps_0(&self) -> bool {
        *self == LPS_A::LPS_0
    }
    #[doc = "Checks if the value of the field is `LPS_1`"]
    #[inline(always)]
    pub fn is_lps_1(&self) -> bool {
        *self == LPS_A::LPS_1
    }
}
impl R {
    #[doc = "Bit 0 - LP Time Alarm"]
    #[inline(always)]
    pub fn lpta(&self) -> LPTA_R {
        LPTA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure Real Time Counter Rollover"]
    #[inline(always)]
    pub fn srtcr(&self) -> SRTCR_R {
        SRTCR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Monotonic Counter Rollover"]
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power Supply Glitch Detected 0 No power supply glitch. 1 Power supply glitch is detected."]
    #[inline(always)]
    pub fn pgd(&self) -> PGD_R {
        PGD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Tampering 1 Detected"]
    #[inline(always)]
    pub fn et1d(&self) -> ET1D_R {
        ET1D_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline(always)]
    pub fn esvd(&self) -> ESVD_R {
        ESVD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Emergency Off This bit is set when a power off is requested."]
    #[inline(always)]
    pub fn eo(&self) -> EO_R {
        EO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Scan Exit Detected"]
    #[inline(always)]
    pub fn sed(&self) -> SED_R {
        SED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
    #[inline(always)]
    pub fn lpns(&self) -> LPNS_R {
        LPNS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
    #[inline(always)]
    pub fn lps(&self) -> LPS_R {
        LPS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LP Time Alarm"]
    #[inline(always)]
    pub fn lpta(&mut self) -> LPTA_W {
        LPTA_W { w: self }
    }
    #[doc = "Bit 1 - Secure Real Time Counter Rollover"]
    #[inline(always)]
    pub fn srtcr(&mut self) -> SRTCR_W {
        SRTCR_W { w: self }
    }
    #[doc = "Bit 2 - Monotonic Counter Rollover"]
    #[inline(always)]
    pub fn mcr(&mut self) -> MCR_W {
        MCR_W { w: self }
    }
    #[doc = "Bit 3 - Power Supply Glitch Detected 0 No power supply glitch. 1 Power supply glitch is detected."]
    #[inline(always)]
    pub fn pgd(&mut self) -> PGD_W {
        PGD_W { w: self }
    }
    #[doc = "Bit 9 - External Tampering 1 Detected"]
    #[inline(always)]
    pub fn et1d(&mut self) -> ET1D_W {
        ET1D_W { w: self }
    }
    #[doc = "Bit 16 - External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline(always)]
    pub fn esvd(&mut self) -> ESVD_W {
        ESVD_W { w: self }
    }
    #[doc = "Bit 17 - Emergency Off This bit is set when a power off is requested."]
    #[inline(always)]
    pub fn eo(&mut self) -> EO_W {
        EO_W { w: self }
    }
    #[doc = "Bit 18 - Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub fn spo(&mut self) -> SPO_W {
        SPO_W { w: self }
    }
    #[doc = "Bit 20 - Scan Exit Detected"]
    #[inline(always)]
    pub fn sed(&mut self) -> SED_W {
        SED_W { w: self }
    }
}
