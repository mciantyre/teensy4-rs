#[doc = "Reader of register FDCRC"]
pub type R = crate::R<u32, super::FDCRC>;
#[doc = "Reader of field `FD_TXCRC`"]
pub type FD_TXCRC_R = crate::R<u32, u32>;
#[doc = "Reader of field `FD_MBCRC`"]
pub type FD_MBCRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:20 - Extended Transmitted CRC value"]
    #[inline(always)]
    pub fn fd_txcrc(&self) -> FD_TXCRC_R {
        FD_TXCRC_R::new((self.bits & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 24:30 - CRC Mailbox Number for FD_TXCRC"]
    #[inline(always)]
    pub fn fd_mbcrc(&self) -> FD_MBCRC_R {
        FD_MBCRC_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
