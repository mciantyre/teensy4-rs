#[doc = "Reader of register PIGEONCTRL2"]
pub type R = crate::R<u32, super::PIGEONCTRL2>;
#[doc = "Writer for register PIGEONCTRL2"]
pub type W = crate::W<u32, super::PIGEONCTRL2>;
#[doc = "Register PIGEONCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIGEONCTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIGEON_DATA_EN`"]
pub type PIGEON_DATA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIGEON_DATA_EN`"]
pub struct PIGEON_DATA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIGEON_DATA_EN_W<'a> {
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
#[doc = "Reader of field `PIGEON_CLK_GATE`"]
pub type PIGEON_CLK_GATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIGEON_CLK_GATE`"]
pub struct PIGEON_CLK_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIGEON_CLK_GATE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Pigeon mode data enable"]
    #[inline(always)]
    pub fn pigeon_data_en(&self) -> PIGEON_DATA_EN_R {
        PIGEON_DATA_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pigeon mode dot clock gate enable"]
    #[inline(always)]
    pub fn pigeon_clk_gate(&self) -> PIGEON_CLK_GATE_R {
        PIGEON_CLK_GATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pigeon mode data enable"]
    #[inline(always)]
    pub fn pigeon_data_en(&mut self) -> PIGEON_DATA_EN_W {
        PIGEON_DATA_EN_W { w: self }
    }
    #[doc = "Bit 1 - Pigeon mode dot clock gate enable"]
    #[inline(always)]
    pub fn pigeon_clk_gate(&mut self) -> PIGEON_CLK_GATE_W {
        PIGEON_CLK_GATE_W { w: self }
    }
}
