#[doc = "Reader of register ADMA_ERR_STATUS"]
pub type R = crate::R<u32, super::ADMA_ERR_STATUS>;
#[doc = "Reader of field `ADMAES`"]
pub type ADMAES_R = crate::R<u8, u8>;
#[doc = "ADMA Length Mismatch Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMALME_A {
    #[doc = "0: No Error"]
    ADMALME_0 = 0,
    #[doc = "1: Error"]
    ADMALME_1 = 1,
}
impl From<ADMALME_A> for bool {
    #[inline(always)]
    fn from(variant: ADMALME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADMALME`"]
pub type ADMALME_R = crate::R<bool, ADMALME_A>;
impl ADMALME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMALME_A {
        match self.bits {
            false => ADMALME_A::ADMALME_0,
            true => ADMALME_A::ADMALME_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADMALME_0`"]
    #[inline(always)]
    pub fn is_admalme_0(&self) -> bool {
        *self == ADMALME_A::ADMALME_0
    }
    #[doc = "Checks if the value of the field is `ADMALME_1`"]
    #[inline(always)]
    pub fn is_admalme_1(&self) -> bool {
        *self == ADMALME_A::ADMALME_1
    }
}
#[doc = "ADMA Descriptor Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMADCE_A {
    #[doc = "0: No Error"]
    ADMADCE_0 = 0,
    #[doc = "1: Error"]
    ADMADCE_1 = 1,
}
impl From<ADMADCE_A> for bool {
    #[inline(always)]
    fn from(variant: ADMADCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADMADCE`"]
pub type ADMADCE_R = crate::R<bool, ADMADCE_A>;
impl ADMADCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMADCE_A {
        match self.bits {
            false => ADMADCE_A::ADMADCE_0,
            true => ADMADCE_A::ADMADCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADMADCE_0`"]
    #[inline(always)]
    pub fn is_admadce_0(&self) -> bool {
        *self == ADMADCE_A::ADMADCE_0
    }
    #[doc = "Checks if the value of the field is `ADMADCE_1`"]
    #[inline(always)]
    pub fn is_admadce_1(&self) -> bool {
        *self == ADMADCE_A::ADMADCE_1
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMA Error State (when ADMA Error is occurred)"]
    #[inline(always)]
    pub fn admaes(&self) -> ADMAES_R {
        ADMAES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn admalme(&self) -> ADMALME_R {
        ADMALME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADMA Descriptor Error"]
    #[inline(always)]
    pub fn admadce(&self) -> ADMADCE_R {
        ADMADCE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
