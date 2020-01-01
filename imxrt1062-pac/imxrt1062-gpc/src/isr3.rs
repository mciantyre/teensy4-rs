#[doc = "Reader of register ISR3"]
pub type R = crate::R<u32, super::ISR3>;
#[doc = "Reader of field `ISR3`"]
pub type ISR3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[95:64\\]
status, read only"]
    #[inline(always)]
    pub fn isr3(&self) -> ISR3_R {
        ISR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
