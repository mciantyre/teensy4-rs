#[doc = "Reader of register SDRAMCR3"]
pub type R = crate::R<u32, super::SDRAMCR3>;
#[doc = "Writer for register SDRAMCR3"]
pub type W = crate::W<u32, super::SDRAMCR3>;
#[doc = "Register SDRAMCR3 `reset()`'s with value 0x4080_8000"]
impl crate::ResetValue for super::SDRAMCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4080_8000
    }
}
#[doc = "Reader of field `REN`"]
pub type REN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REN`"]
pub struct REN_W<'a> {
    w: &'a mut W,
}
impl<'a> REN_W<'a> {
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
#[doc = "Refresh burst length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REBL_A {
    #[doc = "0: 1"]
    REBL_0 = 0,
    #[doc = "1: 2"]
    REBL_1 = 1,
    #[doc = "2: 3"]
    REBL_2 = 2,
    #[doc = "3: 4"]
    REBL_3 = 3,
    #[doc = "4: 5"]
    REBL_4 = 4,
    #[doc = "5: 6"]
    REBL_5 = 5,
    #[doc = "6: 7"]
    REBL_6 = 6,
    #[doc = "7: 8"]
    REBL_7 = 7,
}
impl From<REBL_A> for u8 {
    #[inline(always)]
    fn from(variant: REBL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REBL`"]
