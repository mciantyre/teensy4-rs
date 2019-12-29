#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Status of the value of CCM_REF_EN_B output of ccm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_EN_B_A {
    #[doc = "0: value of CCM_REF_EN_B is '0'"]
    REF_EN_B_0,
    #[doc = "1: value of CCM_REF_EN_B is '1'"]
    REF_EN_B_1,
}
impl From<REF_EN_B_A> for bool {
    #[inline(always)]
    fn from(variant: REF_EN_B_A) -> Self {
        match variant {
            REF_EN_B_A::REF_EN_B_0 => false,
            REF_EN_B_A::REF_EN_B_1 => true,
        }
    }
}
#[doc = "Reader of field `REF_EN_B`"]
pub type REF_EN_B_R = crate::R<bool, REF_EN_B_A>;
impl REF_EN_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_EN_B_A {
        match self.bits {
            false => REF_EN_B_A::REF_EN_B_0,
            true => REF_EN_B_A::REF_EN_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `REF_EN_B_0`"]
    #[inline(always)]
    pub fn is_ref_en_b_0(&self) -> bool {
        *self == REF_EN_B_A::REF_EN_B_0
    }
    #[doc = "Checks if the value of the field is `REF_EN_B_1`"]
    #[inline(always)]
    pub fn is_ref_en_b_1(&self) -> bool {
        *self == REF_EN_B_A::REF_EN_B_1
    }
}
#[doc = "Status indication of CAMP2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAMP2_READY_A {
    #[doc = "0: CAMP2 is not ready."]
    CAMP2_READY_0,
    #[doc = "1: CAMP2 is ready."]
    CAMP2_READY_1,
}
impl From<CAMP2_READY_A> for bool {
    #[inline(always)]
    fn from(variant: CAMP2_READY_A) -> Self {
        match variant {
            CAMP2_READY_A::CAMP2_READY_0 => false,
            CAMP2_READY_A::CAMP2_READY_1 => true,
        }
    }
}
#[doc = "Reader of field `CAMP2_READY`"]
pub type CAMP2_READY_R = crate::R<bool, CAMP2_READY_A>;
impl CAMP2_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAMP2_READY_A {
        match self.bits {
            false => CAMP2_READY_A::CAMP2_READY_0,
            true => CAMP2_READY_A::CAMP2_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAMP2_READY_0`"]
    #[inline(always)]
    pub fn is_camp2_ready_0(&self) -> bool {
        *self == CAMP2_READY_A::CAMP2_READY_0
    }
    #[doc = "Checks if the value of the field is `CAMP2_READY_1`"]
    #[inline(always)]
    pub fn is_camp2_ready_1(&self) -> bool {
        *self == CAMP2_READY_A::CAMP2_READY_1
    }
}
#[doc = "Status indication of on board oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_READY_A {
    #[doc = "0: on board oscillator is not ready."]
    COSC_READY_0,
    #[doc = "1: on board oscillator is ready."]
    COSC_READY_1,
}
impl From<COSC_READY_A> for bool {
    #[inline(always)]
    fn from(variant: COSC_READY_A) -> Self {
        match variant {
            COSC_READY_A::COSC_READY_0 => false,
            COSC_READY_A::COSC_READY_1 => true,
        }
    }
}
#[doc = "Reader of field `COSC_READY`"]
pub type COSC_READY_R = crate::R<bool, COSC_READY_A>;
impl COSC_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSC_READY_A {
        match self.bits {
            false => COSC_READY_A::COSC_READY_0,
            true => COSC_READY_A::COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_READY_0`"]
    #[inline(always)]
    pub fn is_cosc_ready_0(&self) -> bool {
        *self == COSC_READY_A::COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `COSC_READY_1`"]
    #[inline(always)]
    pub fn is_cosc_ready_1(&self) -> bool {
        *self == COSC_READY_A::COSC_READY_1
    }
}
impl R {
    #[doc = "Bit 0 - Status of the value of CCM_REF_EN_B output of ccm"]
    #[inline(always)]
    pub fn ref_en_b(&self) -> REF_EN_B_R {
        REF_EN_B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status indication of CAMP2."]
    #[inline(always)]
    pub fn camp2_ready(&self) -> CAMP2_READY_R {
        CAMP2_READY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status indication of on board oscillator"]
    #[inline(always)]
    pub fn cosc_ready(&self) -> COSC_READY_R {
        COSC_READY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
