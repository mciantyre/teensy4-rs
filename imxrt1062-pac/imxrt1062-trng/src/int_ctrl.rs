#[doc = "Reader of register INT_CTRL"]
pub type R = crate::R<u32, super::INT_CTRL>;
#[doc = "Writer for register INT_CTRL"]
pub type W = crate::W<u32, super::INT_CTRL>;
#[doc = "Register INT_CTRL `reset()`'s with value 0x07"]
impl crate::ResetValue for super::INT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS register has been asserted.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_ERR_A {
    #[doc = "0: Corresponding bit of INT_STATUS register cleared."]
    HW_ERR_0 = 0,
    #[doc = "1: Corresponding bit of INT_STATUS register active."]
    HW_ERR_1 = 1,
}
impl From<HW_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: HW_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HW_ERR`"]
pub type HW_ERR_R = crate::R<bool, HW_ERR_A>;
impl HW_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HW_ERR_A {
        match self.bits {
            false => HW_ERR_A::HW_ERR_0,
            true => HW_ERR_A::HW_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `HW_ERR_0`"]
    #[inline(always)]
    pub fn is_hw_err_0(&self) -> bool {
        *self == HW_ERR_A::HW_ERR_0
    }
    #[doc = "Checks if the value of the field is `HW_ERR_1`"]
    #[inline(always)]
    pub fn is_hw_err_1(&self) -> bool {
        *self == HW_ERR_A::HW_ERR_1
    }
}
#[doc = "Write proxy for field `HW_ERR`"]
pub struct HW_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding bit of INT_STATUS register cleared."]
    #[inline(always)]
    pub fn hw_err_0(self) -> &'a mut W {
        self.variant(HW_ERR_A::HW_ERR_0)
    }
    #[doc = "Corresponding bit of INT_STATUS register active."]
    #[inline(always)]
    pub fn hw_err_1(self) -> &'a mut W {
        self.variant(HW_ERR_A::HW_ERR_1)
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
#[doc = "Same behavior as bit 0 of this register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENT_VAL_A {
    #[doc = "0: Same behavior as bit 0 of this register."]
    ENT_VAL_0 = 0,
    #[doc = "1: Same behavior as bit 0 of this register."]
    ENT_VAL_1 = 1,
}
impl From<ENT_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: ENT_VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENT_VAL`"]
pub type ENT_VAL_R = crate::R<bool, ENT_VAL_A>;
impl ENT_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENT_VAL_A {
        match self.bits {
            false => ENT_VAL_A::ENT_VAL_0,
            true => ENT_VAL_A::ENT_VAL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENT_VAL_0`"]
    #[inline(always)]
    pub fn is_ent_val_0(&self) -> bool {
        *self == ENT_VAL_A::ENT_VAL_0
    }
    #[doc = "Checks if the value of the field is `ENT_VAL_1`"]
    #[inline(always)]
    pub fn is_ent_val_1(&self) -> bool {
        *self == ENT_VAL_A::ENT_VAL_1
    }
}
#[doc = "Write proxy for field `ENT_VAL`"]
pub struct ENT_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENT_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENT_VAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn ent_val_0(self) -> &'a mut W {
        self.variant(ENT_VAL_A::ENT_VAL_0)
    }
    #[doc = "Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn ent_val_1(self) -> &'a mut W {
        self.variant(ENT_VAL_A::ENT_VAL_1)
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
#[doc = "Same behavior as bit 0 of this register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRQ_CT_FAIL_A {
    #[doc = "0: Same behavior as bit 0 of this register."]
    FRQ_CT_FAIL_0 = 0,
    #[doc = "1: Same behavior as bit 0 of this register."]
    FRQ_CT_FAIL_1 = 1,
}
impl From<FRQ_CT_FAIL_A> for bool {
    #[inline(always)]
    fn from(variant: FRQ_CT_FAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRQ_CT_FAIL`"]
pub type FRQ_CT_FAIL_R = crate::R<bool, FRQ_CT_FAIL_A>;
impl FRQ_CT_FAIL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRQ_CT_FAIL_A {
        match self.bits {
            false => FRQ_CT_FAIL_A::FRQ_CT_FAIL_0,
            true => FRQ_CT_FAIL_A::FRQ_CT_FAIL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRQ_CT_FAIL_0`"]
    #[inline(always)]
    pub fn is_frq_ct_fail_0(&self) -> bool {
        *self == FRQ_CT_FAIL_A::FRQ_CT_FAIL_0
    }
    #[doc = "Checks if the value of the field is `FRQ_CT_FAIL_1`"]
    #[inline(always)]
    pub fn is_frq_ct_fail_1(&self) -> bool {
        *self == FRQ_CT_FAIL_A::FRQ_CT_FAIL_1
    }
}
#[doc = "Write proxy for field `FRQ_CT_FAIL`"]
pub struct FRQ_CT_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRQ_CT_FAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRQ_CT_FAIL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn frq_ct_fail_0(self) -> &'a mut W {
        self.variant(FRQ_CT_FAIL_A::FRQ_CT_FAIL_0)
    }
    #[doc = "Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn frq_ct_fail_1(self) -> &'a mut W {
        self.variant(FRQ_CT_FAIL_A::FRQ_CT_FAIL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bit position that can be cleared if corresponding bit of INT_STATUS register has been asserted."]
    #[inline(always)]
    pub fn hw_err(&self) -> HW_ERR_R {
        HW_ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn frq_ct_fail(&self) -> FRQ_CT_FAIL_R {
        FRQ_CT_FAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit position that can be cleared if corresponding bit of INT_STATUS register has been asserted."]
    #[inline(always)]
    pub fn hw_err(&mut self) -> HW_ERR_W {
        HW_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn ent_val(&mut self) -> ENT_VAL_W {
        ENT_VAL_W { w: self }
    }
    #[doc = "Bit 2 - Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn frq_ct_fail(&mut self) -> FRQ_CT_FAIL_W {
        FRQ_CT_FAIL_W { w: self }
    }
}
