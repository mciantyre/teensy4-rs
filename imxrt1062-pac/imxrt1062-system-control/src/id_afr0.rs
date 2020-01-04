#[doc = "Reader of register ID_AFR0"]
pub type R = crate::R<u32, super::ID_AFR0>;
#[doc = "Reader of field `IMPLEMENTATION_DEFINED0`"]
pub type IMPLEMENTATION_DEFINED0_R = crate::R<u8, u8>;
#[doc = "Reader of field `IMPLEMENTATION_DEFINED1`"]
pub type IMPLEMENTATION_DEFINED1_R = crate::R<u8, u8>;
#[doc = "Reader of field `IMPLEMENTATION_DEFINED2`"]
pub type IMPLEMENTATION_DEFINED2_R = crate::R<u8, u8>;
#[doc = "Reader of field `IMPLEMENTATION_DEFINED3`"]
pub type IMPLEMENTATION_DEFINED3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub fn implementation_defined0(&self) -> IMPLEMENTATION_DEFINED0_R {
        IMPLEMENTATION_DEFINED0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub fn implementation_defined1(&self) -> IMPLEMENTATION_DEFINED1_R {
        IMPLEMENTATION_DEFINED1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub fn implementation_defined2(&self) -> IMPLEMENTATION_DEFINED2_R {
        IMPLEMENTATION_DEFINED2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub fn implementation_defined3(&self) -> IMPLEMENTATION_DEFINED3_R {
        IMPLEMENTATION_DEFINED3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
