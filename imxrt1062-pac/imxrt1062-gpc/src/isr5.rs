#[doc = "Reader of register ISR5"]
pub type R = crate::R<u32, super::ISR5>;
#[doc = "Reader of field `ISR4`"]
pub type ISR4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[159:128\\]
status, read only"]
    #[inline(always)]
    pub fn isr4(&self) -> ISR4_R {
        ISR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
