#[doc = "Reader of register TCR4"]
pub type R = crate::R<u32, super::TCR4>;
#[doc = "Writer for register TCR4"]
pub type W = crate::W<u32, super::TCR4>;
#[doc = "Register TCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Frame Sync Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSD_A {
    #[doc = "0: Frame sync is generated externally in Slave mode."]
    FSD_0 = 0,
    #[doc = "1: Frame sync is generated internally in Master mode."]
    FSD_1 = 1,
}
impl From<FSD_A> for bool {
    #[inline(always)]
    fn from(variant: FSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSD`"]
pub type FSD_R = crate::R<bool, FSD_A>;
impl FSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSD_A {
        match self.bits {
            false => FSD_A::FSD_0,
            true => FSD_A::FSD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FSD_0`"]
    #[inline(always)]
    pub fn is_fsd_0(&self) -> bool {
        *self == FSD_A::FSD_0
    }
    #[doc = "Checks if the value of the field is `FSD_1`"]
    #[inline(always)]
    pub fn is_fsd_1(&self) -> bool {
        *self == FSD_A::FSD_1
    }
}
#[doc = "Write proxy for field `FSD`"]
pub struct FSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync is generated externally in Slave mode."]
    #[inline(always)]
    pub fn fsd_0(self) -> &'a mut W {
        self.variant(FSD_A::FSD_0)
    }
    #[doc = "Frame sync is generated internally in Master mode."]
    #[inline(always)]
    pub fn fsd_1(self) -> &'a mut W {
        self.variant(FSD_A::FSD_1)
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
#[doc = "Frame Sync Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSP_A {
    #[doc = "0: Frame sync is active high."]
    FSP_0 = 0,
    #[doc = "1: Frame sync is active low."]
    FSP_1 = 1,
}
impl From<FSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSP`"]
pub type FSP_R = crate::R<bool, FSP_A>;
impl FSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSP_A {
        match self.bits {
            false => FSP_A::FSP_0,
            true => FSP_A::FSP_1,
        }
    }
    #[doc = "Checks if the value of the field is `FSP_0`"]
    #[inline(always)]
    pub fn is_fsp_0(&self) -> bool {
        *self == FSP_A::FSP_0
    }
    #[doc = "Checks if the value of the field is `FSP_1`"]
    #[inline(always)]
    pub fn is_fsp_1(&self) -> bool {
        *self == FSP_A::FSP_1
    }
}
#[doc = "Write proxy for field `FSP`"]
pub struct FSP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync is active high."]
    #[inline(always)]
    pub fn fsp_0(self) -> &'a mut W {
        self.variant(FSP_A::FSP_0)
    }
    #[doc = "Frame sync is active low."]
    #[inline(always)]
    pub fn fsp_1(self) -> &'a mut W {
        self.variant(FSP_A::FSP_1)
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
#[doc = "On Demand Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONDEM_A {
    #[doc = "0: Internal frame sync is generated continuously."]
    ONDEM_0 = 0,
    #[doc = "1: Internal frame sync is generated when the FIFO warning flag is clear."]
    ONDEM_1 = 1,
}
impl From<ONDEM_A> for bool {
    #[inline(always)]
    fn from(variant: ONDEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONDEM`"]
