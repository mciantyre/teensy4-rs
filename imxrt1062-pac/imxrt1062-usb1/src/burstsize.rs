#[doc = "Reader of register BURSTSIZE"]
pub type R = crate::R<u32, super::BURSTSIZE>;
#[doc = "Writer for register BURSTSIZE"]
pub type W = crate::W<u32, super::BURSTSIZE>;
#[doc = "Register BURSTSIZE `reset()`'s with value 0x0808"]
impl crate::ResetValue for super::BURSTSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0808
    }
}
#[doc = "Reader of field `RXPBURST`"]
pub type RXPBURST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXPBURST`"]
pub struct RXPBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TXPBURST`"]
pub type TXPBURST_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXPBURST`"]
pub struct TXPBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 8)) | (((value as u32) & 0x01ff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Programmable RX Burst Size"]
    #[inline(always)]
    pub fn rxpburst(&self) -> RXPBURST_R {
        RXPBURST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - Programmable TX Burst Size"]
    #[inline(always)]
    pub fn txpburst(&self) -> TXPBURST_R {
        TXPBURST_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Programmable RX Burst Size"]
    #[inline(always)]
    pub fn rxpburst(&mut self) -> RXPBURST_W {
        RXPBURST_W { w: self }
    }
    #[doc = "Bits 8:16 - Programmable TX Burst Size"]
    #[inline(always)]
    pub fn txpburst(&mut self) -> TXPBURST_W {
        TXPBURST_W { w: self }
    }
}
