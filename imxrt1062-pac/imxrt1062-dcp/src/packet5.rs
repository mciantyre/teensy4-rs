#[doc = "Reader of register PACKET5"]
pub type R = crate::R<u32, super::PACKET5>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Byte count register. This value is the working value and updates as the operation proceeds."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
