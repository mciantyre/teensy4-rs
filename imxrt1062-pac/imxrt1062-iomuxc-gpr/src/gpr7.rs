#[doc = "Reader of register GPR7"]
pub type R = crate::R<u32, super::GPR7>;
#[doc = "Writer for register GPR7"]
pub type W = crate::W<u32, super::GPR7>;
#[doc = "Register GPR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPI2C1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPI2C1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPI2C1_STOP_REQ_1 = 1,
}
impl From<LPI2C1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPI2C1_STOP_REQ`"]
pub type LPI2C1_STOP_REQ_R = crate::R<bool, LPI2C1_STOP_REQ_A>;
impl LPI2C1_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C1_STOP_REQ_A {
        match self.bits {
            false => LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_0,
            true => LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpi2c1_stop_req_0(&self) -> bool {
        *self == LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpi2c1_stop_req_1(&self) -> bool {
        *self == LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPI2C1_STOP_REQ`"]
pub struct LPI2C1_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C1_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C1_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpi2c1_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpi2c1_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_1)
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
#[doc = "LPI2C2 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPI2C2_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPI2C2_STOP_REQ_1 = 1,
}
impl From<LPI2C2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C2_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPI2C2_STOP_REQ`"]
pub type LPI2C2_STOP_REQ_R = crate::R<bool, LPI2C2_STOP_REQ_A>;
impl LPI2C2_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C2_STOP_REQ_A {
        match self.bits {
            false => LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_0,
            true => LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpi2c2_stop_req_0(&self) -> bool {
        *self == LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpi2c2_stop_req_1(&self) -> bool {
        *self == LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPI2C2_STOP_REQ`"]
pub struct LPI2C2_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C2_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C2_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpi2c2_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpi2c2_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_1)
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
#[doc = "LPI2C3 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPI2C3_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPI2C3_STOP_REQ_1 = 1,
}
impl From<LPI2C3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C3_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPI2C3_STOP_REQ`"]
pub type LPI2C3_STOP_REQ_R = crate::R<bool, LPI2C3_STOP_REQ_A>;
impl LPI2C3_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C3_STOP_REQ_A {
        match self.bits {
            false => LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_0,
            true => LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpi2c3_stop_req_0(&self) -> bool {
        *self == LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpi2c3_stop_req_1(&self) -> bool {
        *self == LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPI2C3_STOP_REQ`"]
pub struct LPI2C3_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C3_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C3_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpi2c3_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpi2c3_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_1)
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
#[doc = "LPI2C4 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C4_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPI2C4_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPI2C4_STOP_REQ_1 = 1,
}
impl From<LPI2C4_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C4_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPI2C4_STOP_REQ`"]
pub type LPI2C4_STOP_REQ_R = crate::R<bool, LPI2C4_STOP_REQ_A>;
impl LPI2C4_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C4_STOP_REQ_A {
        match self.bits {
            false => LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_0,
            true => LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpi2c4_stop_req_0(&self) -> bool {
        *self == LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpi2c4_stop_req_1(&self) -> bool {
        *self == LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPI2C4_STOP_REQ`"]
pub struct LPI2C4_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPI2C4_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPI2C4_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpi2c4_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpi2c4_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_1)
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
#[doc = "LPSPI1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPSPI1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPSPI1_STOP_REQ_1 = 1,
}
impl From<LPSPI1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSPI1_STOP_REQ`"]
pub type LPSPI1_STOP_REQ_R = crate::R<bool, LPSPI1_STOP_REQ_A>;
impl LPSPI1_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI1_STOP_REQ_A {
        match self.bits {
            false => LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_0,
            true => LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpspi1_stop_req_0(&self) -> bool {
        *self == LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpspi1_stop_req_1(&self) -> bool {
        *self == LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPSPI1_STOP_REQ`"]
pub struct LPSPI1_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI1_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI1_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpspi1_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpspi1_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_1)
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
#[doc = "LPSPI2 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPSPI2_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPSPI2_STOP_REQ_1 = 1,
}
impl From<LPSPI2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI2_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSPI2_STOP_REQ`"]
pub type LPSPI2_STOP_REQ_R = crate::R<bool, LPSPI2_STOP_REQ_A>;
impl LPSPI2_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI2_STOP_REQ_A {
        match self.bits {
            false => LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_0,
            true => LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpspi2_stop_req_0(&self) -> bool {
        *self == LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpspi2_stop_req_1(&self) -> bool {
        *self == LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPSPI2_STOP_REQ`"]
