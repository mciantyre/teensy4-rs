#[doc = "Reader of register MSR"]
pub type R = crate::R<u32, super::MSR>;
#[doc = "Writer for register MSR"]
pub type W = crate::W<u32, super::MSR>;
#[doc = "Register MSR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Transmit Data Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDF_A {
    #[doc = "0: Transmit data is not requested"]
    TDF_0 = 0,
    #[doc = "1: Transmit data is requested"]
    TDF_1 = 1,
}
impl From<TDF_A> for bool {
    #[inline(always)]
    fn from(variant: TDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDF`"]
pub type TDF_R = crate::R<bool, TDF_A>;
impl TDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDF_A {
        match self.bits {
            false => TDF_A::TDF_0,
            true => TDF_A::TDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDF_0`"]
    #[inline(always)]
    pub fn is_tdf_0(&self) -> bool {
        *self == TDF_A::TDF_0
    }
    #[doc = "Checks if the value of the field is `TDF_1`"]
    #[inline(always)]
    pub fn is_tdf_1(&self) -> bool {
        *self == TDF_A::TDF_1
    }
}
#[doc = "Receive Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDF_A {
    #[doc = "0: Receive Data is not ready"]
    RDF_0 = 0,
    #[doc = "1: Receive data is ready"]
    RDF_1 = 1,
}
impl From<RDF_A> for bool {
    #[inline(always)]
    fn from(variant: RDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDF`"]
pub type RDF_R = crate::R<bool, RDF_A>;
impl RDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDF_A {
        match self.bits {
            false => RDF_A::RDF_0,
            true => RDF_A::RDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDF_0`"]
    #[inline(always)]
    pub fn is_rdf_0(&self) -> bool {
        *self == RDF_A::RDF_0
    }
    #[doc = "Checks if the value of the field is `RDF_1`"]
    #[inline(always)]
    pub fn is_rdf_1(&self) -> bool {
        *self == RDF_A::RDF_1
    }
}
#[doc = "End Packet Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPF_A {
    #[doc = "0: Master has not generated a STOP or Repeated START condition"]
    EPF_0 = 0,
    #[doc = "1: Master has generated a STOP or Repeated START condition"]
    EPF_1 = 1,
}
impl From<EPF_A> for bool {
    #[inline(always)]
    fn from(variant: EPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPF`"]
pub type EPF_R = crate::R<bool, EPF_A>;
impl EPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPF_A {
        match self.bits {
            false => EPF_A::EPF_0,
            true => EPF_A::EPF_1,
        }
    }
    #[doc = "Checks if the value of the field is `EPF_0`"]
    #[inline(always)]
    pub fn is_epf_0(&self) -> bool {
        *self == EPF_A::EPF_0
    }
    #[doc = "Checks if the value of the field is `EPF_1`"]
    #[inline(always)]
    pub fn is_epf_1(&self) -> bool {
        *self == EPF_A::EPF_1
    }
}
#[doc = "Write proxy for field `EPF`"]
pub struct EPF_W<'a> {
    w: &'a mut W,
}
impl<'a> EPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master has not generated a STOP or Repeated START condition"]
    #[inline(always)]
    pub fn epf_0(self) -> &'a mut W {
        self.variant(EPF_A::EPF_0)
    }
    #[doc = "Master has generated a STOP or Repeated START condition"]
    #[inline(always)]
    pub fn epf_1(self) -> &'a mut W {
        self.variant(EPF_A::EPF_1)
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
#[doc = "STOP Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDF_A {
    #[doc = "0: Master has not generated a STOP condition"]
    SDF_0 = 0,
    #[doc = "1: Master has generated a STOP condition"]
    SDF_1 = 1,
}
impl From<SDF_A> for bool {
    #[inline(always)]
    fn from(variant: SDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDF`"]
pub type SDF_R = crate::R<bool, SDF_A>;
impl SDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDF_A {
        match self.bits {
            false => SDF_A::SDF_0,
            true => SDF_A::SDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDF_0`"]
    #[inline(always)]
    pub fn is_sdf_0(&self) -> bool {
        *self == SDF_A::SDF_0
    }
    #[doc = "Checks if the value of the field is `SDF_1`"]
    #[inline(always)]
    pub fn is_sdf_1(&self) -> bool {
        *self == SDF_A::SDF_1
    }
}
#[doc = "Write proxy for field `SDF`"]
pub struct SDF_W<'a> {
    w: &'a mut W,
}
impl<'a> SDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master has not generated a STOP condition"]
    #[inline(always)]
    pub fn sdf_0(self) -> &'a mut W {
        self.variant(SDF_A::SDF_0)
    }
    #[doc = "Master has generated a STOP condition"]
    #[inline(always)]
    pub fn sdf_1(self) -> &'a mut W {
        self.variant(SDF_A::SDF_1)
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
#[doc = "NACK Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDF_A {
    #[doc = "0: Unexpected NACK was not detected"]
    NDF_0 = 0,
    #[doc = "1: Unexpected NACK was detected"]
    NDF_1 = 1,
}
impl From<NDF_A> for bool {
    #[inline(always)]
    fn from(variant: NDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NDF`"]
pub type NDF_R = crate::R<bool, NDF_A>;
impl NDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDF_A {
        match self.bits {
            false => NDF_A::NDF_0,
            true => NDF_A::NDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDF_0`"]
    #[inline(always)]
    pub fn is_ndf_0(&self) -> bool {
        *self == NDF_A::NDF_0
    }
    #[doc = "Checks if the value of the field is `NDF_1`"]
    #[inline(always)]
    pub fn is_ndf_1(&self) -> bool {
        *self == NDF_A::NDF_1
    }
}
#[doc = "Write proxy for field `NDF`"]
pub struct NDF_W<'a> {
    w: &'a mut W,
}
impl<'a> NDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Unexpected NACK was not detected"]
    #[inline(always)]
    pub fn ndf_0(self) -> &'a mut W {
        self.variant(NDF_A::NDF_0)
    }
    #[doc = "Unexpected NACK was detected"]
    #[inline(always)]
    pub fn ndf_1(self) -> &'a mut W {
        self.variant(NDF_A::NDF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Arbitration Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALF_A {
    #[doc = "0: Master has not lost arbitration"]
    ALF_0 = 0,
    #[doc = "1: Master has lost arbitration"]
    ALF_1 = 1,
}
impl From<ALF_A> for bool {
    #[inline(always)]
    fn from(variant: ALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALF`"]
pub type ALF_R = crate::R<bool, ALF_A>;
impl ALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALF_A {
        match self.bits {
            false => ALF_A::ALF_0,
            true => ALF_A::ALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ALF_0`"]
    #[inline(always)]
    pub fn is_alf_0(&self) -> bool {
        *self == ALF_A::ALF_0
    }
    #[doc = "Checks if the value of the field is `ALF_1`"]
    #[inline(always)]
    pub fn is_alf_1(&self) -> bool {
        *self == ALF_A::ALF_1
    }
}
#[doc = "Write proxy for field `ALF`"]
pub struct ALF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master has not lost arbitration"]
    #[inline(always)]
    pub fn alf_0(self) -> &'a mut W {
        self.variant(ALF_A::ALF_0)
    }
    #[doc = "Master has lost arbitration"]
    #[inline(always)]
    pub fn alf_1(self) -> &'a mut W {
        self.variant(ALF_A::ALF_1)
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
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: No error"]
    FEF_0 = 0,
    #[doc = "1: Master sending or receiving data without a START condition"]
    FEF_1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEF`"]
pub type FEF_R = crate::R<bool, FEF_A>;
impl FEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::FEF_0,
            true => FEF_A::FEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEF_0`"]
    #[inline(always)]
    pub fn is_fef_0(&self) -> bool {
        *self == FEF_A::FEF_0
    }
    #[doc = "Checks if the value of the field is `FEF_1`"]
    #[inline(always)]
    pub fn is_fef_1(&self) -> bool {
        *self == FEF_A::FEF_1
    }
}
#[doc = "Write proxy for field `FEF`"]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn fef_0(self) -> &'a mut W {
        self.variant(FEF_A::FEF_0)
    }
    #[doc = "Master sending or receiving data without a START condition"]
    #[inline(always)]
    pub fn fef_1(self) -> &'a mut W {
        self.variant(FEF_A::FEF_1)
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
#[doc = "Pin Low Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLTF_A {
    #[doc = "0: Pin low timeout has not occurred or is disabled"]
    PLTF_0 = 0,
    #[doc = "1: Pin low timeout has occurred"]
    PLTF_1 = 1,
}
impl From<PLTF_A> for bool {
    #[inline(always)]
    fn from(variant: PLTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLTF`"]
pub type PLTF_R = crate::R<bool, PLTF_A>;
impl PLTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLTF_A {
        match self.bits {
            false => PLTF_A::PLTF_0,
            true => PLTF_A::PLTF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLTF_0`"]
    #[inline(always)]
    pub fn is_pltf_0(&self) -> bool {
        *self == PLTF_A::PLTF_0
    }
    #[doc = "Checks if the value of the field is `PLTF_1`"]
    #[inline(always)]
    pub fn is_pltf_1(&self) -> bool {
        *self == PLTF_A::PLTF_1
    }
}
#[doc = "Write proxy for field `PLTF`"]
pub struct PLTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin low timeout has not occurred or is disabled"]
    #[inline(always)]
    pub fn pltf_0(self) -> &'a mut W {
        self.variant(PLTF_A::PLTF_0)
    }
    #[doc = "Pin low timeout has occurred"]
    #[inline(always)]
    pub fn pltf_1(self) -> &'a mut W {
        self.variant(PLTF_A::PLTF_1)
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
#[doc = "Data Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMF_A {
    #[doc = "0: Have not received matching data"]
    DMF_0 = 0,
    #[doc = "1: Have received matching data"]
    DMF_1 = 1,
}
impl From<DMF_A> for bool {
    #[inline(always)]
    fn from(variant: DMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMF`"]
pub type DMF_R = crate::R<bool, DMF_A>;
impl DMF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMF_A {
        match self.bits {
            false => DMF_A::DMF_0,
            true => DMF_A::DMF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMF_0`"]
    #[inline(always)]
    pub fn is_dmf_0(&self) -> bool {
        *self == DMF_A::DMF_0
    }
    #[doc = "Checks if the value of the field is `DMF_1`"]
    #[inline(always)]
    pub fn is_dmf_1(&self) -> bool {
        *self == DMF_A::DMF_1
    }
}
#[doc = "Write proxy for field `DMF`"]
pub struct DMF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Have not received matching data"]
    #[inline(always)]
    pub fn dmf_0(self) -> &'a mut W {
        self.variant(DMF_A::DMF_0)
    }
    #[doc = "Have received matching data"]
    #[inline(always)]
    pub fn dmf_1(self) -> &'a mut W {
        self.variant(DMF_A::DMF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Master Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBF_A {
    #[doc = "0: I2C Master is idle"]
    MBF_0 = 0,
    #[doc = "1: I2C Master is busy"]
    MBF_1 = 1,
}
impl From<MBF_A> for bool {
    #[inline(always)]
    fn from(variant: MBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MBF`"]
pub type MBF_R = crate::R<bool, MBF_A>;
impl MBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBF_A {
        match self.bits {
            false => MBF_A::MBF_0,
            true => MBF_A::MBF_1,
        }
    }
    #[doc = "Checks if the value of the field is `MBF_0`"]
    #[inline(always)]
    pub fn is_mbf_0(&self) -> bool {
        *self == MBF_A::MBF_0
    }
    #[doc = "Checks if the value of the field is `MBF_1`"]
    #[inline(always)]
    pub fn is_mbf_1(&self) -> bool {
        *self == MBF_A::MBF_1
    }
}
#[doc = "Bus Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBF_A {
    #[doc = "0: I2C Bus is idle"]
    BBF_0 = 0,
    #[doc = "1: I2C Bus is busy"]
    BBF_1 = 1,
}
impl From<BBF_A> for bool {
    #[inline(always)]
    fn from(variant: BBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BBF`"]
pub type BBF_R = crate::R<bool, BBF_A>;
impl BBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BBF_A {
        match self.bits {
            false => BBF_A::BBF_0,
            true => BBF_A::BBF_1,
        }
    }
    #[doc = "Checks if the value of the field is `BBF_0`"]
    #[inline(always)]
    pub fn is_bbf_0(&self) -> bool {
        *self == BBF_A::BBF_0
    }
    #[doc = "Checks if the value of the field is `BBF_1`"]
    #[inline(always)]
    pub fn is_bbf_1(&self) -> bool {
        *self == BBF_A::BBF_1
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data Flag"]
    #[inline(always)]
    pub fn tdf(&self) -> TDF_R {
        TDF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Flag"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline(always)]
    pub fn epf(&self) -> EPF_R {
        EPF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    pub fn sdf(&self) -> SDF_R {
        SDF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline(always)]
    pub fn ndf(&self) -> NDF_R {
        NDF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(&self) -> ALF_R {
        ALF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline(always)]
    pub fn pltf(&self) -> PLTF_R {
        PLTF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline(always)]
    pub fn dmf(&self) -> DMF_R {
        DMF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Master Busy Flag"]
    #[inline(always)]
    pub fn mbf(&self) -> MBF_R {
        MBF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus Busy Flag"]
    #[inline(always)]
    pub fn bbf(&self) -> BBF_R {
        BBF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline(always)]
    pub fn epf(&mut self) -> EPF_W {
        EPF_W { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    pub fn sdf(&mut self) -> SDF_W {
        SDF_W { w: self }
    }
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline(always)]
    pub fn ndf(&mut self) -> NDF_W {
        NDF_W { w: self }
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(&mut self) -> ALF_W {
        ALF_W { w: self }
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline(always)]
    pub fn pltf(&mut self) -> PLTF_W {
        PLTF_W { w: self }
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline(always)]
    pub fn dmf(&mut self) -> DMF_W {
        DMF_W { w: self }
    }
}
