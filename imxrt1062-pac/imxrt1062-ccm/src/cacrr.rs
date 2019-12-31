#[doc = "Reader of register CACRR"]
pub type R = crate::R<u32, super::CACRR>;
#[doc = "Writer for register CACRR"]
pub type W = crate::W<u32, super::CACRR>;
#[doc = "Register CACRR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CACRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Divider for ARM clock root\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ARM_PODF_A {
    #[doc = "0: divide by 1"]
    ARM_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    ARM_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    ARM_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    ARM_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    ARM_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    ARM_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    ARM_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    ARM_PODF_7 = 7,
}
impl From<ARM_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: ARM_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ARM_PODF`"]
pub type ARM_PODF_R = crate::R<u8, ARM_PODF_A>;
impl ARM_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_PODF_A {
        match self.bits {
            0 => ARM_PODF_A::ARM_PODF_0,
            1 => ARM_PODF_A::ARM_PODF_1,
            2 => ARM_PODF_A::ARM_PODF_2,
            3 => ARM_PODF_A::ARM_PODF_3,
            4 => ARM_PODF_A::ARM_PODF_4,
            5 => ARM_PODF_A::ARM_PODF_5,
            6 => ARM_PODF_A::ARM_PODF_6,
            7 => ARM_PODF_A::ARM_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_0`"]
    #[inline(always)]
    pub fn is_arm_podf_0(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_1`"]
    #[inline(always)]
    pub fn is_arm_podf_1(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_1
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_2`"]
    #[inline(always)]
    pub fn is_arm_podf_2(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_2
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_3`"]
    #[inline(always)]
    pub fn is_arm_podf_3(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_3
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_4`"]
    #[inline(always)]
    pub fn is_arm_podf_4(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_4
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_5`"]
    #[inline(always)]
    pub fn is_arm_podf_5(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_5
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_6`"]
    #[inline(always)]
    pub fn is_arm_podf_6(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_6
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_7`"]
    #[inline(always)]
    pub fn is_arm_podf_7(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_7
    }
}
#[doc = "Write proxy for field `ARM_PODF`"]
pub struct ARM_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARM_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARM_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn arm_podf_0(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn arm_podf_1(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn arm_podf_2(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn arm_podf_3(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn arm_podf_4(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn arm_podf_5(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn arm_podf_6(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn arm_podf_7(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Divider for ARM clock root"]
    #[inline(always)]
    pub fn arm_podf(&self) -> ARM_PODF_R {
        ARM_PODF_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Divider for ARM clock root"]
    #[inline(always)]
    pub fn arm_podf(&mut self) -> ARM_PODF_W {
        ARM_PODF_W { w: self }
    }
}
