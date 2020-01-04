#[doc = "Reader of register DEBUG0_STATUS"]
pub type R = crate::R<u32, super::DEBUG0_STATUS>;
#[doc = "Reader of field `LOOP_BACK_FAIL_COUNT`"]
pub type LOOP_BACK_FAIL_COUNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `UTMI_RXERROR_FAIL_COUNT`"]
pub type UTMI_RXERROR_FAIL_COUNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `SQUELCH_COUNT`"]
pub type SQUELCH_COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Running count of the failed pseudo-random generator loopback"]
    #[inline(always)]
    pub fn loop_back_fail_count(&self) -> LOOP_BACK_FAIL_COUNT_R {
        LOOP_BACK_FAIL_COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Running count of the UTMI_RXERROR."]
    #[inline(always)]
    pub fn utmi_rxerror_fail_count(&self) -> UTMI_RXERROR_FAIL_COUNT_R {
        UTMI_RXERROR_FAIL_COUNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - Running count of the squelch reset instead of normal end for HS RX."]
    #[inline(always)]
    pub fn squelch_count(&self) -> SQUELCH_COUNT_R {
        SQUELCH_COUNT_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
