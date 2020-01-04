#[doc = "Reader of register CMD_RSP1"]
pub type R = crate::R<u32, super::CMD_RSP1>;
#[doc = "Reader of field `CMDRSP1`"]
pub type CMDRSP1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 1"]
    #[inline(always)]
    pub fn cmdrsp1(&self) -> CMDRSP1_R {
        CMDRSP1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
