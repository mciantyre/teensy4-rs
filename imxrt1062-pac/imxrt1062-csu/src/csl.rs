#[doc = "Reader of register CSL%s"]
pub type R = crate::R<u32, super::CSL>;
#[doc = "Writer for register CSL%s"]
pub type W = crate::W<u32, super::CSL>;
#[doc = "Register CSL%s `reset()`'s with value 0x0033_0033"]
impl crate::ResetValue for super::CSL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0033_0033
    }
}
#[doc = "Secure user read access control for the second slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUR_S2_A {
    #[doc = "0: The secure user read access is disabled for the second slave."]
    SUR_S2_0 = 0,
    #[doc = "1: The secure user read access is enabled for the second slave."]
    SUR_S2_1 = 1,
}
impl From<SUR_S2_A> for bool {
    #[inline(always)]
    fn from(variant: SUR_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUR_S2`"]
pub type SUR_S2_R = crate::R<bool, SUR_S2_A>;
impl SUR_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUR_S2_A {
        match self.bits {
            false => SUR_S2_A::SUR_S2_0,
            true => SUR_S2_A::SUR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUR_S2_0`"]
    #[inline(always)]
    pub fn is_sur_s2_0(&self) -> bool {
        *self == SUR_S2_A::SUR_S2_0
    }
    #[doc = "Checks if the value of the field is `SUR_S2_1`"]
    #[inline(always)]
    pub fn is_sur_s2_1(&self) -> bool {
        *self == SUR_S2_A::SUR_S2_1
    }
}
#[doc = "Write proxy for field `SUR_S2`"]
pub struct SUR_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> SUR_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUR_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The secure user read access is disabled for the second slave."]
    #[inline(always)]
    pub fn sur_s2_0(self) -> &'a mut W {
        self.variant(SUR_S2_A::SUR_S2_0)
    }
    #[doc = "The secure user read access is enabled for the second slave."]
    #[inline(always)]
    pub fn sur_s2_1(self) -> &'a mut W {
        self.variant(SUR_S2_A::SUR_S2_1)
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
#[doc = "Secure supervisor read access control for the second slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSR_S2_A {
    #[doc = "0: The secure supervisor read access is disabled for the second slave."]
    SSR_S2_0 = 0,
    #[doc = "1: The secure supervisor read access is enabled for the second slave."]
    SSR_S2_1 = 1,
}
impl From<SSR_S2_A> for bool {
    #[inline(always)]
    fn from(variant: SSR_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSR_S2`"]
pub type SSR_S2_R = crate::R<bool, SSR_S2_A>;
impl SSR_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSR_S2_A {
        match self.bits {
            false => SSR_S2_A::SSR_S2_0,
            true => SSR_S2_A::SSR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSR_S2_0`"]
    #[inline(always)]
    pub fn is_ssr_s2_0(&self) -> bool {
        *self == SSR_S2_A::SSR_S2_0
    }
    #[doc = "Checks if the value of the field is `SSR_S2_1`"]
    #[inline(always)]
    pub fn is_ssr_s2_1(&self) -> bool {
        *self == SSR_S2_A::SSR_S2_1
    }
}
#[doc = "Write proxy for field `SSR_S2`"]
pub struct SSR_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> SSR_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSR_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The secure supervisor read access is disabled for the second slave."]
    #[inline(always)]
    pub fn ssr_s2_0(self) -> &'a mut W {
        self.variant(SSR_S2_A::SSR_S2_0)
    }
    #[doc = "The secure supervisor read access is enabled for the second slave."]
    #[inline(always)]
    pub fn ssr_s2_1(self) -> &'a mut W {
        self.variant(SSR_S2_A::SSR_S2_1)
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
#[doc = "Non-secure user read access control for the second slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUR_S2_A {
    #[doc = "0: The non-secure user read access is disabled for the second slave."]
    NUR_S2_0 = 0,
    #[doc = "1: The non-secure user read access is enabled for the second slave."]
    NUR_S2_1 = 1,
}
impl From<NUR_S2_A> for bool {
    #[inline(always)]
    fn from(variant: NUR_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NUR_S2`"]
pub type NUR_S2_R = crate::R<bool, NUR_S2_A>;
impl NUR_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUR_S2_A {
        match self.bits {
            false => NUR_S2_A::NUR_S2_0,
            true => NUR_S2_A::NUR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUR_S2_0`"]
    #[inline(always)]
    pub fn is_nur_s2_0(&self) -> bool {
        *self == NUR_S2_A::NUR_S2_0
    }
    #[doc = "Checks if the value of the field is `NUR_S2_1`"]
    #[inline(always)]
    pub fn is_nur_s2_1(&self) -> bool {
        *self == NUR_S2_A::NUR_S2_1
    }
}
#[doc = "Write proxy for field `NUR_S2`"]
pub struct NUR_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> NUR_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NUR_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The non-secure user read access is disabled for the second slave."]
    #[inline(always)]
    pub fn nur_s2_0(self) -> &'a mut W {
        self.variant(NUR_S2_A::NUR_S2_0)
    }
    #[doc = "The non-secure user read access is enabled for the second slave."]
    #[inline(always)]
    pub fn nur_s2_1(self) -> &'a mut W {
        self.variant(NUR_S2_A::NUR_S2_1)
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
#[doc = "Non-secure supervisor read access control for the second slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSR_S2_A {
    #[doc = "0: The non-secure supervisor read access is disabled for the second slave."]
    NSR_S2_0 = 0,
    #[doc = "1: The non-secure supervisor read access is enabled for the second slave."]
    NSR_S2_1 = 1,
}
impl From<NSR_S2_A> for bool {
    #[inline(always)]
    fn from(variant: NSR_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSR_S2`"]
pub type NSR_S2_R = crate::R<bool, NSR_S2_A>;
impl NSR_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSR_S2_A {
        match self.bits {
            false => NSR_S2_A::NSR_S2_0,
            true => NSR_S2_A::NSR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSR_S2_0`"]
    #[inline(always)]
    pub fn is_nsr_s2_0(&self) -> bool {
        *self == NSR_S2_A::NSR_S2_0
    }
    #[doc = "Checks if the value of the field is `NSR_S2_1`"]
    #[inline(always)]
    pub fn is_nsr_s2_1(&self) -> bool {
        *self == NSR_S2_A::NSR_S2_1
    }
}
#[doc = "Write proxy for field `NSR_S2`"]
pub struct NSR_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> NSR_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSR_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The non-secure supervisor read access is disabled for the second slave."]
    #[inline(always)]
    pub fn nsr_s2_0(self) -> &'a mut W {
        self.variant(NSR_S2_A::NSR_S2_0)
    }
    #[doc = "The non-secure supervisor read access is enabled for the second slave."]
    #[inline(always)]
    pub fn nsr_s2_1(self) -> &'a mut W {
        self.variant(NSR_S2_A::NSR_S2_1)
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
#[doc = "Secure user write access control for the second slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUW_S2_A {
    #[doc = "0: The secure user write access is disabled for the second slave."]
    SUW_S2_0 = 0,
    #[doc = "1: The secure user write access is enabled for the second slave."]
    SUW_S2_1 = 1,
}
impl From<SUW_S2_A> for bool {
    #[inline(always)]
    fn from(variant: SUW_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUW_S2`"]
pub type SUW_S2_R = crate::R<bool, SUW_S2_A>;
impl SUW_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUW_S2_A {
        match self.bits {
            false => SUW_S2_A::SUW_S2_0,
            true => SUW_S2_A::SUW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUW_S2_0`"]
    #[inline(always)]
    pub fn is_suw_s2_0(&self) -> bool {
        *self == SUW_S2_A::SUW_S2_0
    }
    #[doc = "Checks if the value of the field is `SUW_S2_1`"]
    #[inline(always)]
    pub fn is_suw_s2_1(&self) -> bool {
        *self == SUW_S2_A::SUW_S2_1
    }
}
#[doc = "Write proxy for field `SUW_S2`"]
pub struct SUW_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> SUW_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUW_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The secure user write access is disabled for the second slave."]
    #[inline(always)]
    pub fn suw_s2_0(self) -> &'a mut W {
        self.variant(SUW_S2_A::SUW_S2_0)
    }
    #[doc = "The secure user write access is enabled for the second slave."]
    #[inline(always)]
    pub fn suw_s2_1(self) -> &'a mut W {
        self.variant(SUW_S2_A::SUW_S2_1)
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
#[doc = "Secure supervisor write access control for the second slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSW_S2_A {
    #[doc = "0: The secure supervisor write access is disabled for the second slave."]
    SSW_S2_0 = 0,
    #[doc = "1: The secure supervisor write access is enabled for the second slave."]
    SSW_S2_1 = 1,
}
impl From<SSW_S2_A> for bool {
    #[inline(always)]
    fn from(variant: SSW_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSW_S2`"]
pub type SSW_S2_R = crate::R<bool, SSW_S2_A>;
impl SSW_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSW_S2_A {
        match self.bits {
            false => SSW_S2_A::SSW_S2_0,
            true => SSW_S2_A::SSW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSW_S2_0`"]
    #[inline(always)]
    pub fn is_ssw_s2_0(&self) -> bool {
        *self == SSW_S2_A::SSW_S2_0
    }
    #[doc = "Checks if the value of the field is `SSW_S2_1`"]
    #[inline(always)]
    pub fn is_ssw_s2_1(&self) -> bool {
        *self == SSW_S2_A::SSW_S2_1
    }
}
#[doc = "Write proxy for field `SSW_S2`"]
pub struct SSW_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> SSW_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSW_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The secure supervisor write access is disabled for the second slave."]
    #[inline(always)]
    pub fn ssw_s2_0(self) -> &'a mut W {
        self.variant(SSW_S2_A::SSW_S2_0)
    }
    #[doc = "The secure supervisor write access is enabled for the second slave."]
    #[inline(always)]
    pub fn ssw_s2_1(self) -> &'a mut W {
        self.variant(SSW_S2_A::SSW_S2_1)
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
#[doc = "Non-secure user write access control for the second slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUW_S2_A {
    #[doc = "0: The non-secure user write access is disabled for the second slave."]
    NUW_S2_0 = 0,
    #[doc = "1: The non-secure user write access is enabled for the second slave."]
    NUW_S2_1 = 1,
}
impl From<NUW_S2_A> for bool {
    #[inline(always)]
    fn from(variant: NUW_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NUW_S2`"]
pub type NUW_S2_R = crate::R<bool, NUW_S2_A>;
impl NUW_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUW_S2_A {
        match self.bits {
            false => NUW_S2_A::NUW_S2_0,
            true => NUW_S2_A::NUW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUW_S2_0`"]
    #[inline(always)]
    pub fn is_nuw_s2_0(&self) -> bool {
        *self == NUW_S2_A::NUW_S2_0
    }
    #[doc = "Checks if the value of the field is `NUW_S2_1`"]
    #[inline(always)]
    pub fn is_nuw_s2_1(&self) -> bool {
        *self == NUW_S2_A::NUW_S2_1
    }
}
#[doc = "Write proxy for field `NUW_S2`"]
pub struct NUW_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> NUW_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NUW_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The non-secure user write access is disabled for the second slave."]
    #[inline(always)]
    pub fn nuw_s2_0(self) -> &'a mut W {
        self.variant(NUW_S2_A::NUW_S2_0)
    }
    #[doc = "The non-secure user write access is enabled for the second slave."]
    #[inline(always)]
    pub fn nuw_s2_1(self) -> &'a mut W {
        self.variant(NUW_S2_A::NUW_S2_1)
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
#[doc = "Non-secure supervisor write access control for the second slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSW_S2_A {
    #[doc = "0: The non-secure supervisor write access is disabled for the second slave."]
    NSW_S2_0 = 0,
    #[doc = "1: The non-secure supervisor write access is enabled for the second slave."]
    NSW_S2_1 = 1,
}
impl From<NSW_S2_A> for bool {
    #[inline(always)]
    fn from(variant: NSW_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSW_S2`"]
pub type NSW_S2_R = crate::R<bool, NSW_S2_A>;
impl NSW_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSW_S2_A {
        match self.bits {
            false => NSW_S2_A::NSW_S2_0,
            true => NSW_S2_A::NSW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSW_S2_0`"]
    #[inline(always)]
    pub fn is_nsw_s2_0(&self) -> bool {
        *self == NSW_S2_A::NSW_S2_0
    }
    #[doc = "Checks if the value of the field is `NSW_S2_1`"]
    #[inline(always)]
    pub fn is_nsw_s2_1(&self) -> bool {
        *self == NSW_S2_A::NSW_S2_1
    }
}
#[doc = "Write proxy for field `NSW_S2`"]
pub struct NSW_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> NSW_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSW_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The non-secure supervisor write access is disabled for the second slave."]
    #[inline(always)]
    pub fn nsw_s2_0(self) -> &'a mut W {
        self.variant(NSW_S2_A::NSW_S2_0)
    }
    #[doc = "The non-secure supervisor write access is enabled for the second slave."]
    #[inline(always)]
    pub fn nsw_s2_1(self) -> &'a mut W {
        self.variant(NSW_S2_A::NSW_S2_1)
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
#[doc = "The lock bit corresponding to the second slave. It is written by the secure software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_S2_A {
    #[doc = "0: Not locked. Bits 7-0 can be written by the software."]
    LOCK_S2_0 = 0,
    #[doc = "1: Bits 7-0 are locked and cannot be written by the software"]
    LOCK_S2_1 = 1,
}
impl From<LOCK_S2_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_S2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK_S2`"]
pub type LOCK_S2_R = crate::R<bool, LOCK_S2_A>;
impl LOCK_S2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_S2_A {
        match self.bits {
            false => LOCK_S2_A::LOCK_S2_0,
            true => LOCK_S2_A::LOCK_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_S2_0`"]
    #[inline(always)]
    pub fn is_lock_s2_0(&self) -> bool {
        *self == LOCK_S2_A::LOCK_S2_0
    }
    #[doc = "Checks if the value of the field is `LOCK_S2_1`"]
    #[inline(always)]
    pub fn is_lock_s2_1(&self) -> bool {
        *self == LOCK_S2_A::LOCK_S2_1
    }
}
#[doc = "Write proxy for field `LOCK_S2`"]
pub struct LOCK_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_S2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_S2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not locked. Bits 7-0 can be written by the software."]
    #[inline(always)]
    pub fn lock_s2_0(self) -> &'a mut W {
        self.variant(LOCK_S2_A::LOCK_S2_0)
    }
    #[doc = "Bits 7-0 are locked and cannot be written by the software"]
    #[inline(always)]
    pub fn lock_s2_1(self) -> &'a mut W {
        self.variant(LOCK_S2_A::LOCK_S2_1)
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
#[doc = "Secure user read access control for the first slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUR_S1_A {
    #[doc = "0: The secure user read access is disabled for the first slave."]
    SUR_S1_0 = 0,
    #[doc = "1: The secure user read access is enabled for the first slave."]
    SUR_S1_1 = 1,
}
impl From<SUR_S1_A> for bool {
    #[inline(always)]
    fn from(variant: SUR_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUR_S1`"]
pub type SUR_S1_R = crate::R<bool, SUR_S1_A>;
impl SUR_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUR_S1_A {
        match self.bits {
            false => SUR_S1_A::SUR_S1_0,
            true => SUR_S1_A::SUR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUR_S1_0`"]
    #[inline(always)]
    pub fn is_sur_s1_0(&self) -> bool {
        *self == SUR_S1_A::SUR_S1_0
    }
    #[doc = "Checks if the value of the field is `SUR_S1_1`"]
    #[inline(always)]
    pub fn is_sur_s1_1(&self) -> bool {
        *self == SUR_S1_A::SUR_S1_1
    }
}
#[doc = "Write proxy for field `SUR_S1`"]
pub struct SUR_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> SUR_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUR_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The secure user read access is disabled for the first slave."]
    #[inline(always)]
    pub fn sur_s1_0(self) -> &'a mut W {
        self.variant(SUR_S1_A::SUR_S1_0)
    }
    #[doc = "The secure user read access is enabled for the first slave."]
    #[inline(always)]
    pub fn sur_s1_1(self) -> &'a mut W {
        self.variant(SUR_S1_A::SUR_S1_1)
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
#[doc = "Secure supervisor read access control for the first slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSR_S1_A {
    #[doc = "0: The secure supervisor read access is disabled for the first slave."]
    SSR_S1_0 = 0,
    #[doc = "1: The secure supervisor read access is enabled for the first slave."]
    SSR_S1_1 = 1,
}
impl From<SSR_S1_A> for bool {
    #[inline(always)]
    fn from(variant: SSR_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSR_S1`"]
pub type SSR_S1_R = crate::R<bool, SSR_S1_A>;
impl SSR_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSR_S1_A {
        match self.bits {
            false => SSR_S1_A::SSR_S1_0,
            true => SSR_S1_A::SSR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSR_S1_0`"]
    #[inline(always)]
    pub fn is_ssr_s1_0(&self) -> bool {
        *self == SSR_S1_A::SSR_S1_0
    }
    #[doc = "Checks if the value of the field is `SSR_S1_1`"]
    #[inline(always)]
    pub fn is_ssr_s1_1(&self) -> bool {
        *self == SSR_S1_A::SSR_S1_1
    }
}
#[doc = "Write proxy for field `SSR_S1`"]
pub struct SSR_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSR_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSR_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The secure supervisor read access is disabled for the first slave."]
    #[inline(always)]
    pub fn ssr_s1_0(self) -> &'a mut W {
        self.variant(SSR_S1_A::SSR_S1_0)
    }
    #[doc = "The secure supervisor read access is enabled for the first slave."]
    #[inline(always)]
    pub fn ssr_s1_1(self) -> &'a mut W {
        self.variant(SSR_S1_A::SSR_S1_1)
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
#[doc = "Non-secure user read access control for the first slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUR_S1_A {
    #[doc = "0: The non-secure user read access is disabled for the first slave."]
    NUR_S1_0 = 0,
    #[doc = "1: The non-secure user read access is enabled for the first slave."]
    NUR_S1_1 = 1,
}
impl From<NUR_S1_A> for bool {
    #[inline(always)]
    fn from(variant: NUR_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NUR_S1`"]
pub type NUR_S1_R = crate::R<bool, NUR_S1_A>;
impl NUR_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUR_S1_A {
        match self.bits {
            false => NUR_S1_A::NUR_S1_0,
            true => NUR_S1_A::NUR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUR_S1_0`"]
    #[inline(always)]
    pub fn is_nur_s1_0(&self) -> bool {
        *self == NUR_S1_A::NUR_S1_0
    }
    #[doc = "Checks if the value of the field is `NUR_S1_1`"]
    #[inline(always)]
    pub fn is_nur_s1_1(&self) -> bool {
        *self == NUR_S1_A::NUR_S1_1
    }
}
#[doc = "Write proxy for field `NUR_S1`"]
pub struct NUR_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> NUR_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NUR_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The non-secure user read access is disabled for the first slave."]
    #[inline(always)]
    pub fn nur_s1_0(self) -> &'a mut W {
        self.variant(NUR_S1_A::NUR_S1_0)
    }
    #[doc = "The non-secure user read access is enabled for the first slave."]
    #[inline(always)]
    pub fn nur_s1_1(self) -> &'a mut W {
        self.variant(NUR_S1_A::NUR_S1_1)
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
#[doc = "Non-secure supervisor read access control for the first slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSR_S1_A {
    #[doc = "0: The non-secure supervisor read access is disabled for the first slave."]
    NSR_S1_0 = 0,
    #[doc = "1: The non-secure supervisor read access is enabled for the first slave."]
    NSR_S1_1 = 1,
}
impl From<NSR_S1_A> for bool {
    #[inline(always)]
    fn from(variant: NSR_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSR_S1`"]
pub type NSR_S1_R = crate::R<bool, NSR_S1_A>;
impl NSR_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSR_S1_A {
        match self.bits {
            false => NSR_S1_A::NSR_S1_0,
            true => NSR_S1_A::NSR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSR_S1_0`"]
    #[inline(always)]
    pub fn is_nsr_s1_0(&self) -> bool {
        *self == NSR_S1_A::NSR_S1_0
    }
    #[doc = "Checks if the value of the field is `NSR_S1_1`"]
    #[inline(always)]
    pub fn is_nsr_s1_1(&self) -> bool {
        *self == NSR_S1_A::NSR_S1_1
    }
}
#[doc = "Write proxy for field `NSR_S1`"]
pub struct NSR_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> NSR_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSR_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The non-secure supervisor read access is disabled for the first slave."]
    #[inline(always)]
    pub fn nsr_s1_0(self) -> &'a mut W {
        self.variant(NSR_S1_A::NSR_S1_0)
    }
    #[doc = "The non-secure supervisor read access is enabled for the first slave."]
    #[inline(always)]
    pub fn nsr_s1_1(self) -> &'a mut W {
        self.variant(NSR_S1_A::NSR_S1_1)
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
#[doc = "Secure user write access control for the first slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUW_S1_A {
    #[doc = "0: The secure user write access is disabled for the first slave."]
    SUW_S1_0 = 0,
    #[doc = "1: The secure user write access is enabled for the first slave."]
    SUW_S1_1 = 1,
}
impl From<SUW_S1_A> for bool {
    #[inline(always)]
    fn from(variant: SUW_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUW_S1`"]
pub type SUW_S1_R = crate::R<bool, SUW_S1_A>;
impl SUW_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUW_S1_A {
        match self.bits {
            false => SUW_S1_A::SUW_S1_0,
            true => SUW_S1_A::SUW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUW_S1_0`"]
    #[inline(always)]
    pub fn is_suw_s1_0(&self) -> bool {
        *self == SUW_S1_A::SUW_S1_0
    }
    #[doc = "Checks if the value of the field is `SUW_S1_1`"]
    #[inline(always)]
    pub fn is_suw_s1_1(&self) -> bool {
        *self == SUW_S1_A::SUW_S1_1
    }
}
#[doc = "Write proxy for field `SUW_S1`"]
pub struct SUW_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> SUW_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUW_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The secure user write access is disabled for the first slave."]
    #[inline(always)]
    pub fn suw_s1_0(self) -> &'a mut W {
        self.variant(SUW_S1_A::SUW_S1_0)
    }
    #[doc = "The secure user write access is enabled for the first slave."]
    #[inline(always)]
    pub fn suw_s1_1(self) -> &'a mut W {
        self.variant(SUW_S1_A::SUW_S1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Secure supervisor write access control for the first slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSW_S1_A {
    #[doc = "0: The secure supervisor write access is disabled for the first slave."]
    SSW_S1_0 = 0,
    #[doc = "1: The secure supervisor write access is enabled for the first slave."]
    SSW_S1_1 = 1,
}
impl From<SSW_S1_A> for bool {
    #[inline(always)]
    fn from(variant: SSW_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSW_S1`"]
pub type SSW_S1_R = crate::R<bool, SSW_S1_A>;
impl SSW_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSW_S1_A {
        match self.bits {
            false => SSW_S1_A::SSW_S1_0,
            true => SSW_S1_A::SSW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSW_S1_0`"]
    #[inline(always)]
    pub fn is_ssw_s1_0(&self) -> bool {
        *self == SSW_S1_A::SSW_S1_0
    }
    #[doc = "Checks if the value of the field is `SSW_S1_1`"]
    #[inline(always)]
    pub fn is_ssw_s1_1(&self) -> bool {
        *self == SSW_S1_A::SSW_S1_1
    }
}
#[doc = "Write proxy for field `SSW_S1`"]
pub struct SSW_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSW_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSW_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The secure supervisor write access is disabled for the first slave."]
    #[inline(always)]
    pub fn ssw_s1_0(self) -> &'a mut W {
        self.variant(SSW_S1_A::SSW_S1_0)
    }
    #[doc = "The secure supervisor write access is enabled for the first slave."]
    #[inline(always)]
    pub fn ssw_s1_1(self) -> &'a mut W {
        self.variant(SSW_S1_A::SSW_S1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Non-secure user write access control for the first slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUW_S1_A {
    #[doc = "0: The non-secure user write access is disabled for the first slave."]
    NUW_S1_0 = 0,
    #[doc = "1: The non-secure user write access is enabled for the first slave."]
    NUW_S1_1 = 1,
}
impl From<NUW_S1_A> for bool {
    #[inline(always)]
    fn from(variant: NUW_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NUW_S1`"]
pub type NUW_S1_R = crate::R<bool, NUW_S1_A>;
impl NUW_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUW_S1_A {
        match self.bits {
            false => NUW_S1_A::NUW_S1_0,
            true => NUW_S1_A::NUW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUW_S1_0`"]
    #[inline(always)]
    pub fn is_nuw_s1_0(&self) -> bool {
        *self == NUW_S1_A::NUW_S1_0
    }
    #[doc = "Checks if the value of the field is `NUW_S1_1`"]
    #[inline(always)]
    pub fn is_nuw_s1_1(&self) -> bool {
        *self == NUW_S1_A::NUW_S1_1
    }
}
#[doc = "Write proxy for field `NUW_S1`"]
pub struct NUW_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> NUW_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NUW_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The non-secure user write access is disabled for the first slave."]
    #[inline(always)]
    pub fn nuw_s1_0(self) -> &'a mut W {
        self.variant(NUW_S1_A::NUW_S1_0)
    }
    #[doc = "The non-secure user write access is enabled for the first slave."]
    #[inline(always)]
    pub fn nuw_s1_1(self) -> &'a mut W {
        self.variant(NUW_S1_A::NUW_S1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Non-secure supervisor write access control for the first slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSW_S1_A {
    #[doc = "0: The non-secure supervisor write access is disabled for the first slave."]
    NSW_S1_0 = 0,
    #[doc = "1: The non-secure supervisor write access is enabled for the first slave"]
    NSW_S1_1 = 1,
}
impl From<NSW_S1_A> for bool {
    #[inline(always)]
    fn from(variant: NSW_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSW_S1`"]
pub type NSW_S1_R = crate::R<bool, NSW_S1_A>;
impl NSW_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSW_S1_A {
        match self.bits {
            false => NSW_S1_A::NSW_S1_0,
            true => NSW_S1_A::NSW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSW_S1_0`"]
    #[inline(always)]
    pub fn is_nsw_s1_0(&self) -> bool {
        *self == NSW_S1_A::NSW_S1_0
    }
    #[doc = "Checks if the value of the field is `NSW_S1_1`"]
    #[inline(always)]
    pub fn is_nsw_s1_1(&self) -> bool {
        *self == NSW_S1_A::NSW_S1_1
    }
}
#[doc = "Write proxy for field `NSW_S1`"]
pub struct NSW_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> NSW_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSW_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The non-secure supervisor write access is disabled for the first slave."]
    #[inline(always)]
    pub fn nsw_s1_0(self) -> &'a mut W {
        self.variant(NSW_S1_A::NSW_S1_0)
    }
    #[doc = "The non-secure supervisor write access is enabled for the first slave"]
    #[inline(always)]
    pub fn nsw_s1_1(self) -> &'a mut W {
        self.variant(NSW_S1_A::NSW_S1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "The lock bit corresponding to the first slave. It is written by the secure software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_S1_A {
    #[doc = "0: Not locked. The bits 16-23 can be written by the software."]
    LOCK_S1_0 = 0,
    #[doc = "1: The bits 16-23 are locked and can't be written by the software."]
    LOCK_S1_1 = 1,
}
impl From<LOCK_S1_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_S1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK_S1`"]
pub type LOCK_S1_R = crate::R<bool, LOCK_S1_A>;
impl LOCK_S1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_S1_A {
        match self.bits {
            false => LOCK_S1_A::LOCK_S1_0,
            true => LOCK_S1_A::LOCK_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_S1_0`"]
    #[inline(always)]
    pub fn is_lock_s1_0(&self) -> bool {
        *self == LOCK_S1_A::LOCK_S1_0
    }
    #[doc = "Checks if the value of the field is `LOCK_S1_1`"]
    #[inline(always)]
    pub fn is_lock_s1_1(&self) -> bool {
        *self == LOCK_S1_A::LOCK_S1_1
    }
}
#[doc = "Write proxy for field `LOCK_S1`"]
pub struct LOCK_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_S1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_S1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not locked. The bits 16-23 can be written by the software."]
    #[inline(always)]
    pub fn lock_s1_0(self) -> &'a mut W {
        self.variant(LOCK_S1_A::LOCK_S1_0)
    }
    #[doc = "The bits 16-23 are locked and can't be written by the software."]
    #[inline(always)]
    pub fn lock_s1_1(self) -> &'a mut W {
        self.variant(LOCK_S1_A::LOCK_S1_1)
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
impl R {
    #[doc = "Bit 0 - Secure user read access control for the second slave"]
    #[inline(always)]
    pub fn sur_s2(&self) -> SUR_S2_R {
        SUR_S2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub fn ssr_s2(&self) -> SSR_S2_R {
        SSR_S2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Non-secure user read access control for the second slave"]
    #[inline(always)]
    pub fn nur_s2(&self) -> NUR_S2_R {
        NUR_S2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non-secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub fn nsr_s2(&self) -> NSR_S2_R {
        NSR_S2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Secure user write access control for the second slave"]
    #[inline(always)]
    pub fn suw_s2(&self) -> SUW_S2_R {
        SUW_S2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub fn ssw_s2(&self) -> SSW_S2_R {
        SSW_S2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Non-secure user write access control for the second slave"]
    #[inline(always)]
    pub fn nuw_s2(&self) -> NUW_S2_R {
        NUW_S2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Non-secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub fn nsw_s2(&self) -> NSW_S2_R {
        NSW_S2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline(always)]
    pub fn lock_s2(&self) -> LOCK_S2_R {
        LOCK_S2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Secure user read access control for the first slave"]
    #[inline(always)]
    pub fn sur_s1(&self) -> SUR_S1_R {
        SUR_S1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub fn ssr_s1(&self) -> SSR_S1_R {
        SSR_S1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Non-secure user read access control for the first slave"]
    #[inline(always)]
    pub fn nur_s1(&self) -> NUR_S1_R {
        NUR_S1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Non-secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub fn nsr_s1(&self) -> NSR_S1_R {
        NSR_S1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Secure user write access control for the first slave"]
    #[inline(always)]
    pub fn suw_s1(&self) -> SUW_S1_R {
        SUW_S1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub fn ssw_s1(&self) -> SSW_S1_R {
        SSW_S1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Non-secure user write access control for the first slave"]
    #[inline(always)]
    pub fn nuw_s1(&self) -> NUW_S1_R {
        NUW_S1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Non-secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub fn nsw_s1(&self) -> NSW_S1_R {
        NSW_S1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline(always)]
    pub fn lock_s1(&self) -> LOCK_S1_R {
        LOCK_S1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure user read access control for the second slave"]
    #[inline(always)]
    pub fn sur_s2(&mut self) -> SUR_S2_W {
        SUR_S2_W { w: self }
    }
    #[doc = "Bit 1 - Secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub fn ssr_s2(&mut self) -> SSR_S2_W {
        SSR_S2_W { w: self }
    }
    #[doc = "Bit 2 - Non-secure user read access control for the second slave"]
    #[inline(always)]
    pub fn nur_s2(&mut self) -> NUR_S2_W {
        NUR_S2_W { w: self }
    }
    #[doc = "Bit 3 - Non-secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub fn nsr_s2(&mut self) -> NSR_S2_W {
        NSR_S2_W { w: self }
    }
    #[doc = "Bit 4 - Secure user write access control for the second slave"]
    #[inline(always)]
    pub fn suw_s2(&mut self) -> SUW_S2_W {
        SUW_S2_W { w: self }
    }
    #[doc = "Bit 5 - Secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub fn ssw_s2(&mut self) -> SSW_S2_W {
        SSW_S2_W { w: self }
    }
    #[doc = "Bit 6 - Non-secure user write access control for the second slave"]
    #[inline(always)]
    pub fn nuw_s2(&mut self) -> NUW_S2_W {
        NUW_S2_W { w: self }
    }
    #[doc = "Bit 7 - Non-secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub fn nsw_s2(&mut self) -> NSW_S2_W {
        NSW_S2_W { w: self }
    }
    #[doc = "Bit 8 - The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline(always)]
    pub fn lock_s2(&mut self) -> LOCK_S2_W {
        LOCK_S2_W { w: self }
    }
    #[doc = "Bit 16 - Secure user read access control for the first slave"]
    #[inline(always)]
    pub fn sur_s1(&mut self) -> SUR_S1_W {
        SUR_S1_W { w: self }
    }
    #[doc = "Bit 17 - Secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub fn ssr_s1(&mut self) -> SSR_S1_W {
        SSR_S1_W { w: self }
    }
    #[doc = "Bit 18 - Non-secure user read access control for the first slave"]
    #[inline(always)]
    pub fn nur_s1(&mut self) -> NUR_S1_W {
        NUR_S1_W { w: self }
    }
    #[doc = "Bit 19 - Non-secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub fn nsr_s1(&mut self) -> NSR_S1_W {
        NSR_S1_W { w: self }
    }
    #[doc = "Bit 20 - Secure user write access control for the first slave"]
    #[inline(always)]
    pub fn suw_s1(&mut self) -> SUW_S1_W {
        SUW_S1_W { w: self }
    }
    #[doc = "Bit 21 - Secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub fn ssw_s1(&mut self) -> SSW_S1_W {
        SSW_S1_W { w: self }
    }
    #[doc = "Bit 22 - Non-secure user write access control for the first slave"]
    #[inline(always)]
    pub fn nuw_s1(&mut self) -> NUW_S1_W {
        NUW_S1_W { w: self }
    }
    #[doc = "Bit 23 - Non-secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub fn nsw_s1(&mut self) -> NSW_S1_W {
        NSW_S1_W { w: self }
    }
    #[doc = "Bit 24 - The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline(always)]
    pub fn lock_s1(&mut self) -> LOCK_S1_W {
        LOCK_S1_W { w: self }
    }
}
