#[doc = "Reader of register STAT_CLR"]
pub type R = crate::R<u32, super::STAT_CLR>;
#[doc = "Writer for register STAT_CLR"]
pub type W = crate::W<u32, super::STAT_CLR>;
#[doc = "Register STAT_CLR `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::STAT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Reader of field `IRQ`"]
pub type IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQ`"]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum READY_CHANNELS_A {
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "4: CH2"]
    CH2 = 4,
    #[doc = "8: CH3"]
    CH3 = 8,
}
impl From<READY_CHANNELS_A> for u8 {
    #[inline(always)]
    fn from(variant: READY_CHANNELS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `READY_CHANNELS`"]
pub type READY_CHANNELS_R = crate::R<u8, READY_CHANNELS_A>;
impl READY_CHANNELS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, READY_CHANNELS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(READY_CHANNELS_A::CH0),
            2 => Val(READY_CHANNELS_A::CH1),
            4 => Val(READY_CHANNELS_A::CH2),
            8 => Val(READY_CHANNELS_A::CH3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == READY_CHANNELS_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == READY_CHANNELS_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == READY_CHANNELS_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == READY_CHANNELS_A::CH3
    }
}
#[doc = "Current (active) channel (encoded)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CUR_CHANNEL_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "3: CH2"]
    CH2 = 3,
    #[doc = "4: CH3"]
    CH3 = 4,
}
impl From<CUR_CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CUR_CHANNEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CUR_CHANNEL`"]
pub type CUR_CHANNEL_R = crate::R<u8, CUR_CHANNEL_A>;
impl CUR_CHANNEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CUR_CHANNEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CUR_CHANNEL_A::NONE),
            1 => Val(CUR_CHANNEL_A::CH0),
            2 => Val(CUR_CHANNEL_A::CH1),
            3 => Val(CUR_CHANNEL_A::CH2),
            4 => Val(CUR_CHANNEL_A::CH3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CUR_CHANNEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == CUR_CHANNEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == CUR_CHANNEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == CUR_CHANNEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == CUR_CHANNEL_A::CH3
    }
}
#[doc = "Reader of field `OTP_KEY_READY`"]
pub type OTP_KEY_READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub fn ready_channels(&self) -> READY_CHANNELS_R {
        READY_CHANNELS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Current (active) channel (encoded)"]
    #[inline(always)]
    pub fn cur_channel(&self) -> CUR_CHANNEL_R {
        CUR_CHANNEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub fn otp_key_ready(&self) -> OTP_KEY_READY_R {
        OTP_KEY_READY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
}
