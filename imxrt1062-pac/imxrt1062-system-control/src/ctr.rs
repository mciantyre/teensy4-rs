#[doc = "Reader of register CTR"]
pub type R = crate::R<u32, super::CTR>;
#[doc = "Reader of field `IMINLINE`"]
pub type IMINLINE_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMINLINE`"]
pub type DMINLINE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ERG`"]
pub type ERG_R = crate::R<u8, u8>;
#[doc = "Reader of field `CWG`"]
pub type CWG_R = crate::R<u8, u8>;
#[doc = "Indicates the implemented CTR format.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "4: ARMv7 format."]
    FORMAT_4 = 4,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORMAT`"]
pub type FORMAT_R = crate::R<u8, FORMAT_A>;
impl FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FORMAT_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(FORMAT_A::FORMAT_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FORMAT_4`"]
    #[inline(always)]
    pub fn is_format_4(&self) -> bool {
        *self == FORMAT_A::FORMAT_4
    }
}
impl R {
    #[doc = "Bits 0:3 - Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the processor."]
    #[inline(always)]
    pub fn iminline(&self) -> IMINLINE_R {
        IMINLINE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the processor."]
    #[inline(always)]
    pub fn dminline(&self) -> DMINLINE_R {
        DMINLINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Exclusives Reservation Granule. The maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions, encoded as Log2 of the number of words."]
    #[inline(always)]
    pub fn erg(&self) -> ERG_R {
        ERG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Cache Write-back Granule. The maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified, encoded as Log2 of the number of words."]
    #[inline(always)]
    pub fn cwg(&self) -> CWG_R {
        CWG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - Indicates the implemented CTR format."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
