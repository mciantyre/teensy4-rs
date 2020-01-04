#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `R0T0`"]
pub type R0T0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R0T0`"]
pub struct R0T0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0T0_W<'a> {
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
#[doc = "Reader of field `R1T1`"]
pub type R1T1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R1T1`"]
pub struct R1T1_W<'a> {
    w: &'a mut W,
}
impl<'a> R1T1_W<'a> {
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
#[doc = "Reader of field `R2T2`"]
pub type R2T2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R2T2`"]
pub struct R2T2_W<'a> {
    w: &'a mut W,
}
impl<'a> R2T2_W<'a> {
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
#[doc = "Reader of field `R3T3`"]
pub type R3T3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R3T3`"]
pub struct R3T3_W<'a> {
    w: &'a mut W,
}
impl<'a> R3T3_W<'a> {
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
#[doc = "Reader of field `R4T4`"]
pub type R4T4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R4T4`"]
pub struct R4T4_W<'a> {
    w: &'a mut W,
}
impl<'a> R4T4_W<'a> {
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
#[doc = "Reader of field `R5T5`"]
pub type R5T5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R5T5`"]
pub struct R5T5_W<'a> {
    w: &'a mut W,
}
impl<'a> R5T5_W<'a> {
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
#[doc = "Reader of field `R6T6`"]
pub type R6T6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R6T6`"]
pub struct R6T6_W<'a> {
    w: &'a mut W,
}
impl<'a> R6T6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `R7T7`"]
pub type R7T7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R7T7`"]
pub struct R7T7_W<'a> {
    w: &'a mut W,
}
impl<'a> R7T7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `R8T8`"]
pub type R8T8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R8T8`"]
pub struct R8T8_W<'a> {
    w: &'a mut W,
}
impl<'a> R8T8_W<'a> {
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
#[doc = "Reader of field `R9T9`"]
pub type R9T9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R9T9`"]
pub struct R9T9_W<'a> {
    w: &'a mut W,
}
impl<'a> R9T9_W<'a> {
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
#[doc = "Idle Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLINE_A {
    #[doc = "0: Receiver was not idle before receiving this character."]
    IDLINE_0 = 0,
    #[doc = "1: Receiver was idle before receiving this character."]
    IDLINE_1 = 1,
}
impl From<IDLINE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLINE`"]
pub type IDLINE_R = crate::R<bool, IDLINE_A>;
impl IDLINE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLINE_A {
        match self.bits {
            false => IDLINE_A::IDLINE_0,
            true => IDLINE_A::IDLINE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLINE_0`"]
    #[inline(always)]
    pub fn is_idline_0(&self) -> bool {
        *self == IDLINE_A::IDLINE_0
    }
    #[doc = "Checks if the value of the field is `IDLINE_1`"]
    #[inline(always)]
    pub fn is_idline_1(&self) -> bool {
        *self == IDLINE_A::IDLINE_1
    }
}
#[doc = "Receive Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPT_A {
    #[doc = "0: Receive buffer contains valid data."]
    RXEMPT_0 = 0,
    #[doc = "1: Receive buffer is empty, data returned on read is not valid."]
    RXEMPT_1 = 1,
}
impl From<RXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEMPT`"]
pub type RXEMPT_R = crate::R<bool, RXEMPT_A>;
impl RXEMPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPT_A {
        match self.bits {
            false => RXEMPT_A::RXEMPT_0,
            true => RXEMPT_A::RXEMPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEMPT_0`"]
    #[inline(always)]
    pub fn is_rxempt_0(&self) -> bool {
        *self == RXEMPT_A::RXEMPT_0
    }
    #[doc = "Checks if the value of the field is `RXEMPT_1`"]
    #[inline(always)]
    pub fn is_rxempt_1(&self) -> bool {
        *self == RXEMPT_A::RXEMPT_1
    }
}
#[doc = "Frame Error / Transmit Special Character\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRETSC_A {
    #[doc = "0: The dataword was received without a frame error on read, or transmit a normal character on write."]
    FRETSC_0 = 0,
    #[doc = "1: The dataword was received with a frame error, or transmit an idle or break character on transmit."]
    FRETSC_1 = 1,
}
impl From<FRETSC_A> for bool {
    #[inline(always)]
    fn from(variant: FRETSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRETSC`"]
pub type FRETSC_R = crate::R<bool, FRETSC_A>;
impl FRETSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRETSC_A {
        match self.bits {
            false => FRETSC_A::FRETSC_0,
            true => FRETSC_A::FRETSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRETSC_0`"]
    #[inline(always)]
    pub fn is_fretsc_0(&self) -> bool {
        *self == FRETSC_A::FRETSC_0
    }
    #[doc = "Checks if the value of the field is `FRETSC_1`"]
    #[inline(always)]
    pub fn is_fretsc_1(&self) -> bool {
        *self == FRETSC_A::FRETSC_1
    }
}
#[doc = "Write proxy for field `FRETSC`"]
pub struct FRETSC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRETSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRETSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The dataword was received without a frame error on read, or transmit a normal character on write."]
    #[inline(always)]
    pub fn fretsc_0(self) -> &'a mut W {
        self.variant(FRETSC_A::FRETSC_0)
    }
    #[doc = "The dataword was received with a frame error, or transmit an idle or break character on transmit."]
    #[inline(always)]
    pub fn fretsc_1(self) -> &'a mut W {
        self.variant(FRETSC_A::FRETSC_1)
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
#[doc = "PARITYE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYE_A {
    #[doc = "0: The dataword was received without a parity error."]
    PARITYE_0 = 0,
    #[doc = "1: The dataword was received with a parity error."]
    PARITYE_1 = 1,
}
impl From<PARITYE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PARITYE`"]
pub type PARITYE_R = crate::R<bool, PARITYE_A>;
impl PARITYE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYE_A {
        match self.bits {
            false => PARITYE_A::PARITYE_0,
            true => PARITYE_A::PARITYE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PARITYE_0`"]
    #[inline(always)]
    pub fn is_paritye_0(&self) -> bool {
        *self == PARITYE_A::PARITYE_0
    }
    #[doc = "Checks if the value of the field is `PARITYE_1`"]
    #[inline(always)]
    pub fn is_paritye_1(&self) -> bool {
        *self == PARITYE_A::PARITYE_1
    }
}
#[doc = "NOISY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOISY_A {
    #[doc = "0: The dataword was received without noise."]
    NOISY_0 = 0,
    #[doc = "1: The data was received with noise."]
    NOISY_1 = 1,
}
impl From<NOISY_A> for bool {
    #[inline(always)]
    fn from(variant: NOISY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOISY`"]
pub type NOISY_R = crate::R<bool, NOISY_A>;
impl NOISY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOISY_A {
        match self.bits {
            false => NOISY_A::NOISY_0,
            true => NOISY_A::NOISY_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOISY_0`"]
    #[inline(always)]
    pub fn is_noisy_0(&self) -> bool {
        *self == NOISY_A::NOISY_0
    }
    #[doc = "Checks if the value of the field is `NOISY_1`"]
    #[inline(always)]
    pub fn is_noisy_1(&self) -> bool {
        *self == NOISY_A::NOISY_1
    }
}
impl R {
    #[doc = "Bit 0 - R0T0"]
    #[inline(always)]
    pub fn r0t0(&self) -> R0T0_R {
        R0T0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - R1T1"]
    #[inline(always)]
    pub fn r1t1(&self) -> R1T1_R {
        R1T1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - R2T2"]
    #[inline(always)]
    pub fn r2t2(&self) -> R2T2_R {
        R2T2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - R3T3"]
    #[inline(always)]
    pub fn r3t3(&self) -> R3T3_R {
        R3T3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - R4T4"]
    #[inline(always)]
    pub fn r4t4(&self) -> R4T4_R {
        R4T4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - R5T5"]
    #[inline(always)]
    pub fn r5t5(&self) -> R5T5_R {
        R5T5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - R6T6"]
    #[inline(always)]
    pub fn r6t6(&self) -> R6T6_R {
        R6T6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - R7T7"]
    #[inline(always)]
    pub fn r7t7(&self) -> R7T7_R {
        R7T7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - R8T8"]
    #[inline(always)]
    pub fn r8t8(&self) -> R8T8_R {
        R8T8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - R9T9"]
    #[inline(always)]
    pub fn r9t9(&self) -> R9T9_R {
        R9T9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Idle Line"]
    #[inline(always)]
    pub fn idline(&self) -> IDLINE_R {
        IDLINE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RXEMPT_R {
        RXEMPT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline(always)]
    pub fn fretsc(&self) -> FRETSC_R {
        FRETSC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PARITYE"]
    #[inline(always)]
    pub fn paritye(&self) -> PARITYE_R {
        PARITYE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - NOISY"]
    #[inline(always)]
    pub fn noisy(&self) -> NOISY_R {
        NOISY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - R0T0"]
    #[inline(always)]
    pub fn r0t0(&mut self) -> R0T0_W {
        R0T0_W { w: self }
    }
    #[doc = "Bit 1 - R1T1"]
    #[inline(always)]
    pub fn r1t1(&mut self) -> R1T1_W {
        R1T1_W { w: self }
    }
    #[doc = "Bit 2 - R2T2"]
    #[inline(always)]
    pub fn r2t2(&mut self) -> R2T2_W {
        R2T2_W { w: self }
    }
    #[doc = "Bit 3 - R3T3"]
    #[inline(always)]
    pub fn r3t3(&mut self) -> R3T3_W {
        R3T3_W { w: self }
    }
    #[doc = "Bit 4 - R4T4"]
    #[inline(always)]
    pub fn r4t4(&mut self) -> R4T4_W {
        R4T4_W { w: self }
    }
    #[doc = "Bit 5 - R5T5"]
    #[inline(always)]
    pub fn r5t5(&mut self) -> R5T5_W {
        R5T5_W { w: self }
    }
    #[doc = "Bit 6 - R6T6"]
    #[inline(always)]
    pub fn r6t6(&mut self) -> R6T6_W {
        R6T6_W { w: self }
    }
    #[doc = "Bit 7 - R7T7"]
    #[inline(always)]
    pub fn r7t7(&mut self) -> R7T7_W {
        R7T7_W { w: self }
    }
    #[doc = "Bit 8 - R8T8"]
    #[inline(always)]
    pub fn r8t8(&mut self) -> R8T8_W {
        R8T8_W { w: self }
    }
    #[doc = "Bit 9 - R9T9"]
    #[inline(always)]
    pub fn r9t9(&mut self) -> R9T9_W {
        R9T9_W { w: self }
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline(always)]
    pub fn fretsc(&mut self) -> FRETSC_W {
        FRETSC_W { w: self }
    }
}
