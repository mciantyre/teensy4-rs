#[doc = "Reader of register GPR13"]
pub type R = crate::R<u32, super::GPR13>;
#[doc = "Writer for register GPR13"]
pub type W = crate::W<u32, super::GPR13>;
#[doc = "Register GPR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "uSDHC block cacheable attribute value of AXI read transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARCACHE_USDHC_A {
    #[doc = "0: Cacheable attribute is off for read transactions."]
    ARCACHE_USDHC_0 = 0,
    #[doc = "1: Cacheable attribute is on for read transactions."]
    ARCACHE_USDHC_1 = 1,
}
impl From<ARCACHE_USDHC_A> for bool {
    #[inline(always)]
    fn from(variant: ARCACHE_USDHC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARCACHE_USDHC`"]
pub type ARCACHE_USDHC_R = crate::R<bool, ARCACHE_USDHC_A>;
impl ARCACHE_USDHC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARCACHE_USDHC_A {
        match self.bits {
            false => ARCACHE_USDHC_A::ARCACHE_USDHC_0,
            true => ARCACHE_USDHC_A::ARCACHE_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARCACHE_USDHC_0`"]
    #[inline(always)]
    pub fn is_arcache_usdhc_0(&self) -> bool {
        *self == ARCACHE_USDHC_A::ARCACHE_USDHC_0
    }
    #[doc = "Checks if the value of the field is `ARCACHE_USDHC_1`"]
    #[inline(always)]
    pub fn is_arcache_usdhc_1(&self) -> bool {
        *self == ARCACHE_USDHC_A::ARCACHE_USDHC_1
    }
}
#[doc = "Write proxy for field `ARCACHE_USDHC`"]
pub struct ARCACHE_USDHC_W<'a> {
    w: &'a mut W,
}
impl<'a> ARCACHE_USDHC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARCACHE_USDHC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cacheable attribute is off for read transactions."]
    #[inline(always)]
    pub fn arcache_usdhc_0(self) -> &'a mut W {
        self.variant(ARCACHE_USDHC_A::ARCACHE_USDHC_0)
    }
    #[doc = "Cacheable attribute is on for read transactions."]
    #[inline(always)]
    pub fn arcache_usdhc_1(self) -> &'a mut W {
        self.variant(ARCACHE_USDHC_A::ARCACHE_USDHC_1)
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
#[doc = "uSDHC block cacheable attribute value of AXI write transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWCACHE_USDHC_A {
    #[doc = "0: Cacheable attribute is off for write transactions."]
    AWCACHE_USDHC_0 = 0,
    #[doc = "1: Cacheable attribute is on for write transactions."]
    AWCACHE_USDHC_1 = 1,
}
impl From<AWCACHE_USDHC_A> for bool {
    #[inline(always)]
    fn from(variant: AWCACHE_USDHC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWCACHE_USDHC`"]
pub type AWCACHE_USDHC_R = crate::R<bool, AWCACHE_USDHC_A>;
impl AWCACHE_USDHC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWCACHE_USDHC_A {
        match self.bits {
            false => AWCACHE_USDHC_A::AWCACHE_USDHC_0,
            true => AWCACHE_USDHC_A::AWCACHE_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `AWCACHE_USDHC_0`"]
    #[inline(always)]
    pub fn is_awcache_usdhc_0(&self) -> bool {
        *self == AWCACHE_USDHC_A::AWCACHE_USDHC_0
    }
    #[doc = "Checks if the value of the field is `AWCACHE_USDHC_1`"]
    #[inline(always)]
    pub fn is_awcache_usdhc_1(&self) -> bool {
        *self == AWCACHE_USDHC_A::AWCACHE_USDHC_1
    }
}
#[doc = "Write proxy for field `AWCACHE_USDHC`"]
pub struct AWCACHE_USDHC_W<'a> {
    w: &'a mut W,
}
impl<'a> AWCACHE_USDHC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWCACHE_USDHC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cacheable attribute is off for write transactions."]
    #[inline(always)]
    pub fn awcache_usdhc_0(self) -> &'a mut W {
        self.variant(AWCACHE_USDHC_A::AWCACHE_USDHC_0)
    }
    #[doc = "Cacheable attribute is on for write transactions."]
    #[inline(always)]
    pub fn awcache_usdhc_1(self) -> &'a mut W {
        self.variant(AWCACHE_USDHC_A::AWCACHE_USDHC_1)
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
#[doc = "CANFD stop request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANFD_STOP_REQ_A {
    #[doc = "0: stop request off"]
    CANFD_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    CANFD_STOP_REQ_1 = 1,
}
impl From<CANFD_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: CANFD_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CANFD_STOP_REQ`"]
pub type CANFD_STOP_REQ_R = crate::R<bool, CANFD_STOP_REQ_A>;
impl CANFD_STOP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANFD_STOP_REQ_A {
        match self.bits {
            false => CANFD_STOP_REQ_A::CANFD_STOP_REQ_0,
            true => CANFD_STOP_REQ_A::CANFD_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CANFD_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_canfd_stop_req_0(&self) -> bool {
        *self == CANFD_STOP_REQ_A::CANFD_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `CANFD_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_canfd_stop_req_1(&self) -> bool {
        *self == CANFD_STOP_REQ_A::CANFD_STOP_REQ_1
    }
}
#[doc = "Write proxy for field `CANFD_STOP_REQ`"]
pub struct CANFD_STOP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CANFD_STOP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CANFD_STOP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn canfd_stop_req_0(self) -> &'a mut W {
        self.variant(CANFD_STOP_REQ_A::CANFD_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn canfd_stop_req_1(self) -> &'a mut W {
        self.variant(CANFD_STOP_REQ_A::CANFD_STOP_REQ_1)
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
#[doc = "ENET block cacheable attribute value of AXI transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_ENET_A {
    #[doc = "0: Cacheable attribute is off for read/write transactions."]
    CACHE_ENET_0 = 0,
    #[doc = "1: Cacheable attribute is on for read/write transactions."]
    CACHE_ENET_1 = 1,
}
impl From<CACHE_ENET_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_ENET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHE_ENET`"]
pub type CACHE_ENET_R = crate::R<bool, CACHE_ENET_A>;
impl CACHE_ENET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_ENET_A {
        match self.bits {
            false => CACHE_ENET_A::CACHE_ENET_0,
            true => CACHE_ENET_A::CACHE_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHE_ENET_0`"]
    #[inline(always)]
    pub fn is_cache_enet_0(&self) -> bool {
        *self == CACHE_ENET_A::CACHE_ENET_0
    }
    #[doc = "Checks if the value of the field is `CACHE_ENET_1`"]
    #[inline(always)]
    pub fn is_cache_enet_1(&self) -> bool {
        *self == CACHE_ENET_A::CACHE_ENET_1
    }
}
#[doc = "Write proxy for field `CACHE_ENET`"]
pub struct CACHE_ENET_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_ENET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHE_ENET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cacheable attribute is off for read/write transactions."]
    #[inline(always)]
    pub fn cache_enet_0(self) -> &'a mut W {
        self.variant(CACHE_ENET_A::CACHE_ENET_0)
    }
    #[doc = "Cacheable attribute is on for read/write transactions."]
    #[inline(always)]
    pub fn cache_enet_1(self) -> &'a mut W {
        self.variant(CACHE_ENET_A::CACHE_ENET_1)
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
#[doc = "USB block cacheable attribute value of AXI transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_USB_A {
    #[doc = "0: Cacheable attribute is off for read/write transactions."]
    CACHE_USB_0 = 0,
    #[doc = "1: Cacheable attribute is on for read/write transactions."]
    CACHE_USB_1 = 1,
}
impl From<CACHE_USB_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHE_USB`"]
pub type CACHE_USB_R = crate::R<bool, CACHE_USB_A>;
impl CACHE_USB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_USB_A {
        match self.bits {
            false => CACHE_USB_A::CACHE_USB_0,
            true => CACHE_USB_A::CACHE_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHE_USB_0`"]
    #[inline(always)]
    pub fn is_cache_usb_0(&self) -> bool {
        *self == CACHE_USB_A::CACHE_USB_0
    }
    #[doc = "Checks if the value of the field is `CACHE_USB_1`"]
    #[inline(always)]
    pub fn is_cache_usb_1(&self) -> bool {
        *self == CACHE_USB_A::CACHE_USB_1
    }
}
#[doc = "Write proxy for field `CACHE_USB`"]
pub struct CACHE_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_USB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHE_USB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cacheable attribute is off for read/write transactions."]
    #[inline(always)]
    pub fn cache_usb_0(self) -> &'a mut W {
        self.variant(CACHE_USB_A::CACHE_USB_0)
    }
    #[doc = "Cacheable attribute is on for read/write transactions."]
    #[inline(always)]
    pub fn cache_usb_1(self) -> &'a mut W {
        self.variant(CACHE_USB_A::CACHE_USB_1)
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
#[doc = "CANFD stop acknowledge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANFD_STOP_ACK_A {
    #[doc = "0: CANFD stop acknowledge is not asserted"]
    CANFD_STOP_ACK_0 = 0,
    #[doc = "1: CANFD stop acknowledge is asserted"]
    CANFD_STOP_ACK_1 = 1,
}
impl From<CANFD_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: CANFD_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CANFD_STOP_ACK`"]
pub type CANFD_STOP_ACK_R = crate::R<bool, CANFD_STOP_ACK_A>;
impl CANFD_STOP_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANFD_STOP_ACK_A {
        match self.bits {
            false => CANFD_STOP_ACK_A::CANFD_STOP_ACK_0,
            true => CANFD_STOP_ACK_A::CANFD_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CANFD_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_canfd_stop_ack_0(&self) -> bool {
        *self == CANFD_STOP_ACK_A::CANFD_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `CANFD_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_canfd_stop_ack_1(&self) -> bool {
        *self == CANFD_STOP_ACK_A::CANFD_STOP_ACK_1
    }
}
impl R {
    #[doc = "Bit 0 - uSDHC block cacheable attribute value of AXI read transactions"]
    #[inline(always)]
    pub fn arcache_usdhc(&self) -> ARCACHE_USDHC_R {
        ARCACHE_USDHC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - uSDHC block cacheable attribute value of AXI write transactions"]
    #[inline(always)]
    pub fn awcache_usdhc(&self) -> AWCACHE_USDHC_R {
        AWCACHE_USDHC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CANFD stop request."]
    #[inline(always)]
    pub fn canfd_stop_req(&self) -> CANFD_STOP_REQ_R {
        CANFD_STOP_REQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ENET block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub fn cache_enet(&self) -> CACHE_ENET_R {
        CACHE_ENET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USB block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub fn cache_usb(&self) -> CACHE_USB_R {
        CACHE_USB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CANFD stop acknowledge."]
    #[inline(always)]
    pub fn canfd_stop_ack(&self) -> CANFD_STOP_ACK_R {
        CANFD_STOP_ACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uSDHC block cacheable attribute value of AXI read transactions"]
    #[inline(always)]
    pub fn arcache_usdhc(&mut self) -> ARCACHE_USDHC_W {
        ARCACHE_USDHC_W { w: self }
    }
    #[doc = "Bit 1 - uSDHC block cacheable attribute value of AXI write transactions"]
    #[inline(always)]
    pub fn awcache_usdhc(&mut self) -> AWCACHE_USDHC_W {
        AWCACHE_USDHC_W { w: self }
    }
    #[doc = "Bit 4 - CANFD stop request."]
    #[inline(always)]
    pub fn canfd_stop_req(&mut self) -> CANFD_STOP_REQ_W {
        CANFD_STOP_REQ_W { w: self }
    }
    #[doc = "Bit 7 - ENET block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub fn cache_enet(&mut self) -> CACHE_ENET_W {
        CACHE_ENET_W { w: self }
    }
    #[doc = "Bit 13 - USB block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub fn cache_usb(&mut self) -> CACHE_USB_W {
        CACHE_USB_W { w: self }
    }
}
