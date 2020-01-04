#[doc = "Reader of register CMD_RSP3"]
pub type R = crate::R<u32, super::CMD_RSP3>;
#[doc = "Reader of field `CMDRSP3`"]
pub type CMDRSP3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 3"]
    #[inline(always)]
    pub fn cmdrsp3(&self) -> CMDRSP3_R {
        CMDRSP3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
