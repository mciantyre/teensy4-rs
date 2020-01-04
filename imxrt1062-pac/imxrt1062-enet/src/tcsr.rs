#[doc = "Reader of register TCSR%s"]
pub type R = crate::R<u32, super::TCSR>;
#[doc = "Writer for register TCSR%s"]
pub type W = crate::W<u32, super::TCSR>;
#[doc = "Register TCSR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::TCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
    #[doc = "0: DMA request is disabled"]
    TDRE_0 = 0,
    #[doc = "1: DMA request is enabled"]
    TDRE_1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDRE`"]
pub type TDRE_R = crate::R<bool, TDRE_A>;
impl TDRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::TDRE_0,
            true => TDRE_A::TDRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDRE_0`"]
    #[inline(always)]
    pub fn is_tdre_0(&self) -> bool {
        *self == TDRE_A::TDRE_0
    }
    #[doc = "Checks if the value of the field is `TDRE_1`"]
    #[inline(always)]
    pub fn is_tdre_1(&self) -> bool {
        *self == TDRE_A::TDRE_1
    }
}
#[doc = "Write proxy for field `TDRE`"]
pub struct TDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn tdre_0(self) -> &'a mut W {
        self.variant(TDRE_A::TDRE_0)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn tdre_1(self) -> &'a mut W {
        self.variant(TDRE_A::TDRE_1)
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
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMODE_A {
    #[doc = "0: Timer Channel is disabled."]
    TMODE_0 = 0,
    #[doc = "1: Timer Channel is configured for Input Capture on rising edge."]
    TMODE_1 = 1,
    #[doc = "2: Timer Channel is configured for Input Capture on falling edge."]
    TMODE_2 = 2,
    #[doc = "3: Timer Channel is configured for Input Capture on both edges."]
    TMODE_3 = 3,
    #[doc = "4: Timer Channel is configured for Output Compare - software only."]
    TMODE_4 = 4,
    #[doc = "5: Timer Channel is configured for Output Compare - toggle output on compare."]
    TMODE_5 = 5,
    #[doc = "6: Timer Channel is configured for Output Compare - clear output on compare."]
    TMODE_6 = 6,
    #[doc = "7: Timer Channel is configured for Output Compare - set output on compare."]
    TMODE_7 = 7,
    #[doc = "9: Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMODE_9 = 9,
    #[doc = "10: Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMODE_10 = 10,
    #[doc = "14: Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMODE_14 = 14,
    #[doc = "15: Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMODE_15 = 15,
}
impl From<TMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMODE`"]
pub type TMODE_R = crate::R<u8, TMODE_A>;
impl TMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMODE_A::TMODE_0),
            1 => Val(TMODE_A::TMODE_1),
            2 => Val(TMODE_A::TMODE_2),
            3 => Val(TMODE_A::TMODE_3),
            4 => Val(TMODE_A::TMODE_4),
            5 => Val(TMODE_A::TMODE_5),
            6 => Val(TMODE_A::TMODE_6),
            7 => Val(TMODE_A::TMODE_7),
            9 => Val(TMODE_A::TMODE_9),
            10 => Val(TMODE_A::TMODE_10),
            14 => Val(TMODE_A::TMODE_14),
            15 => Val(TMODE_A::TMODE_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMODE_0`"]
    #[inline(always)]
    pub fn is_tmode_0(&self) -> bool {
        *self == TMODE_A::TMODE_0
    }
    #[doc = "Checks if the value of the field is `TMODE_1`"]
    #[inline(always)]
    pub fn is_tmode_1(&self) -> bool {
        *self == TMODE_A::TMODE_1
    }
    #[doc = "Checks if the value of the field is `TMODE_2`"]
    #[inline(always)]
    pub fn is_tmode_2(&self) -> bool {
        *self == TMODE_A::TMODE_2
    }
    #[doc = "Checks if the value of the field is `TMODE_3`"]
    #[inline(always)]
    pub fn is_tmode_3(&self) -> bool {
        *self == TMODE_A::TMODE_3
    }
    #[doc = "Checks if the value of the field is `TMODE_4`"]
    #[inline(always)]
    pub fn is_tmode_4(&self) -> bool {
        *self == TMODE_A::TMODE_4
    }
    #[doc = "Checks if the value of the field is `TMODE_5`"]
    #[inline(always)]
    pub fn is_tmode_5(&self) -> bool {
        *self == TMODE_A::TMODE_5
    }
    #[doc = "Checks if the value of the field is `TMODE_6`"]
    #[inline(always)]
    pub fn is_tmode_6(&self) -> bool {
        *self == TMODE_A::TMODE_6
    }
    #[doc = "Checks if the value of the field is `TMODE_7`"]
    #[inline(always)]
    pub fn is_tmode_7(&self) -> bool {
        *self == TMODE_A::TMODE_7
    }
    #[doc = "Checks if the value of the field is `TMODE_9`"]
    #[inline(always)]
    pub fn is_tmode_9(&self) -> bool {
        *self == TMODE_A::TMODE_9
    }
    #[doc = "Checks if the value of the field is `TMODE_10`"]
    #[inline(always)]
    pub fn is_tmode_10(&self) -> bool {
        *self == TMODE_A::TMODE_10
    }
    #[doc = "Checks if the value of the field is `TMODE_14`"]
    #[inline(always)]
    pub fn is_tmode_14(&self) -> bool {
        *self == TMODE_A::TMODE_14
    }
    #[doc = "Checks if the value of the field is `TMODE_15`"]
    #[inline(always)]
    pub fn is_tmode_15(&self) -> bool {
        *self == TMODE_A::TMODE_15
    }
}
#[doc = "Write proxy for field `TMODE`"]
pub struct TMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer Channel is disabled."]
    #[inline(always)]
    pub fn tmode_0(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_0)
    }
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    #[inline(always)]
    pub fn tmode_1(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_1)
    }
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    #[inline(always)]
    pub fn tmode_2(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_2)
    }
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    #[inline(always)]
    pub fn tmode_3(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_3)
    }
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    #[inline(always)]
    pub fn tmode_4(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_4)
    }
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    #[inline(always)]
    pub fn tmode_5(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_5)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    #[inline(always)]
    pub fn tmode_6(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_6)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    #[inline(always)]
    pub fn tmode_7(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_7)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    #[inline(always)]
    pub fn tmode_9(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_9)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    #[inline(always)]
    pub fn tmode_10(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_10)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    #[inline(always)]
    pub fn tmode_14(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_14)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    #[inline(always)]
    pub fn tmode_15(self) -> &'a mut W {
        self.variant(TMODE_A::TMODE_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Interrupt is disabled"]
    TIE_0 = 0,
    #[doc = "1: Interrupt is enabled"]
    TIE_1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::TIE_0,
            true => TIE_A::TIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE_0`"]
    #[inline(always)]
    pub fn is_tie_0(&self) -> bool {
        *self == TIE_A::TIE_0
    }
    #[doc = "Checks if the value of the field is `TIE_1`"]
    #[inline(always)]
    pub fn is_tie_1(&self) -> bool {
        *self == TIE_A::TIE_1
    }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn tie_0(self) -> &'a mut W {
        self.variant(TIE_A::TIE_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn tie_1(self) -> &'a mut W {
        self.variant(TIE_A::TIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Timer Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TF_A {
    #[doc = "0: Input Capture or Output Compare has not occurred."]
    TF_0 = 0,
    #[doc = "1: Input Capture or Output Compare has occurred."]
    TF_1 = 1,
}
impl From<TF_A> for bool {
    #[inline(always)]
    fn from(variant: TF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TF`"]
pub type TF_R = crate::R<bool, TF_A>;
impl TF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_A {
        match self.bits {
            false => TF_A::TF_0,
            true => TF_A::TF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF_0`"]
    #[inline(always)]
    pub fn is_tf_0(&self) -> bool {
        *self == TF_A::TF_0
    }
    #[doc = "Checks if the value of the field is `TF_1`"]
    #[inline(always)]
    pub fn is_tf_1(&self) -> bool {
        *self == TF_A::TF_1
    }
}
#[doc = "Write proxy for field `TF`"]
pub struct TF_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input Capture or Output Compare has not occurred."]
    #[inline(always)]
    pub fn tf_0(self) -> &'a mut W {
        self.variant(TF_A::TF_0)
    }
    #[doc = "Input Capture or Output Compare has occurred."]
    #[inline(always)]
    pub fn tf_1(self) -> &'a mut W {
        self.variant(TF_A::TF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Timer PulseWidth Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPWC_A {
    #[doc = "0: Pulse width is one 1588-clock cycle."]
    TPWC_0 = 0,
    #[doc = "1: Pulse width is two 1588-clock cycles."]
    TPWC_1 = 1,
    #[doc = "2: Pulse width is three 1588-clock cycles."]
    TPWC_2 = 2,
    #[doc = "3: Pulse width is four 1588-clock cycles."]
    TPWC_3 = 3,
    #[doc = "31: Pulse width is 32 1588-clock cycles."]
    TPWC_31 = 31,
}
impl From<TPWC_A> for u8 {
    #[inline(always)]
    fn from(variant: TPWC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TPWC`"]
pub type TPWC_R = crate::R<u8, TPWC_A>;
impl TPWC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TPWC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TPWC_A::TPWC_0),
            1 => Val(TPWC_A::TPWC_1),
            2 => Val(TPWC_A::TPWC_2),
            3 => Val(TPWC_A::TPWC_3),
            31 => Val(TPWC_A::TPWC_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TPWC_0`"]
    #[inline(always)]
    pub fn is_tpwc_0(&self) -> bool {
        *self == TPWC_A::TPWC_0
    }
    #[doc = "Checks if the value of the field is `TPWC_1`"]
    #[inline(always)]
    pub fn is_tpwc_1(&self) -> bool {
        *self == TPWC_A::TPWC_1
    }
    #[doc = "Checks if the value of the field is `TPWC_2`"]
    #[inline(always)]
    pub fn is_tpwc_2(&self) -> bool {
        *self == TPWC_A::TPWC_2
    }
    #[doc = "Checks if the value of the field is `TPWC_3`"]
    #[inline(always)]
    pub fn is_tpwc_3(&self) -> bool {
        *self == TPWC_A::TPWC_3
    }
    #[doc = "Checks if the value of the field is `TPWC_31`"]
    #[inline(always)]
    pub fn is_tpwc_31(&self) -> bool {
        *self == TPWC_A::TPWC_31
    }
}
#[doc = "Write proxy for field `TPWC`"]
pub struct TPWC_W<'a> {
    w: &'a mut W,
}
impl<'a> TPWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPWC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pulse width is one 1588-clock cycle."]
    #[inline(always)]
    pub fn tpwc_0(self) -> &'a mut W {
        self.variant(TPWC_A::TPWC_0)
    }
    #[doc = "Pulse width is two 1588-clock cycles."]
    #[inline(always)]
    pub fn tpwc_1(self) -> &'a mut W {
        self.variant(TPWC_A::TPWC_1)
    }
    #[doc = "Pulse width is three 1588-clock cycles."]
    #[inline(always)]
    pub fn tpwc_2(self) -> &'a mut W {
        self.variant(TPWC_A::TPWC_2)
    }
    #[doc = "Pulse width is four 1588-clock cycles."]
    #[inline(always)]
    pub fn tpwc_3(self) -> &'a mut W {
        self.variant(TPWC_A::TPWC_3)
    }
    #[doc = "Pulse width is 32 1588-clock cycles."]
    #[inline(always)]
    pub fn tpwc_31(self) -> &'a mut W {
        self.variant(TPWC_A::TPWC_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - Timer PulseWidth Control"]
    #[inline(always)]
    pub fn tpwc(&self) -> TPWC_R {
        TPWC_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TDRE_W {
        TDRE_W { w: self }
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    pub fn tmode(&mut self) -> TMODE_W {
        TMODE_W { w: self }
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    pub fn tf(&mut self) -> TF_W {
        TF_W { w: self }
    }
    #[doc = "Bits 11:15 - Timer PulseWidth Control"]
    #[inline(always)]
    pub fn tpwc(&mut self) -> TPWC_W {
        TPWC_W { w: self }
    }
}
