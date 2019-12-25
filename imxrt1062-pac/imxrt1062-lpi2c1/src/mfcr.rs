#[doc = "Reader of register MFCR"]
pub type R = crate::R<u32, super::MFCR>;
#[doc = "Writer for register MFCR"]
pub type W = crate::W<u32, super::MFCR>;
#[doc = "Register MFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXWATER`"]
pub type TXWATER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXWATER`"]
pub struct TXWATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXWATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RXWATER`"]
pub type RXWATER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXWATER`"]
pub struct RXWATER_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Transmit FIFO Watermark"]
    #[inline(always)]
    pub fn txwater(&self) -> TXWATER_R {
        TXWATER_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Receive FIFO Watermark"]
    #[inline(always)]
    pub fn rxwater(&self) -> RXWATER_R {
        RXWATER_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit FIFO Watermark"]
    #[inline(always)]
    pub fn txwater(&mut self) -> TXWATER_W {
        TXWATER_W { w: self }
    }
    #[doc = "Bits 16:17 - Receive FIFO Watermark"]
    #[inline(always)]
    pub fn rxwater(&mut self) -> RXWATER_W {
        RXWATER_W { w: self }
    }
}
