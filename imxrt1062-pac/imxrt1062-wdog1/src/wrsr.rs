#[doc = "Reader of register WRSR"]
pub type R = crate::R<u16, super::WRSR>;
#[doc = "SFTW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTW_A {
    #[doc = "0: Reset is not the result of a software reset."]
    SFTW_0 = 0,
    #[doc = "1: Reset is the result of a software reset."]
    SFTW_1 = 1,
}
impl From<SFTW_A> for bool {
    #[inline(always)]
    fn from(variant: SFTW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SFTW`"]
pub type SFTW_R = crate::R<bool, SFTW_A>;
impl SFTW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFTW_A {
        match self.bits {
            false => SFTW_A::SFTW_0,
            true => SFTW_A::SFTW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SFTW_0`"]
    #[inline(always)]
    pub fn is_sftw_0(&self) -> bool {
        *self == SFTW_A::SFTW_0
    }
    #[doc = "Checks if the value of the field is `SFTW_1`"]
    #[inline(always)]
    pub fn is_sftw_1(&self) -> bool {
        *self == SFTW_A::SFTW_1
    }
}
#[doc = "TOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUT_A {
    #[doc = "0: Reset is not the result of a WDOG timeout."]
    TOUT_0 = 0,
    #[doc = "1: Reset is the result of a WDOG timeout."]
    TOUT_1 = 1,
}
impl From<TOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOUT`"]
pub type TOUT_R = crate::R<bool, TOUT_A>;
impl TOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUT_A {
        match self.bits {
            false => TOUT_A::TOUT_0,
            true => TOUT_A::TOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOUT_0`"]
    #[inline(always)]
    pub fn is_tout_0(&self) -> bool {
        *self == TOUT_A::TOUT_0
    }
    #[doc = "Checks if the value of the field is `TOUT_1`"]
    #[inline(always)]
    pub fn is_tout_1(&self) -> bool {
        *self == TOUT_A::TOUT_1
    }
}
#[doc = "POR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: Reset is not the result of a power on reset."]
    POR_0 = 0,
    #[doc = "1: Reset is the result of a power on reset."]
    POR_1 = 1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, POR_A>;
impl POR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::POR_0,
            true => POR_A::POR_1,
        }
    }
    #[doc = "Checks if the value of the field is `POR_0`"]
    #[inline(always)]
    pub fn is_por_0(&self) -> bool {
        *self == POR_A::POR_0
    }
    #[doc = "Checks if the value of the field is `POR_1`"]
    #[inline(always)]
    pub fn is_por_1(&self) -> bool {
        *self == POR_A::POR_1
    }
}
impl R {
    #[doc = "Bit 0 - SFTW"]
    #[inline(always)]
    pub fn sftw(&self) -> SFTW_R {
        SFTW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TOUT"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - POR"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
