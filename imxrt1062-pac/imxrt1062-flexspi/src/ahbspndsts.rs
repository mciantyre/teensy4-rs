#[doc = "Reader of register AHBSPNDSTS"]
pub type R = crate::R<u32, super::AHBSPNDSTS>;
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFID`"]
pub type BUFID_R = crate::R<u8, u8>;
#[doc = "Reader of field `DATLFT`"]
pub type DATLFT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Indicates if an AHB read prefetch command sequence has been suspended."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - AHB RX BUF ID for suspended command sequence."]
    #[inline(always)]
    pub fn bufid(&self) -> BUFID_R {
        BUFID_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 16:31 - Left Data size for suspended command sequence (in byte)."]
    #[inline(always)]
    pub fn datlft(&self) -> DATLFT_R {
        DATLFT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
