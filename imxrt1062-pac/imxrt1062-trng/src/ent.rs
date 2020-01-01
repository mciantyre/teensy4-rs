#[doc = "Reader of register ENT[%s]"]
pub type R = crate::R<u32, super::ENT>;
#[doc = "Reader of field `ENT`"]
pub type ENT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Entropy Value"]
    #[inline(always)]
    pub fn ent(&self) -> ENT_R {
        ENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
