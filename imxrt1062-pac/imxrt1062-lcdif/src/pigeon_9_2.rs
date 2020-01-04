#[doc = "Reader of register PIGEON_9_2"]
pub type R = crate::R<u32, super::PIGEON_9_2>;
#[doc = "Writer for register PIGEON_9_2"]
pub type W = crate::W<u32, super::PIGEON_9_2>;
#[doc = "Register PIGEON_9_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIGEON_9_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Logic operation with another signal: DIS/AND/OR/COND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIG_LOGIC_A {
    #[doc = "0: No logic operation"]
    DIS = 0,
    #[doc = "1: sigout = sig_another AND this_sig"]
    AND = 1,
    #[doc = "2: sigout = sig_another OR this_sig"]
    OR = 2,
    #[doc = "3: mask = sig_another AND other_masks"]
    MASK = 3,
}
impl From<SIG_LOGIC_A> for u8 {
    #[inline(always)]
    fn from(variant: SIG_LOGIC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIG_LOGIC`"]
pub type SIG_LOGIC_R = crate::R<u8, SIG_LOGIC_A>;
impl SIG_LOGIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIG_LOGIC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIG_LOGIC_A::DIS),
            1 => Val(SIG_LOGIC_A::AND),
            2 => Val(SIG_LOGIC_A::OR),
            3 => Val(SIG_LOGIC_A::MASK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SIG_LOGIC_A::DIS
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == SIG_LOGIC_A::AND
    }
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == SIG_LOGIC_A::OR
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == SIG_LOGIC_A::MASK
    }
}
#[doc = "Write proxy for field `SIG_LOGIC`"]
pub struct SIG_LOGIC_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_LOGIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIG_LOGIC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No logic operation"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SIG_LOGIC_A::DIS)
    }
    #[doc = "sigout = sig_another AND this_sig"]
    #[inline(always)]
    pub fn and(self) -> &'a mut W {
        self.variant(SIG_LOGIC_A::AND)
    }
    #[doc = "sigout = sig_another OR this_sig"]
    #[inline(always)]
    pub fn or(self) -> &'a mut W {
        self.variant(SIG_LOGIC_A::OR)
    }
    #[doc = "mask = sig_another AND other_masks"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(SIG_LOGIC_A::MASK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Select another signal for logic operation or as mask or counter tick event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIG_ANOTHER_A {
    #[doc = "0: Keep active until mask off"]
    CLEAR_USING_MASK = 0,
}
impl From<SIG_ANOTHER_A> for u8 {
    #[inline(always)]
    fn from(variant: SIG_ANOTHER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIG_ANOTHER`"]
pub type SIG_ANOTHER_R = crate::R<u8, SIG_ANOTHER_A>;
impl SIG_ANOTHER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIG_ANOTHER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIG_ANOTHER_A::CLEAR_USING_MASK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_USING_MASK`"]
    #[inline(always)]
    pub fn is_clear_using_mask(&self) -> bool {
        *self == SIG_ANOTHER_A::CLEAR_USING_MASK
    }
}
#[doc = "Write proxy for field `SIG_ANOTHER`"]
pub struct SIG_ANOTHER_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_ANOTHER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIG_ANOTHER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Keep active until mask off"]
    #[inline(always)]
    pub fn clear_using_mask(self) -> &'a mut W {
        self.variant(SIG_ANOTHER_A::CLEAR_USING_MASK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub fn sig_logic(&self) -> SIG_LOGIC_R {
        SIG_LOGIC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub fn sig_another(&self) -> SIG_ANOTHER_R {
        SIG_ANOTHER_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Logic operation with another signal: DIS/AND/OR/COND"]
    #[inline(always)]
    pub fn sig_logic(&mut self) -> SIG_LOGIC_W {
        SIG_LOGIC_W { w: self }
    }
    #[doc = "Bits 4:8 - Select another signal for logic operation or as mask or counter tick event"]
    #[inline(always)]
    pub fn sig_another(&mut self) -> SIG_ANOTHER_W {
        SIG_ANOTHER_W { w: self }
    }
}
