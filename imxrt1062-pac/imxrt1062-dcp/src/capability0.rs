#[doc = "Reader of register CAPABILITY0"]
pub type R = crate::R<u32, super::CAPABILITY0>;
#[doc = "Writer for register CAPABILITY0"]
pub type W = crate::W<u32, super::CAPABILITY0>;
#[doc = "Register CAPABILITY0 `reset()`'s with value 0x0404"]
impl crate::ResetValue for super::CAPABILITY0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0404
    }
}
#[doc = "Reader of field `NUM_KEYS`"]
pub type NUM_KEYS_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUM_CHANNELS`"]
pub type NUM_CHANNELS_R = crate::R<u8, u8>;
#[doc = "Reader of field `DISABLE_UNIQUE_KEY`"]
pub type DISABLE_UNIQUE_KEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_UNIQUE_KEY`"]
pub struct DISABLE_UNIQUE_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_UNIQUE_KEY_W<'a> {
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
#[doc = "Reader of field `DISABLE_DECRYPT`"]
pub type DISABLE_DECRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_DECRYPT`"]
pub struct DISABLE_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_DECRYPT_W<'a> {
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
    #[doc = "Bits 0:7 - Encoded value indicating the number of key-storage locations implemented in the design"]
    #[inline(always)]
    pub fn num_keys(&self) -> NUM_KEYS_R {
        NUM_KEYS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Encoded value indicating the number of channels implemented in the design"]
    #[inline(always)]
    pub fn num_channels(&self) -> NUM_CHANNELS_R {
        NUM_CHANNELS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Write to a 1 to disable the per-device unique key"]
    #[inline(always)]
    pub fn disable_unique_key(&self) -> DISABLE_UNIQUE_KEY_R {
        DISABLE_UNIQUE_KEY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write to 1 to disable the decryption"]
    #[inline(always)]
    pub fn disable_decrypt(&self) -> DISABLE_DECRYPT_R {
        DISABLE_DECRYPT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Write to a 1 to disable the per-device unique key"]
    #[inline(always)]
    pub fn disable_unique_key(&mut self) -> DISABLE_UNIQUE_KEY_W {
        DISABLE_UNIQUE_KEY_W { w: self }
    }
    #[doc = "Bit 31 - Write to 1 to disable the decryption"]
    #[inline(always)]
    pub fn disable_decrypt(&mut self) -> DISABLE_DECRYPT_W {
        DISABLE_DECRYPT_W { w: self }
    }
}
