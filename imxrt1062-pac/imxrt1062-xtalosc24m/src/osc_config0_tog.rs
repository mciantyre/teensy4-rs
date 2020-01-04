#[doc = "Reader of register OSC_CONFIG0_TOG"]
pub type R = crate::R<u32, super::OSC_CONFIG0_TOG>;
#[doc = "Writer for register OSC_CONFIG0_TOG"]
pub type W = crate::W<u32, super::OSC_CONFIG0_TOG>;
#[doc = "Register OSC_CONFIG0_TOG `reset()`'s with value 0x1020"]
impl crate::ResetValue for super::OSC_CONFIG0_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1020
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Reader of field `INVERT`"]
pub type INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVERT`"]
pub struct INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> INVERT_W<'a> {
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
#[doc = "Reader of field `RC_OSC_PROG`"]
pub type RC_OSC_PROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RC_OSC_PROG`"]
pub struct RC_OSC_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_OSC_PROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `HYST_PLUS`"]
pub type HYST_PLUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HYST_PLUS`"]
pub struct HYST_PLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_PLUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `HYST_MINUS`"]
pub type HYST_MINUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HYST_MINUS`"]
pub struct HYST_MINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_MINUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RC_OSC_PROG_CUR`"]
pub type RC_OSC_PROG_CUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RC_OSC_PROG_CUR`"]
pub struct RC_OSC_PROG_CUR_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_OSC_PROG_CUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:11 - RC osc. tuning values."]
    #[inline(always)]
    pub fn rc_osc_prog(&self) -> RC_OSC_PROG_R {
        RC_OSC_PROG_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Positive hysteresis value"]
    #[inline(always)]
    pub fn hyst_plus(&self) -> HYST_PLUS_R {
        HYST_PLUS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Negative hysteresis value"]
    #[inline(always)]
    pub fn hyst_minus(&self) -> HYST_MINUS_R {
        HYST_MINUS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - The current tuning value in use."]
    #[inline(always)]
    pub fn rc_osc_prog_cur(&self) -> RC_OSC_PROG_CUR_R {
        RC_OSC_PROG_CUR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 3 - Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub fn invert(&mut self) -> INVERT_W {
        INVERT_W { w: self }
    }
    #[doc = "Bits 4:11 - RC osc. tuning values."]
    #[inline(always)]
    pub fn rc_osc_prog(&mut self) -> RC_OSC_PROG_W {
        RC_OSC_PROG_W { w: self }
    }
    #[doc = "Bits 12:15 - Positive hysteresis value"]
    #[inline(always)]
    pub fn hyst_plus(&mut self) -> HYST_PLUS_W {
        HYST_PLUS_W { w: self }
    }
    #[doc = "Bits 16:19 - Negative hysteresis value"]
    #[inline(always)]
    pub fn hyst_minus(&mut self) -> HYST_MINUS_W {
        HYST_MINUS_W { w: self }
    }
    #[doc = "Bits 24:31 - The current tuning value in use."]
    #[inline(always)]
    pub fn rc_osc_prog_cur(&mut self) -> RC_OSC_PROG_CUR_W {
        RC_OSC_PROG_CUR_W { w: self }
    }
}
