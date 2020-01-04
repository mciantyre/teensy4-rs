#[doc = "Reader of register VDCTRL4"]
pub type R = crate::R<u32, super::VDCTRL4>;
#[doc = "Writer for register VDCTRL4"]
pub type W = crate::W<u32, super::VDCTRL4>;
#[doc = "Register VDCTRL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::VDCTRL4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOTCLK_H_VALID_DATA_CNT`"]
pub type DOTCLK_H_VALID_DATA_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DOTCLK_H_VALID_DATA_CNT`"]
pub struct DOTCLK_H_VALID_DATA_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DOTCLK_H_VALID_DATA_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
#[doc = "Reader of field `SYNC_SIGNALS_ON`"]
pub type SYNC_SIGNALS_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC_SIGNALS_ON`"]
pub struct SYNC_SIGNALS_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_SIGNALS_ON_W<'a> {
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
#[doc = "Reader of field `DOTCLK_DLY_SEL`"]
pub type DOTCLK_DLY_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOTCLK_DLY_SEL`"]
pub struct DOTCLK_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DOTCLK_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Total number of DISPLAY CLOCK (pix_clk) cycles on each horizontal line that carry valid data in DOTCLK mode"]
    #[inline(always)]
    pub fn dotclk_h_valid_data_cnt(&self) -> DOTCLK_H_VALID_DATA_CNT_R {
        DOTCLK_H_VALID_DATA_CNT_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 18 - Set this field to 1 if the LCD controller requires that the VSYNC or VSYNC/HSYNC/DOTCLK control signals should be active at least one frame before the data transfers actually start and remain active at least one frame after the data transfers end"]
    #[inline(always)]
    pub fn sync_signals_on(&self) -> SYNC_SIGNALS_ON_R {
        SYNC_SIGNALS_ON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - This bitfield selects the amount of time by which the DOTCLK signal should be delayed before coming out of the LCD_DOTCK pin"]
    #[inline(always)]
    pub fn dotclk_dly_sel(&self) -> DOTCLK_DLY_SEL_R {
        DOTCLK_DLY_SEL_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Total number of DISPLAY CLOCK (pix_clk) cycles on each horizontal line that carry valid data in DOTCLK mode"]
    #[inline(always)]
    pub fn dotclk_h_valid_data_cnt(&mut self) -> DOTCLK_H_VALID_DATA_CNT_W {
        DOTCLK_H_VALID_DATA_CNT_W { w: self }
    }
    #[doc = "Bit 18 - Set this field to 1 if the LCD controller requires that the VSYNC or VSYNC/HSYNC/DOTCLK control signals should be active at least one frame before the data transfers actually start and remain active at least one frame after the data transfers end"]
    #[inline(always)]
    pub fn sync_signals_on(&mut self) -> SYNC_SIGNALS_ON_W {
        SYNC_SIGNALS_ON_W { w: self }
    }
    #[doc = "Bits 29:31 - This bitfield selects the amount of time by which the DOTCLK signal should be delayed before coming out of the LCD_DOTCK pin"]
    #[inline(always)]
    pub fn dotclk_dly_sel(&mut self) -> DOTCLK_DLY_SEL_W {
        DOTCLK_DLY_SEL_W { w: self }
    }
}
