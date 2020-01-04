#[doc = "Reader of register ISR1"]
pub type R = crate::R<u32, super::ISR1>;
#[doc = "Reader of field `ISR1`"]
pub type ISR1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[31:0\\]
status, read only"]
    #[inline(always)]
    pub fn isr1(&self) -> ISR1_R {
        ISR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
