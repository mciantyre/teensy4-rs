#[doc = "Reader of register PACKET1"]
pub type R = crate::R<u32, super::PACKET1>;
#[doc = "Reader of field `INTERRUPT`"]
pub type INTERRUPT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DECR_SEMAPHORE`"]
pub type DECR_SEMAPHORE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHAIN`"]
pub type CHAIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHAIN_CONTIGUOUS`"]
pub type CHAIN_CONTIGUOUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE_MEMCOPY`"]
pub type ENABLE_MEMCOPY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE_CIPHER`"]
pub type ENABLE_CIPHER_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE_HASH`"]
pub type ENABLE_HASH_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE_BLIT`"]
pub type ENABLE_BLIT_R = crate::R<bool, bool>;
#[doc = "When the cipher block is enabled, this bit indicates whether the operation is encryption or decryption\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_ENCRYPT_A {
    #[doc = "0: DECRYPT"]
    DECRYPT = 0,
    #[doc = "1: ENCRYPT"]
    ENCRYPT = 1,
}
impl From<CIPHER_ENCRYPT_A> for bool {
    #[inline(always)]
    fn from(variant: CIPHER_ENCRYPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CIPHER_ENCRYPT`"]
pub type CIPHER_ENCRYPT_R = crate::R<bool, CIPHER_ENCRYPT_A>;
impl CIPHER_ENCRYPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIPHER_ENCRYPT_A {
        match self.bits {
            false => CIPHER_ENCRYPT_A::DECRYPT,
            true => CIPHER_ENCRYPT_A::ENCRYPT,
        }
    }
    #[doc = "Checks if the value of the field is `DECRYPT`"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == CIPHER_ENCRYPT_A::DECRYPT
    }
    #[doc = "Checks if the value of the field is `ENCRYPT`"]
    #[inline(always)]
    pub fn is_encrypt(&self) -> bool {
        *self == CIPHER_ENCRYPT_A::ENCRYPT
    }
}
#[doc = "Reader of field `CIPHER_INIT`"]
pub type CIPHER_INIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTP_KEY`"]
pub type OTP_KEY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PAYLOAD_KEY`"]
pub type PAYLOAD_KEY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HASH_INIT`"]
pub type HASH_INIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `HASH_TERM`"]
pub type HASH_TERM_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHECK_HASH`"]
pub type CHECK_HASH_R = crate::R<bool, bool>;
#[doc = "When the hashing is enabled, this bit controls whether the input or output data is hashed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_OUTPUT_A {
    #[doc = "0: INPUT"]
    INPUT = 0,
    #[doc = "1: OUTPUT"]
    OUTPUT = 1,
}
impl From<HASH_OUTPUT_A> for bool {
    #[inline(always)]
    fn from(variant: HASH_OUTPUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HASH_OUTPUT`"]
pub type HASH_OUTPUT_R = crate::R<bool, HASH_OUTPUT_A>;
impl HASH_OUTPUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_OUTPUT_A {
        match self.bits {
            false => HASH_OUTPUT_A::INPUT,
            true => HASH_OUTPUT_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == HASH_OUTPUT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == HASH_OUTPUT_A::OUTPUT
    }
}
#[doc = "Reader of field `CONSTANT_FILL`"]
pub type CONSTANT_FILL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEST_SEMA_IRQ`"]
pub type TEST_SEMA_IRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY_BYTESWAP`"]
pub type KEY_BYTESWAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY_WORDSWAP`"]
pub type KEY_WORDSWAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `INPUT_BYTESWAP`"]
pub type INPUT_BYTESWAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `INPUT_WORDSWAP`"]
pub type INPUT_WORDSWAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTPUT_BYTESWAP`"]
pub type OUTPUT_BYTESWAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTPUT_WORDSWAP`"]
pub type OUTPUT_WORDSWAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Reflects whether the channel must issue an interrupt upon the completion of the packet."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reflects whether the channel's semaphore must be decremented at the end of the current operation"]
    #[inline(always)]
    pub fn decr_semaphore(&self) -> DECR_SEMAPHORE_R {
        DECR_SEMAPHORE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reflects whether the next command pointer register must be loaded into the channel's current descriptor pointer"]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reflects whether the next packet's address is located following this packet's payload."]
    #[inline(always)]
    pub fn chain_contiguous(&self) -> CHAIN_CONTIGUOUS_R {
        CHAIN_CONTIGUOUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reflects whether the selected hashing function should be enabled for this operation."]
    #[inline(always)]
    pub fn enable_memcopy(&self) -> ENABLE_MEMCOPY_R {
        ENABLE_MEMCOPY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reflects whether the selected cipher function must be enabled for this operation."]
    #[inline(always)]
    pub fn enable_cipher(&self) -> ENABLE_CIPHER_R {
        ENABLE_CIPHER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reflects whether the selected hashing function must be enabled for this operation."]
    #[inline(always)]
    pub fn enable_hash(&self) -> ENABLE_HASH_R {
        ENABLE_HASH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reflects whether the DCP must perform a blit operation"]
    #[inline(always)]
    pub fn enable_blit(&self) -> ENABLE_BLIT_R {
        ENABLE_BLIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When the cipher block is enabled, this bit indicates whether the operation is encryption or decryption"]
    #[inline(always)]
    pub fn cipher_encrypt(&self) -> CIPHER_ENCRYPT_R {
        CIPHER_ENCRYPT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reflects whether the cipher block must load the initialization vector from the payload for this operation"]
    #[inline(always)]
    pub fn cipher_init(&self) -> CIPHER_INIT_R {
        CIPHER_INIT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reflects whether a hardware-based key must be used"]
    #[inline(always)]
    pub fn otp_key(&self) -> OTP_KEY_R {
        OTP_KEY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When set, it indicates the payload contains the key"]
    #[inline(always)]
    pub fn payload_key(&self) -> PAYLOAD_KEY_R {
        PAYLOAD_KEY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reflects whether the current hashing block is the initial block in the hashing operation, so the hash registers must be initialized before the operation"]
    #[inline(always)]
    pub fn hash_init(&self) -> HASH_INIT_R {
        HASH_INIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reflects whether the current hashing block is the final block in the hashing operation, so the hash padding must be applied by the hardware"]
    #[inline(always)]
    pub fn hash_term(&self) -> HASH_TERM_R {
        HASH_TERM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Reflects whether the calculated hash value must be compared to the hash provided in the payload."]
    #[inline(always)]
    pub fn check_hash(&self) -> CHECK_HASH_R {
        CHECK_HASH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - When the hashing is enabled, this bit controls whether the input or output data is hashed."]
    #[inline(always)]
    pub fn hash_output(&self) -> HASH_OUTPUT_R {
        HASH_OUTPUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - When this bit is set (MEMCOPY and BLIT modes only), the DCP simply fills the destination buffer with the value found in the source address field"]
    #[inline(always)]
    pub fn constant_fill(&self) -> CONSTANT_FILL_R {
        CONSTANT_FILL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This bit is used to test the channel semaphore transition to 0. FOR TEST USE ONLY!"]
    #[inline(always)]
    pub fn test_sema_irq(&self) -> TEST_SEMA_IRQ_R {
        TEST_SEMA_IRQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reflects whether the DCP engine swaps the key bytes (big-endian key)."]
    #[inline(always)]
    pub fn key_byteswap(&self) -> KEY_BYTESWAP_R {
        KEY_BYTESWAP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Reflects whether the DCP engine swaps the key words (big-endian key)."]
    #[inline(always)]
    pub fn key_wordswap(&self) -> KEY_WORDSWAP_R {
        KEY_WORDSWAP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Reflects whether the DCP engine byteswaps the input data (big-endian data)."]
    #[inline(always)]
    pub fn input_byteswap(&self) -> INPUT_BYTESWAP_R {
        INPUT_BYTESWAP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Reflects whether the DCP engine wordswaps the input data (big-endian data)."]
    #[inline(always)]
    pub fn input_wordswap(&self) -> INPUT_WORDSWAP_R {
        INPUT_WORDSWAP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Reflects whether the DCP engine byteswaps the output data (big-endian data)."]
    #[inline(always)]
    pub fn output_byteswap(&self) -> OUTPUT_BYTESWAP_R {
        OUTPUT_BYTESWAP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Reflects whether the DCP engine wordswaps the output data (big-endian data)."]
    #[inline(always)]
    pub fn output_wordswap(&self) -> OUTPUT_WORDSWAP_R {
        OUTPUT_WORDSWAP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Packet Tag"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