pub struct LPSPI2_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI2_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI2_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpspi2_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpspi2_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_1)
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
#[doc = "LPSPI3 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPSPI3_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPSPI3_STOP_REQ_1 = 1,
}
impl From<LPSPI3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI3_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSPI3_STOP_REQ`"]
pub type LPSPI3_STOP_REQ_R = crate::R<bool, LPSPI3_STOP_REQ_A>;
impl LPSPI3_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI3_STOP_REQ_A {
        match self.bits {
            false => LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_0,
            true => LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpspi3_stop_req_0(&self) -> bool {
        *self == LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpspi3_stop_req_1(&self) -> bool {
        *self == LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPSPI3_STOP_REQ`"]
pub struct LPSPI3_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI3_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI3_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpspi3_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpspi3_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_1)
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
#[doc = "LPSPI4 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI4_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPSPI4_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPSPI4_STOP_REQ_1 = 1,
}
impl From<LPSPI4_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI4_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSPI4_STOP_REQ`"]
pub type LPSPI4_STOP_REQ_R = crate::R<bool, LPSPI4_STOP_REQ_A>;
impl LPSPI4_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI4_STOP_REQ_A {
        match self.bits {
            false => LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_0,
            true => LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpspi4_stop_req_0(&self) -> bool {
        *self == LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpspi4_stop_req_1(&self) -> bool {
        *self == LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPSPI4_STOP_REQ`"]
pub struct LPSPI4_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI4_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI4_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpspi4_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpspi4_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_1)
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
#[doc = "LPUART1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART1_STOP_REQ_1 = 1,
}
impl From<LPUART1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART1_STOP_REQ`"]
pub type LPUART1_STOP_REQ_R = crate::R<bool, LPUART1_STOP_REQ_A>;
impl LPUART1_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_STOP_REQ_A {
        match self.bits {
            false => LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_0,
            true => LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart1_stop_req_0(&self) -> bool {
        *self == LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart1_stop_req_1(&self) -> bool {
        *self == LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPUART1_STOP_REQ`"]
pub struct LPUART1_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart1_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart1_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_1)
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
#[doc = "LPUART1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART2_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART2_STOP_REQ_1 = 1,
}
impl From<LPUART2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART2_STOP_REQ`"]
pub type LPUART2_STOP_REQ_R = crate::R<bool, LPUART2_STOP_REQ_A>;
impl LPUART2_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_STOP_REQ_A {
        match self.bits {
            false => LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_0,
            true => LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart2_stop_req_0(&self) -> bool {
        *self == LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart2_stop_req_1(&self) -> bool {
        *self == LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPUART2_STOP_REQ`"]
pub struct LPUART2_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART2_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART2_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart2_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart2_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_1)
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
#[doc = "LPUART3 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART3_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART3_STOP_REQ_1 = 1,
}
impl From<LPUART3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART3_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART3_STOP_REQ`"]
pub type LPUART3_STOP_REQ_R = crate::R<bool, LPUART3_STOP_REQ_A>;
impl LPUART3_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART3_STOP_REQ_A {
        match self.bits {
            false => LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_0,
            true => LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart3_stop_req_0(&self) -> bool {
        *self == LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart3_stop_req_1(&self) -> bool {
        *self == LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPUART3_STOP_REQ`"]
pub struct LPUART3_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART3_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART3_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart3_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart3_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_1)
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
#[doc = "LPUART4 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART4_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART4_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART4_STOP_REQ_1 = 1,
}
impl From<LPUART4_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART4_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART4_STOP_REQ`"]
pub type LPUART4_STOP_REQ_R = crate::R<bool, LPUART4_STOP_REQ_A>;
impl LPUART4_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART4_STOP_REQ_A {
        match self.bits {
            false => LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_0,
            true => LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart4_stop_req_0(&self) -> bool {
        *self == LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart4_stop_req_1(&self) -> bool {
        *self == LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPUART4_STOP_REQ`"]
