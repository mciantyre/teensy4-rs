#[doc = "Reader of register CH0STAT_SET"]
pub type R = crate::R<u32, super::CH0STAT_SET>;
#[doc = "Writer for register CH0STAT_SET"]
pub type W = crate::W<u32, super::CH0STAT_SET>;
#[doc = "Register CH0STAT_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0STAT_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HASH_MISMATCH`"]
pub type HASH_MISMATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASH_MISMATCH`"]
pub struct HASH_MISMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_MISMATCH_W<'a> {
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
#[doc = "Reader of field `ERROR_SETUP`"]
pub type ERROR_SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_SETUP`"]
pub struct ERROR_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_SETUP_W<'a> {
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
#[doc = "Reader of field `ERROR_PACKET`"]
pub type ERROR_PACKET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_PACKET`"]
pub struct ERROR_PACKET_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_PACKET_W<'a> {
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
#[doc = "Reader of field `ERROR_SRC`"]
pub type ERROR_SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_SRC`"]
pub struct ERROR_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_SRC_W<'a> {
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
#[doc = "Reader of field `ERROR_DST`"]
pub type ERROR_DST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_DST`"]
pub struct ERROR_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_DST_W<'a> {
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
#[doc = "Reader of field `ERROR_PAGEFAULT`"]
pub type ERROR_PAGEFAULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_PAGEFAULT`"]
pub struct ERROR_PAGEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_PAGEFAULT_W<'a> {
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
#[doc = "Indicates the additional error codes for some of the error conditions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERROR_CODE_A {
    #[doc = "1: Error signalled because the next pointer is 0x00000000"]
    NEXT_CHAIN_IS_0 = 1,
    #[doc = "2: Error signalled because the semaphore is non-zero and neither chain bit is set"]
    NO_CHAIN = 2,
    #[doc = "3: Error signalled because an error is reported reading/writing the context buffer"]
    CONTEXT_ERROR = 3,
    #[doc = "4: Error signalled because an error is reported reading/writing the payload"]
    PAYLOAD_ERROR = 4,
    #[doc = "5: Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    INVALID_MODE = 5,
}
impl From<ERROR_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERROR_CODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ERROR_CODE`"]
pub type ERROR_CODE_R = crate::R<u8, ERROR_CODE_A>;
impl ERROR_CODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ERROR_CODE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ERROR_CODE_A::NEXT_CHAIN_IS_0),
            2 => Val(ERROR_CODE_A::NO_CHAIN),
            3 => Val(ERROR_CODE_A::CONTEXT_ERROR),
            4 => Val(ERROR_CODE_A::PAYLOAD_ERROR),
            5 => Val(ERROR_CODE_A::INVALID_MODE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEXT_CHAIN_IS_0`"]
    #[inline(always)]
    pub fn is_next_chain_is_0(&self) -> bool {
        *self == ERROR_CODE_A::NEXT_CHAIN_IS_0
    }
    #[doc = "Checks if the value of the field is `NO_CHAIN`"]
    #[inline(always)]
    pub fn is_no_chain(&self) -> bool {
        *self == ERROR_CODE_A::NO_CHAIN
    }
    #[doc = "Checks if the value of the field is `CONTEXT_ERROR`"]
    #[inline(always)]
    pub fn is_context_error(&self) -> bool {
        *self == ERROR_CODE_A::CONTEXT_ERROR
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_ERROR`"]
    #[inline(always)]
    pub fn is_payload_error(&self) -> bool {
        *self == ERROR_CODE_A::PAYLOAD_ERROR
    }
    #[doc = "Checks if the value of the field is `INVALID_MODE`"]
    #[inline(always)]
    pub fn is_invalid_mode(&self) -> bool {
        *self == ERROR_CODE_A::INVALID_MODE
    }
}
#[doc = "Write proxy for field `ERROR_CODE`"]
pub struct ERROR_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_CODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_CODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Error signalled because the next pointer is 0x00000000"]
    #[inline(always)]
    pub fn next_chain_is_0(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::NEXT_CHAIN_IS_0)
    }
    #[doc = "Error signalled because the semaphore is non-zero and neither chain bit is set"]
    #[inline(always)]
    pub fn no_chain(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::NO_CHAIN)
    }
    #[doc = "Error signalled because an error is reported reading/writing the context buffer"]
    #[inline(always)]
    pub fn context_error(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::CONTEXT_ERROR)
    }
    #[doc = "Error signalled because an error is reported reading/writing the payload"]
    #[inline(always)]
    pub fn payload_error(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::PAYLOAD_ERROR)
    }
    #[doc = "Error signalled because the control packet specifies an invalid mode select (for instance, blit + hash)"]
    #[inline(always)]
    pub fn invalid_mode(self) -> &'a mut W {
        self.variant(ERROR_CODE_A::INVALID_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 1 - This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub fn hash_mismatch(&self) -> HASH_MISMATCH_R {
        HASH_MISMATCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub fn error_setup(&self) -> ERROR_SETUP_R {
        ERROR_SETUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub fn error_packet(&self) -> ERROR_PACKET_R {
        ERROR_PACKET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub fn error_src(&self) -> ERROR_SRC_R {
        ERROR_SRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub fn error_dst(&self) -> ERROR_DST_R {
        ERROR_DST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub fn error_pagefault(&self) -> ERROR_PAGEFAULT_R {
        ERROR_PAGEFAULT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub fn error_code(&self) -> ERROR_CODE_R {
        ERROR_CODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates the tag from the last completed packet in the command structure"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - This bit indicates that a hashing check operation mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline(always)]
    pub fn hash_mismatch(&mut self) -> HASH_MISMATCH_W {
        HASH_MISMATCH_W { w: self }
    }
    #[doc = "Bit 2 - This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline(always)]
    pub fn error_setup(&mut self) -> ERROR_SETUP_W {
        ERROR_SETUP_W { w: self }
    }
    #[doc = "Bit 3 - This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet payload"]
    #[inline(always)]
    pub fn error_packet(&mut self) -> ERROR_PACKET_W {
        ERROR_PACKET_W { w: self }
    }
    #[doc = "Bit 4 - This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline(always)]
    pub fn error_src(&mut self) -> ERROR_SRC_W {
        ERROR_SRC_W { w: self }
    }
    #[doc = "Bit 5 - This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline(always)]
    pub fn error_dst(&mut self) -> ERROR_DST_W {
        ERROR_DST_W { w: self }
    }
    #[doc = "Bit 6 - This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline(always)]
    pub fn error_pagefault(&mut self) -> ERROR_PAGEFAULT_W {
        ERROR_PAGEFAULT_W { w: self }
    }
    #[doc = "Bits 16:23 - Indicates the additional error codes for some of the error conditions"]
    #[inline(always)]
    pub fn error_code(&mut self) -> ERROR_CODE_W {
        ERROR_CODE_W { w: self }
    }
}
