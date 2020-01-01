#[doc = "Reader of register MATCH"]
pub type R = crate::R<u32, super::MATCH>;
#[doc = "Writer for register MATCH"]
pub type W = crate::W<u32, super::MATCH>;
#[doc = "Register MATCH `reset()`'s with value 0"]
impl crate::ResetValue for super::MATCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MA1`"]
pub type MA1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MA1`"]
pub struct MA1_W<'a> {
    w: &'a mut W,
}
impl<'a> MA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `MA2`"]
pub type MA2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MA2`"]
pub struct MA2_W<'a> {
    w: &'a mut W,
}
impl<'a> MA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Match Address 1"]
    #[inline(always)]
    pub fn ma1(&self) -> MA1_R {
        MA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Match Address 2"]
    #[inline(always)]
    pub fn ma2(&self) -> MA2_R {
        MA2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Match Address 1"]
    #[inline(always)]
    pub fn ma1(&mut self) -> MA1_W {
        MA1_W { w: self }
    }
    #[doc = "Bits 16:25 - Match Address 2"]
    #[inline(always)]
    pub fn ma2(&mut self) -> MA2_W {
        MA2_W { w: self }
    }
}
