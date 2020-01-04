#[doc = "Reader of register ID_MMFR3"]
pub type R = crate::R<u32, super::ID_MMFR3>;
#[doc = "Reader of field `ID_MMFR3`"]
pub type ID_MMFR3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Gives information about the implemented memory model and memory management support."]
    #[inline(always)]
    pub fn id_mmfr3(&self) -> ID_MMFR3_R {
        ID_MMFR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
