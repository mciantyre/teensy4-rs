#[doc = "Reader of register IMASK2"]
pub type R = crate::R<u32, super::IMASK2>;
#[doc = "Writer for register IMASK2"]
pub type W = crate::W<u32, super::IMASK2>;
#[doc = "Register IMASK2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMASK2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF63TO32M`"]
pub type BUF63TO32M_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF63TO32M`"]
pub struct BUF63TO32M_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF63TO32M_W<'a> {
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
    pub fn buf63to32m(&self) -> BUF63TO32M_R {
        BUF63TO32M_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MB i Mask"]
    #[inline(always)]
    pub fn buf63to32m(&mut self) -> BUF63TO32M_W {
        BUF63TO32M_W { w: self }
    }
}
