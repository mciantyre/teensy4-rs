#[doc = "Reader of register TCM_CTRL"]
pub type R = crate::R<u32, super::TCM_CTRL>;
#[doc = "Writer for register TCM_CTRL"]
pub type W = crate::W<u32, super::TCM_CTRL>;
#[doc = "Register TCM_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TCM_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TCM Write Wait Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCM_WWAIT_EN_A {
    #[doc = "0: TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
    TCM_WWAIT_EN_0 = 0,
    #[doc = "1: TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    TCM_WWAIT_EN_1 = 1,
}
impl From<TCM_WWAIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TCM_WWAIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCM_WWAIT_EN`"]
pub type TCM_WWAIT_EN_R = crate::R<bool, TCM_WWAIT_EN_A>;
impl TCM_WWAIT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCM_WWAIT_EN_A {
        match self.bits {
            false => TCM_WWAIT_EN_A::TCM_WWAIT_EN_0,
            true => TCM_WWAIT_EN_A::TCM_WWAIT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCM_WWAIT_EN_0`"]
    #[inline(always)]
    pub fn is_tcm_wwait_en_0(&self) -> bool {
        *self == TCM_WWAIT_EN_A::TCM_WWAIT_EN_0
    }
    #[doc = "Checks if the value of the field is `TCM_WWAIT_EN_1`"]
    #[inline(always)]
    pub fn is_tcm_wwait_en_1(&self) -> bool {
        *self == TCM_WWAIT_EN_A::TCM_WWAIT_EN_1
    }
}
#[doc = "Write proxy for field `TCM_WWAIT_EN`"]
pub struct TCM_WWAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCM_WWAIT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCM_WWAIT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
    #[inline(always)]
    pub fn tcm_wwait_en_0(self) -> &'a mut W {
        self.variant(TCM_WWAIT_EN_A::TCM_WWAIT_EN_0)
    }
    #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    #[inline(always)]
    pub fn tcm_wwait_en_1(self) -> &'a mut W {
        self.variant(TCM_WWAIT_EN_A::TCM_WWAIT_EN_1)
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
#[doc = "TCM Read Wait Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCM_RWAIT_EN_A {
    #[doc = "0: TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    TCM_RWAIT_EN_0 = 0,
    #[doc = "1: TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    TCM_RWAIT_EN_1 = 1,
}
impl From<TCM_RWAIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TCM_RWAIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCM_RWAIT_EN`"]
pub type TCM_RWAIT_EN_R = crate::R<bool, TCM_RWAIT_EN_A>;
impl TCM_RWAIT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCM_RWAIT_EN_A {
        match self.bits {
            false => TCM_RWAIT_EN_A::TCM_RWAIT_EN_0,
            true => TCM_RWAIT_EN_A::TCM_RWAIT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCM_RWAIT_EN_0`"]
    #[inline(always)]
    pub fn is_tcm_rwait_en_0(&self) -> bool {
        *self == TCM_RWAIT_EN_A::TCM_RWAIT_EN_0
    }
    #[doc = "Checks if the value of the field is `TCM_RWAIT_EN_1`"]
    #[inline(always)]
    pub fn is_tcm_rwait_en_1(&self) -> bool {
        *self == TCM_RWAIT_EN_A::TCM_RWAIT_EN_1
    }
}
#[doc = "Write proxy for field `TCM_RWAIT_EN`"]
pub struct TCM_RWAIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCM_RWAIT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCM_RWAIT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    #[inline(always)]
    pub fn tcm_rwait_en_0(self) -> &'a mut W {
        self.variant(TCM_RWAIT_EN_A::TCM_RWAIT_EN_0)
    }
    #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    #[inline(always)]
    pub fn tcm_rwait_en_1(self) -> &'a mut W {
        self.variant(TCM_RWAIT_EN_A::TCM_RWAIT_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FORCE_CLK_ON`"]
pub type FORCE_CLK_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_CLK_ON`"]
pub struct FORCE_CLK_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_CLK_ON_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TCM Write Wait Mode Enable"]
    #[inline(always)]
    pub fn tcm_wwait_en(&self) -> TCM_WWAIT_EN_R {
        TCM_WWAIT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TCM Read Wait Mode Enable"]
    #[inline(always)]
    pub fn tcm_rwait_en(&self) -> TCM_RWAIT_EN_R {
        TCM_RWAIT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force RAM Clock Always On"]
    #[inline(always)]
    pub fn force_clk_on(&self) -> FORCE_CLK_ON_R {
        FORCE_CLK_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCM Write Wait Mode Enable"]
    #[inline(always)]
    pub fn tcm_wwait_en(&mut self) -> TCM_WWAIT_EN_W {
        TCM_WWAIT_EN_W { w: self }
    }
    #[doc = "Bit 1 - TCM Read Wait Mode Enable"]
    #[inline(always)]
    pub fn tcm_rwait_en(&mut self) -> TCM_RWAIT_EN_W {
        TCM_RWAIT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Force RAM Clock Always On"]
    #[inline(always)]
    pub fn force_clk_on(&mut self) -> FORCE_CLK_ON_W {
        FORCE_CLK_ON_W { w: self }
    }
}
