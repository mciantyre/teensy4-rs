#[doc = "Reader of register CAPABILITY1"]
pub type R = crate::R<u32, super::CAPABILITY1>;
#[doc = "One-hot field indicating which cipher algorithms are available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CIPHER_ALGORITHMS_A {
    #[doc = "1: AES128"]
    AES128 = 1,
}
impl From<CIPHER_ALGORITHMS_A> for u16 {
    #[inline(always)]
    fn from(variant: CIPHER_ALGORITHMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CIPHER_ALGORITHMS`"]
pub type CIPHER_ALGORITHMS_R = crate::R<u16, CIPHER_ALGORITHMS_A>;
impl CIPHER_ALGORITHMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CIPHER_ALGORITHMS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CIPHER_ALGORITHMS_A::AES128),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == CIPHER_ALGORITHMS_A::AES128
    }
}
#[doc = "One-hot field indicating which hashing features are implemented in the hardware\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum HASH_ALGORITHMS_A {
    #[doc = "1: SHA1"]
    SHA1 = 1,
    #[doc = "2: CRC32"]
    CRC32 = 2,
    #[doc = "4: SHA256"]
    SHA256 = 4,
}
impl From<HASH_ALGORITHMS_A> for u16 {
    #[inline(always)]
    fn from(variant: HASH_ALGORITHMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HASH_ALGORITHMS`"]
pub type HASH_ALGORITHMS_R = crate::R<u16, HASH_ALGORITHMS_A>;
impl HASH_ALGORITHMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, HASH_ALGORITHMS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(HASH_ALGORITHMS_A::SHA1),
            2 => Val(HASH_ALGORITHMS_A::CRC32),
            4 => Val(HASH_ALGORITHMS_A::SHA256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == HASH_ALGORITHMS_A::SHA1
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == HASH_ALGORITHMS_A::CRC32
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == HASH_ALGORITHMS_A::SHA256
    }
}
impl R {
    #[doc = "Bits 0:15 - One-hot field indicating which cipher algorithms are available"]
    #[inline(always)]
    pub fn cipher_algorithms(&self) -> CIPHER_ALGORITHMS_R {
        CIPHER_ALGORITHMS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - One-hot field indicating which hashing features are implemented in the hardware"]
    #[inline(always)]
    pub fn hash_algorithms(&self) -> HASH_ALGORITHMS_R {
        HASH_ALGORITHMS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
