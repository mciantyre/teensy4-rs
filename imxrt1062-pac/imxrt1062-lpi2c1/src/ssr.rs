#[doc = "Reader of register SSR"]
pub type R = crate::R<u32, super::SSR>;
#[doc = "Writer for register SSR"]
pub type W = crate::W<u32, super::SSR>;
#[doc = "Register SSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDF_A {
    #[doc = "0: Transmit data not requested"]
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
    #[doc = "0: Receive data is not ready"]
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
#[doc = "Address Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVF_A {
    #[doc = "0: Address Status Register is not valid"]
    AVF_0 = 0,
    #[doc = "1: Address Status Register is valid"]
    AVF_1 = 1,
}
impl From<AVF_A> for bool {
    #[inline(always)]
    fn from(variant: AVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AVF`"]
pub type AVF_R = crate::R<bool, AVF_A>;
impl AVF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVF_A {
        match self.bits {
            false => AVF_A::AVF_0,
            true => AVF_A::AVF_1,
        }
    }
    #[doc = "Checks if the value of the field is `AVF_0`"]
    #[inline(always)]
    pub fn is_avf_0(&self) -> bool {
        *self == AVF_A::AVF_0
    }
    #[doc = "Checks if the value of the field is `AVF_1`"]
    #[inline(always)]
    pub fn is_avf_1(&self) -> bool {
        *self == AVF_A::AVF_1
    }
}
#[doc = "Transmit ACK Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAF_A {
    #[doc = "0: Transmit ACK/NACK is not required"]
    TAF_0 = 0,
    #[doc = "1: Transmit ACK/NACK is required"]
    TAF_1 = 1,
}
impl From<TAF_A> for bool {
    #[inline(always)]
    fn from(variant: TAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAF`"]
pub type TAF_R = crate::R<bool, TAF_A>;
impl TAF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAF_A {
        match self.bits {
            false => TAF_A::TAF_0,
            true => TAF_A::TAF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAF_0`"]
    #[inline(always)]
    pub fn is_taf_0(&self) -> bool {
        *self == TAF_A::TAF_0
    }
    #[doc = "Checks if the value of the field is `TAF_1`"]
    #[inline(always)]
    pub fn is_taf_1(&self) -> bool {
        *self == TAF_A::TAF_1
    }
}
#[doc = "Repeated Start Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    #[doc = "0: Slave has not detected a Repeated START condition"]
    RSF_0 = 0,
    #[doc = "1: Slave has detected a Repeated START condition"]
    RSF_1 = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSF`"]
pub type RSF_R = crate::R<bool, RSF_A>;
impl RSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::RSF_0,
            true => RSF_A::RSF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSF_0`"]
    #[inline(always)]
    pub fn is_rsf_0(&self) -> bool {
        *self == RSF_A::RSF_0
    }
    #[doc = "Checks if the value of the field is `RSF_1`"]
    #[inline(always)]
    pub fn is_rsf_1(&self) -> bool {
        *self == RSF_A::RSF_1
    }
}
#[doc = "Write proxy for field `RSF`"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave has not detected a Repeated START condition"]
    #[inline(always)]
    pub fn rsf_0(self) -> &'a mut W {
        self.variant(RSF_A::RSF_0)
    }
    #[doc = "Slave has detected a Repeated START condition"]
    #[inline(always)]
    pub fn rsf_1(self) -> &'a mut W {
        self.variant(RSF_A::RSF_1)
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
    #[doc = "0: Slave has not detected a STOP condition"]
    SDF_0 = 0,
    #[doc = "1: Slave has detected a STOP condition"]
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
    #[doc = "Slave has not detected a STOP condition"]
    #[inline(always)]
    pub fn sdf_0(self) -> &'a mut W {
        self.variant(SDF_A::SDF_0)
    }
    #[doc = "Slave has detected a STOP condition"]
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
#[doc = "Bit Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEF_A {
    #[doc = "0: Slave has not detected a bit error"]
    BEF_0 = 0,
    #[doc = "1: Slave has detected a bit error"]
    BEF_1 = 1,
}
impl From<BEF_A> for bool {
    #[inline(always)]
    fn from(variant: BEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BEF`"]
pub type BEF_R = crate::R<bool, BEF_A>;
impl BEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEF_A {
        match self.bits {
            false => BEF_A::BEF_0,
            true => BEF_A::BEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEF_0`"]
    #[inline(always)]
    pub fn is_bef_0(&self) -> bool {
        *self == BEF_A::BEF_0
    }
    #[doc = "Checks if the value of the field is `BEF_1`"]
    #[inline(always)]
    pub fn is_bef_1(&self) -> bool {
        *self == BEF_A::BEF_1
    }
}
#[doc = "Write proxy for field `BEF`"]
pub struct BEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave has not detected a bit error"]
    #[inline(always)]
    pub fn bef_0(self) -> &'a mut W {
        self.variant(BEF_A::BEF_0)
    }
    #[doc = "Slave has detected a bit error"]
    #[inline(always)]
    pub fn bef_1(self) -> &'a mut W {
        self.variant(BEF_A::BEF_1)
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
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: FIFO underflow or overflow was not detected"]
    FEF_0 = 0,
    #[doc = "1: FIFO underflow or overflow was detected"]
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
    #[doc = "FIFO underflow or overflow was not detected"]
    #[inline(always)]
    pub fn fef_0(self) -> &'a mut W {
        self.variant(FEF_A::FEF_0)
    }
    #[doc = "FIFO underflow or overflow was detected"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Address Match 0 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM0F_A {
    #[doc = "0: Have not received an ADDR0 matching address"]
    AM0F_0 = 0,
    #[doc = "1: Have received an ADDR0 matching address"]
    AM0F_1 = 1,
}
impl From<AM0F_A> for bool {
    #[inline(always)]
    fn from(variant: AM0F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AM0F`"]
pub type AM0F_R = crate::R<bool, AM0F_A>;
impl AM0F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM0F_A {
        match self.bits {
            false => AM0F_A::AM0F_0,
            true => AM0F_A::AM0F_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM0F_0`"]
    #[inline(always)]
    pub fn is_am0f_0(&self) -> bool {
        *self == AM0F_A::AM0F_0
    }
    #[doc = "Checks if the value of the field is `AM0F_1`"]
    #[inline(always)]
    pub fn is_am0f_1(&self) -> bool {
        *self == AM0F_A::AM0F_1
    }
}
#[doc = "Address Match 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM1F_A {
    #[doc = "0: Have not received an ADDR1 or ADDR0/ADDR1 range matching address"]
    AM1F_0 = 0,
    #[doc = "1: Have received an ADDR1 or ADDR0/ADDR1 range matching address"]
    AM1F_1 = 1,
}
impl From<AM1F_A> for bool {
    #[inline(always)]
    fn from(variant: AM1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AM1F`"]
pub type AM1F_R = crate::R<bool, AM1F_A>;
impl AM1F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM1F_A {
        match self.bits {
            false => AM1F_A::AM1F_0,
            true => AM1F_A::AM1F_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM1F_0`"]
    #[inline(always)]
    pub fn is_am1f_0(&self) -> bool {
        *self == AM1F_A::AM1F_0
    }
    #[doc = "Checks if the value of the field is `AM1F_1`"]
    #[inline(always)]
    pub fn is_am1f_1(&self) -> bool {
        *self == AM1F_A::AM1F_1
    }
}
#[doc = "General Call Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCF_A {
    #[doc = "0: Slave has not detected the General Call Address or the General Call Address is disabled"]
    GCF_0 = 0,
    #[doc = "1: Slave has detected the General Call Address"]
    GCF_1 = 1,
}
impl From<GCF_A> for bool {
    #[inline(always)]
    fn from(variant: GCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GCF`"]
pub type GCF_R = crate::R<bool, GCF_A>;
impl GCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCF_A {
        match self.bits {
            false => GCF_A::GCF_0,
            true => GCF_A::GCF_1,
        }
    }
    #[doc = "Checks if the value of the field is `GCF_0`"]
    #[inline(always)]
    pub fn is_gcf_0(&self) -> bool {
        *self == GCF_A::GCF_0
    }
    #[doc = "Checks if the value of the field is `GCF_1`"]
    #[inline(always)]
    pub fn is_gcf_1(&self) -> bool {
        *self == GCF_A::GCF_1
    }
}
#[doc = "SMBus Alert Response Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SARF_A {
    #[doc = "0: SMBus Alert Response is disabled or not detected"]
    SARF_0 = 0,
    #[doc = "1: SMBus Alert Response is enabled and detected"]
    SARF_1 = 1,
}
impl From<SARF_A> for bool {
    #[inline(always)]
    fn from(variant: SARF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SARF`"]
pub type SARF_R = crate::R<bool, SARF_A>;
impl SARF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SARF_A {
        match self.bits {
            false => SARF_A::SARF_0,
            true => SARF_A::SARF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SARF_0`"]
    #[inline(always)]
    pub fn is_sarf_0(&self) -> bool {
        *self == SARF_A::SARF_0
    }
    #[doc = "Checks if the value of the field is `SARF_1`"]
    #[inline(always)]
    pub fn is_sarf_1(&self) -> bool {
        *self == SARF_A::SARF_1
    }
}
#[doc = "Slave Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBF_A {
    #[doc = "0: I2C Slave is idle"]
    SBF_0 = 0,
    #[doc = "1: I2C Slave is busy"]
    SBF_1 = 1,
}
impl From<SBF_A> for bool {
    #[inline(always)]
    fn from(variant: SBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBF`"]
pub type SBF_R = crate::R<bool, SBF_A>;
impl SBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBF_A {
        match self.bits {
            false => SBF_A::SBF_0,
            true => SBF_A::SBF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBF_0`"]
    #[inline(always)]
    pub fn is_sbf_0(&self) -> bool {
        *self == SBF_A::SBF_0
    }
    #[doc = "Checks if the value of the field is `SBF_1`"]
    #[inline(always)]
    pub fn is_sbf_1(&self) -> bool {
        *self == SBF_A::SBF_1
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
    #[doc = "Bit 2 - Address Valid Flag"]
    #[inline(always)]
    pub fn avf(&self) -> AVF_R {
        AVF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit ACK Flag"]
    #[inline(always)]
    pub fn taf(&self) -> TAF_R {
        TAF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Repeated Start Flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    pub fn sdf(&self) -> SDF_R {
        SDF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bit Error Flag"]
    #[inline(always)]
    pub fn bef(&self) -> BEF_R {
        BEF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Address Match 0 Flag"]
    #[inline(always)]
    pub fn am0f(&self) -> AM0F_R {
        AM0F_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Address Match 1 Flag"]
    #[inline(always)]
    pub fn am1f(&self) -> AM1F_R {
        AM1F_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - General Call Flag"]
    #[inline(always)]
    pub fn gcf(&self) -> GCF_R {
        GCF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SMBus Alert Response Flag"]
    #[inline(always)]
    pub fn sarf(&self) -> SARF_R {
        SARF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Slave Busy Flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus Busy Flag"]
    #[inline(always)]
    pub fn bbf(&self) -> BBF_R {
        BBF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Repeated Start Flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline(always)]
    pub fn sdf(&mut self) -> SDF_W {
        SDF_W { w: self }
    }
    #[doc = "Bit 10 - Bit Error Flag"]
    #[inline(always)]
    pub fn bef(&mut self) -> BEF_W {
        BEF_W { w: self }
    }
    #[doc = "Bit 11 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
}
