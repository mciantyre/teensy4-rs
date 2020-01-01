#[doc = "Reader of register INT_STATUS_EN"]
pub type R = crate::R<u32, super::INT_STATUS_EN>;
#[doc = "Writer for register INT_STATUS_EN"]
pub type W = crate::W<u32, super::INT_STATUS_EN>;
#[doc = "Register INT_STATUS_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STATUS_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Command Complete Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCSEN_A {
    #[doc = "0: Masked"]
    CCSEN_0 = 0,
    #[doc = "1: Enabled"]
    CCSEN_1 = 1,
}
impl From<CCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCSEN`"]
pub type CCSEN_R = crate::R<bool, CCSEN_A>;
impl CCSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCSEN_A {
        match self.bits {
            false => CCSEN_A::CCSEN_0,
            true => CCSEN_A::CCSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCSEN_0`"]
    #[inline(always)]
    pub fn is_ccsen_0(&self) -> bool {
        *self == CCSEN_A::CCSEN_0
    }
    #[doc = "Checks if the value of the field is `CCSEN_1`"]
    #[inline(always)]
    pub fn is_ccsen_1(&self) -> bool {
        *self == CCSEN_A::CCSEN_1
    }
}
#[doc = "Write proxy for field `CCSEN`"]
pub struct CCSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ccsen_0(self) -> &'a mut W {
        self.variant(CCSEN_A::CCSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ccsen_1(self) -> &'a mut W {
        self.variant(CCSEN_A::CCSEN_1)
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
#[doc = "Transfer Complete Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCSEN_A {
    #[doc = "0: Masked"]
    TCSEN_0 = 0,
    #[doc = "1: Enabled"]
    TCSEN_1 = 1,
}
impl From<TCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TCSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCSEN`"]
pub type TCSEN_R = crate::R<bool, TCSEN_A>;
impl TCSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCSEN_A {
        match self.bits {
            false => TCSEN_A::TCSEN_0,
            true => TCSEN_A::TCSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCSEN_0`"]
    #[inline(always)]
    pub fn is_tcsen_0(&self) -> bool {
        *self == TCSEN_A::TCSEN_0
    }
    #[doc = "Checks if the value of the field is `TCSEN_1`"]
    #[inline(always)]
    pub fn is_tcsen_1(&self) -> bool {
        *self == TCSEN_A::TCSEN_1
    }
}
#[doc = "Write proxy for field `TCSEN`"]
pub struct TCSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tcsen_0(self) -> &'a mut W {
        self.variant(TCSEN_A::TCSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tcsen_1(self) -> &'a mut W {
        self.variant(TCSEN_A::TCSEN_1)
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
#[doc = "Block Gap Event Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGESEN_A {
    #[doc = "0: Masked"]
    BGESEN_0 = 0,
    #[doc = "1: Enabled"]
    BGESEN_1 = 1,
}
impl From<BGESEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BGESEN`"]
pub type BGESEN_R = crate::R<bool, BGESEN_A>;
impl BGESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGESEN_A {
        match self.bits {
            false => BGESEN_A::BGESEN_0,
            true => BGESEN_A::BGESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGESEN_0`"]
    #[inline(always)]
    pub fn is_bgesen_0(&self) -> bool {
        *self == BGESEN_A::BGESEN_0
    }
    #[doc = "Checks if the value of the field is `BGESEN_1`"]
    #[inline(always)]
    pub fn is_bgesen_1(&self) -> bool {
        *self == BGESEN_A::BGESEN_1
    }
}
#[doc = "Write proxy for field `BGESEN`"]
pub struct BGESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bgesen_0(self) -> &'a mut W {
        self.variant(BGESEN_A::BGESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bgesen_1(self) -> &'a mut W {
        self.variant(BGESEN_A::BGESEN_1)
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
#[doc = "DMA Interrupt Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINTSEN_A {
    #[doc = "0: Masked"]
    DINTSEN_0 = 0,
    #[doc = "1: Enabled"]
    DINTSEN_1 = 1,
}
impl From<DINTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: DINTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DINTSEN`"]
pub type DINTSEN_R = crate::R<bool, DINTSEN_A>;
impl DINTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINTSEN_A {
        match self.bits {
            false => DINTSEN_A::DINTSEN_0,
            true => DINTSEN_A::DINTSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DINTSEN_0`"]
    #[inline(always)]
    pub fn is_dintsen_0(&self) -> bool {
        *self == DINTSEN_A::DINTSEN_0
    }
    #[doc = "Checks if the value of the field is `DINTSEN_1`"]
    #[inline(always)]
    pub fn is_dintsen_1(&self) -> bool {
        *self == DINTSEN_A::DINTSEN_1
    }
}
#[doc = "Write proxy for field `DINTSEN`"]
pub struct DINTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DINTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dintsen_0(self) -> &'a mut W {
        self.variant(DINTSEN_A::DINTSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dintsen_1(self) -> &'a mut W {
        self.variant(DINTSEN_A::DINTSEN_1)
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
#[doc = "Buffer Write Ready Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRSEN_A {
    #[doc = "0: Masked"]
    BWRSEN_0 = 0,
    #[doc = "1: Enabled"]
    BWRSEN_1 = 1,
}
impl From<BWRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWRSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BWRSEN`"]
pub type BWRSEN_R = crate::R<bool, BWRSEN_A>;
impl BWRSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWRSEN_A {
        match self.bits {
            false => BWRSEN_A::BWRSEN_0,
            true => BWRSEN_A::BWRSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWRSEN_0`"]
    #[inline(always)]
    pub fn is_bwrsen_0(&self) -> bool {
        *self == BWRSEN_A::BWRSEN_0
    }
    #[doc = "Checks if the value of the field is `BWRSEN_1`"]
    #[inline(always)]
    pub fn is_bwrsen_1(&self) -> bool {
        *self == BWRSEN_A::BWRSEN_1
    }
}
#[doc = "Write proxy for field `BWRSEN`"]
pub struct BWRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BWRSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWRSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bwrsen_0(self) -> &'a mut W {
        self.variant(BWRSEN_A::BWRSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bwrsen_1(self) -> &'a mut W {
        self.variant(BWRSEN_A::BWRSEN_1)
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
#[doc = "Buffer Read Ready Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRRSEN_A {
    #[doc = "0: Masked"]
    BRRSEN_0 = 0,
    #[doc = "1: Enabled"]
    BRRSEN_1 = 1,
}
impl From<BRRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRRSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRRSEN`"]
pub type BRRSEN_R = crate::R<bool, BRRSEN_A>;
impl BRRSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRRSEN_A {
        match self.bits {
            false => BRRSEN_A::BRRSEN_0,
            true => BRRSEN_A::BRRSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRRSEN_0`"]
    #[inline(always)]
    pub fn is_brrsen_0(&self) -> bool {
        *self == BRRSEN_A::BRRSEN_0
    }
    #[doc = "Checks if the value of the field is `BRRSEN_1`"]
    #[inline(always)]
    pub fn is_brrsen_1(&self) -> bool {
        *self == BRRSEN_A::BRRSEN_1
    }
}
#[doc = "Write proxy for field `BRRSEN`"]
pub struct BRRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRRSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRRSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn brrsen_0(self) -> &'a mut W {
        self.variant(BRRSEN_A::BRRSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn brrsen_1(self) -> &'a mut W {
        self.variant(BRRSEN_A::BRRSEN_1)
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
#[doc = "Card Insertion Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSSEN_A {
    #[doc = "0: Masked"]
    CINSSEN_0 = 0,
    #[doc = "1: Enabled"]
    CINSSEN_1 = 1,
}
impl From<CINSSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINSSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CINSSEN`"]
pub type CINSSEN_R = crate::R<bool, CINSSEN_A>;
impl CINSSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSSEN_A {
        match self.bits {
            false => CINSSEN_A::CINSSEN_0,
            true => CINSSEN_A::CINSSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINSSEN_0`"]
    #[inline(always)]
    pub fn is_cinssen_0(&self) -> bool {
        *self == CINSSEN_A::CINSSEN_0
    }
    #[doc = "Checks if the value of the field is `CINSSEN_1`"]
    #[inline(always)]
    pub fn is_cinssen_1(&self) -> bool {
        *self == CINSSEN_A::CINSSEN_1
    }
}
#[doc = "Write proxy for field `CINSSEN`"]
pub struct CINSSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CINSSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINSSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cinssen_0(self) -> &'a mut W {
        self.variant(CINSSEN_A::CINSSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cinssen_1(self) -> &'a mut W {
        self.variant(CINSSEN_A::CINSSEN_1)
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
#[doc = "Card Removal Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMSEN_A {
    #[doc = "0: Masked"]
    CRMSEN_0 = 0,
    #[doc = "1: Enabled"]
    CRMSEN_1 = 1,
}
impl From<CRMSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRMSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRMSEN`"]
pub type CRMSEN_R = crate::R<bool, CRMSEN_A>;
impl CRMSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMSEN_A {
        match self.bits {
            false => CRMSEN_A::CRMSEN_0,
            true => CRMSEN_A::CRMSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRMSEN_0`"]
    #[inline(always)]
    pub fn is_crmsen_0(&self) -> bool {
        *self == CRMSEN_A::CRMSEN_0
    }
    #[doc = "Checks if the value of the field is `CRMSEN_1`"]
    #[inline(always)]
    pub fn is_crmsen_1(&self) -> bool {
        *self == CRMSEN_A::CRMSEN_1
    }
}
#[doc = "Write proxy for field `CRMSEN`"]
pub struct CRMSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRMSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRMSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn crmsen_0(self) -> &'a mut W {
        self.variant(CRMSEN_A::CRMSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn crmsen_1(self) -> &'a mut W {
        self.variant(CRMSEN_A::CRMSEN_1)
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
#[doc = "Card Interrupt Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTSEN_A {
    #[doc = "0: Masked"]
    CINTSEN_0 = 0,
    #[doc = "1: Enabled"]
    CINTSEN_1 = 1,
}
impl From<CINTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CINTSEN`"]
pub type CINTSEN_R = crate::R<bool, CINTSEN_A>;
impl CINTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINTSEN_A {
        match self.bits {
            false => CINTSEN_A::CINTSEN_0,
            true => CINTSEN_A::CINTSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINTSEN_0`"]
    #[inline(always)]
    pub fn is_cintsen_0(&self) -> bool {
        *self == CINTSEN_A::CINTSEN_0
    }
    #[doc = "Checks if the value of the field is `CINTSEN_1`"]
    #[inline(always)]
    pub fn is_cintsen_1(&self) -> bool {
        *self == CINTSEN_A::CINTSEN_1
    }
}
#[doc = "Write proxy for field `CINTSEN`"]
pub struct CINTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CINTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cintsen_0(self) -> &'a mut W {
        self.variant(CINTSEN_A::CINTSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cintsen_1(self) -> &'a mut W {
        self.variant(CINTSEN_A::CINTSEN_1)
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
#[doc = "Re-Tuning Event Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTESEN_A {
    #[doc = "0: Masked"]
    RTESEN_0 = 0,
    #[doc = "1: Enabled"]
    RTESEN_1 = 1,
}
impl From<RTESEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTESEN`"]
pub type RTESEN_R = crate::R<bool, RTESEN_A>;
impl RTESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTESEN_A {
        match self.bits {
            false => RTESEN_A::RTESEN_0,
            true => RTESEN_A::RTESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTESEN_0`"]
    #[inline(always)]
    pub fn is_rtesen_0(&self) -> bool {
        *self == RTESEN_A::RTESEN_0
    }
    #[doc = "Checks if the value of the field is `RTESEN_1`"]
    #[inline(always)]
    pub fn is_rtesen_1(&self) -> bool {
        *self == RTESEN_A::RTESEN_1
    }
}
#[doc = "Write proxy for field `RTESEN`"]
pub struct RTESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn rtesen_0(self) -> &'a mut W {
        self.variant(RTESEN_A::RTESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rtesen_1(self) -> &'a mut W {
        self.variant(RTESEN_A::RTESEN_1)
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
#[doc = "Tuning Pass Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPSEN_A {
    #[doc = "0: Masked"]
    TPSEN_0 = 0,
    #[doc = "1: Enabled"]
    TPSEN_1 = 1,
}
impl From<TPSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TPSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPSEN`"]
pub type TPSEN_R = crate::R<bool, TPSEN_A>;
impl TPSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPSEN_A {
        match self.bits {
            false => TPSEN_A::TPSEN_0,
            true => TPSEN_A::TPSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TPSEN_0`"]
    #[inline(always)]
    pub fn is_tpsen_0(&self) -> bool {
        *self == TPSEN_A::TPSEN_0
    }
    #[doc = "Checks if the value of the field is `TPSEN_1`"]
    #[inline(always)]
    pub fn is_tpsen_1(&self) -> bool {
        *self == TPSEN_A::TPSEN_1
    }
}
#[doc = "Write proxy for field `TPSEN`"]
pub struct TPSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TPSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tpsen_0(self) -> &'a mut W {
        self.variant(TPSEN_A::TPSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tpsen_1(self) -> &'a mut W {
        self.variant(TPSEN_A::TPSEN_1)
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
#[doc = "Command Timeout Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOESEN_A {
    #[doc = "0: Masked"]
    CTOESEN_0 = 0,
    #[doc = "1: Enabled"]
    CTOESEN_1 = 1,
}
impl From<CTOESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTOESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTOESEN`"]
pub type CTOESEN_R = crate::R<bool, CTOESEN_A>;
impl CTOESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOESEN_A {
        match self.bits {
            false => CTOESEN_A::CTOESEN_0,
            true => CTOESEN_A::CTOESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTOESEN_0`"]
    #[inline(always)]
    pub fn is_ctoesen_0(&self) -> bool {
        *self == CTOESEN_A::CTOESEN_0
    }
    #[doc = "Checks if the value of the field is `CTOESEN_1`"]
    #[inline(always)]
    pub fn is_ctoesen_1(&self) -> bool {
        *self == CTOESEN_A::CTOESEN_1
    }
}
#[doc = "Write proxy for field `CTOESEN`"]
pub struct CTOESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTOESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTOESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ctoesen_0(self) -> &'a mut W {
        self.variant(CTOESEN_A::CTOESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ctoesen_1(self) -> &'a mut W {
        self.variant(CTOESEN_A::CTOESEN_1)
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
#[doc = "Command CRC Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCESEN_A {
    #[doc = "0: Masked"]
    CCESEN_0 = 0,
    #[doc = "1: Enabled"]
    CCESEN_1 = 1,
}
impl From<CCESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCESEN`"]
pub type CCESEN_R = crate::R<bool, CCESEN_A>;
impl CCESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCESEN_A {
        match self.bits {
            false => CCESEN_A::CCESEN_0,
            true => CCESEN_A::CCESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCESEN_0`"]
    #[inline(always)]
    pub fn is_ccesen_0(&self) -> bool {
        *self == CCESEN_A::CCESEN_0
    }
    #[doc = "Checks if the value of the field is `CCESEN_1`"]
    #[inline(always)]
    pub fn is_ccesen_1(&self) -> bool {
        *self == CCESEN_A::CCESEN_1
    }
}
#[doc = "Write proxy for field `CCESEN`"]
pub struct CCESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ccesen_0(self) -> &'a mut W {
        self.variant(CCESEN_A::CCESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ccesen_1(self) -> &'a mut W {
        self.variant(CCESEN_A::CCESEN_1)
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
#[doc = "Command End Bit Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBESEN_A {
    #[doc = "0: Masked"]
    CEBESEN_0 = 0,
    #[doc = "1: Enabled"]
    CEBESEN_1 = 1,
}
impl From<CEBESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEBESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEBESEN`"]
pub type CEBESEN_R = crate::R<bool, CEBESEN_A>;
impl CEBESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBESEN_A {
        match self.bits {
            false => CEBESEN_A::CEBESEN_0,
            true => CEBESEN_A::CEBESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEBESEN_0`"]
    #[inline(always)]
    pub fn is_cebesen_0(&self) -> bool {
        *self == CEBESEN_A::CEBESEN_0
    }
    #[doc = "Checks if the value of the field is `CEBESEN_1`"]
    #[inline(always)]
    pub fn is_cebesen_1(&self) -> bool {
        *self == CEBESEN_A::CEBESEN_1
    }
}
#[doc = "Write proxy for field `CEBESEN`"]
pub struct CEBESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEBESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEBESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cebesen_0(self) -> &'a mut W {
        self.variant(CEBESEN_A::CEBESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cebesen_1(self) -> &'a mut W {
        self.variant(CEBESEN_A::CEBESEN_1)
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
#[doc = "Command Index Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIESEN_A {
    #[doc = "0: Masked"]
    CIESEN_0 = 0,
    #[doc = "1: Enabled"]
    CIESEN_1 = 1,
}
impl From<CIESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CIESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CIESEN`"]
pub type CIESEN_R = crate::R<bool, CIESEN_A>;
impl CIESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIESEN_A {
        match self.bits {
            false => CIESEN_A::CIESEN_0,
            true => CIESEN_A::CIESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIESEN_0`"]
    #[inline(always)]
    pub fn is_ciesen_0(&self) -> bool {
        *self == CIESEN_A::CIESEN_0
    }
    #[doc = "Checks if the value of the field is `CIESEN_1`"]
    #[inline(always)]
    pub fn is_ciesen_1(&self) -> bool {
        *self == CIESEN_A::CIESEN_1
    }
}
#[doc = "Write proxy for field `CIESEN`"]
pub struct CIESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CIESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ciesen_0(self) -> &'a mut W {
        self.variant(CIESEN_A::CIESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ciesen_1(self) -> &'a mut W {
        self.variant(CIESEN_A::CIESEN_1)
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
#[doc = "Data Timeout Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOESEN_A {
    #[doc = "0: Masked"]
    DTOESEN_0 = 0,
    #[doc = "1: Enabled"]
    DTOESEN_1 = 1,
}
impl From<DTOESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTOESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTOESEN`"]
pub type DTOESEN_R = crate::R<bool, DTOESEN_A>;
impl DTOESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOESEN_A {
        match self.bits {
            false => DTOESEN_A::DTOESEN_0,
            true => DTOESEN_A::DTOESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTOESEN_0`"]
    #[inline(always)]
    pub fn is_dtoesen_0(&self) -> bool {
        *self == DTOESEN_A::DTOESEN_0
    }
    #[doc = "Checks if the value of the field is `DTOESEN_1`"]
    #[inline(always)]
    pub fn is_dtoesen_1(&self) -> bool {
        *self == DTOESEN_A::DTOESEN_1
    }
}
#[doc = "Write proxy for field `DTOESEN`"]
pub struct DTOESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dtoesen_0(self) -> &'a mut W {
        self.variant(DTOESEN_A::DTOESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dtoesen_1(self) -> &'a mut W {
        self.variant(DTOESEN_A::DTOESEN_1)
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
#[doc = "Data CRC Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCESEN_A {
    #[doc = "0: Masked"]
    DCESEN_0 = 0,
    #[doc = "1: Enabled"]
    DCESEN_1 = 1,
}
impl From<DCESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCESEN`"]
pub type DCESEN_R = crate::R<bool, DCESEN_A>;
impl DCESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCESEN_A {
        match self.bits {
            false => DCESEN_A::DCESEN_0,
            true => DCESEN_A::DCESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCESEN_0`"]
    #[inline(always)]
    pub fn is_dcesen_0(&self) -> bool {
        *self == DCESEN_A::DCESEN_0
    }
    #[doc = "Checks if the value of the field is `DCESEN_1`"]
    #[inline(always)]
    pub fn is_dcesen_1(&self) -> bool {
        *self == DCESEN_A::DCESEN_1
    }
}
#[doc = "Write proxy for field `DCESEN`"]
pub struct DCESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dcesen_0(self) -> &'a mut W {
        self.variant(DCESEN_A::DCESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dcesen_1(self) -> &'a mut W {
        self.variant(DCESEN_A::DCESEN_1)
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
#[doc = "Data End Bit Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBESEN_A {
    #[doc = "0: Masked"]
    DEBESEN_0 = 0,
    #[doc = "1: Enabled"]
    DEBESEN_1 = 1,
}
impl From<DEBESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEBESEN`"]
pub type DEBESEN_R = crate::R<bool, DEBESEN_A>;
impl DEBESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBESEN_A {
        match self.bits {
            false => DEBESEN_A::DEBESEN_0,
            true => DEBESEN_A::DEBESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBESEN_0`"]
    #[inline(always)]
    pub fn is_debesen_0(&self) -> bool {
        *self == DEBESEN_A::DEBESEN_0
    }
    #[doc = "Checks if the value of the field is `DEBESEN_1`"]
    #[inline(always)]
    pub fn is_debesen_1(&self) -> bool {
        *self == DEBESEN_A::DEBESEN_1
    }
}
#[doc = "Write proxy for field `DEBESEN`"]
pub struct DEBESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn debesen_0(self) -> &'a mut W {
        self.variant(DEBESEN_A::DEBESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn debesen_1(self) -> &'a mut W {
        self.variant(DEBESEN_A::DEBESEN_1)
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
#[doc = "Auto CMD12 Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12ESEN_A {
    #[doc = "0: Masked"]
    AC12ESEN_0 = 0,
    #[doc = "1: Enabled"]
    AC12ESEN_1 = 1,
}
impl From<AC12ESEN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12ESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AC12ESEN`"]
pub type AC12ESEN_R = crate::R<bool, AC12ESEN_A>;
impl AC12ESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12ESEN_A {
        match self.bits {
            false => AC12ESEN_A::AC12ESEN_0,
            true => AC12ESEN_A::AC12ESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12ESEN_0`"]
    #[inline(always)]
    pub fn is_ac12esen_0(&self) -> bool {
        *self == AC12ESEN_A::AC12ESEN_0
    }
    #[doc = "Checks if the value of the field is `AC12ESEN_1`"]
    #[inline(always)]
    pub fn is_ac12esen_1(&self) -> bool {
        *self == AC12ESEN_A::AC12ESEN_1
    }
}
#[doc = "Write proxy for field `AC12ESEN`"]
pub struct AC12ESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12ESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AC12ESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ac12esen_0(self) -> &'a mut W {
        self.variant(AC12ESEN_A::AC12ESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ac12esen_1(self) -> &'a mut W {
        self.variant(AC12ESEN_A::AC12ESEN_1)
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
#[doc = "Tuning Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNESEN_A {
    #[doc = "0: Masked"]
    TNESEN_0 = 0,
    #[doc = "1: Enabled"]
    TNESEN_1 = 1,
}
impl From<TNESEN_A> for bool {
    #[inline(always)]
    fn from(variant: TNESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TNESEN`"]
pub type TNESEN_R = crate::R<bool, TNESEN_A>;
impl TNESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNESEN_A {
        match self.bits {
            false => TNESEN_A::TNESEN_0,
            true => TNESEN_A::TNESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TNESEN_0`"]
    #[inline(always)]
    pub fn is_tnesen_0(&self) -> bool {
        *self == TNESEN_A::TNESEN_0
    }
    #[doc = "Checks if the value of the field is `TNESEN_1`"]
    #[inline(always)]
    pub fn is_tnesen_1(&self) -> bool {
        *self == TNESEN_A::TNESEN_1
    }
}
#[doc = "Write proxy for field `TNESEN`"]
pub struct TNESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TNESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TNESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tnesen_0(self) -> &'a mut W {
        self.variant(TNESEN_A::TNESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tnesen_1(self) -> &'a mut W {
        self.variant(TNESEN_A::TNESEN_1)
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
#[doc = "DMA Error Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAESEN_A {
    #[doc = "0: Masked"]
    DMAESEN_0 = 0,
    #[doc = "1: Enabled"]
    DMAESEN_1 = 1,
}
impl From<DMAESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAESEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAESEN`"]
pub type DMAESEN_R = crate::R<bool, DMAESEN_A>;
impl DMAESEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAESEN_A {
        match self.bits {
            false => DMAESEN_A::DMAESEN_0,
            true => DMAESEN_A::DMAESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAESEN_0`"]
    #[inline(always)]
    pub fn is_dmaesen_0(&self) -> bool {
        *self == DMAESEN_A::DMAESEN_0
    }
    #[doc = "Checks if the value of the field is `DMAESEN_1`"]
    #[inline(always)]
    pub fn is_dmaesen_1(&self) -> bool {
        *self == DMAESEN_A::DMAESEN_1
    }
}
#[doc = "Write proxy for field `DMAESEN`"]
pub struct DMAESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAESEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAESEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dmaesen_0(self) -> &'a mut W {
        self.variant(DMAESEN_A::DMAESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dmaesen_1(self) -> &'a mut W {
        self.variant(DMAESEN_A::DMAESEN_1)
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
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn ccsen(&self) -> CCSEN_R {
        CCSEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn tcsen(&self) -> TCSEN_R {
        TCSEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn bgesen(&self) -> BGESEN_R {
        BGESEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn dintsen(&self) -> DINTSEN_R {
        DINTSEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn bwrsen(&self) -> BWRSEN_R {
        BWRSEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn brrsen(&self) -> BRRSEN_R {
        BRRSEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn cinssen(&self) -> CINSSEN_R {
        CINSSEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn crmsen(&self) -> CRMSEN_R {
        CRMSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn cintsen(&self) -> CINTSEN_R {
        CINTSEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable"]
    #[inline(always)]
    pub fn rtesen(&self) -> RTESEN_R {
        RTESEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tuning Pass Status Enable"]
    #[inline(always)]
    pub fn tpsen(&self) -> TPSEN_R {
        TPSEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline(always)]
    pub fn ctoesen(&self) -> CTOESEN_R {
        CTOESEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn ccesen(&self) -> CCESEN_R {
        CCESEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cebesen(&self) -> CEBESEN_R {
        CEBESEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn ciesen(&self) -> CIESEN_R {
        CIESEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dtoesen(&self) -> DTOESEN_R {
        DTOESEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn dcesen(&self) -> DCESEN_R {
        DCESEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn debesen(&self) -> DEBESEN_R {
        DEBESEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn ac12esen(&self) -> AC12ESEN_R {
        AC12ESEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tnesen(&self) -> TNESEN_R {
        TNESEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline(always)]
    pub fn dmaesen(&self) -> DMAESEN_R {
        DMAESEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn ccsen(&mut self) -> CCSEN_W {
        CCSEN_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn tcsen(&mut self) -> TCSEN_W {
        TCSEN_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn bgesen(&mut self) -> BGESEN_W {
        BGESEN_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn dintsen(&mut self) -> DINTSEN_W {
        DINTSEN_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn bwrsen(&mut self) -> BWRSEN_W {
        BWRSEN_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn brrsen(&mut self) -> BRRSEN_W {
        BRRSEN_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn cinssen(&mut self) -> CINSSEN_W {
        CINSSEN_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn crmsen(&mut self) -> CRMSEN_W {
        CRMSEN_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn cintsen(&mut self) -> CINTSEN_W {
        CINTSEN_W { w: self }
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable"]
    #[inline(always)]
    pub fn rtesen(&mut self) -> RTESEN_W {
        RTESEN_W { w: self }
    }
    #[doc = "Bit 14 - Tuning Pass Status Enable"]
    #[inline(always)]
    pub fn tpsen(&mut self) -> TPSEN_W {
        TPSEN_W { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline(always)]
    pub fn ctoesen(&mut self) -> CTOESEN_W {
        CTOESEN_W { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn ccesen(&mut self) -> CCESEN_W {
        CCESEN_W { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cebesen(&mut self) -> CEBESEN_W {
        CEBESEN_W { w: self }
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn ciesen(&mut self) -> CIESEN_W {
        CIESEN_W { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dtoesen(&mut self) -> DTOESEN_W {
        DTOESEN_W { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn dcesen(&mut self) -> DCESEN_W {
        DCESEN_W { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn debesen(&mut self) -> DEBESEN_W {
        DEBESEN_W { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn ac12esen(&mut self) -> AC12ESEN_W {
        AC12ESEN_W { w: self }
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tnesen(&mut self) -> TNESEN_W {
        TNESEN_W { w: self }
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline(always)]
    pub fn dmaesen(&mut self) -> DMAESEN_W {
        DMAESEN_W { w: self }
    }
}
