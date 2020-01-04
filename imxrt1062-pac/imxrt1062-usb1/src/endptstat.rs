#[doc = "Reader of register ENDPTSTAT"]
pub type R = crate::R<u32, super::ENDPTSTAT>;
#[doc = "Reader of field `ERBR`"]
pub type ERBR_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETBR`"]
pub type ETBR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint Receive Buffer Ready -- Read Only"]
    #[inline(always)]
    pub fn erbr(&self) -> ERBR_R {
        ERBR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Endpoint Transmit Buffer Ready -- Read Only"]
    #[inline(always)]
    pub fn etbr(&self) -> ETBR_R {
        ETBR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
