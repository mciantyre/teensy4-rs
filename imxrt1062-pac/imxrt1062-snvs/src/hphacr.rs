#[doc = "Reader of register HPHACR"]
pub type R = crate::R<u32, super::HPHACR>;
#[doc = "Reader of field `HAC_COUNTER`"]
pub type HAC_COUNTER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - High Assurance Counter When the HAC_EN bit is set and the SSM is in the soft fail state, this counter starts to count down with the system clock"]
    #[inline(always)]
    pub fn hac_counter(&self) -> HAC_COUNTER_R {
        HAC_COUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
