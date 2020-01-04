#[doc = "Reader of register IPCMD"]
pub type R = crate::R<u32, super::IPCMD>;
#[doc = "Writer for register IPCMD"]
pub type W = crate::W<u32, super::IPCMD>;
#[doc = "Register IPCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::IPCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SDRAM Commands: 0x8: READ 0x9: WRITE 0xA: MODESET 0xB: ACTIVE 0xC: AUTO REFRESH 0xD: SELF REFRESH 0xE: PRECHARGE 0xF: PRECHARGE ALL Others: RSVD SELF REFRESH will be sent to all SDRAM devices because they shared same SEMC_CLK pin"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SDRAM Commands: 0x8: READ 0x9: WRITE 0xA: MODESET 0xB: ACTIVE 0xC: AUTO REFRESH 0xD: SELF REFRESH 0xE: PRECHARGE 0xF: PRECHARGE ALL Others: RSVD SELF REFRESH will be sent to all SDRAM devices because they shared same SEMC_CLK pin"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 16:31 - This field should be written with 0xA55A when trigging an IP command for all device types"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
