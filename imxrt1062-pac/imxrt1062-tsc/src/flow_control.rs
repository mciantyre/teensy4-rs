#[doc = "Reader of register FLOW_CONTROL"]
pub type R = crate::R<u32, super::FLOW_CONTROL>;
#[doc = "Writer for register FLOW_CONTROL"]
pub type W = crate::W<u32, super::FLOW_CONTROL>;
#[doc = "Register FLOW_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLOW_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_RST`"]
pub type SW_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_RST`"]
pub struct SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_W<'a> {
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
#[doc = "Start Measure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_MEASURE_A {
    #[doc = "0: Do not start measure for now"]
    START_MEASURE_0 = 0,
    #[doc = "1: Start measure the X/Y coordinate value"]
    START_MEASURE_1 = 1,
}
impl From<START_MEASURE_A> for bool {
    #[inline(always)]
    fn from(variant: START_MEASURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START_MEASURE`"]
pub type START_MEASURE_R = crate::R<bool, START_MEASURE_A>;
impl START_MEASURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_MEASURE_A {
        match self.bits {
            false => START_MEASURE_A::START_MEASURE_0,
            true => START_MEASURE_A::START_MEASURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `START_MEASURE_0`"]
    #[inline(always)]
    pub fn is_start_measure_0(&self) -> bool {
        *self == START_MEASURE_A::START_MEASURE_0
    }
    #[doc = "Checks if the value of the field is `START_MEASURE_1`"]
    #[inline(always)]
    pub fn is_start_measure_1(&self) -> bool {
        *self == START_MEASURE_A::START_MEASURE_1
    }
}
#[doc = "Write proxy for field `START_MEASURE`"]
pub struct START_MEASURE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_MEASURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_MEASURE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not start measure for now"]
    #[inline(always)]
    pub fn start_measure_0(self) -> &'a mut W {
        self.variant(START_MEASURE_A::START_MEASURE_0)
    }
    #[doc = "Start measure the X/Y coordinate value"]
    #[inline(always)]
    pub fn start_measure_1(self) -> &'a mut W {
        self.variant(START_MEASURE_A::START_MEASURE_1)
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
#[doc = "Drop Measure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DROP_MEASURE_A {
    #[doc = "0: Do not drop measure for now"]
    DROP_MEASURE_0 = 0,
    #[doc = "1: Drop the measure and controller return to idle status"]
    DROP_MEASURE_1 = 1,
}
impl From<DROP_MEASURE_A> for bool {
    #[inline(always)]
    fn from(variant: DROP_MEASURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DROP_MEASURE`"]
pub type DROP_MEASURE_R = crate::R<bool, DROP_MEASURE_A>;
impl DROP_MEASURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DROP_MEASURE_A {
        match self.bits {
            false => DROP_MEASURE_A::DROP_MEASURE_0,
            true => DROP_MEASURE_A::DROP_MEASURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DROP_MEASURE_0`"]
    #[inline(always)]
    pub fn is_drop_measure_0(&self) -> bool {
        *self == DROP_MEASURE_A::DROP_MEASURE_0
    }
    #[doc = "Checks if the value of the field is `DROP_MEASURE_1`"]
    #[inline(always)]
    pub fn is_drop_measure_1(&self) -> bool {
        *self == DROP_MEASURE_A::DROP_MEASURE_1
    }
}
#[doc = "Write proxy for field `DROP_MEASURE`"]
pub struct DROP_MEASURE_W<'a> {
    w: &'a mut W,
}
impl<'a> DROP_MEASURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DROP_MEASURE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not drop measure for now"]
    #[inline(always)]
    pub fn drop_measure_0(self) -> &'a mut W {
        self.variant(DROP_MEASURE_A::DROP_MEASURE_0)
    }
    #[doc = "Drop the measure and controller return to idle status"]
    #[inline(always)]
    pub fn drop_measure_1(self) -> &'a mut W {
        self.variant(DROP_MEASURE_A::DROP_MEASURE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Start Sense\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_SENSE_A {
    #[doc = "0: Stay at idle status"]
    START_SENSE_0 = 0,
    #[doc = "1: Start sense detection and (if auto_measure set to 1) measure after detect a touch"]
    START_SENSE_1 = 1,
}
impl From<START_SENSE_A> for bool {
    #[inline(always)]
    fn from(variant: START_SENSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START_SENSE`"]
pub type START_SENSE_R = crate::R<bool, START_SENSE_A>;
impl START_SENSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_SENSE_A {
        match self.bits {
            false => START_SENSE_A::START_SENSE_0,
            true => START_SENSE_A::START_SENSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `START_SENSE_0`"]
    #[inline(always)]
    pub fn is_start_sense_0(&self) -> bool {
        *self == START_SENSE_A::START_SENSE_0
    }
    #[doc = "Checks if the value of the field is `START_SENSE_1`"]
    #[inline(always)]
    pub fn is_start_sense_1(&self) -> bool {
        *self == START_SENSE_A::START_SENSE_1
    }
}
#[doc = "Write proxy for field `START_SENSE`"]
pub struct START_SENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SENSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_SENSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stay at idle status"]
    #[inline(always)]
    pub fn start_sense_0(self) -> &'a mut W {
        self.variant(START_SENSE_A::START_SENSE_0)
    }
    #[doc = "Start sense detection and (if auto_measure set to 1) measure after detect a touch"]
    #[inline(always)]
    pub fn start_sense_1(self) -> &'a mut W {
        self.variant(START_SENSE_A::START_SENSE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "This bit is for SW disable registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_A {
    #[doc = "0: Leave HW state machine control"]
    DISABLE_0 = 0,
    #[doc = "1: SW set to idle status"]
    DISABLE_1 = 1,
}
impl From<DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLE`"]
pub type DISABLE_R = crate::R<bool, DISABLE_A>;
impl DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLE_A {
        match self.bits {
            false => DISABLE_A::DISABLE_0,
            true => DISABLE_A::DISABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_0`"]
    #[inline(always)]
    pub fn is_disable_0(&self) -> bool {
        *self == DISABLE_A::DISABLE_0
    }
    #[doc = "Checks if the value of the field is `DISABLE_1`"]
    #[inline(always)]
    pub fn is_disable_1(&self) -> bool {
        *self == DISABLE_A::DISABLE_1
    }
}
#[doc = "Write proxy for field `DISABLE`"]
pub struct DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Leave HW state machine control"]
    #[inline(always)]
    pub fn disable_0(self) -> &'a mut W {
        self.variant(DISABLE_A::DISABLE_0)
    }
    #[doc = "SW set to idle status"]
    #[inline(always)]
    pub fn disable_1(self) -> &'a mut W {
        self.variant(DISABLE_A::DISABLE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Soft Reset"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Start Measure"]
    #[inline(always)]
    pub fn start_measure(&self) -> START_MEASURE_R {
        START_MEASURE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Drop Measure"]
    #[inline(always)]
    pub fn drop_measure(&self) -> DROP_MEASURE_R {
        DROP_MEASURE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Start Sense"]
    #[inline(always)]
    pub fn start_sense(&self) -> START_SENSE_R {
        START_SENSE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit is for SW disable registers"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset"]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SW_RST_W {
        SW_RST_W { w: self }
    }
    #[doc = "Bit 4 - Start Measure"]
    #[inline(always)]
    pub fn start_measure(&mut self) -> START_MEASURE_W {
        START_MEASURE_W { w: self }
    }
    #[doc = "Bit 8 - Drop Measure"]
    #[inline(always)]
    pub fn drop_measure(&mut self) -> DROP_MEASURE_W {
        DROP_MEASURE_W { w: self }
    }
    #[doc = "Bit 12 - Start Sense"]
    #[inline(always)]
    pub fn start_sense(&mut self) -> START_SENSE_W {
        START_SENSE_W { w: self }
    }
    #[doc = "Bit 16 - This bit is for SW disable registers"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W { w: self }
    }
}
