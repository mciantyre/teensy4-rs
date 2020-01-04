#[doc = "Reader of register LPGPR_alias[%s]"]
pub type R = crate::R<u32, super::LPGPR_ALIAS>;
#[doc = "Writer for register LPGPR_alias[%s]"]
pub type W = crate::W<u32, super::LPGPR_ALIAS>;
#[doc = "Register LPGPR_alias[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::LPGPR_ALIAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPR`"]
pub type GPR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPR`"]
pub struct GPR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[inline(always)]
    pub fn gpr(&self) -> GPR_R {
        GPR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose Register When GPR_SL or GPR_HL bit is set, the register cannot be programmed."]
    #[inline(always)]
    pub fn gpr(&mut self) -> GPR_W {
        GPR_W { w: self }
    }
}
