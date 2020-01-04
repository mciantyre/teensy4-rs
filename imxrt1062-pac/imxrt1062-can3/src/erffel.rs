#[doc = "Reader of register ERFFEL[%s]"]
pub type R = crate::R<u32, super::ERFFEL>;
#[doc = "Writer for register ERFFEL[%s]"]
pub type W = crate::W<u32, super::ERFFEL>;
#[doc = "Register ERFFEL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::ERFFEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FEL`"]
pub type FEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FEL`"]
pub struct FEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Filter Element Bits"]
    #[inline(always)]
    pub fn fel(&self) -> FEL_R {
        FEL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Filter Element Bits"]
    #[inline(always)]
    pub fn fel(&mut self) -> FEL_W {
        FEL_W { w: self }
    }
}
