#[doc = "Reader of register SBMR1"]
pub type R = crate::R<u32, super::SBMR1>;
#[doc = "Reader of field `BOOT_CFG1`"]
pub type BOOT_CFG1_R = crate::R<u8, u8>;
#[doc = "Reader of field `BOOT_CFG2`"]
pub type BOOT_CFG2_R = crate::R<u8, u8>;
#[doc = "Reader of field `BOOT_CFG3`"]
pub type BOOT_CFG3_R = crate::R<u8, u8>;
#[doc = "Reader of field `BOOT_CFG4`"]
pub type BOOT_CFG4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Refer to fusemap."]
    #[inline(always)]
    pub fn boot_cfg1(&self) -> BOOT_CFG1_R {
        BOOT_CFG1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Refer to fusemap."]
    #[inline(always)]
    pub fn boot_cfg2(&self) -> BOOT_CFG2_R {
        BOOT_CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Refer to fusemap."]
    #[inline(always)]
    pub fn boot_cfg3(&self) -> BOOT_CFG3_R {
        BOOT_CFG3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Refer to fusemap."]
    #[inline(always)]
    pub fn boot_cfg4(&self) -> BOOT_CFG4_R {
        BOOT_CFG4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
