#[doc = "Reader of register ECR"]
pub type R = crate::R<u32, super::ECR>;
#[doc = "Writer for register ECR"]
pub type W = crate::W<u32, super::ECR>;
#[doc = "Register ECR `reset()`'s with value 0"]
impl crate::ResetValue for super::ECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXERRCNT`"]
pub type TXERRCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXERRCNT`"]
pub struct TXERRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RXERRCNT`"]
pub type RXERRCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXERRCNT`"]
pub struct RXERRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXERRCNT_FAST`"]
pub type TXERRCNT_FAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXERRCNT_FAST`"]
pub struct TXERRCNT_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRCNT_FAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXERRCNT_FAST`"]
pub type RXERRCNT_FAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXERRCNT_FAST`"]
pub struct RXERRCNT_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRCNT_FAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn txerrcnt(&self) -> TXERRCNT_R {
        TXERRCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rxerrcnt(&self) -> RXERRCNT_R {
        RXERRCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Counter for fast bits"]
    #[inline(always)]
    pub fn txerrcnt_fast(&self) -> TXERRCNT_FAST_R {
        TXERRCNT_FAST_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Error Counter for fast bits"]
    #[inline(always)]
    pub fn rxerrcnt_fast(&self) -> RXERRCNT_FAST_R {
        RXERRCNT_FAST_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn txerrcnt(&mut self) -> TXERRCNT_W {
        TXERRCNT_W { w: self }
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rxerrcnt(&mut self) -> RXERRCNT_W {
        RXERRCNT_W { w: self }
    }
    #[doc = "Bits 16:23 - Transmit Error Counter for fast bits"]
    #[inline(always)]
    pub fn txerrcnt_fast(&mut self) -> TXERRCNT_FAST_W {
        TXERRCNT_FAST_W { w: self }
    }
    #[doc = "Bits 24:31 - Receive Error Counter for fast bits"]
    #[inline(always)]
    pub fn rxerrcnt_fast(&mut self) -> RXERRCNT_FAST_W {
        RXERRCNT_FAST_W { w: self }
    }
}
