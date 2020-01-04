#[doc = "Reader of register PACKET2"]
pub type R = crate::R<u32, super::PACKET2>;
#[doc = "Cipher selection field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIPHER_SELECT_A {
    #[doc = "0: AES128"]
    AES128 = 0,
}
impl From<CIPHER_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CIPHER_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CIPHER_SELECT`"]
pub type CIPHER_SELECT_R = crate::R<u8, CIPHER_SELECT_A>;
impl CIPHER_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CIPHER_SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CIPHER_SELECT_A::AES128),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == CIPHER_SELECT_A::AES128
    }
}
#[doc = "Cipher mode selection field. Reflects the mode of operation for the cipher operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIPHER_MODE_A {
    #[doc = "0: ECB"]
    ECB = 0,
    #[doc = "1: CBC"]
    CBC = 1,
}
impl From<CIPHER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CIPHER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CIPHER_MODE`"]
pub type CIPHER_MODE_R = crate::R<u8, CIPHER_MODE_A>;
impl CIPHER_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CIPHER_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CIPHER_MODE_A::ECB),
            1 => Val(CIPHER_MODE_A::CBC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CIPHER_MODE_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CIPHER_MODE_A::CBC
    }
}
#[doc = "Key selection field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_SELECT_A {
    #[doc = "0: KEY0"]
    KEY0 = 0,
    #[doc = "1: KEY1"]
    KEY1 = 1,
    #[doc = "2: KEY2"]
    KEY2 = 2,
    #[doc = "3: KEY3"]
    KEY3 = 3,
    #[doc = "254: UNIQUE_KEY"]
    UNIQUE_KEY = 254,
    #[doc = "255: OTP_KEY"]
    OTP_KEY = 255,
}
impl From<KEY_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KEY_SELECT`"]
pub type KEY_SELECT_R = crate::R<u8, KEY_SELECT_A>;
impl KEY_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEY_SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KEY_SELECT_A::KEY0),
            1 => Val(KEY_SELECT_A::KEY1),
            2 => Val(KEY_SELECT_A::KEY2),
            3 => Val(KEY_SELECT_A::KEY3),
            254 => Val(KEY_SELECT_A::UNIQUE_KEY),
            255 => Val(KEY_SELECT_A::OTP_KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY0`"]
    #[inline(always)]
    pub fn is_key0(&self) -> bool {
        *self == KEY_SELECT_A::KEY0
    }
    #[doc = "Checks if the value of the field is `KEY1`"]
    #[inline(always)]
    pub fn is_key1(&self) -> bool {
        *self == KEY_SELECT_A::KEY1
    }
    #[doc = "Checks if the value of the field is `KEY2`"]
    #[inline(always)]
    pub fn is_key2(&self) -> bool {
        *self == KEY_SELECT_A::KEY2
    }
    #[doc = "Checks if the value of the field is `KEY3`"]
    #[inline(always)]
    pub fn is_key3(&self) -> bool {
        *self == KEY_SELECT_A::KEY3
    }
    #[doc = "Checks if the value of the field is `UNIQUE_KEY`"]
    #[inline(always)]
    pub fn is_unique_key(&self) -> bool {
        *self == KEY_SELECT_A::UNIQUE_KEY
    }
    #[doc = "Checks if the value of the field is `OTP_KEY`"]
    #[inline(always)]
    pub fn is_otp_key(&self) -> bool {
        *self == KEY_SELECT_A::OTP_KEY
    }
}
#[doc = "Hash Selection Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HASH_SELECT_A {
    #[doc = "0: SHA1"]
    SHA1 = 0,
    #[doc = "1: CRC32"]
    CRC32 = 1,
    #[doc = "2: SHA256"]
    SHA256 = 2,
}
impl From<HASH_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HASH_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HASH_SELECT`"]
pub type HASH_SELECT_R = crate::R<u8, HASH_SELECT_A>;
impl HASH_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HASH_SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HASH_SELECT_A::SHA1),
            1 => Val(HASH_SELECT_A::CRC32),
            2 => Val(HASH_SELECT_A::SHA256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == HASH_SELECT_A::SHA1
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == HASH_SELECT_A::CRC32
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == HASH_SELECT_A::SHA256
    }
}
#[doc = "Reader of field `CIPHER_CFG`"]
pub type CIPHER_CFG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Cipher selection field"]
    #[inline(always)]
    pub fn cipher_select(&self) -> CIPHER_SELECT_R {
        CIPHER_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Cipher mode selection field. Reflects the mode of operation for the cipher operations."]
    #[inline(always)]
    pub fn cipher_mode(&self) -> CIPHER_MODE_R {
        CIPHER_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Key selection field"]
    #[inline(always)]
    pub fn key_select(&self) -> KEY_SELECT_R {
        KEY_SELECT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Hash Selection Field"]
    #[inline(always)]
    pub fn hash_select(&self) -> HASH_SELECT_R {
        HASH_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Cipher configuration bits. Optional configuration bits are required for the ciphers."]
    #[inline(always)]
    pub fn cipher_cfg(&self) -> CIPHER_CFG_R {
        CIPHER_CFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
