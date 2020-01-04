#[doc = "Reader of register CRCR"]
pub type R = crate::R<u32, super::CRCR>;
#[doc = "Reader of field `TXCRC`"]
pub type TXCRC_R = crate::R<u16, u16>;
#[doc = "Reader of field `MBCRC`"]
pub type MBCRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:14 - Transmitted CRC value"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - CRC Mailbox"]
    #[inline(always)]
    pub fn mbcrc(&self) -> MBCRC_R {
        MBCRC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
