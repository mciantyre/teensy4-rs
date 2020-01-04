#[doc = "Reader of register DIGPROG"]
pub type R = crate::R<u32, super::DIGPROG>;
#[doc = "Chip silicon revision\n\nValue on reset: 7077888"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SILICON_REVISION_A {
    #[doc = "7077888: Silicon revision 1.0"]
    SILICON_REVISION_7077888 = 7077888,
}
impl From<SILICON_REVISION_A> for u32 {
    #[inline(always)]
    fn from(variant: SILICON_REVISION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SILICON_REVISION`"]
pub type SILICON_REVISION_R = crate::R<u32, SILICON_REVISION_A>;
impl SILICON_REVISION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SILICON_REVISION_A> {
        use crate::Variant::*;
        match self.bits {
            7077888 => Val(SILICON_REVISION_A::SILICON_REVISION_7077888),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SILICON_REVISION_7077888`"]
    #[inline(always)]
    pub fn is_silicon_revision_7077888(&self) -> bool {
        *self == SILICON_REVISION_A::SILICON_REVISION_7077888
    }
}
impl R {
    #[doc = "Bits 0:31 - Chip silicon revision"]
    #[inline(always)]
    pub fn silicon_revision(&self) -> SILICON_REVISION_R {
        SILICON_REVISION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
