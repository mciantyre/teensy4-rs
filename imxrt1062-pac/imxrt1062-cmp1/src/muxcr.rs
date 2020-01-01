#[doc = "Reader of register MUXCR"]
pub type R = crate::R<u8, super::MUXCR>;
#[doc = "Writer for register MUXCR"]
pub type W = crate::W<u8, super::MUXCR>;
#[doc = "Register MUXCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MUXCR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Minus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: IN0"]
    MSEL_0 = 0,
    #[doc = "1: IN1"]
    MSEL_1 = 1,
    #[doc = "2: IN2"]
    MSEL_2 = 2,
    #[doc = "3: IN3"]
    MSEL_3 = 3,
    #[doc = "4: IN4"]
    MSEL_4 = 4,
    #[doc = "5: IN5"]
    MSEL_5 = 5,
    #[doc = "6: IN6"]
    MSEL_6 = 6,
    #[doc = "7: IN7"]
    MSEL_7 = 7,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, MSEL_A>;
impl MSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::MSEL_0,
            1 => MSEL_A::MSEL_1,
            2 => MSEL_A::MSEL_2,
            3 => MSEL_A::MSEL_3,
            4 => MSEL_A::MSEL_4,
            5 => MSEL_A::MSEL_5,
            6 => MSEL_A::MSEL_6,
            7 => MSEL_A::MSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        *self == MSEL_A::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        *self == MSEL_A::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        *self == MSEL_A::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        *self == MSEL_A::MSEL_3
    }
    #[doc = "Checks if the value of the field is `MSEL_4`"]
    #[inline(always)]
    pub fn is_msel_4(&self) -> bool {
        *self == MSEL_A::MSEL_4
    }
    #[doc = "Checks if the value of the field is `MSEL_5`"]
    #[inline(always)]
    pub fn is_msel_5(&self) -> bool {
        *self == MSEL_A::MSEL_5
    }
    #[doc = "Checks if the value of the field is `MSEL_6`"]
    #[inline(always)]
    pub fn is_msel_6(&self) -> bool {
        *self == MSEL_A::MSEL_6
    }
    #[doc = "Checks if the value of the field is `MSEL_7`"]
    #[inline(always)]
    pub fn is_msel_7(&self) -> bool {
        *self == MSEL_A::MSEL_7
    }
}
#[doc = "Write proxy for field `MSEL`"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_0)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_1)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_2)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_3)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn msel_4(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_4)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn msel_5(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_5)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn msel_6(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_6)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn msel_7(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Plus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: IN0"]
    PSEL_0 = 0,
    #[doc = "1: IN1"]
    PSEL_1 = 1,
    #[doc = "2: IN2"]
    PSEL_2 = 2,
    #[doc = "3: IN3"]
    PSEL_3 = 3,
    #[doc = "4: IN4"]
    PSEL_4 = 4,
    #[doc = "5: IN5"]
    PSEL_5 = 5,
    #[doc = "6: IN6"]
    PSEL_6 = 6,
    #[doc = "7: IN7"]
    PSEL_7 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, PSEL_A>;
impl PSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::PSEL_0,
            1 => PSEL_A::PSEL_1,
            2 => PSEL_A::PSEL_2,
            3 => PSEL_A::PSEL_3,
            4 => PSEL_A::PSEL_4,
            5 => PSEL_A::PSEL_5,
            6 => PSEL_A::PSEL_6,
            7 => PSEL_A::PSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSEL_0`"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        *self == PSEL_A::PSEL_0
    }
    #[doc = "Checks if the value of the field is `PSEL_1`"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        *self == PSEL_A::PSEL_1
    }
    #[doc = "Checks if the value of the field is `PSEL_2`"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        *self == PSEL_A::PSEL_2
    }
    #[doc = "Checks if the value of the field is `PSEL_3`"]
    #[inline(always)]
    pub fn is_psel_3(&self) -> bool {
        *self == PSEL_A::PSEL_3
    }
    #[doc = "Checks if the value of the field is `PSEL_4`"]
    #[inline(always)]
    pub fn is_psel_4(&self) -> bool {
        *self == PSEL_A::PSEL_4
    }
    #[doc = "Checks if the value of the field is `PSEL_5`"]
    #[inline(always)]
    pub fn is_psel_5(&self) -> bool {
        *self == PSEL_A::PSEL_5
    }
    #[doc = "Checks if the value of the field is `PSEL_6`"]
    #[inline(always)]
    pub fn is_psel_6(&self) -> bool {
        *self == PSEL_A::PSEL_6
    }
    #[doc = "Checks if the value of the field is `PSEL_7`"]
    #[inline(always)]
    pub fn is_psel_7(&self) -> bool {
        *self == PSEL_A::PSEL_7
    }
}
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_0)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_1)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_2)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn psel_3(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_3)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn psel_4(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_4)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn psel_5(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_5)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn psel_6(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_6)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn psel_7(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u8) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
}
