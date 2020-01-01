#[doc = "Reader of register ISR4"]
pub type R = crate::R<u32, super::ISR4>;
#[doc = "Reader of field `ISR4`"]
pub type ISR4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[127:96\\]
status, read only"]
    #[inline(always)]
    pub fn isr4(&self) -> ISR4_R {
        ISR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
