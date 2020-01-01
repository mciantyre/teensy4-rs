#[doc = "Reader of register RXIC"]
pub type R = crate::R<u32, super::RXIC>;
#[doc = "Writer for register RXIC"]
pub type W = crate::W<u32, super::RXIC>;
#[doc = "Register RXIC `reset()`'s with value 0"]
impl crate::ResetValue for super::RXIC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICTT`"]
pub type ICTT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ICTT`"]
pub struct ICTT_W<'a> {
    w: &'a mut W,
}
impl<'a> ICTT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ICFT`"]
pub type ICFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICFT`"]
pub struct ICFT_W<'a> {
    w: &'a mut W,
}
impl<'a> ICFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Interrupt Coalescing Timer Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICCS_A {
    #[doc = "0: Use MII/GMII TX clocks."]
    ICCS_0 = 0,
    #[doc = "1: Use ENET system clock."]
    ICCS_1 = 1,
}
impl From<ICCS_A> for bool {
    #[inline(always)]
    fn from(variant: ICCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICCS`"]
pub type ICCS_R = crate::R<bool, ICCS_A>;
impl ICCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICCS_A {
        match self.bits {
            false => ICCS_A::ICCS_0,
            true => ICCS_A::ICCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ICCS_0`"]
    #[inline(always)]
    pub fn is_iccs_0(&self) -> bool {
        *self == ICCS_A::ICCS_0
    }
    #[doc = "Checks if the value of the field is `ICCS_1`"]
    #[inline(always)]
    pub fn is_iccs_1(&self) -> bool {
        *self == ICCS_A::ICCS_1
    }
}
#[doc = "Write proxy for field `ICCS`"]
pub struct ICCS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use MII/GMII TX clocks."]
    #[inline(always)]
    pub fn iccs_0(self) -> &'a mut W {
        self.variant(ICCS_A::ICCS_0)
    }
    #[doc = "Use ENET system clock."]
    #[inline(always)]
    pub fn iccs_1(self) -> &'a mut W {
        self.variant(ICCS_A::ICCS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Interrupt Coalescing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEN_A {
    #[doc = "0: Disable Interrupt coalescing."]
    ICEN_0 = 0,
    #[doc = "1: Enable Interrupt coalescing."]
    ICEN_1 = 1,
}
impl From<ICEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICEN`"]
pub type ICEN_R = crate::R<bool, ICEN_A>;
impl ICEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEN_A {
        match self.bits {
            false => ICEN_A::ICEN_0,
            true => ICEN_A::ICEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ICEN_0`"]
    #[inline(always)]
    pub fn is_icen_0(&self) -> bool {
        *self == ICEN_A::ICEN_0
    }
    #[doc = "Checks if the value of the field is `ICEN_1`"]
    #[inline(always)]
    pub fn is_icen_1(&self) -> bool {
        *self == ICEN_A::ICEN_1
    }
}
#[doc = "Write proxy for field `ICEN`"]
pub struct ICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Interrupt coalescing."]
    #[inline(always)]
    pub fn icen_0(self) -> &'a mut W {
        self.variant(ICEN_A::ICEN_0)
    }
    #[doc = "Enable Interrupt coalescing."]
    #[inline(always)]
    pub fn icen_1(self) -> &'a mut W {
        self.variant(ICEN_A::ICEN_1)
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
    #[doc = "Bits 0:15 - Interrupt coalescing timer threshold"]
    #[inline(always)]
    pub fn ictt(&self) -> ICTT_R {
        ICTT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 20:27 - Interrupt coalescing frame count threshold"]
    #[inline(always)]
    pub fn icft(&self) -> ICFT_R {
        ICFT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Interrupt Coalescing Timer Clock Source Select"]
    #[inline(always)]
    pub fn iccs(&self) -> ICCS_R {
        ICCS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt coalescing timer threshold"]
    #[inline(always)]
    pub fn ictt(&mut self) -> ICTT_W {
        ICTT_W { w: self }
    }
    #[doc = "Bits 20:27 - Interrupt coalescing frame count threshold"]
    #[inline(always)]
    pub fn icft(&mut self) -> ICFT_W {
        ICFT_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt Coalescing Timer Clock Source Select"]
    #[inline(always)]
    pub fn iccs(&mut self) -> ICCS_W {
        ICCS_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable"]
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W {
        ICEN_W { w: self }
    }
}
