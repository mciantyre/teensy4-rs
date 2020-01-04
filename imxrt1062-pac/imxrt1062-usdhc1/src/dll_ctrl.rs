#[doc = "Reader of register DLL_CTRL"]
pub type R = crate::R<u32, super::DLL_CTRL>;
#[doc = "Writer for register DLL_CTRL"]
pub type W = crate::W<u32, super::DLL_CTRL>;
#[doc = "Register DLL_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DLL_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLL_CTRL_ENABLE`"]
pub type DLL_CTRL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLL_CTRL_ENABLE`"]
pub struct DLL_CTRL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_ENABLE_W<'a> {
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
#[doc = "Reader of field `DLL_CTRL_RESET`"]
pub type DLL_CTRL_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLL_CTRL_RESET`"]
pub struct DLL_CTRL_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_RESET_W<'a> {
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
#[doc = "Reader of field `DLL_CTRL_SLV_FORCE_UPD`"]
pub type DLL_CTRL_SLV_FORCE_UPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLL_CTRL_SLV_FORCE_UPD`"]
pub struct DLL_CTRL_SLV_FORCE_UPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_SLV_FORCE_UPD_W<'a> {
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
#[doc = "Reader of field `DLL_CTRL_SLV_DLY_TARGET0`"]
pub type DLL_CTRL_SLV_DLY_TARGET0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLL_CTRL_SLV_DLY_TARGET0`"]
pub struct DLL_CTRL_SLV_DLY_TARGET0_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_SLV_DLY_TARGET0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `DLL_CTRL_GATE_UPDATE`"]
pub type DLL_CTRL_GATE_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLL_CTRL_GATE_UPDATE`"]
pub struct DLL_CTRL_GATE_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_GATE_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DLL_CTRL_SLV_OVERRIDE`"]
pub type DLL_CTRL_SLV_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLL_CTRL_SLV_OVERRIDE`"]
pub struct DLL_CTRL_SLV_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_SLV_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `DLL_CTRL_SLV_OVERRIDE_VAL`"]
pub type DLL_CTRL_SLV_OVERRIDE_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLL_CTRL_SLV_OVERRIDE_VAL`"]
pub struct DLL_CTRL_SLV_OVERRIDE_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_SLV_OVERRIDE_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `DLL_CTRL_SLV_DLY_TARGET1`"]
pub type DLL_CTRL_SLV_DLY_TARGET1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLL_CTRL_SLV_DLY_TARGET1`"]
pub struct DLL_CTRL_SLV_DLY_TARGET1_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_SLV_DLY_TARGET1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `DLL_CTRL_SLV_UPDATE_INT`"]
pub type DLL_CTRL_SLV_UPDATE_INT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLL_CTRL_SLV_UPDATE_INT`"]
pub struct DLL_CTRL_SLV_UPDATE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_SLV_UPDATE_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `DLL_CTRL_REF_UPDATE_INT`"]
pub type DLL_CTRL_REF_UPDATE_INT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLL_CTRL_REF_UPDATE_INT`"]
pub struct DLL_CTRL_REF_UPDATE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CTRL_REF_UPDATE_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DLL_CTRL_ENABLE"]
    #[inline(always)]
    pub fn dll_ctrl_enable(&self) -> DLL_CTRL_ENABLE_R {
        DLL_CTRL_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DLL_CTRL_RESET"]
    #[inline(always)]
    pub fn dll_ctrl_reset(&self) -> DLL_CTRL_RESET_R {
        DLL_CTRL_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DLL_CTRL_SLV_FORCE_UPD"]
    #[inline(always)]
    pub fn dll_ctrl_slv_force_upd(&self) -> DLL_CTRL_SLV_FORCE_UPD_R {
        DLL_CTRL_SLV_FORCE_UPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - DLL_CTRL_SLV_DLY_TARGET0"]
    #[inline(always)]
    pub fn dll_ctrl_slv_dly_target0(&self) -> DLL_CTRL_SLV_DLY_TARGET0_R {
        DLL_CTRL_SLV_DLY_TARGET0_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - DLL_CTRL_GATE_UPDATE"]
    #[inline(always)]
    pub fn dll_ctrl_gate_update(&self) -> DLL_CTRL_GATE_UPDATE_R {
        DLL_CTRL_GATE_UPDATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DLL_CTRL_SLV_OVERRIDE"]
    #[inline(always)]
    pub fn dll_ctrl_slv_override(&self) -> DLL_CTRL_SLV_OVERRIDE_R {
        DLL_CTRL_SLV_OVERRIDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - DLL_CTRL_SLV_OVERRIDE_VAL"]
    #[inline(always)]
    pub fn dll_ctrl_slv_override_val(&self) -> DLL_CTRL_SLV_OVERRIDE_VAL_R {
        DLL_CTRL_SLV_OVERRIDE_VAL_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - DLL_CTRL_SLV_DLY_TARGET1"]
    #[inline(always)]
    pub fn dll_ctrl_slv_dly_target1(&self) -> DLL_CTRL_SLV_DLY_TARGET1_R {
        DLL_CTRL_SLV_DLY_TARGET1_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:27 - DLL_CTRL_SLV_UPDATE_INT"]
    #[inline(always)]
    pub fn dll_ctrl_slv_update_int(&self) -> DLL_CTRL_SLV_UPDATE_INT_R {
        DLL_CTRL_SLV_UPDATE_INT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - DLL_CTRL_REF_UPDATE_INT"]
    #[inline(always)]
    pub fn dll_ctrl_ref_update_int(&self) -> DLL_CTRL_REF_UPDATE_INT_R {
        DLL_CTRL_REF_UPDATE_INT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL_CTRL_ENABLE"]
    #[inline(always)]
    pub fn dll_ctrl_enable(&mut self) -> DLL_CTRL_ENABLE_W {
        DLL_CTRL_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - DLL_CTRL_RESET"]
    #[inline(always)]
    pub fn dll_ctrl_reset(&mut self) -> DLL_CTRL_RESET_W {
        DLL_CTRL_RESET_W { w: self }
    }
    #[doc = "Bit 2 - DLL_CTRL_SLV_FORCE_UPD"]
    #[inline(always)]
    pub fn dll_ctrl_slv_force_upd(&mut self) -> DLL_CTRL_SLV_FORCE_UPD_W {
        DLL_CTRL_SLV_FORCE_UPD_W { w: self }
    }
    #[doc = "Bits 3:6 - DLL_CTRL_SLV_DLY_TARGET0"]
    #[inline(always)]
    pub fn dll_ctrl_slv_dly_target0(&mut self) -> DLL_CTRL_SLV_DLY_TARGET0_W {
        DLL_CTRL_SLV_DLY_TARGET0_W { w: self }
    }
    #[doc = "Bit 7 - DLL_CTRL_GATE_UPDATE"]
    #[inline(always)]
    pub fn dll_ctrl_gate_update(&mut self) -> DLL_CTRL_GATE_UPDATE_W {
        DLL_CTRL_GATE_UPDATE_W { w: self }
    }
    #[doc = "Bit 8 - DLL_CTRL_SLV_OVERRIDE"]
    #[inline(always)]
    pub fn dll_ctrl_slv_override(&mut self) -> DLL_CTRL_SLV_OVERRIDE_W {
        DLL_CTRL_SLV_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 9:15 - DLL_CTRL_SLV_OVERRIDE_VAL"]
    #[inline(always)]
    pub fn dll_ctrl_slv_override_val(&mut self) -> DLL_CTRL_SLV_OVERRIDE_VAL_W {
        DLL_CTRL_SLV_OVERRIDE_VAL_W { w: self }
    }
    #[doc = "Bits 16:18 - DLL_CTRL_SLV_DLY_TARGET1"]
    #[inline(always)]
    pub fn dll_ctrl_slv_dly_target1(&mut self) -> DLL_CTRL_SLV_DLY_TARGET1_W {
        DLL_CTRL_SLV_DLY_TARGET1_W { w: self }
    }
    #[doc = "Bits 20:27 - DLL_CTRL_SLV_UPDATE_INT"]
    #[inline(always)]
    pub fn dll_ctrl_slv_update_int(&mut self) -> DLL_CTRL_SLV_UPDATE_INT_W {
        DLL_CTRL_SLV_UPDATE_INT_W { w: self }
    }
    #[doc = "Bits 28:31 - DLL_CTRL_REF_UPDATE_INT"]
    #[inline(always)]
    pub fn dll_ctrl_ref_update_int(&mut self) -> DLL_CTRL_REF_UPDATE_INT_W {
        DLL_CTRL_REF_UPDATE_INT_W { w: self }
    }
}
