#[doc = "Reader of register MCR2"]
pub type R = crate::R<u32, super::MCR2>;
#[doc = "Writer for register MCR2"]
pub type W = crate::W<u32, super::MCR2>;
#[doc = "Register MCR2 `reset()`'s with value 0x2000_81f7"]
impl crate::ResetValue for super::MCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_81f7
    }
}
#[doc = "This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automaticaly when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRAHBBUFOPT_A {
    #[doc = "0: AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_0 = 0,
    #[doc = "1: AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_1 = 1,
}
impl From<CLRAHBBUFOPT_A> for bool {
    #[inline(always)]
    fn from(variant: CLRAHBBUFOPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLRAHBBUFOPT`"]
pub type CLRAHBBUFOPT_R = crate::R<bool, CLRAHBBUFOPT_A>;
impl CLRAHBBUFOPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRAHBBUFOPT_A {
        match self.bits {
            false => CLRAHBBUFOPT_A::CLRAHBBUFOPT_0,
            true => CLRAHBBUFOPT_A::CLRAHBBUFOPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRAHBBUFOPT_0`"]
    #[inline(always)]
    pub fn is_clrahbbufopt_0(&self) -> bool {
        *self == CLRAHBBUFOPT_A::CLRAHBBUFOPT_0
    }
    #[doc = "Checks if the value of the field is `CLRAHBBUFOPT_1`"]
    #[inline(always)]
    pub fn is_clrahbbufopt_1(&self) -> bool {
        *self == CLRAHBBUFOPT_A::CLRAHBBUFOPT_1
    }
}
#[doc = "Write proxy for field `CLRAHBBUFOPT`"]
pub struct CLRAHBBUFOPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRAHBBUFOPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRAHBBUFOPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline(always)]
    pub fn clrahbbufopt_0(self) -> &'a mut W {
        self.variant(CLRAHBBUFOPT_A::CLRAHBBUFOPT_0)
    }
    #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline(always)]
    pub fn clrahbbufopt_1(self) -> &'a mut W {
        self.variant(CLRAHBBUFOPT_A::CLRAHBBUFOPT_1)
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
#[doc = "Reader of field `CLRLEARNPHASE`"]
pub type CLRLEARNPHASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRLEARNPHASE`"]
pub struct CLRLEARNPHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRLEARNPHASE_W<'a> {
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
#[doc = "All external devices are same devices (both in types and size) for A1/A2/B1/B2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMEDEVICEEN_A {
    #[doc = "0: In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 seperately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    SAMEDEVICEEN_0 = 0,
    #[doc = "1: FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    SAMEDEVICEEN_1 = 1,
}
impl From<SAMEDEVICEEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAMEDEVICEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMEDEVICEEN`"]
pub type SAMEDEVICEEN_R = crate::R<bool, SAMEDEVICEEN_A>;
impl SAMEDEVICEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMEDEVICEEN_A {
        match self.bits {
            false => SAMEDEVICEEN_A::SAMEDEVICEEN_0,
            true => SAMEDEVICEEN_A::SAMEDEVICEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAMEDEVICEEN_0`"]
    #[inline(always)]
    pub fn is_samedeviceen_0(&self) -> bool {
        *self == SAMEDEVICEEN_A::SAMEDEVICEEN_0
    }
    #[doc = "Checks if the value of the field is `SAMEDEVICEEN_1`"]
    #[inline(always)]
    pub fn is_samedeviceen_1(&self) -> bool {
        *self == SAMEDEVICEEN_A::SAMEDEVICEEN_1
    }
}
#[doc = "Write proxy for field `SAMEDEVICEEN`"]
pub struct SAMEDEVICEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMEDEVICEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMEDEVICEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 seperately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    #[inline(always)]
    pub fn samedeviceen_0(self) -> &'a mut W {
        self.variant(SAMEDEVICEEN_A::SAMEDEVICEEN_0)
    }
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    #[inline(always)]
    pub fn samedeviceen_1(self) -> &'a mut W {
        self.variant(SAMEDEVICEEN_A::SAMEDEVICEEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "SCKB pad can be used as SCKA differential clock output (inverted clock to SCKA). In this case, port B flash access is not available. After change the value of this feild, MCR0\\[SWRESET\\]
should be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKBDIFFOPT_A {
    #[doc = "0: SCKB pad is used as port B SCK clock output. Port B flash access is available."]
    SCKBDIFFOPT_0 = 0,
    #[doc = "1: SCKB pad is used as port A SCK inverted clock output (Differential clock to SCKA). Port B flash access is not available."]
    SCKBDIFFOPT_1 = 1,
}
impl From<SCKBDIFFOPT_A> for bool {
    #[inline(always)]
    fn from(variant: SCKBDIFFOPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCKBDIFFOPT`"]
pub type SCKBDIFFOPT_R = crate::R<bool, SCKBDIFFOPT_A>;
impl SCKBDIFFOPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKBDIFFOPT_A {
        match self.bits {
            false => SCKBDIFFOPT_A::SCKBDIFFOPT_0,
            true => SCKBDIFFOPT_A::SCKBDIFFOPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCKBDIFFOPT_0`"]
    #[inline(always)]
    pub fn is_sckbdiffopt_0(&self) -> bool {
        *self == SCKBDIFFOPT_A::SCKBDIFFOPT_0
    }
    #[doc = "Checks if the value of the field is `SCKBDIFFOPT_1`"]
    #[inline(always)]
    pub fn is_sckbdiffopt_1(&self) -> bool {
        *self == SCKBDIFFOPT_A::SCKBDIFFOPT_1
    }
}
#[doc = "Write proxy for field `SCKBDIFFOPT`"]
pub struct SCKBDIFFOPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKBDIFFOPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCKBDIFFOPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCKB pad is used as port B SCK clock output. Port B flash access is available."]
    #[inline(always)]
    pub fn sckbdiffopt_0(self) -> &'a mut W {
        self.variant(SCKBDIFFOPT_A::SCKBDIFFOPT_0)
    }
    #[doc = "SCKB pad is used as port A SCK inverted clock output (Differential clock to SCKA). Port B flash access is not available."]
    #[inline(always)]
    pub fn sckbdiffopt_1(self) -> &'a mut W {
        self.variant(SCKBDIFFOPT_A::SCKBDIFFOPT_1)
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
#[doc = "Reader of field `RESUMEWAIT`"]
pub type RESUMEWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESUMEWAIT`"]
pub struct RESUMEWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automaticaly when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    pub fn clrahbbufopt(&self) -> CLRAHBBUFOPT_R {
        CLRAHBBUFOPT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub fn clrlearnphase(&self) -> CLRLEARNPHASE_R {
        CLRLEARNPHASE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub fn samedeviceen(&self) -> SAMEDEVICEEN_R {
        SAMEDEVICEEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SCKB pad can be used as SCKA differential clock output (inverted clock to SCKA). In this case, port B flash access is not available. After change the value of this feild, MCR0\\[SWRESET\\]
should be set."]
    #[inline(always)]
    pub fn sckbdiffopt(&self) -> SCKBDIFFOPT_R {
        SCKBDIFFOPT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub fn resumewait(&self) -> RESUMEWAIT_R {
        RESUMEWAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automaticaly when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    pub fn clrahbbufopt(&mut self) -> CLRAHBBUFOPT_W {
        CLRAHBBUFOPT_W { w: self }
    }
    #[doc = "Bit 14 - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub fn clrlearnphase(&mut self) -> CLRLEARNPHASE_W {
        CLRLEARNPHASE_W { w: self }
    }
    #[doc = "Bit 15 - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub fn samedeviceen(&mut self) -> SAMEDEVICEEN_W {
        SAMEDEVICEEN_W { w: self }
    }
    #[doc = "Bit 19 - SCKB pad can be used as SCKA differential clock output (inverted clock to SCKA). In this case, port B flash access is not available. After change the value of this feild, MCR0\\[SWRESET\\]
should be set."]
    #[inline(always)]
    pub fn sckbdiffopt(&mut self) -> SCKBDIFFOPT_W {
        SCKBDIFFOPT_W { w: self }
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub fn resumewait(&mut self) -> RESUMEWAIT_W {
        RESUMEWAIT_W { w: self }
    }
}
