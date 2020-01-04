#[doc = "Reader of register CHCFG[%s]"]
pub type R = crate::R<u32, super::CHCFG>;
#[doc = "Writer for register CHCFG[%s]"]
pub type W = crate::W<u32, super::CHCFG>;
#[doc = "Register CHCFG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CHCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOURCE`"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "DMA Channel Always Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A_ON_A {
    #[doc = "0: DMA Channel Always ON function is disabled"]
    A_ON_0 = 0,
    #[doc = "1: DMA Channel Always ON function is enabled"]
    A_ON_1 = 1,
}
impl From<A_ON_A> for bool {
    #[inline(always)]
    fn from(variant: A_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `A_ON`"]
pub type A_ON_R = crate::R<bool, A_ON_A>;
impl A_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> A_ON_A {
        match self.bits {
            false => A_ON_A::A_ON_0,
            true => A_ON_A::A_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `A_ON_0`"]
    #[inline(always)]
    pub fn is_a_on_0(&self) -> bool {
        *self == A_ON_A::A_ON_0
    }
    #[doc = "Checks if the value of the field is `A_ON_1`"]
    #[inline(always)]
    pub fn is_a_on_1(&self) -> bool {
        *self == A_ON_A::A_ON_1
    }
}
#[doc = "Write proxy for field `A_ON`"]
pub struct A_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> A_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: A_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA Channel Always ON function is disabled"]
    #[inline(always)]
    pub fn a_on_0(self) -> &'a mut W {
        self.variant(A_ON_A::A_ON_0)
    }
    #[doc = "DMA Channel Always ON function is enabled"]
    #[inline(always)]
    pub fn a_on_1(self) -> &'a mut W {
        self.variant(A_ON_A::A_ON_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "DMA Channel Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG_A {
    #[doc = "0: Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    TRIG_0 = 0,
    #[doc = "1: Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
    TRIG_1 = 1,
}
impl From<TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIG`"]
pub type TRIG_R = crate::R<bool, TRIG_A>;
impl TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_A {
        match self.bits {
            false => TRIG_A::TRIG_0,
            true => TRIG_A::TRIG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_0`"]
    #[inline(always)]
    pub fn is_trig_0(&self) -> bool {
        *self == TRIG_A::TRIG_0
    }
    #[doc = "Checks if the value of the field is `TRIG_1`"]
    #[inline(always)]
    pub fn is_trig_1(&self) -> bool {
        *self == TRIG_A::TRIG_1
    }
}
#[doc = "Write proxy for field `TRIG`"]
pub struct TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline(always)]
    pub fn trig_0(self) -> &'a mut W {
        self.variant(TRIG_A::TRIG_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
    #[inline(always)]
    pub fn trig_1(self) -> &'a mut W {
        self.variant(TRIG_A::TRIG_1)
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
#[doc = "DMA Mux Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBL_A {
    #[doc = "0: DMA Mux channel is disabled"]
    ENBL_0 = 0,
    #[doc = "1: DMA Mux channel is enabled"]
    ENBL_1 = 1,
}
impl From<ENBL_A> for bool {
    #[inline(always)]
    fn from(variant: ENBL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENBL`"]
pub type ENBL_R = crate::R<bool, ENBL_A>;
impl ENBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBL_A {
        match self.bits {
            false => ENBL_A::ENBL_0,
            true => ENBL_A::ENBL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENBL_0`"]
    #[inline(always)]
    pub fn is_enbl_0(&self) -> bool {
        *self == ENBL_A::ENBL_0
    }
    #[doc = "Checks if the value of the field is `ENBL_1`"]
    #[inline(always)]
    pub fn is_enbl_1(&self) -> bool {
        *self == ENBL_A::ENBL_1
    }
}
#[doc = "Write proxy for field `ENBL`"]
pub struct ENBL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENBL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA Mux channel is disabled"]
    #[inline(always)]
    pub fn enbl_0(self) -> &'a mut W {
        self.variant(ENBL_A::ENBL_0)
    }
    #[doc = "DMA Mux channel is enabled"]
    #[inline(always)]
    pub fn enbl_1(self) -> &'a mut W {
        self.variant(ENBL_A::ENBL_1)
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
    #[doc = "Bits 0:6 - DMA Channel Source (Slot Number)"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 29 - DMA Channel Always Enable"]
    #[inline(always)]
    pub fn a_on(&self) -> A_ON_R {
        A_ON_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Mux Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> ENBL_R {
        ENBL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA Channel Source (Slot Number)"]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W {
        SOURCE_W { w: self }
    }
    #[doc = "Bit 29 - DMA Channel Always Enable"]
    #[inline(always)]
    pub fn a_on(&mut self) -> A_ON_W {
        A_ON_W { w: self }
    }
    #[doc = "Bit 30 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W {
        TRIG_W { w: self }
    }
    #[doc = "Bit 31 - DMA Mux Channel Enable"]
    #[inline(always)]
    pub fn enbl(&mut self) -> ENBL_W {
        ENBL_W { w: self }
    }
}
