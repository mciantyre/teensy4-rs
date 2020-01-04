#[doc = "Reader of register CSIIMAG_PARA"]
pub type R = crate::R<u32, super::CSIIMAG_PARA>;
#[doc = "Writer for register CSIIMAG_PARA"]
pub type W = crate::W<u32, super::CSIIMAG_PARA>;
#[doc = "Register CSIIMAG_PARA `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIIMAG_PARA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMAGE_HEIGHT`"]
pub type IMAGE_HEIGHT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IMAGE_HEIGHT`"]
pub struct IMAGE_HEIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAGE_HEIGHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `IMAGE_WIDTH`"]
pub type IMAGE_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IMAGE_WIDTH`"]
pub struct IMAGE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAGE_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Image Height. Indicates how many pixels in a column of the image from the sensor."]
    #[inline(always)]
    pub fn image_height(&self) -> IMAGE_HEIGHT_R {
        IMAGE_HEIGHT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Image Width"]
    #[inline(always)]
    pub fn image_width(&self) -> IMAGE_WIDTH_R {
        IMAGE_WIDTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Image Height. Indicates how many pixels in a column of the image from the sensor."]
    #[inline(always)]
    pub fn image_height(&mut self) -> IMAGE_HEIGHT_W {
        IMAGE_HEIGHT_W { w: self }
    }
    #[doc = "Bits 16:31 - Image Width"]
    #[inline(always)]
    pub fn image_width(&mut self) -> IMAGE_WIDTH_W {
        IMAGE_WIDTH_W { w: self }
    }
}
