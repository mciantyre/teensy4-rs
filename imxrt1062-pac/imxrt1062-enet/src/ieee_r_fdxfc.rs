#[doc = "Reader of register IEEE_R_FDXFC"]
pub type R = crate::R<u32, super::IEEE_R_FDXFC>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of flow-control pause frames received"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
