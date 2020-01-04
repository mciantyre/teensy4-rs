#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IPCMDDONEEN`"]
pub type IPCMDDONEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDDONEEN`"]
pub struct IPCMDDONEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDDONEEN_W<'a> {
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
#[doc = "Reader of field `IPCMDERREN`"]
pub type IPCMDERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDERREN`"]
pub struct IPCMDERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDERREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AXICMDERREN`"]
pub type AXICMDERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXICMDERREN`"]
pub struct AXICMDERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXICMDERREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AXIBUSERREN`"]
pub type AXIBUSERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXIBUSERREN`"]
pub struct AXIBUSERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIBUSERREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "This bit enable/disable the NDPAGEEND interrupt generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDPAGEENDEN_A {
    #[doc = "0: Disable"]
    NDPAGEENDEN_0 = 0,
    #[doc = "1: Enable"]
    NDPAGEENDEN_1 = 1,
}
impl From<NDPAGEENDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NDPAGEENDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NDPAGEENDEN`"]
pub type NDPAGEENDEN_R = crate::R<bool, NDPAGEENDEN_A>;
impl NDPAGEENDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDPAGEENDEN_A {
        match self.bits {
            false => NDPAGEENDEN_A::NDPAGEENDEN_0,
            true => NDPAGEENDEN_A::NDPAGEENDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDPAGEENDEN_0`"]
    #[inline(always)]
    pub fn is_ndpageenden_0(&self) -> bool {
        *self == NDPAGEENDEN_A::NDPAGEENDEN_0
    }
    #[doc = "Checks if the value of the field is `NDPAGEENDEN_1`"]
    #[inline(always)]
    pub fn is_ndpageenden_1(&self) -> bool {
        *self == NDPAGEENDEN_A::NDPAGEENDEN_1
    }
}
#[doc = "Write proxy for field `NDPAGEENDEN`"]
pub struct NDPAGEENDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NDPAGEENDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDPAGEENDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn ndpageenden_0(self) -> &'a mut W {
        self.variant(NDPAGEENDEN_A::NDPAGEENDEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn ndpageenden_1(self) -> &'a mut W {
        self.variant(NDPAGEENDEN_A::NDPAGEENDEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "This bit enable/disable the NDNOPEND interrupt generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDNOPENDEN_A {
    #[doc = "0: Disable"]
    NDNOPENDEN_0 = 0,
    #[doc = "1: Enable"]
    NDNOPENDEN_1 = 1,
}
impl From<NDNOPENDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NDNOPENDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NDNOPENDEN`"]
pub type NDNOPENDEN_R = crate::R<bool, NDNOPENDEN_A>;
impl NDNOPENDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDNOPENDEN_A {
        match self.bits {
            false => NDNOPENDEN_A::NDNOPENDEN_0,
            true => NDNOPENDEN_A::NDNOPENDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDNOPENDEN_0`"]
    #[inline(always)]
    pub fn is_ndnopenden_0(&self) -> bool {
        *self == NDNOPENDEN_A::NDNOPENDEN_0
    }
    #[doc = "Checks if the value of the field is `NDNOPENDEN_1`"]
    #[inline(always)]
    pub fn is_ndnopenden_1(&self) -> bool {
        *self == NDNOPENDEN_A::NDNOPENDEN_1
    }
}
#[doc = "Write proxy for field `NDNOPENDEN`"]
pub struct NDNOPENDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NDNOPENDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDNOPENDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn ndnopenden_0(self) -> &'a mut W {
        self.variant(NDNOPENDEN_A::NDNOPENDEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn ndnopenden_1(self) -> &'a mut W {
        self.variant(NDNOPENDEN_A::NDNOPENDEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IP command done interrupt enable"]
    #[inline(always)]
    pub fn ipcmddoneen(&self) -> IPCMDDONEEN_R {
        IPCMDDONEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IP command error interrupt enable"]
    #[inline(always)]
    pub fn ipcmderren(&self) -> IPCMDERREN_R {
        IPCMDERREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AXI command error interrupt enable"]
    #[inline(always)]
    pub fn axicmderren(&self) -> AXICMDERREN_R {
        AXICMDERREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXI bus error interrupt enable"]
    #[inline(always)]
    pub fn axibuserren(&self) -> AXIBUSERREN_R {
        AXIBUSERREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit enable/disable the NDPAGEEND interrupt generation."]
    #[inline(always)]
    pub fn ndpageenden(&self) -> NDPAGEENDEN_R {
        NDPAGEENDEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit enable/disable the NDNOPEND interrupt generation."]
    #[inline(always)]
    pub fn ndnopenden(&self) -> NDNOPENDEN_R {
        NDNOPENDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP command done interrupt enable"]
    #[inline(always)]
    pub fn ipcmddoneen(&mut self) -> IPCMDDONEEN_W {
        IPCMDDONEEN_W { w: self }
    }
    #[doc = "Bit 1 - IP command error interrupt enable"]
    #[inline(always)]
    pub fn ipcmderren(&mut self) -> IPCMDERREN_W {
        IPCMDERREN_W { w: self }
    }
    #[doc = "Bit 2 - AXI command error interrupt enable"]
    #[inline(always)]
    pub fn axicmderren(&mut self) -> AXICMDERREN_W {
        AXICMDERREN_W { w: self }
    }
    #[doc = "Bit 3 - AXI bus error interrupt enable"]
    #[inline(always)]
    pub fn axibuserren(&mut self) -> AXIBUSERREN_W {
        AXIBUSERREN_W { w: self }
    }
    #[doc = "Bit 4 - This bit enable/disable the NDPAGEEND interrupt generation."]
    #[inline(always)]
    pub fn ndpageenden(&mut self) -> NDPAGEENDEN_W {
        NDPAGEENDEN_W { w: self }
    }
    #[doc = "Bit 5 - This bit enable/disable the NDNOPEND interrupt generation."]
    #[inline(always)]
    pub fn ndnopenden(&mut self) -> NDNOPENDEN_W {
        NDNOPENDEN_W { w: self }
    }
}
