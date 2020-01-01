#[doc = "Reader of register CLK_TUNE_CTRL_STATUS"]
pub type R = crate::R<u32, super::CLK_TUNE_CTRL_STATUS>;
#[doc = "Writer for register CLK_TUNE_CTRL_STATUS"]
pub type W = crate::W<u32, super::CLK_TUNE_CTRL_STATUS>;
#[doc = "Register CLK_TUNE_CTRL_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TUNE_CTRL_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLY_CELL_SET_POST`"]
pub type DLY_CELL_SET_POST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLY_CELL_SET_POST`"]
pub struct DLY_CELL_SET_POST_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_CELL_SET_POST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DLY_CELL_SET_OUT`"]
pub type DLY_CELL_SET_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLY_CELL_SET_OUT`"]
pub struct DLY_CELL_SET_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_CELL_SET_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DLY_CELL_SET_PRE`"]
pub type DLY_CELL_SET_PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLY_CELL_SET_PRE`"]
pub struct DLY_CELL_SET_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_CELL_SET_PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NXT_ERR`"]
pub type NXT_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAP_SEL_POST`"]
pub type TAP_SEL_POST_R = crate::R<u8, u8>;
#[doc = "Reader of field `TAP_SEL_OUT`"]
pub type TAP_SEL_OUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TAP_SEL_PRE`"]
pub type TAP_SEL_PRE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRE_ERR`"]
pub type PRE_ERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - DLY_CELL_SET_POST"]
    #[inline(always)]
    pub fn dly_cell_set_post(&self) -> DLY_CELL_SET_POST_R {
        DLY_CELL_SET_POST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DLY_CELL_SET_OUT"]
    #[inline(always)]
    pub fn dly_cell_set_out(&self) -> DLY_CELL_SET_OUT_R {
        DLY_CELL_SET_OUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - DLY_CELL_SET_PRE"]
    #[inline(always)]
    pub fn dly_cell_set_pre(&self) -> DLY_CELL_SET_PRE_R {
        DLY_CELL_SET_PRE_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - NXT_ERR"]
    #[inline(always)]
    pub fn nxt_err(&self) -> NXT_ERR_R {
        NXT_ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - TAP_SEL_POST"]
    #[inline(always)]
    pub fn tap_sel_post(&self) -> TAP_SEL_POST_R {
        TAP_SEL_POST_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TAP_SEL_OUT"]
    #[inline(always)]
    pub fn tap_sel_out(&self) -> TAP_SEL_OUT_R {
        TAP_SEL_OUT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - TAP_SEL_PRE"]
    #[inline(always)]
    pub fn tap_sel_pre(&self) -> TAP_SEL_PRE_R {
        TAP_SEL_PRE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - PRE_ERR"]
    #[inline(always)]
    pub fn pre_err(&self) -> PRE_ERR_R {
        PRE_ERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DLY_CELL_SET_POST"]
    #[inline(always)]
    pub fn dly_cell_set_post(&mut self) -> DLY_CELL_SET_POST_W {
        DLY_CELL_SET_POST_W { w: self }
    }
    #[doc = "Bits 4:7 - DLY_CELL_SET_OUT"]
    #[inline(always)]
    pub fn dly_cell_set_out(&mut self) -> DLY_CELL_SET_OUT_W {
        DLY_CELL_SET_OUT_W { w: self }
    }
    #[doc = "Bits 8:14 - DLY_CELL_SET_PRE"]
    #[inline(always)]
    pub fn dly_cell_set_pre(&mut self) -> DLY_CELL_SET_PRE_W {
        DLY_CELL_SET_PRE_W { w: self }
    }
}
