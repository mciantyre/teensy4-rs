#[doc = "Reader of register CMD_RSP2"]
pub type R = crate::R<u32, super::CMD_RSP2>;
#[doc = "Reader of field `CMDRSP2`"]
pub type CMDRSP2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 2"]
    #[inline(always)]
    pub fn cmdrsp2(&self) -> CMDRSP2_R {
        CMDRSP2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
