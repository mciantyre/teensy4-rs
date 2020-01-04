#[doc = "Reader of register DMA%s"]
pub type R = crate::R<u16, super::DMA>;
#[doc = "Writer for register DMA%s"]
pub type W = crate::W<u16, super::DMA>;
#[doc = "Register DMA%s `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEFDE`"]
pub type IEFDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEFDE`"]
pub struct IEFDE_W<'a> {
    w: &'a mut W,
}
impl<'a> IEFDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CMPLD1DE`"]
pub type CMPLD1DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPLD1DE`"]
pub struct CMPLD1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPLD1DE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CMPLD2DE`"]
pub type CMPLD2DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPLD2DE`"]
pub struct CMPLD2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPLD2DE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub fn iefde(&self) -> IEFDE_R {
        IEFDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub fn cmpld1de(&self) -> CMPLD1DE_R {
        CMPLD1DE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub fn cmpld2de(&self) -> CMPLD2DE_R {
        CMPLD2DE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub fn iefde(&mut self) -> IEFDE_W {
        IEFDE_W { w: self }
    }
    #[doc = "Bit 1 - Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub fn cmpld1de(&mut self) -> CMPLD1DE_W {
        CMPLD1DE_W { w: self }
    }
    #[doc = "Bit 2 - Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub fn cmpld2de(&mut self) -> CMPLD2DE_W {
        CMPLD2DE_W { w: self }
    }
}
