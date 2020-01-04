#[doc = "Reader of register ISR2"]
pub type R = crate::R<u32, super::ISR2>;
#[doc = "Reader of field `ISR2`"]
pub type ISR2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[63:32\\]
status, read only"]
    #[inline(always)]
    pub fn isr2(&self) -> ISR2_R {
        ISR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
