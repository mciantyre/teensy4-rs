#[doc = "Reader of register ERFCR"]
pub type R = crate::R<u32, super::ERFCR>;
#[doc = "Writer for register ERFCR"]
pub type W = crate::W<u32, super::ERFCR>;
#[doc = "Register ERFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERFWM`"]
pub type ERFWM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERFWM`"]
pub struct ERFWM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `NFE`"]
pub type NFE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFE`"]
pub struct NFE_W<'a> {
    w: &'a mut W,
}
impl<'a> NFE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NEXIF`"]
pub type NEXIF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NEXIF`"]
pub struct NEXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXIF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMALW`"]
pub type DMALW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMALW`"]
pub struct DMALW_W<'a> {
    w: &'a mut W,
}
impl<'a> DMALW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Enhanced Rx FIFO enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFEN_A {
    #[doc = "0: Enhanced Rx FIFO is disabled"]
    ERFEN_0 = 0,
    #[doc = "1: Enhanced Rx FIFO is enabled"]
    ERFEN_1 = 1,
}
impl From<ERFEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFEN`"]
pub type ERFEN_R = crate::R<bool, ERFEN_A>;
impl ERFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFEN_A {
        match self.bits {
            false => ERFEN_A::ERFEN_0,
            true => ERFEN_A::ERFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFEN_0`"]
    #[inline(always)]
    pub fn is_erfen_0(&self) -> bool {
        *self == ERFEN_A::ERFEN_0
    }
    #[doc = "Checks if the value of the field is `ERFEN_1`"]
    #[inline(always)]
    pub fn is_erfen_1(&self) -> bool {
        *self == ERFEN_A::ERFEN_1
    }
}
#[doc = "Write proxy for field `ERFEN`"]
pub struct ERFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enhanced Rx FIFO is disabled"]
    #[inline(always)]
    pub fn erfen_0(self) -> &'a mut W {
        self.variant(ERFEN_A::ERFEN_0)
    }
    #[doc = "Enhanced Rx FIFO is enabled"]
    #[inline(always)]
    pub fn erfen_1(self) -> &'a mut W {
        self.variant(ERFEN_A::ERFEN_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Enhanced Rx FIFO Watermark"]
    #[inline(always)]
    pub fn erfwm(&self) -> ERFWM_R {
        ERFWM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Number of Enhanced Rx FIFO Filter Elements"]
    #[inline(always)]
    pub fn nfe(&self) -> NFE_R {
        NFE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Number of Extended ID Filter Elements"]
    #[inline(always)]
    pub fn nexif(&self) -> NEXIF_R {
        NEXIF_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 26:30 - DMA Last Word"]
    #[inline(always)]
    pub fn dmalw(&self) -> DMALW_R {
        DMALW_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enhanced Rx FIFO enable"]
    #[inline(always)]
    pub fn erfen(&self) -> ERFEN_R {
        ERFEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Enhanced Rx FIFO Watermark"]
    #[inline(always)]
    pub fn erfwm(&mut self) -> ERFWM_W {
        ERFWM_W { w: self }
    }
    #[doc = "Bits 8:13 - Number of Enhanced Rx FIFO Filter Elements"]
    #[inline(always)]
    pub fn nfe(&mut self) -> NFE_W {
        NFE_W { w: self }
    }
    #[doc = "Bits 16:22 - Number of Extended ID Filter Elements"]
    #[inline(always)]
    pub fn nexif(&mut self) -> NEXIF_W {
        NEXIF_W { w: self }
    }
    #[doc = "Bits 26:30 - DMA Last Word"]
    #[inline(always)]
    pub fn dmalw(&mut self) -> DMALW_W {
        DMALW_W { w: self }
    }
    #[doc = "Bit 31 - Enhanced Rx FIFO enable"]
    #[inline(always)]
    pub fn erfen(&mut self) -> ERFEN_W {
        ERFEN_W { w: self }
    }
}
