#[doc = "Reader of register HPVIDR1"]
pub type R = crate::R<u32, super::HPVIDR1>;
#[doc = "Reader of field `MINOR_REV`"]
pub type MINOR_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR_REV`"]
pub type MAJOR_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `IP_ID`"]
pub type IP_ID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - SNVS block minor version number"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SNVS block major version number"]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - SNVS block ID"]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
