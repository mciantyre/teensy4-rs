#[doc = "Reader of register CHANNELCTRL"]
pub type R = crate::R<u32, super::CHANNELCTRL>;
#[doc = "Writer for register CHANNELCTRL"]
pub type W = crate::W<u32, super::CHANNELCTRL>;
#[doc = "Register CHANNELCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CHANNELCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Setting a bit in this field enables the DMA channel associated with it\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENABLE_CHANNEL_A {
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "4: CH2"]
    CH2 = 4,
    #[doc = "8: CH3"]
    CH3 = 8,
}
impl From<ENABLE_CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_CHANNEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENABLE_CHANNEL`"]
pub type ENABLE_CHANNEL_R = crate::R<u8, ENABLE_CHANNEL_A>;
impl ENABLE_CHANNEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENABLE_CHANNEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ENABLE_CHANNEL_A::CH0),
            2 => Val(ENABLE_CHANNEL_A::CH1),
            4 => Val(ENABLE_CHANNEL_A::CH2),
            8 => Val(ENABLE_CHANNEL_A::CH3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == ENABLE_CHANNEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == ENABLE_CHANNEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == ENABLE_CHANNEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == ENABLE_CHANNEL_A::CH3
    }
}
#[doc = "Write proxy for field `ENABLE_CHANNEL`"]
pub struct ENABLE_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_CHANNEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_CHANNEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CH0"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(ENABLE_CHANNEL_A::CH0)
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(ENABLE_CHANNEL_A::CH1)
    }
    #[doc = "CH2"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(ENABLE_CHANNEL_A::CH2)
    }
    #[doc = "CH3"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(ENABLE_CHANNEL_A::CH3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HIGH_PRIORITY_CHANNEL_A {
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "4: CH2"]
    CH2 = 4,
    #[doc = "8: CH3"]
    CH3 = 8,
}
impl From<HIGH_PRIORITY_CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIGH_PRIORITY_CHANNEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HIGH_PRIORITY_CHANNEL`"]
pub type HIGH_PRIORITY_CHANNEL_R = crate::R<u8, HIGH_PRIORITY_CHANNEL_A>;
impl HIGH_PRIORITY_CHANNEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HIGH_PRIORITY_CHANNEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(HIGH_PRIORITY_CHANNEL_A::CH0),
            2 => Val(HIGH_PRIORITY_CHANNEL_A::CH1),
            4 => Val(HIGH_PRIORITY_CHANNEL_A::CH2),
            8 => Val(HIGH_PRIORITY_CHANNEL_A::CH3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNEL_A::CH3
    }
}
#[doc = "Write proxy for field `HIGH_PRIORITY_CHANNEL`"]
pub struct HIGH_PRIORITY_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_PRIORITY_CHANNEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIGH_PRIORITY_CHANNEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CH0"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNEL_A::CH0)
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNEL_A::CH1)
    }
    #[doc = "CH2"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNEL_A::CH2)
    }
    #[doc = "CH3"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNEL_A::CH3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH0_IRQ_MERGED`"]
pub type CH0_IRQ_MERGED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_IRQ_MERGED`"]
pub struct CH0_IRQ_MERGED_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_IRQ_MERGED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub fn enable_channel(&self) -> ENABLE_CHANNEL_R {
        ENABLE_CHANNEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub fn high_priority_channel(&self) -> HIGH_PRIORITY_CHANNEL_R {
        HIGH_PRIORITY_CHANNEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub fn ch0_irq_merged(&self) -> CH0_IRQ_MERGED_R {
        CH0_IRQ_MERGED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub fn enable_channel(&mut self) -> ENABLE_CHANNEL_W {
        ENABLE_CHANNEL_W { w: self }
    }
    #[doc = "Bits 8:15 - Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub fn high_priority_channel(&mut self) -> HIGH_PRIORITY_CHANNEL_W {
        HIGH_PRIORITY_CHANNEL_W { w: self }
    }
    #[doc = "Bit 16 - Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub fn ch0_irq_merged(&mut self) -> CH0_IRQ_MERGED_W {
        CH0_IRQ_MERGED_W { w: self }
    }
}