pub type ONDEM_R = crate::R<bool, ONDEM_A>;
impl ONDEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONDEM_A {
        match self.bits {
            false => ONDEM_A::ONDEM_0,
            true => ONDEM_A::ONDEM_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONDEM_0`"]
    #[inline(always)]
    pub fn is_ondem_0(&self) -> bool {
        *self == ONDEM_A::ONDEM_0
    }
    #[doc = "Checks if the value of the field is `ONDEM_1`"]
    #[inline(always)]
    pub fn is_ondem_1(&self) -> bool {
        *self == ONDEM_A::ONDEM_1
    }
}
#[doc = "Write proxy for field `ONDEM`"]
pub struct ONDEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONDEM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal frame sync is generated continuously."]
    #[inline(always)]
    pub fn ondem_0(self) -> &'a mut W {
        self.variant(ONDEM_A::ONDEM_0)
    }
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    #[inline(always)]
    pub fn ondem_1(self) -> &'a mut W {
        self.variant(ONDEM_A::ONDEM_1)
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
#[doc = "Frame Sync Early\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSE_A {
    #[doc = "0: Frame sync asserts with the first bit of the frame."]
    FSE_0 = 0,
    #[doc = "1: Frame sync asserts one bit before the first bit of the frame."]
    FSE_1 = 1,
}
impl From<FSE_A> for bool {
    #[inline(always)]
    fn from(variant: FSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSE`"]
pub type FSE_R = crate::R<bool, FSE_A>;
impl FSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSE_A {
        match self.bits {
            false => FSE_A::FSE_0,
            true => FSE_A::FSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FSE_0`"]
    #[inline(always)]
    pub fn is_fse_0(&self) -> bool {
        *self == FSE_A::FSE_0
    }
    #[doc = "Checks if the value of the field is `FSE_1`"]
    #[inline(always)]
    pub fn is_fse_1(&self) -> bool {
        *self == FSE_A::FSE_1
    }
}
#[doc = "Write proxy for field `FSE`"]
pub struct FSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline(always)]
    pub fn fse_0(self) -> &'a mut W {
        self.variant(FSE_A::FSE_0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline(always)]
    pub fn fse_1(self) -> &'a mut W {
        self.variant(FSE_A::FSE_1)
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
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MF_A {
    #[doc = "0: LSB is transmitted first."]
    MF_0 = 0,
    #[doc = "1: MSB is transmitted first."]
    MF_1 = 1,
}
impl From<MF_A> for bool {
    #[inline(always)]
    fn from(variant: MF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MF`"]
pub type MF_R = crate::R<bool, MF_A>;
impl MF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MF_A {
        match self.bits {
            false => MF_A::MF_0,
            true => MF_A::MF_1,
        }
    }
    #[doc = "Checks if the value of the field is `MF_0`"]
    #[inline(always)]
    pub fn is_mf_0(&self) -> bool {
        *self == MF_A::MF_0
    }
    #[doc = "Checks if the value of the field is `MF_1`"]
    #[inline(always)]
    pub fn is_mf_1(&self) -> bool {
        *self == MF_A::MF_1
    }
}
#[doc = "Write proxy for field `MF`"]
pub struct MF_W<'a> {
    w: &'a mut W,
}
impl<'a> MF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSB is transmitted first."]
    #[inline(always)]
    pub fn mf_0(self) -> &'a mut W {
        self.variant(MF_A::MF_0)
    }
    #[doc = "MSB is transmitted first."]
    #[inline(always)]
    pub fn mf_1(self) -> &'a mut W {
        self.variant(MF_A::MF_1)
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
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHMOD_A {
    #[doc = "0: TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled."]
    CHMOD_0 = 0,
    #[doc = "1: Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled."]
    CHMOD_1 = 1,
}
impl From<CHMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CHMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHMOD`"]
pub type CHMOD_R = crate::R<bool, CHMOD_A>;
impl CHMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMOD_A {
        match self.bits {
            false => CHMOD_A::CHMOD_0,
            true => CHMOD_A::CHMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `CHMOD_0`"]
    #[inline(always)]
    pub fn is_chmod_0(&self) -> bool {
        *self == CHMOD_A::CHMOD_0
    }
    #[doc = "Checks if the value of the field is `CHMOD_1`"]
    #[inline(always)]
    pub fn is_chmod_1(&self) -> bool {
        *self == CHMOD_A::CHMOD_1
    }
}
#[doc = "Write proxy for field `CHMOD`"]
pub struct CHMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled."]
    #[inline(always)]
    pub fn chmod_0(self) -> &'a mut W {
        self.variant(CHMOD_A::CHMOD_0)
    }
    #[doc = "Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled."]
    #[inline(always)]
    pub fn chmod_1(self) -> &'a mut W {
        self.variant(CHMOD_A::CHMOD_1)
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
#[doc = "Reader of field `SYWD`"]
pub type SYWD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYWD`"]
pub struct SYWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FRSZ`"]
pub type FRSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRSZ`"]
pub struct FRSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "FIFO Packing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FPACK_A {
    #[doc = "0: FIFO packing is disabled"]
    FPACK_0 = 0,
    #[doc = "2: 8-bit FIFO packing is enabled"]
    FPACK_2 = 2,
    #[doc = "3: 16-bit FIFO packing is enabled"]
    FPACK_3 = 3,
}
impl From<FPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: FPACK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FPACK`"]
pub type FPACK_R = crate::R<u8, FPACK_A>;
impl FPACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FPACK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FPACK_A::FPACK_0),
            2 => Val(FPACK_A::FPACK_2),
            3 => Val(FPACK_A::FPACK_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FPACK_0`"]
    #[inline(always)]
    pub fn is_fpack_0(&self) -> bool {
        *self == FPACK_A::FPACK_0
    }
    #[doc = "Checks if the value of the field is `FPACK_2`"]
    #[inline(always)]
    pub fn is_fpack_2(&self) -> bool {
        *self == FPACK_A::FPACK_2
    }
    #[doc = "Checks if the value of the field is `FPACK_3`"]
    #[inline(always)]
    pub fn is_fpack_3(&self) -> bool {
        *self == FPACK_A::FPACK_3
    }
}
#[doc = "Write proxy for field `FPACK`"]
pub struct FPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> FPACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FIFO packing is disabled"]
    #[inline(always)]
    pub fn fpack_0(self) -> &'a mut W {
        self.variant(FPACK_A::FPACK_0)
    }
    #[doc = "8-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn fpack_2(self) -> &'a mut W {
        self.variant(FPACK_A::FPACK_2)
    }
    #[doc = "16-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn fpack_3(self) -> &'a mut W {
        self.variant(FPACK_A::FPACK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "FIFO Combine Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCOMB_A {
    #[doc = "0: FIFO combine mode disabled."]
    FCOMB_0 = 0,
    #[doc = "1: FIFO combine mode enabled on FIFO reads (from transmit shift registers)."]
    FCOMB_1 = 1,
    #[doc = "2: FIFO combine mode enabled on FIFO writes (by software)."]
    FCOMB_2 = 2,
    #[doc = "3: FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
    FCOMB_3 = 3,
}
impl From<FCOMB_A> for u8 {
    #[inline(always)]
    fn from(variant: FCOMB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FCOMB`"]
pub type FCOMB_R = crate::R<u8, FCOMB_A>;
impl FCOMB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCOMB_A {
        match self.bits {
            0 => FCOMB_A::FCOMB_0,
            1 => FCOMB_A::FCOMB_1,
            2 => FCOMB_A::FCOMB_2,
            3 => FCOMB_A::FCOMB_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FCOMB_0`"]
    #[inline(always)]
    pub fn is_fcomb_0(&self) -> bool {
        *self == FCOMB_A::FCOMB_0
    }
    #[doc = "Checks if the value of the field is `FCOMB_1`"]
    #[inline(always)]
    pub fn is_fcomb_1(&self) -> bool {
        *self == FCOMB_A::FCOMB_1
    }
    #[doc = "Checks if the value of the field is `FCOMB_2`"]
    #[inline(always)]
    pub fn is_fcomb_2(&self) -> bool {
        *self == FCOMB_A::FCOMB_2
    }
    #[doc = "Checks if the value of the field is `FCOMB_3`"]
    #[inline(always)]
    pub fn is_fcomb_3(&self) -> bool {
        *self == FCOMB_A::FCOMB_3
    }
}
#[doc = "Write proxy for field `FCOMB`"]
pub struct FCOMB_W<'a> {
    w: &'a mut W,
}
impl<'a> FCOMB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCOMB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FIFO combine mode disabled."]
    #[inline(always)]
    pub fn fcomb_0(self) -> &'a mut W {
        self.variant(FCOMB_A::FCOMB_0)
    }
    #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers)."]
    #[inline(always)]
    pub fn fcomb_1(self) -> &'a mut W {
        self.variant(FCOMB_A::FCOMB_1)
    }
    #[doc = "FIFO combine mode enabled on FIFO writes (by software)."]
    #[inline(always)]
    pub fn fcomb_2(self) -> &'a mut W {
        self.variant(FCOMB_A::FCOMB_2)
    }
    #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
    #[inline(always)]
    pub fn fcomb_3(self) -> &'a mut W {
        self.variant(FCOMB_A::FCOMB_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "FIFO Continue on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCONT_A {
    #[doc = "0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    FCONT_0 = 0,
    #[doc = "1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    FCONT_1 = 1,
}
impl From<FCONT_A> for bool {
    #[inline(always)]
    fn from(variant: FCONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCONT`"]
pub type FCONT_R = crate::R<bool, FCONT_A>;
impl FCONT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCONT_A {
        match self.bits {
            false => FCONT_A::FCONT_0,
            true => FCONT_A::FCONT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCONT_0`"]
    #[inline(always)]
    pub fn is_fcont_0(&self) -> bool {
        *self == FCONT_A::FCONT_0
    }
    #[doc = "Checks if the value of the field is `FCONT_1`"]
    #[inline(always)]
    pub fn is_fcont_1(&self) -> bool {
        *self == FCONT_A::FCONT_1
    }
}
#[doc = "Write proxy for field `FCONT`"]
pub struct FCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCONT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    #[inline(always)]
    pub fn fcont_0(self) -> &'a mut W {
        self.variant(FCONT_A::FCONT_0)
    }
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    #[inline(always)]
    pub fn fcont_1(self) -> &'a mut W {
        self.variant(FCONT_A::FCONT_1)
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
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&self) -> FSP_R {
        FSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    pub fn ondem(&self) -> ONDEM_R {
        ONDEM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&self) -> FSE_R {
        FSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel Mode"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&self) -> SYWD_R {
        SYWD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline(always)]
    pub fn frsz(&self) -> FRSZ_R {
        FRSZ_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    pub fn fpack(&self) -> FPACK_R {
        FPACK_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline(always)]
    pub fn fcomb(&self) -> FCOMB_R {
        FCOMB_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    pub fn fcont(&self) -> FCONT_R {
        FCONT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W {
        FSD_W { w: self }
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&mut self) -> FSP_W {
        FSP_W { w: self }
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    pub fn ondem(&mut self) -> ONDEM_W {
        ONDEM_W { w: self }
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&mut self) -> FSE_W {
        FSE_W { w: self }
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&mut self) -> MF_W {
        MF_W { w: self }
    }
    #[doc = "Bit 5 - Channel Mode"]
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W {
        CHMOD_W { w: self }
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&mut self) -> SYWD_W {
        SYWD_W { w: self }
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline(always)]
    pub fn frsz(&mut self) -> FRSZ_W {
        FRSZ_W { w: self }
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    pub fn fpack(&mut self) -> FPACK_W {
        FPACK_W { w: self }
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline(always)]
    pub fn fcomb(&mut self) -> FCOMB_W {
        FCOMB_W { w: self }
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    pub fn fcont(&mut self) -> FCONT_W {
        FCONT_W { w: self }
    }
}
