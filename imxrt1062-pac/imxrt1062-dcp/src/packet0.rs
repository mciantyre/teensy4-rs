#[doc = "Reader of register PACKET0"]
pub type R = crate::R<u32, super::PACKET0>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Next pointer register"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
