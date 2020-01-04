#[doc = "Reader of register MIBC"]
pub type R = crate::R<u32, super::MIBC>;
#[doc = "Writer for register MIBC"]
pub type W = crate::W<u32, super::MIBC>;
#[doc = "Register MIBC `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::MIBC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "MIB Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_CLEAR_A {
    #[doc = "0: See note above."]
    MIB_CLEAR_0 = 0,
    #[doc = "1: All statistics counters are reset to 0."]
    MIB_CLEAR_1 = 1,
}
impl From<MIB_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIB_CLEAR`"]
pub type MIB_CLEAR_R = crate::R<bool, MIB_CLEAR_A>;
impl MIB_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_CLEAR_A {
        match self.bits {
            false => MIB_CLEAR_A::MIB_CLEAR_0,
            true => MIB_CLEAR_A::MIB_CLEAR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIB_CLEAR_0`"]
    #[inline(always)]
    pub fn is_mib_clear_0(&self) -> bool {
        *self == MIB_CLEAR_A::MIB_CLEAR_0
    }
    #[doc = "Checks if the value of the field is `MIB_CLEAR_1`"]
    #[inline(always)]
    pub fn is_mib_clear_1(&self) -> bool {
        *self == MIB_CLEAR_A::MIB_CLEAR_1
    }
}
#[doc = "Write proxy for field `MIB_CLEAR`"]
pub struct MIB_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MIB_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIB_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "See note above."]
    #[inline(always)]
    pub fn mib_clear_0(self) -> &'a mut W {
        self.variant(MIB_CLEAR_A::MIB_CLEAR_0)
    }
    #[doc = "All statistics counters are reset to 0."]
    #[inline(always)]
    pub fn mib_clear_1(self) -> &'a mut W {
        self.variant(MIB_CLEAR_A::MIB_CLEAR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "MIB Idle\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_IDLE_A {
    #[doc = "0: The MIB block is updating MIB counters."]
    MIB_IDLE_0 = 0,
    #[doc = "1: The MIB block is not currently updating any MIB counters."]
    MIB_IDLE_1 = 1,
}
impl From<MIB_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIB_IDLE`"]
pub type MIB_IDLE_R = crate::R<bool, MIB_IDLE_A>;
impl MIB_IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_IDLE_A {
        match self.bits {
            false => MIB_IDLE_A::MIB_IDLE_0,
            true => MIB_IDLE_A::MIB_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIB_IDLE_0`"]
    #[inline(always)]
    pub fn is_mib_idle_0(&self) -> bool {
        *self == MIB_IDLE_A::MIB_IDLE_0
    }
    #[doc = "Checks if the value of the field is `MIB_IDLE_1`"]
    #[inline(always)]
    pub fn is_mib_idle_1(&self) -> bool {
        *self == MIB_IDLE_A::MIB_IDLE_1
    }
}
#[doc = "Disable MIB Logic\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_DIS_A {
    #[doc = "0: MIB logic is enabled."]
    MIB_DIS_0 = 0,
    #[doc = "1: MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    MIB_DIS_1 = 1,
}
impl From<MIB_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: MIB_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIB_DIS`"]
pub type MIB_DIS_R = crate::R<bool, MIB_DIS_A>;
impl MIB_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIB_DIS_A {
        match self.bits {
            false => MIB_DIS_A::MIB_DIS_0,
            true => MIB_DIS_A::MIB_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIB_DIS_0`"]
    #[inline(always)]
    pub fn is_mib_dis_0(&self) -> bool {
        *self == MIB_DIS_A::MIB_DIS_0
    }
    #[doc = "Checks if the value of the field is `MIB_DIS_1`"]
    #[inline(always)]
    pub fn is_mib_dis_1(&self) -> bool {
        *self == MIB_DIS_A::MIB_DIS_1
    }
}
#[doc = "Write proxy for field `MIB_DIS`"]
pub struct MIB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MIB_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIB_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MIB logic is enabled."]
    #[inline(always)]
    pub fn mib_dis_0(self) -> &'a mut W {
        self.variant(MIB_DIS_A::MIB_DIS_0)
    }
    #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    #[inline(always)]
    pub fn mib_dis_1(self) -> &'a mut W {
        self.variant(MIB_DIS_A::MIB_DIS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    pub fn mib_clear(&self) -> MIB_CLEAR_R {
        MIB_CLEAR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - MIB Idle"]
    #[inline(always)]
    pub fn mib_idle(&self) -> MIB_IDLE_R {
        MIB_IDLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    pub fn mib_dis(&self) -> MIB_DIS_R {
        MIB_DIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    pub fn mib_clear(&mut self) -> MIB_CLEAR_W {
        MIB_CLEAR_W { w: self }
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    pub fn mib_dis(&mut self) -> MIB_DIS_W {
        MIB_DIS_W { w: self }
    }
}
