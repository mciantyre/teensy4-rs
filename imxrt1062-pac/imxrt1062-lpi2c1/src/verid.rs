#[doc = "Reader of register VERID"]
pub type R = crate::R<u32, super::VERID>;
#[doc = "Feature Specification Number\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FEATURE_A {
    #[doc = "2: Master only, with standard feature set"]
    FEATURE_2 = 2,
    #[doc = "3: Master and slave, with standard feature set"]
    FEATURE_3 = 3,
}
impl From<FEATURE_A> for u16 {
    #[inline(always)]
    fn from(variant: FEATURE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FEATURE`"]
pub type FEATURE_R = crate::R<u16, FEATURE_A>;
impl FEATURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FEATURE_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(FEATURE_A::FEATURE_2),
            3 => Val(FEATURE_A::FEATURE_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FEATURE_2`"]
    #[inline(always)]
    pub fn is_feature_2(&self) -> bool {
        *self == FEATURE_A::FEATURE_2
    }
    #[doc = "Checks if the value of the field is `FEATURE_3`"]
    #[inline(always)]
    pub fn is_feature_3(&self) -> bool {
        *self == FEATURE_A::FEATURE_3
    }
}
#[doc = "Reader of field `MINOR`"]
pub type MINOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Feature Specification Number"]
    #[inline(always)]
    pub fn feature(&self) -> FEATURE_R {
        FEATURE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
