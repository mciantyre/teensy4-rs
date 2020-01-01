#[doc = "Reader of register IMASK1"]
pub type R = crate::R<u32, super::IMASK1>;
#[doc = "Writer for register IMASK1"]
pub type W = crate::W<u32, super::IMASK1>;
#[doc = "Register IMASK1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF31TO0M`"]
pub type BUF31TO0M_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF31TO0M`"]
pub struct BUF31TO0M_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO0M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buf31to0m(&self) -> BUF31TO0M_R {
        BUF31TO0M_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buf31to0m(&mut self) -> BUF31TO0M_W {
        BUF31TO0M_W { w: self }
    }
}
