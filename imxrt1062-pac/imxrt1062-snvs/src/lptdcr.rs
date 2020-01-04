#[doc = "Reader of register LPTDCR"]
pub type R = crate::R<u32, super::LPTDCR>;
#[doc = "Writer for register LPTDCR"]
pub type W = crate::W<u32, super::LPTDCR>;
#[doc = "Register LPTDCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPTDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTCR_EN_A {
    #[doc = "0: SRTC rollover is disabled."]
    SRTCR_EN_0 = 0,
    #[doc = "1: SRTC rollover is enabled."]
    SRTCR_EN_1 = 1,
}
impl From<SRTCR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRTCR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRTCR_EN`"]
pub type SRTCR_EN_R = crate::R<bool, SRTCR_EN_A>;
impl SRTCR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTCR_EN_A {
        match self.bits {
            false => SRTCR_EN_A::SRTCR_EN_0,
            true => SRTCR_EN_A::SRTCR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTCR_EN_0`"]
    #[inline(always)]
    pub fn is_srtcr_en_0(&self) -> bool {
        *self == SRTCR_EN_A::SRTCR_EN_0
    }
    #[doc = "Checks if the value of the field is `SRTCR_EN_1`"]
    #[inline(always)]
    pub fn is_srtcr_en_1(&self) -> bool {
        *self == SRTCR_EN_A::SRTCR_EN_1
    }
}
#[doc = "Write proxy for field `SRTCR_EN`"]
pub struct SRTCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTCR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRTCR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRTC rollover is disabled."]
    #[inline(always)]
    pub fn srtcr_en_0(self) -> &'a mut W {
        self.variant(SRTCR_EN_A::SRTCR_EN_0)
    }
    #[doc = "SRTC rollover is enabled."]
    #[inline(always)]
    pub fn srtcr_en_1(self) -> &'a mut W {
        self.variant(SRTCR_EN_A::SRTCR_EN_1)
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
#[doc = "MC Rollover Enable When set, an MC Rollover event generates an LP security violation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCR_EN_A {
    #[doc = "0: MC rollover is disabled."]
    MCR_EN_0 = 0,
    #[doc = "1: MC rollover is enabled."]
    MCR_EN_1 = 1,
}
impl From<MCR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MCR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCR_EN`"]
pub type MCR_EN_R = crate::R<bool, MCR_EN_A>;
impl MCR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCR_EN_A {
        match self.bits {
            false => MCR_EN_A::MCR_EN_0,
            true => MCR_EN_A::MCR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCR_EN_0`"]
    #[inline(always)]
    pub fn is_mcr_en_0(&self) -> bool {
        *self == MCR_EN_A::MCR_EN_0
    }
    #[doc = "Checks if the value of the field is `MCR_EN_1`"]
    #[inline(always)]
    pub fn is_mcr_en_1(&self) -> bool {
        *self == MCR_EN_A::MCR_EN_1
    }
}
#[doc = "Write proxy for field `MCR_EN`"]
pub struct MCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MC rollover is disabled."]
    #[inline(always)]
    pub fn mcr_en_0(self) -> &'a mut W {
        self.variant(MCR_EN_A::MCR_EN_0)
    }
    #[doc = "MC rollover is enabled."]
    #[inline(always)]
    pub fn mcr_en_1(self) -> &'a mut W {
        self.variant(MCR_EN_A::MCR_EN_1)
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
#[doc = "External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET1_EN_A {
    #[doc = "0: External tamper 1 is disabled."]
    ET1_EN_0 = 0,
    #[doc = "1: External tamper 1 is enabled."]
    ET1_EN_1 = 1,
}
impl From<ET1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ET1_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ET1_EN`"]
pub type ET1_EN_R = crate::R<bool, ET1_EN_A>;
impl ET1_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ET1_EN_A {
        match self.bits {
            false => ET1_EN_A::ET1_EN_0,
            true => ET1_EN_A::ET1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ET1_EN_0`"]
    #[inline(always)]
    pub fn is_et1_en_0(&self) -> bool {
        *self == ET1_EN_A::ET1_EN_0
    }
    #[doc = "Checks if the value of the field is `ET1_EN_1`"]
    #[inline(always)]
    pub fn is_et1_en_1(&self) -> bool {
        *self == ET1_EN_A::ET1_EN_1
    }
}
#[doc = "Write proxy for field `ET1_EN`"]
pub struct ET1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ET1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ET1_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External tamper 1 is disabled."]
    #[inline(always)]
    pub fn et1_en_0(self) -> &'a mut W {
        self.variant(ET1_EN_A::ET1_EN_0)
    }
    #[doc = "External tamper 1 is enabled."]
    #[inline(always)]
    pub fn et1_en_1(self) -> &'a mut W {
        self.variant(ET1_EN_A::ET1_EN_1)
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
#[doc = "External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ET1P_A {
    #[doc = "0: External tamper 1 is active low."]
    ET1P_0 = 0,
    #[doc = "1: External tamper 1 is active high."]
    ET1P_1 = 1,
}
impl From<ET1P_A> for bool {
    #[inline(always)]
    fn from(variant: ET1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ET1P`"]
pub type ET1P_R = crate::R<bool, ET1P_A>;
impl ET1P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ET1P_A {
        match self.bits {
            false => ET1P_A::ET1P_0,
            true => ET1P_A::ET1P_1,
        }
    }
    #[doc = "Checks if the value of the field is `ET1P_0`"]
    #[inline(always)]
    pub fn is_et1p_0(&self) -> bool {
        *self == ET1P_A::ET1P_0
    }
    #[doc = "Checks if the value of the field is `ET1P_1`"]
    #[inline(always)]
    pub fn is_et1p_1(&self) -> bool {
        *self == ET1P_A::ET1P_1
    }
}
#[doc = "Write proxy for field `ET1P`"]
pub struct ET1P_W<'a> {
    w: &'a mut W,
}
impl<'a> ET1P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ET1P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External tamper 1 is active low."]
    #[inline(always)]
    pub fn et1p_0(self) -> &'a mut W {
        self.variant(ET1P_A::ET1P_0)
    }
    #[doc = "External tamper 1 is active high."]
    #[inline(always)]
    pub fn et1p_1(self) -> &'a mut W {
        self.variant(ET1P_A::ET1P_1)
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
#[doc = "Reader of field `PFD_OBSERV`"]
pub type PFD_OBSERV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFD_OBSERV`"]
pub struct PFD_OBSERV_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD_OBSERV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `POR_OBSERV`"]
pub type POR_OBSERV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POR_OBSERV`"]
pub struct POR_OBSERV_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_OBSERV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCB_A {
    #[doc = "0: Normal SRTC clock oscillator not bypassed."]
    OSCB_0 = 0,
    #[doc = "1: Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source."]
    OSCB_1 = 1,
}
impl From<OSCB_A> for bool {
    #[inline(always)]
    fn from(variant: OSCB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCB`"]
pub type OSCB_R = crate::R<bool, OSCB_A>;
impl OSCB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCB_A {
        match self.bits {
            false => OSCB_A::OSCB_0,
            true => OSCB_A::OSCB_1,
        }
    }
    #[doc = "Checks if the value of the field is `OSCB_0`"]
    #[inline(always)]
    pub fn is_oscb_0(&self) -> bool {
        *self == OSCB_A::OSCB_0
    }
    #[doc = "Checks if the value of the field is `OSCB_1`"]
    #[inline(always)]
    pub fn is_oscb_1(&self) -> bool {
        *self == OSCB_A::OSCB_1
    }
}
#[doc = "Write proxy for field `OSCB`"]
pub struct OSCB_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal SRTC clock oscillator not bypassed."]
    #[inline(always)]
    pub fn oscb_0(self) -> &'a mut W {
        self.variant(OSCB_A::OSCB_0)
    }
    #[doc = "Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source."]
    #[inline(always)]
    pub fn oscb_1(self) -> &'a mut W {
        self.variant(OSCB_A::OSCB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline(always)]
    pub fn srtcr_en(&self) -> SRTCR_EN_R {
        SRTCR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline(always)]
    pub fn mcr_en(&self) -> MCR_EN_R {
        MCR_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    #[inline(always)]
    pub fn et1_en(&self) -> ET1_EN_R {
        ET1_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    #[inline(always)]
    pub fn et1p(&self) -> ET1P_R {
        ET1P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline(always)]
    pub fn pfd_observ(&self) -> PFD_OBSERV_R {
        PFD_OBSERV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline(always)]
    pub fn por_observ(&self) -> POR_OBSERV_R {
        POR_OBSERV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline(always)]
    pub fn oscb(&self) -> OSCB_R {
        OSCB_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline(always)]
    pub fn srtcr_en(&mut self) -> SRTCR_EN_W {
        SRTCR_EN_W { w: self }
    }
    #[doc = "Bit 2 - MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline(always)]
    pub fn mcr_en(&mut self) -> MCR_EN_W {
        MCR_EN_W { w: self }
    }
    #[doc = "Bit 9 - External Tampering 1 Enable When set, external tampering 1 detection generates an LP security violation"]
    #[inline(always)]
    pub fn et1_en(&mut self) -> ET1_EN_W {
        ET1_EN_W { w: self }
    }
    #[doc = "Bit 11 - External Tampering 1 Polarity This bit is used to determine the polarity of external tamper 1."]
    #[inline(always)]
    pub fn et1p(&mut self) -> ET1P_W {
        ET1P_W { w: self }
    }
    #[doc = "Bit 14 - System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline(always)]
    pub fn pfd_observ(&mut self) -> PFD_OBSERV_W {
        PFD_OBSERV_W { w: self }
    }
    #[doc = "Bit 15 - Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline(always)]
    pub fn por_observ(&mut self) -> POR_OBSERV_W {
        POR_OBSERV_W { w: self }
    }
    #[doc = "Bit 28 - Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline(always)]
    pub fn oscb(&mut self) -> OSCB_W {
        OSCB_W { w: self }
    }
}
