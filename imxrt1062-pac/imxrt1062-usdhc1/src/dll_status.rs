#[doc = "Reader of register DLL_STATUS"]
pub type R = crate::R<u32, super::DLL_STATUS>;
#[doc = "Reader of field `DLL_STS_SLV_LOCK`"]
pub type DLL_STS_SLV_LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `DLL_STS_REF_LOCK`"]
pub type DLL_STS_REF_LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `DLL_STS_SLV_SEL`"]
pub type DLL_STS_SLV_SEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `DLL_STS_REF_SEL`"]
pub type DLL_STS_REF_SEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - DLL_STS_SLV_LOCK"]
    #[inline(always)]
    pub fn dll_sts_slv_lock(&self) -> DLL_STS_SLV_LOCK_R {
        DLL_STS_SLV_LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DLL_STS_REF_LOCK"]
    #[inline(always)]
    pub fn dll_sts_ref_lock(&self) -> DLL_STS_REF_LOCK_R {
        DLL_STS_REF_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:8 - DLL_STS_SLV_SEL"]
    #[inline(always)]
    pub fn dll_sts_slv_sel(&self) -> DLL_STS_SLV_SEL_R {
        DLL_STS_SLV_SEL_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - DLL_STS_REF_SEL"]
    #[inline(always)]
    pub fn dll_sts_ref_sel(&self) -> DLL_STS_REF_SEL_R {
        DLL_STS_REF_SEL_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
