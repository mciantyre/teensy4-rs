#[doc = "Reader of register LOCK"]
pub type R = crate::R<u32, super::LOCK>;
#[doc = "Writer for register LOCK"]
pub type W = crate::W<u32, super::LOCK>;
#[doc = "Register LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TESTER`"]
pub type TESTER_R = crate::R<u8, u8>;
#[doc = "Reader of field `BOOT_CFG`"]
pub type BOOT_CFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `MEM_TRIM`"]
pub type MEM_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `SJC_RESP`"]
pub type SJC_RESP_R = crate::R<bool, bool>;
#[doc = "Reader of field `GP4_RLOCK`"]
pub type GP4_RLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAC_ADDR`"]
pub type MAC_ADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `GP1`"]
pub type GP1_R = crate::R<u8, u8>;
#[doc = "Reader of field `GP2`"]
pub type GP2_R = crate::R<u8, u8>;
#[doc = "Reader of field `ROM_PATCH`"]
pub type ROM_PATCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_GP1`"]
pub type SW_GP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTPMK`"]
pub type OTPMK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ANALOG`"]
pub type ANALOG_R = crate::R<u8, u8>;
#[doc = "Reader of field `OTPMK_CRC`"]
pub type OTPMK_CRC_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_GP2_LOCK`"]
pub type SW_GP2_LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISC_CONF`"]
pub type MISC_CONF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_GP2_RLOCK`"]
pub type SW_GP2_RLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `GP4`"]
pub type GP4_R = crate::R<u8, u8>;
#[doc = "Reader of field `GP3`"]
pub type GP3_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIELD_RETURN`"]
pub type FIELD_RETURN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIELD_RETURN`"]
pub struct FIELD_RETURN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELD_RETURN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TESTER"]
    #[inline(always)]
    pub fn tester(&self) -> TESTER_R {
        TESTER_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BOOT_CFG"]
    #[inline(always)]
    pub fn boot_cfg(&self) -> BOOT_CFG_R {
        BOOT_CFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - MEM_TRIM"]
    #[inline(always)]
    pub fn mem_trim(&self) -> MEM_TRIM_R {
        MEM_TRIM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - SJC_RESP"]
    #[inline(always)]
    pub fn sjc_resp(&self) -> SJC_RESP_R {
        SJC_RESP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GP4_RLOCK"]
    #[inline(always)]
    pub fn gp4_rlock(&self) -> GP4_RLOCK_R {
        GP4_RLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - MAC_ADDR"]
    #[inline(always)]
    pub fn mac_addr(&self) -> MAC_ADDR_R {
        MAC_ADDR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - GP1"]
    #[inline(always)]
    pub fn gp1(&self) -> GP1_R {
        GP1_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - GP2"]
    #[inline(always)]
    pub fn gp2(&self) -> GP2_R {
        GP2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - ROM_PATCH"]
    #[inline(always)]
    pub fn rom_patch(&self) -> ROM_PATCH_R {
        ROM_PATCH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SW_GP1"]
    #[inline(always)]
    pub fn sw_gp1(&self) -> SW_GP1_R {
        SW_GP1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OTPMK"]
    #[inline(always)]
    pub fn otpmk(&self) -> OTPMK_R {
        OTPMK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - ANALOG"]
    #[inline(always)]
    pub fn analog(&self) -> ANALOG_R {
        ANALOG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - OTPMK_CRC"]
    #[inline(always)]
    pub fn otpmk_crc(&self) -> OTPMK_CRC_R {
        OTPMK_CRC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SW_GP2_LOCK"]
    #[inline(always)]
    pub fn sw_gp2_lock(&self) -> SW_GP2_LOCK_R {
        SW_GP2_LOCK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MISC_CONF"]
    #[inline(always)]
    pub fn misc_conf(&self) -> MISC_CONF_R {
        MISC_CONF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SW_GP2_RLOCK"]
    #[inline(always)]
    pub fn sw_gp2_rlock(&self) -> SW_GP2_RLOCK_R {
        SW_GP2_RLOCK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - GP4"]
    #[inline(always)]
    pub fn gp4(&self) -> GP4_R {
        GP4_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - GP3"]
    #[inline(always)]
    pub fn gp3(&self) -> GP3_R {
        GP3_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:31 - FIELD_RETURN"]
    #[inline(always)]
    pub fn field_return(&self) -> FIELD_RETURN_R {
        FIELD_RETURN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - FIELD_RETURN"]
    #[inline(always)]
    pub fn field_return(&mut self) -> FIELD_RETURN_W {
        FIELD_RETURN_W { w: self }
    }
}
