#[doc = "Reader of register SW_STICKY"]
pub type R = crate::R<u32, super::SW_STICKY>;
#[doc = "Writer for register SW_STICKY"]
pub type W = crate::W<u32, super::SW_STICKY>;
#[doc = "Register SW_STICKY `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_STICKY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLOCK_DTCP_KEY`"]
pub type BLOCK_DTCP_KEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCK_DTCP_KEY`"]
pub struct BLOCK_DTCP_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_DTCP_KEY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SRK_REVOKE_LOCK`"]
pub type SRK_REVOKE_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRK_REVOKE_LOCK`"]
pub struct SRK_REVOKE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRK_REVOKE_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FIELD_RETURN_LOCK`"]
pub type FIELD_RETURN_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIELD_RETURN_LOCK`"]
pub struct FIELD_RETURN_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELD_RETURN_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BLOCK_ROM_PART`"]
pub type BLOCK_ROM_PART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCK_ROM_PART`"]
pub struct BLOCK_ROM_PART_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_ROM_PART_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `JTAG_BLOCK_RELEASE`"]
pub type JTAG_BLOCK_RELEASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JTAG_BLOCK_RELEASE`"]
pub struct JTAG_BLOCK_RELEASE_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_BLOCK_RELEASE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 0 - BLOCK_DTCP_KEY"]
    #[inline(always)]
    pub fn block_dtcp_key(&self) -> BLOCK_DTCP_KEY_R {
        BLOCK_DTCP_KEY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRK_REVOKE_LOCK"]
    #[inline(always)]
    pub fn srk_revoke_lock(&self) -> SRK_REVOKE_LOCK_R {
        SRK_REVOKE_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIELD_RETURN_LOCK"]
    #[inline(always)]
    pub fn field_return_lock(&self) -> FIELD_RETURN_LOCK_R {
        FIELD_RETURN_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BLOCK_ROM_PART"]
    #[inline(always)]
    pub fn block_rom_part(&self) -> BLOCK_ROM_PART_R {
        BLOCK_ROM_PART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - JTAG_BLOCK_RELEASE"]
    #[inline(always)]
    pub fn jtag_block_release(&self) -> JTAG_BLOCK_RELEASE_R {
        JTAG_BLOCK_RELEASE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:31 - RSVD0"]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - BLOCK_DTCP_KEY"]
    #[inline(always)]
    pub fn block_dtcp_key(&mut self) -> BLOCK_DTCP_KEY_W {
        BLOCK_DTCP_KEY_W { w: self }
    }
    #[doc = "Bit 1 - SRK_REVOKE_LOCK"]
    #[inline(always)]
    pub fn srk_revoke_lock(&mut self) -> SRK_REVOKE_LOCK_W {
        SRK_REVOKE_LOCK_W { w: self }
    }
    #[doc = "Bit 2 - FIELD_RETURN_LOCK"]
    #[inline(always)]
    pub fn field_return_lock(&mut self) -> FIELD_RETURN_LOCK_W {
        FIELD_RETURN_LOCK_W { w: self }
    }
    #[doc = "Bit 3 - BLOCK_ROM_PART"]
    #[inline(always)]
    pub fn block_rom_part(&mut self) -> BLOCK_ROM_PART_W {
        BLOCK_ROM_PART_W { w: self }
    }
    #[doc = "Bit 4 - JTAG_BLOCK_RELEASE"]
    #[inline(always)]
    pub fn jtag_block_release(&mut self) -> JTAG_BLOCK_RELEASE_W {
        JTAG_BLOCK_RELEASE_W { w: self }
    }
}
