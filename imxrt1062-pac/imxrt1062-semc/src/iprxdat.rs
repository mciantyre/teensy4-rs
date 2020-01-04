#[doc = "Reader of register IPRXDAT"]
pub type R = crate::R<u32, super::IPRXDAT>;
#[doc = "Reader of field `DAT`"]
pub type DAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
