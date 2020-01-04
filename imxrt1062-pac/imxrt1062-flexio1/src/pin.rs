#[doc = "Reader of register PIN"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Reader of field `PDI`"]
pub type PDI_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pin Data Input"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new((self.bits & 0xffff) as u16)
    }
}
