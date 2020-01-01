#[doc = "Reader of register HPSICR"]
pub type R = crate::R<u32, super::HPSICR>;
#[doc = "Writer for register HPSICR"]
pub type W = crate::W<u32, super::HPSICR>;
#[doc = "Register HPSICR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPSICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV0_EN_A {
    #[doc = "0: Security Violation 0 Interrupt is Disabled"]
    SV0_EN_0 = 0,
    #[doc = "1: Security Violation 0 Interrupt is Enabled"]
    SV0_EN_1 = 1,
}
impl From<SV0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV0_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV0_EN`"]
pub type SV0_EN_R = crate::R<bool, SV0_EN_A>;
impl SV0_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV0_EN_A {
        match self.bits {
            false => SV0_EN_A::SV0_EN_0,
            true => SV0_EN_A::SV0_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV0_EN_0`"]
    #[inline(always)]
    pub fn is_sv0_en_0(&self) -> bool {
        *self == SV0_EN_A::SV0_EN_0
    }
    #[doc = "Checks if the value of the field is `SV0_EN_1`"]
    #[inline(always)]
    pub fn is_sv0_en_1(&self) -> bool {
        *self == SV0_EN_A::SV0_EN_1
    }
}
#[doc = "Write proxy for field `SV0_EN`"]
pub struct SV0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SV0_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV0_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 0 Interrupt is Disabled"]
    #[inline(always)]
    pub fn sv0_en_0(self) -> &'a mut W {
        self.variant(SV0_EN_A::SV0_EN_0)
    }
    #[doc = "Security Violation 0 Interrupt is Enabled"]
    #[inline(always)]
    pub fn sv0_en_1(self) -> &'a mut W {
        self.variant(SV0_EN_A::SV0_EN_1)
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
#[doc = "Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV1_EN_A {
    #[doc = "0: Security Violation 1 Interrupt is Disabled"]
    SV1_EN_0 = 0,
    #[doc = "1: Security Violation 1 Interrupt is Enabled"]
    SV1_EN_1 = 1,
}
impl From<SV1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV1_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV1_EN`"]
pub type SV1_EN_R = crate::R<bool, SV1_EN_A>;
impl SV1_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV1_EN_A {
        match self.bits {
            false => SV1_EN_A::SV1_EN_0,
            true => SV1_EN_A::SV1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV1_EN_0`"]
    #[inline(always)]
    pub fn is_sv1_en_0(&self) -> bool {
        *self == SV1_EN_A::SV1_EN_0
    }
    #[doc = "Checks if the value of the field is `SV1_EN_1`"]
    #[inline(always)]
    pub fn is_sv1_en_1(&self) -> bool {
        *self == SV1_EN_A::SV1_EN_1
    }
}
#[doc = "Write proxy for field `SV1_EN`"]
pub struct SV1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SV1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV1_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 1 Interrupt is Disabled"]
    #[inline(always)]
    pub fn sv1_en_0(self) -> &'a mut W {
        self.variant(SV1_EN_A::SV1_EN_0)
    }
    #[doc = "Security Violation 1 Interrupt is Enabled"]
    #[inline(always)]
    pub fn sv1_en_1(self) -> &'a mut W {
        self.variant(SV1_EN_A::SV1_EN_1)
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
#[doc = "Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV2_EN_A {
    #[doc = "0: Security Violation 2 Interrupt is Disabled"]
    SV2_EN_0 = 0,
    #[doc = "1: Security Violation 2 Interrupt is Enabled"]
    SV2_EN_1 = 1,
}
impl From<SV2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV2_EN`"]
pub type SV2_EN_R = crate::R<bool, SV2_EN_A>;
impl SV2_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV2_EN_A {
        match self.bits {
            false => SV2_EN_A::SV2_EN_0,
            true => SV2_EN_A::SV2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV2_EN_0`"]
    #[inline(always)]
    pub fn is_sv2_en_0(&self) -> bool {
        *self == SV2_EN_A::SV2_EN_0
    }
    #[doc = "Checks if the value of the field is `SV2_EN_1`"]
    #[inline(always)]
    pub fn is_sv2_en_1(&self) -> bool {
        *self == SV2_EN_A::SV2_EN_1
    }
}
#[doc = "Write proxy for field `SV2_EN`"]
pub struct SV2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SV2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV2_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 2 Interrupt is Disabled"]
    #[inline(always)]
    pub fn sv2_en_0(self) -> &'a mut W {
        self.variant(SV2_EN_A::SV2_EN_0)
    }
    #[doc = "Security Violation 2 Interrupt is Enabled"]
    #[inline(always)]
    pub fn sv2_en_1(self) -> &'a mut W {
        self.variant(SV2_EN_A::SV2_EN_1)
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
#[doc = "Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV3_EN_A {
    #[doc = "0: Security Violation 3 Interrupt is Disabled"]
    SV3_EN_0 = 0,
    #[doc = "1: Security Violation 3 Interrupt is Enabled"]
    SV3_EN_1 = 1,
}
impl From<SV3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV3_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV3_EN`"]
pub type SV3_EN_R = crate::R<bool, SV3_EN_A>;
impl SV3_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV3_EN_A {
        match self.bits {
            false => SV3_EN_A::SV3_EN_0,
            true => SV3_EN_A::SV3_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV3_EN_0`"]
    #[inline(always)]
    pub fn is_sv3_en_0(&self) -> bool {
        *self == SV3_EN_A::SV3_EN_0
    }
    #[doc = "Checks if the value of the field is `SV3_EN_1`"]
    #[inline(always)]
    pub fn is_sv3_en_1(&self) -> bool {
        *self == SV3_EN_A::SV3_EN_1
    }
}
#[doc = "Write proxy for field `SV3_EN`"]
pub struct SV3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SV3_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV3_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 3 Interrupt is Disabled"]
    #[inline(always)]
    pub fn sv3_en_0(self) -> &'a mut W {
        self.variant(SV3_EN_A::SV3_EN_0)
    }
    #[doc = "Security Violation 3 Interrupt is Enabled"]
    #[inline(always)]
    pub fn sv3_en_1(self) -> &'a mut W {
        self.variant(SV3_EN_A::SV3_EN_1)
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
#[doc = "Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV4_EN_A {
    #[doc = "0: Security Violation 4 Interrupt is Disabled"]
    SV4_EN_0 = 0,
    #[doc = "1: Security Violation 4 Interrupt is Enabled"]
    SV4_EN_1 = 1,
}
impl From<SV4_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV4_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV4_EN`"]
pub type SV4_EN_R = crate::R<bool, SV4_EN_A>;
impl SV4_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV4_EN_A {
        match self.bits {
            false => SV4_EN_A::SV4_EN_0,
            true => SV4_EN_A::SV4_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV4_EN_0`"]
    #[inline(always)]
    pub fn is_sv4_en_0(&self) -> bool {
        *self == SV4_EN_A::SV4_EN_0
    }
    #[doc = "Checks if the value of the field is `SV4_EN_1`"]
    #[inline(always)]
    pub fn is_sv4_en_1(&self) -> bool {
        *self == SV4_EN_A::SV4_EN_1
    }
}
#[doc = "Write proxy for field `SV4_EN`"]
pub struct SV4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SV4_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV4_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 4 Interrupt is Disabled"]
    #[inline(always)]
    pub fn sv4_en_0(self) -> &'a mut W {
        self.variant(SV4_EN_A::SV4_EN_0)
    }
    #[doc = "Security Violation 4 Interrupt is Enabled"]
    #[inline(always)]
    pub fn sv4_en_1(self) -> &'a mut W {
        self.variant(SV4_EN_A::SV4_EN_1)
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
#[doc = "Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SV5_EN_A {
    #[doc = "0: Security Violation 5 Interrupt is Disabled"]
    SV5_EN_0 = 0,
    #[doc = "1: Security Violation 5 Interrupt is Enabled"]
    SV5_EN_1 = 1,
}
impl From<SV5_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV5_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SV5_EN`"]
pub type SV5_EN_R = crate::R<bool, SV5_EN_A>;
impl SV5_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV5_EN_A {
        match self.bits {
            false => SV5_EN_A::SV5_EN_0,
            true => SV5_EN_A::SV5_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SV5_EN_0`"]
    #[inline(always)]
    pub fn is_sv5_en_0(&self) -> bool {
        *self == SV5_EN_A::SV5_EN_0
    }
    #[doc = "Checks if the value of the field is `SV5_EN_1`"]
    #[inline(always)]
    pub fn is_sv5_en_1(&self) -> bool {
        *self == SV5_EN_A::SV5_EN_1
    }
}
#[doc = "Write proxy for field `SV5_EN`"]
pub struct SV5_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SV5_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SV5_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Security Violation 5 Interrupt is Disabled"]
    #[inline(always)]
    pub fn sv5_en_0(self) -> &'a mut W {
        self.variant(SV5_EN_A::SV5_EN_0)
    }
    #[doc = "Security Violation 5 Interrupt is Enabled"]
    #[inline(always)]
    pub fn sv5_en_1(self) -> &'a mut W {
        self.variant(SV5_EN_A::SV5_EN_1)
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
#[doc = "LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSVI_EN_A {
    #[doc = "0: LP Security Violation Interrupt is Disabled"]
    LPSVI_EN_0 = 0,
    #[doc = "1: LP Security Violation Interrupt is Enabled"]
    LPSVI_EN_1 = 1,
}
impl From<LPSVI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPSVI_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSVI_EN`"]
pub type LPSVI_EN_R = crate::R<bool, LPSVI_EN_A>;
impl LPSVI_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSVI_EN_A {
        match self.bits {
            false => LPSVI_EN_A::LPSVI_EN_0,
            true => LPSVI_EN_A::LPSVI_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSVI_EN_0`"]
    #[inline(always)]
    pub fn is_lpsvi_en_0(&self) -> bool {
        *self == LPSVI_EN_A::LPSVI_EN_0
    }
    #[doc = "Checks if the value of the field is `LPSVI_EN_1`"]
    #[inline(always)]
    pub fn is_lpsvi_en_1(&self) -> bool {
        *self == LPSVI_EN_A::LPSVI_EN_1
    }
}
#[doc = "Write proxy for field `LPSVI_EN`"]
pub struct LPSVI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSVI_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSVI_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LP Security Violation Interrupt is Disabled"]
    #[inline(always)]
    pub fn lpsvi_en_0(self) -> &'a mut W {
        self.variant(LPSVI_EN_A::LPSVI_EN_0)
    }
    #[doc = "LP Security Violation Interrupt is Enabled"]
    #[inline(always)]
    pub fn lpsvi_en_1(self) -> &'a mut W {
        self.variant(LPSVI_EN_A::LPSVI_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    #[inline(always)]
    pub fn sv0_en(&self) -> SV0_EN_R {
        SV0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    #[inline(always)]
    pub fn sv1_en(&self) -> SV1_EN_R {
        SV1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    #[inline(always)]
    pub fn sv2_en(&self) -> SV2_EN_R {
        SV2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    #[inline(always)]
    pub fn sv3_en(&self) -> SV3_EN_R {
        SV3_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    #[inline(always)]
    pub fn sv4_en(&self) -> SV4_EN_R {
        SV4_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    #[inline(always)]
    pub fn sv5_en(&self) -> SV5_EN_R {
        SV5_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    #[inline(always)]
    pub fn lpsvi_en(&self) -> LPSVI_EN_R {
        LPSVI_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    #[inline(always)]
    pub fn sv0_en(&mut self) -> SV0_EN_W {
        SV0_EN_W { w: self }
    }
    #[doc = "Bit 1 - Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    #[inline(always)]
    pub fn sv1_en(&mut self) -> SV1_EN_W {
        SV1_EN_W { w: self }
    }
    #[doc = "Bit 2 - Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    #[inline(always)]
    pub fn sv2_en(&mut self) -> SV2_EN_W {
        SV2_EN_W { w: self }
    }
    #[doc = "Bit 3 - Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    #[inline(always)]
    pub fn sv3_en(&mut self) -> SV3_EN_W {
        SV3_EN_W { w: self }
    }
    #[doc = "Bit 4 - Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    #[inline(always)]
    pub fn sv4_en(&mut self) -> SV4_EN_W {
        SV4_EN_W { w: self }
    }
    #[doc = "Bit 5 - Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    #[inline(always)]
    pub fn sv5_en(&mut self) -> SV5_EN_W {
        SV5_EN_W { w: self }
    }
    #[doc = "Bit 31 - LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    #[inline(always)]
    pub fn lpsvi_en(&mut self) -> LPSVI_EN_W {
        LPSVI_EN_W { w: self }
    }
}