pub type REBL_R = crate::R<u8, REBL_A>;
impl REBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REBL_A {
        match self.bits {
            0 => REBL_A::REBL_0,
            1 => REBL_A::REBL_1,
            2 => REBL_A::REBL_2,
            3 => REBL_A::REBL_3,
            4 => REBL_A::REBL_4,
            5 => REBL_A::REBL_5,
            6 => REBL_A::REBL_6,
            7 => REBL_A::REBL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REBL_0`"]
    #[inline(always)]
    pub fn is_rebl_0(&self) -> bool {
        *self == REBL_A::REBL_0
    }
    #[doc = "Checks if the value of the field is `REBL_1`"]
    #[inline(always)]
    pub fn is_rebl_1(&self) -> bool {
        *self == REBL_A::REBL_1
    }
    #[doc = "Checks if the value of the field is `REBL_2`"]
    #[inline(always)]
    pub fn is_rebl_2(&self) -> bool {
        *self == REBL_A::REBL_2
    }
    #[doc = "Checks if the value of the field is `REBL_3`"]
    #[inline(always)]
    pub fn is_rebl_3(&self) -> bool {
        *self == REBL_A::REBL_3
    }
    #[doc = "Checks if the value of the field is `REBL_4`"]
    #[inline(always)]
    pub fn is_rebl_4(&self) -> bool {
        *self == REBL_A::REBL_4
    }
    #[doc = "Checks if the value of the field is `REBL_5`"]
    #[inline(always)]
    pub fn is_rebl_5(&self) -> bool {
        *self == REBL_A::REBL_5
    }
    #[doc = "Checks if the value of the field is `REBL_6`"]
    #[inline(always)]
    pub fn is_rebl_6(&self) -> bool {
        *self == REBL_A::REBL_6
    }
    #[doc = "Checks if the value of the field is `REBL_7`"]
    #[inline(always)]
    pub fn is_rebl_7(&self) -> bool {
        *self == REBL_A::REBL_7
    }
}
#[doc = "Write proxy for field `REBL`"]
pub struct REBL_W<'a> {
    w: &'a mut W,
}
impl<'a> REBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REBL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn rebl_0(self) -> &'a mut W {
        self.variant(REBL_A::REBL_0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn rebl_1(self) -> &'a mut W {
        self.variant(REBL_A::REBL_1)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn rebl_2(self) -> &'a mut W {
        self.variant(REBL_A::REBL_2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn rebl_3(self) -> &'a mut W {
        self.variant(REBL_A::REBL_3)
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn rebl_4(self) -> &'a mut W {
        self.variant(REBL_A::REBL_4)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn rebl_5(self) -> &'a mut W {
        self.variant(REBL_A::REBL_5)
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn rebl_6(self) -> &'a mut W {
        self.variant(REBL_A::REBL_6)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn rebl_7(self) -> &'a mut W {
        self.variant(REBL_A::REBL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Prescaler timer period\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: 256*16 cycle"]
    PRESCALE_0 = 0,
    #[doc = "1: PRESCALE*16 cycle"]
    PRESCALE_1 = 1,
    #[doc = "2: PRESCALE*16 cycle"]
    PRESCALE_2 = 2,
    #[doc = "3: PRESCALE*16 cycle"]
    PRESCALE_3 = 3,
    #[doc = "4: PRESCALE*16 cycle"]
    PRESCALE_4 = 4,
    #[doc = "5: PRESCALE*16 cycle"]
    PRESCALE_5 = 5,
    #[doc = "6: PRESCALE*16 cycle"]
    PRESCALE_6 = 6,
    #[doc = "7: PRESCALE*16 cycle"]
    PRESCALE_7 = 7,
    #[doc = "8: PRESCALE*16 cycle"]
    PRESCALE_8 = 8,
    #[doc = "9: PRESCALE*16 cycle"]
    PRESCALE_9 = 9,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, PRESCALE_A>;
impl PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCALE_A::PRESCALE_0),
            1 => Val(PRESCALE_A::PRESCALE_1),
            2 => Val(PRESCALE_A::PRESCALE_2),
            3 => Val(PRESCALE_A::PRESCALE_3),
            4 => Val(PRESCALE_A::PRESCALE_4),
            5 => Val(PRESCALE_A::PRESCALE_5),
            6 => Val(PRESCALE_A::PRESCALE_6),
            7 => Val(PRESCALE_A::PRESCALE_7),
            8 => Val(PRESCALE_A::PRESCALE_8),
            9 => Val(PRESCALE_A::PRESCALE_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALE_0`"]
    #[inline(always)]
    pub fn is_prescale_0(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_0
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1`"]
    #[inline(always)]
    pub fn is_prescale_1(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `PRESCALE_3`"]
    #[inline(always)]
    pub fn is_prescale_3(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `PRESCALE_4`"]
    #[inline(always)]
    pub fn is_prescale_4(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `PRESCALE_5`"]
    #[inline(always)]
    pub fn is_prescale_5(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `PRESCALE_6`"]
    #[inline(always)]
    pub fn is_prescale_6(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `PRESCALE_7`"]
    #[inline(always)]
    pub fn is_prescale_7(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_7
    }
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `PRESCALE_9`"]
    #[inline(always)]
    pub fn is_prescale_9(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_9
    }
}
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256*16 cycle"]
    #[inline(always)]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_0)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_1)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_2)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_3(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_3)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_4(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_4)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_5(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_5)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_6(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_6)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_7(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_7)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_8)
    }
    #[doc = "PRESCALE*16 cycle"]
    #[inline(always)]
    pub fn prescale_9(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Refresh timer period\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT_A {
    #[doc = "0: 256*Prescaler period"]
    RT_0 = 0,
    #[doc = "1: RT*Prescaler period"]
    RT_1 = 1,
    #[doc = "2: RT*Prescaler period"]
    RT_2 = 2,
    #[doc = "3: RT*Prescaler period"]
    RT_3 = 3,
    #[doc = "4: RT*Prescaler period"]
    RT_4 = 4,
    #[doc = "5: RT*Prescaler period"]
    RT_5 = 5,
    #[doc = "6: RT*Prescaler period"]
    RT_6 = 6,
    #[doc = "7: RT*Prescaler period"]
    RT_7 = 7,
    #[doc = "8: RT*Prescaler period"]
    RT_8 = 8,
    #[doc = "9: RT*Prescaler period"]
    RT_9 = 9,
}
impl From<RT_A> for u8 {
    #[inline(always)]
    fn from(variant: RT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RT`"]
pub type RT_R = crate::R<u8, RT_A>;
impl RT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RT_A::RT_0),
            1 => Val(RT_A::RT_1),
            2 => Val(RT_A::RT_2),
            3 => Val(RT_A::RT_3),
            4 => Val(RT_A::RT_4),
            5 => Val(RT_A::RT_5),
            6 => Val(RT_A::RT_6),
            7 => Val(RT_A::RT_7),
            8 => Val(RT_A::RT_8),
            9 => Val(RT_A::RT_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RT_0`"]
    #[inline(always)]
    pub fn is_rt_0(&self) -> bool {
        *self == RT_A::RT_0
    }
    #[doc = "Checks if the value of the field is `RT_1`"]
    #[inline(always)]
    pub fn is_rt_1(&self) -> bool {
        *self == RT_A::RT_1
    }
    #[doc = "Checks if the value of the field is `RT_2`"]
    #[inline(always)]
    pub fn is_rt_2(&self) -> bool {
        *self == RT_A::RT_2
    }
    #[doc = "Checks if the value of the field is `RT_3`"]
    #[inline(always)]
    pub fn is_rt_3(&self) -> bool {
        *self == RT_A::RT_3
    }
    #[doc = "Checks if the value of the field is `RT_4`"]
    #[inline(always)]
    pub fn is_rt_4(&self) -> bool {
        *self == RT_A::RT_4
    }
    #[doc = "Checks if the value of the field is `RT_5`"]
    #[inline(always)]
    pub fn is_rt_5(&self) -> bool {
        *self == RT_A::RT_5
    }
    #[doc = "Checks if the value of the field is `RT_6`"]
    #[inline(always)]
    pub fn is_rt_6(&self) -> bool {
        *self == RT_A::RT_6
    }
    #[doc = "Checks if the value of the field is `RT_7`"]
    #[inline(always)]
    pub fn is_rt_7(&self) -> bool {
        *self == RT_A::RT_7
    }
    #[doc = "Checks if the value of the field is `RT_8`"]
    #[inline(always)]
    pub fn is_rt_8(&self) -> bool {
        *self == RT_A::RT_8
    }
    #[doc = "Checks if the value of the field is `RT_9`"]
    #[inline(always)]
    pub fn is_rt_9(&self) -> bool {
        *self == RT_A::RT_9
    }
}
#[doc = "Write proxy for field `RT`"]
pub struct RT_W<'a> {
    w: &'a mut W,
}
impl<'a> RT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256*Prescaler period"]
    #[inline(always)]
    pub fn rt_0(self) -> &'a mut W {
        self.variant(RT_A::RT_0)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_1(self) -> &'a mut W {
        self.variant(RT_A::RT_1)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_2(self) -> &'a mut W {
        self.variant(RT_A::RT_2)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_3(self) -> &'a mut W {
        self.variant(RT_A::RT_3)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_4(self) -> &'a mut W {
        self.variant(RT_A::RT_4)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_5(self) -> &'a mut W {
        self.variant(RT_A::RT_5)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_6(self) -> &'a mut W {
        self.variant(RT_A::RT_6)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_7(self) -> &'a mut W {
        self.variant(RT_A::RT_7)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_8(self) -> &'a mut W {
        self.variant(RT_A::RT_8)
    }
    #[doc = "RT*Prescaler period"]
    #[inline(always)]
    pub fn rt_9(self) -> &'a mut W {
        self.variant(RT_A::RT_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Refresh urgent threshold\n\nValue on reset: 64"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UT_A {
    #[doc = "0: 256*Prescaler period"]
    UT_0 = 0,
    #[doc = "1: UT*Prescaler period"]
    UT_1 = 1,
    #[doc = "2: UT*Prescaler period"]
    UT_2 = 2,
    #[doc = "3: UT*Prescaler period"]
    UT_3 = 3,
    #[doc = "4: UT*Prescaler period"]
    UT_4 = 4,
    #[doc = "5: UT*Prescaler period"]
    UT_5 = 5,
    #[doc = "6: UT*Prescaler period"]
    UT_6 = 6,
    #[doc = "7: UT*Prescaler period"]
    UT_7 = 7,
    #[doc = "8: UT*Prescaler period"]
    UT_8 = 8,
    #[doc = "9: UT*Prescaler period"]
    UT_9 = 9,
}
impl From<UT_A> for u8 {
    #[inline(always)]
    fn from(variant: UT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UT`"]
pub type UT_R = crate::R<u8, UT_A>;
impl UT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UT_A::UT_0),
            1 => Val(UT_A::UT_1),
            2 => Val(UT_A::UT_2),
            3 => Val(UT_A::UT_3),
            4 => Val(UT_A::UT_4),
            5 => Val(UT_A::UT_5),
            6 => Val(UT_A::UT_6),
            7 => Val(UT_A::UT_7),
            8 => Val(UT_A::UT_8),
            9 => Val(UT_A::UT_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UT_0`"]
    #[inline(always)]
    pub fn is_ut_0(&self) -> bool {
        *self == UT_A::UT_0
    }
    #[doc = "Checks if the value of the field is `UT_1`"]
    #[inline(always)]
    pub fn is_ut_1(&self) -> bool {
        *self == UT_A::UT_1
    }
    #[doc = "Checks if the value of the field is `UT_2`"]
    #[inline(always)]
    pub fn is_ut_2(&self) -> bool {
        *self == UT_A::UT_2
    }
    #[doc = "Checks if the value of the field is `UT_3`"]
    #[inline(always)]
    pub fn is_ut_3(&self) -> bool {
        *self == UT_A::UT_3
    }
    #[doc = "Checks if the value of the field is `UT_4`"]
    #[inline(always)]
    pub fn is_ut_4(&self) -> bool {
        *self == UT_A::UT_4
    }
    #[doc = "Checks if the value of the field is `UT_5`"]
    #[inline(always)]
    pub fn is_ut_5(&self) -> bool {
        *self == UT_A::UT_5
    }
    #[doc = "Checks if the value of the field is `UT_6`"]
    #[inline(always)]
    pub fn is_ut_6(&self) -> bool {
        *self == UT_A::UT_6
    }
    #[doc = "Checks if the value of the field is `UT_7`"]
    #[inline(always)]
    pub fn is_ut_7(&self) -> bool {
        *self == UT_A::UT_7
    }
    #[doc = "Checks if the value of the field is `UT_8`"]
    #[inline(always)]
    pub fn is_ut_8(&self) -> bool {
        *self == UT_A::UT_8
    }
    #[doc = "Checks if the value of the field is `UT_9`"]
    #[inline(always)]
    pub fn is_ut_9(&self) -> bool {
        *self == UT_A::UT_9
    }
}
#[doc = "Write proxy for field `UT`"]
pub struct UT_W<'a> {
    w: &'a mut W,
}
impl<'a> UT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256*Prescaler period"]
    #[inline(always)]
    pub fn ut_0(self) -> &'a mut W {
        self.variant(UT_A::UT_0)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_1(self) -> &'a mut W {
        self.variant(UT_A::UT_1)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_2(self) -> &'a mut W {
        self.variant(UT_A::UT_2)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_3(self) -> &'a mut W {
        self.variant(UT_A::UT_3)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_4(self) -> &'a mut W {
        self.variant(UT_A::UT_4)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_5(self) -> &'a mut W {
        self.variant(UT_A::UT_5)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_6(self) -> &'a mut W {
        self.variant(UT_A::UT_6)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_7(self) -> &'a mut W {
        self.variant(UT_A::UT_7)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_8(self) -> &'a mut W {
        self.variant(UT_A::UT_8)
    }
    #[doc = "UT*Prescaler period"]
    #[inline(always)]
    pub fn ut_9(self) -> &'a mut W {
        self.variant(UT_A::UT_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Refresh enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Refresh burst length"]
    #[inline(always)]
    pub fn rebl(&self) -> REBL_R {
        REBL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Prescaler timer period"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Refresh timer period"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Refresh urgent threshold"]
    #[inline(always)]
    pub fn ut(&self) -> UT_R {
        UT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Refresh enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W {
        REN_W { w: self }
    }
    #[doc = "Bits 1:3 - Refresh burst length"]
    #[inline(always)]
    pub fn rebl(&mut self) -> REBL_W {
        REBL_W { w: self }
    }
    #[doc = "Bits 8:15 - Prescaler timer period"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bits 16:23 - Refresh timer period"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W {
        RT_W { w: self }
    }
    #[doc = "Bits 24:31 - Refresh urgent threshold"]
    #[inline(always)]
    pub fn ut(&mut self) -> UT_W {
        UT_W { w: self }
    }
}
