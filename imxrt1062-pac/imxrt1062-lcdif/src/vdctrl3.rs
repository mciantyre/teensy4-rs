#[doc = "Reader of register VDCTRL3"]
pub type R = crate::R<u32, super::VDCTRL3>;
#[doc = "Writer for register VDCTRL3"]
pub type W = crate::W<u32, super::VDCTRL3>;
#[doc = "Register VDCTRL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::VDCTRL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VERTICAL_WAIT_CNT`"]
pub type VERTICAL_WAIT_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VERTICAL_WAIT_CNT`"]
pub struct VERTICAL_WAIT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> VERTICAL_WAIT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `HORIZONTAL_WAIT_CNT`"]
pub type HORIZONTAL_WAIT_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HORIZONTAL_WAIT_CNT`"]
pub struct HORIZONTAL_WAIT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HORIZONTAL_WAIT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `VSYNC_ONLY`"]
pub type VSYNC_ONLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNC_ONLY`"]
pub struct VSYNC_ONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_ONLY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `MUX_SYNC_SIGNALS`"]
pub type MUX_SYNC_SIGNALS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_SYNC_SIGNALS`"]
pub struct MUX_SYNC_SIGNALS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SYNC_SIGNALS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - In the VSYNC interface mode, wait for this number of DISPLAY CLOCK (pix_clk) cycles from the falling VSYNC edge (or rising if VSYNC_POL is 1) before starting LCD transactions and is applicable only if WAIT_FOR_VSYNC_EDGE is set"]
    #[inline(always)]
    pub fn vertical_wait_cnt(&self) -> VERTICAL_WAIT_CNT_R {
        VERTICAL_WAIT_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - In the DOTCLK mode, wait for this number of clocks from falling edge (or rising if HSYNC_POL is 1) of HSYNC signal to account for horizontal back porch plus the number of DOTCLKs before the moving picture information begins"]
    #[inline(always)]
    pub fn horizontal_wait_cnt(&self) -> HORIZONTAL_WAIT_CNT_R {
        HORIZONTAL_WAIT_CNT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - This bit must be set to 1 in the VSYNC mode of operation, and 0 in the DOTCLK mode of operation."]
    #[inline(always)]
    pub fn vsync_only(&self) -> VSYNC_ONLY_R {
        VSYNC_ONLY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - When this bit is set, the LCDIF block will internally mux HSYNC with LCD_D14, DOTCLK with LCD_D13 and ENABLE with LCD_D12, otherwise these signals will go out on separate pins"]
    #[inline(always)]
    pub fn mux_sync_signals(&self) -> MUX_SYNC_SIGNALS_R {
        MUX_SYNC_SIGNALS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - In the VSYNC interface mode, wait for this number of DISPLAY CLOCK (pix_clk) cycles from the falling VSYNC edge (or rising if VSYNC_POL is 1) before starting LCD transactions and is applicable only if WAIT_FOR_VSYNC_EDGE is set"]
    #[inline(always)]
    pub fn vertical_wait_cnt(&mut self) -> VERTICAL_WAIT_CNT_W {
        VERTICAL_WAIT_CNT_W { w: self }
    }
    #[doc = "Bits 16:27 - In the DOTCLK mode, wait for this number of clocks from falling edge (or rising if HSYNC_POL is 1) of HSYNC signal to account for horizontal back porch plus the number of DOTCLKs before the moving picture information begins"]
    #[inline(always)]
    pub fn horizontal_wait_cnt(&mut self) -> HORIZONTAL_WAIT_CNT_W {
        HORIZONTAL_WAIT_CNT_W { w: self }
    }
    #[doc = "Bit 28 - This bit must be set to 1 in the VSYNC mode of operation, and 0 in the DOTCLK mode of operation."]
    #[inline(always)]
    pub fn vsync_only(&mut self) -> VSYNC_ONLY_W {
        VSYNC_ONLY_W { w: self }
    }
    #[doc = "Bit 29 - When this bit is set, the LCDIF block will internally mux HSYNC with LCD_D14, DOTCLK with LCD_D13 and ENABLE with LCD_D12, otherwise these signals will go out on separate pins"]
    #[inline(always)]
    pub fn mux_sync_signals(&mut self) -> MUX_SYNC_SIGNALS_W {
        MUX_SYNC_SIGNALS_W { w: self }
    }
}
