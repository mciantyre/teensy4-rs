#[doc = "Reader of register GPR14"]
pub type R = crate::R<u32, super::GPR14>;
#[doc = "Writer for register GPR14"]
pub type W = crate::W<u32, super::GPR14>;
#[doc = "Register GPR14 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "reduces ACMP1 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1_CMP_IGEN_TRIM_DN_A {
    #[doc = "0: no reduce"]
    ACMP1_CMP_IGEN_TRIM_DN_0,
    #[doc = "1: reduces"]
    ACMP1_CMP_IGEN_TRIM_DN_1,
}
impl From<ACMP1_CMP_IGEN_TRIM_DN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1_CMP_IGEN_TRIM_DN_A) -> Self {
        match variant {
            ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_0 => false,
            ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP1_CMP_IGEN_TRIM_DN`"]
pub type ACMP1_CMP_IGEN_TRIM_DN_R = crate::R<bool, ACMP1_CMP_IGEN_TRIM_DN_A>;
impl ACMP1_CMP_IGEN_TRIM_DN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1_CMP_IGEN_TRIM_DN_A {
        match self.bits {
            false => ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_0,
            true => ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_DN_0`"]
    #[inline(always)]
    pub fn is_acmp1_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_DN_1`"]
    #[inline(always)]
    pub fn is_acmp1_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Write proxy for field `ACMP1_CMP_IGEN_TRIM_DN`"]
pub struct ACMP1_CMP_IGEN_TRIM_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1_CMP_IGEN_TRIM_DN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP1_CMP_IGEN_TRIM_DN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no reduce"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_1)
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
#[doc = "reduces ACMP2 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP2_CMP_IGEN_TRIM_DN_A {
    #[doc = "0: no reduce"]
    ACMP2_CMP_IGEN_TRIM_DN_0,
    #[doc = "1: reduces"]
    ACMP2_CMP_IGEN_TRIM_DN_1,
}
impl From<ACMP2_CMP_IGEN_TRIM_DN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP2_CMP_IGEN_TRIM_DN_A) -> Self {
        match variant {
            ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_0 => false,
            ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP2_CMP_IGEN_TRIM_DN`"]
pub type ACMP2_CMP_IGEN_TRIM_DN_R = crate::R<bool, ACMP2_CMP_IGEN_TRIM_DN_A>;
impl ACMP2_CMP_IGEN_TRIM_DN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP2_CMP_IGEN_TRIM_DN_A {
        match self.bits {
            false => ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_0,
            true => ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_DN_0`"]
    #[inline(always)]
    pub fn is_acmp2_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_DN_1`"]
    #[inline(always)]
    pub fn is_acmp2_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Write proxy for field `ACMP2_CMP_IGEN_TRIM_DN`"]
pub struct ACMP2_CMP_IGEN_TRIM_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP2_CMP_IGEN_TRIM_DN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP2_CMP_IGEN_TRIM_DN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no reduce"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_1)
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
#[doc = "reduces ACMP3 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP3_CMP_IGEN_TRIM_DN_A {
    #[doc = "0: no reduce"]
    ACMP3_CMP_IGEN_TRIM_DN_0,
    #[doc = "1: reduces"]
    ACMP3_CMP_IGEN_TRIM_DN_1,
}
impl From<ACMP3_CMP_IGEN_TRIM_DN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP3_CMP_IGEN_TRIM_DN_A) -> Self {
        match variant {
            ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_0 => false,
            ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP3_CMP_IGEN_TRIM_DN`"]
pub type ACMP3_CMP_IGEN_TRIM_DN_R = crate::R<bool, ACMP3_CMP_IGEN_TRIM_DN_A>;
impl ACMP3_CMP_IGEN_TRIM_DN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP3_CMP_IGEN_TRIM_DN_A {
        match self.bits {
            false => ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_0,
            true => ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_DN_0`"]
    #[inline(always)]
    pub fn is_acmp3_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_DN_1`"]
    #[inline(always)]
    pub fn is_acmp3_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Write proxy for field `ACMP3_CMP_IGEN_TRIM_DN`"]
pub struct ACMP3_CMP_IGEN_TRIM_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP3_CMP_IGEN_TRIM_DN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP3_CMP_IGEN_TRIM_DN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no reduce"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_1)
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
#[doc = "reduces ACMP4 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP4_CMP_IGEN_TRIM_DN_A {
    #[doc = "0: no reduce"]
    ACMP4_CMP_IGEN_TRIM_DN_0,
    #[doc = "1: reduces"]
    ACMP4_CMP_IGEN_TRIM_DN_1,
}
impl From<ACMP4_CMP_IGEN_TRIM_DN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP4_CMP_IGEN_TRIM_DN_A) -> Self {
        match variant {
            ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_0 => false,
            ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP4_CMP_IGEN_TRIM_DN`"]
pub type ACMP4_CMP_IGEN_TRIM_DN_R = crate::R<bool, ACMP4_CMP_IGEN_TRIM_DN_A>;
impl ACMP4_CMP_IGEN_TRIM_DN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP4_CMP_IGEN_TRIM_DN_A {
        match self.bits {
            false => ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_0,
            true => ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_DN_0`"]
    #[inline(always)]
    pub fn is_acmp4_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_DN_1`"]
    #[inline(always)]
    pub fn is_acmp4_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Write proxy for field `ACMP4_CMP_IGEN_TRIM_DN`"]
pub struct ACMP4_CMP_IGEN_TRIM_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP4_CMP_IGEN_TRIM_DN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP4_CMP_IGEN_TRIM_DN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no reduce"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_1)
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
#[doc = "increases ACMP1 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1_CMP_IGEN_TRIM_UP_A {
    #[doc = "0: no increase"]
    ACMP1_CMP_IGEN_TRIM_UP_0,
    #[doc = "1: increases"]
    ACMP1_CMP_IGEN_TRIM_UP_1,
}
impl From<ACMP1_CMP_IGEN_TRIM_UP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1_CMP_IGEN_TRIM_UP_A) -> Self {
        match variant {
            ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_0 => false,
            ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP1_CMP_IGEN_TRIM_UP`"]
pub type ACMP1_CMP_IGEN_TRIM_UP_R = crate::R<bool, ACMP1_CMP_IGEN_TRIM_UP_A>;
impl ACMP1_CMP_IGEN_TRIM_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1_CMP_IGEN_TRIM_UP_A {
        match self.bits {
            false => ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_0,
            true => ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_UP_0`"]
    #[inline(always)]
    pub fn is_acmp1_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_UP_1`"]
    #[inline(always)]
    pub fn is_acmp1_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Write proxy for field `ACMP1_CMP_IGEN_TRIM_UP`"]
pub struct ACMP1_CMP_IGEN_TRIM_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1_CMP_IGEN_TRIM_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP1_CMP_IGEN_TRIM_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no increase"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_1)
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
#[doc = "increases ACMP2 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP2_CMP_IGEN_TRIM_UP_A {
    #[doc = "0: no increase"]
    ACMP2_CMP_IGEN_TRIM_UP_0,
    #[doc = "1: increases"]
    ACMP2_CMP_IGEN_TRIM_UP_1,
}
impl From<ACMP2_CMP_IGEN_TRIM_UP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP2_CMP_IGEN_TRIM_UP_A) -> Self {
        match variant {
            ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_0 => false,
            ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP2_CMP_IGEN_TRIM_UP`"]
pub type ACMP2_CMP_IGEN_TRIM_UP_R = crate::R<bool, ACMP2_CMP_IGEN_TRIM_UP_A>;
impl ACMP2_CMP_IGEN_TRIM_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP2_CMP_IGEN_TRIM_UP_A {
        match self.bits {
            false => ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_0,
            true => ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_UP_0`"]
    #[inline(always)]
    pub fn is_acmp2_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_UP_1`"]
    #[inline(always)]
    pub fn is_acmp2_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Write proxy for field `ACMP2_CMP_IGEN_TRIM_UP`"]
pub struct ACMP2_CMP_IGEN_TRIM_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP2_CMP_IGEN_TRIM_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP2_CMP_IGEN_TRIM_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no increase"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_1)
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
#[doc = "increases ACMP3 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP3_CMP_IGEN_TRIM_UP_A {
    #[doc = "0: no increase"]
    ACMP3_CMP_IGEN_TRIM_UP_0,
    #[doc = "1: increases"]
    ACMP3_CMP_IGEN_TRIM_UP_1,
}
impl From<ACMP3_CMP_IGEN_TRIM_UP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP3_CMP_IGEN_TRIM_UP_A) -> Self {
        match variant {
            ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_0 => false,
            ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP3_CMP_IGEN_TRIM_UP`"]
pub type ACMP3_CMP_IGEN_TRIM_UP_R = crate::R<bool, ACMP3_CMP_IGEN_TRIM_UP_A>;
impl ACMP3_CMP_IGEN_TRIM_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP3_CMP_IGEN_TRIM_UP_A {
        match self.bits {
            false => ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_0,
            true => ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_UP_0`"]
    #[inline(always)]
    pub fn is_acmp3_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_UP_1`"]
    #[inline(always)]
    pub fn is_acmp3_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Write proxy for field `ACMP3_CMP_IGEN_TRIM_UP`"]
pub struct ACMP3_CMP_IGEN_TRIM_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP3_CMP_IGEN_TRIM_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP3_CMP_IGEN_TRIM_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no increase"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_1)
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
#[doc = "increases ACMP4 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP4_CMP_IGEN_TRIM_UP_A {
    #[doc = "0: no increase"]
    ACMP4_CMP_IGEN_TRIM_UP_0,
    #[doc = "1: increases"]
    ACMP4_CMP_IGEN_TRIM_UP_1,
}
impl From<ACMP4_CMP_IGEN_TRIM_UP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP4_CMP_IGEN_TRIM_UP_A) -> Self {
        match variant {
            ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_0 => false,
            ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP4_CMP_IGEN_TRIM_UP`"]
pub type ACMP4_CMP_IGEN_TRIM_UP_R = crate::R<bool, ACMP4_CMP_IGEN_TRIM_UP_A>;
impl ACMP4_CMP_IGEN_TRIM_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP4_CMP_IGEN_TRIM_UP_A {
        match self.bits {
            false => ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_0,
            true => ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_UP_0`"]
    #[inline(always)]
    pub fn is_acmp4_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_UP_1`"]
    #[inline(always)]
    pub fn is_acmp4_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Write proxy for field `ACMP4_CMP_IGEN_TRIM_UP`"]
pub struct ACMP4_CMP_IGEN_TRIM_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP4_CMP_IGEN_TRIM_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP4_CMP_IGEN_TRIM_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no increase"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_1)
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
#[doc = "ACMP1 sample_lv source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1_SAMPLE_SYNC_EN_A {
    #[doc = "0: select XBAR output"]
    ACMP1_SAMPLE_SYNC_EN_0,
    #[doc = "1: select synced sample_lv"]
    ACMP1_SAMPLE_SYNC_EN_1,
}
impl From<ACMP1_SAMPLE_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1_SAMPLE_SYNC_EN_A) -> Self {
        match variant {
            ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_0 => false,
            ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP1_SAMPLE_SYNC_EN`"]
pub type ACMP1_SAMPLE_SYNC_EN_R = crate::R<bool, ACMP1_SAMPLE_SYNC_EN_A>;
impl ACMP1_SAMPLE_SYNC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1_SAMPLE_SYNC_EN_A {
        match self.bits {
            false => ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_0,
            true => ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_SAMPLE_SYNC_EN_0`"]
    #[inline(always)]
    pub fn is_acmp1_sample_sync_en_0(&self) -> bool {
        *self == ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_SAMPLE_SYNC_EN_1`"]
    #[inline(always)]
    pub fn is_acmp1_sample_sync_en_1(&self) -> bool {
        *self == ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Write proxy for field `ACMP1_SAMPLE_SYNC_EN`"]
pub struct ACMP1_SAMPLE_SYNC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1_SAMPLE_SYNC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP1_SAMPLE_SYNC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "select XBAR output"]
    #[inline(always)]
    pub fn acmp1_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline(always)]
    pub fn acmp1_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_1)
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
#[doc = "ACMP2 sample_lv source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP2_SAMPLE_SYNC_EN_A {
    #[doc = "0: select XBAR output"]
    ACMP2_SAMPLE_SYNC_EN_0,
    #[doc = "1: select synced sample_lv"]
    ACMP2_SAMPLE_SYNC_EN_1,
}
impl From<ACMP2_SAMPLE_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP2_SAMPLE_SYNC_EN_A) -> Self {
        match variant {
            ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_0 => false,
            ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP2_SAMPLE_SYNC_EN`"]
pub type ACMP2_SAMPLE_SYNC_EN_R = crate::R<bool, ACMP2_SAMPLE_SYNC_EN_A>;
impl ACMP2_SAMPLE_SYNC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP2_SAMPLE_SYNC_EN_A {
        match self.bits {
            false => ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_0,
            true => ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_SAMPLE_SYNC_EN_0`"]
    #[inline(always)]
    pub fn is_acmp2_sample_sync_en_0(&self) -> bool {
        *self == ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_SAMPLE_SYNC_EN_1`"]
    #[inline(always)]
    pub fn is_acmp2_sample_sync_en_1(&self) -> bool {
        *self == ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Write proxy for field `ACMP2_SAMPLE_SYNC_EN`"]
pub struct ACMP2_SAMPLE_SYNC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP2_SAMPLE_SYNC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP2_SAMPLE_SYNC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "select XBAR output"]
    #[inline(always)]
    pub fn acmp2_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline(always)]
    pub fn acmp2_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_1)
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
#[doc = "ACMP3 sample_lv source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP3_SAMPLE_SYNC_EN_A {
    #[doc = "0: select XBAR output"]
    ACMP3_SAMPLE_SYNC_EN_0,
    #[doc = "1: select synced sample_lv"]
    ACMP3_SAMPLE_SYNC_EN_1,
}
impl From<ACMP3_SAMPLE_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP3_SAMPLE_SYNC_EN_A) -> Self {
        match variant {
            ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_0 => false,
            ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP3_SAMPLE_SYNC_EN`"]
pub type ACMP3_SAMPLE_SYNC_EN_R = crate::R<bool, ACMP3_SAMPLE_SYNC_EN_A>;
impl ACMP3_SAMPLE_SYNC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP3_SAMPLE_SYNC_EN_A {
        match self.bits {
            false => ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_0,
            true => ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_SAMPLE_SYNC_EN_0`"]
    #[inline(always)]
    pub fn is_acmp3_sample_sync_en_0(&self) -> bool {
        *self == ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_SAMPLE_SYNC_EN_1`"]
    #[inline(always)]
    pub fn is_acmp3_sample_sync_en_1(&self) -> bool {
        *self == ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Write proxy for field `ACMP3_SAMPLE_SYNC_EN`"]
pub struct ACMP3_SAMPLE_SYNC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP3_SAMPLE_SYNC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP3_SAMPLE_SYNC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "select XBAR output"]
    #[inline(always)]
    pub fn acmp3_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline(always)]
    pub fn acmp3_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_1)
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
#[doc = "ACMP4 sample_lv source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP4_SAMPLE_SYNC_EN_A {
    #[doc = "0: select XBAR output"]
    ACMP4_SAMPLE_SYNC_EN_0,
    #[doc = "1: select synced sample_lv"]
    ACMP4_SAMPLE_SYNC_EN_1,
}
impl From<ACMP4_SAMPLE_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP4_SAMPLE_SYNC_EN_A) -> Self {
        match variant {
            ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_0 => false,
            ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `ACMP4_SAMPLE_SYNC_EN`"]
pub type ACMP4_SAMPLE_SYNC_EN_R = crate::R<bool, ACMP4_SAMPLE_SYNC_EN_A>;
impl ACMP4_SAMPLE_SYNC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP4_SAMPLE_SYNC_EN_A {
        match self.bits {
            false => ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_0,
            true => ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_SAMPLE_SYNC_EN_0`"]
    #[inline(always)]
    pub fn is_acmp4_sample_sync_en_0(&self) -> bool {
        *self == ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_SAMPLE_SYNC_EN_1`"]
    #[inline(always)]
    pub fn is_acmp4_sample_sync_en_1(&self) -> bool {
        *self == ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Write proxy for field `ACMP4_SAMPLE_SYNC_EN`"]
pub struct ACMP4_SAMPLE_SYNC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP4_SAMPLE_SYNC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP4_SAMPLE_SYNC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "select XBAR output"]
    #[inline(always)]
    pub fn acmp4_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline(always)]
    pub fn acmp4_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_1)
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
#[doc = "ITCM total size configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM7_CFGITCMSZ_A {
    #[doc = "0: 0 KB (No ITCM)"]
    CM7_CFGITCMSZ_0,
    #[doc = "3: 4 KB"]
    CM7_CFGITCMSZ_3,
    #[doc = "4: 8 KB"]
    CM7_CFGITCMSZ_4,
    #[doc = "5: 16 KB"]
    CM7_CFGITCMSZ_5,
    #[doc = "6: 32 KB"]
    CM7_CFGITCMSZ_6,
    #[doc = "7: 64 KB"]
    CM7_CFGITCMSZ_7,
    #[doc = "8: 128 KB"]
    CM7_CFGITCMSZ_8,
    #[doc = "9: 256 KB"]
    CM7_CFGITCMSZ_9,
    #[doc = "10: 512 KB"]
    CM7_CFGITCMSZ_10,
}
impl From<CM7_CFGITCMSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: CM7_CFGITCMSZ_A) -> Self {
        match variant {
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_0 => 0,
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_3 => 3,
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_4 => 4,
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_5 => 5,
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_6 => 6,
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_7 => 7,
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_8 => 8,
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_9 => 9,
            CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_10 => 10,
        }
    }
}
#[doc = "Reader of field `CM7_CFGITCMSZ`"]
pub type CM7_CFGITCMSZ_R = crate::R<u8, CM7_CFGITCMSZ_A>;
impl CM7_CFGITCMSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM7_CFGITCMSZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_0),
            3 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_3),
            4 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_4),
            5 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_5),
            6 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_6),
            7 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_7),
            8 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_8),
            9 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_9),
            10 => Val(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_0`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_0(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_0
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_3`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_3(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_3
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_4`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_4(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_4
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_5`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_5(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_5
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_6`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_6(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_6
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_7`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_7(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_7
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_8`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_8(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_8
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_9`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_9(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_9
    }
    #[doc = "Checks if the value of the field is `CM7_CFGITCMSZ_10`"]
    #[inline(always)]
    pub fn is_cm7_cfgitcmsz_10(&self) -> bool {
        *self == CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_10
    }
}
#[doc = "Write proxy for field `CM7_CFGITCMSZ`"]
pub struct CM7_CFGITCMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CM7_CFGITCMSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM7_CFGITCMSZ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 KB (No ITCM)"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_0(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_0)
    }
    #[doc = "4 KB"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_3(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_3)
    }
    #[doc = "8 KB"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_4(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_4)
    }
    #[doc = "16 KB"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_5(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_5)
    }
    #[doc = "32 KB"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_6(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_6)
    }
    #[doc = "64 KB"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_7(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_7)
    }
    #[doc = "128 KB"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_8(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_8)
    }
    #[doc = "256 KB"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_9(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_9)
    }
    #[doc = "512 KB"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz_10(self) -> &'a mut W {
        self.variant(CM7_CFGITCMSZ_A::CM7_CFGITCMSZ_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "DTCM total size configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM7_CFGDTCMSZ_A {
    #[doc = "0: 0 KB (No DTCM)"]
    CM7_CFGDTCMSZ_0,
    #[doc = "3: 4 KB"]
    CM7_CFGDTCMSZ_3,
    #[doc = "4: 8 KB"]
    CM7_CFGDTCMSZ_4,
    #[doc = "5: 16 KB"]
    CM7_CFGDTCMSZ_5,
    #[doc = "6: 32 KB"]
    CM7_CFGDTCMSZ_6,
    #[doc = "7: 64 KB"]
    CM7_CFGDTCMSZ_7,
    #[doc = "8: 128 KB"]
    CM7_CFGDTCMSZ_8,
    #[doc = "9: 256 KB"]
    CM7_CFGDTCMSZ_9,
    #[doc = "10: 512 KB"]
    CM7_CFGDTCMSZ_10,
}
impl From<CM7_CFGDTCMSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: CM7_CFGDTCMSZ_A) -> Self {
        match variant {
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_0 => 0,
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_3 => 3,
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_4 => 4,
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_5 => 5,
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_6 => 6,
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_7 => 7,
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_8 => 8,
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_9 => 9,
            CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_10 => 10,
        }
    }
}
#[doc = "Reader of field `CM7_CFGDTCMSZ`"]
pub type CM7_CFGDTCMSZ_R = crate::R<u8, CM7_CFGDTCMSZ_A>;
impl CM7_CFGDTCMSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM7_CFGDTCMSZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_0),
            3 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_3),
            4 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_4),
            5 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_5),
            6 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_6),
            7 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_7),
            8 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_8),
            9 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_9),
            10 => Val(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_0`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_0(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_0
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_3`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_3(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_3
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_4`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_4(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_4
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_5`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_5(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_5
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_6`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_6(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_6
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_7`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_7(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_7
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_8`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_8(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_8
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_9`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_9(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_9
    }
    #[doc = "Checks if the value of the field is `CM7_CFGDTCMSZ_10`"]
    #[inline(always)]
    pub fn is_cm7_cfgdtcmsz_10(&self) -> bool {
        *self == CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_10
    }
}
#[doc = "Write proxy for field `CM7_CFGDTCMSZ`"]
pub struct CM7_CFGDTCMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CM7_CFGDTCMSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM7_CFGDTCMSZ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 KB (No DTCM)"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_0(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_0)
    }
    #[doc = "4 KB"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_3(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_3)
    }
    #[doc = "8 KB"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_4(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_4)
    }
    #[doc = "16 KB"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_5(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_5)
    }
    #[doc = "32 KB"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_6(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_6)
    }
    #[doc = "64 KB"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_7(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_7)
    }
    #[doc = "128 KB"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_8(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_8)
    }
    #[doc = "256 KB"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_9(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_9)
    }
    #[doc = "512 KB"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz_10(self) -> &'a mut W {
        self.variant(CM7_CFGDTCMSZ_A::CM7_CFGDTCMSZ_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - reduces ACMP1 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_dn(&self) -> ACMP1_CMP_IGEN_TRIM_DN_R {
        ACMP1_CMP_IGEN_TRIM_DN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - reduces ACMP2 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_dn(&self) -> ACMP2_CMP_IGEN_TRIM_DN_R {
        ACMP2_CMP_IGEN_TRIM_DN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - reduces ACMP3 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_dn(&self) -> ACMP3_CMP_IGEN_TRIM_DN_R {
        ACMP3_CMP_IGEN_TRIM_DN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - reduces ACMP4 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_dn(&self) -> ACMP4_CMP_IGEN_TRIM_DN_R {
        ACMP4_CMP_IGEN_TRIM_DN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - increases ACMP1 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_up(&self) -> ACMP1_CMP_IGEN_TRIM_UP_R {
        ACMP1_CMP_IGEN_TRIM_UP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - increases ACMP2 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_up(&self) -> ACMP2_CMP_IGEN_TRIM_UP_R {
        ACMP2_CMP_IGEN_TRIM_UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - increases ACMP3 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_up(&self) -> ACMP3_CMP_IGEN_TRIM_UP_R {
        ACMP3_CMP_IGEN_TRIM_UP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - increases ACMP4 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_up(&self) -> ACMP4_CMP_IGEN_TRIM_UP_R {
        ACMP4_CMP_IGEN_TRIM_UP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ACMP1 sample_lv source select"]
    #[inline(always)]
    pub fn acmp1_sample_sync_en(&self) -> ACMP1_SAMPLE_SYNC_EN_R {
        ACMP1_SAMPLE_SYNC_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ACMP2 sample_lv source select"]
    #[inline(always)]
    pub fn acmp2_sample_sync_en(&self) -> ACMP2_SAMPLE_SYNC_EN_R {
        ACMP2_SAMPLE_SYNC_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ACMP3 sample_lv source select"]
    #[inline(always)]
    pub fn acmp3_sample_sync_en(&self) -> ACMP3_SAMPLE_SYNC_EN_R {
        ACMP3_SAMPLE_SYNC_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ACMP4 sample_lv source select"]
    #[inline(always)]
    pub fn acmp4_sample_sync_en(&self) -> ACMP4_SAMPLE_SYNC_EN_R {
        ACMP4_SAMPLE_SYNC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - ITCM total size configuration"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz(&self) -> CM7_CFGITCMSZ_R {
        CM7_CFGITCMSZ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DTCM total size configuration"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz(&self) -> CM7_CFGDTCMSZ_R {
        CM7_CFGDTCMSZ_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - reduces ACMP1 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_dn(&mut self) -> ACMP1_CMP_IGEN_TRIM_DN_W {
        ACMP1_CMP_IGEN_TRIM_DN_W { w: self }
    }
    #[doc = "Bit 1 - reduces ACMP2 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_dn(&mut self) -> ACMP2_CMP_IGEN_TRIM_DN_W {
        ACMP2_CMP_IGEN_TRIM_DN_W { w: self }
    }
    #[doc = "Bit 2 - reduces ACMP3 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_dn(&mut self) -> ACMP3_CMP_IGEN_TRIM_DN_W {
        ACMP3_CMP_IGEN_TRIM_DN_W { w: self }
    }
    #[doc = "Bit 3 - reduces ACMP4 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_dn(&mut self) -> ACMP4_CMP_IGEN_TRIM_DN_W {
        ACMP4_CMP_IGEN_TRIM_DN_W { w: self }
    }
    #[doc = "Bit 4 - increases ACMP1 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_up(&mut self) -> ACMP1_CMP_IGEN_TRIM_UP_W {
        ACMP1_CMP_IGEN_TRIM_UP_W { w: self }
    }
    #[doc = "Bit 5 - increases ACMP2 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_up(&mut self) -> ACMP2_CMP_IGEN_TRIM_UP_W {
        ACMP2_CMP_IGEN_TRIM_UP_W { w: self }
    }
    #[doc = "Bit 6 - increases ACMP3 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_up(&mut self) -> ACMP3_CMP_IGEN_TRIM_UP_W {
        ACMP3_CMP_IGEN_TRIM_UP_W { w: self }
    }
    #[doc = "Bit 7 - increases ACMP4 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_up(&mut self) -> ACMP4_CMP_IGEN_TRIM_UP_W {
        ACMP4_CMP_IGEN_TRIM_UP_W { w: self }
    }
    #[doc = "Bit 8 - ACMP1 sample_lv source select"]
    #[inline(always)]
    pub fn acmp1_sample_sync_en(&mut self) -> ACMP1_SAMPLE_SYNC_EN_W {
        ACMP1_SAMPLE_SYNC_EN_W { w: self }
    }
    #[doc = "Bit 9 - ACMP2 sample_lv source select"]
    #[inline(always)]
    pub fn acmp2_sample_sync_en(&mut self) -> ACMP2_SAMPLE_SYNC_EN_W {
        ACMP2_SAMPLE_SYNC_EN_W { w: self }
    }
    #[doc = "Bit 10 - ACMP3 sample_lv source select"]
    #[inline(always)]
    pub fn acmp3_sample_sync_en(&mut self) -> ACMP3_SAMPLE_SYNC_EN_W {
        ACMP3_SAMPLE_SYNC_EN_W { w: self }
    }
    #[doc = "Bit 11 - ACMP4 sample_lv source select"]
    #[inline(always)]
    pub fn acmp4_sample_sync_en(&mut self) -> ACMP4_SAMPLE_SYNC_EN_W {
        ACMP4_SAMPLE_SYNC_EN_W { w: self }
    }
    #[doc = "Bits 16:19 - ITCM total size configuration"]
    #[inline(always)]
    pub fn cm7_cfgitcmsz(&mut self) -> CM7_CFGITCMSZ_W {
        CM7_CFGITCMSZ_W { w: self }
    }
    #[doc = "Bits 20:23 - DTCM total size configuration"]
    #[inline(always)]
    pub fn cm7_cfgdtcmsz(&mut self) -> CM7_CFGDTCMSZ_W {
        CM7_CFGDTCMSZ_W { w: self }
    }
}
