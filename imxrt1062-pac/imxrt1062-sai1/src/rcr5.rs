#[doc = "Reader of register RCR5"]
pub type R = crate::R<u32, super::RCR5>;
#[doc = "Writer for register RCR5"]
pub type W = crate::W<u32, super::RCR5>;
#[doc = "Register RCR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FBT`"]
pub type FBT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FBT`"]
pub struct FBT_W<'a> {
    w: &'a mut W,
}
impl<'a> FBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `W0W`"]
pub type W0W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `W0W`"]
pub struct W0W_W<'a> {
    w: &'a mut W,
}
impl<'a> W0W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WNW`"]
pub type WNW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WNW`"]
pub struct WNW_W<'a> {
    w: &'a mut W,
}
impl<'a> WNW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - First Bit Shifted"]
    #[inline(always)]
    pub fn fbt(&self) -> FBT_R {
        FBT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Word 0 Width"]
    #[inline(always)]
    pub fn w0w(&self) -> W0W_R {
        W0W_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Word N Width"]
    #[inline(always)]
    pub fn wnw(&self) -> WNW_R {
        WNW_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - First Bit Shifted"]
    #[inline(always)]
    pub fn fbt(&mut self) -> FBT_W {
        FBT_W { w: self }
    }
    #[doc = "Bits 16:20 - Word 0 Width"]
    #[inline(always)]
    pub fn w0w(&mut self) -> W0W_W {
        W0W_W { w: self }
    }
    #[doc = "Bits 24:28 - Word N Width"]
    #[inline(always)]
    pub fn wnw(&mut self) -> WNW_W {
        WNW_W { w: self }
    }
}
