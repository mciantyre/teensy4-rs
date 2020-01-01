#[doc = "Reader of register LPSMCMR"]
pub type R = crate::R<u32, super::LPSMCMR>;
#[doc = "Reader of field `MON_COUNTER`"]
pub type MON_COUNTER_R = crate::R<u16, u16>;
#[doc = "Reader of field `MC_ERA_BITS`"]
pub type MC_ERA_BITS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Monotonic Counter most-significant 16 Bits The MC is incremented by one when: A write transaction to the LPSMCMR or LPSMCLR register is detected"]
    #[inline(always)]
    pub fn mon_counter(&self) -> MON_COUNTER_R {
        MON_COUNTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses"]
    #[inline(always)]
    pub fn mc_era_bits(&self) -> MC_ERA_BITS_R {
        MC_ERA_BITS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
