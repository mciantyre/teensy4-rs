#[doc = "Reader of register IEEE_T_EXCOL"]
pub type R = crate::R<u32, super::IEEE_T_EXCOL>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of frames transmitted with excessive collisions"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
