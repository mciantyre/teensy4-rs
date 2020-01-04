#[doc = "Reader of register SYS_CTRL"]
pub type R = crate::R<u32, super::SYS_CTRL>;
#[doc = "Writer for register SYS_CTRL"]
pub type W = crate::W<u32, super::SYS_CTRL>;
#[doc = "Register SYS_CTRL `reset()`'s with value 0x0080_800f"]
impl crate::ResetValue for super::SYS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_800f
    }
}
#[doc = "Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DVS_A {
    #[doc = "0: Divide-by-1"]
    DVS_0 = 0,
    #[doc = "1: Divide-by-2"]
    DVS_1 = 1,
    #[doc = "14: Divide-by-15"]
    DVS_14 = 14,
    #[doc = "15: Divide-by-16"]
    DVS_15 = 15,
}
impl From<DVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DVS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DVS`"]
pub type DVS_R = crate::R<u8, DVS_A>;
impl DVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DVS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DVS_A::DVS_0),
            1 => Val(DVS_A::DVS_1),
            14 => Val(DVS_A::DVS_14),
            15 => Val(DVS_A::DVS_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DVS_0`"]
    #[inline(always)]
    pub fn is_dvs_0(&self) -> bool {
        *self == DVS_A::DVS_0
    }
    #[doc = "Checks if the value of the field is `DVS_1`"]
    #[inline(always)]
    pub fn is_dvs_1(&self) -> bool {
        *self == DVS_A::DVS_1
    }
    #[doc = "Checks if the value of the field is `DVS_14`"]
    #[inline(always)]
    pub fn is_dvs_14(&self) -> bool {
        *self == DVS_A::DVS_14
    }
    #[doc = "Checks if the value of the field is `DVS_15`"]
    #[inline(always)]
    pub fn is_dvs_15(&self) -> bool {
        *self == DVS_A::DVS_15
    }
}
#[doc = "Write proxy for field `DVS`"]
pub struct DVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DVS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn dvs_0(self) -> &'a mut W {
        self.variant(DVS_A::DVS_0)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn dvs_1(self) -> &'a mut W {
        self.variant(DVS_A::DVS_1)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn dvs_14(self) -> &'a mut W {
        self.variant(DVS_A::DVS_14)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn dvs_15(self) -> &'a mut W {
        self.variant(DVS_A::DVS_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SDCLKFS`"]
pub type SDCLKFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDCLKFS`"]
pub struct SDCLKFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Data Timeout Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTOCV_A {
    #[doc = "0: SDCLK x 2 14"]
    DTOCV_0 = 0,
    #[doc = "1: SDCLK x 2 15"]
    DTOCV_1 = 1,
    #[doc = "13: SDCLK x 2 27"]
    DTOCV_13 = 13,
    #[doc = "14: SDCLK x 2 28"]
    DTOCV_14 = 14,
    #[doc = "15: SDCLK x 2 29"]
    DTOCV_15 = 15,
}
impl From<DTOCV_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOCV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTOCV`"]
pub type DTOCV_R = crate::R<u8, DTOCV_A>;
impl DTOCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTOCV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTOCV_A::DTOCV_0),
            1 => Val(DTOCV_A::DTOCV_1),
            13 => Val(DTOCV_A::DTOCV_13),
            14 => Val(DTOCV_A::DTOCV_14),
            15 => Val(DTOCV_A::DTOCV_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DTOCV_0`"]
    #[inline(always)]
    pub fn is_dtocv_0(&self) -> bool {
        *self == DTOCV_A::DTOCV_0
    }
    #[doc = "Checks if the value of the field is `DTOCV_1`"]
    #[inline(always)]
    pub fn is_dtocv_1(&self) -> bool {
        *self == DTOCV_A::DTOCV_1
    }
    #[doc = "Checks if the value of the field is `DTOCV_13`"]
    #[inline(always)]
    pub fn is_dtocv_13(&self) -> bool {
        *self == DTOCV_A::DTOCV_13
    }
    #[doc = "Checks if the value of the field is `DTOCV_14`"]
    #[inline(always)]
    pub fn is_dtocv_14(&self) -> bool {
        *self == DTOCV_A::DTOCV_14
    }
    #[doc = "Checks if the value of the field is `DTOCV_15`"]
    #[inline(always)]
    pub fn is_dtocv_15(&self) -> bool {
        *self == DTOCV_A::DTOCV_15
    }
}
#[doc = "Write proxy for field `DTOCV`"]
pub struct DTOCV_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOCV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDCLK x 2 14"]
    #[inline(always)]
    pub fn dtocv_0(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_0)
    }
    #[doc = "SDCLK x 2 15"]
    #[inline(always)]
    pub fn dtocv_1(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_1)
    }
    #[doc = "SDCLK x 2 27"]
    #[inline(always)]
    pub fn dtocv_13(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_13)
    }
    #[doc = "SDCLK x 2 28"]
    #[inline(always)]
    pub fn dtocv_14(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_14)
    }
    #[doc = "SDCLK x 2 29"]
    #[inline(always)]
    pub fn dtocv_15(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `IPP_RST_N`"]
pub type IPP_RST_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPP_RST_N`"]
pub struct IPP_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> IPP_RST_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Software Reset For ALL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTA_A {
    #[doc = "0: No Reset"]
    RSTA_0 = 0,
    #[doc = "1: Reset"]
    RSTA_1 = 1,
}
impl From<RSTA_A> for bool {
    #[inline(always)]
    fn from(variant: RSTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTA`"]
pub type RSTA_R = crate::R<bool, RSTA_A>;
impl RSTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTA_A {
        match self.bits {
            false => RSTA_A::RSTA_0,
            true => RSTA_A::RSTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTA_0`"]
    #[inline(always)]
    pub fn is_rsta_0(&self) -> bool {
        *self == RSTA_A::RSTA_0
    }
    #[doc = "Checks if the value of the field is `RSTA_1`"]
    #[inline(always)]
    pub fn is_rsta_1(&self) -> bool {
        *self == RSTA_A::RSTA_1
    }
}
#[doc = "Write proxy for field `RSTA`"]
pub struct RSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn rsta_0(self) -> &'a mut W {
        self.variant(RSTA_A::RSTA_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rsta_1(self) -> &'a mut W {
        self.variant(RSTA_A::RSTA_1)
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
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTC_A {
    #[doc = "0: No Reset"]
    RSTC_0 = 0,
    #[doc = "1: Reset"]
    RSTC_1 = 1,
}
impl From<RSTC_A> for bool {
    #[inline(always)]
    fn from(variant: RSTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTC`"]
pub type RSTC_R = crate::R<bool, RSTC_A>;
impl RSTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTC_A {
        match self.bits {
            false => RSTC_A::RSTC_0,
            true => RSTC_A::RSTC_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTC_0`"]
    #[inline(always)]
    pub fn is_rstc_0(&self) -> bool {
        *self == RSTC_A::RSTC_0
    }
    #[doc = "Checks if the value of the field is `RSTC_1`"]
    #[inline(always)]
    pub fn is_rstc_1(&self) -> bool {
        *self == RSTC_A::RSTC_1
    }
}
#[doc = "Write proxy for field `RSTC`"]
pub struct RSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn rstc_0(self) -> &'a mut W {
        self.variant(RSTC_A::RSTC_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rstc_1(self) -> &'a mut W {
        self.variant(RSTC_A::RSTC_1)
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
#[doc = "Software Reset For DATA Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTD_A {
    #[doc = "0: No Reset"]
    RSTD_0 = 0,
    #[doc = "1: Reset"]
    RSTD_1 = 1,
}
impl From<RSTD_A> for bool {
    #[inline(always)]
    fn from(variant: RSTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTD`"]
pub type RSTD_R = crate::R<bool, RSTD_A>;
impl RSTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTD_A {
        match self.bits {
            false => RSTD_A::RSTD_0,
            true => RSTD_A::RSTD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTD_0`"]
    #[inline(always)]
    pub fn is_rstd_0(&self) -> bool {
        *self == RSTD_A::RSTD_0
    }
    #[doc = "Checks if the value of the field is `RSTD_1`"]
    #[inline(always)]
    pub fn is_rstd_1(&self) -> bool {
        *self == RSTD_A::RSTD_1
    }
}
#[doc = "Write proxy for field `RSTD`"]
pub struct RSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Reset"]
    #[inline(always)]
    pub fn rstd_0(self) -> &'a mut W {
        self.variant(RSTD_A::RSTD_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rstd_1(self) -> &'a mut W {
        self.variant(RSTD_A::RSTD_1)
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
#[doc = "Reader of field `INITA`"]
pub type INITA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INITA`"]
pub struct INITA_W<'a> {
    w: &'a mut W,
}
impl<'a> INITA_W<'a> {
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
#[doc = "Reader of field `RSTT`"]
pub type RSTT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTT`"]
pub struct RSTT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTT_W<'a> {
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
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    pub fn dvs(&self) -> DVS_R {
        DVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfs(&self) -> SDCLKFS_R {
        SDCLKFS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtocv(&self) -> DTOCV_R {
        DTOCV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - IPP_RST_N"]
    #[inline(always)]
    pub fn ipp_rst_n(&self) -> IPP_RST_N_R {
        IPP_RST_N_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline(always)]
    pub fn rsta(&self) -> RSTA_R {
        RSTA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn rstc(&self) -> RSTC_R {
        RSTC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Software Reset For DATA Line"]
    #[inline(always)]
    pub fn rstd(&self) -> RSTD_R {
        RSTD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline(always)]
    pub fn inita(&self) -> INITA_R {
        INITA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Reset Tuning"]
    #[inline(always)]
    pub fn rstt(&self) -> RSTT_R {
        RSTT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    pub fn dvs(&mut self) -> DVS_W {
        DVS_W { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfs(&mut self) -> SDCLKFS_W {
        SDCLKFS_W { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtocv(&mut self) -> DTOCV_W {
        DTOCV_W { w: self }
    }
    #[doc = "Bit 23 - IPP_RST_N"]
    #[inline(always)]
    pub fn ipp_rst_n(&mut self) -> IPP_RST_N_W {
        IPP_RST_N_W { w: self }
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline(always)]
    pub fn rsta(&mut self) -> RSTA_W {
        RSTA_W { w: self }
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn rstc(&mut self) -> RSTC_W {
        RSTC_W { w: self }
    }
    #[doc = "Bit 26 - Software Reset For DATA Line"]
    #[inline(always)]
    pub fn rstd(&mut self) -> RSTD_W {
        RSTD_W { w: self }
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline(always)]
    pub fn inita(&mut self) -> INITA_W {
        INITA_W { w: self }
    }
    #[doc = "Bit 28 - Reset Tuning"]
    #[inline(always)]
    pub fn rstt(&mut self) -> RSTT_W {
        RSTT_W { w: self }
    }
}
