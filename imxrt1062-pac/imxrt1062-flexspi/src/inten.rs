#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IPCMDDONEEN`"]
pub type IPCMDDONEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDDONEEN`"]
pub struct IPCMDDONEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDDONEEN_W<'a> {
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
#[doc = "Reader of field `IPCMDGEEN`"]
pub type IPCMDGEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDGEEN`"]
pub struct IPCMDGEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDGEEN_W<'a> {
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
#[doc = "Reader of field `AHBCMDGEEN`"]
pub type AHBCMDGEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBCMDGEEN`"]
pub struct AHBCMDGEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBCMDGEEN_W<'a> {
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
#[doc = "Reader of field `IPCMDERREN`"]
pub type IPCMDERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDERREN`"]
pub struct IPCMDERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDERREN_W<'a> {
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
#[doc = "Reader of field `AHBCMDERREN`"]
pub type AHBCMDERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBCMDERREN`"]
pub struct AHBCMDERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBCMDERREN_W<'a> {
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
#[doc = "Reader of field `IPRXWAEN`"]
pub type IPRXWAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPRXWAEN`"]
pub struct IPRXWAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPRXWAEN_W<'a> {
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
#[doc = "Reader of field `IPTXWEEN`"]
pub type IPTXWEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPTXWEEN`"]
pub struct IPTXWEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTXWEEN_W<'a> {
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
#[doc = "Reader of field `SCKSTOPBYRDEN`"]
pub type SCKSTOPBYRDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCKSTOPBYRDEN`"]
pub struct SCKSTOPBYRDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKSTOPBYRDEN_W<'a> {
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
#[doc = "Reader of field `SCKSTOPBYWREN`"]
pub type SCKSTOPBYWREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCKSTOPBYWREN`"]
pub struct SCKSTOPBYWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKSTOPBYWREN_W<'a> {
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
#[doc = "Reader of field `AHBBUSTIMEOUTEN`"]
pub type AHBBUSTIMEOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBBUSTIMEOUTEN`"]
pub struct AHBBUSTIMEOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBBUSTIMEOUTEN_W<'a> {
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
#[doc = "Reader of field `SEQTIMEOUTEN`"]
pub type SEQTIMEOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEQTIMEOUTEN`"]
pub struct SEQTIMEOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQTIMEOUTEN_W<'a> {
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
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    pub fn ipcmddoneen(&self) -> IPCMDDONEEN_R {
        IPCMDDONEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ipcmdgeen(&self) -> IPCMDGEEN_R {
        IPCMDGEEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ahbcmdgeen(&self) -> AHBCMDGEEN_R {
        AHBCMDGEEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ipcmderren(&self) -> IPCMDERREN_R {
        IPCMDERREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ahbcmderren(&self) -> AHBCMDERREN_R {
        AHBCMDERREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    pub fn iprxwaen(&self) -> IPRXWAEN_R {
        IPRXWAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    pub fn iptxween(&self) -> IPTXWEEN_R {
        IPTXWEEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SCK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    pub fn sckstopbyrden(&self) -> SCKSTOPBYRDEN_R {
        SCKSTOPBYRDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SCK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn sckstopbywren(&self) -> SCKSTOPBYWREN_R {
        SCKSTOPBYWREN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeouten(&self) -> AHBBUSTIMEOUTEN_R {
        AHBBUSTIMEOUTEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn seqtimeouten(&self) -> SEQTIMEOUTEN_R {
        SEQTIMEOUTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    pub fn ipcmddoneen(&mut self) -> IPCMDDONEEN_W {
        IPCMDDONEEN_W { w: self }
    }
    #[doc = "Bit 1 - IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ipcmdgeen(&mut self) -> IPCMDGEEN_W {
        IPCMDGEEN_W { w: self }
    }
    #[doc = "Bit 2 - AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn ahbcmdgeen(&mut self) -> AHBCMDGEEN_W {
        AHBCMDGEEN_W { w: self }
    }
    #[doc = "Bit 3 - IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ipcmderren(&mut self) -> IPCMDERREN_W {
        IPCMDERREN_W { w: self }
    }
    #[doc = "Bit 4 - AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn ahbcmderren(&mut self) -> AHBCMDERREN_W {
        AHBCMDERREN_W { w: self }
    }
    #[doc = "Bit 5 - IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    pub fn iprxwaen(&mut self) -> IPRXWAEN_W {
        IPRXWAEN_W { w: self }
    }
    #[doc = "Bit 6 - IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    pub fn iptxween(&mut self) -> IPTXWEEN_W {
        IPTXWEEN_W { w: self }
    }
    #[doc = "Bit 8 - SCK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    pub fn sckstopbyrden(&mut self) -> SCKSTOPBYRDEN_W {
        SCKSTOPBYRDEN_W { w: self }
    }
    #[doc = "Bit 9 - SCK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn sckstopbywren(&mut self) -> SCKSTOPBYWREN_W {
        SCKSTOPBYWREN_W { w: self }
    }
    #[doc = "Bit 10 - AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn ahbbustimeouten(&mut self) -> AHBBUSTIMEOUTEN_W {
        AHBBUSTIMEOUTEN_W { w: self }
    }
    #[doc = "Bit 11 - Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn seqtimeouten(&mut self) -> SEQTIMEOUTEN_W {
        SEQTIMEOUTEN_W { w: self }
    }
}
