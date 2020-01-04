#[doc = "Reader of register PS_SCALE"]
pub type R = crate::R<u32, super::PS_SCALE>;
#[doc = "Writer for register PS_SCALE"]
pub type W = crate::W<u32, super::PS_SCALE>;
#[doc = "Register PS_SCALE `reset()`'s with value 0x1000_1000"]
impl crate::ResetValue for super::PS_SCALE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_1000
    }
}
#[doc = "Reader of field `XSCALE`"]
pub type XSCALE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XSCALE`"]
pub struct XSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> XSCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<bool, bool>;
#[doc = "Reader of field `YSCALE`"]
pub type YSCALE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `YSCALE`"]
pub struct YSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> YSCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:14 - This is a two bit integer and 12 bit fractional representation (##"]
    #[inline(always)]
    pub fn xscale(&self) -> XSCALE_R {
        XSCALE_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - This is a two bit integer and 12 bit fractional representation (##"]
    #[inline(always)]
    pub fn yscale(&self) -> YSCALE_R {
        YSCALE_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - This is a two bit integer and 12 bit fractional representation (##"]
    #[inline(always)]
    pub fn xscale(&mut self) -> XSCALE_W {
        XSCALE_W { w: self }
    }
    #[doc = "Bits 16:30 - This is a two bit integer and 12 bit fractional representation (##"]
    #[inline(always)]
    pub fn yscale(&mut self) -> YSCALE_W {
        YSCALE_W { w: self }
    }
}
