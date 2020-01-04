#[doc = "Reader of register EARS"]
pub type R = crate::R<u32, super::EARS>;
#[doc = "Writer for register EARS"]
pub type W = crate::W<u32, super::EARS>;
#[doc = "Register EARS `reset()`'s with value 0"]
impl crate::ResetValue for super::EARS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable asynchronous DMA request in stop mode for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_0_A {
    #[doc = "0: Disable asynchronous DMA request for channel 0."]
    EDREQ_0_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 0."]
    EDREQ_0_1 = 1,
}
impl From<EDREQ_0_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_0`"]
pub type EDREQ_0_R = crate::R<bool, EDREQ_0_A>;
impl EDREQ_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_0_A {
        match self.bits {
            false => EDREQ_0_A::EDREQ_0_0,
            true => EDREQ_0_A::EDREQ_0_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_0_0`"]
    #[inline(always)]
    pub fn is_edreq_0_0(&self) -> bool {
        *self == EDREQ_0_A::EDREQ_0_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_0_1`"]
    #[inline(always)]
    pub fn is_edreq_0_1(&self) -> bool {
        *self == EDREQ_0_A::EDREQ_0_1
    }
}
#[doc = "Write proxy for field `EDREQ_0`"]
pub struct EDREQ_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 0."]
    #[inline(always)]
    pub fn edreq_0_0(self) -> &'a mut W {
        self.variant(EDREQ_0_A::EDREQ_0_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 0."]
    #[inline(always)]
    pub fn edreq_0_1(self) -> &'a mut W {
        self.variant(EDREQ_0_A::EDREQ_0_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_1_A {
    #[doc = "0: Disable asynchronous DMA request for channel 1"]
    EDREQ_1_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 1."]
    EDREQ_1_1 = 1,
}
impl From<EDREQ_1_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_1`"]
pub type EDREQ_1_R = crate::R<bool, EDREQ_1_A>;
impl EDREQ_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_1_A {
        match self.bits {
            false => EDREQ_1_A::EDREQ_1_0,
            true => EDREQ_1_A::EDREQ_1_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_1_0`"]
    #[inline(always)]
    pub fn is_edreq_1_0(&self) -> bool {
        *self == EDREQ_1_A::EDREQ_1_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_1_1`"]
    #[inline(always)]
    pub fn is_edreq_1_1(&self) -> bool {
        *self == EDREQ_1_A::EDREQ_1_1
    }
}
#[doc = "Write proxy for field `EDREQ_1`"]
pub struct EDREQ_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 1"]
    #[inline(always)]
    pub fn edreq_1_0(self) -> &'a mut W {
        self.variant(EDREQ_1_A::EDREQ_1_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 1."]
    #[inline(always)]
    pub fn edreq_1_1(self) -> &'a mut W {
        self.variant(EDREQ_1_A::EDREQ_1_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_2_A {
    #[doc = "0: Disable asynchronous DMA request for channel 2."]
    EDREQ_2_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 2."]
    EDREQ_2_1 = 1,
}
impl From<EDREQ_2_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_2`"]
pub type EDREQ_2_R = crate::R<bool, EDREQ_2_A>;
impl EDREQ_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_2_A {
        match self.bits {
            false => EDREQ_2_A::EDREQ_2_0,
            true => EDREQ_2_A::EDREQ_2_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_2_0`"]
    #[inline(always)]
    pub fn is_edreq_2_0(&self) -> bool {
        *self == EDREQ_2_A::EDREQ_2_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_2_1`"]
    #[inline(always)]
    pub fn is_edreq_2_1(&self) -> bool {
        *self == EDREQ_2_A::EDREQ_2_1
    }
}
#[doc = "Write proxy for field `EDREQ_2`"]
pub struct EDREQ_2_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 2."]
    #[inline(always)]
    pub fn edreq_2_0(self) -> &'a mut W {
        self.variant(EDREQ_2_A::EDREQ_2_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 2."]
    #[inline(always)]
    pub fn edreq_2_1(self) -> &'a mut W {
        self.variant(EDREQ_2_A::EDREQ_2_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_3_A {
    #[doc = "0: Disable asynchronous DMA request for channel 3."]
    EDREQ_3_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 3."]
    EDREQ_3_1 = 1,
}
impl From<EDREQ_3_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_3`"]
pub type EDREQ_3_R = crate::R<bool, EDREQ_3_A>;
impl EDREQ_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_3_A {
        match self.bits {
            false => EDREQ_3_A::EDREQ_3_0,
            true => EDREQ_3_A::EDREQ_3_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_3_0`"]
    #[inline(always)]
    pub fn is_edreq_3_0(&self) -> bool {
        *self == EDREQ_3_A::EDREQ_3_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_3_1`"]
    #[inline(always)]
    pub fn is_edreq_3_1(&self) -> bool {
        *self == EDREQ_3_A::EDREQ_3_1
    }
}
#[doc = "Write proxy for field `EDREQ_3`"]
pub struct EDREQ_3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 3."]
    #[inline(always)]
    pub fn edreq_3_0(self) -> &'a mut W {
        self.variant(EDREQ_3_A::EDREQ_3_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 3."]
    #[inline(always)]
    pub fn edreq_3_1(self) -> &'a mut W {
        self.variant(EDREQ_3_A::EDREQ_3_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_4_A {
    #[doc = "0: Disable asynchronous DMA request for channel 4."]
    EDREQ_4_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 4."]
    EDREQ_4_1 = 1,
}
impl From<EDREQ_4_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_4`"]
pub type EDREQ_4_R = crate::R<bool, EDREQ_4_A>;
impl EDREQ_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_4_A {
        match self.bits {
            false => EDREQ_4_A::EDREQ_4_0,
            true => EDREQ_4_A::EDREQ_4_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_4_0`"]
    #[inline(always)]
    pub fn is_edreq_4_0(&self) -> bool {
        *self == EDREQ_4_A::EDREQ_4_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_4_1`"]
    #[inline(always)]
    pub fn is_edreq_4_1(&self) -> bool {
        *self == EDREQ_4_A::EDREQ_4_1
    }
}
#[doc = "Write proxy for field `EDREQ_4`"]
pub struct EDREQ_4_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 4."]
    #[inline(always)]
    pub fn edreq_4_0(self) -> &'a mut W {
        self.variant(EDREQ_4_A::EDREQ_4_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 4."]
    #[inline(always)]
    pub fn edreq_4_1(self) -> &'a mut W {
        self.variant(EDREQ_4_A::EDREQ_4_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_5_A {
    #[doc = "0: Disable asynchronous DMA request for channel 5."]
    EDREQ_5_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 5."]
    EDREQ_5_1 = 1,
}
impl From<EDREQ_5_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_5`"]
pub type EDREQ_5_R = crate::R<bool, EDREQ_5_A>;
impl EDREQ_5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_5_A {
        match self.bits {
            false => EDREQ_5_A::EDREQ_5_0,
            true => EDREQ_5_A::EDREQ_5_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_5_0`"]
    #[inline(always)]
    pub fn is_edreq_5_0(&self) -> bool {
        *self == EDREQ_5_A::EDREQ_5_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_5_1`"]
    #[inline(always)]
    pub fn is_edreq_5_1(&self) -> bool {
        *self == EDREQ_5_A::EDREQ_5_1
    }
}
#[doc = "Write proxy for field `EDREQ_5`"]
pub struct EDREQ_5_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 5."]
    #[inline(always)]
    pub fn edreq_5_0(self) -> &'a mut W {
        self.variant(EDREQ_5_A::EDREQ_5_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 5."]
    #[inline(always)]
    pub fn edreq_5_1(self) -> &'a mut W {
        self.variant(EDREQ_5_A::EDREQ_5_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_6_A {
    #[doc = "0: Disable asynchronous DMA request for channel 6."]
    EDREQ_6_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 6."]
    EDREQ_6_1 = 1,
}
impl From<EDREQ_6_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_6`"]
pub type EDREQ_6_R = crate::R<bool, EDREQ_6_A>;
impl EDREQ_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_6_A {
        match self.bits {
            false => EDREQ_6_A::EDREQ_6_0,
            true => EDREQ_6_A::EDREQ_6_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_6_0`"]
    #[inline(always)]
    pub fn is_edreq_6_0(&self) -> bool {
        *self == EDREQ_6_A::EDREQ_6_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_6_1`"]
    #[inline(always)]
    pub fn is_edreq_6_1(&self) -> bool {
        *self == EDREQ_6_A::EDREQ_6_1
    }
}
#[doc = "Write proxy for field `EDREQ_6`"]
pub struct EDREQ_6_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 6."]
    #[inline(always)]
    pub fn edreq_6_0(self) -> &'a mut W {
        self.variant(EDREQ_6_A::EDREQ_6_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 6."]
    #[inline(always)]
    pub fn edreq_6_1(self) -> &'a mut W {
        self.variant(EDREQ_6_A::EDREQ_6_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_7_A {
    #[doc = "0: Disable asynchronous DMA request for channel 7."]
    EDREQ_7_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 7."]
    EDREQ_7_1 = 1,
}
impl From<EDREQ_7_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_7`"]
pub type EDREQ_7_R = crate::R<bool, EDREQ_7_A>;
impl EDREQ_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_7_A {
        match self.bits {
            false => EDREQ_7_A::EDREQ_7_0,
            true => EDREQ_7_A::EDREQ_7_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_7_0`"]
    #[inline(always)]
    pub fn is_edreq_7_0(&self) -> bool {
        *self == EDREQ_7_A::EDREQ_7_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_7_1`"]
    #[inline(always)]
    pub fn is_edreq_7_1(&self) -> bool {
        *self == EDREQ_7_A::EDREQ_7_1
    }
}
#[doc = "Write proxy for field `EDREQ_7`"]
pub struct EDREQ_7_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 7."]
    #[inline(always)]
    pub fn edreq_7_0(self) -> &'a mut W {
        self.variant(EDREQ_7_A::EDREQ_7_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 7."]
    #[inline(always)]
    pub fn edreq_7_1(self) -> &'a mut W {
        self.variant(EDREQ_7_A::EDREQ_7_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_8_A {
    #[doc = "0: Disable asynchronous DMA request for channel 8."]
    EDREQ_8_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 8."]
    EDREQ_8_1 = 1,
}
impl From<EDREQ_8_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_8`"]
pub type EDREQ_8_R = crate::R<bool, EDREQ_8_A>;
impl EDREQ_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_8_A {
        match self.bits {
            false => EDREQ_8_A::EDREQ_8_0,
            true => EDREQ_8_A::EDREQ_8_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_8_0`"]
    #[inline(always)]
    pub fn is_edreq_8_0(&self) -> bool {
        *self == EDREQ_8_A::EDREQ_8_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_8_1`"]
    #[inline(always)]
    pub fn is_edreq_8_1(&self) -> bool {
        *self == EDREQ_8_A::EDREQ_8_1
    }
}
#[doc = "Write proxy for field `EDREQ_8`"]
pub struct EDREQ_8_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 8."]
    #[inline(always)]
    pub fn edreq_8_0(self) -> &'a mut W {
        self.variant(EDREQ_8_A::EDREQ_8_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 8."]
    #[inline(always)]
    pub fn edreq_8_1(self) -> &'a mut W {
        self.variant(EDREQ_8_A::EDREQ_8_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_9_A {
    #[doc = "0: Disable asynchronous DMA request for channel 9."]
    EDREQ_9_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 9."]
    EDREQ_9_1 = 1,
}
impl From<EDREQ_9_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_9`"]
pub type EDREQ_9_R = crate::R<bool, EDREQ_9_A>;
impl EDREQ_9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_9_A {
        match self.bits {
            false => EDREQ_9_A::EDREQ_9_0,
            true => EDREQ_9_A::EDREQ_9_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_9_0`"]
    #[inline(always)]
    pub fn is_edreq_9_0(&self) -> bool {
        *self == EDREQ_9_A::EDREQ_9_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_9_1`"]
    #[inline(always)]
    pub fn is_edreq_9_1(&self) -> bool {
        *self == EDREQ_9_A::EDREQ_9_1
    }
}
#[doc = "Write proxy for field `EDREQ_9`"]
pub struct EDREQ_9_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 9."]
    #[inline(always)]
    pub fn edreq_9_0(self) -> &'a mut W {
        self.variant(EDREQ_9_A::EDREQ_9_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 9."]
    #[inline(always)]
    pub fn edreq_9_1(self) -> &'a mut W {
        self.variant(EDREQ_9_A::EDREQ_9_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_10_A {
    #[doc = "0: Disable asynchronous DMA request for channel 10."]
    EDREQ_10_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 10."]
    EDREQ_10_1 = 1,
}
impl From<EDREQ_10_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_10`"]
pub type EDREQ_10_R = crate::R<bool, EDREQ_10_A>;
impl EDREQ_10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_10_A {
        match self.bits {
            false => EDREQ_10_A::EDREQ_10_0,
            true => EDREQ_10_A::EDREQ_10_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_10_0`"]
    #[inline(always)]
    pub fn is_edreq_10_0(&self) -> bool {
        *self == EDREQ_10_A::EDREQ_10_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_10_1`"]
    #[inline(always)]
    pub fn is_edreq_10_1(&self) -> bool {
        *self == EDREQ_10_A::EDREQ_10_1
    }
}
#[doc = "Write proxy for field `EDREQ_10`"]
pub struct EDREQ_10_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 10."]
    #[inline(always)]
    pub fn edreq_10_0(self) -> &'a mut W {
        self.variant(EDREQ_10_A::EDREQ_10_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 10."]
    #[inline(always)]
    pub fn edreq_10_1(self) -> &'a mut W {
        self.variant(EDREQ_10_A::EDREQ_10_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_11_A {
    #[doc = "0: Disable asynchronous DMA request for channel 11."]
    EDREQ_11_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 11."]
    EDREQ_11_1 = 1,
}
impl From<EDREQ_11_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_11`"]
pub type EDREQ_11_R = crate::R<bool, EDREQ_11_A>;
impl EDREQ_11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_11_A {
        match self.bits {
            false => EDREQ_11_A::EDREQ_11_0,
            true => EDREQ_11_A::EDREQ_11_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_11_0`"]
    #[inline(always)]
    pub fn is_edreq_11_0(&self) -> bool {
        *self == EDREQ_11_A::EDREQ_11_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_11_1`"]
    #[inline(always)]
    pub fn is_edreq_11_1(&self) -> bool {
        *self == EDREQ_11_A::EDREQ_11_1
    }
}
#[doc = "Write proxy for field `EDREQ_11`"]
pub struct EDREQ_11_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 11."]
    #[inline(always)]
    pub fn edreq_11_0(self) -> &'a mut W {
        self.variant(EDREQ_11_A::EDREQ_11_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 11."]
    #[inline(always)]
    pub fn edreq_11_1(self) -> &'a mut W {
        self.variant(EDREQ_11_A::EDREQ_11_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_12_A {
    #[doc = "0: Disable asynchronous DMA request for channel 12."]
    EDREQ_12_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 12."]
    EDREQ_12_1 = 1,
}
impl From<EDREQ_12_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_12`"]
pub type EDREQ_12_R = crate::R<bool, EDREQ_12_A>;
impl EDREQ_12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_12_A {
        match self.bits {
            false => EDREQ_12_A::EDREQ_12_0,
            true => EDREQ_12_A::EDREQ_12_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_12_0`"]
    #[inline(always)]
    pub fn is_edreq_12_0(&self) -> bool {
        *self == EDREQ_12_A::EDREQ_12_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_12_1`"]
    #[inline(always)]
    pub fn is_edreq_12_1(&self) -> bool {
        *self == EDREQ_12_A::EDREQ_12_1
    }
}
#[doc = "Write proxy for field `EDREQ_12`"]
pub struct EDREQ_12_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 12."]
    #[inline(always)]
    pub fn edreq_12_0(self) -> &'a mut W {
        self.variant(EDREQ_12_A::EDREQ_12_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 12."]
    #[inline(always)]
    pub fn edreq_12_1(self) -> &'a mut W {
        self.variant(EDREQ_12_A::EDREQ_12_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_13_A {
    #[doc = "0: Disable asynchronous DMA request for channel 13."]
    EDREQ_13_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 13."]
    EDREQ_13_1 = 1,
}
impl From<EDREQ_13_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_13`"]
pub type EDREQ_13_R = crate::R<bool, EDREQ_13_A>;
impl EDREQ_13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_13_A {
        match self.bits {
            false => EDREQ_13_A::EDREQ_13_0,
            true => EDREQ_13_A::EDREQ_13_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_13_0`"]
    #[inline(always)]
    pub fn is_edreq_13_0(&self) -> bool {
        *self == EDREQ_13_A::EDREQ_13_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_13_1`"]
    #[inline(always)]
    pub fn is_edreq_13_1(&self) -> bool {
        *self == EDREQ_13_A::EDREQ_13_1
    }
}
#[doc = "Write proxy for field `EDREQ_13`"]
pub struct EDREQ_13_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 13."]
    #[inline(always)]
    pub fn edreq_13_0(self) -> &'a mut W {
        self.variant(EDREQ_13_A::EDREQ_13_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 13."]
    #[inline(always)]
    pub fn edreq_13_1(self) -> &'a mut W {
        self.variant(EDREQ_13_A::EDREQ_13_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_14_A {
    #[doc = "0: Disable asynchronous DMA request for channel 14."]
    EDREQ_14_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 14."]
    EDREQ_14_1 = 1,
}
impl From<EDREQ_14_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_14`"]
pub type EDREQ_14_R = crate::R<bool, EDREQ_14_A>;
impl EDREQ_14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_14_A {
        match self.bits {
            false => EDREQ_14_A::EDREQ_14_0,
            true => EDREQ_14_A::EDREQ_14_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_14_0`"]
    #[inline(always)]
    pub fn is_edreq_14_0(&self) -> bool {
        *self == EDREQ_14_A::EDREQ_14_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_14_1`"]
    #[inline(always)]
    pub fn is_edreq_14_1(&self) -> bool {
        *self == EDREQ_14_A::EDREQ_14_1
    }
}
#[doc = "Write proxy for field `EDREQ_14`"]
pub struct EDREQ_14_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 14."]
    #[inline(always)]
    pub fn edreq_14_0(self) -> &'a mut W {
        self.variant(EDREQ_14_A::EDREQ_14_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 14."]
    #[inline(always)]
    pub fn edreq_14_1(self) -> &'a mut W {
        self.variant(EDREQ_14_A::EDREQ_14_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_15_A {
    #[doc = "0: Disable asynchronous DMA request for channel 15."]
    EDREQ_15_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 15."]
    EDREQ_15_1 = 1,
}
impl From<EDREQ_15_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_15`"]
pub type EDREQ_15_R = crate::R<bool, EDREQ_15_A>;
impl EDREQ_15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_15_A {
        match self.bits {
            false => EDREQ_15_A::EDREQ_15_0,
            true => EDREQ_15_A::EDREQ_15_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_15_0`"]
    #[inline(always)]
    pub fn is_edreq_15_0(&self) -> bool {
        *self == EDREQ_15_A::EDREQ_15_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_15_1`"]
    #[inline(always)]
    pub fn is_edreq_15_1(&self) -> bool {
        *self == EDREQ_15_A::EDREQ_15_1
    }
}
#[doc = "Write proxy for field `EDREQ_15`"]
pub struct EDREQ_15_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 15."]
    #[inline(always)]
    pub fn edreq_15_0(self) -> &'a mut W {
        self.variant(EDREQ_15_A::EDREQ_15_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 15."]
    #[inline(always)]
    pub fn edreq_15_1(self) -> &'a mut W {
        self.variant(EDREQ_15_A::EDREQ_15_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_16_A {
    #[doc = "0: Disable asynchronous DMA request for channel 16"]
    EDREQ_16_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 16"]
    EDREQ_16_1 = 1,
}
impl From<EDREQ_16_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_16`"]
pub type EDREQ_16_R = crate::R<bool, EDREQ_16_A>;
impl EDREQ_16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_16_A {
        match self.bits {
            false => EDREQ_16_A::EDREQ_16_0,
            true => EDREQ_16_A::EDREQ_16_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_16_0`"]
    #[inline(always)]
    pub fn is_edreq_16_0(&self) -> bool {
        *self == EDREQ_16_A::EDREQ_16_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_16_1`"]
    #[inline(always)]
    pub fn is_edreq_16_1(&self) -> bool {
        *self == EDREQ_16_A::EDREQ_16_1
    }
}
#[doc = "Write proxy for field `EDREQ_16`"]
pub struct EDREQ_16_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 16"]
    #[inline(always)]
    pub fn edreq_16_0(self) -> &'a mut W {
        self.variant(EDREQ_16_A::EDREQ_16_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 16"]
    #[inline(always)]
    pub fn edreq_16_1(self) -> &'a mut W {
        self.variant(EDREQ_16_A::EDREQ_16_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_17_A {
    #[doc = "0: Disable asynchronous DMA request for channel 17"]
    EDREQ_17_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 17"]
    EDREQ_17_1 = 1,
}
impl From<EDREQ_17_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_17`"]
pub type EDREQ_17_R = crate::R<bool, EDREQ_17_A>;
impl EDREQ_17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_17_A {
        match self.bits {
            false => EDREQ_17_A::EDREQ_17_0,
            true => EDREQ_17_A::EDREQ_17_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_17_0`"]
    #[inline(always)]
    pub fn is_edreq_17_0(&self) -> bool {
        *self == EDREQ_17_A::EDREQ_17_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_17_1`"]
    #[inline(always)]
    pub fn is_edreq_17_1(&self) -> bool {
        *self == EDREQ_17_A::EDREQ_17_1
    }
}
#[doc = "Write proxy for field `EDREQ_17`"]
pub struct EDREQ_17_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 17"]
    #[inline(always)]
    pub fn edreq_17_0(self) -> &'a mut W {
        self.variant(EDREQ_17_A::EDREQ_17_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 17"]
    #[inline(always)]
    pub fn edreq_17_1(self) -> &'a mut W {
        self.variant(EDREQ_17_A::EDREQ_17_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_18_A {
    #[doc = "0: Disable asynchronous DMA request for channel 18"]
    EDREQ_18_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 18"]
    EDREQ_18_1 = 1,
}
impl From<EDREQ_18_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_18`"]
pub type EDREQ_18_R = crate::R<bool, EDREQ_18_A>;
impl EDREQ_18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_18_A {
        match self.bits {
            false => EDREQ_18_A::EDREQ_18_0,
            true => EDREQ_18_A::EDREQ_18_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_18_0`"]
    #[inline(always)]
    pub fn is_edreq_18_0(&self) -> bool {
        *self == EDREQ_18_A::EDREQ_18_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_18_1`"]
    #[inline(always)]
    pub fn is_edreq_18_1(&self) -> bool {
        *self == EDREQ_18_A::EDREQ_18_1
    }
}
#[doc = "Write proxy for field `EDREQ_18`"]
pub struct EDREQ_18_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 18"]
    #[inline(always)]
    pub fn edreq_18_0(self) -> &'a mut W {
        self.variant(EDREQ_18_A::EDREQ_18_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 18"]
    #[inline(always)]
    pub fn edreq_18_1(self) -> &'a mut W {
        self.variant(EDREQ_18_A::EDREQ_18_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_19_A {
    #[doc = "0: Disable asynchronous DMA request for channel 19"]
    EDREQ_19_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 19"]
    EDREQ_19_1 = 1,
}
impl From<EDREQ_19_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_19`"]
pub type EDREQ_19_R = crate::R<bool, EDREQ_19_A>;
impl EDREQ_19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_19_A {
        match self.bits {
            false => EDREQ_19_A::EDREQ_19_0,
            true => EDREQ_19_A::EDREQ_19_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_19_0`"]
    #[inline(always)]
    pub fn is_edreq_19_0(&self) -> bool {
        *self == EDREQ_19_A::EDREQ_19_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_19_1`"]
    #[inline(always)]
    pub fn is_edreq_19_1(&self) -> bool {
        *self == EDREQ_19_A::EDREQ_19_1
    }
}
#[doc = "Write proxy for field `EDREQ_19`"]
pub struct EDREQ_19_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 19"]
    #[inline(always)]
    pub fn edreq_19_0(self) -> &'a mut W {
        self.variant(EDREQ_19_A::EDREQ_19_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 19"]
    #[inline(always)]
    pub fn edreq_19_1(self) -> &'a mut W {
        self.variant(EDREQ_19_A::EDREQ_19_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_20_A {
    #[doc = "0: Disable asynchronous DMA request for channel 20"]
    EDREQ_20_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 20"]
    EDREQ_20_1 = 1,
}
impl From<EDREQ_20_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_20`"]
pub type EDREQ_20_R = crate::R<bool, EDREQ_20_A>;
impl EDREQ_20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_20_A {
        match self.bits {
            false => EDREQ_20_A::EDREQ_20_0,
            true => EDREQ_20_A::EDREQ_20_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_20_0`"]
    #[inline(always)]
    pub fn is_edreq_20_0(&self) -> bool {
        *self == EDREQ_20_A::EDREQ_20_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_20_1`"]
    #[inline(always)]
    pub fn is_edreq_20_1(&self) -> bool {
        *self == EDREQ_20_A::EDREQ_20_1
    }
}
#[doc = "Write proxy for field `EDREQ_20`"]
pub struct EDREQ_20_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 20"]
    #[inline(always)]
    pub fn edreq_20_0(self) -> &'a mut W {
        self.variant(EDREQ_20_A::EDREQ_20_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 20"]
    #[inline(always)]
    pub fn edreq_20_1(self) -> &'a mut W {
        self.variant(EDREQ_20_A::EDREQ_20_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_21_A {
    #[doc = "0: Disable asynchronous DMA request for channel 21"]
    EDREQ_21_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 21"]
    EDREQ_21_1 = 1,
}
impl From<EDREQ_21_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_21`"]
pub type EDREQ_21_R = crate::R<bool, EDREQ_21_A>;
impl EDREQ_21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_21_A {
        match self.bits {
            false => EDREQ_21_A::EDREQ_21_0,
            true => EDREQ_21_A::EDREQ_21_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_21_0`"]
    #[inline(always)]
    pub fn is_edreq_21_0(&self) -> bool {
        *self == EDREQ_21_A::EDREQ_21_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_21_1`"]
    #[inline(always)]
    pub fn is_edreq_21_1(&self) -> bool {
        *self == EDREQ_21_A::EDREQ_21_1
    }
}
#[doc = "Write proxy for field `EDREQ_21`"]
pub struct EDREQ_21_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 21"]
    #[inline(always)]
    pub fn edreq_21_0(self) -> &'a mut W {
        self.variant(EDREQ_21_A::EDREQ_21_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 21"]
    #[inline(always)]
    pub fn edreq_21_1(self) -> &'a mut W {
        self.variant(EDREQ_21_A::EDREQ_21_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_22_A {
    #[doc = "0: Disable asynchronous DMA request for channel 22"]
    EDREQ_22_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 22"]
    EDREQ_22_1 = 1,
}
impl From<EDREQ_22_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_22`"]
pub type EDREQ_22_R = crate::R<bool, EDREQ_22_A>;
impl EDREQ_22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_22_A {
        match self.bits {
            false => EDREQ_22_A::EDREQ_22_0,
            true => EDREQ_22_A::EDREQ_22_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_22_0`"]
    #[inline(always)]
    pub fn is_edreq_22_0(&self) -> bool {
        *self == EDREQ_22_A::EDREQ_22_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_22_1`"]
    #[inline(always)]
    pub fn is_edreq_22_1(&self) -> bool {
        *self == EDREQ_22_A::EDREQ_22_1
    }
}
#[doc = "Write proxy for field `EDREQ_22`"]
pub struct EDREQ_22_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 22"]
    #[inline(always)]
    pub fn edreq_22_0(self) -> &'a mut W {
        self.variant(EDREQ_22_A::EDREQ_22_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 22"]
    #[inline(always)]
    pub fn edreq_22_1(self) -> &'a mut W {
        self.variant(EDREQ_22_A::EDREQ_22_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_23_A {
    #[doc = "0: Disable asynchronous DMA request for channel 23"]
    EDREQ_23_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 23"]
    EDREQ_23_1 = 1,
}
impl From<EDREQ_23_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_23`"]
pub type EDREQ_23_R = crate::R<bool, EDREQ_23_A>;
impl EDREQ_23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_23_A {
        match self.bits {
            false => EDREQ_23_A::EDREQ_23_0,
            true => EDREQ_23_A::EDREQ_23_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_23_0`"]
    #[inline(always)]
    pub fn is_edreq_23_0(&self) -> bool {
        *self == EDREQ_23_A::EDREQ_23_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_23_1`"]
    #[inline(always)]
    pub fn is_edreq_23_1(&self) -> bool {
        *self == EDREQ_23_A::EDREQ_23_1
    }
}
#[doc = "Write proxy for field `EDREQ_23`"]
pub struct EDREQ_23_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 23"]
    #[inline(always)]
    pub fn edreq_23_0(self) -> &'a mut W {
        self.variant(EDREQ_23_A::EDREQ_23_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 23"]
    #[inline(always)]
    pub fn edreq_23_1(self) -> &'a mut W {
        self.variant(EDREQ_23_A::EDREQ_23_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_24_A {
    #[doc = "0: Disable asynchronous DMA request for channel 24"]
    EDREQ_24_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 24"]
    EDREQ_24_1 = 1,
}
impl From<EDREQ_24_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_24`"]
pub type EDREQ_24_R = crate::R<bool, EDREQ_24_A>;
impl EDREQ_24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_24_A {
        match self.bits {
            false => EDREQ_24_A::EDREQ_24_0,
            true => EDREQ_24_A::EDREQ_24_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_24_0`"]
    #[inline(always)]
    pub fn is_edreq_24_0(&self) -> bool {
        *self == EDREQ_24_A::EDREQ_24_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_24_1`"]
    #[inline(always)]
    pub fn is_edreq_24_1(&self) -> bool {
        *self == EDREQ_24_A::EDREQ_24_1
    }
}
#[doc = "Write proxy for field `EDREQ_24`"]
pub struct EDREQ_24_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 24"]
    #[inline(always)]
    pub fn edreq_24_0(self) -> &'a mut W {
        self.variant(EDREQ_24_A::EDREQ_24_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 24"]
    #[inline(always)]
    pub fn edreq_24_1(self) -> &'a mut W {
        self.variant(EDREQ_24_A::EDREQ_24_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_25_A {
    #[doc = "0: Disable asynchronous DMA request for channel 25"]
    EDREQ_25_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 25"]
    EDREQ_25_1 = 1,
}
impl From<EDREQ_25_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_25`"]
pub type EDREQ_25_R = crate::R<bool, EDREQ_25_A>;
impl EDREQ_25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_25_A {
        match self.bits {
            false => EDREQ_25_A::EDREQ_25_0,
            true => EDREQ_25_A::EDREQ_25_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_25_0`"]
    #[inline(always)]
    pub fn is_edreq_25_0(&self) -> bool {
        *self == EDREQ_25_A::EDREQ_25_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_25_1`"]
    #[inline(always)]
    pub fn is_edreq_25_1(&self) -> bool {
        *self == EDREQ_25_A::EDREQ_25_1
    }
}
#[doc = "Write proxy for field `EDREQ_25`"]
pub struct EDREQ_25_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 25"]
    #[inline(always)]
    pub fn edreq_25_0(self) -> &'a mut W {
        self.variant(EDREQ_25_A::EDREQ_25_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 25"]
    #[inline(always)]
    pub fn edreq_25_1(self) -> &'a mut W {
        self.variant(EDREQ_25_A::EDREQ_25_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_26_A {
    #[doc = "0: Disable asynchronous DMA request for channel 26"]
    EDREQ_26_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 26"]
    EDREQ_26_1 = 1,
}
impl From<EDREQ_26_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_26`"]
pub type EDREQ_26_R = crate::R<bool, EDREQ_26_A>;
impl EDREQ_26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_26_A {
        match self.bits {
            false => EDREQ_26_A::EDREQ_26_0,
            true => EDREQ_26_A::EDREQ_26_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_26_0`"]
    #[inline(always)]
    pub fn is_edreq_26_0(&self) -> bool {
        *self == EDREQ_26_A::EDREQ_26_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_26_1`"]
    #[inline(always)]
    pub fn is_edreq_26_1(&self) -> bool {
        *self == EDREQ_26_A::EDREQ_26_1
    }
}
#[doc = "Write proxy for field `EDREQ_26`"]
pub struct EDREQ_26_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 26"]
    #[inline(always)]
    pub fn edreq_26_0(self) -> &'a mut W {
        self.variant(EDREQ_26_A::EDREQ_26_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 26"]
    #[inline(always)]
    pub fn edreq_26_1(self) -> &'a mut W {
        self.variant(EDREQ_26_A::EDREQ_26_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_27_A {
    #[doc = "0: Disable asynchronous DMA request for channel 27"]
    EDREQ_27_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 27"]
    EDREQ_27_1 = 1,
}
impl From<EDREQ_27_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_27`"]
pub type EDREQ_27_R = crate::R<bool, EDREQ_27_A>;
impl EDREQ_27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_27_A {
        match self.bits {
            false => EDREQ_27_A::EDREQ_27_0,
            true => EDREQ_27_A::EDREQ_27_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_27_0`"]
    #[inline(always)]
    pub fn is_edreq_27_0(&self) -> bool {
        *self == EDREQ_27_A::EDREQ_27_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_27_1`"]
    #[inline(always)]
    pub fn is_edreq_27_1(&self) -> bool {
        *self == EDREQ_27_A::EDREQ_27_1
    }
}
#[doc = "Write proxy for field `EDREQ_27`"]
pub struct EDREQ_27_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 27"]
    #[inline(always)]
    pub fn edreq_27_0(self) -> &'a mut W {
        self.variant(EDREQ_27_A::EDREQ_27_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 27"]
    #[inline(always)]
    pub fn edreq_27_1(self) -> &'a mut W {
        self.variant(EDREQ_27_A::EDREQ_27_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_28_A {
    #[doc = "0: Disable asynchronous DMA request for channel 28"]
    EDREQ_28_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 28"]
    EDREQ_28_1 = 1,
}
impl From<EDREQ_28_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_28`"]
pub type EDREQ_28_R = crate::R<bool, EDREQ_28_A>;
impl EDREQ_28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_28_A {
        match self.bits {
            false => EDREQ_28_A::EDREQ_28_0,
            true => EDREQ_28_A::EDREQ_28_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_28_0`"]
    #[inline(always)]
    pub fn is_edreq_28_0(&self) -> bool {
        *self == EDREQ_28_A::EDREQ_28_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_28_1`"]
    #[inline(always)]
    pub fn is_edreq_28_1(&self) -> bool {
        *self == EDREQ_28_A::EDREQ_28_1
    }
}
#[doc = "Write proxy for field `EDREQ_28`"]
pub struct EDREQ_28_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 28"]
    #[inline(always)]
    pub fn edreq_28_0(self) -> &'a mut W {
        self.variant(EDREQ_28_A::EDREQ_28_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 28"]
    #[inline(always)]
    pub fn edreq_28_1(self) -> &'a mut W {
        self.variant(EDREQ_28_A::EDREQ_28_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_29_A {
    #[doc = "0: Disable asynchronous DMA request for channel 29"]
    EDREQ_29_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 29"]
    EDREQ_29_1 = 1,
}
impl From<EDREQ_29_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_29`"]
pub type EDREQ_29_R = crate::R<bool, EDREQ_29_A>;
impl EDREQ_29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_29_A {
        match self.bits {
            false => EDREQ_29_A::EDREQ_29_0,
            true => EDREQ_29_A::EDREQ_29_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_29_0`"]
    #[inline(always)]
    pub fn is_edreq_29_0(&self) -> bool {
        *self == EDREQ_29_A::EDREQ_29_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_29_1`"]
    #[inline(always)]
    pub fn is_edreq_29_1(&self) -> bool {
        *self == EDREQ_29_A::EDREQ_29_1
    }
}
#[doc = "Write proxy for field `EDREQ_29`"]
pub struct EDREQ_29_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 29"]
    #[inline(always)]
    pub fn edreq_29_0(self) -> &'a mut W {
        self.variant(EDREQ_29_A::EDREQ_29_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 29"]
    #[inline(always)]
    pub fn edreq_29_1(self) -> &'a mut W {
        self.variant(EDREQ_29_A::EDREQ_29_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Enable asynchronous DMA request in stop mode for channel 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_30_A {
    #[doc = "0: Disable asynchronous DMA request for channel 30"]
    EDREQ_30_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 30"]
    EDREQ_30_1 = 1,
}
impl From<EDREQ_30_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_30`"]
pub type EDREQ_30_R = crate::R<bool, EDREQ_30_A>;
impl EDREQ_30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_30_A {
        match self.bits {
            false => EDREQ_30_A::EDREQ_30_0,
            true => EDREQ_30_A::EDREQ_30_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_30_0`"]
    #[inline(always)]
    pub fn is_edreq_30_0(&self) -> bool {
        *self == EDREQ_30_A::EDREQ_30_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_30_1`"]
    #[inline(always)]
    pub fn is_edreq_30_1(&self) -> bool {
        *self == EDREQ_30_A::EDREQ_30_1
    }
}
#[doc = "Write proxy for field `EDREQ_30`"]
pub struct EDREQ_30_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 30"]
    #[inline(always)]
    pub fn edreq_30_0(self) -> &'a mut W {
        self.variant(EDREQ_30_A::EDREQ_30_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 30"]
    #[inline(always)]
    pub fn edreq_30_1(self) -> &'a mut W {
        self.variant(EDREQ_30_A::EDREQ_30_1)
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
#[doc = "Enable asynchronous DMA request in stop mode for channel 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_31_A {
    #[doc = "0: Disable asynchronous DMA request for channel 31"]
    EDREQ_31_0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 31"]
    EDREQ_31_1 = 1,
}
impl From<EDREQ_31_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDREQ_31`"]
pub type EDREQ_31_R = crate::R<bool, EDREQ_31_A>;
impl EDREQ_31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_31_A {
        match self.bits {
            false => EDREQ_31_A::EDREQ_31_0,
            true => EDREQ_31_A::EDREQ_31_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDREQ_31_0`"]
    #[inline(always)]
    pub fn is_edreq_31_0(&self) -> bool {
        *self == EDREQ_31_A::EDREQ_31_0
    }
    #[doc = "Checks if the value of the field is `EDREQ_31_1`"]
    #[inline(always)]
    pub fn is_edreq_31_1(&self) -> bool {
        *self == EDREQ_31_A::EDREQ_31_1
    }
}
#[doc = "Write proxy for field `EDREQ_31`"]
pub struct EDREQ_31_W<'a> {
    w: &'a mut W,
}
impl<'a> EDREQ_31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDREQ_31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 31"]
    #[inline(always)]
    pub fn edreq_31_0(self) -> &'a mut W {
        self.variant(EDREQ_31_A::EDREQ_31_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 31"]
    #[inline(always)]
    pub fn edreq_31_1(self) -> &'a mut W {
        self.variant(EDREQ_31_A::EDREQ_31_1)
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
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub fn edreq_0(&self) -> EDREQ_0_R {
        EDREQ_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    pub fn edreq_1(&self) -> EDREQ_1_R {
        EDREQ_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    pub fn edreq_2(&self) -> EDREQ_2_R {
        EDREQ_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    pub fn edreq_3(&self) -> EDREQ_3_R {
        EDREQ_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
    #[inline(always)]
    pub fn edreq_4(&self) -> EDREQ_4_R {
        EDREQ_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
    #[inline(always)]
    pub fn edreq_5(&self) -> EDREQ_5_R {
        EDREQ_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
    #[inline(always)]
    pub fn edreq_6(&self) -> EDREQ_6_R {
        EDREQ_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
    #[inline(always)]
    pub fn edreq_7(&self) -> EDREQ_7_R {
        EDREQ_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8"]
    #[inline(always)]
    pub fn edreq_8(&self) -> EDREQ_8_R {
        EDREQ_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9"]
    #[inline(always)]
    pub fn edreq_9(&self) -> EDREQ_9_R {
        EDREQ_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10"]
    #[inline(always)]
    pub fn edreq_10(&self) -> EDREQ_10_R {
        EDREQ_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11"]
    #[inline(always)]
    pub fn edreq_11(&self) -> EDREQ_11_R {
        EDREQ_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12"]
    #[inline(always)]
    pub fn edreq_12(&self) -> EDREQ_12_R {
        EDREQ_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13"]
    #[inline(always)]
    pub fn edreq_13(&self) -> EDREQ_13_R {
        EDREQ_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14"]
    #[inline(always)]
    pub fn edreq_14(&self) -> EDREQ_14_R {
        EDREQ_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15"]
    #[inline(always)]
    pub fn edreq_15(&self) -> EDREQ_15_R {
        EDREQ_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable asynchronous DMA request in stop mode for channel 16"]
    #[inline(always)]
    pub fn edreq_16(&self) -> EDREQ_16_R {
        EDREQ_16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable asynchronous DMA request in stop mode for channel 17"]
    #[inline(always)]
    pub fn edreq_17(&self) -> EDREQ_17_R {
        EDREQ_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable asynchronous DMA request in stop mode for channel 18"]
    #[inline(always)]
    pub fn edreq_18(&self) -> EDREQ_18_R {
        EDREQ_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable asynchronous DMA request in stop mode for channel 19"]
    #[inline(always)]
    pub fn edreq_19(&self) -> EDREQ_19_R {
        EDREQ_19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable asynchronous DMA request in stop mode for channel 20"]
    #[inline(always)]
    pub fn edreq_20(&self) -> EDREQ_20_R {
        EDREQ_20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable asynchronous DMA request in stop mode for channel 21"]
    #[inline(always)]
    pub fn edreq_21(&self) -> EDREQ_21_R {
        EDREQ_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable asynchronous DMA request in stop mode for channel 22"]
    #[inline(always)]
    pub fn edreq_22(&self) -> EDREQ_22_R {
        EDREQ_22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable asynchronous DMA request in stop mode for channel 23"]
    #[inline(always)]
    pub fn edreq_23(&self) -> EDREQ_23_R {
        EDREQ_23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable asynchronous DMA request in stop mode for channel 24"]
    #[inline(always)]
    pub fn edreq_24(&self) -> EDREQ_24_R {
        EDREQ_24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable asynchronous DMA request in stop mode for channel 25"]
    #[inline(always)]
    pub fn edreq_25(&self) -> EDREQ_25_R {
        EDREQ_25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable asynchronous DMA request in stop mode for channel 26"]
    #[inline(always)]
    pub fn edreq_26(&self) -> EDREQ_26_R {
        EDREQ_26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable asynchronous DMA request in stop mode for channel 27"]
    #[inline(always)]
    pub fn edreq_27(&self) -> EDREQ_27_R {
        EDREQ_27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable asynchronous DMA request in stop mode for channel 28"]
    #[inline(always)]
    pub fn edreq_28(&self) -> EDREQ_28_R {
        EDREQ_28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable asynchronous DMA request in stop mode for channel 29"]
    #[inline(always)]
    pub fn edreq_29(&self) -> EDREQ_29_R {
        EDREQ_29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable asynchronous DMA request in stop mode for channel 30"]
    #[inline(always)]
    pub fn edreq_30(&self) -> EDREQ_30_R {
        EDREQ_30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable asynchronous DMA request in stop mode for channel 31"]
    #[inline(always)]
    pub fn edreq_31(&self) -> EDREQ_31_R {
        EDREQ_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub fn edreq_0(&mut self) -> EDREQ_0_W {
        EDREQ_0_W { w: self }
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    pub fn edreq_1(&mut self) -> EDREQ_1_W {
        EDREQ_1_W { w: self }
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    pub fn edreq_2(&mut self) -> EDREQ_2_W {
        EDREQ_2_W { w: self }
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    pub fn edreq_3(&mut self) -> EDREQ_3_W {
        EDREQ_3_W { w: self }
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
    #[inline(always)]
    pub fn edreq_4(&mut self) -> EDREQ_4_W {
        EDREQ_4_W { w: self }
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
    #[inline(always)]
    pub fn edreq_5(&mut self) -> EDREQ_5_W {
        EDREQ_5_W { w: self }
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
    #[inline(always)]
    pub fn edreq_6(&mut self) -> EDREQ_6_W {
        EDREQ_6_W { w: self }
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
    #[inline(always)]
    pub fn edreq_7(&mut self) -> EDREQ_7_W {
        EDREQ_7_W { w: self }
    }
    #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8"]
    #[inline(always)]
    pub fn edreq_8(&mut self) -> EDREQ_8_W {
        EDREQ_8_W { w: self }
    }
    #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9"]
    #[inline(always)]
    pub fn edreq_9(&mut self) -> EDREQ_9_W {
        EDREQ_9_W { w: self }
    }
    #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10"]
    #[inline(always)]
    pub fn edreq_10(&mut self) -> EDREQ_10_W {
        EDREQ_10_W { w: self }
    }
    #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11"]
    #[inline(always)]
    pub fn edreq_11(&mut self) -> EDREQ_11_W {
        EDREQ_11_W { w: self }
    }
    #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12"]
    #[inline(always)]
    pub fn edreq_12(&mut self) -> EDREQ_12_W {
        EDREQ_12_W { w: self }
    }
    #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13"]
    #[inline(always)]
    pub fn edreq_13(&mut self) -> EDREQ_13_W {
        EDREQ_13_W { w: self }
    }
    #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14"]
    #[inline(always)]
    pub fn edreq_14(&mut self) -> EDREQ_14_W {
        EDREQ_14_W { w: self }
    }
    #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15"]
    #[inline(always)]
    pub fn edreq_15(&mut self) -> EDREQ_15_W {
        EDREQ_15_W { w: self }
    }
    #[doc = "Bit 16 - Enable asynchronous DMA request in stop mode for channel 16"]
    #[inline(always)]
    pub fn edreq_16(&mut self) -> EDREQ_16_W {
        EDREQ_16_W { w: self }
    }
    #[doc = "Bit 17 - Enable asynchronous DMA request in stop mode for channel 17"]
    #[inline(always)]
    pub fn edreq_17(&mut self) -> EDREQ_17_W {
        EDREQ_17_W { w: self }
    }
    #[doc = "Bit 18 - Enable asynchronous DMA request in stop mode for channel 18"]
    #[inline(always)]
    pub fn edreq_18(&mut self) -> EDREQ_18_W {
        EDREQ_18_W { w: self }
    }
    #[doc = "Bit 19 - Enable asynchronous DMA request in stop mode for channel 19"]
    #[inline(always)]
    pub fn edreq_19(&mut self) -> EDREQ_19_W {
        EDREQ_19_W { w: self }
    }
    #[doc = "Bit 20 - Enable asynchronous DMA request in stop mode for channel 20"]
    #[inline(always)]
    pub fn edreq_20(&mut self) -> EDREQ_20_W {
        EDREQ_20_W { w: self }
    }
    #[doc = "Bit 21 - Enable asynchronous DMA request in stop mode for channel 21"]
    #[inline(always)]
    pub fn edreq_21(&mut self) -> EDREQ_21_W {
        EDREQ_21_W { w: self }
    }
    #[doc = "Bit 22 - Enable asynchronous DMA request in stop mode for channel 22"]
    #[inline(always)]
    pub fn edreq_22(&mut self) -> EDREQ_22_W {
        EDREQ_22_W { w: self }
    }
    #[doc = "Bit 23 - Enable asynchronous DMA request in stop mode for channel 23"]
    #[inline(always)]
    pub fn edreq_23(&mut self) -> EDREQ_23_W {
        EDREQ_23_W { w: self }
    }
    #[doc = "Bit 24 - Enable asynchronous DMA request in stop mode for channel 24"]
    #[inline(always)]
    pub fn edreq_24(&mut self) -> EDREQ_24_W {
        EDREQ_24_W { w: self }
    }
    #[doc = "Bit 25 - Enable asynchronous DMA request in stop mode for channel 25"]
    #[inline(always)]
    pub fn edreq_25(&mut self) -> EDREQ_25_W {
        EDREQ_25_W { w: self }
    }
    #[doc = "Bit 26 - Enable asynchronous DMA request in stop mode for channel 26"]
    #[inline(always)]
    pub fn edreq_26(&mut self) -> EDREQ_26_W {
        EDREQ_26_W { w: self }
    }
    #[doc = "Bit 27 - Enable asynchronous DMA request in stop mode for channel 27"]
    #[inline(always)]
    pub fn edreq_27(&mut self) -> EDREQ_27_W {
        EDREQ_27_W { w: self }
    }
    #[doc = "Bit 28 - Enable asynchronous DMA request in stop mode for channel 28"]
    #[inline(always)]
    pub fn edreq_28(&mut self) -> EDREQ_28_W {
        EDREQ_28_W { w: self }
    }
    #[doc = "Bit 29 - Enable asynchronous DMA request in stop mode for channel 29"]
    #[inline(always)]
    pub fn edreq_29(&mut self) -> EDREQ_29_W {
        EDREQ_29_W { w: self }
    }
    #[doc = "Bit 30 - Enable asynchronous DMA request in stop mode for channel 30"]
    #[inline(always)]
    pub fn edreq_30(&mut self) -> EDREQ_30_W {
        EDREQ_30_W { w: self }
    }
    #[doc = "Bit 31 - Enable asynchronous DMA request in stop mode for channel 31"]
    #[inline(always)]
    pub fn edreq_31(&mut self) -> EDREQ_31_W {
        EDREQ_31_W { w: self }
    }
}
