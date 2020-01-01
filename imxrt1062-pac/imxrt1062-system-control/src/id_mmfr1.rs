#[doc = "Reader of register ID_MMFR1"]
pub type R = crate::R<u32, super::ID_MMFR1>;
#[doc = "Reader of field `ID_MMFR1`"]
pub type ID_MMFR1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Gives information about the implemented memory model and memory management support."]
    #[inline(always)]
    pub fn id_mmfr1(&self) -> ID_MMFR1_R {
        ID_MMFR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
