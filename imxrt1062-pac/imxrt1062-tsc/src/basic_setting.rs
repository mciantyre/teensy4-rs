#[doc = "Reader of register BASIC_SETTING"]
pub type R = crate::R<u32, super::BASIC_SETTING>;
#[doc = "Writer for register BASIC_SETTING"]
pub type W = crate::W<u32, super::BASIC_SETTING>;
#[doc = "Register BASIC_SETTING `reset()`'s with value 0"]
impl crate::ResetValue for super::BASIC_SETTING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Auto Measure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_MEASURE_A {
    #[doc = "0: Disable Auto Measure"]
    AUTO_MEASURE_0 = 0,
    #[doc = "1: Auto Measure"]
    AUTO_MEASURE_1 = 1,
}
impl From<AUTO_MEASURE_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_MEASURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTO_MEASURE`"]
pub type AUTO_MEASURE_R = crate::R<bool, AUTO_MEASURE_A>;
impl AUTO_MEASURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_MEASURE_A {
        match self.bits {
            false => AUTO_MEASURE_A::AUTO_MEASURE_0,
            true => AUTO_MEASURE_A::AUTO_MEASURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_MEASURE_0`"]
    #[inline(always)]
    pub fn is_auto_measure_0(&self) -> bool {
        *self == AUTO_MEASURE_A::AUTO_MEASURE_0
    }
    #[doc = "Checks if the value of the field is `AUTO_MEASURE_1`"]
    #[inline(always)]
    pub fn is_auto_measure_1(&self) -> bool {
        *self == AUTO_MEASURE_A::AUTO_MEASURE_1
    }
}
#[doc = "Write proxy for field `AUTO_MEASURE`"]
pub struct AUTO_MEASURE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_MEASURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTO_MEASURE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Auto Measure"]
    #[inline(always)]
    pub fn auto_measure_0(self) -> &'a mut W {
        self.variant(AUTO_MEASURE_A::AUTO_MEASURE_0)
    }
    #[doc = "Auto Measure"]
    #[inline(always)]
    pub fn auto_measure_1(self) -> &'a mut W {
        self.variant(AUTO_MEASURE_A::AUTO_MEASURE_1)
    }
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
#[doc = "4/5 Wire detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum _4_5_WIRE_A {
    #[doc = "0: 4-Wire Detection Mode"]
    _4_5_WIRE_0 = 0,
    #[doc = "1: 5-Wire Detection Mode"]
    _4_5_WIRE_1 = 1,
}
impl From<_4_5_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: _4_5_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `_4_5_WIRE`"]
pub type _4_5_WIRE_R = crate::R<bool, _4_5_WIRE_A>;
impl _4_5_WIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> _4_5_WIRE_A {
        match self.bits {
            false => _4_5_WIRE_A::_4_5_WIRE_0,
            true => _4_5_WIRE_A::_4_5_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `_4_5_WIRE_0`"]
    #[inline(always)]
    pub fn is_4_5_wire_0(&self) -> bool {
        *self == _4_5_WIRE_A::_4_5_WIRE_0
    }
    #[doc = "Checks if the value of the field is `_4_5_WIRE_1`"]
    #[inline(always)]
    pub fn is_4_5_wire_1(&self) -> bool {
        *self == _4_5_WIRE_A::_4_5_WIRE_1
    }
}
#[doc = "Write proxy for field `_4_5_WIRE`"]
pub struct _4_5_WIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> _4_5_WIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: _4_5_WIRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "4-Wire Detection Mode"]
    #[inline(always)]
    pub fn _4_5_wire_0(self) -> &'a mut W {
        self.variant(_4_5_WIRE_A::_4_5_WIRE_0)
    }
    #[doc = "5-Wire Detection Mode"]
    #[inline(always)]
    pub fn _4_5_wire_1(self) -> &'a mut W {
        self.variant(_4_5_WIRE_A::_4_5_WIRE_1)
    }
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
#[doc = "Reader of field `MEASURE_DELAY_TIME`"]
pub type MEASURE_DELAY_TIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MEASURE_DELAY_TIME`"]
pub struct MEASURE_DELAY_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEASURE_DELAY_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto Measure"]
    #[inline(always)]
    pub fn auto_measure(&self) -> AUTO_MEASURE_R {
        AUTO_MEASURE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4/5 Wire detection"]
    #[inline(always)]
    pub fn _4_5_wire(&self) -> _4_5_WIRE_R {
        _4_5_WIRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Measure Delay Time"]
    #[inline(always)]
    pub fn measure_delay_time(&self) -> MEASURE_DELAY_TIME_R {
        MEASURE_DELAY_TIME_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Measure"]
    #[inline(always)]
    pub fn auto_measure(&mut self) -> AUTO_MEASURE_W {
        AUTO_MEASURE_W { w: self }
    }
    #[doc = "Bit 4 - 4/5 Wire detection"]
    #[inline(always)]
    pub fn _4_5_wire(&mut self) -> _4_5_WIRE_W {
        _4_5_WIRE_W { w: self }
    }
    #[doc = "Bits 8:31 - Measure Delay Time"]
    #[inline(always)]
    pub fn measure_delay_time(&mut self) -> MEASURE_DELAY_TIME_W {
        MEASURE_DELAY_TIME_W { w: self }
    }
}
