#[doc = "Reader of register HPVIDR2"]
pub type R = crate::R<u32, super::HPVIDR2>;
#[doc = "Reader of field `CONFIG_OPT`"]
pub type CONFIG_OPT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ECO_REV`"]
pub type ECO_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `INTG_OPT`"]
pub type INTG_OPT_R = crate::R<u8, u8>;
#[doc = "Reader of field `IP_ERA`"]
pub type IP_ERA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SNVS Configuration Options"]
    #[inline(always)]
    pub fn config_opt(&self) -> CONFIG_OPT_R {
        CONFIG_OPT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SNVS ECO Revision"]
    #[inline(always)]
    pub fn eco_rev(&self) -> ECO_REV_R {
        ECO_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SNVS Integration Options"]
    #[inline(always)]
    pub fn intg_opt(&self) -> INTG_OPT_R {
        INTG_OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5"]
    #[inline(always)]
    pub fn ip_era(&self) -> IP_ERA_R {
        IP_ERA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
