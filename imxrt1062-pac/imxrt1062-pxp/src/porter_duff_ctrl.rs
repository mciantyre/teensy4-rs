#[doc = "Reader of register PORTER_DUFF_CTRL"]
pub type R = crate::R<u32, super::PORTER_DUFF_CTRL>;
#[doc = "Writer for register PORTER_DUFF_CTRL"]
pub type W = crate::W<u32, super::PORTER_DUFF_CTRL>;
#[doc = "Register PORTER_DUFF_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PORTER_DUFF_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POTER_DUFF_ENABLE`"]
pub type POTER_DUFF_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POTER_DUFF_ENABLE`"]
pub struct POTER_DUFF_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> POTER_DUFF_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `S0_S1_FACTOR_MODE`"]
pub type S0_S1_FACTOR_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S0_S1_FACTOR_MODE`"]
pub struct S0_S1_FACTOR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_S1_FACTOR_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `S0_GLOBAL_ALPHA_MODE`"]
pub type S0_GLOBAL_ALPHA_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S0_GLOBAL_ALPHA_MODE`"]
pub struct S0_GLOBAL_ALPHA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_GLOBAL_ALPHA_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `S0_ALPHA_MODE`"]
pub type S0_ALPHA_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0_ALPHA_MODE`"]
pub struct S0_ALPHA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_ALPHA_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `S0_COLOR_MODE`"]
pub type S0_COLOR_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0_COLOR_MODE`"]
pub struct S0_COLOR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_COLOR_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `S1_S0_FACTOR_MODE`"]
pub type S1_S0_FACTOR_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S1_S0_FACTOR_MODE`"]
pub struct S1_S0_FACTOR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_S0_FACTOR_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `S1_GLOBAL_ALPHA_MODE`"]
pub type S1_GLOBAL_ALPHA_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S1_GLOBAL_ALPHA_MODE`"]
pub struct S1_GLOBAL_ALPHA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_GLOBAL_ALPHA_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `S1_ALPHA_MODE`"]
pub type S1_ALPHA_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1_ALPHA_MODE`"]
pub struct S1_ALPHA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_ALPHA_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `S1_COLOR_MODE`"]
pub type S1_COLOR_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S1_COLOR_MODE`"]
pub struct S1_COLOR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_COLOR_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `S0_GLOBAL_ALPHA`"]
pub type S0_GLOBAL_ALPHA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S0_GLOBAL_ALPHA`"]
pub struct S0_GLOBAL_ALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_GLOBAL_ALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `S1_GLOBAL_ALPHA`"]
pub type S1_GLOBAL_ALPHA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S1_GLOBAL_ALPHA`"]
pub struct S1_GLOBAL_ALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_GLOBAL_ALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - poter_duff enable"]
    #[inline(always)]
    pub fn poter_duff_enable(&self) -> POTER_DUFF_ENABLE_R {
        POTER_DUFF_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - s0 to s1 factor mode"]
    #[inline(always)]
    pub fn s0_s1_factor_mode(&self) -> S0_S1_FACTOR_MODE_R {
        S0_S1_FACTOR_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - s0 global alpha mode"]
    #[inline(always)]
    pub fn s0_global_alpha_mode(&self) -> S0_GLOBAL_ALPHA_MODE_R {
        S0_GLOBAL_ALPHA_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - s0 alpha mode"]
    #[inline(always)]
    pub fn s0_alpha_mode(&self) -> S0_ALPHA_MODE_R {
        S0_ALPHA_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - s0 color mode"]
    #[inline(always)]
    pub fn s0_color_mode(&self) -> S0_COLOR_MODE_R {
        S0_COLOR_MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - s1 to s0 factor mode"]
    #[inline(always)]
    pub fn s1_s0_factor_mode(&self) -> S1_S0_FACTOR_MODE_R {
        S1_S0_FACTOR_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - s1 global alpha mode"]
    #[inline(always)]
    pub fn s1_global_alpha_mode(&self) -> S1_GLOBAL_ALPHA_MODE_R {
        S1_GLOBAL_ALPHA_MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - s1 alpha mode"]
    #[inline(always)]
    pub fn s1_alpha_mode(&self) -> S1_ALPHA_MODE_R {
        S1_ALPHA_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - s1 color mode"]
    #[inline(always)]
    pub fn s1_color_mode(&self) -> S1_COLOR_MODE_R {
        S1_COLOR_MODE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - s0 global alpha"]
    #[inline(always)]
    pub fn s0_global_alpha(&self) -> S0_GLOBAL_ALPHA_R {
        S0_GLOBAL_ALPHA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - s1 global alpha"]
    #[inline(always)]
    pub fn s1_global_alpha(&self) -> S1_GLOBAL_ALPHA_R {
        S1_GLOBAL_ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - poter_duff enable"]
    #[inline(always)]
    pub fn poter_duff_enable(&mut self) -> POTER_DUFF_ENABLE_W {
        POTER_DUFF_ENABLE_W { w: self }
    }
    #[doc = "Bits 1:2 - s0 to s1 factor mode"]
    #[inline(always)]
    pub fn s0_s1_factor_mode(&mut self) -> S0_S1_FACTOR_MODE_W {
        S0_S1_FACTOR_MODE_W { w: self }
    }
    #[doc = "Bits 3:4 - s0 global alpha mode"]
    #[inline(always)]
    pub fn s0_global_alpha_mode(&mut self) -> S0_GLOBAL_ALPHA_MODE_W {
        S0_GLOBAL_ALPHA_MODE_W { w: self }
    }
    #[doc = "Bit 5 - s0 alpha mode"]
    #[inline(always)]
    pub fn s0_alpha_mode(&mut self) -> S0_ALPHA_MODE_W {
        S0_ALPHA_MODE_W { w: self }
    }
    #[doc = "Bit 6 - s0 color mode"]
    #[inline(always)]
    pub fn s0_color_mode(&mut self) -> S0_COLOR_MODE_W {
        S0_COLOR_MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - s1 to s0 factor mode"]
    #[inline(always)]
    pub fn s1_s0_factor_mode(&mut self) -> S1_S0_FACTOR_MODE_W {
        S1_S0_FACTOR_MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - s1 global alpha mode"]
    #[inline(always)]
    pub fn s1_global_alpha_mode(&mut self) -> S1_GLOBAL_ALPHA_MODE_W {
        S1_GLOBAL_ALPHA_MODE_W { w: self }
    }
    #[doc = "Bit 12 - s1 alpha mode"]
    #[inline(always)]
    pub fn s1_alpha_mode(&mut self) -> S1_ALPHA_MODE_W {
        S1_ALPHA_MODE_W { w: self }
    }
    #[doc = "Bit 13 - s1 color mode"]
    #[inline(always)]
    pub fn s1_color_mode(&mut self) -> S1_COLOR_MODE_W {
        S1_COLOR_MODE_W { w: self }
    }
    #[doc = "Bits 16:23 - s0 global alpha"]
    #[inline(always)]
    pub fn s0_global_alpha(&mut self) -> S0_GLOBAL_ALPHA_W {
        S0_GLOBAL_ALPHA_W { w: self }
    }
    #[doc = "Bits 24:31 - s1 global alpha"]
    #[inline(always)]
    pub fn s1_global_alpha(&mut self) -> S1_GLOBAL_ALPHA_W {
        S1_GLOBAL_ALPHA_W { w: self }
    }
}
