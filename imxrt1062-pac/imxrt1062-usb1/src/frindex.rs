#[doc = "Reader of register FRINDEX"]
pub type R = crate::R<u32, super::FRINDEX>;
#[doc = "Writer for register FRINDEX"]
pub type W = crate::W<u32, super::FRINDEX>;
#[doc = "Register FRINDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::FRINDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Frame Index\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FRINDEX_A {
    #[doc = "0: (1024) 12"]
    FRINDEX_0 = 0,
    #[doc = "1: (512) 11"]
    FRINDEX_1 = 1,
    #[doc = "2: (256) 10"]
    FRINDEX_2 = 2,
    #[doc = "3: (128) 9"]
    FRINDEX_3 = 3,
    #[doc = "4: (64) 8"]
    FRINDEX_4 = 4,
    #[doc = "5: (32) 7"]
    FRINDEX_5 = 5,
    #[doc = "6: (16) 6"]
    FRINDEX_6 = 6,
    #[doc = "7: (8) 5"]
    FRINDEX_7 = 7,
}
impl From<FRINDEX_A> for u16 {
    #[inline(always)]
    fn from(variant: FRINDEX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRINDEX`"]
pub type FRINDEX_R = crate::R<u16, FRINDEX_A>;
impl FRINDEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FRINDEX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FRINDEX_A::FRINDEX_0),
            1 => Val(FRINDEX_A::FRINDEX_1),
            2 => Val(FRINDEX_A::FRINDEX_2),
            3 => Val(FRINDEX_A::FRINDEX_3),
            4 => Val(FRINDEX_A::FRINDEX_4),
            5 => Val(FRINDEX_A::FRINDEX_5),
            6 => Val(FRINDEX_A::FRINDEX_6),
            7 => Val(FRINDEX_A::FRINDEX_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRINDEX_0`"]
    #[inline(always)]
    pub fn is_frindex_0(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_0
    }
    #[doc = "Checks if the value of the field is `FRINDEX_1`"]
    #[inline(always)]
    pub fn is_frindex_1(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_1
    }
    #[doc = "Checks if the value of the field is `FRINDEX_2`"]
    #[inline(always)]
    pub fn is_frindex_2(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_2
    }
    #[doc = "Checks if the value of the field is `FRINDEX_3`"]
    #[inline(always)]
    pub fn is_frindex_3(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_3
    }
    #[doc = "Checks if the value of the field is `FRINDEX_4`"]
    #[inline(always)]
    pub fn is_frindex_4(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_4
    }
    #[doc = "Checks if the value of the field is `FRINDEX_5`"]
    #[inline(always)]
    pub fn is_frindex_5(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_5
    }
    #[doc = "Checks if the value of the field is `FRINDEX_6`"]
    #[inline(always)]
    pub fn is_frindex_6(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_6
    }
    #[doc = "Checks if the value of the field is `FRINDEX_7`"]
    #[inline(always)]
    pub fn is_frindex_7(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_7
    }
}
#[doc = "Write proxy for field `FRINDEX`"]
pub struct FRINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRINDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRINDEX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "(1024) 12"]
    #[inline(always)]
    pub fn frindex_0(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_0)
    }
    #[doc = "(512) 11"]
    #[inline(always)]
    pub fn frindex_1(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_1)
    }
    #[doc = "(256) 10"]
    #[inline(always)]
    pub fn frindex_2(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_2)
    }
    #[doc = "(128) 9"]
    #[inline(always)]
    pub fn frindex_3(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_3)
    }
    #[doc = "(64) 8"]
    #[inline(always)]
    pub fn frindex_4(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_4)
    }
    #[doc = "(32) 7"]
    #[inline(always)]
    pub fn frindex_5(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_5)
    }
    #[doc = "(16) 6"]
    #[inline(always)]
    pub fn frindex_6(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_6)
    }
    #[doc = "(8) 5"]
    #[inline(always)]
    pub fn frindex_7(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frindex(&self) -> FRINDEX_R {
        FRINDEX_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frindex(&mut self) -> FRINDEX_W {
        FRINDEX_W { w: self }
    }
}
