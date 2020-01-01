#[doc = "Reader of register CV"]
pub type R = crate::R<u32, super::CV>;
#[doc = "Writer for register CV"]
pub type W = crate::W<u32, super::CV>;
#[doc = "Register CV `reset()`'s with value 0"]
impl crate::ResetValue for super::CV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CV1`"]
pub type CV1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CV1`"]
pub struct CV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `CV2`"]
pub type CV2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CV2`"]
pub struct CV2_W<'a> {
    w: &'a mut W,
}
impl<'a> CV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Compare Value 1"]
    #[inline(always)]
    pub fn cv1(&self) -> CV1_R {
        CV1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Compare Value 2"]
    #[inline(always)]
    pub fn cv2(&self) -> CV2_R {
        CV2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare Value 1"]
    #[inline(always)]
    pub fn cv1(&mut self) -> CV1_W {
        CV1_W { w: self }
    }
    #[doc = "Bits 16:27 - Compare Value 2"]
    #[inline(always)]
    pub fn cv2(&mut self) -> CV2_W {
        CV2_W { w: self }
    }
}
