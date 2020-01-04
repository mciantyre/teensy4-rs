#[doc = "Reader of register VDCTRL0_CLR"]
pub type R = crate::R<u32, super::VDCTRL0_CLR>;
#[doc = "Writer for register VDCTRL0_CLR"]
pub type W = crate::W<u32, super::VDCTRL0_CLR>;
#[doc = "Register VDCTRL0_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::VDCTRL0_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSYNC_PULSE_WIDTH`"]
pub type VSYNC_PULSE_WIDTH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VSYNC_PULSE_WIDTH`"]
pub struct VSYNC_PULSE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_PULSE_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
#[doc = "Reader of field `HALF_LINE_MODE`"]
pub type HALF_LINE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALF_LINE_MODE`"]
pub struct HALF_LINE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_LINE_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `HALF_LINE`"]
pub type HALF_LINE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALF_LINE`"]
pub struct HALF_LINE_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_LINE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `VSYNC_PULSE_WIDTH_UNIT`"]
pub type VSYNC_PULSE_WIDTH_UNIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNC_PULSE_WIDTH_UNIT`"]
pub struct VSYNC_PULSE_WIDTH_UNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_PULSE_WIDTH_UNIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `VSYNC_PERIOD_UNIT`"]
pub type VSYNC_PERIOD_UNIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNC_PERIOD_UNIT`"]
pub struct VSYNC_PERIOD_UNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_PERIOD_UNIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_POL`"]
pub type ENABLE_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_POL`"]
pub struct ENABLE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DOTCLK_POL`"]
pub type DOTCLK_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOTCLK_POL`"]
pub struct DOTCLK_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DOTCLK_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `HSYNC_POL`"]
pub type HSYNC_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSYNC_POL`"]
pub struct HSYNC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNC_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `VSYNC_POL`"]
pub type VSYNC_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNC_POL`"]
pub struct VSYNC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_PRESENT`"]
pub type ENABLE_PRESENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_PRESENT`"]
pub struct ENABLE_PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_PRESENT_W<'a> {
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
impl R {
    #[doc = "Bits 0:17 - Number of units for which VSYNC signal is active"]
    #[inline(always)]
    pub fn vsync_pulse_width(&self) -> VSYNC_PULSE_WIDTH_R {
        VSYNC_PULSE_WIDTH_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 18 - When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[inline(always)]
    pub fn half_line_mode(&self) -> HALF_LINE_MODE_R {
        HALF_LINE_MODE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[inline(always)]
    pub fn half_line(&self) -> HALF_LINE_R {
        HALF_LINE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub fn vsync_pulse_width_unit(&self) -> VSYNC_PULSE_WIDTH_UNIT_R {
        VSYNC_PULSE_WIDTH_UNIT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub fn vsync_period_unit(&self) -> VSYNC_PERIOD_UNIT_R {
        VSYNC_PERIOD_UNIT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Default 0 active low during valid data transfer on each horizontal line."]
    #[inline(always)]
    pub fn enable_pol(&self) -> ENABLE_POL_R {
        ENABLE_POL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[inline(always)]
    pub fn dotclk_pol(&self) -> DOTCLK_POL_R {
        DOTCLK_POL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[inline(always)]
    pub fn hsync_pol(&self) -> HSYNC_POL_R {
        HSYNC_POL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline(always)]
    pub fn vsync_pol(&self) -> VSYNC_POL_R {
        VSYNC_POL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[inline(always)]
    pub fn enable_present(&self) -> ENABLE_PRESENT_R {
        ENABLE_PRESENT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Number of units for which VSYNC signal is active"]
    #[inline(always)]
    pub fn vsync_pulse_width(&mut self) -> VSYNC_PULSE_WIDTH_W {
        VSYNC_PULSE_WIDTH_W { w: self }
    }
    #[doc = "Bit 18 - When this bit is 0, the first field (VSYNC period) will end in half a horizontal line and the second field will begin with half a horizontal line"]
    #[inline(always)]
    pub fn half_line_mode(&mut self) -> HALF_LINE_MODE_W {
        HALF_LINE_MODE_W { w: self }
    }
    #[doc = "Bit 19 - Setting this bit to 1 will make the total VSYNC period equal to the VSYNC_PERIOD field plus half the HORIZONTAL_PERIOD field (i"]
    #[inline(always)]
    pub fn half_line(&mut self) -> HALF_LINE_W {
        HALF_LINE_W { w: self }
    }
    #[doc = "Bit 20 - Default 0 for counting VSYNC_PULSE_WIDTH in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub fn vsync_pulse_width_unit(&mut self) -> VSYNC_PULSE_WIDTH_UNIT_W {
        VSYNC_PULSE_WIDTH_UNIT_W { w: self }
    }
    #[doc = "Bit 21 - Default 0 for counting VSYNC_PERIOD in terms of DISPLAY CLOCK (pix_clk) cycles"]
    #[inline(always)]
    pub fn vsync_period_unit(&mut self) -> VSYNC_PERIOD_UNIT_W {
        VSYNC_PERIOD_UNIT_W { w: self }
    }
    #[doc = "Bit 24 - Default 0 active low during valid data transfer on each horizontal line."]
    #[inline(always)]
    pub fn enable_pol(&mut self) -> ENABLE_POL_W {
        ENABLE_POL_W { w: self }
    }
    #[doc = "Bit 25 - Default is data launched at negative edge of DOTCLK and captured at positive edge"]
    #[inline(always)]
    pub fn dotclk_pol(&mut self) -> DOTCLK_POL_W {
        DOTCLK_POL_W { w: self }
    }
    #[doc = "Bit 26 - Default 0 active low during HSYNC_PULSE_WIDTH time and will be high during the rest of the HSYNC period"]
    #[inline(always)]
    pub fn hsync_pol(&mut self) -> HSYNC_POL_W {
        HSYNC_POL_W { w: self }
    }
    #[doc = "Bit 27 - Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline(always)]
    pub fn vsync_pol(&mut self) -> VSYNC_POL_W {
        VSYNC_POL_W { w: self }
    }
    #[doc = "Bit 28 - Setting this bit to 1 will make the hardware generate the ENABLE signal in the DOTCLK mode, thereby making it the true RGB interface along with the remaining three signals VSYNC, HSYNC and DOTCLK"]
    #[inline(always)]
    pub fn enable_present(&mut self) -> ENABLE_PRESENT_W {
        ENABLE_PRESENT_W { w: self }
    }
}
