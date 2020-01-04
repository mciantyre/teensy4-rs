#[doc = "Reader of register PS_OFFSET"]
pub type R = crate::R<u32, super::PS_OFFSET>;
#[doc = "Writer for register PS_OFFSET"]
pub type W = crate::W<u32, super::PS_OFFSET>;
#[doc = "Register PS_OFFSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PS_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOFFSET`"]
pub type XOFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XOFFSET`"]
pub struct XOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
#[doc = "Reader of field `YOFFSET`"]
pub type YOFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `YOFFSET`"]
pub struct YOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> YOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - This is a 12 bit fractional representation (0"]
    #[inline(always)]
    pub fn xoffset(&self) -> XOFFSET_R {
        XOFFSET_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - This is a 12 bit fractional representation (0"]
    #[inline(always)]
    pub fn yoffset(&self) -> YOFFSET_R {
        YOFFSET_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - This is a 12 bit fractional representation (0"]
    #[inline(always)]
    pub fn xoffset(&mut self) -> XOFFSET_W {
        XOFFSET_W { w: self }
    }
    #[doc = "Bits 16:27 - This is a 12 bit fractional representation (0"]
    #[inline(always)]
    pub fn yoffset(&mut self) -> YOFFSET_W {
        YOFFSET_W { w: self }
    }
}
