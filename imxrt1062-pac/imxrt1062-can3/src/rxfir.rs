#[doc = "Reader of register RXFIR"]
pub type R = crate::R<u32, super::RXFIR>;
#[doc = "Reader of field `IDHIT`"]
pub type IDHIT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - Identifier Acceptance Filter Hit Indicator"]
    #[inline(always)]
    pub fn idhit(&self) -> IDHIT_R {
        IDHIT_R::new((self.bits & 0x01ff) as u16)
    }
}
