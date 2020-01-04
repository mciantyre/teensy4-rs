#[doc = "Reader of register CAPLENGTH"]
pub type R = crate::R<u8, super::CAPLENGTH>;
#[doc = "Reader of field `CAPLENGTH`"]
pub type CAPLENGTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits are used as an offset to add to register base to find the beginning of the Operational Register"]
    #[inline(always)]
    pub fn caplength(&self) -> CAPLENGTH_R {
        CAPLENGTH_R::new((self.bits & 0xff) as u8)
    }
}
