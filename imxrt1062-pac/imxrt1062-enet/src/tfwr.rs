#[doc = "Reader of register TFWR"]
pub type R = crate::R<u32, super::TFWR>;
#[doc = "Writer for register TFWR"]
pub type W = crate::W<u32, super::TFWR>;
#[doc = "Register TFWR `reset()`'s with value 0"]
impl crate::ResetValue for super::TFWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit FIFO Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TFWR_A {
    #[doc = "0: 64 bytes written."]
    TFWR_0 = 0,
    #[doc = "1: 64 bytes written."]
    TFWR_1 = 1,
    #[doc = "2: 128 bytes written."]
    TFWR_2 = 2,
    #[doc = "3: 192 bytes written."]
    TFWR_3 = 3,
    #[doc = "31: 1984 bytes written."]
    TFWR_31 = 31,
}
impl From<TFWR_A> for u8 {
    #[inline(always)]
    fn from(variant: TFWR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TFWR`"]
pub type TFWR_R = crate::R<u8, TFWR_A>;
impl TFWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TFWR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TFWR_A::TFWR_0),
            1 => Val(TFWR_A::TFWR_1),
            2 => Val(TFWR_A::TFWR_2),
            3 => Val(TFWR_A::TFWR_3),
            31 => Val(TFWR_A::TFWR_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TFWR_0`"]
    #[inline(always)]
    pub fn is_tfwr_0(&self) -> bool {
        *self == TFWR_A::TFWR_0
    }
    #[doc = "Checks if the value of the field is `TFWR_1`"]
    #[inline(always)]
    pub fn is_tfwr_1(&self) -> bool {
        *self == TFWR_A::TFWR_1
    }
    #[doc = "Checks if the value of the field is `TFWR_2`"]
    #[inline(always)]
    pub fn is_tfwr_2(&self) -> bool {
        *self == TFWR_A::TFWR_2
    }
    #[doc = "Checks if the value of the field is `TFWR_3`"]
    #[inline(always)]
    pub fn is_tfwr_3(&self) -> bool {
        *self == TFWR_A::TFWR_3
    }
    #[doc = "Checks if the value of the field is `TFWR_31`"]
    #[inline(always)]
    pub fn is_tfwr_31(&self) -> bool {
        *self == TFWR_A::TFWR_31
    }
}
#[doc = "Write proxy for field `TFWR`"]
pub struct TFWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFWR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "64 bytes written."]
    #[inline(always)]
    pub fn tfwr_0(self) -> &'a mut W {
        self.variant(TFWR_A::TFWR_0)
    }
    #[doc = "64 bytes written."]
    #[inline(always)]
    pub fn tfwr_1(self) -> &'a mut W {
        self.variant(TFWR_A::TFWR_1)
    }
    #[doc = "128 bytes written."]
    #[inline(always)]
    pub fn tfwr_2(self) -> &'a mut W {
        self.variant(TFWR_A::TFWR_2)
    }
    #[doc = "192 bytes written."]
    #[inline(always)]
    pub fn tfwr_3(self) -> &'a mut W {
        self.variant(TFWR_A::TFWR_3)
    }
    #[doc = "1984 bytes written."]
    #[inline(always)]
    pub fn tfwr_31(self) -> &'a mut W {
        self.variant(TFWR_A::TFWR_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Store And Forward Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRFWD_A {
    #[doc = "0: Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    STRFWD_0 = 0,
    #[doc = "1: Enabled."]
    STRFWD_1 = 1,
}
impl From<STRFWD_A> for bool {
    #[inline(always)]
    fn from(variant: STRFWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STRFWD`"]
pub type STRFWD_R = crate::R<bool, STRFWD_A>;
impl STRFWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRFWD_A {
        match self.bits {
            false => STRFWD_A::STRFWD_0,
            true => STRFWD_A::STRFWD_1,
        }
    }
    #[doc = "Checks if the value of the field is `STRFWD_0`"]
    #[inline(always)]
    pub fn is_strfwd_0(&self) -> bool {
        *self == STRFWD_A::STRFWD_0
    }
    #[doc = "Checks if the value of the field is `STRFWD_1`"]
    #[inline(always)]
    pub fn is_strfwd_1(&self) -> bool {
        *self == STRFWD_A::STRFWD_1
    }
}
#[doc = "Write proxy for field `STRFWD`"]
pub struct STRFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> STRFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRFWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    #[inline(always)]
    pub fn strfwd_0(self) -> &'a mut W {
        self.variant(STRFWD_A::STRFWD_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn strfwd_1(self) -> &'a mut W {
        self.variant(STRFWD_A::STRFWD_1)
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
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline(always)]
    pub fn tfwr(&self) -> TFWR_R {
        TFWR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline(always)]
    pub fn strfwd(&self) -> STRFWD_R {
        STRFWD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline(always)]
    pub fn tfwr(&mut self) -> TFWR_W {
        TFWR_W { w: self }
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline(always)]
    pub fn strfwd(&mut self) -> STRFWD_W {
        STRFWD_W { w: self }
    }
}
