#[doc = "Reader of register GPR4"]
pub type R = crate::R<u32, super::GPR4>;
#[doc = "Writer for register GPR4"]
pub type W = crate::W<u32, super::GPR4>;
#[doc = "Register GPR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EDMA stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDMA_STOP_REQ_A {
    #[doc = "0: stop request off"]
    EDMA_STOP_REQ_0,
    #[doc = "1: stop request on"]
    EDMA_STOP_REQ_1,
}
impl From<EDMA_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: EDMA_STOP_REQ_A) -> Self {
        match variant {
            EDMA_STOP_REQ_A::EDMA_STOP_REQ_0 => false,
            EDMA_STOP_REQ_A::EDMA_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `EDMA_STOP_REQ`"]
pub type EDMA_STOP_REQ_R = crate::R<bool, EDMA_STOP_REQ_A>;
impl EDMA_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDMA_STOP_REQ_A {
        match self.bits {
            false => EDMA_STOP_REQ_A::EDMA_STOP_REQ_0,
            true => EDMA_STOP_REQ_A::EDMA_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_edma_stop_req_0(&self) -> bool {
        *self == EDMA_STOP_REQ_A::EDMA_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_edma_stop_req_1(&self) -> bool {
        *self == EDMA_STOP_REQ_A::EDMA_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `EDMA_STOP_REQ`"]
pub struct EDMA_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EDMA_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDMA_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn edma_stop_req_0(self) -> &'a mut W {
        self.variant(EDMA_STOP_REQ_A::EDMA_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn edma_stop_req_1(self) -> &'a mut W {
        self.variant(EDMA_STOP_REQ_A::EDMA_STOP_REQ_1)
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
#[doc = "CAN1 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    CAN1_STOP_REQ_0,
    #[doc = "1: stop request on"]
    CAN1_STOP_REQ_1,
}
impl From<CAN1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: CAN1_STOP_REQ_A) -> Self {
        match variant {
            CAN1_STOP_REQ_A::CAN1_STOP_REQ_0 => false,
            CAN1_STOP_REQ_A::CAN1_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `CAN1_STOP_REQ`"]
pub type CAN1_STOP_REQ_R = crate::R<bool, CAN1_STOP_REQ_A>;
impl CAN1_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN1_STOP_REQ_A {
        match self.bits {
            false => CAN1_STOP_REQ_A::CAN1_STOP_REQ_0,
            true => CAN1_STOP_REQ_A::CAN1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_can1_stop_req_0(&self) -> bool {
        *self == CAN1_STOP_REQ_A::CAN1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_can1_stop_req_1(&self) -> bool {
        *self == CAN1_STOP_REQ_A::CAN1_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `CAN1_STOP_REQ`"]
pub struct CAN1_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN1_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn can1_stop_req_0(self) -> &'a mut W {
        self.variant(CAN1_STOP_REQ_A::CAN1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn can1_stop_req_1(self) -> &'a mut W {
        self.variant(CAN1_STOP_REQ_A::CAN1_STOP_REQ_1)
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
#[doc = "CAN2 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    CAN2_STOP_REQ_0,
    #[doc = "1: stop request on"]
    CAN2_STOP_REQ_1,
}
impl From<CAN2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: CAN2_STOP_REQ_A) -> Self {
        match variant {
            CAN2_STOP_REQ_A::CAN2_STOP_REQ_0 => false,
            CAN2_STOP_REQ_A::CAN2_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `CAN2_STOP_REQ`"]
pub type CAN2_STOP_REQ_R = crate::R<bool, CAN2_STOP_REQ_A>;
impl CAN2_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN2_STOP_REQ_A {
        match self.bits {
            false => CAN2_STOP_REQ_A::CAN2_STOP_REQ_0,
            true => CAN2_STOP_REQ_A::CAN2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_can2_stop_req_0(&self) -> bool {
        *self == CAN2_STOP_REQ_A::CAN2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_can2_stop_req_1(&self) -> bool {
        *self == CAN2_STOP_REQ_A::CAN2_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `CAN2_STOP_REQ`"]
pub struct CAN2_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN2_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn can2_stop_req_0(self) -> &'a mut W {
        self.variant(CAN2_STOP_REQ_A::CAN2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn can2_stop_req_1(self) -> &'a mut W {
        self.variant(CAN2_STOP_REQ_A::CAN2_STOP_REQ_1)
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
#[doc = "TRNG stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG_STOP_REQ_A {
    #[doc = "0: stop request off"]
    TRNG_STOP_REQ_0,
    #[doc = "1: stop request on"]
    TRNG_STOP_REQ_1,
}
impl From<TRNG_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_STOP_REQ_A) -> Self {
        match variant {
            TRNG_STOP_REQ_A::TRNG_STOP_REQ_0 => false,
            TRNG_STOP_REQ_A::TRNG_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `TRNG_STOP_REQ`"]
pub type TRNG_STOP_REQ_R = crate::R<bool, TRNG_STOP_REQ_A>;
impl TRNG_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_STOP_REQ_A {
        match self.bits {
            false => TRNG_STOP_REQ_A::TRNG_STOP_REQ_0,
            true => TRNG_STOP_REQ_A::TRNG_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_trng_stop_req_0(&self) -> bool {
        *self == TRNG_STOP_REQ_A::TRNG_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_trng_stop_req_1(&self) -> bool {
        *self == TRNG_STOP_REQ_A::TRNG_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `TRNG_STOP_REQ`"]
pub struct TRNG_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRNG_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn trng_stop_req_0(self) -> &'a mut W {
        self.variant(TRNG_STOP_REQ_A::TRNG_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn trng_stop_req_1(self) -> &'a mut W {
        self.variant(TRNG_STOP_REQ_A::TRNG_STOP_REQ_1)
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
#[doc = "ENET stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_STOP_REQ_A {
    #[doc = "0: stop request off"]
    ENET_STOP_REQ_0,
    #[doc = "1: stop request on"]
    ENET_STOP_REQ_1,
}
impl From<ENET_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_STOP_REQ_A) -> Self {
        match variant {
            ENET_STOP_REQ_A::ENET_STOP_REQ_0 => false,
            ENET_STOP_REQ_A::ENET_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `ENET_STOP_REQ`"]
pub type ENET_STOP_REQ_R = crate::R<bool, ENET_STOP_REQ_A>;
impl ENET_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_STOP_REQ_A {
        match self.bits {
            false => ENET_STOP_REQ_A::ENET_STOP_REQ_0,
            true => ENET_STOP_REQ_A::ENET_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_enet_stop_req_0(&self) -> bool {
        *self == ENET_STOP_REQ_A::ENET_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_enet_stop_req_1(&self) -> bool {
        *self == ENET_STOP_REQ_A::ENET_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `ENET_STOP_REQ`"]
pub struct ENET_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn enet_stop_req_0(self) -> &'a mut W {
        self.variant(ENET_STOP_REQ_A::ENET_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn enet_stop_req_1(self) -> &'a mut W {
        self.variant(ENET_STOP_REQ_A::ENET_STOP_REQ_1)
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
#[doc = "SAI1 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    SAI1_STOP_REQ_0,
    #[doc = "1: stop request on"]
    SAI1_STOP_REQ_1,
}
impl From<SAI1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1_STOP_REQ_A) -> Self {
        match variant {
            SAI1_STOP_REQ_A::SAI1_STOP_REQ_0 => false,
            SAI1_STOP_REQ_A::SAI1_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `SAI1_STOP_REQ`"]
pub type SAI1_STOP_REQ_R = crate::R<bool, SAI1_STOP_REQ_A>;
impl SAI1_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1_STOP_REQ_A {
        match self.bits {
            false => SAI1_STOP_REQ_A::SAI1_STOP_REQ_0,
            true => SAI1_STOP_REQ_A::SAI1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_sai1_stop_req_0(&self) -> bool {
        *self == SAI1_STOP_REQ_A::SAI1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_sai1_stop_req_1(&self) -> bool {
        *self == SAI1_STOP_REQ_A::SAI1_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `SAI1_STOP_REQ`"]
pub struct SAI1_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn sai1_stop_req_0(self) -> &'a mut W {
        self.variant(SAI1_STOP_REQ_A::SAI1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn sai1_stop_req_1(self) -> &'a mut W {
        self.variant(SAI1_STOP_REQ_A::SAI1_STOP_REQ_1)
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
#[doc = "SAI2 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    SAI2_STOP_REQ_0,
    #[doc = "1: stop request on"]
    SAI2_STOP_REQ_1,
}
impl From<SAI2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2_STOP_REQ_A) -> Self {
        match variant {
            SAI2_STOP_REQ_A::SAI2_STOP_REQ_0 => false,
            SAI2_STOP_REQ_A::SAI2_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `SAI2_STOP_REQ`"]
pub type SAI2_STOP_REQ_R = crate::R<bool, SAI2_STOP_REQ_A>;
impl SAI2_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_STOP_REQ_A {
        match self.bits {
            false => SAI2_STOP_REQ_A::SAI2_STOP_REQ_0,
            true => SAI2_STOP_REQ_A::SAI2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_sai2_stop_req_0(&self) -> bool {
        *self == SAI2_STOP_REQ_A::SAI2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_sai2_stop_req_1(&self) -> bool {
        *self == SAI2_STOP_REQ_A::SAI2_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `SAI2_STOP_REQ`"]
pub struct SAI2_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn sai2_stop_req_0(self) -> &'a mut W {
        self.variant(SAI2_STOP_REQ_A::SAI2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn sai2_stop_req_1(self) -> &'a mut W {
        self.variant(SAI2_STOP_REQ_A::SAI2_STOP_REQ_1)
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
#[doc = "SAI3 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    SAI3_STOP_REQ_0,
    #[doc = "1: stop request on"]
    SAI3_STOP_REQ_1,
}
impl From<SAI3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3_STOP_REQ_A) -> Self {
        match variant {
            SAI3_STOP_REQ_A::SAI3_STOP_REQ_0 => false,
            SAI3_STOP_REQ_A::SAI3_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `SAI3_STOP_REQ`"]
pub type SAI3_STOP_REQ_R = crate::R<bool, SAI3_STOP_REQ_A>;
impl SAI3_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3_STOP_REQ_A {
        match self.bits {
            false => SAI3_STOP_REQ_A::SAI3_STOP_REQ_0,
            true => SAI3_STOP_REQ_A::SAI3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_sai3_stop_req_0(&self) -> bool {
        *self == SAI3_STOP_REQ_A::SAI3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_sai3_stop_req_1(&self) -> bool {
        *self == SAI3_STOP_REQ_A::SAI3_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `SAI3_STOP_REQ`"]
pub struct SAI3_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI3_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn sai3_stop_req_0(self) -> &'a mut W {
        self.variant(SAI3_STOP_REQ_A::SAI3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn sai3_stop_req_1(self) -> &'a mut W {
        self.variant(SAI3_STOP_REQ_A::SAI3_STOP_REQ_1)
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
#[doc = "ENET2 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    ENET2_STOP_REQ_0,
    #[doc = "1: stop request on"]
    ENET2_STOP_REQ_1,
}
impl From<ENET2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: ENET2_STOP_REQ_A) -> Self {
        match variant {
            ENET2_STOP_REQ_A::ENET2_STOP_REQ_0 => false,
            ENET2_STOP_REQ_A::ENET2_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `ENET2_STOP_REQ`"]
pub type ENET2_STOP_REQ_R = crate::R<bool, ENET2_STOP_REQ_A>;
impl ENET2_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET2_STOP_REQ_A {
        match self.bits {
            false => ENET2_STOP_REQ_A::ENET2_STOP_REQ_0,
            true => ENET2_STOP_REQ_A::ENET2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_enet2_stop_req_0(&self) -> bool {
        *self == ENET2_STOP_REQ_A::ENET2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `ENET2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_enet2_stop_req_1(&self) -> bool {
        *self == ENET2_STOP_REQ_A::ENET2_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `ENET2_STOP_REQ`"]
pub struct ENET2_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET2_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET2_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn enet2_stop_req_0(self) -> &'a mut W {
        self.variant(ENET2_STOP_REQ_A::ENET2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn enet2_stop_req_1(self) -> &'a mut W {
        self.variant(ENET2_STOP_REQ_A::ENET2_STOP_REQ_1)
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
#[doc = "SEMC stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_STOP_REQ_A {
    #[doc = "0: stop request off"]
    SEMC_STOP_REQ_0,
    #[doc = "1: stop request on"]
    SEMC_STOP_REQ_1,
}
impl From<SEMC_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_STOP_REQ_A) -> Self {
        match variant {
            SEMC_STOP_REQ_A::SEMC_STOP_REQ_0 => false,
            SEMC_STOP_REQ_A::SEMC_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `SEMC_STOP_REQ`"]
pub type SEMC_STOP_REQ_R = crate::R<bool, SEMC_STOP_REQ_A>;
impl SEMC_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_STOP_REQ_A {
        match self.bits {
            false => SEMC_STOP_REQ_A::SEMC_STOP_REQ_0,
            true => SEMC_STOP_REQ_A::SEMC_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_semc_stop_req_0(&self) -> bool {
        *self == SEMC_STOP_REQ_A::SEMC_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_semc_stop_req_1(&self) -> bool {
        *self == SEMC_STOP_REQ_A::SEMC_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `SEMC_STOP_REQ`"]
pub struct SEMC_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMC_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEMC_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn semc_stop_req_0(self) -> &'a mut W {
        self.variant(SEMC_STOP_REQ_A::SEMC_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn semc_stop_req_1(self) -> &'a mut W {
        self.variant(SEMC_STOP_REQ_A::SEMC_STOP_REQ_1)
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
#[doc = "PIT stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT_STOP_REQ_A {
    #[doc = "0: stop request off"]
    PIT_STOP_REQ_0,
    #[doc = "1: stop request on"]
    PIT_STOP_REQ_1,
}
impl From<PIT_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_STOP_REQ_A) -> Self {
        match variant {
            PIT_STOP_REQ_A::PIT_STOP_REQ_0 => false,
            PIT_STOP_REQ_A::PIT_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `PIT_STOP_REQ`"]
pub type PIT_STOP_REQ_R = crate::R<bool, PIT_STOP_REQ_A>;
impl PIT_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_STOP_REQ_A {
        match self.bits {
            false => PIT_STOP_REQ_A::PIT_STOP_REQ_0,
            true => PIT_STOP_REQ_A::PIT_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_pit_stop_req_0(&self) -> bool {
        *self == PIT_STOP_REQ_A::PIT_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_pit_stop_req_1(&self) -> bool {
        *self == PIT_STOP_REQ_A::PIT_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `PIT_STOP_REQ`"]
pub struct PIT_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PIT_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIT_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn pit_stop_req_0(self) -> &'a mut W {
        self.variant(PIT_STOP_REQ_A::PIT_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn pit_stop_req_1(self) -> &'a mut W {
        self.variant(PIT_STOP_REQ_A::PIT_STOP_REQ_1)
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
#[doc = "FlexSPI stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI_STOP_REQ_A {
    #[doc = "0: stop request off"]
    FLEXSPI_STOP_REQ_0,
    #[doc = "1: stop request on"]
    FLEXSPI_STOP_REQ_1,
}
impl From<FLEXSPI_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI_STOP_REQ_A) -> Self {
        match variant {
            FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_0 => false,
            FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXSPI_STOP_REQ`"]
pub type FLEXSPI_STOP_REQ_R = crate::R<bool, FLEXSPI_STOP_REQ_A>;
impl FLEXSPI_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI_STOP_REQ_A {
        match self.bits {
            false => FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_0,
            true => FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_flexspi_stop_req_0(&self) -> bool {
        *self == FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_flexspi_stop_req_1(&self) -> bool {
        *self == FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `FLEXSPI_STOP_REQ`"]
pub struct FLEXSPI_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXSPI_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXSPI_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn flexspi_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn flexspi_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_1)
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
#[doc = "FlexIO1 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    FLEXIO1_STOP_REQ_0,
    #[doc = "1: stop request on"]
    FLEXIO1_STOP_REQ_1,
}
impl From<FLEXIO1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO1_STOP_REQ_A) -> Self {
        match variant {
            FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_0 => false,
            FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXIO1_STOP_REQ`"]
pub type FLEXIO1_STOP_REQ_R = crate::R<bool, FLEXIO1_STOP_REQ_A>;
impl FLEXIO1_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_STOP_REQ_A {
        match self.bits {
            false => FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_0,
            true => FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_flexio1_stop_req_0(&self) -> bool {
        *self == FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_flexio1_stop_req_1(&self) -> bool {
        *self == FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `FLEXIO1_STOP_REQ`"]
pub struct FLEXIO1_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO1_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO1_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn flexio1_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn flexio1_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_1)
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
#[doc = "FlexIO2 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    FLEXIO2_STOP_REQ_0,
    #[doc = "1: stop request on"]
    FLEXIO2_STOP_REQ_1,
}
impl From<FLEXIO2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO2_STOP_REQ_A) -> Self {
        match variant {
            FLEXIO2_STOP_REQ_A::FLEXIO2_STOP_REQ_0 => false,
            FLEXIO2_STOP_REQ_A::FLEXIO2_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXIO2_STOP_REQ`"]
pub type FLEXIO2_STOP_REQ_R = crate::R<bool, FLEXIO2_STOP_REQ_A>;
impl FLEXIO2_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO2_STOP_REQ_A {
        match self.bits {
            false => FLEXIO2_STOP_REQ_A::FLEXIO2_STOP_REQ_0,
            true => FLEXIO2_STOP_REQ_A::FLEXIO2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_flexio2_stop_req_0(&self) -> bool {
        *self == FLEXIO2_STOP_REQ_A::FLEXIO2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_flexio2_stop_req_1(&self) -> bool {
        *self == FLEXIO2_STOP_REQ_A::FLEXIO2_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `FLEXIO2_STOP_REQ`"]
pub struct FLEXIO2_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO2_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO2_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn flexio2_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXIO2_STOP_REQ_A::FLEXIO2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn flexio2_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXIO2_STOP_REQ_A::FLEXIO2_STOP_REQ_1)
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
#[doc = "On-platform flexio3 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    FLEXIO3_STOP_REQ_0,
    #[doc = "1: stop request on"]
    FLEXIO3_STOP_REQ_1,
}
impl From<FLEXIO3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO3_STOP_REQ_A) -> Self {
        match variant {
            FLEXIO3_STOP_REQ_A::FLEXIO3_STOP_REQ_0 => false,
            FLEXIO3_STOP_REQ_A::FLEXIO3_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXIO3_STOP_REQ`"]
pub type FLEXIO3_STOP_REQ_R = crate::R<bool, FLEXIO3_STOP_REQ_A>;
impl FLEXIO3_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO3_STOP_REQ_A {
        match self.bits {
            false => FLEXIO3_STOP_REQ_A::FLEXIO3_STOP_REQ_0,
            true => FLEXIO3_STOP_REQ_A::FLEXIO3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_flexio3_stop_req_0(&self) -> bool {
        *self == FLEXIO3_STOP_REQ_A::FLEXIO3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_flexio3_stop_req_1(&self) -> bool {
        *self == FLEXIO3_STOP_REQ_A::FLEXIO3_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `FLEXIO3_STOP_REQ`"]
pub struct FLEXIO3_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXIO3_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXIO3_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn flexio3_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXIO3_STOP_REQ_A::FLEXIO3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn flexio3_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXIO3_STOP_REQ_A::FLEXIO3_STOP_REQ_1)
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
#[doc = "FlexSPI2 stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    FLEXSPI2_STOP_REQ_0,
    #[doc = "1: stop request on"]
    FLEXSPI2_STOP_REQ_1,
}
impl From<FLEXSPI2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI2_STOP_REQ_A) -> Self {
        match variant {
            FLEXSPI2_STOP_REQ_A::FLEXSPI2_STOP_REQ_0 => false,
            FLEXSPI2_STOP_REQ_A::FLEXSPI2_STOP_REQ_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXSPI2_STOP_REQ`"]
pub type FLEXSPI2_STOP_REQ_R = crate::R<bool, FLEXSPI2_STOP_REQ_A>;
impl FLEXSPI2_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI2_STOP_REQ_A {
        match self.bits {
            false => FLEXSPI2_STOP_REQ_A::FLEXSPI2_STOP_REQ_0,
            true => FLEXSPI2_STOP_REQ_A::FLEXSPI2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_flexspi2_stop_req_0(&self) -> bool {
        *self == FLEXSPI2_STOP_REQ_A::FLEXSPI2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_flexspi2_stop_req_1(&self) -> bool {
        *self == FLEXSPI2_STOP_REQ_A::FLEXSPI2_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `FLEXSPI2_STOP_REQ`"]
pub struct FLEXSPI2_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXSPI2_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXSPI2_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn flexspi2_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXSPI2_STOP_REQ_A::FLEXSPI2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn flexspi2_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXSPI2_STOP_REQ_A::FLEXSPI2_STOP_REQ_1)
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
#[doc = "EDMA stop acknowledge. This is a status (read-only) bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDMA_STOP_ACK_A {
    #[doc = "0: EDMA stop acknowledge is not asserted"]
    EDMA_STOP_ACK_0,
    #[doc = "1: EDMA stop acknowledge is asserted (EDMA is in STOP mode)."]
    EDMA_STOP_ACK_1,
}
impl From<EDMA_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: EDMA_STOP_ACK_A) -> Self {
        match variant {
            EDMA_STOP_ACK_A::EDMA_STOP_ACK_0 => false,
            EDMA_STOP_ACK_A::EDMA_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `EDMA_STOP_ACK`"]
pub type EDMA_STOP_ACK_R = crate::R<bool, EDMA_STOP_ACK_A>;
impl EDMA_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDMA_STOP_ACK_A {
        match self.bits {
            false => EDMA_STOP_ACK_A::EDMA_STOP_ACK_0,
            true => EDMA_STOP_ACK_A::EDMA_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_edma_stop_ack_0(&self) -> bool {
        *self == EDMA_STOP_ACK_A::EDMA_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_edma_stop_ack_1(&self) -> bool {
        *self == EDMA_STOP_ACK_A::EDMA_STOP_ACK_1
    }
}
#[doc = "CAN1 stop acknowledge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN1_STOP_ACK_A {
    #[doc = "0: CAN1 stop acknowledge is not asserted"]
    CAN1_STOP_ACK_0,
    #[doc = "1: CAN1 stop acknowledge is asserted"]
    CAN1_STOP_ACK_1,
}
impl From<CAN1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: CAN1_STOP_ACK_A) -> Self {
        match variant {
            CAN1_STOP_ACK_A::CAN1_STOP_ACK_0 => false,
            CAN1_STOP_ACK_A::CAN1_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `CAN1_STOP_ACK`"]
pub type CAN1_STOP_ACK_R = crate::R<bool, CAN1_STOP_ACK_A>;
impl CAN1_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN1_STOP_ACK_A {
        match self.bits {
            false => CAN1_STOP_ACK_A::CAN1_STOP_ACK_0,
            true => CAN1_STOP_ACK_A::CAN1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_can1_stop_ack_0(&self) -> bool {
        *self == CAN1_STOP_ACK_A::CAN1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_can1_stop_ack_1(&self) -> bool {
        *self == CAN1_STOP_ACK_A::CAN1_STOP_ACK_1
    }
}
#[doc = "CAN2 stop acknowledge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN2_STOP_ACK_A {
    #[doc = "0: CAN2 stop acknowledge is not asserted"]
    CAN2_STOP_ACK_0,
    #[doc = "1: CAN2 stop acknowledge is asserted"]
    CAN2_STOP_ACK_1,
}
impl From<CAN2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: CAN2_STOP_ACK_A) -> Self {
        match variant {
            CAN2_STOP_ACK_A::CAN2_STOP_ACK_0 => false,
            CAN2_STOP_ACK_A::CAN2_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `CAN2_STOP_ACK`"]
pub type CAN2_STOP_ACK_R = crate::R<bool, CAN2_STOP_ACK_A>;
impl CAN2_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN2_STOP_ACK_A {
        match self.bits {
            false => CAN2_STOP_ACK_A::CAN2_STOP_ACK_0,
            true => CAN2_STOP_ACK_A::CAN2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_can2_stop_ack_0(&self) -> bool {
        *self == CAN2_STOP_ACK_A::CAN2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_can2_stop_ack_1(&self) -> bool {
        *self == CAN2_STOP_ACK_A::CAN2_STOP_ACK_1
    }
}
#[doc = "TRNG stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRNG_STOP_ACK_A {
    #[doc = "0: TRNG stop acknowledge is not asserted"]
    TRNG_STOP_ACK_0,
    #[doc = "1: TRNG stop acknowledge is asserted"]
    TRNG_STOP_ACK_1,
}
impl From<TRNG_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_STOP_ACK_A) -> Self {
        match variant {
            TRNG_STOP_ACK_A::TRNG_STOP_ACK_0 => false,
            TRNG_STOP_ACK_A::TRNG_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `TRNG_STOP_ACK`"]
pub type TRNG_STOP_ACK_R = crate::R<bool, TRNG_STOP_ACK_A>;
impl TRNG_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_STOP_ACK_A {
        match self.bits {
            false => TRNG_STOP_ACK_A::TRNG_STOP_ACK_0,
            true => TRNG_STOP_ACK_A::TRNG_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_trng_stop_ack_0(&self) -> bool {
        *self == TRNG_STOP_ACK_A::TRNG_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_trng_stop_ack_1(&self) -> bool {
        *self == TRNG_STOP_ACK_A::TRNG_STOP_ACK_1
    }
}
#[doc = "ENET stop acknowledge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_STOP_ACK_A {
    #[doc = "0: ENET1 stop acknowledge is not asserted"]
    ENET_STOP_ACK_0,
    #[doc = "1: ENET1 stop acknowledge is asserted"]
    ENET_STOP_ACK_1,
}
impl From<ENET_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_STOP_ACK_A) -> Self {
        match variant {
            ENET_STOP_ACK_A::ENET_STOP_ACK_0 => false,
            ENET_STOP_ACK_A::ENET_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `ENET_STOP_ACK`"]
pub type ENET_STOP_ACK_R = crate::R<bool, ENET_STOP_ACK_A>;
impl ENET_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_STOP_ACK_A {
        match self.bits {
            false => ENET_STOP_ACK_A::ENET_STOP_ACK_0,
            true => ENET_STOP_ACK_A::ENET_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_enet_stop_ack_0(&self) -> bool {
        *self == ENET_STOP_ACK_A::ENET_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_enet_stop_ack_1(&self) -> bool {
        *self == ENET_STOP_ACK_A::ENET_STOP_ACK_1
    }
}
#[doc = "SAI1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_STOP_ACK_A {
    #[doc = "0: SAI1 stop acknowledge is not asserted"]
    SAI1_STOP_ACK_0,
    #[doc = "1: SAI1 stop acknowledge is asserted"]
    SAI1_STOP_ACK_1,
}
impl From<SAI1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1_STOP_ACK_A) -> Self {
        match variant {
            SAI1_STOP_ACK_A::SAI1_STOP_ACK_0 => false,
            SAI1_STOP_ACK_A::SAI1_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `SAI1_STOP_ACK`"]
pub type SAI1_STOP_ACK_R = crate::R<bool, SAI1_STOP_ACK_A>;
impl SAI1_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1_STOP_ACK_A {
        match self.bits {
            false => SAI1_STOP_ACK_A::SAI1_STOP_ACK_0,
            true => SAI1_STOP_ACK_A::SAI1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_sai1_stop_ack_0(&self) -> bool {
        *self == SAI1_STOP_ACK_A::SAI1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_sai1_stop_ack_1(&self) -> bool {
        *self == SAI1_STOP_ACK_A::SAI1_STOP_ACK_1
    }
}
#[doc = "SAI2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_STOP_ACK_A {
    #[doc = "0: SAI2 stop acknowledge is not asserted"]
    SAI2_STOP_ACK_0,
    #[doc = "1: SAI2 stop acknowledge is asserted"]
    SAI2_STOP_ACK_1,
}
impl From<SAI2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2_STOP_ACK_A) -> Self {
        match variant {
            SAI2_STOP_ACK_A::SAI2_STOP_ACK_0 => false,
            SAI2_STOP_ACK_A::SAI2_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `SAI2_STOP_ACK`"]
pub type SAI2_STOP_ACK_R = crate::R<bool, SAI2_STOP_ACK_A>;
impl SAI2_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_STOP_ACK_A {
        match self.bits {
            false => SAI2_STOP_ACK_A::SAI2_STOP_ACK_0,
            true => SAI2_STOP_ACK_A::SAI2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_sai2_stop_ack_0(&self) -> bool {
        *self == SAI2_STOP_ACK_A::SAI2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_sai2_stop_ack_1(&self) -> bool {
        *self == SAI2_STOP_ACK_A::SAI2_STOP_ACK_1
    }
}
#[doc = "SAI3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_STOP_ACK_A {
    #[doc = "0: SAI3 stop acknowledge is not asserted"]
    SAI3_STOP_ACK_0,
    #[doc = "1: SAI3 stop acknowledge is asserted"]
    SAI3_STOP_ACK_1,
}
impl From<SAI3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3_STOP_ACK_A) -> Self {
        match variant {
            SAI3_STOP_ACK_A::SAI3_STOP_ACK_0 => false,
            SAI3_STOP_ACK_A::SAI3_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `SAI3_STOP_ACK`"]
pub type SAI3_STOP_ACK_R = crate::R<bool, SAI3_STOP_ACK_A>;
impl SAI3_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3_STOP_ACK_A {
        match self.bits {
            false => SAI3_STOP_ACK_A::SAI3_STOP_ACK_0,
            true => SAI3_STOP_ACK_A::SAI3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_sai3_stop_ack_0(&self) -> bool {
        *self == SAI3_STOP_ACK_A::SAI3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_sai3_stop_ack_1(&self) -> bool {
        *self == SAI3_STOP_ACK_A::SAI3_STOP_ACK_1
    }
}
#[doc = "ENET2 stop acknowledge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET2_STOP_ACK_A {
    #[doc = "0: ENET2 stop acknowledge is not asserted"]
    ENET2_STOP_ACK_0,
    #[doc = "1: ENET2 stop acknowledge is asserted"]
    ENET2_STOP_ACK_1,
}
impl From<ENET2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ENET2_STOP_ACK_A) -> Self {
        match variant {
            ENET2_STOP_ACK_A::ENET2_STOP_ACK_0 => false,
            ENET2_STOP_ACK_A::ENET2_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `ENET2_STOP_ACK`"]
pub type ENET2_STOP_ACK_R = crate::R<bool, ENET2_STOP_ACK_A>;
impl ENET2_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET2_STOP_ACK_A {
        match self.bits {
            false => ENET2_STOP_ACK_A::ENET2_STOP_ACK_0,
            true => ENET2_STOP_ACK_A::ENET2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_enet2_stop_ack_0(&self) -> bool {
        *self == ENET2_STOP_ACK_A::ENET2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `ENET2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_enet2_stop_ack_1(&self) -> bool {
        *self == ENET2_STOP_ACK_A::ENET2_STOP_ACK_1
    }
}
#[doc = "SEMC stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_STOP_ACK_A {
    #[doc = "0: SEMC stop acknowledge is not asserted"]
    SEMC_STOP_ACK_0,
    #[doc = "1: SEMC stop acknowledge is asserted"]
    SEMC_STOP_ACK_1,
}
impl From<SEMC_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_STOP_ACK_A) -> Self {
        match variant {
            SEMC_STOP_ACK_A::SEMC_STOP_ACK_0 => false,
            SEMC_STOP_ACK_A::SEMC_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `SEMC_STOP_ACK`"]
pub type SEMC_STOP_ACK_R = crate::R<bool, SEMC_STOP_ACK_A>;
impl SEMC_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_STOP_ACK_A {
        match self.bits {
            false => SEMC_STOP_ACK_A::SEMC_STOP_ACK_0,
            true => SEMC_STOP_ACK_A::SEMC_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_semc_stop_ack_0(&self) -> bool {
        *self == SEMC_STOP_ACK_A::SEMC_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_semc_stop_ack_1(&self) -> bool {
        *self == SEMC_STOP_ACK_A::SEMC_STOP_ACK_1
    }
}
#[doc = "PIT stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT_STOP_ACK_A {
    #[doc = "0: PIT stop acknowledge is not asserted"]
    PIT_STOP_ACK_0,
    #[doc = "1: PIT stop acknowledge is asserted"]
    PIT_STOP_ACK_1,
}
impl From<PIT_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_STOP_ACK_A) -> Self {
        match variant {
            PIT_STOP_ACK_A::PIT_STOP_ACK_0 => false,
            PIT_STOP_ACK_A::PIT_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `PIT_STOP_ACK`"]
pub type PIT_STOP_ACK_R = crate::R<bool, PIT_STOP_ACK_A>;
impl PIT_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_STOP_ACK_A {
        match self.bits {
            false => PIT_STOP_ACK_A::PIT_STOP_ACK_0,
            true => PIT_STOP_ACK_A::PIT_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_pit_stop_ack_0(&self) -> bool {
        *self == PIT_STOP_ACK_A::PIT_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_pit_stop_ack_1(&self) -> bool {
        *self == PIT_STOP_ACK_A::PIT_STOP_ACK_1
    }
}
#[doc = "FLEXSPI stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI_STOP_ACK_A {
    #[doc = "0: FLEXSPI stop acknowledge is not asserted"]
    FLEXSPI_STOP_ACK_0,
    #[doc = "1: FLEXSPI stop acknowledge is asserted"]
    FLEXSPI_STOP_ACK_1,
}
impl From<FLEXSPI_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI_STOP_ACK_A) -> Self {
        match variant {
            FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_0 => false,
            FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXSPI_STOP_ACK`"]
pub type FLEXSPI_STOP_ACK_R = crate::R<bool, FLEXSPI_STOP_ACK_A>;
impl FLEXSPI_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI_STOP_ACK_A {
        match self.bits {
            false => FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_0,
            true => FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_flexspi_stop_ack_0(&self) -> bool {
        *self == FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_flexspi_stop_ack_1(&self) -> bool {
        *self == FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_1
    }
}
#[doc = "FLEXIO1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO1_STOP_ACK_A {
    #[doc = "0: FLEXIO1 stop acknowledge is not asserted"]
    FLEXIO1_STOP_ACK_0,
    #[doc = "1: FLEXIO1 stop acknowledge is asserted"]
    FLEXIO1_STOP_ACK_1,
}
impl From<FLEXIO1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO1_STOP_ACK_A) -> Self {
        match variant {
            FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_0 => false,
            FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXIO1_STOP_ACK`"]
pub type FLEXIO1_STOP_ACK_R = crate::R<bool, FLEXIO1_STOP_ACK_A>;
impl FLEXIO1_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_STOP_ACK_A {
        match self.bits {
            false => FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_0,
            true => FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_flexio1_stop_ack_0(&self) -> bool {
        *self == FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_flexio1_stop_ack_1(&self) -> bool {
        *self == FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_1
    }
}
#[doc = "FLEXIO2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO2_STOP_ACK_A {
    #[doc = "0: FLEXIO2 stop acknowledge is not asserted"]
    FLEXIO2_STOP_ACK_0,
    #[doc = "1: FLEXIO2 stop acknowledge is asserted (FLEXIO2 is in STOP mode)"]
    FLEXIO2_STOP_ACK_1,
}
impl From<FLEXIO2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO2_STOP_ACK_A) -> Self {
        match variant {
            FLEXIO2_STOP_ACK_A::FLEXIO2_STOP_ACK_0 => false,
            FLEXIO2_STOP_ACK_A::FLEXIO2_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXIO2_STOP_ACK`"]
pub type FLEXIO2_STOP_ACK_R = crate::R<bool, FLEXIO2_STOP_ACK_A>;
impl FLEXIO2_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO2_STOP_ACK_A {
        match self.bits {
            false => FLEXIO2_STOP_ACK_A::FLEXIO2_STOP_ACK_0,
            true => FLEXIO2_STOP_ACK_A::FLEXIO2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_flexio2_stop_ack_0(&self) -> bool {
        *self == FLEXIO2_STOP_ACK_A::FLEXIO2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_flexio2_stop_ack_1(&self) -> bool {
        *self == FLEXIO2_STOP_ACK_A::FLEXIO2_STOP_ACK_1
    }
}
#[doc = "On-platform FLEXIO3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXIO3_STOP_ACK_A {
    #[doc = "0: FLEXIO3 stop acknowledge is not asserted"]
    FLEXIO3_STOP_ACK_0,
    #[doc = "1: FLEXIO3 stop acknowledge is asserted"]
    FLEXIO3_STOP_ACK_1,
}
impl From<FLEXIO3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO3_STOP_ACK_A) -> Self {
        match variant {
            FLEXIO3_STOP_ACK_A::FLEXIO3_STOP_ACK_0 => false,
            FLEXIO3_STOP_ACK_A::FLEXIO3_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXIO3_STOP_ACK`"]
pub type FLEXIO3_STOP_ACK_R = crate::R<bool, FLEXIO3_STOP_ACK_A>;
impl FLEXIO3_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO3_STOP_ACK_A {
        match self.bits {
            false => FLEXIO3_STOP_ACK_A::FLEXIO3_STOP_ACK_0,
            true => FLEXIO3_STOP_ACK_A::FLEXIO3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_flexio3_stop_ack_0(&self) -> bool {
        *self == FLEXIO3_STOP_ACK_A::FLEXIO3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_flexio3_stop_ack_1(&self) -> bool {
        *self == FLEXIO3_STOP_ACK_A::FLEXIO3_STOP_ACK_1
    }
}
#[doc = "FLEXSPI2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXSPI2_STOP_ACK_A {
    #[doc = "0: FLEXSPI2 stop acknowledge is not asserted"]
    FLEXSPI2_STOP_ACK_0,
    #[doc = "1: FLEXSPI2 stop acknowledge is asserted"]
    FLEXSPI2_STOP_ACK_1,
}
impl From<FLEXSPI2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI2_STOP_ACK_A) -> Self {
        match variant {
            FLEXSPI2_STOP_ACK_A::FLEXSPI2_STOP_ACK_0 => false,
            FLEXSPI2_STOP_ACK_A::FLEXSPI2_STOP_ACK_1 => true,
        }
    }
}
#[doc = "Reader of field `FLEXSPI2_STOP_ACK`"]
pub type FLEXSPI2_STOP_ACK_R = crate::R<bool, FLEXSPI2_STOP_ACK_A>;
impl FLEXSPI2_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI2_STOP_ACK_A {
        match self.bits {
            false => FLEXSPI2_STOP_ACK_A::FLEXSPI2_STOP_ACK_0,
            true => FLEXSPI2_STOP_ACK_A::FLEXSPI2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_flexspi2_stop_ack_0(&self) -> bool {
        *self == FLEXSPI2_STOP_ACK_A::FLEXSPI2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_flexspi2_stop_ack_1(&self) -> bool {
        *self == FLEXSPI2_STOP_ACK_A::FLEXSPI2_STOP_ACK_1
    }
}
impl R {
    #[doc = "Bit 0 - EDMA stop request."]
    #[inline(always)]
    pub fn edma_stop_req(&self) -> EDMA_STOP_REQ_R {
        EDMA_STOP_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAN1 stop request."]
    #[inline(always)]
    pub fn can1_stop_req(&self) -> CAN1_STOP_REQ_R {
        CAN1_STOP_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAN2 stop request."]
    #[inline(always)]
    pub fn can2_stop_req(&self) -> CAN2_STOP_REQ_R {
        CAN2_STOP_REQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TRNG stop request."]
    #[inline(always)]
    pub fn trng_stop_req(&self) -> TRNG_STOP_REQ_R {
        TRNG_STOP_REQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ENET stop request."]
    #[inline(always)]
    pub fn enet_stop_req(&self) -> ENET_STOP_REQ_R {
        ENET_STOP_REQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1 stop request."]
    #[inline(always)]
    pub fn sai1_stop_req(&self) -> SAI1_STOP_REQ_R {
        SAI1_STOP_REQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2 stop request."]
    #[inline(always)]
    pub fn sai2_stop_req(&self) -> SAI2_STOP_REQ_R {
        SAI2_STOP_REQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SAI3 stop request."]
    #[inline(always)]
    pub fn sai3_stop_req(&self) -> SAI3_STOP_REQ_R {
        SAI3_STOP_REQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ENET2 stop request."]
    #[inline(always)]
    pub fn enet2_stop_req(&self) -> ENET2_STOP_REQ_R {
        ENET2_STOP_REQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SEMC stop request."]
    #[inline(always)]
    pub fn semc_stop_req(&self) -> SEMC_STOP_REQ_R {
        SEMC_STOP_REQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PIT stop request."]
    #[inline(always)]
    pub fn pit_stop_req(&self) -> PIT_STOP_REQ_R {
        PIT_STOP_REQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FlexSPI stop request."]
    #[inline(always)]
    pub fn flexspi_stop_req(&self) -> FLEXSPI_STOP_REQ_R {
        FLEXSPI_STOP_REQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FlexIO1 stop request."]
    #[inline(always)]
    pub fn flexio1_stop_req(&self) -> FLEXIO1_STOP_REQ_R {
        FLEXIO1_STOP_REQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FlexIO2 stop request."]
    #[inline(always)]
    pub fn flexio2_stop_req(&self) -> FLEXIO2_STOP_REQ_R {
        FLEXIO2_STOP_REQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - On-platform flexio3 stop request."]
    #[inline(always)]
    pub fn flexio3_stop_req(&self) -> FLEXIO3_STOP_REQ_R {
        FLEXIO3_STOP_REQ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FlexSPI2 stop request."]
    #[inline(always)]
    pub fn flexspi2_stop_req(&self) -> FLEXSPI2_STOP_REQ_R {
        FLEXSPI2_STOP_REQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EDMA stop acknowledge. This is a status (read-only) bit"]
    #[inline(always)]
    pub fn edma_stop_ack(&self) -> EDMA_STOP_ACK_R {
        EDMA_STOP_ACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CAN1 stop acknowledge."]
    #[inline(always)]
    pub fn can1_stop_ack(&self) -> CAN1_STOP_ACK_R {
        CAN1_STOP_ACK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CAN2 stop acknowledge."]
    #[inline(always)]
    pub fn can2_stop_ack(&self) -> CAN2_STOP_ACK_R {
        CAN2_STOP_ACK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TRNG stop acknowledge"]
    #[inline(always)]
    pub fn trng_stop_ack(&self) -> TRNG_STOP_ACK_R {
        TRNG_STOP_ACK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ENET stop acknowledge."]
    #[inline(always)]
    pub fn enet_stop_ack(&self) -> ENET_STOP_ACK_R {
        ENET_STOP_ACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SAI1 stop acknowledge"]
    #[inline(always)]
    pub fn sai1_stop_ack(&self) -> SAI1_STOP_ACK_R {
        SAI1_STOP_ACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SAI2 stop acknowledge"]
    #[inline(always)]
    pub fn sai2_stop_ack(&self) -> SAI2_STOP_ACK_R {
        SAI2_STOP_ACK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SAI3 stop acknowledge"]
    #[inline(always)]
    pub fn sai3_stop_ack(&self) -> SAI3_STOP_ACK_R {
        SAI3_STOP_ACK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ENET2 stop acknowledge."]
    #[inline(always)]
    pub fn enet2_stop_ack(&self) -> ENET2_STOP_ACK_R {
        ENET2_STOP_ACK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SEMC stop acknowledge"]
    #[inline(always)]
    pub fn semc_stop_ack(&self) -> SEMC_STOP_ACK_R {
        SEMC_STOP_ACK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PIT stop acknowledge"]
    #[inline(always)]
    pub fn pit_stop_ack(&self) -> PIT_STOP_ACK_R {
        PIT_STOP_ACK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FLEXSPI stop acknowledge"]
    #[inline(always)]
    pub fn flexspi_stop_ack(&self) -> FLEXSPI_STOP_ACK_R {
        FLEXSPI_STOP_ACK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - FLEXIO1 stop acknowledge"]
    #[inline(always)]
    pub fn flexio1_stop_ack(&self) -> FLEXIO1_STOP_ACK_R {
        FLEXIO1_STOP_ACK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - FLEXIO2 stop acknowledge"]
    #[inline(always)]
    pub fn flexio2_stop_ack(&self) -> FLEXIO2_STOP_ACK_R {
        FLEXIO2_STOP_ACK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - On-platform FLEXIO3 stop acknowledge"]
    #[inline(always)]
    pub fn flexio3_stop_ack(&self) -> FLEXIO3_STOP_ACK_R {
        FLEXIO3_STOP_ACK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - FLEXSPI2 stop acknowledge"]
    #[inline(always)]
    pub fn flexspi2_stop_ack(&self) -> FLEXSPI2_STOP_ACK_R {
        FLEXSPI2_STOP_ACK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EDMA stop request."]
    #[inline(always)]
    pub fn edma_stop_req(&mut self) -> EDMA_STOP_REQ_W {
        EDMA_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 1 - CAN1 stop request."]
    #[inline(always)]
    pub fn can1_stop_req(&mut self) -> CAN1_STOP_REQ_W {
        CAN1_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 2 - CAN2 stop request."]
    #[inline(always)]
    pub fn can2_stop_req(&mut self) -> CAN2_STOP_REQ_W {
        CAN2_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 3 - TRNG stop request."]
    #[inline(always)]
    pub fn trng_stop_req(&mut self) -> TRNG_STOP_REQ_W {
        TRNG_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 4 - ENET stop request."]
    #[inline(always)]
    pub fn enet_stop_req(&mut self) -> ENET_STOP_REQ_W {
        ENET_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 5 - SAI1 stop request."]
    #[inline(always)]
    pub fn sai1_stop_req(&mut self) -> SAI1_STOP_REQ_W {
        SAI1_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 6 - SAI2 stop request."]
    #[inline(always)]
    pub fn sai2_stop_req(&mut self) -> SAI2_STOP_REQ_W {
        SAI2_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 7 - SAI3 stop request."]
    #[inline(always)]
    pub fn sai3_stop_req(&mut self) -> SAI3_STOP_REQ_W {
        SAI3_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 8 - ENET2 stop request."]
    #[inline(always)]
    pub fn enet2_stop_req(&mut self) -> ENET2_STOP_REQ_W {
        ENET2_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 9 - SEMC stop request."]
    #[inline(always)]
    pub fn semc_stop_req(&mut self) -> SEMC_STOP_REQ_W {
        SEMC_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 10 - PIT stop request."]
    #[inline(always)]
    pub fn pit_stop_req(&mut self) -> PIT_STOP_REQ_W {
        PIT_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 11 - FlexSPI stop request."]
    #[inline(always)]
    pub fn flexspi_stop_req(&mut self) -> FLEXSPI_STOP_REQ_W {
        FLEXSPI_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 12 - FlexIO1 stop request."]
    #[inline(always)]
    pub fn flexio1_stop_req(&mut self) -> FLEXIO1_STOP_REQ_W {
        FLEXIO1_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 13 - FlexIO2 stop request."]
    #[inline(always)]
    pub fn flexio2_stop_req(&mut self) -> FLEXIO2_STOP_REQ_W {
        FLEXIO2_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 14 - On-platform flexio3 stop request."]
    #[inline(always)]
    pub fn flexio3_stop_req(&mut self) -> FLEXIO3_STOP_REQ_W {
        FLEXIO3_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 15 - FlexSPI2 stop request."]
    #[inline(always)]
    pub fn flexspi2_stop_req(&mut self) -> FLEXSPI2_STOP_REQ_W {
        FLEXSPI2_STOP_REQ_W { w: self }
    }
}
