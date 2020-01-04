#[doc = "Reader of register CTRL_TOG"]
pub type R = crate::R<u32, super::CTRL_TOG>;
#[doc = "Writer for register CTRL_TOG"]
pub type W = crate::W<u32, super::CTRL_TOG>;
#[doc = "Register CTRL_TOG `reset()`'s with value 0xf080_0000"]
impl crate::ResetValue for super::CTRL_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf080_0000
    }
}
#[doc = "Per-channel interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHANNEL_INTERRUPT_ENABLE_A {
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "4: CH2"]
    CH2 = 4,
    #[doc = "8: CH3"]
    CH3 = 8,
}
impl From<CHANNEL_INTERRUPT_ENABLE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_INTERRUPT_ENABLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHANNEL_INTERRUPT_ENABLE`"]
pub type CHANNEL_INTERRUPT_ENABLE_R = crate::R<u8, CHANNEL_INTERRUPT_ENABLE_A>;
impl CHANNEL_INTERRUPT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHANNEL_INTERRUPT_ENABLE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CHANNEL_INTERRUPT_ENABLE_A::CH0),
            2 => Val(CHANNEL_INTERRUPT_ENABLE_A::CH1),
            4 => Val(CHANNEL_INTERRUPT_ENABLE_A::CH2),
            8 => Val(CHANNEL_INTERRUPT_ENABLE_A::CH3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLE_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLE_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLE_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLE_A::CH3
    }
}
#[doc = "Write proxy for field `CHANNEL_INTERRUPT_ENABLE`"]
pub struct CHANNEL_INTERRUPT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_INTERRUPT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNEL_INTERRUPT_ENABLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CH0"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLE_A::CH0)
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLE_A::CH1)
    }
    #[doc = "CH2"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLE_A::CH2)
    }
    #[doc = "CH3"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLE_A::CH3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_CONTEXT_SWITCHING`"]
pub type ENABLE_CONTEXT_SWITCHING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_CONTEXT_SWITCHING`"]
pub struct ENABLE_CONTEXT_SWITCHING_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_CONTEXT_SWITCHING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_CONTEXT_CACHING`"]
pub type ENABLE_CONTEXT_CACHING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_CONTEXT_CACHING`"]
pub struct ENABLE_CONTEXT_CACHING_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_CONTEXT_CACHING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `GATHER_RESIDUAL_WRITES`"]
pub type GATHER_RESIDUAL_WRITES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GATHER_RESIDUAL_WRITES`"]
pub struct GATHER_RESIDUAL_WRITES_W<'a> {
    w: &'a mut W,
}
impl<'a> GATHER_RESIDUAL_WRITES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Indicates whether the SHA1/SHA2 functions are present.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESENT_SHA_A {
    #[doc = "0: Absent"]
    ABSENT = 0,
    #[doc = "1: Present"]
    PRESENT = 1,
}
impl From<PRESENT_SHA_A> for bool {
    #[inline(always)]
    fn from(variant: PRESENT_SHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRESENT_SHA`"]
pub type PRESENT_SHA_R = crate::R<bool, PRESENT_SHA_A>;
impl PRESENT_SHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESENT_SHA_A {
        match self.bits {
            false => PRESENT_SHA_A::ABSENT,
            true => PRESENT_SHA_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `ABSENT`"]
    #[inline(always)]
    pub fn is_absent(&self) -> bool {
        *self == PRESENT_SHA_A::ABSENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == PRESENT_SHA_A::PRESENT
    }
}
#[doc = "Indicates whether the crypto (cipher/hash) functions are present.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESENT_CRYPTO_A {
    #[doc = "0: Absent"]
    ABSENT = 0,
    #[doc = "1: Present"]
    PRESENT = 1,
}
impl From<PRESENT_CRYPTO_A> for bool {
    #[inline(always)]
    fn from(variant: PRESENT_CRYPTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRESENT_CRYPTO`"]
pub type PRESENT_CRYPTO_R = crate::R<bool, PRESENT_CRYPTO_A>;
impl PRESENT_CRYPTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESENT_CRYPTO_A {
        match self.bits {
            false => PRESENT_CRYPTO_A::ABSENT,
            true => PRESENT_CRYPTO_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `ABSENT`"]
    #[inline(always)]
    pub fn is_absent(&self) -> bool {
        *self == PRESENT_CRYPTO_A::ABSENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == PRESENT_CRYPTO_A::PRESENT
    }
}
#[doc = "Reader of field `CLKGATE`"]
pub type CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGATE`"]
pub struct CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_W<'a> {
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
#[doc = "Reader of field `SFTRST`"]
pub type SFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTRST`"]
pub struct SFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRST_W<'a> {
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
    #[doc = "Bits 0:7 - Per-channel interrupt enable bit"]
    #[inline(always)]
    pub fn channel_interrupt_enable(&self) -> CHANNEL_INTERRUPT_ENABLE_R {
        CHANNEL_INTERRUPT_ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 21 - Enable automatic context switching for the channels"]
    #[inline(always)]
    pub fn enable_context_switching(&self) -> ENABLE_CONTEXT_SWITCHING_R {
        ENABLE_CONTEXT_SWITCHING_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub fn enable_context_caching(&self) -> ENABLE_CONTEXT_CACHING_R {
        ENABLE_CONTEXT_CACHING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub fn gather_residual_writes(&self) -> GATHER_RESIDUAL_WRITES_R {
        GATHER_RESIDUAL_WRITES_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub fn present_sha(&self) -> PRESENT_SHA_R {
        PRESENT_SHA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub fn present_crypto(&self) -> PRESENT_CRYPTO_R {
        PRESENT_CRYPTO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Set this bit to zero to enable a normal DCP operation"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Per-channel interrupt enable bit"]
    #[inline(always)]
    pub fn channel_interrupt_enable(&mut self) -> CHANNEL_INTERRUPT_ENABLE_W {
        CHANNEL_INTERRUPT_ENABLE_W { w: self }
    }
    #[doc = "Bit 21 - Enable automatic context switching for the channels"]
    #[inline(always)]
    pub fn enable_context_switching(&mut self) -> ENABLE_CONTEXT_SWITCHING_W {
        ENABLE_CONTEXT_SWITCHING_W { w: self }
    }
    #[doc = "Bit 22 - The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub fn enable_context_caching(&mut self) -> ENABLE_CONTEXT_CACHING_W {
        ENABLE_CONTEXT_CACHING_W { w: self }
    }
    #[doc = "Bit 23 - The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub fn gather_residual_writes(&mut self) -> GATHER_RESIDUAL_WRITES_W {
        GATHER_RESIDUAL_WRITES_W { w: self }
    }
    #[doc = "Bit 30 - This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
    #[doc = "Bit 31 - Set this bit to zero to enable a normal DCP operation"]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SFTRST_W {
        SFTRST_W { w: self }
    }
}
