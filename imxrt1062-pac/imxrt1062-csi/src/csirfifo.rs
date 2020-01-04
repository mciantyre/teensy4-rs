#[doc = "Reader of register CSIRFIFO"]
pub type R = crate::R<u32, super::CSIRFIFO>;
#[doc = "Reader of field `IMAGE`"]
pub type IMAGE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received image data"]
    #[inline(always)]
    pub fn image(&self) -> IMAGE_R {
        IMAGE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
