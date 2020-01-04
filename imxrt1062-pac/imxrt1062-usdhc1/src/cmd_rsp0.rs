#[doc = "Reader of register CMD_RSP0"]
pub type R = crate::R<u32, super::CMD_RSP0>;
#[doc = "Reader of field `CMDRSP0`"]
pub type CMDRSP0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 0"]
    #[inline(always)]
    pub fn cmdrsp0(&self) -> CMDRSP0_R {
        CMDRSP0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
