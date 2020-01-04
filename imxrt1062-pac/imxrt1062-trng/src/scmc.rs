#[doc = "Reader of register SCMC"]
pub type R = crate::R<u32, super::SCMC>;
#[doc = "Reader of field `MONO_CT`"]
pub type MONO_CT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Monobit Count"]
    #[inline(always)]
    pub fn mono_ct(&self) -> MONO_CT_R {
        MONO_CT_R::new((self.bits & 0xffff) as u16)
    }
}