pub struct LPUART4_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART4_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART4_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart4_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart4_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_1)
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
#[doc = "LPUART5 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART5_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART5_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART5_STOP_REQ_1 = 1,
}
impl From<LPUART5_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART5_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART5_STOP_REQ`"]
pub type LPUART5_STOP_REQ_R = crate::R<bool, LPUART5_STOP_REQ_A>;
impl LPUART5_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART5_STOP_REQ_A {
        match self.bits {
            false => LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_0,
            true => LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart5_stop_req_0(&self) -> bool {
        *self == LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart5_stop_req_1(&self) -> bool {
        *self == LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPUART5_STOP_REQ`"]
pub struct LPUART5_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART5_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART5_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart5_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart5_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_1)
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
#[doc = "LPUART6 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART6_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART6_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART6_STOP_REQ_1 = 1,
}
impl From<LPUART6_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART6_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART6_STOP_REQ`"]
pub type LPUART6_STOP_REQ_R = crate::R<bool, LPUART6_STOP_REQ_A>;
impl LPUART6_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART6_STOP_REQ_A {
        match self.bits {
            false => LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_0,
            true => LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart6_stop_req_0(&self) -> bool {
        *self == LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart6_stop_req_1(&self) -> bool {
        *self == LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPUART6_STOP_REQ`"]
pub struct LPUART6_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART6_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART6_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart6_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart6_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "LPUART7 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART7_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART7_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART7_STOP_REQ_1 = 1,
}
impl From<LPUART7_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART7_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART7_STOP_REQ`"]
pub type LPUART7_STOP_REQ_R = crate::R<bool, LPUART7_STOP_REQ_A>;
impl LPUART7_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART7_STOP_REQ_A {
        match self.bits {
            false => LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_0,
            true => LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart7_stop_req_0(&self) -> bool {
        *self == LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart7_stop_req_1(&self) -> bool {
        *self == LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPUART7_STOP_REQ`"]
