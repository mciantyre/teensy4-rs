#[doc = "Reader of register TCD21_ATTR"]
pub type R = crate::R<u16, super::TCD21_ATTR>;
#[doc = "Writer for register TCD21_ATTR"]
pub type W = crate::W<u16, super::TCD21_ATTR>;
#[doc = "Register TCD21_ATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD21_ATTR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSIZE`"]
pub type DSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSIZE`"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DMOD`"]
pub type DMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMOD`"]
pub struct DMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u16) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Source data transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSIZE_A {
    #[doc = "0: 8-bit"]
    SSIZE_0 = 0,
    #[doc = "1: 16-bit"]
    SSIZE_1 = 1,
    #[doc = "2: 32-bit"]
    SSIZE_2 = 2,
    #[doc = "3: 64-bit"]
    SSIZE_3 = 3,
    #[doc = "5: 32-byte burst (4 beats of 64 bits)"]
    SSIZE_5 = 5,
}
impl From<SSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSIZE`"]
pub type SSIZE_R = crate::R<u8, SSIZE_A>;
impl SSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSIZE_A::SSIZE_0),
            1 => Val(SSIZE_A::SSIZE_1),
            2 => Val(SSIZE_A::SSIZE_2),
            3 => Val(SSIZE_A::SSIZE_3),
            5 => Val(SSIZE_A::SSIZE_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSIZE_0`"]
    #[inline(always)]
    pub fn is_ssize_0(&self) -> bool {
        *self == SSIZE_A::SSIZE_0
    }
    #[doc = "Checks if the value of the field is `SSIZE_1`"]
    #[inline(always)]
    pub fn is_ssize_1(&self) -> bool {
        *self == SSIZE_A::SSIZE_1
    }
    #[doc = "Checks if the value of the field is `SSIZE_2`"]
    #[inline(always)]
    pub fn is_ssize_2(&self) -> bool {
        *self == SSIZE_A::SSIZE_2
    }
    #[doc = "Checks if the value of the field is `SSIZE_3`"]
    #[inline(always)]
    pub fn is_ssize_3(&self) -> bool {
        *self == SSIZE_A::SSIZE_3
    }
    #[doc = "Checks if the value of the field is `SSIZE_5`"]
    #[inline(always)]
    pub fn is_ssize_5(&self) -> bool {
        *self == SSIZE_A::SSIZE_5
    }
}
#[doc = "Write proxy for field `SSIZE`"]
pub struct SSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn ssize_0(self) -> &'a mut W {
        self.variant(SSIZE_A::SSIZE_0)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn ssize_1(self) -> &'a mut W {
        self.variant(SSIZE_A::SSIZE_1)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn ssize_2(self) -> &'a mut W {
        self.variant(SSIZE_A::SSIZE_2)
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn ssize_3(self) -> &'a mut W {
        self.variant(SSIZE_A::SSIZE_3)
    }
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    #[inline(always)]
    pub fn ssize_5(self) -> &'a mut W {
        self.variant(SSIZE_A::SSIZE_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Source Address Modulo\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Source address modulo feature is disabled"]
    SMOD_0 = 0,
    #[doc = "1: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_1 = 1,
    #[doc = "2: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_2 = 2,
    #[doc = "3: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_3 = 3,
    #[doc = "4: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_4 = 4,
    #[doc = "5: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_5 = 5,
    #[doc = "6: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_6 = 6,
    #[doc = "7: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_7 = 7,
    #[doc = "8: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_8 = 8,
    #[doc = "9: This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_9 = 9,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMOD`"]
pub type SMOD_R = crate::R<u8, SMOD_A>;
impl SMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMOD_A::SMOD_0),
            1 => Val(SMOD_A::SMOD_1),
            2 => Val(SMOD_A::SMOD_2),
            3 => Val(SMOD_A::SMOD_3),
            4 => Val(SMOD_A::SMOD_4),
            5 => Val(SMOD_A::SMOD_5),
            6 => Val(SMOD_A::SMOD_6),
            7 => Val(SMOD_A::SMOD_7),
            8 => Val(SMOD_A::SMOD_8),
            9 => Val(SMOD_A::SMOD_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SMOD_0`"]
    #[inline(always)]
    pub fn is_smod_0(&self) -> bool {
        *self == SMOD_A::SMOD_0
    }
    #[doc = "Checks if the value of the field is `SMOD_1`"]
    #[inline(always)]
    pub fn is_smod_1(&self) -> bool {
        *self == SMOD_A::SMOD_1
    }
    #[doc = "Checks if the value of the field is `SMOD_2`"]
    #[inline(always)]
    pub fn is_smod_2(&self) -> bool {
        *self == SMOD_A::SMOD_2
    }
    #[doc = "Checks if the value of the field is `SMOD_3`"]
    #[inline(always)]
    pub fn is_smod_3(&self) -> bool {
        *self == SMOD_A::SMOD_3
    }
    #[doc = "Checks if the value of the field is `SMOD_4`"]
    #[inline(always)]
    pub fn is_smod_4(&self) -> bool {
        *self == SMOD_A::SMOD_4
    }
    #[doc = "Checks if the value of the field is `SMOD_5`"]
    #[inline(always)]
    pub fn is_smod_5(&self) -> bool {
        *self == SMOD_A::SMOD_5
    }
    #[doc = "Checks if the value of the field is `SMOD_6`"]
    #[inline(always)]
    pub fn is_smod_6(&self) -> bool {
        *self == SMOD_A::SMOD_6
    }
    #[doc = "Checks if the value of the field is `SMOD_7`"]
    #[inline(always)]
    pub fn is_smod_7(&self) -> bool {
        *self == SMOD_A::SMOD_7
    }
    #[doc = "Checks if the value of the field is `SMOD_8`"]
    #[inline(always)]
    pub fn is_smod_8(&self) -> bool {
        *self == SMOD_A::SMOD_8
    }
    #[doc = "Checks if the value of the field is `SMOD_9`"]
    #[inline(always)]
    pub fn is_smod_9(&self) -> bool {
        *self == SMOD_A::SMOD_9
    }
}
#[doc = "Write proxy for field `SMOD`"]
pub struct SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Source address modulo feature is disabled"]
    #[inline(always)]
    pub fn smod_0(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_0)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_1(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_1)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_2(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_2)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_3(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_3)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_4(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_4)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_5(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_5)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_6(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_6)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_7(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_7)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_8(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_8)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline(always)]
    pub fn smod_9(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&self) -> DMOD_R {
        DMOD_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&mut self) -> DMOD_W {
        DMOD_W { w: self }
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W {
        SSIZE_W { w: self }
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W {
        SMOD_W { w: self }
    }
}
