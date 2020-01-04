#[doc = "Reader of register STS12"]
pub type R = crate::R<u32, super::STS12>;
#[doc = "Reader of field `NDADDR`"]
pub type NDADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
    #[inline(always)]
    pub fn ndaddr(&self) -> NDADDR_R {
        NDADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
