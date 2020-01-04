#[doc = "Reader of register KEY"]
pub type R = crate::R<u32, super::KEY>;
#[doc = "Writer for register KEY"]
pub type W = crate::W<u32, super::KEY>;
#[doc = "Register KEY `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUBWORD`"]
pub type SUBWORD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUBWORD`"]
pub struct SUBWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `INDEX`"]
pub type INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INDEX`"]
pub struct INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Key subword pointer"]
    #[inline(always)]
    pub fn subword(&self) -> SUBWORD_R {
        SUBWORD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Key index pointer. The valid indices are 0-\\[number_keys\\]."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Key subword pointer"]
    #[inline(always)]
    pub fn subword(&mut self) -> SUBWORD_W {
        SUBWORD_W { w: self }
    }
    #[doc = "Bits 4:5 - Key index pointer. The valid indices are 0-\\[number_keys\\]."]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W {
        INDEX_W { w: self }
    }
}
