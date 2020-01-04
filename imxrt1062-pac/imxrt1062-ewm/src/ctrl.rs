#[doc = "Reader of register CTRL"]
pub type R = crate::R<u8, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u8, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EWMEN`"]
pub type EWMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWMEN`"]
pub struct EWMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EWMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ASSIN`"]
pub type ASSIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASSIN`"]
pub struct ASSIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `INEN`"]
pub type INEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEN`"]
pub struct INEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN`"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EWM enable."]
    #[inline(always)]
    pub fn ewmen(&self) -> EWMEN_R {
        EWMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EWM_in's Assertion State Select."]
    #[inline(always)]
    pub fn assin(&self) -> ASSIN_R {
        ASSIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input Enable."]
    #[inline(always)]
    pub fn inen(&self) -> INEN_R {
        INEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EWM enable."]
    #[inline(always)]
    pub fn ewmen(&mut self) -> EWMEN_W {
        EWMEN_W { w: self }
    }
    #[doc = "Bit 1 - EWM_in's Assertion State Select."]
    #[inline(always)]
    pub fn assin(&mut self) -> ASSIN_W {
        ASSIN_W { w: self }
    }
    #[doc = "Bit 2 - Input Enable."]
    #[inline(always)]
    pub fn inen(&mut self) -> INEN_W {
        INEN_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Enable."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
}
