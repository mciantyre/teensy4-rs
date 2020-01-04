#[doc = "Reader of register PROT_CTRL"]
pub type R = crate::R<u32, super::PROT_CTRL>;
#[doc = "Writer for register PROT_CTRL"]
pub type W = crate::W<u32, super::PROT_CTRL>;
#[doc = "Register PROT_CTRL `reset()`'s with value 0x0880_0020"]
impl crate::ResetValue for super::PROT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0880_0020
    }
}
#[doc = "LED Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCTL_A {
    #[doc = "0: LED off"]
    LCTL_0 = 0,
    #[doc = "1: LED on"]
    LCTL_1 = 1,
}
impl From<LCTL_A> for bool {
    #[inline(always)]
    fn from(variant: LCTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCTL`"]
pub type LCTL_R = crate::R<bool, LCTL_A>;
impl LCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCTL_A {
        match self.bits {
            false => LCTL_A::LCTL_0,
            true => LCTL_A::LCTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCTL_0`"]
    #[inline(always)]
    pub fn is_lctl_0(&self) -> bool {
        *self == LCTL_A::LCTL_0
    }
    #[doc = "Checks if the value of the field is `LCTL_1`"]
    #[inline(always)]
    pub fn is_lctl_1(&self) -> bool {
        *self == LCTL_A::LCTL_1
    }
}
#[doc = "Write proxy for field `LCTL`"]
pub struct LCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LED off"]
    #[inline(always)]
    pub fn lctl_0(self) -> &'a mut W {
        self.variant(LCTL_A::LCTL_0)
    }
    #[doc = "LED on"]
    #[inline(always)]
    pub fn lctl_1(self) -> &'a mut W {
        self.variant(LCTL_A::LCTL_1)
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
#[doc = "Data Transfer Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTW_A {
    #[doc = "0: 1-bit mode"]
    DTW_0 = 0,
    #[doc = "1: 4-bit mode"]
    DTW_1 = 1,
    #[doc = "2: 8-bit mode"]
    DTW_2 = 2,
}
impl From<DTW_A> for u8 {
    #[inline(always)]
    fn from(variant: DTW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTW`"]
pub type DTW_R = crate::R<u8, DTW_A>;
impl DTW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTW_A::DTW_0),
            1 => Val(DTW_A::DTW_1),
            2 => Val(DTW_A::DTW_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DTW_0`"]
    #[inline(always)]
    pub fn is_dtw_0(&self) -> bool {
        *self == DTW_A::DTW_0
    }
    #[doc = "Checks if the value of the field is `DTW_1`"]
    #[inline(always)]
    pub fn is_dtw_1(&self) -> bool {
        *self == DTW_A::DTW_1
    }
    #[doc = "Checks if the value of the field is `DTW_2`"]
    #[inline(always)]
    pub fn is_dtw_2(&self) -> bool {
        *self == DTW_A::DTW_2
    }
}
#[doc = "Write proxy for field `DTW`"]
pub struct DTW_W<'a> {
    w: &'a mut W,
}
impl<'a> DTW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn dtw_0(self) -> &'a mut W {
        self.variant(DTW_A::DTW_0)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn dtw_1(self) -> &'a mut W {
        self.variant(DTW_A::DTW_1)
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn dtw_2(self) -> &'a mut W {
        self.variant(DTW_A::DTW_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "DATA3 as Card Detection Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D3CD_A {
    #[doc = "0: DATA3 does not monitor Card Insertion"]
    D3CD_0 = 0,
    #[doc = "1: DATA3 as Card Detection Pin"]
    D3CD_1 = 1,
}
impl From<D3CD_A> for bool {
    #[inline(always)]
    fn from(variant: D3CD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `D3CD`"]
pub type D3CD_R = crate::R<bool, D3CD_A>;
impl D3CD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D3CD_A {
        match self.bits {
            false => D3CD_A::D3CD_0,
            true => D3CD_A::D3CD_1,
        }
    }
    #[doc = "Checks if the value of the field is `D3CD_0`"]
    #[inline(always)]
    pub fn is_d3cd_0(&self) -> bool {
        *self == D3CD_A::D3CD_0
    }
    #[doc = "Checks if the value of the field is `D3CD_1`"]
    #[inline(always)]
    pub fn is_d3cd_1(&self) -> bool {
        *self == D3CD_A::D3CD_1
    }
}
#[doc = "Write proxy for field `D3CD`"]
pub struct D3CD_W<'a> {
    w: &'a mut W,
}
impl<'a> D3CD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D3CD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DATA3 does not monitor Card Insertion"]
    #[inline(always)]
    pub fn d3cd_0(self) -> &'a mut W {
        self.variant(D3CD_A::D3CD_0)
    }
    #[doc = "DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd_1(self) -> &'a mut W {
        self.variant(D3CD_A::D3CD_1)
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
#[doc = "Endian Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMODE_A {
    #[doc = "0: Big Endian Mode"]
    EMODE_0 = 0,
    #[doc = "1: Half Word Big Endian Mode"]
    EMODE_1 = 1,
    #[doc = "2: Little Endian Mode"]
    EMODE_2 = 2,
}
impl From<EMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EMODE`"]
pub type EMODE_R = crate::R<u8, EMODE_A>;
impl EMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EMODE_A::EMODE_0),
            1 => Val(EMODE_A::EMODE_1),
            2 => Val(EMODE_A::EMODE_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMODE_0`"]
    #[inline(always)]
    pub fn is_emode_0(&self) -> bool {
        *self == EMODE_A::EMODE_0
    }
    #[doc = "Checks if the value of the field is `EMODE_1`"]
    #[inline(always)]
    pub fn is_emode_1(&self) -> bool {
        *self == EMODE_A::EMODE_1
    }
    #[doc = "Checks if the value of the field is `EMODE_2`"]
    #[inline(always)]
    pub fn is_emode_2(&self) -> bool {
        *self == EMODE_A::EMODE_2
    }
}
#[doc = "Write proxy for field `EMODE`"]
pub struct EMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Big Endian Mode"]
    #[inline(always)]
    pub fn emode_0(self) -> &'a mut W {
        self.variant(EMODE_A::EMODE_0)
    }
    #[doc = "Half Word Big Endian Mode"]
    #[inline(always)]
    pub fn emode_1(self) -> &'a mut W {
        self.variant(EMODE_A::EMODE_1)
    }
    #[doc = "Little Endian Mode"]
    #[inline(always)]
    pub fn emode_2(self) -> &'a mut W {
        self.variant(EMODE_A::EMODE_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Card Detect Test Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDTL_A {
    #[doc = "0: Card Detect Test Level is 0, no card inserted"]
    CDTL_0 = 0,
    #[doc = "1: Card Detect Test Level is 1, card inserted"]
    CDTL_1 = 1,
}
impl From<CDTL_A> for bool {
    #[inline(always)]
    fn from(variant: CDTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CDTL`"]
pub type CDTL_R = crate::R<bool, CDTL_A>;
impl CDTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDTL_A {
        match self.bits {
            false => CDTL_A::CDTL_0,
            true => CDTL_A::CDTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDTL_0`"]
    #[inline(always)]
    pub fn is_cdtl_0(&self) -> bool {
        *self == CDTL_A::CDTL_0
    }
    #[doc = "Checks if the value of the field is `CDTL_1`"]
    #[inline(always)]
    pub fn is_cdtl_1(&self) -> bool {
        *self == CDTL_A::CDTL_1
    }
}
#[doc = "Write proxy for field `CDTL`"]
pub struct CDTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card Detect Test Level is 0, no card inserted"]
    #[inline(always)]
    pub fn cdtl_0(self) -> &'a mut W {
        self.variant(CDTL_A::CDTL_0)
    }
    #[doc = "Card Detect Test Level is 1, card inserted"]
    #[inline(always)]
    pub fn cdtl_1(self) -> &'a mut W {
        self.variant(CDTL_A::CDTL_1)
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
#[doc = "Card Detect Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDSS_A {
    #[doc = "0: Card Detection Level is selected (for normal purpose)."]
    CDSS_0 = 0,
    #[doc = "1: Card Detection Test Level is selected (for test purpose)."]
    CDSS_1 = 1,
}
impl From<CDSS_A> for bool {
    #[inline(always)]
    fn from(variant: CDSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CDSS`"]
pub type CDSS_R = crate::R<bool, CDSS_A>;
impl CDSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSS_A {
        match self.bits {
            false => CDSS_A::CDSS_0,
            true => CDSS_A::CDSS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDSS_0`"]
    #[inline(always)]
    pub fn is_cdss_0(&self) -> bool {
        *self == CDSS_A::CDSS_0
    }
    #[doc = "Checks if the value of the field is `CDSS_1`"]
    #[inline(always)]
    pub fn is_cdss_1(&self) -> bool {
        *self == CDSS_A::CDSS_1
    }
}
#[doc = "Write proxy for field `CDSS`"]
pub struct CDSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDSS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card Detection Level is selected (for normal purpose)."]
    #[inline(always)]
    pub fn cdss_0(self) -> &'a mut W {
        self.variant(CDSS_A::CDSS_0)
    }
    #[doc = "Card Detection Test Level is selected (for test purpose)."]
    #[inline(always)]
    pub fn cdss_1(self) -> &'a mut W {
        self.variant(CDSS_A::CDSS_1)
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
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMASEL_A {
    #[doc = "0: No DMA or Simple DMA is selected"]
    DMASEL_0 = 0,
    #[doc = "1: ADMA1 is selected"]
    DMASEL_1 = 1,
    #[doc = "2: ADMA2 is selected"]
    DMASEL_2 = 2,
}
impl From<DMASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMASEL`"]
pub type DMASEL_R = crate::R<u8, DMASEL_A>;
impl DMASEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMASEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMASEL_A::DMASEL_0),
            1 => Val(DMASEL_A::DMASEL_1),
            2 => Val(DMASEL_A::DMASEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMASEL_0`"]
    #[inline(always)]
    pub fn is_dmasel_0(&self) -> bool {
        *self == DMASEL_A::DMASEL_0
    }
    #[doc = "Checks if the value of the field is `DMASEL_1`"]
    #[inline(always)]
    pub fn is_dmasel_1(&self) -> bool {
        *self == DMASEL_A::DMASEL_1
    }
    #[doc = "Checks if the value of the field is `DMASEL_2`"]
    #[inline(always)]
    pub fn is_dmasel_2(&self) -> bool {
        *self == DMASEL_A::DMASEL_2
    }
}
#[doc = "Write proxy for field `DMASEL`"]
pub struct DMASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No DMA or Simple DMA is selected"]
    #[inline(always)]
    pub fn dmasel_0(self) -> &'a mut W {
        self.variant(DMASEL_A::DMASEL_0)
    }
    #[doc = "ADMA1 is selected"]
    #[inline(always)]
    pub fn dmasel_1(self) -> &'a mut W {
        self.variant(DMASEL_A::DMASEL_1)
    }
    #[doc = "ADMA2 is selected"]
    #[inline(always)]
    pub fn dmasel_2(self) -> &'a mut W {
        self.variant(DMASEL_A::DMASEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Stop At Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SABGREQ_A {
    #[doc = "0: Transfer"]
    SABGREQ_0 = 0,
    #[doc = "1: Stop"]
    SABGREQ_1 = 1,
}
impl From<SABGREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SABGREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SABGREQ`"]
pub type SABGREQ_R = crate::R<bool, SABGREQ_A>;
impl SABGREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SABGREQ_A {
        match self.bits {
            false => SABGREQ_A::SABGREQ_0,
            true => SABGREQ_A::SABGREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SABGREQ_0`"]
    #[inline(always)]
    pub fn is_sabgreq_0(&self) -> bool {
        *self == SABGREQ_A::SABGREQ_0
    }
    #[doc = "Checks if the value of the field is `SABGREQ_1`"]
    #[inline(always)]
    pub fn is_sabgreq_1(&self) -> bool {
        *self == SABGREQ_A::SABGREQ_1
    }
}
#[doc = "Write proxy for field `SABGREQ`"]
pub struct SABGREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SABGREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SABGREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn sabgreq_0(self) -> &'a mut W {
        self.variant(SABGREQ_A::SABGREQ_0)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn sabgreq_1(self) -> &'a mut W {
        self.variant(SABGREQ_A::SABGREQ_1)
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
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREQ_A {
    #[doc = "0: No effect"]
    CREQ_0 = 0,
    #[doc = "1: Restart"]
    CREQ_1 = 1,
}
impl From<CREQ_A> for bool {
    #[inline(always)]
    fn from(variant: CREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CREQ`"]
pub type CREQ_R = crate::R<bool, CREQ_A>;
impl CREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CREQ_A {
        match self.bits {
            false => CREQ_A::CREQ_0,
            true => CREQ_A::CREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CREQ_0`"]
    #[inline(always)]
    pub fn is_creq_0(&self) -> bool {
        *self == CREQ_A::CREQ_0
    }
    #[doc = "Checks if the value of the field is `CREQ_1`"]
    #[inline(always)]
    pub fn is_creq_1(&self) -> bool {
        *self == CREQ_A::CREQ_1
    }
}
#[doc = "Write proxy for field `CREQ`"]
pub struct CREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn creq_0(self) -> &'a mut W {
        self.variant(CREQ_A::CREQ_0)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn creq_1(self) -> &'a mut W {
        self.variant(CREQ_A::CREQ_1)
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
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWCTL_A {
    #[doc = "0: Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    RWCTL_0 = 0,
    #[doc = "1: Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    RWCTL_1 = 1,
}
impl From<RWCTL_A> for bool {
    #[inline(always)]
    fn from(variant: RWCTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RWCTL`"]
pub type RWCTL_R = crate::R<bool, RWCTL_A>;
impl RWCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWCTL_A {
        match self.bits {
            false => RWCTL_A::RWCTL_0,
            true => RWCTL_A::RWCTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWCTL_0`"]
    #[inline(always)]
    pub fn is_rwctl_0(&self) -> bool {
        *self == RWCTL_A::RWCTL_0
    }
    #[doc = "Checks if the value of the field is `RWCTL_1`"]
    #[inline(always)]
    pub fn is_rwctl_1(&self) -> bool {
        *self == RWCTL_A::RWCTL_1
    }
}
#[doc = "Write proxy for field `RWCTL`"]
pub struct RWCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RWCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWCTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    #[inline(always)]
    pub fn rwctl_0(self) -> &'a mut W {
        self.variant(RWCTL_A::RWCTL_0)
    }
    #[doc = "Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    #[inline(always)]
    pub fn rwctl_1(self) -> &'a mut W {
        self.variant(RWCTL_A::RWCTL_1)
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
#[doc = "Interrupt At Block Gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IABG_A {
    #[doc = "0: Disabled"]
    IABG_0 = 0,
    #[doc = "1: Enabled"]
    IABG_1 = 1,
}
impl From<IABG_A> for bool {
    #[inline(always)]
    fn from(variant: IABG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IABG`"]
pub type IABG_R = crate::R<bool, IABG_A>;
impl IABG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IABG_A {
        match self.bits {
            false => IABG_A::IABG_0,
            true => IABG_A::IABG_1,
        }
    }
    #[doc = "Checks if the value of the field is `IABG_0`"]
    #[inline(always)]
    pub fn is_iabg_0(&self) -> bool {
        *self == IABG_A::IABG_0
    }
    #[doc = "Checks if the value of the field is `IABG_1`"]
    #[inline(always)]
    pub fn is_iabg_1(&self) -> bool {
        *self == IABG_A::IABG_1
    }
}
#[doc = "Write proxy for field `IABG`"]
pub struct IABG_W<'a> {
    w: &'a mut W,
}
impl<'a> IABG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IABG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn iabg_0(self) -> &'a mut W {
        self.variant(IABG_A::IABG_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn iabg_1(self) -> &'a mut W {
        self.variant(IABG_A::IABG_1)
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
#[doc = "Reader of field `RD_DONE_NO_8CLK`"]
pub type RD_DONE_NO_8CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_DONE_NO_8CLK`"]
pub struct RD_DONE_NO_8CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_DONE_NO_8CLK_W<'a> {
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
#[doc = "Wakeup Event Enable On Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECINT_A {
    #[doc = "0: Disable"]
    WECINT_0 = 0,
    #[doc = "1: Enable"]
    WECINT_1 = 1,
}
impl From<WECINT_A> for bool {
    #[inline(always)]
    fn from(variant: WECINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WECINT`"]
pub type WECINT_R = crate::R<bool, WECINT_A>;
impl WECINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECINT_A {
        match self.bits {
            false => WECINT_A::WECINT_0,
            true => WECINT_A::WECINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECINT_0`"]
    #[inline(always)]
    pub fn is_wecint_0(&self) -> bool {
        *self == WECINT_A::WECINT_0
    }
    #[doc = "Checks if the value of the field is `WECINT_1`"]
    #[inline(always)]
    pub fn is_wecint_1(&self) -> bool {
        *self == WECINT_A::WECINT_1
    }
}
#[doc = "Write proxy for field `WECINT`"]
pub struct WECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WECINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WECINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wecint_0(self) -> &'a mut W {
        self.variant(WECINT_A::WECINT_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wecint_1(self) -> &'a mut W {
        self.variant(WECINT_A::WECINT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Wakeup Event Enable On SD Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECINS_A {
    #[doc = "0: Disable"]
    WECINS_0 = 0,
    #[doc = "1: Enable"]
    WECINS_1 = 1,
}
impl From<WECINS_A> for bool {
    #[inline(always)]
    fn from(variant: WECINS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WECINS`"]
pub type WECINS_R = crate::R<bool, WECINS_A>;
impl WECINS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECINS_A {
        match self.bits {
            false => WECINS_A::WECINS_0,
            true => WECINS_A::WECINS_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECINS_0`"]
    #[inline(always)]
    pub fn is_wecins_0(&self) -> bool {
        *self == WECINS_A::WECINS_0
    }
    #[doc = "Checks if the value of the field is `WECINS_1`"]
    #[inline(always)]
    pub fn is_wecins_1(&self) -> bool {
        *self == WECINS_A::WECINS_1
    }
}
#[doc = "Write proxy for field `WECINS`"]
pub struct WECINS_W<'a> {
    w: &'a mut W,
}
impl<'a> WECINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WECINS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wecins_0(self) -> &'a mut W {
        self.variant(WECINS_A::WECINS_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wecins_1(self) -> &'a mut W {
        self.variant(WECINS_A::WECINS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Wakeup Event Enable On SD Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WECRM_A {
    #[doc = "0: Disable"]
    WECRM_0 = 0,
    #[doc = "1: Enable"]
    WECRM_1 = 1,
}
impl From<WECRM_A> for bool {
    #[inline(always)]
    fn from(variant: WECRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WECRM`"]
pub type WECRM_R = crate::R<bool, WECRM_A>;
impl WECRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECRM_A {
        match self.bits {
            false => WECRM_A::WECRM_0,
            true => WECRM_A::WECRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECRM_0`"]
    #[inline(always)]
    pub fn is_wecrm_0(&self) -> bool {
        *self == WECRM_A::WECRM_0
    }
    #[doc = "Checks if the value of the field is `WECRM_1`"]
    #[inline(always)]
    pub fn is_wecrm_1(&self) -> bool {
        *self == WECRM_A::WECRM_1
    }
}
#[doc = "Write proxy for field `WECRM`"]
pub struct WECRM_W<'a> {
    w: &'a mut W,
}
impl<'a> WECRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WECRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wecrm_0(self) -> &'a mut W {
        self.variant(WECRM_A::WECRM_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wecrm_1(self) -> &'a mut W {
        self.variant(WECRM_A::WECRM_1)
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
#[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURST_LEN_EN_A {
    #[doc = "1: Burst length is enabled for INCR"]
    BURST_LEN_EN_1 = 1,
}
impl From<BURST_LEN_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_LEN_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BURST_LEN_EN`"]
pub type BURST_LEN_EN_R = crate::R<u8, BURST_LEN_EN_A>;
impl BURST_LEN_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BURST_LEN_EN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(BURST_LEN_EN_A::BURST_LEN_EN_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BURST_LEN_EN_1`"]
    #[inline(always)]
    pub fn is_burst_len_en_1(&self) -> bool {
        *self == BURST_LEN_EN_A::BURST_LEN_EN_1
    }
}
#[doc = "Write proxy for field `BURST_LEN_EN`"]
pub struct BURST_LEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_LEN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_LEN_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Burst length is enabled for INCR"]
    #[inline(always)]
    pub fn burst_len_en_1(self) -> &'a mut W {
        self.variant(BURST_LEN_EN_A::BURST_LEN_EN_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "NON_EXACT_BLK_RD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NON_EXACT_BLK_RD_A {
    #[doc = "0: The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_0 = 0,
    #[doc = "1: The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_1 = 1,
}
impl From<NON_EXACT_BLK_RD_A> for bool {
    #[inline(always)]
    fn from(variant: NON_EXACT_BLK_RD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NON_EXACT_BLK_RD`"]
pub type NON_EXACT_BLK_RD_R = crate::R<bool, NON_EXACT_BLK_RD_A>;
impl NON_EXACT_BLK_RD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NON_EXACT_BLK_RD_A {
        match self.bits {
            false => NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_0,
            true => NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_1,
        }
    }
    #[doc = "Checks if the value of the field is `NON_EXACT_BLK_RD_0`"]
    #[inline(always)]
    pub fn is_non_exact_blk_rd_0(&self) -> bool {
        *self == NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_0
    }
    #[doc = "Checks if the value of the field is `NON_EXACT_BLK_RD_1`"]
    #[inline(always)]
    pub fn is_non_exact_blk_rd_1(&self) -> bool {
        *self == NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_1
    }
}
#[doc = "Write proxy for field `NON_EXACT_BLK_RD`"]
pub struct NON_EXACT_BLK_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> NON_EXACT_BLK_RD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NON_EXACT_BLK_RD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    #[inline(always)]
    pub fn non_exact_blk_rd_0(self) -> &'a mut W {
        self.variant(NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_0)
    }
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    #[inline(always)]
    pub fn non_exact_blk_rd_1(self) -> &'a mut W {
        self.variant(NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn lctl(&self) -> LCTL_R {
        LCTL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline(always)]
    pub fn dtw(&self) -> DTW_R {
        DTW_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd(&self) -> D3CD_R {
        D3CD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtl(&self) -> CDTL_R {
        CDTL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn cdss(&self) -> CDSS_R {
        CDSS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn sabgreq(&self) -> SABGREQ_R {
        SABGREQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn creq(&self) -> CREQ_R {
        CREQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctl(&self) -> RWCTL_R {
        RWCTL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn iabg(&self) -> IABG_R {
        IABG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - RD_DONE_NO_8CLK"]
    #[inline(always)]
    pub fn rd_done_no_8clk(&self) -> RD_DONE_NO_8CLK_R {
        RD_DONE_NO_8CLK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wecint(&self) -> WECINT_R {
        WECINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wecins(&self) -> WECINS_R {
        WECINS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wecrm(&self) -> WECRM_R {
        WECRM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:29 - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    pub fn burst_len_en(&self) -> BURST_LEN_EN_R {
        BURST_LEN_EN_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 30 - NON_EXACT_BLK_RD"]
    #[inline(always)]
    pub fn non_exact_blk_rd(&self) -> NON_EXACT_BLK_RD_R {
        NON_EXACT_BLK_RD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn lctl(&mut self) -> LCTL_W {
        LCTL_W { w: self }
    }
    #[doc = "Bits 1:2 - Data Transfer Width"]
    #[inline(always)]
    pub fn dtw(&mut self) -> DTW_W {
        DTW_W { w: self }
    }
    #[doc = "Bit 3 - DATA3 as Card Detection Pin"]
    #[inline(always)]
    pub fn d3cd(&mut self) -> D3CD_W {
        D3CD_W { w: self }
    }
    #[doc = "Bits 4:5 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&mut self) -> EMODE_W {
        EMODE_W { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtl(&mut self) -> CDTL_W {
        CDTL_W { w: self }
    }
    #[doc = "Bit 7 - Card Detect Signal Selection"]
    #[inline(always)]
    pub fn cdss(&mut self) -> CDSS_W {
        CDSS_W { w: self }
    }
    #[doc = "Bits 8:9 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&mut self) -> DMASEL_W {
        DMASEL_W { w: self }
    }
    #[doc = "Bit 16 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn sabgreq(&mut self) -> SABGREQ_W {
        SABGREQ_W { w: self }
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn creq(&mut self) -> CREQ_W {
        CREQ_W { w: self }
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rwctl(&mut self) -> RWCTL_W {
        RWCTL_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn iabg(&mut self) -> IABG_W {
        IABG_W { w: self }
    }
    #[doc = "Bit 20 - RD_DONE_NO_8CLK"]
    #[inline(always)]
    pub fn rd_done_no_8clk(&mut self) -> RD_DONE_NO_8CLK_W {
        RD_DONE_NO_8CLK_W { w: self }
    }
    #[doc = "Bit 24 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wecint(&mut self) -> WECINT_W {
        WECINT_W { w: self }
    }
    #[doc = "Bit 25 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wecins(&mut self) -> WECINS_W {
        WECINS_W { w: self }
    }
    #[doc = "Bit 26 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wecrm(&mut self) -> WECRM_W {
        WECRM_W { w: self }
    }
    #[doc = "Bits 27:29 - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    pub fn burst_len_en(&mut self) -> BURST_LEN_EN_W {
        BURST_LEN_EN_W { w: self }
    }
    #[doc = "Bit 30 - NON_EXACT_BLK_RD"]
    #[inline(always)]
    pub fn non_exact_blk_rd(&mut self) -> NON_EXACT_BLK_RD_W {
        NON_EXACT_BLK_RD_W { w: self }
    }
}
