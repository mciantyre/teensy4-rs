#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIE_A {
    #[doc = "0: Disabled"]
    TDIE_0 = 0,
    #[doc = "1: Enabled"]
    TDIE_1 = 1,
}
impl From<TDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDIE`"]
pub type TDIE_R = crate::R<bool, TDIE_A>;
impl TDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIE_A {
        match self.bits {
            false => TDIE_A::TDIE_0,
            true => TDIE_A::TDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDIE_0`"]
    #[inline(always)]
    pub fn is_tdie_0(&self) -> bool {
        *self == TDIE_A::TDIE_0
    }
    #[doc = "Checks if the value of the field is `TDIE_1`"]
    #[inline(always)]
    pub fn is_tdie_1(&self) -> bool {
        *self == TDIE_A::TDIE_1
    }
}
#[doc = "Write proxy for field `TDIE`"]
pub struct TDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn tdie_0(self) -> &'a mut W {
        self.variant(TDIE_A::TDIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tdie_1(self) -> &'a mut W {
        self.variant(TDIE_A::TDIE_1)
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
#[doc = "Receive Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIE_A {
    #[doc = "0: Disabled"]
    RDIE_0 = 0,
    #[doc = "1: Enabled"]
    RDIE_1 = 1,
}
impl From<RDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDIE`"]
pub type RDIE_R = crate::R<bool, RDIE_A>;
impl RDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIE_A {
        match self.bits {
            false => RDIE_A::RDIE_0,
            true => RDIE_A::RDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDIE_0`"]
    #[inline(always)]
    pub fn is_rdie_0(&self) -> bool {
        *self == RDIE_A::RDIE_0
    }
    #[doc = "Checks if the value of the field is `RDIE_1`"]
    #[inline(always)]
    pub fn is_rdie_1(&self) -> bool {
        *self == RDIE_A::RDIE_1
    }
}
#[doc = "Write proxy for field `RDIE`"]
pub struct RDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn rdie_0(self) -> &'a mut W {
        self.variant(RDIE_A::RDIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rdie_1(self) -> &'a mut W {
        self.variant(RDIE_A::RDIE_1)
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
#[doc = "Word Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCIE_A {
    #[doc = "0: Disabled"]
    WCIE_0 = 0,
    #[doc = "1: Enabled"]
    WCIE_1 = 1,
}
impl From<WCIE_A> for bool {
    #[inline(always)]
    fn from(variant: WCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCIE`"]
pub type WCIE_R = crate::R<bool, WCIE_A>;
impl WCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCIE_A {
        match self.bits {
            false => WCIE_A::WCIE_0,
            true => WCIE_A::WCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WCIE_0`"]
    #[inline(always)]
    pub fn is_wcie_0(&self) -> bool {
        *self == WCIE_A::WCIE_0
    }
    #[doc = "Checks if the value of the field is `WCIE_1`"]
    #[inline(always)]
    pub fn is_wcie_1(&self) -> bool {
        *self == WCIE_A::WCIE_1
    }
}
#[doc = "Write proxy for field `WCIE`"]
pub struct WCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn wcie_0(self) -> &'a mut W {
        self.variant(WCIE_A::WCIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn wcie_1(self) -> &'a mut W {
        self.variant(WCIE_A::WCIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Frame Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCIE_A {
    #[doc = "0: Disabled"]
    FCIE_0 = 0,
    #[doc = "1: Enabled"]
    FCIE_1 = 1,
}
impl From<FCIE_A> for bool {
    #[inline(always)]
    fn from(variant: FCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCIE`"]
pub type FCIE_R = crate::R<bool, FCIE_A>;
impl FCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCIE_A {
        match self.bits {
            false => FCIE_A::FCIE_0,
            true => FCIE_A::FCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCIE_0`"]
    #[inline(always)]
    pub fn is_fcie_0(&self) -> bool {
        *self == FCIE_A::FCIE_0
    }
    #[doc = "Checks if the value of the field is `FCIE_1`"]
    #[inline(always)]
    pub fn is_fcie_1(&self) -> bool {
        *self == FCIE_A::FCIE_1
    }
}
#[doc = "Write proxy for field `FCIE`"]
pub struct FCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn fcie_0(self) -> &'a mut W {
        self.variant(FCIE_A::FCIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn fcie_1(self) -> &'a mut W {
        self.variant(FCIE_A::FCIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Transfer Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: Disabled"]
    TCIE_0 = 0,
    #[doc = "1: Enabled"]
    TCIE_1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIE`"]
pub type TCIE_R = crate::R<bool, TCIE_A>;
impl TCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::TCIE_0,
            true => TCIE_A::TCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCIE_0`"]
    #[inline(always)]
    pub fn is_tcie_0(&self) -> bool {
        *self == TCIE_A::TCIE_0
    }
    #[doc = "Checks if the value of the field is `TCIE_1`"]
    #[inline(always)]
    pub fn is_tcie_1(&self) -> bool {
        *self == TCIE_A::TCIE_1
    }
}
#[doc = "Write proxy for field `TCIE`"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn tcie_0(self) -> &'a mut W {
        self.variant(TCIE_A::TCIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tcie_1(self) -> &'a mut W {
        self.variant(TCIE_A::TCIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Transmit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIE_A {
    #[doc = "0: Disabled"]
    TEIE_0 = 0,
    #[doc = "1: Enabled"]
    TEIE_1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIE`"]
pub type TEIE_R = crate::R<bool, TEIE_A>;
impl TEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::TEIE_0,
            true => TEIE_A::TEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEIE_0`"]
    #[inline(always)]
    pub fn is_teie_0(&self) -> bool {
        *self == TEIE_A::TEIE_0
    }
    #[doc = "Checks if the value of the field is `TEIE_1`"]
    #[inline(always)]
    pub fn is_teie_1(&self) -> bool {
        *self == TEIE_A::TEIE_1
    }
}
#[doc = "Write proxy for field `TEIE`"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn teie_0(self) -> &'a mut W {
        self.variant(TEIE_A::TEIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn teie_1(self) -> &'a mut W {
        self.variant(TEIE_A::TEIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Receive Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIE_A {
    #[doc = "0: Disabled"]
    REIE_0 = 0,
    #[doc = "1: Enabled"]
    REIE_1 = 1,
}
impl From<REIE_A> for bool {
    #[inline(always)]
    fn from(variant: REIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REIE`"]
pub type REIE_R = crate::R<bool, REIE_A>;
impl REIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REIE_A {
        match self.bits {
            false => REIE_A::REIE_0,
            true => REIE_A::REIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `REIE_0`"]
    #[inline(always)]
    pub fn is_reie_0(&self) -> bool {
        *self == REIE_A::REIE_0
    }
    #[doc = "Checks if the value of the field is `REIE_1`"]
    #[inline(always)]
    pub fn is_reie_1(&self) -> bool {
        *self == REIE_A::REIE_1
    }
}
#[doc = "Write proxy for field `REIE`"]
pub struct REIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn reie_0(self) -> &'a mut W {
        self.variant(REIE_A::REIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn reie_1(self) -> &'a mut W {
        self.variant(REIE_A::REIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Data Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIE_A {
    #[doc = "0: Disabled"]
    DMIE_0 = 0,
    #[doc = "1: Enabled"]
    DMIE_1 = 1,
}
impl From<DMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIE`"]
pub type DMIE_R = crate::R<bool, DMIE_A>;
impl DMIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIE_A {
        match self.bits {
            false => DMIE_A::DMIE_0,
            true => DMIE_A::DMIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIE_0`"]
    #[inline(always)]
    pub fn is_dmie_0(&self) -> bool {
        *self == DMIE_A::DMIE_0
    }
    #[doc = "Checks if the value of the field is `DMIE_1`"]
    #[inline(always)]
    pub fn is_dmie_1(&self) -> bool {
        *self == DMIE_A::DMIE_1
    }
}
#[doc = "Write proxy for field `DMIE`"]
pub struct DMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dmie_0(self) -> &'a mut W {
        self.variant(DMIE_A::DMIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dmie_1(self) -> &'a mut W {
        self.variant(DMIE_A::DMIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Word Complete Interrupt Enable"]
    #[inline(always)]
    pub fn wcie(&self) -> WCIE_R {
        WCIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame Complete Interrupt Enable"]
    #[inline(always)]
    pub fn fcie(&self) -> FCIE_R {
        FCIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&self) -> DMIE_R {
        DMIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&mut self) -> TDIE_W {
        TDIE_W { w: self }
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&mut self) -> RDIE_W {
        RDIE_W { w: self }
    }
    #[doc = "Bit 8 - Word Complete Interrupt Enable"]
    #[inline(always)]
    pub fn wcie(&mut self) -> WCIE_W {
        WCIE_W { w: self }
    }
    #[doc = "Bit 9 - Frame Complete Interrupt Enable"]
    #[inline(always)]
    pub fn fcie(&mut self) -> FCIE_W {
        FCIE_W { w: self }
    }
    #[doc = "Bit 10 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Bit 12 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&mut self) -> REIE_W {
        REIE_W { w: self }
    }
    #[doc = "Bit 13 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&mut self) -> DMIE_W {
        DMIE_W { w: self }
    }
}