pub struct LPUART7_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART7_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART7_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart7_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart7_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_1)
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
#[doc = "LPUART8 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART8_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART8_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART8_STOP_REQ_1 = 1,
}
impl From<LPUART8_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART8_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART8_STOP_REQ`"]
pub type LPUART8_STOP_REQ_R = crate::R<bool, LPUART8_STOP_REQ_A>;
impl LPUART8_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART8_STOP_REQ_A {
        match self.bits {
            false => LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_0,
            true => LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart8_stop_req_0(&self) -> bool {
        *self == LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart8_stop_req_1(&self) -> bool {
        *self == LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `LPUART8_STOP_REQ`"]
pub struct LPUART8_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART8_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART8_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart8_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart8_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "LPI2C1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C1_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPI2C1_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted (the module is in Stop mode)"]
    LPI2C1_STOP_ACK_1 = 1,
}
impl From<LPI2C1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPI2C1_STOP_ACK`"]
pub type LPI2C1_STOP_ACK_R = crate::R<bool, LPI2C1_STOP_ACK_A>;
impl LPI2C1_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C1_STOP_ACK_A {
        match self.bits {
            false => LPI2C1_STOP_ACK_A::LPI2C1_STOP_ACK_0,
            true => LPI2C1_STOP_ACK_A::LPI2C1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpi2c1_stop_ack_0(&self) -> bool {
        *self == LPI2C1_STOP_ACK_A::LPI2C1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpi2c1_stop_ack_1(&self) -> bool {
        *self == LPI2C1_STOP_ACK_A::LPI2C1_STOP_ACK_1
    }
}
#[doc = "LPI2C2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C2_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPI2C2_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPI2C2_STOP_ACK_1 = 1,
}
impl From<LPI2C2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C2_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPI2C2_STOP_ACK`"]
pub type LPI2C2_STOP_ACK_R = crate::R<bool, LPI2C2_STOP_ACK_A>;
impl LPI2C2_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C2_STOP_ACK_A {
        match self.bits {
            false => LPI2C2_STOP_ACK_A::LPI2C2_STOP_ACK_0,
            true => LPI2C2_STOP_ACK_A::LPI2C2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpi2c2_stop_ack_0(&self) -> bool {
        *self == LPI2C2_STOP_ACK_A::LPI2C2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpi2c2_stop_ack_1(&self) -> bool {
        *self == LPI2C2_STOP_ACK_A::LPI2C2_STOP_ACK_1
    }
}
#[doc = "LPI2C3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C3_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPI2C3_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPI2C3_STOP_ACK_1 = 1,
}
impl From<LPI2C3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C3_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPI2C3_STOP_ACK`"]
pub type LPI2C3_STOP_ACK_R = crate::R<bool, LPI2C3_STOP_ACK_A>;
impl LPI2C3_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C3_STOP_ACK_A {
        match self.bits {
            false => LPI2C3_STOP_ACK_A::LPI2C3_STOP_ACK_0,
            true => LPI2C3_STOP_ACK_A::LPI2C3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpi2c3_stop_ack_0(&self) -> bool {
        *self == LPI2C3_STOP_ACK_A::LPI2C3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpi2c3_stop_ack_1(&self) -> bool {
        *self == LPI2C3_STOP_ACK_A::LPI2C3_STOP_ACK_1
    }
}
#[doc = "LPI2C4 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPI2C4_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPI2C4_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPI2C4_STOP_ACK_1 = 1,
}
impl From<LPI2C4_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C4_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPI2C4_STOP_ACK`"]
pub type LPI2C4_STOP_ACK_R = crate::R<bool, LPI2C4_STOP_ACK_A>;
impl LPI2C4_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C4_STOP_ACK_A {
        match self.bits {
            false => LPI2C4_STOP_ACK_A::LPI2C4_STOP_ACK_0,
            true => LPI2C4_STOP_ACK_A::LPI2C4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpi2c4_stop_ack_0(&self) -> bool {
        *self == LPI2C4_STOP_ACK_A::LPI2C4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpi2c4_stop_ack_1(&self) -> bool {
        *self == LPI2C4_STOP_ACK_A::LPI2C4_STOP_ACK_1
    }
}
#[doc = "LPSPI1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI1_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPSPI1_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPSPI1_STOP_ACK_1 = 1,
}
impl From<LPSPI1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSPI1_STOP_ACK`"]
pub type LPSPI1_STOP_ACK_R = crate::R<bool, LPSPI1_STOP_ACK_A>;
impl LPSPI1_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI1_STOP_ACK_A {
        match self.bits {
            false => LPSPI1_STOP_ACK_A::LPSPI1_STOP_ACK_0,
            true => LPSPI1_STOP_ACK_A::LPSPI1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpspi1_stop_ack_0(&self) -> bool {
        *self == LPSPI1_STOP_ACK_A::LPSPI1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpspi1_stop_ack_1(&self) -> bool {
        *self == LPSPI1_STOP_ACK_A::LPSPI1_STOP_ACK_1
    }
}
#[doc = "LPSPI2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI2_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPSPI2_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPSPI2_STOP_ACK_1 = 1,
}
impl From<LPSPI2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI2_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSPI2_STOP_ACK`"]
pub type LPSPI2_STOP_ACK_R = crate::R<bool, LPSPI2_STOP_ACK_A>;
impl LPSPI2_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI2_STOP_ACK_A {
        match self.bits {
            false => LPSPI2_STOP_ACK_A::LPSPI2_STOP_ACK_0,
            true => LPSPI2_STOP_ACK_A::LPSPI2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpspi2_stop_ack_0(&self) -> bool {
        *self == LPSPI2_STOP_ACK_A::LPSPI2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpspi2_stop_ack_1(&self) -> bool {
        *self == LPSPI2_STOP_ACK_A::LPSPI2_STOP_ACK_1
    }
}
#[doc = "LPSPI3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI3_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPSPI3_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPSPI3_STOP_ACK_1 = 1,
}
impl From<LPSPI3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI3_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSPI3_STOP_ACK`"]
pub type LPSPI3_STOP_ACK_R = crate::R<bool, LPSPI3_STOP_ACK_A>;
impl LPSPI3_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI3_STOP_ACK_A {
        match self.bits {
            false => LPSPI3_STOP_ACK_A::LPSPI3_STOP_ACK_0,
            true => LPSPI3_STOP_ACK_A::LPSPI3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpspi3_stop_ack_0(&self) -> bool {
        *self == LPSPI3_STOP_ACK_A::LPSPI3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpspi3_stop_ack_1(&self) -> bool {
        *self == LPSPI3_STOP_ACK_A::LPSPI3_STOP_ACK_1
    }
}
#[doc = "LPSPI4 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSPI4_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPSPI4_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPSPI4_STOP_ACK_1 = 1,
}
impl From<LPSPI4_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI4_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPSPI4_STOP_ACK`"]
pub type LPSPI4_STOP_ACK_R = crate::R<bool, LPSPI4_STOP_ACK_A>;
impl LPSPI4_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI4_STOP_ACK_A {
        match self.bits {
            false => LPSPI4_STOP_ACK_A::LPSPI4_STOP_ACK_0,
            true => LPSPI4_STOP_ACK_A::LPSPI4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpspi4_stop_ack_0(&self) -> bool {
        *self == LPSPI4_STOP_ACK_A::LPSPI4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpspi4_stop_ack_1(&self) -> bool {
        *self == LPSPI4_STOP_ACK_A::LPSPI4_STOP_ACK_1
    }
}
#[doc = "LPUART1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART1_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART1_STOP_ACK_1 = 1,
}
impl From<LPUART1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART1_STOP_ACK`"]
pub type LPUART1_STOP_ACK_R = crate::R<bool, LPUART1_STOP_ACK_A>;
impl LPUART1_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_STOP_ACK_A {
        match self.bits {
            false => LPUART1_STOP_ACK_A::LPUART1_STOP_ACK_0,
            true => LPUART1_STOP_ACK_A::LPUART1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart1_stop_ack_0(&self) -> bool {
        *self == LPUART1_STOP_ACK_A::LPUART1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart1_stop_ack_1(&self) -> bool {
        *self == LPUART1_STOP_ACK_A::LPUART1_STOP_ACK_1
    }
}
#[doc = "LPUART1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART2_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART2_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART2_STOP_ACK_1 = 1,
}
impl From<LPUART2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART2_STOP_ACK`"]
pub type LPUART2_STOP_ACK_R = crate::R<bool, LPUART2_STOP_ACK_A>;
impl LPUART2_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_STOP_ACK_A {
        match self.bits {
            false => LPUART2_STOP_ACK_A::LPUART2_STOP_ACK_0,
            true => LPUART2_STOP_ACK_A::LPUART2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart2_stop_ack_0(&self) -> bool {
        *self == LPUART2_STOP_ACK_A::LPUART2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart2_stop_ack_1(&self) -> bool {
        *self == LPUART2_STOP_ACK_A::LPUART2_STOP_ACK_1
    }
}
#[doc = "LPUART3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART3_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART3_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART3_STOP_ACK_1 = 1,
}
impl From<LPUART3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART3_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART3_STOP_ACK`"]
pub type LPUART3_STOP_ACK_R = crate::R<bool, LPUART3_STOP_ACK_A>;
impl LPUART3_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART3_STOP_ACK_A {
        match self.bits {
            false => LPUART3_STOP_ACK_A::LPUART3_STOP_ACK_0,
            true => LPUART3_STOP_ACK_A::LPUART3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart3_stop_ack_0(&self) -> bool {
        *self == LPUART3_STOP_ACK_A::LPUART3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart3_stop_ack_1(&self) -> bool {
        *self == LPUART3_STOP_ACK_A::LPUART3_STOP_ACK_1
    }
}
#[doc = "LPUART4 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART4_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART4_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART4_STOP_ACK_1 = 1,
}
impl From<LPUART4_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART4_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART4_STOP_ACK`"]
pub type LPUART4_STOP_ACK_R = crate::R<bool, LPUART4_STOP_ACK_A>;
impl LPUART4_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART4_STOP_ACK_A {
        match self.bits {
            false => LPUART4_STOP_ACK_A::LPUART4_STOP_ACK_0,
            true => LPUART4_STOP_ACK_A::LPUART4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart4_stop_ack_0(&self) -> bool {
        *self == LPUART4_STOP_ACK_A::LPUART4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart4_stop_ack_1(&self) -> bool {
        *self == LPUART4_STOP_ACK_A::LPUART4_STOP_ACK_1
    }
}
#[doc = "LPUART5 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART5_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART5_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART5_STOP_ACK_1 = 1,
}
impl From<LPUART5_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART5_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART5_STOP_ACK`"]
pub type LPUART5_STOP_ACK_R = crate::R<bool, LPUART5_STOP_ACK_A>;
impl LPUART5_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART5_STOP_ACK_A {
        match self.bits {
            false => LPUART5_STOP_ACK_A::LPUART5_STOP_ACK_0,
            true => LPUART5_STOP_ACK_A::LPUART5_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart5_stop_ack_0(&self) -> bool {
        *self == LPUART5_STOP_ACK_A::LPUART5_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart5_stop_ack_1(&self) -> bool {
        *self == LPUART5_STOP_ACK_A::LPUART5_STOP_ACK_1
    }
}
#[doc = "LPUART6 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART6_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART6_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART6_STOP_ACK_1 = 1,
}
impl From<LPUART6_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART6_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART6_STOP_ACK`"]
pub type LPUART6_STOP_ACK_R = crate::R<bool, LPUART6_STOP_ACK_A>;
impl LPUART6_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART6_STOP_ACK_A {
        match self.bits {
            false => LPUART6_STOP_ACK_A::LPUART6_STOP_ACK_0,
            true => LPUART6_STOP_ACK_A::LPUART6_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart6_stop_ack_0(&self) -> bool {
        *self == LPUART6_STOP_ACK_A::LPUART6_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart6_stop_ack_1(&self) -> bool {
        *self == LPUART6_STOP_ACK_A::LPUART6_STOP_ACK_1
    }
}
#[doc = "LPUART7 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART7_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART7_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART7_STOP_ACK_1 = 1,
}
impl From<LPUART7_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART7_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART7_STOP_ACK`"]
pub type LPUART7_STOP_ACK_R = crate::R<bool, LPUART7_STOP_ACK_A>;
impl LPUART7_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART7_STOP_ACK_A {
        match self.bits {
            false => LPUART7_STOP_ACK_A::LPUART7_STOP_ACK_0,
            true => LPUART7_STOP_ACK_A::LPUART7_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart7_stop_ack_0(&self) -> bool {
        *self == LPUART7_STOP_ACK_A::LPUART7_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart7_stop_ack_1(&self) -> bool {
        *self == LPUART7_STOP_ACK_A::LPUART7_STOP_ACK_1
    }
}
#[doc = "LPUART8 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART8_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART8_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted (the module is in Stop mode)"]
    LPUART8_STOP_ACK_1 = 1,
}
impl From<LPUART8_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART8_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART8_STOP_ACK`"]
pub type LPUART8_STOP_ACK_R = crate::R<bool, LPUART8_STOP_ACK_A>;
impl LPUART8_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART8_STOP_ACK_A {
        match self.bits {
            false => LPUART8_STOP_ACK_A::LPUART8_STOP_ACK_0,
            true => LPUART8_STOP_ACK_A::LPUART8_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart8_stop_ack_0(&self) -> bool {
        *self == LPUART8_STOP_ACK_A::LPUART8_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart8_stop_ack_1(&self) -> bool {
        *self == LPUART8_STOP_ACK_A::LPUART8_STOP_ACK_1
    }
}
impl R {
    #[doc = "Bit 0 - LPI2C1 stop request"]
    #[inline(always)]
    pub fn lpi2c1_stop_req(&self) -> LPI2C1_STOP_REQ_R {
        LPI2C1_STOP_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPI2C2 stop request"]
    #[inline(always)]
    pub fn lpi2c2_stop_req(&self) -> LPI2C2_STOP_REQ_R {
        LPI2C2_STOP_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPI2C3 stop request"]
    #[inline(always)]
    pub fn lpi2c3_stop_req(&self) -> LPI2C3_STOP_REQ_R {
        LPI2C3_STOP_REQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPI2C4 stop request"]
    #[inline(always)]
    pub fn lpi2c4_stop_req(&self) -> LPI2C4_STOP_REQ_R {
        LPI2C4_STOP_REQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPSPI1 stop request"]
    #[inline(always)]
    pub fn lpspi1_stop_req(&self) -> LPSPI1_STOP_REQ_R {
        LPSPI1_STOP_REQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPSPI2 stop request"]
    #[inline(always)]
    pub fn lpspi2_stop_req(&self) -> LPSPI2_STOP_REQ_R {
        LPSPI2_STOP_REQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPSPI3 stop request"]
    #[inline(always)]
    pub fn lpspi3_stop_req(&self) -> LPSPI3_STOP_REQ_R {
        LPSPI3_STOP_REQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPSPI4 stop request"]
    #[inline(always)]
    pub fn lpspi4_stop_req(&self) -> LPSPI4_STOP_REQ_R {
        LPSPI4_STOP_REQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LPUART1 stop request"]
    #[inline(always)]
    pub fn lpuart1_stop_req(&self) -> LPUART1_STOP_REQ_R {
        LPUART1_STOP_REQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPUART1 stop request"]
    #[inline(always)]
    pub fn lpuart2_stop_req(&self) -> LPUART2_STOP_REQ_R {
        LPUART2_STOP_REQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPUART3 stop request"]
    #[inline(always)]
    pub fn lpuart3_stop_req(&self) -> LPUART3_STOP_REQ_R {
        LPUART3_STOP_REQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPUART4 stop request"]
    #[inline(always)]
    pub fn lpuart4_stop_req(&self) -> LPUART4_STOP_REQ_R {
        LPUART4_STOP_REQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LPUART5 stop request"]
    #[inline(always)]
    pub fn lpuart5_stop_req(&self) -> LPUART5_STOP_REQ_R {
        LPUART5_STOP_REQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LPUART6 stop request"]
    #[inline(always)]
    pub fn lpuart6_stop_req(&self) -> LPUART6_STOP_REQ_R {
        LPUART6_STOP_REQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LPUART7 stop request"]
    #[inline(always)]
    pub fn lpuart7_stop_req(&self) -> LPUART7_STOP_REQ_R {
        LPUART7_STOP_REQ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LPUART8 stop request"]
    #[inline(always)]
    pub fn lpuart8_stop_req(&self) -> LPUART8_STOP_REQ_R {
        LPUART8_STOP_REQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LPI2C1 stop acknowledge"]
    #[inline(always)]
    pub fn lpi2c1_stop_ack(&self) -> LPI2C1_STOP_ACK_R {
        LPI2C1_STOP_ACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LPI2C2 stop acknowledge"]
    #[inline(always)]
    pub fn lpi2c2_stop_ack(&self) -> LPI2C2_STOP_ACK_R {
        LPI2C2_STOP_ACK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LPI2C3 stop acknowledge"]
    #[inline(always)]
    pub fn lpi2c3_stop_ack(&self) -> LPI2C3_STOP_ACK_R {
        LPI2C3_STOP_ACK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LPI2C4 stop acknowledge"]
    #[inline(always)]
    pub fn lpi2c4_stop_ack(&self) -> LPI2C4_STOP_ACK_R {
        LPI2C4_STOP_ACK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPSPI1 stop acknowledge"]
    #[inline(always)]
    pub fn lpspi1_stop_ack(&self) -> LPSPI1_STOP_ACK_R {
        LPSPI1_STOP_ACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPSPI2 stop acknowledge"]
    #[inline(always)]
    pub fn lpspi2_stop_ack(&self) -> LPSPI2_STOP_ACK_R {
        LPSPI2_STOP_ACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LPSPI3 stop acknowledge"]
    #[inline(always)]
    pub fn lpspi3_stop_ack(&self) -> LPSPI3_STOP_ACK_R {
        LPSPI3_STOP_ACK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LPSPI4 stop acknowledge"]
    #[inline(always)]
    pub fn lpspi4_stop_ack(&self) -> LPSPI4_STOP_ACK_R {
        LPSPI4_STOP_ACK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LPUART1 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart1_stop_ack(&self) -> LPUART1_STOP_ACK_R {
        LPUART1_STOP_ACK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LPUART1 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart2_stop_ack(&self) -> LPUART2_STOP_ACK_R {
        LPUART2_STOP_ACK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LPUART3 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart3_stop_ack(&self) -> LPUART3_STOP_ACK_R {
        LPUART3_STOP_ACK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LPUART4 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart4_stop_ack(&self) -> LPUART4_STOP_ACK_R {
        LPUART4_STOP_ACK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LPUART5 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart5_stop_ack(&self) -> LPUART5_STOP_ACK_R {
        LPUART5_STOP_ACK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LPUART6 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart6_stop_ack(&self) -> LPUART6_STOP_ACK_R {
        LPUART6_STOP_ACK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LPUART7 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart7_stop_ack(&self) -> LPUART7_STOP_ACK_R {
        LPUART7_STOP_ACK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LPUART8 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart8_stop_ack(&self) -> LPUART8_STOP_ACK_R {
        LPUART8_STOP_ACK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPI2C1 stop request"]
    #[inline(always)]
    pub fn lpi2c1_stop_req(&mut self) -> LPI2C1_STOP_REQ_W {
        LPI2C1_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 1 - LPI2C2 stop request"]
    #[inline(always)]
    pub fn lpi2c2_stop_req(&mut self) -> LPI2C2_STOP_REQ_W {
        LPI2C2_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 2 - LPI2C3 stop request"]
    #[inline(always)]
    pub fn lpi2c3_stop_req(&mut self) -> LPI2C3_STOP_REQ_W {
        LPI2C3_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 3 - LPI2C4 stop request"]
    #[inline(always)]
    pub fn lpi2c4_stop_req(&mut self) -> LPI2C4_STOP_REQ_W {
        LPI2C4_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 4 - LPSPI1 stop request"]
    #[inline(always)]
    pub fn lpspi1_stop_req(&mut self) -> LPSPI1_STOP_REQ_W {
        LPSPI1_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 5 - LPSPI2 stop request"]
    #[inline(always)]
    pub fn lpspi2_stop_req(&mut self) -> LPSPI2_STOP_REQ_W {
        LPSPI2_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 6 - LPSPI3 stop request"]
    #[inline(always)]
    pub fn lpspi3_stop_req(&mut self) -> LPSPI3_STOP_REQ_W {
        LPSPI3_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 7 - LPSPI4 stop request"]
    #[inline(always)]
    pub fn lpspi4_stop_req(&mut self) -> LPSPI4_STOP_REQ_W {
        LPSPI4_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 8 - LPUART1 stop request"]
    #[inline(always)]
    pub fn lpuart1_stop_req(&mut self) -> LPUART1_STOP_REQ_W {
        LPUART1_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 9 - LPUART1 stop request"]
    #[inline(always)]
    pub fn lpuart2_stop_req(&mut self) -> LPUART2_STOP_REQ_W {
        LPUART2_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 10 - LPUART3 stop request"]
    #[inline(always)]
    pub fn lpuart3_stop_req(&mut self) -> LPUART3_STOP_REQ_W {
        LPUART3_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 11 - LPUART4 stop request"]
    #[inline(always)]
    pub fn lpuart4_stop_req(&mut self) -> LPUART4_STOP_REQ_W {
        LPUART4_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 12 - LPUART5 stop request"]
    #[inline(always)]
    pub fn lpuart5_stop_req(&mut self) -> LPUART5_STOP_REQ_W {
        LPUART5_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 13 - LPUART6 stop request"]
    #[inline(always)]
    pub fn lpuart6_stop_req(&mut self) -> LPUART6_STOP_REQ_W {
        LPUART6_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 14 - LPUART7 stop request"]
    #[inline(always)]
    pub fn lpuart7_stop_req(&mut self) -> LPUART7_STOP_REQ_W {
        LPUART7_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 15 - LPUART8 stop request"]
    #[inline(always)]
    pub fn lpuart8_stop_req(&mut self) -> LPUART8_STOP_REQ_W {
        LPUART8_STOP_REQ_W { w: self }
    }
}
