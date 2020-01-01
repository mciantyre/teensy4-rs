#[doc = "Reader of register LPSMCLR"]
pub type R = crate::R<u32, super::LPSMCLR>;
#[doc = "Reader of field `MON_COUNTER`"]
pub type MON_COUNTER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Monotonic Counter bits The MC is incremented by one when: A write transaction to the LPSMCMR or LPSMCLR Register is detected"]
    #[inline(always)]
    pub fn mon_counter(&self) -> MON_COUNTER_R {
        MON_COUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
