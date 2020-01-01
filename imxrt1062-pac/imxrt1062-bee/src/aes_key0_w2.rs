#[doc = "Reader of register AES_KEY0_W2"]
pub type R = crate::R<u32, super::AES_KEY0_W2>;
#[doc = "Writer for register AES_KEY0_W2"]
pub type W = crate::W<u32, super::AES_KEY0_W2>;
#[doc = "Register AES_KEY0_W2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_KEY0_W2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY2`"]
pub type KEY2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEY2`"]
pub struct KEY2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES 128 key from software"]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES 128 key from software"]
    #[inline(always)]
    pub fn key2(&mut self) -> KEY2_W {
        KEY2_W { w: self }
    }
}
