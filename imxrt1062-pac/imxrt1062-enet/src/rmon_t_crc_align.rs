#[doc = "Reader of register RMON_T_CRC_ALIGN"]
pub type R = crate::R<u32, super::RMON_T_CRC_ALIGN>;
#[doc = "Reader of field `TXPKTS`"]
pub type TXPKTS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Packets with CRC/align error"]
    #[inline(always)]
    pub fn txpkts(&self) -> TXPKTS_R {
        TXPKTS_R::new((self.bits & 0xffff) as u16)
    }
}
