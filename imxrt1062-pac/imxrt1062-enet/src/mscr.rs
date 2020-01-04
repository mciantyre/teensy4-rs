#[doc = "Reader of register MSCR"]
pub type R = crate::R<u32, super::MSCR>;
#[doc = "Writer for register MSCR"]
pub type W = crate::W<u32, super::MSCR>;
#[doc = "Register MSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MII_SPEED`"]
pub type MII_SPEED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MII_SPEED`"]
pub struct MII_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | (((value as u32) & 0x3f) << 1);
        self.w
    }
}
#[doc = "Disable Preamble\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_PRE_A {
    #[doc = "0: Preamble enabled."]
    DIS_PRE_0 = 0,
    #[doc = "1: Preamble (32 ones) is not prepended to the MII management frame."]
    DIS_PRE_1 = 1,
}
impl From<DIS_PRE_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_PRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIS_PRE`"]
pub type DIS_PRE_R = crate::R<bool, DIS_PRE_A>;
impl DIS_PRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_PRE_A {
        match self.bits {
            false => DIS_PRE_A::DIS_PRE_0,
            true => DIS_PRE_A::DIS_PRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_PRE_0`"]
    #[inline(always)]
    pub fn is_dis_pre_0(&self) -> bool {
        *self == DIS_PRE_A::DIS_PRE_0
    }
    #[doc = "Checks if the value of the field is `DIS_PRE_1`"]
    #[inline(always)]
    pub fn is_dis_pre_1(&self) -> bool {
        *self == DIS_PRE_A::DIS_PRE_1
    }
}
#[doc = "Write proxy for field `DIS_PRE`"]
pub struct DIS_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_PRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Preamble enabled."]
    #[inline(always)]
    pub fn dis_pre_0(self) -> &'a mut W {
        self.variant(DIS_PRE_A::DIS_PRE_0)
    }
    #[doc = "Preamble (32 ones) is not prepended to the MII management frame."]
    #[inline(always)]
    pub fn dis_pre_1(self) -> &'a mut W {
        self.variant(DIS_PRE_A::DIS_PRE_1)
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
#[doc = "Hold time On MDIO Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HOLDTIME_A {
    #[doc = "0: 1 internal module clock cycle"]
    HOLDTIME_0 = 0,
    #[doc = "1: 2 internal module clock cycles"]
    HOLDTIME_1 = 1,
    #[doc = "2: 3 internal module clock cycles"]
    HOLDTIME_2 = 2,
    #[doc = "7: 8 internal module clock cycles"]
    HOLDTIME_7 = 7,
}
impl From<HOLDTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: HOLDTIME_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HOLDTIME`"]
pub type HOLDTIME_R = crate::R<u8, HOLDTIME_A>;
impl HOLDTIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HOLDTIME_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HOLDTIME_A::HOLDTIME_0),
            1 => Val(HOLDTIME_A::HOLDTIME_1),
            2 => Val(HOLDTIME_A::HOLDTIME_2),
            7 => Val(HOLDTIME_A::HOLDTIME_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HOLDTIME_0`"]
    #[inline(always)]
    pub fn is_holdtime_0(&self) -> bool {
        *self == HOLDTIME_A::HOLDTIME_0
    }
    #[doc = "Checks if the value of the field is `HOLDTIME_1`"]
    #[inline(always)]
    pub fn is_holdtime_1(&self) -> bool {
        *self == HOLDTIME_A::HOLDTIME_1
    }
    #[doc = "Checks if the value of the field is `HOLDTIME_2`"]
    #[inline(always)]
    pub fn is_holdtime_2(&self) -> bool {
        *self == HOLDTIME_A::HOLDTIME_2
    }
    #[doc = "Checks if the value of the field is `HOLDTIME_7`"]
    #[inline(always)]
    pub fn is_holdtime_7(&self) -> bool {
        *self == HOLDTIME_A::HOLDTIME_7
    }
}
#[doc = "Write proxy for field `HOLDTIME`"]
pub struct HOLDTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLDTIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOLDTIME_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 internal module clock cycle"]
    #[inline(always)]
    pub fn holdtime_0(self) -> &'a mut W {
        self.variant(HOLDTIME_A::HOLDTIME_0)
    }
    #[doc = "2 internal module clock cycles"]
    #[inline(always)]
    pub fn holdtime_1(self) -> &'a mut W {
        self.variant(HOLDTIME_A::HOLDTIME_1)
    }
    #[doc = "3 internal module clock cycles"]
    #[inline(always)]
    pub fn holdtime_2(self) -> &'a mut W {
        self.variant(HOLDTIME_A::HOLDTIME_2)
    }
    #[doc = "8 internal module clock cycles"]
    #[inline(always)]
    pub fn holdtime_7(self) -> &'a mut W {
        self.variant(HOLDTIME_A::HOLDTIME_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:6 - MII Speed"]
    #[inline(always)]
    pub fn mii_speed(&self) -> MII_SPEED_R {
        MII_SPEED_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Disable Preamble"]
    #[inline(always)]
    pub fn dis_pre(&self) -> DIS_PRE_R {
        DIS_PRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Hold time On MDIO Output"]
    #[inline(always)]
    pub fn holdtime(&self) -> HOLDTIME_R {
        HOLDTIME_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 1:6 - MII Speed"]
    #[inline(always)]
    pub fn mii_speed(&mut self) -> MII_SPEED_W {
        MII_SPEED_W { w: self }
    }
    #[doc = "Bit 7 - Disable Preamble"]
    #[inline(always)]
    pub fn dis_pre(&mut self) -> DIS_PRE_W {
        DIS_PRE_W { w: self }
    }
    #[doc = "Bits 8:10 - Hold time On MDIO Output"]
    #[inline(always)]
    pub fn holdtime(&mut self) -> HOLDTIME_W {
        HOLDTIME_W { w: self }
    }
}
