#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0x40"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `IPCMDDONE`"]
pub type IPCMDDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDDONE`"]
pub struct IPCMDDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDDONE_W<'a> {
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
#[doc = "Reader of field `IPCMDGE`"]
pub type IPCMDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDGE`"]
pub struct IPCMDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDGE_W<'a> {
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
#[doc = "Reader of field `AHBCMDGE`"]
pub type AHBCMDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBCMDGE`"]
pub struct AHBCMDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBCMDGE_W<'a> {
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
#[doc = "Reader of field `IPCMDERR`"]
pub type IPCMDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDERR`"]
pub struct IPCMDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDERR_W<'a> {
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
#[doc = "Reader of field `AHBCMDERR`"]
pub type AHBCMDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBCMDERR`"]
pub struct AHBCMDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBCMDERR_W<'a> {
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
#[doc = "Reader of field `IPRXWA`"]
pub type IPRXWA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPRXWA`"]
pub struct IPRXWA_W<'a> {
    w: &'a mut W,
}
impl<'a> IPRXWA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `IPTXWE`"]
pub type IPTXWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPTXWE`"]
pub struct IPTXWE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTXWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SCKSTOPBYRD`"]
pub type SCKSTOPBYRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCKSTOPBYRD`"]
pub struct SCKSTOPBYRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKSTOPBYRD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCKSTOPBYWR`"]
pub type SCKSTOPBYWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCKSTOPBYWR`"]
pub struct SCKSTOPBYWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKSTOPBYWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `AHBBUSTIMEOUT`"]
pub type AHBBUSTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBBUSTIMEOUT`"]
pub struct AHBBUSTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBBUSTIMEOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SEQTIMEOUT`"]
pub type SEQTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEQTIMEOUT`"]
pub struct SEQTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQTIMEOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    pub fn ipcmddone(&self) -> IPCMDDONE_R {
        IPCMDDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ipcmdge(&self) -> IPCMDGE_R {
        IPCMDGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ahbcmdge(&self) -> AHBCMDGE_R {
        AHBCMDGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ipcmderr(&self) -> IPCMDERR_R {
        IPCMDERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ahbcmderr(&self) -> AHBCMDERR_R {
        AHBCMDERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    pub fn iprxwa(&self) -> IPRXWA_R {
        IPRXWA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    pub fn iptxwe(&self) -> IPTXWE_R {
        IPTXWE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SCK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    pub fn sckstopbyrd(&self) -> SCKSTOPBYRD_R {
        SCKSTOPBYRD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SCK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    pub fn sckstopbywr(&self) -> SCKSTOPBYWR_R {
        SCKSTOPBYWR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeout(&self) -> AHBBUSTIMEOUT_R {
        AHBBUSTIMEOUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt."]
    #[inline(always)]
    pub fn seqtimeout(&self) -> SEQTIMEOUT_R {
        SEQTIMEOUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    pub fn ipcmddone(&mut self) -> IPCMDDONE_W {
        IPCMDDONE_W { w: self }
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ipcmdge(&mut self) -> IPCMDGE_W {
        IPCMDGE_W { w: self }
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn ahbcmdge(&mut self) -> AHBCMDGE_W {
        AHBCMDGE_W { w: self }
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ipcmderr(&mut self) -> IPCMDERR_W {
        IPCMDERR_W { w: self }
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn ahbcmderr(&mut self) -> AHBCMDERR_W {
        AHBCMDERR_W { w: self }
    }
    #[doc = "Bit 5 - IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    pub fn iprxwa(&mut self) -> IPRXWA_W {
        IPRXWA_W { w: self }
    }
    #[doc = "Bit 6 - IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    pub fn iptxwe(&mut self) -> IPTXWE_W {
        IPTXWE_W { w: self }
    }
    #[doc = "Bit 8 - SCK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    pub fn sckstopbyrd(&mut self) -> SCKSTOPBYRD_W {
        SCKSTOPBYRD_W { w: self }
    }
    #[doc = "Bit 9 - SCK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    pub fn sckstopbywr(&mut self) -> SCKSTOPBYWR_W {
        SCKSTOPBYWR_W { w: self }
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeout(&mut self) -> AHBBUSTIMEOUT_W {
        AHBBUSTIMEOUT_W { w: self }
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt."]
    #[inline(always)]
    pub fn seqtimeout(&mut self) -> SEQTIMEOUT_W {
        SEQTIMEOUT_W { w: self }
    }
}
