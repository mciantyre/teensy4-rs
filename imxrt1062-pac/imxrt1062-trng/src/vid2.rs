#[doc = "Reader of register VID2"]
pub type R = crate::R<u32, super::VID2>;
#[doc = "Shows the IP's Configuaration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONFIG_OPT_A {
    #[doc = "0: TRNG_CONFIG_OPT for TRNG."]
    CONFIG_OPT_0 = 0,
}
impl From<CONFIG_OPT_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFIG_OPT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONFIG_OPT`"]
pub type CONFIG_OPT_R = crate::R<u8, CONFIG_OPT_A>;
impl CONFIG_OPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONFIG_OPT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CONFIG_OPT_A::CONFIG_OPT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONFIG_OPT_0`"]
    #[inline(always)]
    pub fn is_config_opt_0(&self) -> bool {
        *self == CONFIG_OPT_A::CONFIG_OPT_0
    }
}
#[doc = "Shows the IP's ECO revision of the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECO_REV_A {
    #[doc = "0: TRNG_ECO_REV for TRNG."]
    ECO_REV_0 = 0,
}
impl From<ECO_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: ECO_REV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ECO_REV`"]
pub type ECO_REV_R = crate::R<u8, ECO_REV_A>;
impl ECO_REV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ECO_REV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ECO_REV_A::ECO_REV_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECO_REV_0`"]
    #[inline(always)]
    pub fn is_eco_rev_0(&self) -> bool {
        *self == ECO_REV_A::ECO_REV_0
    }
}
#[doc = "Shows the integration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTG_OPT_A {
    #[doc = "0: INTG_OPT for TRNG."]
    INTG_OPT_0 = 0,
}
impl From<INTG_OPT_A> for u8 {
    #[inline(always)]
    fn from(variant: INTG_OPT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTG_OPT`"]
pub type INTG_OPT_R = crate::R<u8, INTG_OPT_A>;
impl INTG_OPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTG_OPT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTG_OPT_A::INTG_OPT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTG_OPT_0`"]
    #[inline(always)]
    pub fn is_intg_opt_0(&self) -> bool {
        *self == INTG_OPT_A::INTG_OPT_0
    }
}
#[doc = "Shows the compile options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERA_A {
    #[doc = "0: COMPILE_OPT for TRNG."]
    ERA_0 = 0,
}
impl From<ERA_A> for u8 {
    #[inline(always)]
    fn from(variant: ERA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ERA`"]
pub type ERA_R = crate::R<u8, ERA_A>;
impl ERA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ERA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ERA_A::ERA_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ERA_0`"]
    #[inline(always)]
    pub fn is_era_0(&self) -> bool {
        *self == ERA_A::ERA_0
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the IP's Configuaration options for the TRNG."]
    #[inline(always)]
    pub fn config_opt(&self) -> CONFIG_OPT_R {
        CONFIG_OPT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the IP's ECO revision of the TRNG."]
    #[inline(always)]
    pub fn eco_rev(&self) -> ECO_REV_R {
        ECO_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Shows the integration options for the TRNG."]
    #[inline(always)]
    pub fn intg_opt(&self) -> INTG_OPT_R {
        INTG_OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Shows the compile options for the TRNG."]
    #[inline(always)]
    pub fn era(&self) -> ERA_R {
        ERA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
