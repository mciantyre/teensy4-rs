#[doc = "Reader of register INT_STATUS"]
pub type R = crate::R<u32, super::INT_STATUS>;
#[doc = "Read: Error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_ERR_A {
    #[doc = "0: no error"]
    HW_ERR_0 = 0,
    #[doc = "1: error detected."]
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
#[doc = "Read only: Entropy Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENT_VAL_A {
    #[doc = "0: Busy generation entropy. Any value read is invalid."]
    ENT_VAL_0 = 0,
    #[doc = "1: TRNG can be stopped and entropy is valid if read."]
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
#[doc = "Read only: Frequency Count Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRQ_CT_FAIL_A {
    #[doc = "0: No hardware nor self test frequency errors."]
    FRQ_CT_FAIL_0 = 0,
    #[doc = "1: The frequency counter has detected a failure."]
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
impl R {
    #[doc = "Bit 0 - Read: Error status"]
    #[inline(always)]
    pub fn hw_err(&self) -> HW_ERR_R {
        HW_ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read only: Entropy Valid"]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn frq_ct_fail(&self) -> FRQ_CT_FAIL_R {
        FRQ_CT_FAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
