#[doc = "Reader of register OSC_CONFIG2_CLR"]
pub type R = crate::R<u32, super::OSC_CONFIG2_CLR>;
#[doc = "Writer for register OSC_CONFIG2_CLR"]
pub type W = crate::W<u32, super::OSC_CONFIG2_CLR>;
#[doc = "Register OSC_CONFIG2_CLR `reset()`'s with value 0x0001_02e2"]
impl crate::ResetValue for super::OSC_CONFIG2_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_02e2
    }
}
#[doc = "Reader of field `COUNT_1M_TRG`"]
pub type COUNT_1M_TRG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT_1M_TRG`"]
pub struct COUNT_1M_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_1M_TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_1M`"]
pub type ENABLE_1M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_1M`"]
pub struct ENABLE_1M_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_1M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MUX_1M`"]
pub type MUX_1M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_1M`"]
pub struct MUX_1M_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_1M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CLK_1M_ERR_FL`"]
pub type CLK_1M_ERR_FL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_1M_ERR_FL`"]
pub struct CLK_1M_ERR_FL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_1M_ERR_FL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub fn count_1m_trg(&self) -> COUNT_1M_TRG_R {
        COUNT_1M_TRG_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub fn enable_1m(&self) -> ENABLE_1M_R {
        ENABLE_1M_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub fn mux_1m(&self) -> MUX_1M_R {
        MUX_1M_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub fn clk_1m_err_fl(&self) -> CLK_1M_ERR_FL_R {
        CLK_1M_ERR_FL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub fn count_1m_trg(&mut self) -> COUNT_1M_TRG_W {
        COUNT_1M_TRG_W { w: self }
    }
    #[doc = "Bit 16 - Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub fn enable_1m(&mut self) -> ENABLE_1M_W {
        ENABLE_1M_W { w: self }
    }
    #[doc = "Bit 17 - Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub fn mux_1m(&mut self) -> MUX_1M_W {
        MUX_1M_W { w: self }
    }
    #[doc = "Bit 31 - Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub fn clk_1m_err_fl(&mut self) -> CLK_1M_ERR_FL_W {
        CLK_1M_ERR_FL_W { w: self }
    }
}
