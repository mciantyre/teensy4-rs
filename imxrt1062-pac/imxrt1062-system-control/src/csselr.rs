#[doc = "Reader of register CSSELR"]
pub type R = crate::R<u32, super::CSSELR>;
#[doc = "Writer for register CSSELR"]
pub type W = crate::W<u32, super::CSSELR>;
#[doc = "Register CSSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Instruction not data bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IND_A {
    #[doc = "0: Data or unified cache."]
    IND_0 = 0,
    #[doc = "1: Instruction cache."]
    IND_1 = 1,
}
impl From<IND_A> for bool {
    #[inline(always)]
    fn from(variant: IND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IND`"]
pub type IND_R = crate::R<bool, IND_A>;
impl IND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IND_A {
        match self.bits {
            false => IND_A::IND_0,
            true => IND_A::IND_1,
        }
    }
    #[doc = "Checks if the value of the field is `IND_0`"]
    #[inline(always)]
    pub fn is_ind_0(&self) -> bool {
        *self == IND_A::IND_0
    }
    #[doc = "Checks if the value of the field is `IND_1`"]
    #[inline(always)]
    pub fn is_ind_1(&self) -> bool {
        *self == IND_A::IND_1
    }
}
#[doc = "Write proxy for field `IND`"]
pub struct IND_W<'a> {
    w: &'a mut W,
}
impl<'a> IND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data or unified cache."]
    #[inline(always)]
    pub fn ind_0(self) -> &'a mut W {
        self.variant(IND_A::IND_0)
    }
    #[doc = "Instruction cache."]
    #[inline(always)]
    pub fn ind_1(self) -> &'a mut W {
        self.variant(IND_A::IND_1)
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
#[doc = "Cache level of required cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEVEL_A {
    #[doc = "0: Level 1 cache."]
    LEVEL_0 = 0,
    #[doc = "1: Level 2 cache."]
    LEVEL_1 = 1,
    #[doc = "2: Level 3 cache."]
    LEVEL_2 = 2,
    #[doc = "3: Level 4 cache."]
    LEVEL_3 = 3,
    #[doc = "4: Level 5 cache."]
    LEVEL_4 = 4,
    #[doc = "5: Level 6 cache."]
    LEVEL_5 = 5,
    #[doc = "6: Level 7 cache."]
    LEVEL_6 = 6,
}
impl From<LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEVEL`"]
pub type LEVEL_R = crate::R<u8, LEVEL_A>;
impl LEVEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LEVEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LEVEL_A::LEVEL_0),
            1 => Val(LEVEL_A::LEVEL_1),
            2 => Val(LEVEL_A::LEVEL_2),
            3 => Val(LEVEL_A::LEVEL_3),
            4 => Val(LEVEL_A::LEVEL_4),
            5 => Val(LEVEL_A::LEVEL_5),
            6 => Val(LEVEL_A::LEVEL_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0`"]
    #[inline(always)]
    pub fn is_level_0(&self) -> bool {
        *self == LEVEL_A::LEVEL_0
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == LEVEL_A::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == LEVEL_A::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == LEVEL_A::LEVEL_3
    }
    #[doc = "Checks if the value of the field is `LEVEL_4`"]
    #[inline(always)]
    pub fn is_level_4(&self) -> bool {
        *self == LEVEL_A::LEVEL_4
    }
    #[doc = "Checks if the value of the field is `LEVEL_5`"]
    #[inline(always)]
    pub fn is_level_5(&self) -> bool {
        *self == LEVEL_A::LEVEL_5
    }
    #[doc = "Checks if the value of the field is `LEVEL_6`"]
    #[inline(always)]
    pub fn is_level_6(&self) -> bool {
        *self == LEVEL_A::LEVEL_6
    }
}
#[doc = "Write proxy for field `LEVEL`"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEVEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Level 1 cache."]
    #[inline(always)]
    pub fn level_0(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_0)
    }
    #[doc = "Level 2 cache."]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_1)
    }
    #[doc = "Level 3 cache."]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_2)
    }
    #[doc = "Level 4 cache."]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_3)
    }
    #[doc = "Level 5 cache."]
    #[inline(always)]
    pub fn level_4(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_4)
    }
    #[doc = "Level 6 cache."]
    #[inline(always)]
    pub fn level_5(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_5)
    }
    #[doc = "Level 7 cache."]
    #[inline(always)]
    pub fn level_6(self) -> &'a mut W {
        self.variant(LEVEL_A::LEVEL_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Instruction not data bit"]
    #[inline(always)]
    pub fn ind(&self) -> IND_R {
        IND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Cache level of required cache"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction not data bit"]
    #[inline(always)]
    pub fn ind(&mut self) -> IND_W {
        IND_W { w: self }
    }
    #[doc = "Bits 1:3 - Cache level of required cache"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
}
