#[doc = "Reader of register AES_KEY0_W1"]
pub type R = crate::R<u32, super::AES_KEY0_W1>;
#[doc = "Writer for register AES_KEY0_W1"]
pub type W = crate::W<u32, super::AES_KEY0_W1>;
#[doc = "Register AES_KEY0_W1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_KEY0_W1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY1`"]
pub type KEY1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEY1`"]
pub struct KEY1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY1_W<'a> {
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
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES 128 key from software"]
    #[inline(always)]
    pub fn key1(&mut self) -> KEY1_W {
        KEY1_W { w: self }
    }
}
