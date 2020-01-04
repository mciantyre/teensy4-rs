#[doc = "Reader of register SBMR2"]
pub type R = crate::R<u32, super::SBMR2>;
#[doc = "Reader of field `SEC_CONFIG`"]
pub type SEC_CONFIG_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIR_BT_DIS`"]
pub type DIR_BT_DIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BT_FUSE_SEL`"]
pub type BT_FUSE_SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BMOD`"]
pub type BMOD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - SECONFIG\\[1\\]
shows the state of the SECONFIG\\[1\\]
fuse"]
    #[inline(always)]
    pub fn sec_config(&self) -> SEC_CONFIG_R {
        SEC_CONFIG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - DIR_BT_DIS shows the state of the DIR_BT_DIS fuse"]
    #[inline(always)]
    pub fn dir_bt_dis(&self) -> DIR_BT_DIS_R {
        DIR_BT_DIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BT_FUSE_SEL (connected to gpio bt_fuse_sel) shows the state of the BT_FUSE_SEL fuse"]
    #[inline(always)]
    pub fn bt_fuse_sel(&self) -> BT_FUSE_SEL_R {
        BT_FUSE_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - BMOD\\[1:0\\]
shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B"]
    #[inline(always)]
    pub fn bmod(&self) -> BMOD_R {
        BMOD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
