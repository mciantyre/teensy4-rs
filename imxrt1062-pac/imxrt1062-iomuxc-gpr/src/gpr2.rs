#[doc = "Reader of register GPR2"]
pub type R = crate::R<u32, super::GPR2>;
#[doc = "Writer for register GPR2"]
pub type W = crate::W<u32, super::GPR2>;
#[doc = "Register GPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AXBS_L AHBXL master has higher priority.Do not set both DMA and AHBXL to high priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_L_AHBXL_HIGH_PRIORITY_A {
    #[doc = "0: AXBS_L AHBXL master does not have high priority"]
    AXBS_L_AHBXL_HIGH_PRIORITY_0 = 0,
    #[doc = "1: AXBS_P AHBXL master has high priority"]
    AXBS_L_AHBXL_HIGH_PRIORITY_1 = 1,
}
impl From<AXBS_L_AHBXL_HIGH_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_L_AHBXL_HIGH_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_L_AHBXL_HIGH_PRIORITY`"]
pub type AXBS_L_AHBXL_HIGH_PRIORITY_R = crate::R<bool, AXBS_L_AHBXL_HIGH_PRIORITY_A>;
impl AXBS_L_AHBXL_HIGH_PRIORITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_L_AHBXL_HIGH_PRIORITY_A {
        match self.bits {
            false => AXBS_L_AHBXL_HIGH_PRIORITY_A::AXBS_L_AHBXL_HIGH_PRIORITY_0,
            true => AXBS_L_AHBXL_HIGH_PRIORITY_A::AXBS_L_AHBXL_HIGH_PRIORITY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_L_AHBXL_HIGH_PRIORITY_0`"]
    #[inline(always)]
    pub fn is_axbs_l_ahbxl_high_priority_0(&self) -> bool {
        *self == AXBS_L_AHBXL_HIGH_PRIORITY_A::AXBS_L_AHBXL_HIGH_PRIORITY_0
    }
    #[doc = "Checks if the value of the field is `AXBS_L_AHBXL_HIGH_PRIORITY_1`"]
    #[inline(always)]
    pub fn is_axbs_l_ahbxl_high_priority_1(&self) -> bool {
        *self == AXBS_L_AHBXL_HIGH_PRIORITY_A::AXBS_L_AHBXL_HIGH_PRIORITY_1
    }
}
#[doc = "Write proxy for field `AXBS_L_AHBXL_HIGH_PRIORITY`"]
pub struct AXBS_L_AHBXL_HIGH_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> AXBS_L_AHBXL_HIGH_PRIORITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXBS_L_AHBXL_HIGH_PRIORITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXBS_L AHBXL master does not have high priority"]
    #[inline(always)]
    pub fn axbs_l_ahbxl_high_priority_0(self) -> &'a mut W {
        self.variant(AXBS_L_AHBXL_HIGH_PRIORITY_A::AXBS_L_AHBXL_HIGH_PRIORITY_0)
    }
    #[doc = "AXBS_P AHBXL master has high priority"]
    #[inline(always)]
    pub fn axbs_l_ahbxl_high_priority_1(self) -> &'a mut W {
        self.variant(AXBS_L_AHBXL_HIGH_PRIORITY_A::AXBS_L_AHBXL_HIGH_PRIORITY_1)
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
#[doc = "AXBS_L DMA master has higher priority.Do not set both DMA and AHBXL to high priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_L_DMA_HIGH_PRIORITY_A {
    #[doc = "0: AXBS_L DMA master does not have high priority"]
    AXBS_L_DMA_HIGH_PRIORITY_0 = 0,
    #[doc = "1: AXBS_L DMA master has high priority"]
    AXBS_L_DMA_HIGH_PRIORITY_1 = 1,
}
impl From<AXBS_L_DMA_HIGH_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_L_DMA_HIGH_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_L_DMA_HIGH_PRIORITY`"]
pub type AXBS_L_DMA_HIGH_PRIORITY_R = crate::R<bool, AXBS_L_DMA_HIGH_PRIORITY_A>;
impl AXBS_L_DMA_HIGH_PRIORITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_L_DMA_HIGH_PRIORITY_A {
        match self.bits {
            false => AXBS_L_DMA_HIGH_PRIORITY_A::AXBS_L_DMA_HIGH_PRIORITY_0,
            true => AXBS_L_DMA_HIGH_PRIORITY_A::AXBS_L_DMA_HIGH_PRIORITY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_L_DMA_HIGH_PRIORITY_0`"]
    #[inline(always)]
    pub fn is_axbs_l_dma_high_priority_0(&self) -> bool {
        *self == AXBS_L_DMA_HIGH_PRIORITY_A::AXBS_L_DMA_HIGH_PRIORITY_0
    }
    #[doc = "Checks if the value of the field is `AXBS_L_DMA_HIGH_PRIORITY_1`"]
    #[inline(always)]
    pub fn is_axbs_l_dma_high_priority_1(&self) -> bool {
        *self == AXBS_L_DMA_HIGH_PRIORITY_A::AXBS_L_DMA_HIGH_PRIORITY_1
    }
}
#[doc = "Write proxy for field `AXBS_L_DMA_HIGH_PRIORITY`"]
pub struct AXBS_L_DMA_HIGH_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> AXBS_L_DMA_HIGH_PRIORITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXBS_L_DMA_HIGH_PRIORITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXBS_L DMA master does not have high priority"]
    #[inline(always)]
    pub fn axbs_l_dma_high_priority_0(self) -> &'a mut W {
        self.variant(AXBS_L_DMA_HIGH_PRIORITY_A::AXBS_L_DMA_HIGH_PRIORITY_0)
    }
    #[doc = "AXBS_L DMA master has high priority"]
    #[inline(always)]
    pub fn axbs_l_dma_high_priority_1(self) -> &'a mut W {
        self.variant(AXBS_L_DMA_HIGH_PRIORITY_A::AXBS_L_DMA_HIGH_PRIORITY_1)
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
#[doc = "Force Round Robin in AXBS_L\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_L_FORCE_ROUND_ROBIN_A {
    #[doc = "0: AXBS_L masters are not arbitored in round robin, depending on DMA and AHBXL master priority settings."]
    AXBS_L_FORCE_ROUND_ROBIN_0 = 0,
    #[doc = "1: AXBS_L masters are arbitored in round robin"]
    AXBS_L_FORCE_ROUND_ROBIN_1 = 1,
}
impl From<AXBS_L_FORCE_ROUND_ROBIN_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_L_FORCE_ROUND_ROBIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_L_FORCE_ROUND_ROBIN`"]
pub type AXBS_L_FORCE_ROUND_ROBIN_R = crate::R<bool, AXBS_L_FORCE_ROUND_ROBIN_A>;
impl AXBS_L_FORCE_ROUND_ROBIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_L_FORCE_ROUND_ROBIN_A {
        match self.bits {
            false => AXBS_L_FORCE_ROUND_ROBIN_A::AXBS_L_FORCE_ROUND_ROBIN_0,
            true => AXBS_L_FORCE_ROUND_ROBIN_A::AXBS_L_FORCE_ROUND_ROBIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_L_FORCE_ROUND_ROBIN_0`"]
    #[inline(always)]
    pub fn is_axbs_l_force_round_robin_0(&self) -> bool {
        *self == AXBS_L_FORCE_ROUND_ROBIN_A::AXBS_L_FORCE_ROUND_ROBIN_0
    }
    #[doc = "Checks if the value of the field is `AXBS_L_FORCE_ROUND_ROBIN_1`"]
    #[inline(always)]
    pub fn is_axbs_l_force_round_robin_1(&self) -> bool {
        *self == AXBS_L_FORCE_ROUND_ROBIN_A::AXBS_L_FORCE_ROUND_ROBIN_1
    }
}
#[doc = "Write proxy for field `AXBS_L_FORCE_ROUND_ROBIN`"]
pub struct AXBS_L_FORCE_ROUND_ROBIN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXBS_L_FORCE_ROUND_ROBIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXBS_L_FORCE_ROUND_ROBIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXBS_L masters are not arbitored in round robin, depending on DMA and AHBXL master priority settings."]
    #[inline(always)]
    pub fn axbs_l_force_round_robin_0(self) -> &'a mut W {
        self.variant(AXBS_L_FORCE_ROUND_ROBIN_A::AXBS_L_FORCE_ROUND_ROBIN_0)
    }
    #[doc = "AXBS_L masters are arbitored in round robin"]
    #[inline(always)]
    pub fn axbs_l_force_round_robin_1(self) -> &'a mut W {
        self.variant(AXBS_L_FORCE_ROUND_ROBIN_A::AXBS_L_FORCE_ROUND_ROBIN_1)
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
#[doc = "AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_P_M0_HIGH_PRIORITY_A {
    #[doc = "0: AXBS_P M0 master doesn't have high priority"]
    AXBS_P_M0_HIGH_PRIORITY_0 = 0,
    #[doc = "1: AXBS_P M0 master has high priority"]
    AXBS_P_M0_HIGH_PRIORITY_1 = 1,
}
impl From<AXBS_P_M0_HIGH_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_P_M0_HIGH_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_P_M0_HIGH_PRIORITY`"]
pub type AXBS_P_M0_HIGH_PRIORITY_R = crate::R<bool, AXBS_P_M0_HIGH_PRIORITY_A>;
impl AXBS_P_M0_HIGH_PRIORITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_P_M0_HIGH_PRIORITY_A {
        match self.bits {
            false => AXBS_P_M0_HIGH_PRIORITY_A::AXBS_P_M0_HIGH_PRIORITY_0,
            true => AXBS_P_M0_HIGH_PRIORITY_A::AXBS_P_M0_HIGH_PRIORITY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_P_M0_HIGH_PRIORITY_0`"]
    #[inline(always)]
    pub fn is_axbs_p_m0_high_priority_0(&self) -> bool {
        *self == AXBS_P_M0_HIGH_PRIORITY_A::AXBS_P_M0_HIGH_PRIORITY_0
    }
    #[doc = "Checks if the value of the field is `AXBS_P_M0_HIGH_PRIORITY_1`"]
    #[inline(always)]
    pub fn is_axbs_p_m0_high_priority_1(&self) -> bool {
        *self == AXBS_P_M0_HIGH_PRIORITY_A::AXBS_P_M0_HIGH_PRIORITY_1
    }
}
#[doc = "Write proxy for field `AXBS_P_M0_HIGH_PRIORITY`"]
pub struct AXBS_P_M0_HIGH_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> AXBS_P_M0_HIGH_PRIORITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXBS_P_M0_HIGH_PRIORITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXBS_P M0 master doesn't have high priority"]
    #[inline(always)]
    pub fn axbs_p_m0_high_priority_0(self) -> &'a mut W {
        self.variant(AXBS_P_M0_HIGH_PRIORITY_A::AXBS_P_M0_HIGH_PRIORITY_0)
    }
    #[doc = "AXBS_P M0 master has high priority"]
    #[inline(always)]
    pub fn axbs_p_m0_high_priority_1(self) -> &'a mut W {
        self.variant(AXBS_P_M0_HIGH_PRIORITY_A::AXBS_P_M0_HIGH_PRIORITY_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_P_M1_HIGH_PRIORITY_A {
    #[doc = "0: AXBS_P M1 master does not have high priority"]
    AXBS_P_M1_HIGH_PRIORITY_0 = 0,
    #[doc = "1: AXBS_P M1 master has high priority"]
    AXBS_P_M1_HIGH_PRIORITY_1 = 1,
}
impl From<AXBS_P_M1_HIGH_PRIORITY_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_P_M1_HIGH_PRIORITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_P_M1_HIGH_PRIORITY`"]
pub type AXBS_P_M1_HIGH_PRIORITY_R = crate::R<bool, AXBS_P_M1_HIGH_PRIORITY_A>;
impl AXBS_P_M1_HIGH_PRIORITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_P_M1_HIGH_PRIORITY_A {
        match self.bits {
            false => AXBS_P_M1_HIGH_PRIORITY_A::AXBS_P_M1_HIGH_PRIORITY_0,
            true => AXBS_P_M1_HIGH_PRIORITY_A::AXBS_P_M1_HIGH_PRIORITY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_P_M1_HIGH_PRIORITY_0`"]
    #[inline(always)]
    pub fn is_axbs_p_m1_high_priority_0(&self) -> bool {
        *self == AXBS_P_M1_HIGH_PRIORITY_A::AXBS_P_M1_HIGH_PRIORITY_0
    }
    #[doc = "Checks if the value of the field is `AXBS_P_M1_HIGH_PRIORITY_1`"]
    #[inline(always)]
    pub fn is_axbs_p_m1_high_priority_1(&self) -> bool {
        *self == AXBS_P_M1_HIGH_PRIORITY_A::AXBS_P_M1_HIGH_PRIORITY_1
    }
}
#[doc = "Write proxy for field `AXBS_P_M1_HIGH_PRIORITY`"]
pub struct AXBS_P_M1_HIGH_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> AXBS_P_M1_HIGH_PRIORITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXBS_P_M1_HIGH_PRIORITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXBS_P M1 master does not have high priority"]
    #[inline(always)]
    pub fn axbs_p_m1_high_priority_0(self) -> &'a mut W {
        self.variant(AXBS_P_M1_HIGH_PRIORITY_A::AXBS_P_M1_HIGH_PRIORITY_0)
    }
    #[doc = "AXBS_P M1 master has high priority"]
    #[inline(always)]
    pub fn axbs_p_m1_high_priority_1(self) -> &'a mut W {
        self.variant(AXBS_P_M1_HIGH_PRIORITY_A::AXBS_P_M1_HIGH_PRIORITY_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_P_FORCE_ROUND_ROBIN_A {
    #[doc = "0: AXBS_P masters are not arbitored in round robin, depending on M0/M1 master priority settings."]
    AXBS_P_FORCE_ROUND_ROBIN_0 = 0,
    #[doc = "1: AXBS_P masters are arbitored in round robin"]
    AXBS_P_FORCE_ROUND_ROBIN_1 = 1,
}
impl From<AXBS_P_FORCE_ROUND_ROBIN_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_P_FORCE_ROUND_ROBIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_P_FORCE_ROUND_ROBIN`"]
pub type AXBS_P_FORCE_ROUND_ROBIN_R = crate::R<bool, AXBS_P_FORCE_ROUND_ROBIN_A>;
impl AXBS_P_FORCE_ROUND_ROBIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_P_FORCE_ROUND_ROBIN_A {
        match self.bits {
            false => AXBS_P_FORCE_ROUND_ROBIN_A::AXBS_P_FORCE_ROUND_ROBIN_0,
            true => AXBS_P_FORCE_ROUND_ROBIN_A::AXBS_P_FORCE_ROUND_ROBIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_P_FORCE_ROUND_ROBIN_0`"]
    #[inline(always)]
    pub fn is_axbs_p_force_round_robin_0(&self) -> bool {
        *self == AXBS_P_FORCE_ROUND_ROBIN_A::AXBS_P_FORCE_ROUND_ROBIN_0
    }
    #[doc = "Checks if the value of the field is `AXBS_P_FORCE_ROUND_ROBIN_1`"]
    #[inline(always)]
    pub fn is_axbs_p_force_round_robin_1(&self) -> bool {
        *self == AXBS_P_FORCE_ROUND_ROBIN_A::AXBS_P_FORCE_ROUND_ROBIN_1
    }
}
#[doc = "Write proxy for field `AXBS_P_FORCE_ROUND_ROBIN`"]
pub struct AXBS_P_FORCE_ROUND_ROBIN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXBS_P_FORCE_ROUND_ROBIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXBS_P_FORCE_ROUND_ROBIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXBS_P masters are not arbitored in round robin, depending on M0/M1 master priority settings."]
    #[inline(always)]
    pub fn axbs_p_force_round_robin_0(self) -> &'a mut W {
        self.variant(AXBS_P_FORCE_ROUND_ROBIN_A::AXBS_P_FORCE_ROUND_ROBIN_0)
    }
    #[doc = "AXBS_P masters are arbitored in round robin"]
    #[inline(always)]
    pub fn axbs_p_force_round_robin_1(self) -> &'a mut W {
        self.variant(AXBS_P_FORCE_ROUND_ROBIN_A::AXBS_P_FORCE_ROUND_ROBIN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Disable CANFD filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CANFD_FILTER_BYPASS_A {
    #[doc = "0: enable CANFD filter"]
    CANFD_FILTER_BYPASS_0 = 0,
    #[doc = "1: disable CANFD filter"]
    CANFD_FILTER_BYPASS_1 = 1,
}
impl From<CANFD_FILTER_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: CANFD_FILTER_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CANFD_FILTER_BYPASS`"]
pub type CANFD_FILTER_BYPASS_R = crate::R<bool, CANFD_FILTER_BYPASS_A>;
impl CANFD_FILTER_BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANFD_FILTER_BYPASS_A {
        match self.bits {
            false => CANFD_FILTER_BYPASS_A::CANFD_FILTER_BYPASS_0,
            true => CANFD_FILTER_BYPASS_A::CANFD_FILTER_BYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CANFD_FILTER_BYPASS_0`"]
    #[inline(always)]
    pub fn is_canfd_filter_bypass_0(&self) -> bool {
        *self == CANFD_FILTER_BYPASS_A::CANFD_FILTER_BYPASS_0
    }
    #[doc = "Checks if the value of the field is `CANFD_FILTER_BYPASS_1`"]
    #[inline(always)]
    pub fn is_canfd_filter_bypass_1(&self) -> bool {
        *self == CANFD_FILTER_BYPASS_A::CANFD_FILTER_BYPASS_1
    }
}
#[doc = "Write proxy for field `CANFD_FILTER_BYPASS`"]
pub struct CANFD_FILTER_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CANFD_FILTER_BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CANFD_FILTER_BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "enable CANFD filter"]
    #[inline(always)]
    pub fn canfd_filter_bypass_0(self) -> &'a mut W {
        self.variant(CANFD_FILTER_BYPASS_A::CANFD_FILTER_BYPASS_0)
    }
    #[doc = "disable CANFD filter"]
    #[inline(always)]
    pub fn canfd_filter_bypass_1(self) -> &'a mut W {
        self.variant(CANFD_FILTER_BYPASS_A::CANFD_FILTER_BYPASS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "enable power saving features on L2 memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L2_MEM_EN_POWERSAVING_A {
    #[doc = "0: none memory power saving features enabled, SHUTDOWN/DEEPSLEEP/LIGHTSLEEP will have no effect"]
    L2_MEM_EN_POWERSAVING_0 = 0,
    #[doc = "1: memory power saving features enabled, set SHUTDOWN/DEEPSLEEP/LIGHTSLEEP (priority high to low) to enable power saving levels"]
    L2_MEM_EN_POWERSAVING_1 = 1,
}
impl From<L2_MEM_EN_POWERSAVING_A> for bool {
    #[inline(always)]
    fn from(variant: L2_MEM_EN_POWERSAVING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L2_MEM_EN_POWERSAVING`"]
pub type L2_MEM_EN_POWERSAVING_R = crate::R<bool, L2_MEM_EN_POWERSAVING_A>;
impl L2_MEM_EN_POWERSAVING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L2_MEM_EN_POWERSAVING_A {
        match self.bits {
            false => L2_MEM_EN_POWERSAVING_A::L2_MEM_EN_POWERSAVING_0,
            true => L2_MEM_EN_POWERSAVING_A::L2_MEM_EN_POWERSAVING_1,
        }
    }
    #[doc = "Checks if the value of the field is `L2_MEM_EN_POWERSAVING_0`"]
    #[inline(always)]
    pub fn is_l2_mem_en_powersaving_0(&self) -> bool {
        *self == L2_MEM_EN_POWERSAVING_A::L2_MEM_EN_POWERSAVING_0
    }
    #[doc = "Checks if the value of the field is `L2_MEM_EN_POWERSAVING_1`"]
    #[inline(always)]
    pub fn is_l2_mem_en_powersaving_1(&self) -> bool {
        *self == L2_MEM_EN_POWERSAVING_A::L2_MEM_EN_POWERSAVING_1
    }
}
#[doc = "Write proxy for field `L2_MEM_EN_POWERSAVING`"]
pub struct L2_MEM_EN_POWERSAVING_W<'a> {
    w: &'a mut W,
}
impl<'a> L2_MEM_EN_POWERSAVING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L2_MEM_EN_POWERSAVING_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "none memory power saving features enabled, SHUTDOWN/DEEPSLEEP/LIGHTSLEEP will have no effect"]
    #[inline(always)]
    pub fn l2_mem_en_powersaving_0(self) -> &'a mut W {
        self.variant(L2_MEM_EN_POWERSAVING_A::L2_MEM_EN_POWERSAVING_0)
    }
    #[doc = "memory power saving features enabled, set SHUTDOWN/DEEPSLEEP/LIGHTSLEEP (priority high to low) to enable power saving levels"]
    #[inline(always)]
    pub fn l2_mem_en_powersaving_1(self) -> &'a mut W {
        self.variant(L2_MEM_EN_POWERSAVING_A::L2_MEM_EN_POWERSAVING_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Automatically gate off RAM clock when RAM is not accessed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AUTO_CLK_GATING_EN_A {
    #[doc = "0: disable automatically gate off RAM clock"]
    RAM_AUTO_CLK_GATING_EN_0 = 0,
    #[doc = "1: enable automatically gate off RAM clock"]
    RAM_AUTO_CLK_GATING_EN_1 = 1,
}
impl From<RAM_AUTO_CLK_GATING_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AUTO_CLK_GATING_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAM_AUTO_CLK_GATING_EN`"]
pub type RAM_AUTO_CLK_GATING_EN_R = crate::R<bool, RAM_AUTO_CLK_GATING_EN_A>;
impl RAM_AUTO_CLK_GATING_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AUTO_CLK_GATING_EN_A {
        match self.bits {
            false => RAM_AUTO_CLK_GATING_EN_A::RAM_AUTO_CLK_GATING_EN_0,
            true => RAM_AUTO_CLK_GATING_EN_A::RAM_AUTO_CLK_GATING_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RAM_AUTO_CLK_GATING_EN_0`"]
    #[inline(always)]
    pub fn is_ram_auto_clk_gating_en_0(&self) -> bool {
        *self == RAM_AUTO_CLK_GATING_EN_A::RAM_AUTO_CLK_GATING_EN_0
    }
    #[doc = "Checks if the value of the field is `RAM_AUTO_CLK_GATING_EN_1`"]
    #[inline(always)]
    pub fn is_ram_auto_clk_gating_en_1(&self) -> bool {
        *self == RAM_AUTO_CLK_GATING_EN_A::RAM_AUTO_CLK_GATING_EN_1
    }
}
#[doc = "Write proxy for field `RAM_AUTO_CLK_GATING_EN`"]
pub struct RAM_AUTO_CLK_GATING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AUTO_CLK_GATING_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AUTO_CLK_GATING_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable automatically gate off RAM clock"]
    #[inline(always)]
    pub fn ram_auto_clk_gating_en_0(self) -> &'a mut W {
        self.variant(RAM_AUTO_CLK_GATING_EN_A::RAM_AUTO_CLK_GATING_EN_0)
    }
    #[doc = "enable automatically gate off RAM clock"]
    #[inline(always)]
    pub fn ram_auto_clk_gating_en_1(self) -> &'a mut W {
        self.variant(RAM_AUTO_CLK_GATING_EN_A::RAM_AUTO_CLK_GATING_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "control how memory enter Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L2_MEM_DEEPSLEEP_A {
    #[doc = "0: no force sleep control supported, memory deep sleep mode only entered when whole system in stop mode"]
    L2_MEM_DEEPSLEEP_0 = 0,
    #[doc = "1: force memory into deep sleep mode"]
    L2_MEM_DEEPSLEEP_1 = 1,
}
impl From<L2_MEM_DEEPSLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: L2_MEM_DEEPSLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L2_MEM_DEEPSLEEP`"]
pub type L2_MEM_DEEPSLEEP_R = crate::R<bool, L2_MEM_DEEPSLEEP_A>;
impl L2_MEM_DEEPSLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L2_MEM_DEEPSLEEP_A {
        match self.bits {
            false => L2_MEM_DEEPSLEEP_A::L2_MEM_DEEPSLEEP_0,
            true => L2_MEM_DEEPSLEEP_A::L2_MEM_DEEPSLEEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L2_MEM_DEEPSLEEP_0`"]
    #[inline(always)]
    pub fn is_l2_mem_deepsleep_0(&self) -> bool {
        *self == L2_MEM_DEEPSLEEP_A::L2_MEM_DEEPSLEEP_0
    }
    #[doc = "Checks if the value of the field is `L2_MEM_DEEPSLEEP_1`"]
    #[inline(always)]
    pub fn is_l2_mem_deepsleep_1(&self) -> bool {
        *self == L2_MEM_DEEPSLEEP_A::L2_MEM_DEEPSLEEP_1
    }
}
#[doc = "Write proxy for field `L2_MEM_DEEPSLEEP`"]
pub struct L2_MEM_DEEPSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> L2_MEM_DEEPSLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L2_MEM_DEEPSLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no force sleep control supported, memory deep sleep mode only entered when whole system in stop mode"]
    #[inline(always)]
    pub fn l2_mem_deepsleep_0(self) -> &'a mut W {
        self.variant(L2_MEM_DEEPSLEEP_A::L2_MEM_DEEPSLEEP_0)
    }
    #[doc = "force memory into deep sleep mode"]
    #[inline(always)]
    pub fn l2_mem_deepsleep_1(self) -> &'a mut W {
        self.variant(L2_MEM_DEEPSLEEP_A::L2_MEM_DEEPSLEEP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MQS_CLK_DIV_A {
    #[doc = "0: mclk frequency = 1/1 * hmclk frequency"]
    DIVIDE_1 = 0,
    #[doc = "1: mclk frequency = 1/2 * hmclk frequency"]
    DIVIDE_2 = 1,
    #[doc = "2: mclk frequency = 1/3 * hmclk frequency"]
    DIVIDE_3 = 2,
    #[doc = "3: mclk frequency = 1/4 * hmclk frequency"]
    DIVIDE_4 = 3,
    #[doc = "4: mclk frequency = 1/5 * hmclk frequency"]
    DIVIDE_5 = 4,
    #[doc = "5: mclk frequency = 1/6 * hmclk frequency"]
    DIVIDE_6 = 5,
    #[doc = "6: mclk frequency = 1/7 * hmclk frequency"]
    DIVIDE_7 = 6,
    #[doc = "7: mclk frequency = 1/8 * hmclk frequency"]
    DIVIDE_8 = 7,
    #[doc = "8: mclk frequency = 1/9 * hmclk frequency"]
    DIVIDE_9 = 8,
    #[doc = "9: mclk frequency = 1/10 * hmclk frequency"]
    DIVIDE_10 = 9,
    #[doc = "10: mclk frequency = 1/11 * hmclk frequency"]
    DIVIDE_11 = 10,
    #[doc = "11: mclk frequency = 1/12 * hmclk frequency"]
    DIVIDE_12 = 11,
    #[doc = "12: mclk frequency = 1/13 * hmclk frequency"]
    DIVIDE_13 = 12,
    #[doc = "13: mclk frequency = 1/14 * hmclk frequency"]
    DIVIDE_14 = 13,
    #[doc = "14: mclk frequency = 1/15 * hmclk frequency"]
    DIVIDE_15 = 14,
    #[doc = "15: mclk frequency = 1/16 * hmclk frequency"]
    DIVIDE_16 = 15,
    #[doc = "16: mclk frequency = 1/17 * hmclk frequency"]
    DIVIDE_17 = 16,
    #[doc = "17: mclk frequency = 1/18 * hmclk frequency"]
    DIVIDE_18 = 17,
    #[doc = "18: mclk frequency = 1/19 * hmclk frequency"]
    DIVIDE_19 = 18,
    #[doc = "19: mclk frequency = 1/20 * hmclk frequency"]
    DIVIDE_20 = 19,
    #[doc = "20: mclk frequency = 1/21 * hmclk frequency"]
    DIVIDE_21 = 20,
    #[doc = "21: mclk frequency = 1/22 * hmclk frequency"]
    DIVIDE_22 = 21,
    #[doc = "22: mclk frequency = 1/23 * hmclk frequency"]
    DIVIDE_23 = 22,
    #[doc = "23: mclk frequency = 1/24 * hmclk frequency"]
    DIVIDE_24 = 23,
    #[doc = "24: mclk frequency = 1/25 * hmclk frequency"]
    DIVIDE_25 = 24,
    #[doc = "25: mclk frequency = 1/26 * hmclk frequency"]
    DIVIDE_26 = 25,
    #[doc = "26: mclk frequency = 1/27 * hmclk frequency"]
    DIVIDE_27 = 26,
    #[doc = "27: mclk frequency = 1/28 * hmclk frequency"]
    DIVIDE_28 = 27,
    #[doc = "28: mclk frequency = 1/29 * hmclk frequency"]
    DIVIDE_29 = 28,
    #[doc = "29: mclk frequency = 1/30 * hmclk frequency"]
    DIVIDE_30 = 29,
    #[doc = "30: mclk frequency = 1/31 * hmclk frequency"]
    DIVIDE_31 = 30,
    #[doc = "31: mclk frequency = 1/32 * hmclk frequency"]
    DIVIDE_32 = 31,
    #[doc = "32: mclk frequency = 1/33 * hmclk frequency"]
    DIVIDE_33 = 32,
    #[doc = "33: mclk frequency = 1/34 * hmclk frequency"]
    DIVIDE_34 = 33,
    #[doc = "34: mclk frequency = 1/35 * hmclk frequency"]
    DIVIDE_35 = 34,
    #[doc = "35: mclk frequency = 1/36 * hmclk frequency"]
    DIVIDE_36 = 35,
    #[doc = "36: mclk frequency = 1/37 * hmclk frequency"]
    DIVIDE_37 = 36,
    #[doc = "37: mclk frequency = 1/38 * hmclk frequency"]
    DIVIDE_38 = 37,
    #[doc = "38: mclk frequency = 1/39 * hmclk frequency"]
    DIVIDE_39 = 38,
    #[doc = "39: mclk frequency = 1/40 * hmclk frequency"]
    DIVIDE_40 = 39,
    #[doc = "40: mclk frequency = 1/41 * hmclk frequency"]
    DIVIDE_41 = 40,
    #[doc = "41: mclk frequency = 1/42 * hmclk frequency"]
    DIVIDE_42 = 41,
    #[doc = "42: mclk frequency = 1/43 * hmclk frequency"]
    DIVIDE_43 = 42,
    #[doc = "43: mclk frequency = 1/44 * hmclk frequency"]
    DIVIDE_44 = 43,
    #[doc = "44: mclk frequency = 1/45 * hmclk frequency"]
    DIVIDE_45 = 44,
    #[doc = "45: mclk frequency = 1/46 * hmclk frequency"]
    DIVIDE_46 = 45,
    #[doc = "46: mclk frequency = 1/47 * hmclk frequency"]
    DIVIDE_47 = 46,
    #[doc = "47: mclk frequency = 1/48 * hmclk frequency"]
    DIVIDE_48 = 47,
    #[doc = "48: mclk frequency = 1/49 * hmclk frequency"]
    DIVIDE_49 = 48,
    #[doc = "49: mclk frequency = 1/50 * hmclk frequency"]
    DIVIDE_50 = 49,
    #[doc = "50: mclk frequency = 1/51 * hmclk frequency"]
    DIVIDE_51 = 50,
    #[doc = "51: mclk frequency = 1/52 * hmclk frequency"]
    DIVIDE_52 = 51,
    #[doc = "52: mclk frequency = 1/53 * hmclk frequency"]
    DIVIDE_53 = 52,
    #[doc = "53: mclk frequency = 1/54 * hmclk frequency"]
    DIVIDE_54 = 53,
    #[doc = "54: mclk frequency = 1/55 * hmclk frequency"]
    DIVIDE_55 = 54,
    #[doc = "55: mclk frequency = 1/56 * hmclk frequency"]
    DIVIDE_56 = 55,
    #[doc = "56: mclk frequency = 1/57 * hmclk frequency"]
    DIVIDE_57 = 56,
    #[doc = "57: mclk frequency = 1/58 * hmclk frequency"]
    DIVIDE_58 = 57,
    #[doc = "58: mclk frequency = 1/59 * hmclk frequency"]
    DIVIDE_59 = 58,
    #[doc = "59: mclk frequency = 1/60 * hmclk frequency"]
    DIVIDE_60 = 59,
    #[doc = "60: mclk frequency = 1/61 * hmclk frequency"]
    DIVIDE_61 = 60,
    #[doc = "61: mclk frequency = 1/62 * hmclk frequency"]
    DIVIDE_62 = 61,
    #[doc = "62: mclk frequency = 1/63 * hmclk frequency"]
    DIVIDE_63 = 62,
    #[doc = "63: mclk frequency = 1/64 * hmclk frequency"]
    DIVIDE_64 = 63,
    #[doc = "64: mclk frequency = 1/65 * hmclk frequency"]
    DIVIDE_65 = 64,
    #[doc = "65: mclk frequency = 1/66 * hmclk frequency"]
    DIVIDE_66 = 65,
    #[doc = "66: mclk frequency = 1/67 * hmclk frequency"]
    DIVIDE_67 = 66,
    #[doc = "67: mclk frequency = 1/68 * hmclk frequency"]
    DIVIDE_68 = 67,
    #[doc = "68: mclk frequency = 1/69 * hmclk frequency"]
    DIVIDE_69 = 68,
    #[doc = "69: mclk frequency = 1/70 * hmclk frequency"]
    DIVIDE_70 = 69,
    #[doc = "70: mclk frequency = 1/71 * hmclk frequency"]
    DIVIDE_71 = 70,
    #[doc = "71: mclk frequency = 1/72 * hmclk frequency"]
    DIVIDE_72 = 71,
    #[doc = "72: mclk frequency = 1/73 * hmclk frequency"]
    DIVIDE_73 = 72,
    #[doc = "73: mclk frequency = 1/74 * hmclk frequency"]
    DIVIDE_74 = 73,
    #[doc = "74: mclk frequency = 1/75 * hmclk frequency"]
    DIVIDE_75 = 74,
    #[doc = "75: mclk frequency = 1/76 * hmclk frequency"]
    DIVIDE_76 = 75,
    #[doc = "76: mclk frequency = 1/77 * hmclk frequency"]
    DIVIDE_77 = 76,
    #[doc = "77: mclk frequency = 1/78 * hmclk frequency"]
    DIVIDE_78 = 77,
    #[doc = "78: mclk frequency = 1/79 * hmclk frequency"]
    DIVIDE_79 = 78,
    #[doc = "79: mclk frequency = 1/80 * hmclk frequency"]
    DIVIDE_80 = 79,
    #[doc = "80: mclk frequency = 1/81 * hmclk frequency"]
    DIVIDE_81 = 80,
    #[doc = "81: mclk frequency = 1/82 * hmclk frequency"]
    DIVIDE_82 = 81,
    #[doc = "82: mclk frequency = 1/83 * hmclk frequency"]
    DIVIDE_83 = 82,
    #[doc = "83: mclk frequency = 1/84 * hmclk frequency"]
    DIVIDE_84 = 83,
    #[doc = "84: mclk frequency = 1/85 * hmclk frequency"]
    DIVIDE_85 = 84,
    #[doc = "85: mclk frequency = 1/86 * hmclk frequency"]
    DIVIDE_86 = 85,
    #[doc = "86: mclk frequency = 1/87 * hmclk frequency"]
    DIVIDE_87 = 86,
    #[doc = "87: mclk frequency = 1/88 * hmclk frequency"]
    DIVIDE_88 = 87,
    #[doc = "88: mclk frequency = 1/89 * hmclk frequency"]
    DIVIDE_89 = 88,
    #[doc = "89: mclk frequency = 1/90 * hmclk frequency"]
    DIVIDE_90 = 89,
    #[doc = "90: mclk frequency = 1/91 * hmclk frequency"]
    DIVIDE_91 = 90,
    #[doc = "91: mclk frequency = 1/92 * hmclk frequency"]
    DIVIDE_92 = 91,
    #[doc = "92: mclk frequency = 1/93 * hmclk frequency"]
    DIVIDE_93 = 92,
    #[doc = "93: mclk frequency = 1/94 * hmclk frequency"]
    DIVIDE_94 = 93,
    #[doc = "94: mclk frequency = 1/95 * hmclk frequency"]
    DIVIDE_95 = 94,
    #[doc = "95: mclk frequency = 1/96 * hmclk frequency"]
    DIVIDE_96 = 95,
    #[doc = "96: mclk frequency = 1/97 * hmclk frequency"]
    DIVIDE_97 = 96,
    #[doc = "97: mclk frequency = 1/98 * hmclk frequency"]
    DIVIDE_98 = 97,
    #[doc = "98: mclk frequency = 1/99 * hmclk frequency"]
    DIVIDE_99 = 98,
    #[doc = "99: mclk frequency = 1/100 * hmclk frequency"]
    DIVIDE_100 = 99,
    #[doc = "100: mclk frequency = 1/101 * hmclk frequency"]
    DIVIDE_101 = 100,
    #[doc = "101: mclk frequency = 1/102 * hmclk frequency"]
    DIVIDE_102 = 101,
    #[doc = "102: mclk frequency = 1/103 * hmclk frequency"]
    DIVIDE_103 = 102,
    #[doc = "103: mclk frequency = 1/104 * hmclk frequency"]
    DIVIDE_104 = 103,
    #[doc = "104: mclk frequency = 1/105 * hmclk frequency"]
    DIVIDE_105 = 104,
    #[doc = "105: mclk frequency = 1/106 * hmclk frequency"]
    DIVIDE_106 = 105,
    #[doc = "106: mclk frequency = 1/107 * hmclk frequency"]
    DIVIDE_107 = 106,
    #[doc = "107: mclk frequency = 1/108 * hmclk frequency"]
    DIVIDE_108 = 107,
    #[doc = "108: mclk frequency = 1/109 * hmclk frequency"]
    DIVIDE_109 = 108,
    #[doc = "109: mclk frequency = 1/110 * hmclk frequency"]
    DIVIDE_110 = 109,
    #[doc = "110: mclk frequency = 1/111 * hmclk frequency"]
    DIVIDE_111 = 110,
    #[doc = "111: mclk frequency = 1/112 * hmclk frequency"]
    DIVIDE_112 = 111,
    #[doc = "112: mclk frequency = 1/113 * hmclk frequency"]
    DIVIDE_113 = 112,
    #[doc = "113: mclk frequency = 1/114 * hmclk frequency"]
    DIVIDE_114 = 113,
    #[doc = "114: mclk frequency = 1/115 * hmclk frequency"]
    DIVIDE_115 = 114,
    #[doc = "115: mclk frequency = 1/116 * hmclk frequency"]
    DIVIDE_116 = 115,
    #[doc = "116: mclk frequency = 1/117 * hmclk frequency"]
    DIVIDE_117 = 116,
    #[doc = "117: mclk frequency = 1/118 * hmclk frequency"]
    DIVIDE_118 = 117,
    #[doc = "118: mclk frequency = 1/119 * hmclk frequency"]
    DIVIDE_119 = 118,
    #[doc = "119: mclk frequency = 1/120 * hmclk frequency"]
    DIVIDE_120 = 119,
    #[doc = "120: mclk frequency = 1/121 * hmclk frequency"]
    DIVIDE_121 = 120,
    #[doc = "121: mclk frequency = 1/122 * hmclk frequency"]
    DIVIDE_122 = 121,
    #[doc = "122: mclk frequency = 1/123 * hmclk frequency"]
    DIVIDE_123 = 122,
    #[doc = "123: mclk frequency = 1/124 * hmclk frequency"]
    DIVIDE_124 = 123,
    #[doc = "124: mclk frequency = 1/125 * hmclk frequency"]
    DIVIDE_125 = 124,
    #[doc = "125: mclk frequency = 1/126 * hmclk frequency"]
    DIVIDE_126 = 125,
    #[doc = "126: mclk frequency = 1/127 * hmclk frequency"]
    DIVIDE_127 = 126,
    #[doc = "127: mclk frequency = 1/128 * hmclk frequency"]
    DIVIDE_128 = 127,
    #[doc = "128: mclk frequency = 1/129 * hmclk frequency"]
    DIVIDE_129 = 128,
    #[doc = "129: mclk frequency = 1/130 * hmclk frequency"]
    DIVIDE_130 = 129,
    #[doc = "130: mclk frequency = 1/131 * hmclk frequency"]
    DIVIDE_131 = 130,
    #[doc = "131: mclk frequency = 1/132 * hmclk frequency"]
    DIVIDE_132 = 131,
    #[doc = "132: mclk frequency = 1/133 * hmclk frequency"]
    DIVIDE_133 = 132,
    #[doc = "133: mclk frequency = 1/134 * hmclk frequency"]
    DIVIDE_134 = 133,
    #[doc = "134: mclk frequency = 1/135 * hmclk frequency"]
    DIVIDE_135 = 134,
    #[doc = "135: mclk frequency = 1/136 * hmclk frequency"]
    DIVIDE_136 = 135,
    #[doc = "136: mclk frequency = 1/137 * hmclk frequency"]
    DIVIDE_137 = 136,
    #[doc = "137: mclk frequency = 1/138 * hmclk frequency"]
    DIVIDE_138 = 137,
    #[doc = "138: mclk frequency = 1/139 * hmclk frequency"]
    DIVIDE_139 = 138,
    #[doc = "139: mclk frequency = 1/140 * hmclk frequency"]
    DIVIDE_140 = 139,
    #[doc = "140: mclk frequency = 1/141 * hmclk frequency"]
    DIVIDE_141 = 140,
    #[doc = "141: mclk frequency = 1/142 * hmclk frequency"]
    DIVIDE_142 = 141,
    #[doc = "142: mclk frequency = 1/143 * hmclk frequency"]
    DIVIDE_143 = 142,
    #[doc = "143: mclk frequency = 1/144 * hmclk frequency"]
    DIVIDE_144 = 143,
    #[doc = "144: mclk frequency = 1/145 * hmclk frequency"]
    DIVIDE_145 = 144,
    #[doc = "145: mclk frequency = 1/146 * hmclk frequency"]
    DIVIDE_146 = 145,
    #[doc = "146: mclk frequency = 1/147 * hmclk frequency"]
    DIVIDE_147 = 146,
    #[doc = "147: mclk frequency = 1/148 * hmclk frequency"]
    DIVIDE_148 = 147,
    #[doc = "148: mclk frequency = 1/149 * hmclk frequency"]
    DIVIDE_149 = 148,
    #[doc = "149: mclk frequency = 1/150 * hmclk frequency"]
    DIVIDE_150 = 149,
    #[doc = "150: mclk frequency = 1/151 * hmclk frequency"]
    DIVIDE_151 = 150,
    #[doc = "151: mclk frequency = 1/152 * hmclk frequency"]
    DIVIDE_152 = 151,
    #[doc = "152: mclk frequency = 1/153 * hmclk frequency"]
    DIVIDE_153 = 152,
    #[doc = "153: mclk frequency = 1/154 * hmclk frequency"]
    DIVIDE_154 = 153,
    #[doc = "154: mclk frequency = 1/155 * hmclk frequency"]
    DIVIDE_155 = 154,
    #[doc = "155: mclk frequency = 1/156 * hmclk frequency"]
    DIVIDE_156 = 155,
    #[doc = "156: mclk frequency = 1/157 * hmclk frequency"]
    DIVIDE_157 = 156,
    #[doc = "157: mclk frequency = 1/158 * hmclk frequency"]
    DIVIDE_158 = 157,
    #[doc = "158: mclk frequency = 1/159 * hmclk frequency"]
    DIVIDE_159 = 158,
    #[doc = "159: mclk frequency = 1/160 * hmclk frequency"]
    DIVIDE_160 = 159,
    #[doc = "160: mclk frequency = 1/161 * hmclk frequency"]
    DIVIDE_161 = 160,
    #[doc = "161: mclk frequency = 1/162 * hmclk frequency"]
    DIVIDE_162 = 161,
    #[doc = "162: mclk frequency = 1/163 * hmclk frequency"]
    DIVIDE_163 = 162,
    #[doc = "163: mclk frequency = 1/164 * hmclk frequency"]
    DIVIDE_164 = 163,
    #[doc = "164: mclk frequency = 1/165 * hmclk frequency"]
    DIVIDE_165 = 164,
    #[doc = "165: mclk frequency = 1/166 * hmclk frequency"]
    DIVIDE_166 = 165,
    #[doc = "166: mclk frequency = 1/167 * hmclk frequency"]
    DIVIDE_167 = 166,
    #[doc = "167: mclk frequency = 1/168 * hmclk frequency"]
    DIVIDE_168 = 167,
    #[doc = "168: mclk frequency = 1/169 * hmclk frequency"]
    DIVIDE_169 = 168,
    #[doc = "169: mclk frequency = 1/170 * hmclk frequency"]
    DIVIDE_170 = 169,
    #[doc = "170: mclk frequency = 1/171 * hmclk frequency"]
    DIVIDE_171 = 170,
    #[doc = "171: mclk frequency = 1/172 * hmclk frequency"]
    DIVIDE_172 = 171,
    #[doc = "172: mclk frequency = 1/173 * hmclk frequency"]
    DIVIDE_173 = 172,
    #[doc = "173: mclk frequency = 1/174 * hmclk frequency"]
    DIVIDE_174 = 173,
    #[doc = "174: mclk frequency = 1/175 * hmclk frequency"]
    DIVIDE_175 = 174,
    #[doc = "175: mclk frequency = 1/176 * hmclk frequency"]
    DIVIDE_176 = 175,
    #[doc = "176: mclk frequency = 1/177 * hmclk frequency"]
    DIVIDE_177 = 176,
    #[doc = "177: mclk frequency = 1/178 * hmclk frequency"]
    DIVIDE_178 = 177,
    #[doc = "178: mclk frequency = 1/179 * hmclk frequency"]
    DIVIDE_179 = 178,
    #[doc = "179: mclk frequency = 1/180 * hmclk frequency"]
    DIVIDE_180 = 179,
    #[doc = "180: mclk frequency = 1/181 * hmclk frequency"]
    DIVIDE_181 = 180,
    #[doc = "181: mclk frequency = 1/182 * hmclk frequency"]
    DIVIDE_182 = 181,
    #[doc = "182: mclk frequency = 1/183 * hmclk frequency"]
    DIVIDE_183 = 182,
    #[doc = "183: mclk frequency = 1/184 * hmclk frequency"]
    DIVIDE_184 = 183,
    #[doc = "184: mclk frequency = 1/185 * hmclk frequency"]
    DIVIDE_185 = 184,
    #[doc = "185: mclk frequency = 1/186 * hmclk frequency"]
    DIVIDE_186 = 185,
    #[doc = "186: mclk frequency = 1/187 * hmclk frequency"]
    DIVIDE_187 = 186,
    #[doc = "187: mclk frequency = 1/188 * hmclk frequency"]
    DIVIDE_188 = 187,
    #[doc = "188: mclk frequency = 1/189 * hmclk frequency"]
    DIVIDE_189 = 188,
    #[doc = "189: mclk frequency = 1/190 * hmclk frequency"]
    DIVIDE_190 = 189,
    #[doc = "190: mclk frequency = 1/191 * hmclk frequency"]
    DIVIDE_191 = 190,
    #[doc = "191: mclk frequency = 1/192 * hmclk frequency"]
    DIVIDE_192 = 191,
    #[doc = "192: mclk frequency = 1/193 * hmclk frequency"]
    DIVIDE_193 = 192,
    #[doc = "193: mclk frequency = 1/194 * hmclk frequency"]
    DIVIDE_194 = 193,
    #[doc = "194: mclk frequency = 1/195 * hmclk frequency"]
    DIVIDE_195 = 194,
    #[doc = "195: mclk frequency = 1/196 * hmclk frequency"]
    DIVIDE_196 = 195,
    #[doc = "196: mclk frequency = 1/197 * hmclk frequency"]
    DIVIDE_197 = 196,
    #[doc = "197: mclk frequency = 1/198 * hmclk frequency"]
    DIVIDE_198 = 197,
    #[doc = "198: mclk frequency = 1/199 * hmclk frequency"]
    DIVIDE_199 = 198,
    #[doc = "199: mclk frequency = 1/200 * hmclk frequency"]
    DIVIDE_200 = 199,
    #[doc = "200: mclk frequency = 1/201 * hmclk frequency"]
    DIVIDE_201 = 200,
    #[doc = "201: mclk frequency = 1/202 * hmclk frequency"]
    DIVIDE_202 = 201,
    #[doc = "202: mclk frequency = 1/203 * hmclk frequency"]
    DIVIDE_203 = 202,
    #[doc = "203: mclk frequency = 1/204 * hmclk frequency"]
    DIVIDE_204 = 203,
    #[doc = "204: mclk frequency = 1/205 * hmclk frequency"]
    DIVIDE_205 = 204,
    #[doc = "205: mclk frequency = 1/206 * hmclk frequency"]
    DIVIDE_206 = 205,
    #[doc = "206: mclk frequency = 1/207 * hmclk frequency"]
    DIVIDE_207 = 206,
    #[doc = "207: mclk frequency = 1/208 * hmclk frequency"]
    DIVIDE_208 = 207,
    #[doc = "208: mclk frequency = 1/209 * hmclk frequency"]
    DIVIDE_209 = 208,
    #[doc = "209: mclk frequency = 1/210 * hmclk frequency"]
    DIVIDE_210 = 209,
    #[doc = "210: mclk frequency = 1/211 * hmclk frequency"]
    DIVIDE_211 = 210,
    #[doc = "211: mclk frequency = 1/212 * hmclk frequency"]
    DIVIDE_212 = 211,
    #[doc = "212: mclk frequency = 1/213 * hmclk frequency"]
    DIVIDE_213 = 212,
    #[doc = "213: mclk frequency = 1/214 * hmclk frequency"]
    DIVIDE_214 = 213,
    #[doc = "214: mclk frequency = 1/215 * hmclk frequency"]
    DIVIDE_215 = 214,
    #[doc = "215: mclk frequency = 1/216 * hmclk frequency"]
    DIVIDE_216 = 215,
    #[doc = "216: mclk frequency = 1/217 * hmclk frequency"]
    DIVIDE_217 = 216,
    #[doc = "217: mclk frequency = 1/218 * hmclk frequency"]
    DIVIDE_218 = 217,
    #[doc = "218: mclk frequency = 1/219 * hmclk frequency"]
    DIVIDE_219 = 218,
    #[doc = "219: mclk frequency = 1/220 * hmclk frequency"]
    DIVIDE_220 = 219,
    #[doc = "220: mclk frequency = 1/221 * hmclk frequency"]
    DIVIDE_221 = 220,
    #[doc = "221: mclk frequency = 1/222 * hmclk frequency"]
    DIVIDE_222 = 221,
    #[doc = "222: mclk frequency = 1/223 * hmclk frequency"]
    DIVIDE_223 = 222,
    #[doc = "223: mclk frequency = 1/224 * hmclk frequency"]
    DIVIDE_224 = 223,
    #[doc = "224: mclk frequency = 1/225 * hmclk frequency"]
    DIVIDE_225 = 224,
    #[doc = "225: mclk frequency = 1/226 * hmclk frequency"]
    DIVIDE_226 = 225,
    #[doc = "226: mclk frequency = 1/227 * hmclk frequency"]
    DIVIDE_227 = 226,
    #[doc = "227: mclk frequency = 1/228 * hmclk frequency"]
    DIVIDE_228 = 227,
    #[doc = "228: mclk frequency = 1/229 * hmclk frequency"]
    DIVIDE_229 = 228,
    #[doc = "229: mclk frequency = 1/230 * hmclk frequency"]
    DIVIDE_230 = 229,
    #[doc = "230: mclk frequency = 1/231 * hmclk frequency"]
    DIVIDE_231 = 230,
    #[doc = "231: mclk frequency = 1/232 * hmclk frequency"]
    DIVIDE_232 = 231,
    #[doc = "232: mclk frequency = 1/233 * hmclk frequency"]
    DIVIDE_233 = 232,
    #[doc = "233: mclk frequency = 1/234 * hmclk frequency"]
    DIVIDE_234 = 233,
    #[doc = "234: mclk frequency = 1/235 * hmclk frequency"]
    DIVIDE_235 = 234,
    #[doc = "235: mclk frequency = 1/236 * hmclk frequency"]
    DIVIDE_236 = 235,
    #[doc = "236: mclk frequency = 1/237 * hmclk frequency"]
    DIVIDE_237 = 236,
    #[doc = "237: mclk frequency = 1/238 * hmclk frequency"]
    DIVIDE_238 = 237,
    #[doc = "238: mclk frequency = 1/239 * hmclk frequency"]
    DIVIDE_239 = 238,
    #[doc = "239: mclk frequency = 1/240 * hmclk frequency"]
    DIVIDE_240 = 239,
    #[doc = "240: mclk frequency = 1/241 * hmclk frequency"]
    DIVIDE_241 = 240,
    #[doc = "241: mclk frequency = 1/242 * hmclk frequency"]
    DIVIDE_242 = 241,
    #[doc = "242: mclk frequency = 1/243 * hmclk frequency"]
    DIVIDE_243 = 242,
    #[doc = "243: mclk frequency = 1/244 * hmclk frequency"]
    DIVIDE_244 = 243,
    #[doc = "244: mclk frequency = 1/245 * hmclk frequency"]
    DIVIDE_245 = 244,
    #[doc = "245: mclk frequency = 1/246 * hmclk frequency"]
    DIVIDE_246 = 245,
    #[doc = "246: mclk frequency = 1/247 * hmclk frequency"]
    DIVIDE_247 = 246,
    #[doc = "247: mclk frequency = 1/248 * hmclk frequency"]
    DIVIDE_248 = 247,
    #[doc = "248: mclk frequency = 1/249 * hmclk frequency"]
    DIVIDE_249 = 248,
    #[doc = "249: mclk frequency = 1/250 * hmclk frequency"]
    DIVIDE_250 = 249,
    #[doc = "250: mclk frequency = 1/251 * hmclk frequency"]
    DIVIDE_251 = 250,
    #[doc = "251: mclk frequency = 1/252 * hmclk frequency"]
    DIVIDE_252 = 251,
    #[doc = "252: mclk frequency = 1/253 * hmclk frequency"]
    DIVIDE_253 = 252,
    #[doc = "253: mclk frequency = 1/254 * hmclk frequency"]
    DIVIDE_254 = 253,
    #[doc = "254: mclk frequency = 1/255 * hmclk frequency"]
    DIVIDE_255 = 254,
    #[doc = "255: mclk frequency = 1/256 * hmclk frequency"]
    DIVIDE_256 = 255,
}
impl From<MQS_CLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MQS_CLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MQS_CLK_DIV`"]
pub type MQS_CLK_DIV_R = crate::R<u8, MQS_CLK_DIV_A>;
impl MQS_CLK_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MQS_CLK_DIV_A {
        match self.bits {
            0 => MQS_CLK_DIV_A::DIVIDE_1,
            1 => MQS_CLK_DIV_A::DIVIDE_2,
            2 => MQS_CLK_DIV_A::DIVIDE_3,
            3 => MQS_CLK_DIV_A::DIVIDE_4,
            4 => MQS_CLK_DIV_A::DIVIDE_5,
            5 => MQS_CLK_DIV_A::DIVIDE_6,
            6 => MQS_CLK_DIV_A::DIVIDE_7,
            7 => MQS_CLK_DIV_A::DIVIDE_8,
            8 => MQS_CLK_DIV_A::DIVIDE_9,
            9 => MQS_CLK_DIV_A::DIVIDE_10,
            10 => MQS_CLK_DIV_A::DIVIDE_11,
            11 => MQS_CLK_DIV_A::DIVIDE_12,
            12 => MQS_CLK_DIV_A::DIVIDE_13,
            13 => MQS_CLK_DIV_A::DIVIDE_14,
            14 => MQS_CLK_DIV_A::DIVIDE_15,
            15 => MQS_CLK_DIV_A::DIVIDE_16,
            16 => MQS_CLK_DIV_A::DIVIDE_17,
            17 => MQS_CLK_DIV_A::DIVIDE_18,
            18 => MQS_CLK_DIV_A::DIVIDE_19,
            19 => MQS_CLK_DIV_A::DIVIDE_20,
            20 => MQS_CLK_DIV_A::DIVIDE_21,
            21 => MQS_CLK_DIV_A::DIVIDE_22,
            22 => MQS_CLK_DIV_A::DIVIDE_23,
            23 => MQS_CLK_DIV_A::DIVIDE_24,
            24 => MQS_CLK_DIV_A::DIVIDE_25,
            25 => MQS_CLK_DIV_A::DIVIDE_26,
            26 => MQS_CLK_DIV_A::DIVIDE_27,
            27 => MQS_CLK_DIV_A::DIVIDE_28,
            28 => MQS_CLK_DIV_A::DIVIDE_29,
            29 => MQS_CLK_DIV_A::DIVIDE_30,
            30 => MQS_CLK_DIV_A::DIVIDE_31,
            31 => MQS_CLK_DIV_A::DIVIDE_32,
            32 => MQS_CLK_DIV_A::DIVIDE_33,
            33 => MQS_CLK_DIV_A::DIVIDE_34,
            34 => MQS_CLK_DIV_A::DIVIDE_35,
            35 => MQS_CLK_DIV_A::DIVIDE_36,
            36 => MQS_CLK_DIV_A::DIVIDE_37,
            37 => MQS_CLK_DIV_A::DIVIDE_38,
            38 => MQS_CLK_DIV_A::DIVIDE_39,
            39 => MQS_CLK_DIV_A::DIVIDE_40,
            40 => MQS_CLK_DIV_A::DIVIDE_41,
            41 => MQS_CLK_DIV_A::DIVIDE_42,
            42 => MQS_CLK_DIV_A::DIVIDE_43,
            43 => MQS_CLK_DIV_A::DIVIDE_44,
            44 => MQS_CLK_DIV_A::DIVIDE_45,
            45 => MQS_CLK_DIV_A::DIVIDE_46,
            46 => MQS_CLK_DIV_A::DIVIDE_47,
            47 => MQS_CLK_DIV_A::DIVIDE_48,
            48 => MQS_CLK_DIV_A::DIVIDE_49,
            49 => MQS_CLK_DIV_A::DIVIDE_50,
            50 => MQS_CLK_DIV_A::DIVIDE_51,
            51 => MQS_CLK_DIV_A::DIVIDE_52,
            52 => MQS_CLK_DIV_A::DIVIDE_53,
            53 => MQS_CLK_DIV_A::DIVIDE_54,
            54 => MQS_CLK_DIV_A::DIVIDE_55,
            55 => MQS_CLK_DIV_A::DIVIDE_56,
            56 => MQS_CLK_DIV_A::DIVIDE_57,
            57 => MQS_CLK_DIV_A::DIVIDE_58,
            58 => MQS_CLK_DIV_A::DIVIDE_59,
            59 => MQS_CLK_DIV_A::DIVIDE_60,
            60 => MQS_CLK_DIV_A::DIVIDE_61,
            61 => MQS_CLK_DIV_A::DIVIDE_62,
            62 => MQS_CLK_DIV_A::DIVIDE_63,
            63 => MQS_CLK_DIV_A::DIVIDE_64,
            64 => MQS_CLK_DIV_A::DIVIDE_65,
            65 => MQS_CLK_DIV_A::DIVIDE_66,
            66 => MQS_CLK_DIV_A::DIVIDE_67,
            67 => MQS_CLK_DIV_A::DIVIDE_68,
            68 => MQS_CLK_DIV_A::DIVIDE_69,
            69 => MQS_CLK_DIV_A::DIVIDE_70,
            70 => MQS_CLK_DIV_A::DIVIDE_71,
            71 => MQS_CLK_DIV_A::DIVIDE_72,
            72 => MQS_CLK_DIV_A::DIVIDE_73,
            73 => MQS_CLK_DIV_A::DIVIDE_74,
            74 => MQS_CLK_DIV_A::DIVIDE_75,
            75 => MQS_CLK_DIV_A::DIVIDE_76,
            76 => MQS_CLK_DIV_A::DIVIDE_77,
            77 => MQS_CLK_DIV_A::DIVIDE_78,
            78 => MQS_CLK_DIV_A::DIVIDE_79,
            79 => MQS_CLK_DIV_A::DIVIDE_80,
            80 => MQS_CLK_DIV_A::DIVIDE_81,
            81 => MQS_CLK_DIV_A::DIVIDE_82,
            82 => MQS_CLK_DIV_A::DIVIDE_83,
            83 => MQS_CLK_DIV_A::DIVIDE_84,
            84 => MQS_CLK_DIV_A::DIVIDE_85,
            85 => MQS_CLK_DIV_A::DIVIDE_86,
            86 => MQS_CLK_DIV_A::DIVIDE_87,
            87 => MQS_CLK_DIV_A::DIVIDE_88,
            88 => MQS_CLK_DIV_A::DIVIDE_89,
            89 => MQS_CLK_DIV_A::DIVIDE_90,
            90 => MQS_CLK_DIV_A::DIVIDE_91,
            91 => MQS_CLK_DIV_A::DIVIDE_92,
            92 => MQS_CLK_DIV_A::DIVIDE_93,
            93 => MQS_CLK_DIV_A::DIVIDE_94,
            94 => MQS_CLK_DIV_A::DIVIDE_95,
            95 => MQS_CLK_DIV_A::DIVIDE_96,
            96 => MQS_CLK_DIV_A::DIVIDE_97,
            97 => MQS_CLK_DIV_A::DIVIDE_98,
            98 => MQS_CLK_DIV_A::DIVIDE_99,
            99 => MQS_CLK_DIV_A::DIVIDE_100,
            100 => MQS_CLK_DIV_A::DIVIDE_101,
            101 => MQS_CLK_DIV_A::DIVIDE_102,
            102 => MQS_CLK_DIV_A::DIVIDE_103,
            103 => MQS_CLK_DIV_A::DIVIDE_104,
            104 => MQS_CLK_DIV_A::DIVIDE_105,
            105 => MQS_CLK_DIV_A::DIVIDE_106,
            106 => MQS_CLK_DIV_A::DIVIDE_107,
            107 => MQS_CLK_DIV_A::DIVIDE_108,
            108 => MQS_CLK_DIV_A::DIVIDE_109,
            109 => MQS_CLK_DIV_A::DIVIDE_110,
            110 => MQS_CLK_DIV_A::DIVIDE_111,
            111 => MQS_CLK_DIV_A::DIVIDE_112,
            112 => MQS_CLK_DIV_A::DIVIDE_113,
            113 => MQS_CLK_DIV_A::DIVIDE_114,
            114 => MQS_CLK_DIV_A::DIVIDE_115,
            115 => MQS_CLK_DIV_A::DIVIDE_116,
            116 => MQS_CLK_DIV_A::DIVIDE_117,
            117 => MQS_CLK_DIV_A::DIVIDE_118,
            118 => MQS_CLK_DIV_A::DIVIDE_119,
            119 => MQS_CLK_DIV_A::DIVIDE_120,
            120 => MQS_CLK_DIV_A::DIVIDE_121,
            121 => MQS_CLK_DIV_A::DIVIDE_122,
            122 => MQS_CLK_DIV_A::DIVIDE_123,
            123 => MQS_CLK_DIV_A::DIVIDE_124,
            124 => MQS_CLK_DIV_A::DIVIDE_125,
            125 => MQS_CLK_DIV_A::DIVIDE_126,
            126 => MQS_CLK_DIV_A::DIVIDE_127,
            127 => MQS_CLK_DIV_A::DIVIDE_128,
            128 => MQS_CLK_DIV_A::DIVIDE_129,
            129 => MQS_CLK_DIV_A::DIVIDE_130,
            130 => MQS_CLK_DIV_A::DIVIDE_131,
            131 => MQS_CLK_DIV_A::DIVIDE_132,
            132 => MQS_CLK_DIV_A::DIVIDE_133,
            133 => MQS_CLK_DIV_A::DIVIDE_134,
            134 => MQS_CLK_DIV_A::DIVIDE_135,
            135 => MQS_CLK_DIV_A::DIVIDE_136,
            136 => MQS_CLK_DIV_A::DIVIDE_137,
            137 => MQS_CLK_DIV_A::DIVIDE_138,
            138 => MQS_CLK_DIV_A::DIVIDE_139,
            139 => MQS_CLK_DIV_A::DIVIDE_140,
            140 => MQS_CLK_DIV_A::DIVIDE_141,
            141 => MQS_CLK_DIV_A::DIVIDE_142,
            142 => MQS_CLK_DIV_A::DIVIDE_143,
            143 => MQS_CLK_DIV_A::DIVIDE_144,
            144 => MQS_CLK_DIV_A::DIVIDE_145,
            145 => MQS_CLK_DIV_A::DIVIDE_146,
            146 => MQS_CLK_DIV_A::DIVIDE_147,
            147 => MQS_CLK_DIV_A::DIVIDE_148,
            148 => MQS_CLK_DIV_A::DIVIDE_149,
            149 => MQS_CLK_DIV_A::DIVIDE_150,
            150 => MQS_CLK_DIV_A::DIVIDE_151,
            151 => MQS_CLK_DIV_A::DIVIDE_152,
            152 => MQS_CLK_DIV_A::DIVIDE_153,
            153 => MQS_CLK_DIV_A::DIVIDE_154,
            154 => MQS_CLK_DIV_A::DIVIDE_155,
            155 => MQS_CLK_DIV_A::DIVIDE_156,
            156 => MQS_CLK_DIV_A::DIVIDE_157,
            157 => MQS_CLK_DIV_A::DIVIDE_158,
            158 => MQS_CLK_DIV_A::DIVIDE_159,
            159 => MQS_CLK_DIV_A::DIVIDE_160,
            160 => MQS_CLK_DIV_A::DIVIDE_161,
            161 => MQS_CLK_DIV_A::DIVIDE_162,
            162 => MQS_CLK_DIV_A::DIVIDE_163,
            163 => MQS_CLK_DIV_A::DIVIDE_164,
            164 => MQS_CLK_DIV_A::DIVIDE_165,
            165 => MQS_CLK_DIV_A::DIVIDE_166,
            166 => MQS_CLK_DIV_A::DIVIDE_167,
            167 => MQS_CLK_DIV_A::DIVIDE_168,
            168 => MQS_CLK_DIV_A::DIVIDE_169,
            169 => MQS_CLK_DIV_A::DIVIDE_170,
            170 => MQS_CLK_DIV_A::DIVIDE_171,
            171 => MQS_CLK_DIV_A::DIVIDE_172,
            172 => MQS_CLK_DIV_A::DIVIDE_173,
            173 => MQS_CLK_DIV_A::DIVIDE_174,
            174 => MQS_CLK_DIV_A::DIVIDE_175,
            175 => MQS_CLK_DIV_A::DIVIDE_176,
            176 => MQS_CLK_DIV_A::DIVIDE_177,
            177 => MQS_CLK_DIV_A::DIVIDE_178,
            178 => MQS_CLK_DIV_A::DIVIDE_179,
            179 => MQS_CLK_DIV_A::DIVIDE_180,
            180 => MQS_CLK_DIV_A::DIVIDE_181,
            181 => MQS_CLK_DIV_A::DIVIDE_182,
            182 => MQS_CLK_DIV_A::DIVIDE_183,
            183 => MQS_CLK_DIV_A::DIVIDE_184,
            184 => MQS_CLK_DIV_A::DIVIDE_185,
            185 => MQS_CLK_DIV_A::DIVIDE_186,
            186 => MQS_CLK_DIV_A::DIVIDE_187,
            187 => MQS_CLK_DIV_A::DIVIDE_188,
            188 => MQS_CLK_DIV_A::DIVIDE_189,
            189 => MQS_CLK_DIV_A::DIVIDE_190,
            190 => MQS_CLK_DIV_A::DIVIDE_191,
            191 => MQS_CLK_DIV_A::DIVIDE_192,
            192 => MQS_CLK_DIV_A::DIVIDE_193,
            193 => MQS_CLK_DIV_A::DIVIDE_194,
            194 => MQS_CLK_DIV_A::DIVIDE_195,
            195 => MQS_CLK_DIV_A::DIVIDE_196,
            196 => MQS_CLK_DIV_A::DIVIDE_197,
            197 => MQS_CLK_DIV_A::DIVIDE_198,
            198 => MQS_CLK_DIV_A::DIVIDE_199,
            199 => MQS_CLK_DIV_A::DIVIDE_200,
            200 => MQS_CLK_DIV_A::DIVIDE_201,
            201 => MQS_CLK_DIV_A::DIVIDE_202,
            202 => MQS_CLK_DIV_A::DIVIDE_203,
            203 => MQS_CLK_DIV_A::DIVIDE_204,
            204 => MQS_CLK_DIV_A::DIVIDE_205,
            205 => MQS_CLK_DIV_A::DIVIDE_206,
            206 => MQS_CLK_DIV_A::DIVIDE_207,
            207 => MQS_CLK_DIV_A::DIVIDE_208,
            208 => MQS_CLK_DIV_A::DIVIDE_209,
            209 => MQS_CLK_DIV_A::DIVIDE_210,
            210 => MQS_CLK_DIV_A::DIVIDE_211,
            211 => MQS_CLK_DIV_A::DIVIDE_212,
            212 => MQS_CLK_DIV_A::DIVIDE_213,
            213 => MQS_CLK_DIV_A::DIVIDE_214,
            214 => MQS_CLK_DIV_A::DIVIDE_215,
            215 => MQS_CLK_DIV_A::DIVIDE_216,
            216 => MQS_CLK_DIV_A::DIVIDE_217,
            217 => MQS_CLK_DIV_A::DIVIDE_218,
            218 => MQS_CLK_DIV_A::DIVIDE_219,
            219 => MQS_CLK_DIV_A::DIVIDE_220,
            220 => MQS_CLK_DIV_A::DIVIDE_221,
            221 => MQS_CLK_DIV_A::DIVIDE_222,
            222 => MQS_CLK_DIV_A::DIVIDE_223,
            223 => MQS_CLK_DIV_A::DIVIDE_224,
            224 => MQS_CLK_DIV_A::DIVIDE_225,
            225 => MQS_CLK_DIV_A::DIVIDE_226,
            226 => MQS_CLK_DIV_A::DIVIDE_227,
            227 => MQS_CLK_DIV_A::DIVIDE_228,
            228 => MQS_CLK_DIV_A::DIVIDE_229,
            229 => MQS_CLK_DIV_A::DIVIDE_230,
            230 => MQS_CLK_DIV_A::DIVIDE_231,
            231 => MQS_CLK_DIV_A::DIVIDE_232,
            232 => MQS_CLK_DIV_A::DIVIDE_233,
            233 => MQS_CLK_DIV_A::DIVIDE_234,
            234 => MQS_CLK_DIV_A::DIVIDE_235,
            235 => MQS_CLK_DIV_A::DIVIDE_236,
            236 => MQS_CLK_DIV_A::DIVIDE_237,
            237 => MQS_CLK_DIV_A::DIVIDE_238,
            238 => MQS_CLK_DIV_A::DIVIDE_239,
            239 => MQS_CLK_DIV_A::DIVIDE_240,
            240 => MQS_CLK_DIV_A::DIVIDE_241,
            241 => MQS_CLK_DIV_A::DIVIDE_242,
            242 => MQS_CLK_DIV_A::DIVIDE_243,
            243 => MQS_CLK_DIV_A::DIVIDE_244,
            244 => MQS_CLK_DIV_A::DIVIDE_245,
            245 => MQS_CLK_DIV_A::DIVIDE_246,
            246 => MQS_CLK_DIV_A::DIVIDE_247,
            247 => MQS_CLK_DIV_A::DIVIDE_248,
            248 => MQS_CLK_DIV_A::DIVIDE_249,
            249 => MQS_CLK_DIV_A::DIVIDE_250,
            250 => MQS_CLK_DIV_A::DIVIDE_251,
            251 => MQS_CLK_DIV_A::DIVIDE_252,
            252 => MQS_CLK_DIV_A::DIVIDE_253,
            253 => MQS_CLK_DIV_A::DIVIDE_254,
            254 => MQS_CLK_DIV_A::DIVIDE_255,
            255 => MQS_CLK_DIV_A::DIVIDE_256,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_8
    }
    #[doc = "Checks if the value of the field is `DIVIDE_9`"]
    #[inline(always)]
    pub fn is_divide_9(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_9
    }
    #[doc = "Checks if the value of the field is `DIVIDE_10`"]
    #[inline(always)]
    pub fn is_divide_10(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_10
    }
    #[doc = "Checks if the value of the field is `DIVIDE_11`"]
    #[inline(always)]
    pub fn is_divide_11(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_11
    }
    #[doc = "Checks if the value of the field is `DIVIDE_12`"]
    #[inline(always)]
    pub fn is_divide_12(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_12
    }
    #[doc = "Checks if the value of the field is `DIVIDE_13`"]
    #[inline(always)]
    pub fn is_divide_13(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_13
    }
    #[doc = "Checks if the value of the field is `DIVIDE_14`"]
    #[inline(always)]
    pub fn is_divide_14(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_14
    }
    #[doc = "Checks if the value of the field is `DIVIDE_15`"]
    #[inline(always)]
    pub fn is_divide_15(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_15
    }
    #[doc = "Checks if the value of the field is `DIVIDE_16`"]
    #[inline(always)]
    pub fn is_divide_16(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_16
    }
    #[doc = "Checks if the value of the field is `DIVIDE_17`"]
    #[inline(always)]
    pub fn is_divide_17(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_17
    }
    #[doc = "Checks if the value of the field is `DIVIDE_18`"]
    #[inline(always)]
    pub fn is_divide_18(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_18
    }
    #[doc = "Checks if the value of the field is `DIVIDE_19`"]
    #[inline(always)]
    pub fn is_divide_19(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_19
    }
    #[doc = "Checks if the value of the field is `DIVIDE_20`"]
    #[inline(always)]
    pub fn is_divide_20(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_20
    }
    #[doc = "Checks if the value of the field is `DIVIDE_21`"]
    #[inline(always)]
    pub fn is_divide_21(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_21
    }
    #[doc = "Checks if the value of the field is `DIVIDE_22`"]
    #[inline(always)]
    pub fn is_divide_22(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_22
    }
    #[doc = "Checks if the value of the field is `DIVIDE_23`"]
    #[inline(always)]
    pub fn is_divide_23(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_23
    }
    #[doc = "Checks if the value of the field is `DIVIDE_24`"]
    #[inline(always)]
    pub fn is_divide_24(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_24
    }
    #[doc = "Checks if the value of the field is `DIVIDE_25`"]
    #[inline(always)]
    pub fn is_divide_25(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_25
    }
    #[doc = "Checks if the value of the field is `DIVIDE_26`"]
    #[inline(always)]
    pub fn is_divide_26(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_26
    }
    #[doc = "Checks if the value of the field is `DIVIDE_27`"]
    #[inline(always)]
    pub fn is_divide_27(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_27
    }
    #[doc = "Checks if the value of the field is `DIVIDE_28`"]
    #[inline(always)]
    pub fn is_divide_28(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_28
    }
    #[doc = "Checks if the value of the field is `DIVIDE_29`"]
    #[inline(always)]
    pub fn is_divide_29(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_29
    }
    #[doc = "Checks if the value of the field is `DIVIDE_30`"]
    #[inline(always)]
    pub fn is_divide_30(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_30
    }
    #[doc = "Checks if the value of the field is `DIVIDE_31`"]
    #[inline(always)]
    pub fn is_divide_31(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_31
    }
    #[doc = "Checks if the value of the field is `DIVIDE_32`"]
    #[inline(always)]
    pub fn is_divide_32(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_32
    }
    #[doc = "Checks if the value of the field is `DIVIDE_33`"]
    #[inline(always)]
    pub fn is_divide_33(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_33
    }
    #[doc = "Checks if the value of the field is `DIVIDE_34`"]
    #[inline(always)]
    pub fn is_divide_34(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_34
    }
    #[doc = "Checks if the value of the field is `DIVIDE_35`"]
    #[inline(always)]
    pub fn is_divide_35(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_35
    }
    #[doc = "Checks if the value of the field is `DIVIDE_36`"]
    #[inline(always)]
    pub fn is_divide_36(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_36
    }
    #[doc = "Checks if the value of the field is `DIVIDE_37`"]
    #[inline(always)]
    pub fn is_divide_37(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_37
    }
    #[doc = "Checks if the value of the field is `DIVIDE_38`"]
    #[inline(always)]
    pub fn is_divide_38(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_38
    }
    #[doc = "Checks if the value of the field is `DIVIDE_39`"]
    #[inline(always)]
    pub fn is_divide_39(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_39
    }
    #[doc = "Checks if the value of the field is `DIVIDE_40`"]
    #[inline(always)]
    pub fn is_divide_40(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_40
    }
    #[doc = "Checks if the value of the field is `DIVIDE_41`"]
    #[inline(always)]
    pub fn is_divide_41(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_41
    }
    #[doc = "Checks if the value of the field is `DIVIDE_42`"]
    #[inline(always)]
    pub fn is_divide_42(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_42
    }
    #[doc = "Checks if the value of the field is `DIVIDE_43`"]
    #[inline(always)]
    pub fn is_divide_43(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_43
    }
    #[doc = "Checks if the value of the field is `DIVIDE_44`"]
    #[inline(always)]
    pub fn is_divide_44(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_44
    }
    #[doc = "Checks if the value of the field is `DIVIDE_45`"]
    #[inline(always)]
    pub fn is_divide_45(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_45
    }
    #[doc = "Checks if the value of the field is `DIVIDE_46`"]
    #[inline(always)]
    pub fn is_divide_46(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_46
    }
    #[doc = "Checks if the value of the field is `DIVIDE_47`"]
    #[inline(always)]
    pub fn is_divide_47(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_47
    }
    #[doc = "Checks if the value of the field is `DIVIDE_48`"]
    #[inline(always)]
    pub fn is_divide_48(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_48
    }
    #[doc = "Checks if the value of the field is `DIVIDE_49`"]
    #[inline(always)]
    pub fn is_divide_49(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_49
    }
    #[doc = "Checks if the value of the field is `DIVIDE_50`"]
    #[inline(always)]
    pub fn is_divide_50(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_50
    }
    #[doc = "Checks if the value of the field is `DIVIDE_51`"]
    #[inline(always)]
    pub fn is_divide_51(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_51
    }
    #[doc = "Checks if the value of the field is `DIVIDE_52`"]
    #[inline(always)]
    pub fn is_divide_52(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_52
    }
    #[doc = "Checks if the value of the field is `DIVIDE_53`"]
    #[inline(always)]
    pub fn is_divide_53(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_53
    }
    #[doc = "Checks if the value of the field is `DIVIDE_54`"]
    #[inline(always)]
    pub fn is_divide_54(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_54
    }
    #[doc = "Checks if the value of the field is `DIVIDE_55`"]
    #[inline(always)]
    pub fn is_divide_55(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_55
    }
    #[doc = "Checks if the value of the field is `DIVIDE_56`"]
    #[inline(always)]
    pub fn is_divide_56(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_56
    }
    #[doc = "Checks if the value of the field is `DIVIDE_57`"]
    #[inline(always)]
    pub fn is_divide_57(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_57
    }
    #[doc = "Checks if the value of the field is `DIVIDE_58`"]
    #[inline(always)]
    pub fn is_divide_58(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_58
    }
    #[doc = "Checks if the value of the field is `DIVIDE_59`"]
    #[inline(always)]
    pub fn is_divide_59(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_59
    }
    #[doc = "Checks if the value of the field is `DIVIDE_60`"]
    #[inline(always)]
    pub fn is_divide_60(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_60
    }
    #[doc = "Checks if the value of the field is `DIVIDE_61`"]
    #[inline(always)]
    pub fn is_divide_61(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_61
    }
    #[doc = "Checks if the value of the field is `DIVIDE_62`"]
    #[inline(always)]
    pub fn is_divide_62(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_62
    }
    #[doc = "Checks if the value of the field is `DIVIDE_63`"]
    #[inline(always)]
    pub fn is_divide_63(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_63
    }
    #[doc = "Checks if the value of the field is `DIVIDE_64`"]
    #[inline(always)]
    pub fn is_divide_64(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_64
    }
    #[doc = "Checks if the value of the field is `DIVIDE_65`"]
    #[inline(always)]
    pub fn is_divide_65(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_65
    }
    #[doc = "Checks if the value of the field is `DIVIDE_66`"]
    #[inline(always)]
    pub fn is_divide_66(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_66
    }
    #[doc = "Checks if the value of the field is `DIVIDE_67`"]
    #[inline(always)]
    pub fn is_divide_67(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_67
    }
    #[doc = "Checks if the value of the field is `DIVIDE_68`"]
    #[inline(always)]
    pub fn is_divide_68(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_68
    }
    #[doc = "Checks if the value of the field is `DIVIDE_69`"]
    #[inline(always)]
    pub fn is_divide_69(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_69
    }
    #[doc = "Checks if the value of the field is `DIVIDE_70`"]
    #[inline(always)]
    pub fn is_divide_70(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_70
    }
    #[doc = "Checks if the value of the field is `DIVIDE_71`"]
    #[inline(always)]
    pub fn is_divide_71(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_71
    }
    #[doc = "Checks if the value of the field is `DIVIDE_72`"]
    #[inline(always)]
    pub fn is_divide_72(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_72
    }
    #[doc = "Checks if the value of the field is `DIVIDE_73`"]
    #[inline(always)]
    pub fn is_divide_73(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_73
    }
    #[doc = "Checks if the value of the field is `DIVIDE_74`"]
    #[inline(always)]
    pub fn is_divide_74(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_74
    }
    #[doc = "Checks if the value of the field is `DIVIDE_75`"]
    #[inline(always)]
    pub fn is_divide_75(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_75
    }
    #[doc = "Checks if the value of the field is `DIVIDE_76`"]
    #[inline(always)]
    pub fn is_divide_76(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_76
    }
    #[doc = "Checks if the value of the field is `DIVIDE_77`"]
    #[inline(always)]
    pub fn is_divide_77(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_77
    }
    #[doc = "Checks if the value of the field is `DIVIDE_78`"]
    #[inline(always)]
    pub fn is_divide_78(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_78
    }
    #[doc = "Checks if the value of the field is `DIVIDE_79`"]
    #[inline(always)]
    pub fn is_divide_79(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_79
    }
    #[doc = "Checks if the value of the field is `DIVIDE_80`"]
    #[inline(always)]
    pub fn is_divide_80(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_80
    }
    #[doc = "Checks if the value of the field is `DIVIDE_81`"]
    #[inline(always)]
    pub fn is_divide_81(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_81
    }
    #[doc = "Checks if the value of the field is `DIVIDE_82`"]
    #[inline(always)]
    pub fn is_divide_82(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_82
    }
    #[doc = "Checks if the value of the field is `DIVIDE_83`"]
    #[inline(always)]
    pub fn is_divide_83(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_83
    }
    #[doc = "Checks if the value of the field is `DIVIDE_84`"]
    #[inline(always)]
    pub fn is_divide_84(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_84
    }
    #[doc = "Checks if the value of the field is `DIVIDE_85`"]
    #[inline(always)]
    pub fn is_divide_85(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_85
    }
    #[doc = "Checks if the value of the field is `DIVIDE_86`"]
    #[inline(always)]
    pub fn is_divide_86(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_86
    }
    #[doc = "Checks if the value of the field is `DIVIDE_87`"]
    #[inline(always)]
    pub fn is_divide_87(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_87
    }
    #[doc = "Checks if the value of the field is `DIVIDE_88`"]
    #[inline(always)]
    pub fn is_divide_88(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_88
    }
    #[doc = "Checks if the value of the field is `DIVIDE_89`"]
    #[inline(always)]
    pub fn is_divide_89(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_89
    }
    #[doc = "Checks if the value of the field is `DIVIDE_90`"]
    #[inline(always)]
    pub fn is_divide_90(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_90
    }
    #[doc = "Checks if the value of the field is `DIVIDE_91`"]
    #[inline(always)]
    pub fn is_divide_91(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_91
    }
    #[doc = "Checks if the value of the field is `DIVIDE_92`"]
    #[inline(always)]
    pub fn is_divide_92(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_92
    }
    #[doc = "Checks if the value of the field is `DIVIDE_93`"]
    #[inline(always)]
    pub fn is_divide_93(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_93
    }
    #[doc = "Checks if the value of the field is `DIVIDE_94`"]
    #[inline(always)]
    pub fn is_divide_94(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_94
    }
    #[doc = "Checks if the value of the field is `DIVIDE_95`"]
    #[inline(always)]
    pub fn is_divide_95(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_95
    }
    #[doc = "Checks if the value of the field is `DIVIDE_96`"]
    #[inline(always)]
    pub fn is_divide_96(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_96
    }
    #[doc = "Checks if the value of the field is `DIVIDE_97`"]
    #[inline(always)]
    pub fn is_divide_97(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_97
    }
    #[doc = "Checks if the value of the field is `DIVIDE_98`"]
    #[inline(always)]
    pub fn is_divide_98(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_98
    }
    #[doc = "Checks if the value of the field is `DIVIDE_99`"]
    #[inline(always)]
    pub fn is_divide_99(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_99
    }
    #[doc = "Checks if the value of the field is `DIVIDE_100`"]
    #[inline(always)]
    pub fn is_divide_100(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_100
    }
    #[doc = "Checks if the value of the field is `DIVIDE_101`"]
    #[inline(always)]
    pub fn is_divide_101(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_101
    }
    #[doc = "Checks if the value of the field is `DIVIDE_102`"]
    #[inline(always)]
    pub fn is_divide_102(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_102
    }
    #[doc = "Checks if the value of the field is `DIVIDE_103`"]
    #[inline(always)]
    pub fn is_divide_103(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_103
    }
    #[doc = "Checks if the value of the field is `DIVIDE_104`"]
    #[inline(always)]
    pub fn is_divide_104(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_104
    }
    #[doc = "Checks if the value of the field is `DIVIDE_105`"]
    #[inline(always)]
    pub fn is_divide_105(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_105
    }
    #[doc = "Checks if the value of the field is `DIVIDE_106`"]
    #[inline(always)]
    pub fn is_divide_106(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_106
    }
    #[doc = "Checks if the value of the field is `DIVIDE_107`"]
    #[inline(always)]
    pub fn is_divide_107(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_107
    }
    #[doc = "Checks if the value of the field is `DIVIDE_108`"]
    #[inline(always)]
    pub fn is_divide_108(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_108
    }
    #[doc = "Checks if the value of the field is `DIVIDE_109`"]
    #[inline(always)]
    pub fn is_divide_109(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_109
    }
    #[doc = "Checks if the value of the field is `DIVIDE_110`"]
    #[inline(always)]
    pub fn is_divide_110(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_110
    }
    #[doc = "Checks if the value of the field is `DIVIDE_111`"]
    #[inline(always)]
    pub fn is_divide_111(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_111
    }
    #[doc = "Checks if the value of the field is `DIVIDE_112`"]
    #[inline(always)]
    pub fn is_divide_112(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_112
    }
    #[doc = "Checks if the value of the field is `DIVIDE_113`"]
    #[inline(always)]
    pub fn is_divide_113(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_113
    }
    #[doc = "Checks if the value of the field is `DIVIDE_114`"]
    #[inline(always)]
    pub fn is_divide_114(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_114
    }
    #[doc = "Checks if the value of the field is `DIVIDE_115`"]
    #[inline(always)]
    pub fn is_divide_115(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_115
    }
    #[doc = "Checks if the value of the field is `DIVIDE_116`"]
    #[inline(always)]
    pub fn is_divide_116(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_116
    }
    #[doc = "Checks if the value of the field is `DIVIDE_117`"]
    #[inline(always)]
    pub fn is_divide_117(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_117
    }
    #[doc = "Checks if the value of the field is `DIVIDE_118`"]
    #[inline(always)]
    pub fn is_divide_118(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_118
    }
    #[doc = "Checks if the value of the field is `DIVIDE_119`"]
    #[inline(always)]
    pub fn is_divide_119(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_119
    }
    #[doc = "Checks if the value of the field is `DIVIDE_120`"]
    #[inline(always)]
    pub fn is_divide_120(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_120
    }
    #[doc = "Checks if the value of the field is `DIVIDE_121`"]
    #[inline(always)]
    pub fn is_divide_121(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_121
    }
    #[doc = "Checks if the value of the field is `DIVIDE_122`"]
    #[inline(always)]
    pub fn is_divide_122(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_122
    }
    #[doc = "Checks if the value of the field is `DIVIDE_123`"]
    #[inline(always)]
    pub fn is_divide_123(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_123
    }
    #[doc = "Checks if the value of the field is `DIVIDE_124`"]
    #[inline(always)]
    pub fn is_divide_124(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_124
    }
    #[doc = "Checks if the value of the field is `DIVIDE_125`"]
    #[inline(always)]
    pub fn is_divide_125(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_125
    }
    #[doc = "Checks if the value of the field is `DIVIDE_126`"]
    #[inline(always)]
    pub fn is_divide_126(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_126
    }
    #[doc = "Checks if the value of the field is `DIVIDE_127`"]
    #[inline(always)]
    pub fn is_divide_127(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_127
    }
    #[doc = "Checks if the value of the field is `DIVIDE_128`"]
    #[inline(always)]
    pub fn is_divide_128(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_128
    }
    #[doc = "Checks if the value of the field is `DIVIDE_129`"]
    #[inline(always)]
    pub fn is_divide_129(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_129
    }
    #[doc = "Checks if the value of the field is `DIVIDE_130`"]
    #[inline(always)]
    pub fn is_divide_130(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_130
    }
    #[doc = "Checks if the value of the field is `DIVIDE_131`"]
    #[inline(always)]
    pub fn is_divide_131(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_131
    }
    #[doc = "Checks if the value of the field is `DIVIDE_132`"]
    #[inline(always)]
    pub fn is_divide_132(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_132
    }
    #[doc = "Checks if the value of the field is `DIVIDE_133`"]
    #[inline(always)]
    pub fn is_divide_133(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_133
    }
    #[doc = "Checks if the value of the field is `DIVIDE_134`"]
    #[inline(always)]
    pub fn is_divide_134(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_134
    }
    #[doc = "Checks if the value of the field is `DIVIDE_135`"]
    #[inline(always)]
    pub fn is_divide_135(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_135
    }
    #[doc = "Checks if the value of the field is `DIVIDE_136`"]
    #[inline(always)]
    pub fn is_divide_136(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_136
    }
    #[doc = "Checks if the value of the field is `DIVIDE_137`"]
    #[inline(always)]
    pub fn is_divide_137(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_137
    }
    #[doc = "Checks if the value of the field is `DIVIDE_138`"]
    #[inline(always)]
    pub fn is_divide_138(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_138
    }
    #[doc = "Checks if the value of the field is `DIVIDE_139`"]
    #[inline(always)]
    pub fn is_divide_139(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_139
    }
    #[doc = "Checks if the value of the field is `DIVIDE_140`"]
    #[inline(always)]
    pub fn is_divide_140(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_140
    }
    #[doc = "Checks if the value of the field is `DIVIDE_141`"]
    #[inline(always)]
    pub fn is_divide_141(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_141
    }
    #[doc = "Checks if the value of the field is `DIVIDE_142`"]
    #[inline(always)]
    pub fn is_divide_142(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_142
    }
    #[doc = "Checks if the value of the field is `DIVIDE_143`"]
    #[inline(always)]
    pub fn is_divide_143(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_143
    }
    #[doc = "Checks if the value of the field is `DIVIDE_144`"]
    #[inline(always)]
    pub fn is_divide_144(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_144
    }
    #[doc = "Checks if the value of the field is `DIVIDE_145`"]
    #[inline(always)]
    pub fn is_divide_145(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_145
    }
    #[doc = "Checks if the value of the field is `DIVIDE_146`"]
    #[inline(always)]
    pub fn is_divide_146(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_146
    }
    #[doc = "Checks if the value of the field is `DIVIDE_147`"]
    #[inline(always)]
    pub fn is_divide_147(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_147
    }
    #[doc = "Checks if the value of the field is `DIVIDE_148`"]
    #[inline(always)]
    pub fn is_divide_148(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_148
    }
    #[doc = "Checks if the value of the field is `DIVIDE_149`"]
    #[inline(always)]
    pub fn is_divide_149(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_149
    }
    #[doc = "Checks if the value of the field is `DIVIDE_150`"]
    #[inline(always)]
    pub fn is_divide_150(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_150
    }
    #[doc = "Checks if the value of the field is `DIVIDE_151`"]
    #[inline(always)]
    pub fn is_divide_151(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_151
    }
    #[doc = "Checks if the value of the field is `DIVIDE_152`"]
    #[inline(always)]
    pub fn is_divide_152(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_152
    }
    #[doc = "Checks if the value of the field is `DIVIDE_153`"]
    #[inline(always)]
    pub fn is_divide_153(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_153
    }
    #[doc = "Checks if the value of the field is `DIVIDE_154`"]
    #[inline(always)]
    pub fn is_divide_154(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_154
    }
    #[doc = "Checks if the value of the field is `DIVIDE_155`"]
    #[inline(always)]
    pub fn is_divide_155(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_155
    }
    #[doc = "Checks if the value of the field is `DIVIDE_156`"]
    #[inline(always)]
    pub fn is_divide_156(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_156
    }
    #[doc = "Checks if the value of the field is `DIVIDE_157`"]
    #[inline(always)]
    pub fn is_divide_157(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_157
    }
    #[doc = "Checks if the value of the field is `DIVIDE_158`"]
    #[inline(always)]
    pub fn is_divide_158(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_158
    }
    #[doc = "Checks if the value of the field is `DIVIDE_159`"]
    #[inline(always)]
    pub fn is_divide_159(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_159
    }
    #[doc = "Checks if the value of the field is `DIVIDE_160`"]
    #[inline(always)]
    pub fn is_divide_160(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_160
    }
    #[doc = "Checks if the value of the field is `DIVIDE_161`"]
    #[inline(always)]
    pub fn is_divide_161(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_161
    }
    #[doc = "Checks if the value of the field is `DIVIDE_162`"]
    #[inline(always)]
    pub fn is_divide_162(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_162
    }
    #[doc = "Checks if the value of the field is `DIVIDE_163`"]
    #[inline(always)]
    pub fn is_divide_163(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_163
    }
    #[doc = "Checks if the value of the field is `DIVIDE_164`"]
    #[inline(always)]
    pub fn is_divide_164(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_164
    }
    #[doc = "Checks if the value of the field is `DIVIDE_165`"]
    #[inline(always)]
    pub fn is_divide_165(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_165
    }
    #[doc = "Checks if the value of the field is `DIVIDE_166`"]
    #[inline(always)]
    pub fn is_divide_166(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_166
    }
    #[doc = "Checks if the value of the field is `DIVIDE_167`"]
    #[inline(always)]
    pub fn is_divide_167(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_167
    }
    #[doc = "Checks if the value of the field is `DIVIDE_168`"]
    #[inline(always)]
    pub fn is_divide_168(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_168
    }
    #[doc = "Checks if the value of the field is `DIVIDE_169`"]
    #[inline(always)]
    pub fn is_divide_169(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_169
    }
    #[doc = "Checks if the value of the field is `DIVIDE_170`"]
    #[inline(always)]
    pub fn is_divide_170(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_170
    }
    #[doc = "Checks if the value of the field is `DIVIDE_171`"]
    #[inline(always)]
    pub fn is_divide_171(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_171
    }
    #[doc = "Checks if the value of the field is `DIVIDE_172`"]
    #[inline(always)]
    pub fn is_divide_172(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_172
    }
    #[doc = "Checks if the value of the field is `DIVIDE_173`"]
    #[inline(always)]
    pub fn is_divide_173(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_173
    }
    #[doc = "Checks if the value of the field is `DIVIDE_174`"]
    #[inline(always)]
    pub fn is_divide_174(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_174
    }
    #[doc = "Checks if the value of the field is `DIVIDE_175`"]
    #[inline(always)]
    pub fn is_divide_175(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_175
    }
    #[doc = "Checks if the value of the field is `DIVIDE_176`"]
    #[inline(always)]
    pub fn is_divide_176(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_176
    }
    #[doc = "Checks if the value of the field is `DIVIDE_177`"]
    #[inline(always)]
    pub fn is_divide_177(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_177
    }
    #[doc = "Checks if the value of the field is `DIVIDE_178`"]
    #[inline(always)]
    pub fn is_divide_178(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_178
    }
    #[doc = "Checks if the value of the field is `DIVIDE_179`"]
    #[inline(always)]
    pub fn is_divide_179(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_179
    }
    #[doc = "Checks if the value of the field is `DIVIDE_180`"]
    #[inline(always)]
    pub fn is_divide_180(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_180
    }
    #[doc = "Checks if the value of the field is `DIVIDE_181`"]
    #[inline(always)]
    pub fn is_divide_181(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_181
    }
    #[doc = "Checks if the value of the field is `DIVIDE_182`"]
    #[inline(always)]
    pub fn is_divide_182(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_182
    }
    #[doc = "Checks if the value of the field is `DIVIDE_183`"]
    #[inline(always)]
    pub fn is_divide_183(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_183
    }
    #[doc = "Checks if the value of the field is `DIVIDE_184`"]
    #[inline(always)]
    pub fn is_divide_184(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_184
    }
    #[doc = "Checks if the value of the field is `DIVIDE_185`"]
    #[inline(always)]
    pub fn is_divide_185(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_185
    }
    #[doc = "Checks if the value of the field is `DIVIDE_186`"]
    #[inline(always)]
    pub fn is_divide_186(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_186
    }
    #[doc = "Checks if the value of the field is `DIVIDE_187`"]
    #[inline(always)]
    pub fn is_divide_187(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_187
    }
    #[doc = "Checks if the value of the field is `DIVIDE_188`"]
    #[inline(always)]
    pub fn is_divide_188(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_188
    }
    #[doc = "Checks if the value of the field is `DIVIDE_189`"]
    #[inline(always)]
    pub fn is_divide_189(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_189
    }
    #[doc = "Checks if the value of the field is `DIVIDE_190`"]
    #[inline(always)]
    pub fn is_divide_190(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_190
    }
    #[doc = "Checks if the value of the field is `DIVIDE_191`"]
    #[inline(always)]
    pub fn is_divide_191(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_191
    }
    #[doc = "Checks if the value of the field is `DIVIDE_192`"]
    #[inline(always)]
    pub fn is_divide_192(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_192
    }
    #[doc = "Checks if the value of the field is `DIVIDE_193`"]
    #[inline(always)]
    pub fn is_divide_193(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_193
    }
    #[doc = "Checks if the value of the field is `DIVIDE_194`"]
    #[inline(always)]
    pub fn is_divide_194(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_194
    }
    #[doc = "Checks if the value of the field is `DIVIDE_195`"]
    #[inline(always)]
    pub fn is_divide_195(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_195
    }
    #[doc = "Checks if the value of the field is `DIVIDE_196`"]
    #[inline(always)]
    pub fn is_divide_196(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_196
    }
    #[doc = "Checks if the value of the field is `DIVIDE_197`"]
    #[inline(always)]
    pub fn is_divide_197(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_197
    }
    #[doc = "Checks if the value of the field is `DIVIDE_198`"]
    #[inline(always)]
    pub fn is_divide_198(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_198
    }
    #[doc = "Checks if the value of the field is `DIVIDE_199`"]
    #[inline(always)]
    pub fn is_divide_199(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_199
    }
    #[doc = "Checks if the value of the field is `DIVIDE_200`"]
    #[inline(always)]
    pub fn is_divide_200(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_200
    }
    #[doc = "Checks if the value of the field is `DIVIDE_201`"]
    #[inline(always)]
    pub fn is_divide_201(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_201
    }
    #[doc = "Checks if the value of the field is `DIVIDE_202`"]
    #[inline(always)]
    pub fn is_divide_202(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_202
    }
    #[doc = "Checks if the value of the field is `DIVIDE_203`"]
    #[inline(always)]
    pub fn is_divide_203(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_203
    }
    #[doc = "Checks if the value of the field is `DIVIDE_204`"]
    #[inline(always)]
    pub fn is_divide_204(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_204
    }
    #[doc = "Checks if the value of the field is `DIVIDE_205`"]
    #[inline(always)]
    pub fn is_divide_205(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_205
    }
    #[doc = "Checks if the value of the field is `DIVIDE_206`"]
    #[inline(always)]
    pub fn is_divide_206(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_206
    }
    #[doc = "Checks if the value of the field is `DIVIDE_207`"]
    #[inline(always)]
    pub fn is_divide_207(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_207
    }
    #[doc = "Checks if the value of the field is `DIVIDE_208`"]
    #[inline(always)]
    pub fn is_divide_208(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_208
    }
    #[doc = "Checks if the value of the field is `DIVIDE_209`"]
    #[inline(always)]
    pub fn is_divide_209(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_209
    }
    #[doc = "Checks if the value of the field is `DIVIDE_210`"]
    #[inline(always)]
    pub fn is_divide_210(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_210
    }
    #[doc = "Checks if the value of the field is `DIVIDE_211`"]
    #[inline(always)]
    pub fn is_divide_211(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_211
    }
    #[doc = "Checks if the value of the field is `DIVIDE_212`"]
    #[inline(always)]
    pub fn is_divide_212(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_212
    }
    #[doc = "Checks if the value of the field is `DIVIDE_213`"]
    #[inline(always)]
    pub fn is_divide_213(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_213
    }
    #[doc = "Checks if the value of the field is `DIVIDE_214`"]
    #[inline(always)]
    pub fn is_divide_214(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_214
    }
    #[doc = "Checks if the value of the field is `DIVIDE_215`"]
    #[inline(always)]
    pub fn is_divide_215(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_215
    }
    #[doc = "Checks if the value of the field is `DIVIDE_216`"]
    #[inline(always)]
    pub fn is_divide_216(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_216
    }
    #[doc = "Checks if the value of the field is `DIVIDE_217`"]
    #[inline(always)]
    pub fn is_divide_217(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_217
    }
    #[doc = "Checks if the value of the field is `DIVIDE_218`"]
    #[inline(always)]
    pub fn is_divide_218(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_218
    }
    #[doc = "Checks if the value of the field is `DIVIDE_219`"]
    #[inline(always)]
    pub fn is_divide_219(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_219
    }
    #[doc = "Checks if the value of the field is `DIVIDE_220`"]
    #[inline(always)]
    pub fn is_divide_220(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_220
    }
    #[doc = "Checks if the value of the field is `DIVIDE_221`"]
    #[inline(always)]
    pub fn is_divide_221(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_221
    }
    #[doc = "Checks if the value of the field is `DIVIDE_222`"]
    #[inline(always)]
    pub fn is_divide_222(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_222
    }
    #[doc = "Checks if the value of the field is `DIVIDE_223`"]
    #[inline(always)]
    pub fn is_divide_223(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_223
    }
    #[doc = "Checks if the value of the field is `DIVIDE_224`"]
    #[inline(always)]
    pub fn is_divide_224(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_224
    }
    #[doc = "Checks if the value of the field is `DIVIDE_225`"]
    #[inline(always)]
    pub fn is_divide_225(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_225
    }
    #[doc = "Checks if the value of the field is `DIVIDE_226`"]
    #[inline(always)]
    pub fn is_divide_226(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_226
    }
    #[doc = "Checks if the value of the field is `DIVIDE_227`"]
    #[inline(always)]
    pub fn is_divide_227(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_227
    }
    #[doc = "Checks if the value of the field is `DIVIDE_228`"]
    #[inline(always)]
    pub fn is_divide_228(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_228
    }
    #[doc = "Checks if the value of the field is `DIVIDE_229`"]
    #[inline(always)]
    pub fn is_divide_229(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_229
    }
    #[doc = "Checks if the value of the field is `DIVIDE_230`"]
    #[inline(always)]
    pub fn is_divide_230(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_230
    }
    #[doc = "Checks if the value of the field is `DIVIDE_231`"]
    #[inline(always)]
    pub fn is_divide_231(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_231
    }
    #[doc = "Checks if the value of the field is `DIVIDE_232`"]
    #[inline(always)]
    pub fn is_divide_232(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_232
    }
    #[doc = "Checks if the value of the field is `DIVIDE_233`"]
    #[inline(always)]
    pub fn is_divide_233(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_233
    }
    #[doc = "Checks if the value of the field is `DIVIDE_234`"]
    #[inline(always)]
    pub fn is_divide_234(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_234
    }
    #[doc = "Checks if the value of the field is `DIVIDE_235`"]
    #[inline(always)]
    pub fn is_divide_235(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_235
    }
    #[doc = "Checks if the value of the field is `DIVIDE_236`"]
    #[inline(always)]
    pub fn is_divide_236(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_236
    }
    #[doc = "Checks if the value of the field is `DIVIDE_237`"]
    #[inline(always)]
    pub fn is_divide_237(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_237
    }
    #[doc = "Checks if the value of the field is `DIVIDE_238`"]
    #[inline(always)]
    pub fn is_divide_238(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_238
    }
    #[doc = "Checks if the value of the field is `DIVIDE_239`"]
    #[inline(always)]
    pub fn is_divide_239(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_239
    }
    #[doc = "Checks if the value of the field is `DIVIDE_240`"]
    #[inline(always)]
    pub fn is_divide_240(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_240
    }
    #[doc = "Checks if the value of the field is `DIVIDE_241`"]
    #[inline(always)]
    pub fn is_divide_241(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_241
    }
    #[doc = "Checks if the value of the field is `DIVIDE_242`"]
    #[inline(always)]
    pub fn is_divide_242(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_242
    }
    #[doc = "Checks if the value of the field is `DIVIDE_243`"]
    #[inline(always)]
    pub fn is_divide_243(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_243
    }
    #[doc = "Checks if the value of the field is `DIVIDE_244`"]
    #[inline(always)]
    pub fn is_divide_244(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_244
    }
    #[doc = "Checks if the value of the field is `DIVIDE_245`"]
    #[inline(always)]
    pub fn is_divide_245(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_245
    }
    #[doc = "Checks if the value of the field is `DIVIDE_246`"]
    #[inline(always)]
    pub fn is_divide_246(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_246
    }
    #[doc = "Checks if the value of the field is `DIVIDE_247`"]
    #[inline(always)]
    pub fn is_divide_247(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_247
    }
    #[doc = "Checks if the value of the field is `DIVIDE_248`"]
    #[inline(always)]
    pub fn is_divide_248(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_248
    }
    #[doc = "Checks if the value of the field is `DIVIDE_249`"]
    #[inline(always)]
    pub fn is_divide_249(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_249
    }
    #[doc = "Checks if the value of the field is `DIVIDE_250`"]
    #[inline(always)]
    pub fn is_divide_250(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_250
    }
    #[doc = "Checks if the value of the field is `DIVIDE_251`"]
    #[inline(always)]
    pub fn is_divide_251(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_251
    }
    #[doc = "Checks if the value of the field is `DIVIDE_252`"]
    #[inline(always)]
    pub fn is_divide_252(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_252
    }
    #[doc = "Checks if the value of the field is `DIVIDE_253`"]
    #[inline(always)]
    pub fn is_divide_253(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_253
    }
    #[doc = "Checks if the value of the field is `DIVIDE_254`"]
    #[inline(always)]
    pub fn is_divide_254(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_254
    }
    #[doc = "Checks if the value of the field is `DIVIDE_255`"]
    #[inline(always)]
    pub fn is_divide_255(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_255
    }
    #[doc = "Checks if the value of the field is `DIVIDE_256`"]
    #[inline(always)]
    pub fn is_divide_256(&self) -> bool {
        *self == MQS_CLK_DIV_A::DIVIDE_256
    }
}
#[doc = "Write proxy for field `MQS_CLK_DIV`"]
pub struct MQS_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MQS_CLK_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MQS_CLK_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "mclk frequency = 1/1 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_1)
    }
    #[doc = "mclk frequency = 1/2 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_2)
    }
    #[doc = "mclk frequency = 1/3 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_3)
    }
    #[doc = "mclk frequency = 1/4 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_4)
    }
    #[doc = "mclk frequency = 1/5 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_5)
    }
    #[doc = "mclk frequency = 1/6 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_6)
    }
    #[doc = "mclk frequency = 1/7 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_7)
    }
    #[doc = "mclk frequency = 1/8 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_8)
    }
    #[doc = "mclk frequency = 1/9 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_9(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_9)
    }
    #[doc = "mclk frequency = 1/10 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_10(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_10)
    }
    #[doc = "mclk frequency = 1/11 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_11(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_11)
    }
    #[doc = "mclk frequency = 1/12 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_12(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_12)
    }
    #[doc = "mclk frequency = 1/13 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_13(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_13)
    }
    #[doc = "mclk frequency = 1/14 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_14(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_14)
    }
    #[doc = "mclk frequency = 1/15 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_15(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_15)
    }
    #[doc = "mclk frequency = 1/16 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_16(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_16)
    }
    #[doc = "mclk frequency = 1/17 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_17(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_17)
    }
    #[doc = "mclk frequency = 1/18 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_18(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_18)
    }
    #[doc = "mclk frequency = 1/19 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_19(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_19)
    }
    #[doc = "mclk frequency = 1/20 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_20(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_20)
    }
    #[doc = "mclk frequency = 1/21 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_21(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_21)
    }
    #[doc = "mclk frequency = 1/22 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_22(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_22)
    }
    #[doc = "mclk frequency = 1/23 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_23(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_23)
    }
    #[doc = "mclk frequency = 1/24 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_24(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_24)
    }
    #[doc = "mclk frequency = 1/25 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_25(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_25)
    }
    #[doc = "mclk frequency = 1/26 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_26(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_26)
    }
    #[doc = "mclk frequency = 1/27 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_27(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_27)
    }
    #[doc = "mclk frequency = 1/28 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_28(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_28)
    }
    #[doc = "mclk frequency = 1/29 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_29(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_29)
    }
    #[doc = "mclk frequency = 1/30 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_30(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_30)
    }
    #[doc = "mclk frequency = 1/31 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_31(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_31)
    }
    #[doc = "mclk frequency = 1/32 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_32(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_32)
    }
    #[doc = "mclk frequency = 1/33 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_33(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_33)
    }
    #[doc = "mclk frequency = 1/34 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_34(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_34)
    }
    #[doc = "mclk frequency = 1/35 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_35(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_35)
    }
    #[doc = "mclk frequency = 1/36 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_36(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_36)
    }
    #[doc = "mclk frequency = 1/37 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_37(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_37)
    }
    #[doc = "mclk frequency = 1/38 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_38(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_38)
    }
    #[doc = "mclk frequency = 1/39 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_39(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_39)
    }
    #[doc = "mclk frequency = 1/40 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_40(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_40)
    }
    #[doc = "mclk frequency = 1/41 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_41(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_41)
    }
    #[doc = "mclk frequency = 1/42 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_42(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_42)
    }
    #[doc = "mclk frequency = 1/43 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_43(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_43)
    }
    #[doc = "mclk frequency = 1/44 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_44(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_44)
    }
    #[doc = "mclk frequency = 1/45 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_45(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_45)
    }
    #[doc = "mclk frequency = 1/46 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_46(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_46)
    }
    #[doc = "mclk frequency = 1/47 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_47(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_47)
    }
    #[doc = "mclk frequency = 1/48 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_48(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_48)
    }
    #[doc = "mclk frequency = 1/49 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_49(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_49)
    }
    #[doc = "mclk frequency = 1/50 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_50(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_50)
    }
    #[doc = "mclk frequency = 1/51 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_51(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_51)
    }
    #[doc = "mclk frequency = 1/52 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_52(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_52)
    }
    #[doc = "mclk frequency = 1/53 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_53(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_53)
    }
    #[doc = "mclk frequency = 1/54 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_54(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_54)
    }
    #[doc = "mclk frequency = 1/55 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_55(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_55)
    }
    #[doc = "mclk frequency = 1/56 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_56(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_56)
    }
    #[doc = "mclk frequency = 1/57 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_57(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_57)
    }
    #[doc = "mclk frequency = 1/58 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_58(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_58)
    }
    #[doc = "mclk frequency = 1/59 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_59(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_59)
    }
    #[doc = "mclk frequency = 1/60 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_60(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_60)
    }
    #[doc = "mclk frequency = 1/61 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_61(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_61)
    }
    #[doc = "mclk frequency = 1/62 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_62(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_62)
    }
    #[doc = "mclk frequency = 1/63 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_63(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_63)
    }
    #[doc = "mclk frequency = 1/64 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_64(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_64)
    }
    #[doc = "mclk frequency = 1/65 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_65(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_65)
    }
    #[doc = "mclk frequency = 1/66 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_66(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_66)
    }
    #[doc = "mclk frequency = 1/67 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_67(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_67)
    }
    #[doc = "mclk frequency = 1/68 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_68(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_68)
    }
    #[doc = "mclk frequency = 1/69 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_69(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_69)
    }
    #[doc = "mclk frequency = 1/70 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_70(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_70)
    }
    #[doc = "mclk frequency = 1/71 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_71(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_71)
    }
    #[doc = "mclk frequency = 1/72 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_72(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_72)
    }
    #[doc = "mclk frequency = 1/73 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_73(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_73)
    }
    #[doc = "mclk frequency = 1/74 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_74(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_74)
    }
    #[doc = "mclk frequency = 1/75 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_75(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_75)
    }
    #[doc = "mclk frequency = 1/76 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_76(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_76)
    }
    #[doc = "mclk frequency = 1/77 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_77(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_77)
    }
    #[doc = "mclk frequency = 1/78 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_78(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_78)
    }
    #[doc = "mclk frequency = 1/79 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_79(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_79)
    }
    #[doc = "mclk frequency = 1/80 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_80(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_80)
    }
    #[doc = "mclk frequency = 1/81 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_81(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_81)
    }
    #[doc = "mclk frequency = 1/82 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_82(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_82)
    }
    #[doc = "mclk frequency = 1/83 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_83(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_83)
    }
    #[doc = "mclk frequency = 1/84 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_84(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_84)
    }
    #[doc = "mclk frequency = 1/85 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_85(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_85)
    }
    #[doc = "mclk frequency = 1/86 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_86(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_86)
    }
    #[doc = "mclk frequency = 1/87 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_87(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_87)
    }
    #[doc = "mclk frequency = 1/88 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_88(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_88)
    }
    #[doc = "mclk frequency = 1/89 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_89(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_89)
    }
    #[doc = "mclk frequency = 1/90 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_90(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_90)
    }
    #[doc = "mclk frequency = 1/91 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_91(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_91)
    }
    #[doc = "mclk frequency = 1/92 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_92(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_92)
    }
    #[doc = "mclk frequency = 1/93 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_93(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_93)
    }
    #[doc = "mclk frequency = 1/94 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_94(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_94)
    }
    #[doc = "mclk frequency = 1/95 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_95(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_95)
    }
    #[doc = "mclk frequency = 1/96 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_96(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_96)
    }
    #[doc = "mclk frequency = 1/97 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_97(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_97)
    }
    #[doc = "mclk frequency = 1/98 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_98(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_98)
    }
    #[doc = "mclk frequency = 1/99 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_99(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_99)
    }
    #[doc = "mclk frequency = 1/100 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_100(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_100)
    }
    #[doc = "mclk frequency = 1/101 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_101(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_101)
    }
    #[doc = "mclk frequency = 1/102 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_102(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_102)
    }
    #[doc = "mclk frequency = 1/103 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_103(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_103)
    }
    #[doc = "mclk frequency = 1/104 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_104(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_104)
    }
    #[doc = "mclk frequency = 1/105 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_105(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_105)
    }
    #[doc = "mclk frequency = 1/106 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_106(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_106)
    }
    #[doc = "mclk frequency = 1/107 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_107(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_107)
    }
    #[doc = "mclk frequency = 1/108 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_108(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_108)
    }
    #[doc = "mclk frequency = 1/109 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_109(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_109)
    }
    #[doc = "mclk frequency = 1/110 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_110(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_110)
    }
    #[doc = "mclk frequency = 1/111 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_111(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_111)
    }
    #[doc = "mclk frequency = 1/112 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_112(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_112)
    }
    #[doc = "mclk frequency = 1/113 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_113(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_113)
    }
    #[doc = "mclk frequency = 1/114 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_114(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_114)
    }
    #[doc = "mclk frequency = 1/115 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_115(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_115)
    }
    #[doc = "mclk frequency = 1/116 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_116(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_116)
    }
    #[doc = "mclk frequency = 1/117 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_117(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_117)
    }
    #[doc = "mclk frequency = 1/118 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_118(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_118)
    }
    #[doc = "mclk frequency = 1/119 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_119(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_119)
    }
    #[doc = "mclk frequency = 1/120 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_120(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_120)
    }
    #[doc = "mclk frequency = 1/121 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_121(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_121)
    }
    #[doc = "mclk frequency = 1/122 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_122(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_122)
    }
    #[doc = "mclk frequency = 1/123 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_123(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_123)
    }
    #[doc = "mclk frequency = 1/124 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_124(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_124)
    }
    #[doc = "mclk frequency = 1/125 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_125(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_125)
    }
    #[doc = "mclk frequency = 1/126 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_126(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_126)
    }
    #[doc = "mclk frequency = 1/127 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_127(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_127)
    }
    #[doc = "mclk frequency = 1/128 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_128(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_128)
    }
    #[doc = "mclk frequency = 1/129 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_129(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_129)
    }
    #[doc = "mclk frequency = 1/130 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_130(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_130)
    }
    #[doc = "mclk frequency = 1/131 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_131(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_131)
    }
    #[doc = "mclk frequency = 1/132 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_132(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_132)
    }
    #[doc = "mclk frequency = 1/133 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_133(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_133)
    }
    #[doc = "mclk frequency = 1/134 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_134(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_134)
    }
    #[doc = "mclk frequency = 1/135 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_135(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_135)
    }
    #[doc = "mclk frequency = 1/136 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_136(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_136)
    }
    #[doc = "mclk frequency = 1/137 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_137(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_137)
    }
    #[doc = "mclk frequency = 1/138 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_138(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_138)
    }
    #[doc = "mclk frequency = 1/139 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_139(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_139)
    }
    #[doc = "mclk frequency = 1/140 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_140(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_140)
    }
    #[doc = "mclk frequency = 1/141 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_141(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_141)
    }
    #[doc = "mclk frequency = 1/142 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_142(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_142)
    }
    #[doc = "mclk frequency = 1/143 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_143(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_143)
    }
    #[doc = "mclk frequency = 1/144 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_144(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_144)
    }
    #[doc = "mclk frequency = 1/145 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_145(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_145)
    }
    #[doc = "mclk frequency = 1/146 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_146(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_146)
    }
    #[doc = "mclk frequency = 1/147 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_147(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_147)
    }
    #[doc = "mclk frequency = 1/148 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_148(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_148)
    }
    #[doc = "mclk frequency = 1/149 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_149(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_149)
    }
    #[doc = "mclk frequency = 1/150 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_150(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_150)
    }
    #[doc = "mclk frequency = 1/151 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_151(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_151)
    }
    #[doc = "mclk frequency = 1/152 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_152(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_152)
    }
    #[doc = "mclk frequency = 1/153 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_153(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_153)
    }
    #[doc = "mclk frequency = 1/154 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_154(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_154)
    }
    #[doc = "mclk frequency = 1/155 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_155(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_155)
    }
    #[doc = "mclk frequency = 1/156 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_156(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_156)
    }
    #[doc = "mclk frequency = 1/157 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_157(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_157)
    }
    #[doc = "mclk frequency = 1/158 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_158(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_158)
    }
    #[doc = "mclk frequency = 1/159 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_159(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_159)
    }
    #[doc = "mclk frequency = 1/160 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_160(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_160)
    }
    #[doc = "mclk frequency = 1/161 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_161(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_161)
    }
    #[doc = "mclk frequency = 1/162 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_162(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_162)
    }
    #[doc = "mclk frequency = 1/163 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_163(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_163)
    }
    #[doc = "mclk frequency = 1/164 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_164(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_164)
    }
    #[doc = "mclk frequency = 1/165 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_165(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_165)
    }
    #[doc = "mclk frequency = 1/166 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_166(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_166)
    }
    #[doc = "mclk frequency = 1/167 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_167(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_167)
    }
    #[doc = "mclk frequency = 1/168 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_168(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_168)
    }
    #[doc = "mclk frequency = 1/169 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_169(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_169)
    }
    #[doc = "mclk frequency = 1/170 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_170(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_170)
    }
    #[doc = "mclk frequency = 1/171 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_171(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_171)
    }
    #[doc = "mclk frequency = 1/172 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_172(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_172)
    }
    #[doc = "mclk frequency = 1/173 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_173(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_173)
    }
    #[doc = "mclk frequency = 1/174 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_174(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_174)
    }
    #[doc = "mclk frequency = 1/175 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_175(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_175)
    }
    #[doc = "mclk frequency = 1/176 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_176(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_176)
    }
    #[doc = "mclk frequency = 1/177 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_177(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_177)
    }
    #[doc = "mclk frequency = 1/178 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_178(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_178)
    }
    #[doc = "mclk frequency = 1/179 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_179(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_179)
    }
    #[doc = "mclk frequency = 1/180 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_180(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_180)
    }
    #[doc = "mclk frequency = 1/181 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_181(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_181)
    }
    #[doc = "mclk frequency = 1/182 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_182(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_182)
    }
    #[doc = "mclk frequency = 1/183 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_183(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_183)
    }
    #[doc = "mclk frequency = 1/184 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_184(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_184)
    }
    #[doc = "mclk frequency = 1/185 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_185(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_185)
    }
    #[doc = "mclk frequency = 1/186 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_186(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_186)
    }
    #[doc = "mclk frequency = 1/187 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_187(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_187)
    }
    #[doc = "mclk frequency = 1/188 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_188(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_188)
    }
    #[doc = "mclk frequency = 1/189 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_189(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_189)
    }
    #[doc = "mclk frequency = 1/190 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_190(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_190)
    }
    #[doc = "mclk frequency = 1/191 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_191(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_191)
    }
    #[doc = "mclk frequency = 1/192 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_192(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_192)
    }
    #[doc = "mclk frequency = 1/193 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_193(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_193)
    }
    #[doc = "mclk frequency = 1/194 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_194(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_194)
    }
    #[doc = "mclk frequency = 1/195 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_195(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_195)
    }
    #[doc = "mclk frequency = 1/196 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_196(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_196)
    }
    #[doc = "mclk frequency = 1/197 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_197(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_197)
    }
    #[doc = "mclk frequency = 1/198 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_198(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_198)
    }
    #[doc = "mclk frequency = 1/199 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_199(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_199)
    }
    #[doc = "mclk frequency = 1/200 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_200(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_200)
    }
    #[doc = "mclk frequency = 1/201 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_201(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_201)
    }
    #[doc = "mclk frequency = 1/202 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_202(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_202)
    }
    #[doc = "mclk frequency = 1/203 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_203(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_203)
    }
    #[doc = "mclk frequency = 1/204 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_204(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_204)
    }
    #[doc = "mclk frequency = 1/205 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_205(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_205)
    }
    #[doc = "mclk frequency = 1/206 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_206(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_206)
    }
    #[doc = "mclk frequency = 1/207 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_207(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_207)
    }
    #[doc = "mclk frequency = 1/208 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_208(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_208)
    }
    #[doc = "mclk frequency = 1/209 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_209(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_209)
    }
    #[doc = "mclk frequency = 1/210 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_210(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_210)
    }
    #[doc = "mclk frequency = 1/211 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_211(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_211)
    }
    #[doc = "mclk frequency = 1/212 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_212(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_212)
    }
    #[doc = "mclk frequency = 1/213 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_213(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_213)
    }
    #[doc = "mclk frequency = 1/214 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_214(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_214)
    }
    #[doc = "mclk frequency = 1/215 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_215(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_215)
    }
    #[doc = "mclk frequency = 1/216 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_216(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_216)
    }
    #[doc = "mclk frequency = 1/217 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_217(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_217)
    }
    #[doc = "mclk frequency = 1/218 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_218(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_218)
    }
    #[doc = "mclk frequency = 1/219 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_219(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_219)
    }
    #[doc = "mclk frequency = 1/220 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_220(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_220)
    }
    #[doc = "mclk frequency = 1/221 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_221(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_221)
    }
    #[doc = "mclk frequency = 1/222 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_222(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_222)
    }
    #[doc = "mclk frequency = 1/223 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_223(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_223)
    }
    #[doc = "mclk frequency = 1/224 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_224(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_224)
    }
    #[doc = "mclk frequency = 1/225 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_225(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_225)
    }
    #[doc = "mclk frequency = 1/226 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_226(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_226)
    }
    #[doc = "mclk frequency = 1/227 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_227(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_227)
    }
    #[doc = "mclk frequency = 1/228 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_228(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_228)
    }
    #[doc = "mclk frequency = 1/229 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_229(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_229)
    }
    #[doc = "mclk frequency = 1/230 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_230(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_230)
    }
    #[doc = "mclk frequency = 1/231 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_231(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_231)
    }
    #[doc = "mclk frequency = 1/232 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_232(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_232)
    }
    #[doc = "mclk frequency = 1/233 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_233(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_233)
    }
    #[doc = "mclk frequency = 1/234 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_234(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_234)
    }
    #[doc = "mclk frequency = 1/235 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_235(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_235)
    }
    #[doc = "mclk frequency = 1/236 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_236(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_236)
    }
    #[doc = "mclk frequency = 1/237 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_237(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_237)
    }
    #[doc = "mclk frequency = 1/238 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_238(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_238)
    }
    #[doc = "mclk frequency = 1/239 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_239(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_239)
    }
    #[doc = "mclk frequency = 1/240 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_240(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_240)
    }
    #[doc = "mclk frequency = 1/241 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_241(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_241)
    }
    #[doc = "mclk frequency = 1/242 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_242(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_242)
    }
    #[doc = "mclk frequency = 1/243 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_243(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_243)
    }
    #[doc = "mclk frequency = 1/244 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_244(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_244)
    }
    #[doc = "mclk frequency = 1/245 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_245(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_245)
    }
    #[doc = "mclk frequency = 1/246 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_246(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_246)
    }
    #[doc = "mclk frequency = 1/247 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_247(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_247)
    }
    #[doc = "mclk frequency = 1/248 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_248(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_248)
    }
    #[doc = "mclk frequency = 1/249 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_249(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_249)
    }
    #[doc = "mclk frequency = 1/250 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_250(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_250)
    }
    #[doc = "mclk frequency = 1/251 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_251(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_251)
    }
    #[doc = "mclk frequency = 1/252 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_252(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_252)
    }
    #[doc = "mclk frequency = 1/253 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_253(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_253)
    }
    #[doc = "mclk frequency = 1/254 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_254(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_254)
    }
    #[doc = "mclk frequency = 1/255 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_255(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_255)
    }
    #[doc = "mclk frequency = 1/256 * hmclk frequency"]
    #[inline(always)]
    pub fn divide_256(self) -> &'a mut W {
        self.variant(MQS_CLK_DIV_A::DIVIDE_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "MQS software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MQS_SW_RST_A {
    #[doc = "0: Exit software reset for MQS"]
    MQS_SW_RST_0 = 0,
    #[doc = "1: Enable software reset for MQS"]
    MQS_SW_RST_1 = 1,
}
impl From<MQS_SW_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MQS_SW_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MQS_SW_RST`"]
pub type MQS_SW_RST_R = crate::R<bool, MQS_SW_RST_A>;
impl MQS_SW_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MQS_SW_RST_A {
        match self.bits {
            false => MQS_SW_RST_A::MQS_SW_RST_0,
            true => MQS_SW_RST_A::MQS_SW_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `MQS_SW_RST_0`"]
    #[inline(always)]
    pub fn is_mqs_sw_rst_0(&self) -> bool {
        *self == MQS_SW_RST_A::MQS_SW_RST_0
    }
    #[doc = "Checks if the value of the field is `MQS_SW_RST_1`"]
    #[inline(always)]
    pub fn is_mqs_sw_rst_1(&self) -> bool {
        *self == MQS_SW_RST_A::MQS_SW_RST_1
    }
}
#[doc = "Write proxy for field `MQS_SW_RST`"]
pub struct MQS_SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MQS_SW_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MQS_SW_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exit software reset for MQS"]
    #[inline(always)]
    pub fn mqs_sw_rst_0(self) -> &'a mut W {
        self.variant(MQS_SW_RST_A::MQS_SW_RST_0)
    }
    #[doc = "Enable software reset for MQS"]
    #[inline(always)]
    pub fn mqs_sw_rst_1(self) -> &'a mut W {
        self.variant(MQS_SW_RST_A::MQS_SW_RST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "MQS enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MQS_EN_A {
    #[doc = "0: Disable MQS"]
    MQS_EN_0 = 0,
    #[doc = "1: Enable MQS"]
    MQS_EN_1 = 1,
}
impl From<MQS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MQS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MQS_EN`"]
pub type MQS_EN_R = crate::R<bool, MQS_EN_A>;
impl MQS_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MQS_EN_A {
        match self.bits {
            false => MQS_EN_A::MQS_EN_0,
            true => MQS_EN_A::MQS_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MQS_EN_0`"]
    #[inline(always)]
    pub fn is_mqs_en_0(&self) -> bool {
        *self == MQS_EN_A::MQS_EN_0
    }
    #[doc = "Checks if the value of the field is `MQS_EN_1`"]
    #[inline(always)]
    pub fn is_mqs_en_1(&self) -> bool {
        *self == MQS_EN_A::MQS_EN_1
    }
}
#[doc = "Write proxy for field `MQS_EN`"]
pub struct MQS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MQS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MQS_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable MQS"]
    #[inline(always)]
    pub fn mqs_en_0(self) -> &'a mut W {
        self.variant(MQS_EN_A::MQS_EN_0)
    }
    #[doc = "Enable MQS"]
    #[inline(always)]
    pub fn mqs_en_1(self) -> &'a mut W {
        self.variant(MQS_EN_A::MQS_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Used to control the PWM oversampling rate compared with mclk.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MQS_OVERSAMPLE_A {
    #[doc = "0: 32"]
    MQS_OVERSAMPLE_0 = 0,
    #[doc = "1: 64"]
    MQS_OVERSAMPLE_1 = 1,
}
impl From<MQS_OVERSAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: MQS_OVERSAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MQS_OVERSAMPLE`"]
pub type MQS_OVERSAMPLE_R = crate::R<bool, MQS_OVERSAMPLE_A>;
impl MQS_OVERSAMPLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MQS_OVERSAMPLE_A {
        match self.bits {
            false => MQS_OVERSAMPLE_A::MQS_OVERSAMPLE_0,
            true => MQS_OVERSAMPLE_A::MQS_OVERSAMPLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MQS_OVERSAMPLE_0`"]
    #[inline(always)]
    pub fn is_mqs_oversample_0(&self) -> bool {
        *self == MQS_OVERSAMPLE_A::MQS_OVERSAMPLE_0
    }
    #[doc = "Checks if the value of the field is `MQS_OVERSAMPLE_1`"]
    #[inline(always)]
    pub fn is_mqs_oversample_1(&self) -> bool {
        *self == MQS_OVERSAMPLE_A::MQS_OVERSAMPLE_1
    }
}
#[doc = "Write proxy for field `MQS_OVERSAMPLE`"]
pub struct MQS_OVERSAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MQS_OVERSAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MQS_OVERSAMPLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn mqs_oversample_0(self) -> &'a mut W {
        self.variant(MQS_OVERSAMPLE_A::MQS_OVERSAMPLE_0)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn mqs_oversample_1(self) -> &'a mut W {
        self.variant(MQS_OVERSAMPLE_A::MQS_OVERSAMPLE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "QTIMER1 timer counter freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER1_TMR_CNTS_FREEZE_A {
    #[doc = "0: timer counter work normally"]
    QTIMER1_TMR_CNTS_FREEZE_0 = 0,
    #[doc = "1: reset counter and ouput flags"]
    QTIMER1_TMR_CNTS_FREEZE_1 = 1,
}
impl From<QTIMER1_TMR_CNTS_FREEZE_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TMR_CNTS_FREEZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QTIMER1_TMR_CNTS_FREEZE`"]
pub type QTIMER1_TMR_CNTS_FREEZE_R = crate::R<bool, QTIMER1_TMR_CNTS_FREEZE_A>;
impl QTIMER1_TMR_CNTS_FREEZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TMR_CNTS_FREEZE_A {
        match self.bits {
            false => QTIMER1_TMR_CNTS_FREEZE_A::QTIMER1_TMR_CNTS_FREEZE_0,
            true => QTIMER1_TMR_CNTS_FREEZE_A::QTIMER1_TMR_CNTS_FREEZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TMR_CNTS_FREEZE_0`"]
    #[inline(always)]
    pub fn is_qtimer1_tmr_cnts_freeze_0(&self) -> bool {
        *self == QTIMER1_TMR_CNTS_FREEZE_A::QTIMER1_TMR_CNTS_FREEZE_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TMR_CNTS_FREEZE_1`"]
    #[inline(always)]
    pub fn is_qtimer1_tmr_cnts_freeze_1(&self) -> bool {
        *self == QTIMER1_TMR_CNTS_FREEZE_A::QTIMER1_TMR_CNTS_FREEZE_1
    }
}
#[doc = "Write proxy for field `QTIMER1_TMR_CNTS_FREEZE`"]
pub struct QTIMER1_TMR_CNTS_FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER1_TMR_CNTS_FREEZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER1_TMR_CNTS_FREEZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "timer counter work normally"]
    #[inline(always)]
    pub fn qtimer1_tmr_cnts_freeze_0(self) -> &'a mut W {
        self.variant(QTIMER1_TMR_CNTS_FREEZE_A::QTIMER1_TMR_CNTS_FREEZE_0)
    }
    #[doc = "reset counter and ouput flags"]
    #[inline(always)]
    pub fn qtimer1_tmr_cnts_freeze_1(self) -> &'a mut W {
        self.variant(QTIMER1_TMR_CNTS_FREEZE_A::QTIMER1_TMR_CNTS_FREEZE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "QTIMER2 timer counter freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER2_TMR_CNTS_FREEZE_A {
    #[doc = "0: timer counter work normally"]
    QTIMER2_TMR_CNTS_FREEZE_0 = 0,
    #[doc = "1: reset counter and ouput flags"]
    QTIMER2_TMR_CNTS_FREEZE_1 = 1,
}
impl From<QTIMER2_TMR_CNTS_FREEZE_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TMR_CNTS_FREEZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QTIMER2_TMR_CNTS_FREEZE`"]
pub type QTIMER2_TMR_CNTS_FREEZE_R = crate::R<bool, QTIMER2_TMR_CNTS_FREEZE_A>;
impl QTIMER2_TMR_CNTS_FREEZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TMR_CNTS_FREEZE_A {
        match self.bits {
            false => QTIMER2_TMR_CNTS_FREEZE_A::QTIMER2_TMR_CNTS_FREEZE_0,
            true => QTIMER2_TMR_CNTS_FREEZE_A::QTIMER2_TMR_CNTS_FREEZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TMR_CNTS_FREEZE_0`"]
    #[inline(always)]
    pub fn is_qtimer2_tmr_cnts_freeze_0(&self) -> bool {
        *self == QTIMER2_TMR_CNTS_FREEZE_A::QTIMER2_TMR_CNTS_FREEZE_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TMR_CNTS_FREEZE_1`"]
    #[inline(always)]
    pub fn is_qtimer2_tmr_cnts_freeze_1(&self) -> bool {
        *self == QTIMER2_TMR_CNTS_FREEZE_A::QTIMER2_TMR_CNTS_FREEZE_1
    }
}
#[doc = "Write proxy for field `QTIMER2_TMR_CNTS_FREEZE`"]
pub struct QTIMER2_TMR_CNTS_FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER2_TMR_CNTS_FREEZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER2_TMR_CNTS_FREEZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "timer counter work normally"]
    #[inline(always)]
    pub fn qtimer2_tmr_cnts_freeze_0(self) -> &'a mut W {
        self.variant(QTIMER2_TMR_CNTS_FREEZE_A::QTIMER2_TMR_CNTS_FREEZE_0)
    }
    #[doc = "reset counter and ouput flags"]
    #[inline(always)]
    pub fn qtimer2_tmr_cnts_freeze_1(self) -> &'a mut W {
        self.variant(QTIMER2_TMR_CNTS_FREEZE_A::QTIMER2_TMR_CNTS_FREEZE_1)
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
#[doc = "QTIMER3 timer counter freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER3_TMR_CNTS_FREEZE_A {
    #[doc = "0: timer counter work normally"]
    QTIMER3_TMR_CNTS_FREEZE_0 = 0,
    #[doc = "1: reset counter and ouput flags"]
    QTIMER3_TMR_CNTS_FREEZE_1 = 1,
}
impl From<QTIMER3_TMR_CNTS_FREEZE_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER3_TMR_CNTS_FREEZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QTIMER3_TMR_CNTS_FREEZE`"]
pub type QTIMER3_TMR_CNTS_FREEZE_R = crate::R<bool, QTIMER3_TMR_CNTS_FREEZE_A>;
impl QTIMER3_TMR_CNTS_FREEZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER3_TMR_CNTS_FREEZE_A {
        match self.bits {
            false => QTIMER3_TMR_CNTS_FREEZE_A::QTIMER3_TMR_CNTS_FREEZE_0,
            true => QTIMER3_TMR_CNTS_FREEZE_A::QTIMER3_TMR_CNTS_FREEZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TMR_CNTS_FREEZE_0`"]
    #[inline(always)]
    pub fn is_qtimer3_tmr_cnts_freeze_0(&self) -> bool {
        *self == QTIMER3_TMR_CNTS_FREEZE_A::QTIMER3_TMR_CNTS_FREEZE_0
    }
    #[doc = "Checks if the value of the field is `QTIMER3_TMR_CNTS_FREEZE_1`"]
    #[inline(always)]
    pub fn is_qtimer3_tmr_cnts_freeze_1(&self) -> bool {
        *self == QTIMER3_TMR_CNTS_FREEZE_A::QTIMER3_TMR_CNTS_FREEZE_1
    }
}
#[doc = "Write proxy for field `QTIMER3_TMR_CNTS_FREEZE`"]
pub struct QTIMER3_TMR_CNTS_FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER3_TMR_CNTS_FREEZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER3_TMR_CNTS_FREEZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "timer counter work normally"]
    #[inline(always)]
    pub fn qtimer3_tmr_cnts_freeze_0(self) -> &'a mut W {
        self.variant(QTIMER3_TMR_CNTS_FREEZE_A::QTIMER3_TMR_CNTS_FREEZE_0)
    }
    #[doc = "reset counter and ouput flags"]
    #[inline(always)]
    pub fn qtimer3_tmr_cnts_freeze_1(self) -> &'a mut W {
        self.variant(QTIMER3_TMR_CNTS_FREEZE_A::QTIMER3_TMR_CNTS_FREEZE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "QTIMER4 timer counter freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTIMER4_TMR_CNTS_FREEZE_A {
    #[doc = "0: timer counter work normally"]
    QTIMER4_TMR_CNTS_FREEZE_0 = 0,
    #[doc = "1: reset counter and ouput flags"]
    QTIMER4_TMR_CNTS_FREEZE_1 = 1,
}
impl From<QTIMER4_TMR_CNTS_FREEZE_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER4_TMR_CNTS_FREEZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QTIMER4_TMR_CNTS_FREEZE`"]
pub type QTIMER4_TMR_CNTS_FREEZE_R = crate::R<bool, QTIMER4_TMR_CNTS_FREEZE_A>;
impl QTIMER4_TMR_CNTS_FREEZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER4_TMR_CNTS_FREEZE_A {
        match self.bits {
            false => QTIMER4_TMR_CNTS_FREEZE_A::QTIMER4_TMR_CNTS_FREEZE_0,
            true => QTIMER4_TMR_CNTS_FREEZE_A::QTIMER4_TMR_CNTS_FREEZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TMR_CNTS_FREEZE_0`"]
    #[inline(always)]
    pub fn is_qtimer4_tmr_cnts_freeze_0(&self) -> bool {
        *self == QTIMER4_TMR_CNTS_FREEZE_A::QTIMER4_TMR_CNTS_FREEZE_0
    }
    #[doc = "Checks if the value of the field is `QTIMER4_TMR_CNTS_FREEZE_1`"]
    #[inline(always)]
    pub fn is_qtimer4_tmr_cnts_freeze_1(&self) -> bool {
        *self == QTIMER4_TMR_CNTS_FREEZE_A::QTIMER4_TMR_CNTS_FREEZE_1
    }
}
#[doc = "Write proxy for field `QTIMER4_TMR_CNTS_FREEZE`"]
pub struct QTIMER4_TMR_CNTS_FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> QTIMER4_TMR_CNTS_FREEZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QTIMER4_TMR_CNTS_FREEZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "timer counter work normally"]
    #[inline(always)]
    pub fn qtimer4_tmr_cnts_freeze_0(self) -> &'a mut W {
        self.variant(QTIMER4_TMR_CNTS_FREEZE_A::QTIMER4_TMR_CNTS_FREEZE_0)
    }
    #[doc = "reset counter and ouput flags"]
    #[inline(always)]
    pub fn qtimer4_tmr_cnts_freeze_1(self) -> &'a mut W {
        self.variant(QTIMER4_TMR_CNTS_FREEZE_A::QTIMER4_TMR_CNTS_FREEZE_1)
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
    #[doc = "Bit 0 - AXBS_L AHBXL master has higher priority.Do not set both DMA and AHBXL to high priority."]
    #[inline(always)]
    pub fn axbs_l_ahbxl_high_priority(&self) -> AXBS_L_AHBXL_HIGH_PRIORITY_R {
        AXBS_L_AHBXL_HIGH_PRIORITY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AXBS_L DMA master has higher priority.Do not set both DMA and AHBXL to high priority."]
    #[inline(always)]
    pub fn axbs_l_dma_high_priority(&self) -> AXBS_L_DMA_HIGH_PRIORITY_R {
        AXBS_L_DMA_HIGH_PRIORITY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Round Robin in AXBS_L"]
    #[inline(always)]
    pub fn axbs_l_force_round_robin(&self) -> AXBS_L_FORCE_ROUND_ROBIN_R {
        AXBS_L_FORCE_ROUND_ROBIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub fn axbs_p_m0_high_priority(&self) -> AXBS_P_M0_HIGH_PRIORITY_R {
        AXBS_P_M0_HIGH_PRIORITY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub fn axbs_p_m1_high_priority(&self) -> AXBS_P_M1_HIGH_PRIORITY_R {
        AXBS_P_M1_HIGH_PRIORITY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    #[inline(always)]
    pub fn axbs_p_force_round_robin(&self) -> AXBS_P_FORCE_ROUND_ROBIN_R {
        AXBS_P_FORCE_ROUND_ROBIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable CANFD filter"]
    #[inline(always)]
    pub fn canfd_filter_bypass(&self) -> CANFD_FILTER_BYPASS_R {
        CANFD_FILTER_BYPASS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - enable power saving features on L2 memory"]
    #[inline(always)]
    pub fn l2_mem_en_powersaving(&self) -> L2_MEM_EN_POWERSAVING_R {
        L2_MEM_EN_POWERSAVING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Automatically gate off RAM clock when RAM is not accessed."]
    #[inline(always)]
    pub fn ram_auto_clk_gating_en(&self) -> RAM_AUTO_CLK_GATING_EN_R {
        RAM_AUTO_CLK_GATING_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - control how memory enter Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low)"]
    #[inline(always)]
    pub fn l2_mem_deepsleep(&self) -> L2_MEM_DEEPSLEEP_R {
        L2_MEM_DEEPSLEEP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency."]
    #[inline(always)]
    pub fn mqs_clk_div(&self) -> MQS_CLK_DIV_R {
        MQS_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - MQS software reset"]
    #[inline(always)]
    pub fn mqs_sw_rst(&self) -> MQS_SW_RST_R {
        MQS_SW_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MQS enable."]
    #[inline(always)]
    pub fn mqs_en(&self) -> MQS_EN_R {
        MQS_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Used to control the PWM oversampling rate compared with mclk."]
    #[inline(always)]
    pub fn mqs_oversample(&self) -> MQS_OVERSAMPLE_R {
        MQS_OVERSAMPLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - QTIMER1 timer counter freeze"]
    #[inline(always)]
    pub fn qtimer1_tmr_cnts_freeze(&self) -> QTIMER1_TMR_CNTS_FREEZE_R {
        QTIMER1_TMR_CNTS_FREEZE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - QTIMER2 timer counter freeze"]
    #[inline(always)]
    pub fn qtimer2_tmr_cnts_freeze(&self) -> QTIMER2_TMR_CNTS_FREEZE_R {
        QTIMER2_TMR_CNTS_FREEZE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - QTIMER3 timer counter freeze"]
    #[inline(always)]
    pub fn qtimer3_tmr_cnts_freeze(&self) -> QTIMER3_TMR_CNTS_FREEZE_R {
        QTIMER3_TMR_CNTS_FREEZE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - QTIMER4 timer counter freeze"]
    #[inline(always)]
    pub fn qtimer4_tmr_cnts_freeze(&self) -> QTIMER4_TMR_CNTS_FREEZE_R {
        QTIMER4_TMR_CNTS_FREEZE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AXBS_L AHBXL master has higher priority.Do not set both DMA and AHBXL to high priority."]
    #[inline(always)]
    pub fn axbs_l_ahbxl_high_priority(&mut self) -> AXBS_L_AHBXL_HIGH_PRIORITY_W {
        AXBS_L_AHBXL_HIGH_PRIORITY_W { w: self }
    }
    #[doc = "Bit 1 - AXBS_L DMA master has higher priority.Do not set both DMA and AHBXL to high priority."]
    #[inline(always)]
    pub fn axbs_l_dma_high_priority(&mut self) -> AXBS_L_DMA_HIGH_PRIORITY_W {
        AXBS_L_DMA_HIGH_PRIORITY_W { w: self }
    }
    #[doc = "Bit 2 - Force Round Robin in AXBS_L"]
    #[inline(always)]
    pub fn axbs_l_force_round_robin(&mut self) -> AXBS_L_FORCE_ROUND_ROBIN_W {
        AXBS_L_FORCE_ROUND_ROBIN_W { w: self }
    }
    #[doc = "Bit 3 - AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub fn axbs_p_m0_high_priority(&mut self) -> AXBS_P_M0_HIGH_PRIORITY_W {
        AXBS_P_M0_HIGH_PRIORITY_W { w: self }
    }
    #[doc = "Bit 4 - AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub fn axbs_p_m1_high_priority(&mut self) -> AXBS_P_M1_HIGH_PRIORITY_W {
        AXBS_P_M1_HIGH_PRIORITY_W { w: self }
    }
    #[doc = "Bit 5 - Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    #[inline(always)]
    pub fn axbs_p_force_round_robin(&mut self) -> AXBS_P_FORCE_ROUND_ROBIN_W {
        AXBS_P_FORCE_ROUND_ROBIN_W { w: self }
    }
    #[doc = "Bit 6 - Disable CANFD filter"]
    #[inline(always)]
    pub fn canfd_filter_bypass(&mut self) -> CANFD_FILTER_BYPASS_W {
        CANFD_FILTER_BYPASS_W { w: self }
    }
    #[doc = "Bit 12 - enable power saving features on L2 memory"]
    #[inline(always)]
    pub fn l2_mem_en_powersaving(&mut self) -> L2_MEM_EN_POWERSAVING_W {
        L2_MEM_EN_POWERSAVING_W { w: self }
    }
    #[doc = "Bit 13 - Automatically gate off RAM clock when RAM is not accessed."]
    #[inline(always)]
    pub fn ram_auto_clk_gating_en(&mut self) -> RAM_AUTO_CLK_GATING_EN_W {
        RAM_AUTO_CLK_GATING_EN_W { w: self }
    }
    #[doc = "Bit 14 - control how memory enter Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low)"]
    #[inline(always)]
    pub fn l2_mem_deepsleep(&mut self) -> L2_MEM_DEEPSLEEP_W {
        L2_MEM_DEEPSLEEP_W { w: self }
    }
    #[doc = "Bits 16:23 - Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency."]
    #[inline(always)]
    pub fn mqs_clk_div(&mut self) -> MQS_CLK_DIV_W {
        MQS_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 24 - MQS software reset"]
    #[inline(always)]
    pub fn mqs_sw_rst(&mut self) -> MQS_SW_RST_W {
        MQS_SW_RST_W { w: self }
    }
    #[doc = "Bit 25 - MQS enable."]
    #[inline(always)]
    pub fn mqs_en(&mut self) -> MQS_EN_W {
        MQS_EN_W { w: self }
    }
    #[doc = "Bit 26 - Used to control the PWM oversampling rate compared with mclk."]
    #[inline(always)]
    pub fn mqs_oversample(&mut self) -> MQS_OVERSAMPLE_W {
        MQS_OVERSAMPLE_W { w: self }
    }
    #[doc = "Bit 28 - QTIMER1 timer counter freeze"]
    #[inline(always)]
    pub fn qtimer1_tmr_cnts_freeze(&mut self) -> QTIMER1_TMR_CNTS_FREEZE_W {
        QTIMER1_TMR_CNTS_FREEZE_W { w: self }
    }
    #[doc = "Bit 29 - QTIMER2 timer counter freeze"]
    #[inline(always)]
    pub fn qtimer2_tmr_cnts_freeze(&mut self) -> QTIMER2_TMR_CNTS_FREEZE_W {
        QTIMER2_TMR_CNTS_FREEZE_W { w: self }
    }
    #[doc = "Bit 30 - QTIMER3 timer counter freeze"]
    #[inline(always)]
    pub fn qtimer3_tmr_cnts_freeze(&mut self) -> QTIMER3_TMR_CNTS_FREEZE_W {
        QTIMER3_TMR_CNTS_FREEZE_W { w: self }
    }
    #[doc = "Bit 31 - QTIMER4 timer counter freeze"]
    #[inline(always)]
    pub fn qtimer4_tmr_cnts_freeze(&mut self) -> QTIMER4_TMR_CNTS_FREEZE_W {
        QTIMER4_TMR_CNTS_FREEZE_W { w: self }
    }
}
