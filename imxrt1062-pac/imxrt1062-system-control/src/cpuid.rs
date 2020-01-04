#[doc = "Reader of register CPUID"]
pub type R = crate::R<u32, super::CPUID>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `PARTNO`"]
pub type PARTNO_R = crate::R<u16, u16>;
#[doc = "Reader of field `ARCHITECTURE`"]
pub type ARCHITECTURE_R = crate::R<u8, u8>;
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u8, u8>;
#[doc = "Reader of field `IMPLEMENTER`"]
pub type IMPLEMENTER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Indicates patch release: 0x0 = Patch 0"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Indicates part number"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - ARCHITECTURE"]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates processor revision: 0x2 = Revision 2"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
