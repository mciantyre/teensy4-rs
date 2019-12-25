#[doc = "Reader of register MDER"]
pub type R = crate::R<u32, super::MDER>;
#[doc = "Writer for register MDER"]
pub type W = crate::W<u32, super::MDER>;
#[doc = "Register MDER `reset()`'s with value 0"]
impl crate::ResetValue for super::MDER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit Data DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDDE_A {
    #[doc = "0: DMA request is disabled"]
    TDDE_0 = 0,
    #[doc = "1: DMA request is enabled"]
    TDDE_1 = 1,
}
impl From<TDDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDDE`"]
pub type TDDE_R = crate::R<bool, TDDE_A>;
impl TDDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDDE_A {
        match self.bits {
            false => TDDE_A::TDDE_0,
            true => TDDE_A::TDDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDDE_0`"]
    #[inline(always)]
    pub fn is_tdde_0(&self) -> bool {
        *self == TDDE_A::TDDE_0
    }
    #[doc = "Checks if the value of the field is `TDDE_1`"]
    #[inline(always)]
    pub fn is_tdde_1(&self) -> bool {
        *self == TDDE_A::TDDE_1
    }
}
#[doc = "Write proxy for field `TDDE`"]
pub struct TDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn tdde_0(self) -> &'a mut W {
        self.variant(TDDE_A::TDDE_0)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn tdde_1(self) -> &'a mut W {
        self.variant(TDDE_A::TDDE_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Receive Data DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDDE_A {
    #[doc = "0: DMA request is disabled"]
    RDDE_0 = 0,
    #[doc = "1: DMA request is enabled"]
    RDDE_1 = 1,
}
impl From<RDDE_A> for bool {
    #[inline(always)]
    fn from(variant: RDDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDDE`"]
pub type RDDE_R = crate::R<bool, RDDE_A>;
impl RDDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDDE_A {
        match self.bits {
            false => RDDE_A::RDDE_0,
            true => RDDE_A::RDDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDDE_0`"]
    #[inline(always)]
    pub fn is_rdde_0(&self) -> bool {
        *self == RDDE_A::RDDE_0
    }
    #[doc = "Checks if the value of the field is `RDDE_1`"]
    #[inline(always)]
    pub fn is_rdde_1(&self) -> bool {
        *self == RDDE_A::RDDE_1
    }
}
#[doc = "Write proxy for field `RDDE`"]
pub struct RDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn rdde_0(self) -> &'a mut W {
        self.variant(RDDE_A::RDDE_0)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn rdde_1(self) -> &'a mut W {
        self.variant(RDDE_A::RDDE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data DMA Enable"]
    #[inline(always)]
    pub fn tdde(&self) -> TDDE_R {
        TDDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data DMA Enable"]
    #[inline(always)]
    pub fn rdde(&self) -> RDDE_R {
        RDDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data DMA Enable"]
    #[inline(always)]
    pub fn tdde(&mut self) -> TDDE_W {
        TDDE_W { w: self }
    }
    #[doc = "Bit 1 - Receive Data DMA Enable"]
    #[inline(always)]
    pub fn rdde(&mut self) -> RDDE_W {
        RDDE_W { w: self }
    }
}
