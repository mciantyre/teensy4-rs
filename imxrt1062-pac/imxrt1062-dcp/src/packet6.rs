#[doc = "Reader of register PACKET6"]
pub type R = crate::R<u32, super::PACKET6>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This regiser reflects the payload pointer for the current control packet."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
