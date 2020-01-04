#[doc = "Reader of register HPSVSR"]
pub type R = crate::R<u32, super::HPSVSR>;
#[doc = "Writer for register HPSVSR"]
pub type W = crate::W<u32, super::HPSVSR>;
#[doc = "Register HPSVSR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::HPSVSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Security Violation 0 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV0_A {
    #[doc = "0: No Security Violation 0 security violation was detected."]
    SV0_0 = 0,
    #[doc = "1: Security Violation 0 security violation was detected."]
    SV0_1 = 1,
}
impl From<SV0_A> for bool {
    #[inline(always)]
    fn from(variant: SV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV0`"]
pub type SV0_R = crate::R<bool, SV0_A>;
impl SV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV0_A {
        match self.bits {
            false => SV0_A::SV0_0,
            true => SV0_A::SV0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV0_0`"]
    #[inline(always)]
    pub fn is_sv0_0(&self) -> bool {
        *self == SV0_A::SV0_0
    }
    #[doc = "Checks if the value of the field is `SV0_1`"]
    #[inline(always)]
    pub fn is_sv0_1(&self) -> bool {
        *self == SV0_A::SV0_1
    }
}
#[doc = "Write proxy for field `SV0`"]
pub struct SV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub fn sv0_0(self) -> &'a mut W {
        self.variant(SV0_A::SV0_0)
    }
    #[doc = "Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub fn sv0_1(self) -> &'a mut W {
        self.variant(SV0_A::SV0_1)
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
#[doc = "Security Violation 1 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV1_A {
    #[doc = "0: No Security Violation 1 security violation was detected."]
    SV1_0 = 0,
    #[doc = "1: Security Violation 1 security violation was detected."]
    SV1_1 = 1,
}
impl From<SV1_A> for bool {
    #[inline(always)]
    fn from(variant: SV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV1`"]
pub type SV1_R = crate::R<bool, SV1_A>;
impl SV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV1_A {
        match self.bits {
            false => SV1_A::SV1_0,
            true => SV1_A::SV1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV1_0`"]
    #[inline(always)]
    pub fn is_sv1_0(&self) -> bool {
        *self == SV1_A::SV1_0
    }
    #[doc = "Checks if the value of the field is `SV1_1`"]
    #[inline(always)]
    pub fn is_sv1_1(&self) -> bool {
        *self == SV1_A::SV1_1
    }
}
#[doc = "Write proxy for field `SV1`"]
pub struct SV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub fn sv1_0(self) -> &'a mut W {
        self.variant(SV1_A::SV1_0)
    }
    #[doc = "Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub fn sv1_1(self) -> &'a mut W {
        self.variant(SV1_A::SV1_1)
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
#[doc = "Security Violation 2 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV2_A {
    #[doc = "0: No Security Violation 2 security violation was detected."]
    SV2_0 = 0,
    #[doc = "1: Security Violation 2 security violation was detected."]
    SV2_1 = 1,
}
impl From<SV2_A> for bool {
    #[inline(always)]
    fn from(variant: SV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV2`"]
pub type SV2_R = crate::R<bool, SV2_A>;
impl SV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV2_A {
        match self.bits {
            false => SV2_A::SV2_0,
            true => SV2_A::SV2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV2_0`"]
    #[inline(always)]
    pub fn is_sv2_0(&self) -> bool {
        *self == SV2_A::SV2_0
    }
    #[doc = "Checks if the value of the field is `SV2_1`"]
    #[inline(always)]
    pub fn is_sv2_1(&self) -> bool {
        *self == SV2_A::SV2_1
    }
}
#[doc = "Write proxy for field `SV2`"]
pub struct SV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub fn sv2_0(self) -> &'a mut W {
        self.variant(SV2_A::SV2_0)
    }
    #[doc = "Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub fn sv2_1(self) -> &'a mut W {
        self.variant(SV2_A::SV2_1)
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
#[doc = "Security Violation 3 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV3_A {
    #[doc = "0: No Security Violation 3 security violation was detected."]
    SV3_0 = 0,
    #[doc = "1: Security Violation 3 security violation was detected."]
    SV3_1 = 1,
}
impl From<SV3_A> for bool {
    #[inline(always)]
    fn from(variant: SV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV3`"]
pub type SV3_R = crate::R<bool, SV3_A>;
impl SV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV3_A {
        match self.bits {
            false => SV3_A::SV3_0,
            true => SV3_A::SV3_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV3_0`"]
    #[inline(always)]
    pub fn is_sv3_0(&self) -> bool {
        *self == SV3_A::SV3_0
    }
    #[doc = "Checks if the value of the field is `SV3_1`"]
    #[inline(always)]
    pub fn is_sv3_1(&self) -> bool {
        *self == SV3_A::SV3_1
    }
}
#[doc = "Write proxy for field `SV3`"]
pub struct SV3_W<'a> {
    w: &'a mut W,
}
impl<'a> SV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub fn sv3_0(self) -> &'a mut W {
        self.variant(SV3_A::SV3_0)
    }
    #[doc = "Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub fn sv3_1(self) -> &'a mut W {
        self.variant(SV3_A::SV3_1)
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
#[doc = "Security Violation 4 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV4_A {
    #[doc = "0: No Security Violation 4 security violation was detected."]
    SV4_0 = 0,
    #[doc = "1: Security Violation 4 security violation was detected."]
    SV4_1 = 1,
}
impl From<SV4_A> for bool {
    #[inline(always)]
    fn from(variant: SV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV4`"]
pub type SV4_R = crate::R<bool, SV4_A>;
impl SV4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV4_A {
        match self.bits {
            false => SV4_A::SV4_0,
            true => SV4_A::SV4_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV4_0`"]
    #[inline(always)]
    pub fn is_sv4_0(&self) -> bool {
        *self == SV4_A::SV4_0
    }
    #[doc = "Checks if the value of the field is `SV4_1`"]
    #[inline(always)]
    pub fn is_sv4_1(&self) -> bool {
        *self == SV4_A::SV4_1
    }
}
#[doc = "Write proxy for field `SV4`"]
pub struct SV4_W<'a> {
    w: &'a mut W,
}
impl<'a> SV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub fn sv4_0(self) -> &'a mut W {
        self.variant(SV4_A::SV4_0)
    }
    #[doc = "Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub fn sv4_1(self) -> &'a mut W {
        self.variant(SV4_A::SV4_1)
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
#[doc = "Security Violation 5 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV5_A {
    #[doc = "0: No Security Violation 5 security violation was detected."]
    SV5_0 = 0,
    #[doc = "1: Security Violation 5 security violation was detected."]
    SV5_1 = 1,
}
impl From<SV5_A> for bool {
    #[inline(always)]
    fn from(variant: SV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV5`"]
pub type SV5_R = crate::R<bool, SV5_A>;
impl SV5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV5_A {
        match self.bits {
            false => SV5_A::SV5_0,
            true => SV5_A::SV5_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV5_0`"]
    #[inline(always)]
    pub fn is_sv5_0(&self) -> bool {
        *self == SV5_A::SV5_0
    }
    #[doc = "Checks if the value of the field is `SV5_1`"]
    #[inline(always)]
    pub fn is_sv5_1(&self) -> bool {
        *self == SV5_A::SV5_1
    }
}
#[doc = "Write proxy for field `SV5`"]
pub struct SV5_W<'a> {
    w: &'a mut W,
}
impl<'a> SV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub fn sv5_0(self) -> &'a mut W {
        self.variant(SV5_A::SV5_0)
    }
    #[doc = "Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub fn sv5_1(self) -> &'a mut W {
        self.variant(SV5_A::SV5_1)
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
#[doc = "Reader of field `SW_SV`"]
pub type SW_SV_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_FSV`"]
pub type SW_FSV_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_LPSV`"]
pub type SW_LPSV_R = crate::R<bool, bool>;
#[doc = "Reader of field `ZMK_SYNDROME`"]
pub type ZMK_SYNDROME_R = crate::R<u16, u16>;
#[doc = "Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_ECC_FAIL_A {
    #[doc = "0: ZMK ECC Failure was not detected."]
    ZMK_ECC_FAIL_0 = 0,
    #[doc = "1: ZMK ECC Failure was detected."]
    ZMK_ECC_FAIL_1 = 1,
}
impl From<ZMK_ECC_FAIL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_ECC_FAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_ECC_FAIL`"]
pub type ZMK_ECC_FAIL_R = crate::R<bool, ZMK_ECC_FAIL_A>;
impl ZMK_ECC_FAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_ECC_FAIL_A {
        match self.bits {
            false => ZMK_ECC_FAIL_A::ZMK_ECC_FAIL_0,
            true => ZMK_ECC_FAIL_A::ZMK_ECC_FAIL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_ECC_FAIL_0`"]
    #[inline(always)]
    pub fn is_zmk_ecc_fail_0(&self) -> bool {
        *self == ZMK_ECC_FAIL_A::ZMK_ECC_FAIL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_ECC_FAIL_1`"]
    #[inline(always)]
    pub fn is_zmk_ecc_fail_1(&self) -> bool {
        *self == ZMK_ECC_FAIL_A::ZMK_ECC_FAIL_1
    }
}
#[doc = "Write proxy for field `ZMK_ECC_FAIL`"]
pub struct ZMK_ECC_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_ECC_FAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZMK_ECC_FAIL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ZMK ECC Failure was not detected."]
    #[inline(always)]
    pub fn zmk_ecc_fail_0(self) -> &'a mut W {
        self.variant(ZMK_ECC_FAIL_A::ZMK_ECC_FAIL_0)
    }
    #[doc = "ZMK ECC Failure was detected."]
    #[inline(always)]
    pub fn zmk_ecc_fail_1(self) -> &'a mut W {
        self.variant(ZMK_ECC_FAIL_A::ZMK_ECC_FAIL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `LP_SEC_VIO`"]
pub type LP_SEC_VIO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub fn sv0(&self) -> SV0_R {
        SV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub fn sv1(&self) -> SV1_R {
        SV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub fn sv2(&self) -> SV2_R {
        SV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub fn sv3(&self) -> SV3_R {
        SV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub fn sv4(&self) -> SV4_R {
        SV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub fn sv5(&self) -> SV5_R {
        SV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
    #[inline(always)]
    pub fn sw_sv(&self) -> SW_SV_R {
        SW_SV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
    #[inline(always)]
    pub fn sw_fsv(&self) -> SW_FSV_R {
        SW_FSV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
    #[inline(always)]
    pub fn sw_lpsv(&self) -> SW_LPSV_R {
        SW_LPSV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
    #[inline(always)]
    pub fn zmk_syndrome(&self) -> ZMK_SYNDROME_R {
        ZMK_SYNDROME_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline(always)]
    pub fn zmk_ecc_fail(&self) -> ZMK_ECC_FAIL_R {
        ZMK_ECC_FAIL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LP Security Violation A security volation was detected in the SNVS low power section."]
    #[inline(always)]
    pub fn lp_sec_vio(&self) -> LP_SEC_VIO_R {
        LP_SEC_VIO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub fn sv0(&mut self) -> SV0_W {
        SV0_W { w: self }
    }
    #[doc = "Bit 1 - Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub fn sv1(&mut self) -> SV1_W {
        SV1_W { w: self }
    }
    #[doc = "Bit 2 - Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub fn sv2(&mut self) -> SV2_W {
        SV2_W { w: self }
    }
    #[doc = "Bit 3 - Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub fn sv3(&mut self) -> SV3_W {
        SV3_W { w: self }
    }
    #[doc = "Bit 4 - Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub fn sv4(&mut self) -> SV4_W {
        SV4_W { w: self }
    }
    #[doc = "Bit 5 - Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub fn sv5(&mut self) -> SV5_W {
        SV5_W { w: self }
    }
    #[doc = "Bit 27 - Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline(always)]
    pub fn zmk_ecc_fail(&mut self) -> ZMK_ECC_FAIL_W {
        ZMK_ECC_FAIL_W { w: self }
    }
}
