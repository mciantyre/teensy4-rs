#[doc = "Reader of register HPSVCR"]
pub type R = crate::R<u32, super::HPSVCR>;
#[doc = "Writer for register HPSVCR"]
pub type W = crate::W<u32, super::HPSVCR>;
#[doc = "Register HPSVCR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPSVCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV0_CFG_A {
    #[doc = "0: Security Violation 0 is a non-fatal violation"]
    SV0_CFG_0 = 0,
    #[doc = "1: Security Violation 0 is a fatal violation"]
    SV0_CFG_1 = 1,
}
impl From<SV0_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV0_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV0_CFG`"]
pub type SV0_CFG_R = crate::R<bool, SV0_CFG_A>;
impl SV0_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV0_CFG_A {
        match self.bits {
            false => SV0_CFG_A::SV0_CFG_0,
            true => SV0_CFG_A::SV0_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV0_CFG_0`"]
    #[inline(always)]
    pub fn is_sv0_cfg_0(&self) -> bool {
        *self == SV0_CFG_A::SV0_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV0_CFG_1`"]
    #[inline(always)]
    pub fn is_sv0_cfg_1(&self) -> bool {
        *self == SV0_CFG_A::SV0_CFG_1
    }
}
#[doc = "Write proxy for field `SV0_CFG`"]
pub struct SV0_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SV0_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV0_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 0 is a non-fatal violation"]
    #[inline(always)]
    pub fn sv0_cfg_0(self) -> &'a mut W {
        self.variant(SV0_CFG_A::SV0_CFG_0)
    }
    #[doc = "Security Violation 0 is a fatal violation"]
    #[inline(always)]
    pub fn sv0_cfg_1(self) -> &'a mut W {
        self.variant(SV0_CFG_A::SV0_CFG_1)
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
#[doc = "Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV1_CFG_A {
    #[doc = "0: Security Violation 1 is a non-fatal violation"]
    SV1_CFG_0 = 0,
    #[doc = "1: Security Violation 1 is a fatal violation"]
    SV1_CFG_1 = 1,
}
impl From<SV1_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV1_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV1_CFG`"]
pub type SV1_CFG_R = crate::R<bool, SV1_CFG_A>;
impl SV1_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV1_CFG_A {
        match self.bits {
            false => SV1_CFG_A::SV1_CFG_0,
            true => SV1_CFG_A::SV1_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV1_CFG_0`"]
    #[inline(always)]
    pub fn is_sv1_cfg_0(&self) -> bool {
        *self == SV1_CFG_A::SV1_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV1_CFG_1`"]
    #[inline(always)]
    pub fn is_sv1_cfg_1(&self) -> bool {
        *self == SV1_CFG_A::SV1_CFG_1
    }
}
#[doc = "Write proxy for field `SV1_CFG`"]
pub struct SV1_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SV1_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV1_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 1 is a non-fatal violation"]
    #[inline(always)]
    pub fn sv1_cfg_0(self) -> &'a mut W {
        self.variant(SV1_CFG_A::SV1_CFG_0)
    }
    #[doc = "Security Violation 1 is a fatal violation"]
    #[inline(always)]
    pub fn sv1_cfg_1(self) -> &'a mut W {
        self.variant(SV1_CFG_A::SV1_CFG_1)
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
#[doc = "Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV2_CFG_A {
    #[doc = "0: Security Violation 2 is a non-fatal violation"]
    SV2_CFG_0 = 0,
    #[doc = "1: Security Violation 2 is a fatal violation"]
    SV2_CFG_1 = 1,
}
impl From<SV2_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV2_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV2_CFG`"]
pub type SV2_CFG_R = crate::R<bool, SV2_CFG_A>;
impl SV2_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV2_CFG_A {
        match self.bits {
            false => SV2_CFG_A::SV2_CFG_0,
            true => SV2_CFG_A::SV2_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV2_CFG_0`"]
    #[inline(always)]
    pub fn is_sv2_cfg_0(&self) -> bool {
        *self == SV2_CFG_A::SV2_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV2_CFG_1`"]
    #[inline(always)]
    pub fn is_sv2_cfg_1(&self) -> bool {
        *self == SV2_CFG_A::SV2_CFG_1
    }
}
#[doc = "Write proxy for field `SV2_CFG`"]
pub struct SV2_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SV2_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV2_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 2 is a non-fatal violation"]
    #[inline(always)]
    pub fn sv2_cfg_0(self) -> &'a mut W {
        self.variant(SV2_CFG_A::SV2_CFG_0)
    }
    #[doc = "Security Violation 2 is a fatal violation"]
    #[inline(always)]
    pub fn sv2_cfg_1(self) -> &'a mut W {
        self.variant(SV2_CFG_A::SV2_CFG_1)
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
#[doc = "Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV3_CFG_A {
    #[doc = "0: Security Violation 3 is a non-fatal violation"]
    SV3_CFG_0 = 0,
    #[doc = "1: Security Violation 3 is a fatal violation"]
    SV3_CFG_1 = 1,
}
impl From<SV3_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV3_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV3_CFG`"]
pub type SV3_CFG_R = crate::R<bool, SV3_CFG_A>;
impl SV3_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV3_CFG_A {
        match self.bits {
            false => SV3_CFG_A::SV3_CFG_0,
            true => SV3_CFG_A::SV3_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV3_CFG_0`"]
    #[inline(always)]
    pub fn is_sv3_cfg_0(&self) -> bool {
        *self == SV3_CFG_A::SV3_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV3_CFG_1`"]
    #[inline(always)]
    pub fn is_sv3_cfg_1(&self) -> bool {
        *self == SV3_CFG_A::SV3_CFG_1
    }
}
#[doc = "Write proxy for field `SV3_CFG`"]
pub struct SV3_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SV3_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV3_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 3 is a non-fatal violation"]
    #[inline(always)]
    pub fn sv3_cfg_0(self) -> &'a mut W {
        self.variant(SV3_CFG_A::SV3_CFG_0)
    }
    #[doc = "Security Violation 3 is a fatal violation"]
    #[inline(always)]
    pub fn sv3_cfg_1(self) -> &'a mut W {
        self.variant(SV3_CFG_A::SV3_CFG_1)
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
#[doc = "Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV4_CFG_A {
    #[doc = "0: Security Violation 4 is a non-fatal violation"]
    SV4_CFG_0 = 0,
    #[doc = "1: Security Violation 4 is a fatal violation"]
    SV4_CFG_1 = 1,
}
impl From<SV4_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV4_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV4_CFG`"]
pub type SV4_CFG_R = crate::R<bool, SV4_CFG_A>;
impl SV4_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV4_CFG_A {
        match self.bits {
            false => SV4_CFG_A::SV4_CFG_0,
            true => SV4_CFG_A::SV4_CFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV4_CFG_0`"]
    #[inline(always)]
    pub fn is_sv4_cfg_0(&self) -> bool {
        *self == SV4_CFG_A::SV4_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV4_CFG_1`"]
    #[inline(always)]
    pub fn is_sv4_cfg_1(&self) -> bool {
        *self == SV4_CFG_A::SV4_CFG_1
    }
}
#[doc = "Write proxy for field `SV4_CFG`"]
pub struct SV4_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SV4_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV4_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 4 is a non-fatal violation"]
    #[inline(always)]
    pub fn sv4_cfg_0(self) -> &'a mut W {
        self.variant(SV4_CFG_A::SV4_CFG_0)
    }
    #[doc = "Security Violation 4 is a fatal violation"]
    #[inline(always)]
    pub fn sv4_cfg_1(self) -> &'a mut W {
        self.variant(SV4_CFG_A::SV4_CFG_1)
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
#[doc = "Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SV5_CFG_A {
    #[doc = "0: Security Violation 5 is disabled"]
    SV5_CFG_0 = 0,
    #[doc = "1: Security Violation 5 is a non-fatal violation"]
    SV5_CFG_1 = 1,
    #[doc = "2: Security Violation 5 is a fatal violation"]
    SV5_CFG_2 = 2,
}
impl From<SV5_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SV5_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SV5_CFG`"]
pub type SV5_CFG_R = crate::R<u8, SV5_CFG_A>;
impl SV5_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SV5_CFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SV5_CFG_A::SV5_CFG_0),
            1 => Val(SV5_CFG_A::SV5_CFG_1),
            2 => Val(SV5_CFG_A::SV5_CFG_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SV5_CFG_0`"]
    #[inline(always)]
    pub fn is_sv5_cfg_0(&self) -> bool {
        *self == SV5_CFG_A::SV5_CFG_0
    }
    #[doc = "Checks if the value of the field is `SV5_CFG_1`"]
    #[inline(always)]
    pub fn is_sv5_cfg_1(&self) -> bool {
        *self == SV5_CFG_A::SV5_CFG_1
    }
    #[doc = "Checks if the value of the field is `SV5_CFG_2`"]
    #[inline(always)]
    pub fn is_sv5_cfg_2(&self) -> bool {
        *self == SV5_CFG_A::SV5_CFG_2
    }
}
#[doc = "Write proxy for field `SV5_CFG`"]
pub struct SV5_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SV5_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV5_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Security Violation 5 is disabled"]
    #[inline(always)]
    pub fn sv5_cfg_0(self) -> &'a mut W {
        self.variant(SV5_CFG_A::SV5_CFG_0)
    }
    #[doc = "Security Violation 5 is a non-fatal violation"]
    #[inline(always)]
    pub fn sv5_cfg_1(self) -> &'a mut W {
        self.variant(SV5_CFG_A::SV5_CFG_1)
    }
    #[doc = "Security Violation 5 is a fatal violation"]
    #[inline(always)]
    pub fn sv5_cfg_2(self) -> &'a mut W {
        self.variant(SV5_CFG_A::SV5_CFG_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "LP Security Violation Configuration This field configures the LP security violation source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPSV_CFG_A {
    #[doc = "0: LP security violation is disabled"]
    LPSV_CFG_0 = 0,
    #[doc = "1: LP security violation is a non-fatal violation"]
    LPSV_CFG_1 = 1,
    #[doc = "2: LP security violation is a fatal violation"]
    LPSV_CFG_2 = 2,
}
impl From<LPSV_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: LPSV_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPSV_CFG`"]
pub type LPSV_CFG_R = crate::R<u8, LPSV_CFG_A>;
impl LPSV_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPSV_CFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPSV_CFG_A::LPSV_CFG_0),
            1 => Val(LPSV_CFG_A::LPSV_CFG_1),
            2 => Val(LPSV_CFG_A::LPSV_CFG_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPSV_CFG_0`"]
    #[inline(always)]
    pub fn is_lpsv_cfg_0(&self) -> bool {
        *self == LPSV_CFG_A::LPSV_CFG_0
    }
    #[doc = "Checks if the value of the field is `LPSV_CFG_1`"]
    #[inline(always)]
    pub fn is_lpsv_cfg_1(&self) -> bool {
        *self == LPSV_CFG_A::LPSV_CFG_1
    }
    #[doc = "Checks if the value of the field is `LPSV_CFG_2`"]
    #[inline(always)]
    pub fn is_lpsv_cfg_2(&self) -> bool {
        *self == LPSV_CFG_A::LPSV_CFG_2
    }
}
#[doc = "Write proxy for field `LPSV_CFG`"]
pub struct LPSV_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSV_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSV_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LP security violation is disabled"]
    #[inline(always)]
    pub fn lpsv_cfg_0(self) -> &'a mut W {
        self.variant(LPSV_CFG_A::LPSV_CFG_0)
    }
    #[doc = "LP security violation is a non-fatal violation"]
    #[inline(always)]
    pub fn lpsv_cfg_1(self) -> &'a mut W {
        self.variant(LPSV_CFG_A::LPSV_CFG_1)
    }
    #[doc = "LP security violation is a fatal violation"]
    #[inline(always)]
    pub fn lpsv_cfg_2(self) -> &'a mut W {
        self.variant(LPSV_CFG_A::LPSV_CFG_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline(always)]
    pub fn sv0_cfg(&self) -> SV0_CFG_R {
        SV0_CFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline(always)]
    pub fn sv1_cfg(&self) -> SV1_CFG_R {
        SV1_CFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline(always)]
    pub fn sv2_cfg(&self) -> SV2_CFG_R {
        SV2_CFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline(always)]
    pub fn sv3_cfg(&self) -> SV3_CFG_R {
        SV3_CFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline(always)]
    pub fn sv4_cfg(&self) -> SV4_CFG_R {
        SV4_CFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline(always)]
    pub fn sv5_cfg(&self) -> SV5_CFG_R {
        SV5_CFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline(always)]
    pub fn lpsv_cfg(&self) -> LPSV_CFG_R {
        LPSV_CFG_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline(always)]
    pub fn sv0_cfg(&mut self) -> SV0_CFG_W {
        SV0_CFG_W { w: self }
    }
    #[doc = "Bit 1 - Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline(always)]
    pub fn sv1_cfg(&mut self) -> SV1_CFG_W {
        SV1_CFG_W { w: self }
    }
    #[doc = "Bit 2 - Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline(always)]
    pub fn sv2_cfg(&mut self) -> SV2_CFG_W {
        SV2_CFG_W { w: self }
    }
    #[doc = "Bit 3 - Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline(always)]
    pub fn sv3_cfg(&mut self) -> SV3_CFG_W {
        SV3_CFG_W { w: self }
    }
    #[doc = "Bit 4 - Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline(always)]
    pub fn sv4_cfg(&mut self) -> SV4_CFG_W {
        SV4_CFG_W { w: self }
    }
    #[doc = "Bits 5:6 - Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline(always)]
    pub fn sv5_cfg(&mut self) -> SV5_CFG_W {
        SV5_CFG_W { w: self }
    }
    #[doc = "Bits 30:31 - LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline(always)]
    pub fn lpsv_cfg(&mut self) -> LPSV_CFG_W {
        LPSV_CFG_W { w: self }
    }
}
