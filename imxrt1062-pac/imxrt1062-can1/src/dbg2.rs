#[doc = "Reader of register DBG2"]
pub type R = crate::R<u32, super::DBG2>;
#[doc = "Reader of field `RMP`"]
pub type RMP_R = crate::R<u8, u8>;
#[doc = "Matching Process in Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPP_A {
    #[doc = "0: No matching process ongoing."]
    MPP_0 = 0,
    #[doc = "1: Matching process is in progress."]
    MPP_1 = 1,
}
impl From<MPP_A> for bool {
    #[inline(always)]
    fn from(variant: MPP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPP`"]
pub type MPP_R = crate::R<bool, MPP_A>;
impl MPP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPP_A {
        match self.bits {
            false => MPP_A::MPP_0,
            true => MPP_A::MPP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPP_0`"]
    #[inline(always)]
    pub fn is_mpp_0(&self) -> bool {
        *self == MPP_A::MPP_0
    }
    #[doc = "Checks if the value of the field is `MPP_1`"]
    #[inline(always)]
    pub fn is_mpp_1(&self) -> bool {
        *self == MPP_A::MPP_1
    }
}
#[doc = "Reader of field `TAP`"]
pub type TAP_R = crate::R<u8, u8>;
#[doc = "Arbitration Process in Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APP_A {
    #[doc = "0: No matching process ongoing."]
    APP_0 = 0,
    #[doc = "1: Matching process is in progress."]
    APP_1 = 1,
}
impl From<APP_A> for bool {
    #[inline(always)]
    fn from(variant: APP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `APP`"]
pub type APP_R = crate::R<bool, APP_A>;
impl APP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APP_A {
        match self.bits {
            false => APP_A::APP_0,
            true => APP_A::APP_1,
        }
    }
    #[doc = "Checks if the value of the field is `APP_0`"]
    #[inline(always)]
    pub fn is_app_0(&self) -> bool {
        *self == APP_A::APP_0
    }
    #[doc = "Checks if the value of the field is `APP_1`"]
    #[inline(always)]
    pub fn is_app_1(&self) -> bool {
        *self == APP_A::APP_1
    }
}
impl R {
    #[doc = "Bits 0:6 - Rx Matching Pointer"]
    #[inline(always)]
    pub fn rmp(&self) -> RMP_R {
        RMP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Matching Process in Progress"]
    #[inline(always)]
    pub fn mpp(&self) -> MPP_R {
        MPP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Tx Arbitration Pointer"]
    #[inline(always)]
    pub fn tap(&self) -> TAP_R {
        TAP_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Arbitration Process in Progress"]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
