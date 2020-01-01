#[doc = "Reader of register SCTRL%s"]
pub type R = crate::R<u16, super::SCTRL>;
#[doc = "Writer for register SCTRL%s"]
pub type W = crate::W<u16, super::SCTRL>;
#[doc = "Register SCTRL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::SCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEN_A {
    #[doc = "0: The external pin is configured as an input."]
    OEN_0 = 0,
    #[doc = "1: The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    OEN_1 = 1,
}
impl From<OEN_A> for bool {
    #[inline(always)]
    fn from(variant: OEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OEN`"]
pub type OEN_R = crate::R<bool, OEN_A>;
impl OEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEN_A {
        match self.bits {
            false => OEN_A::OEN_0,
            true => OEN_A::OEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OEN_0`"]
    #[inline(always)]
    pub fn is_oen_0(&self) -> bool {
        *self == OEN_A::OEN_0
    }
    #[doc = "Checks if the value of the field is `OEN_1`"]
    #[inline(always)]
    pub fn is_oen_1(&self) -> bool {
        *self == OEN_A::OEN_1
    }
}
#[doc = "Write proxy for field `OEN`"]
pub struct OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The external pin is configured as an input."]
    #[inline(always)]
    pub fn oen_0(self) -> &'a mut W {
        self.variant(OEN_A::OEN_0)
    }
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    #[inline(always)]
    pub fn oen_1(self) -> &'a mut W {
        self.variant(OEN_A::OEN_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Output Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPS_A {
    #[doc = "0: True polarity."]
    OPS_0 = 0,
    #[doc = "1: Inverted polarity."]
    OPS_1 = 1,
}
impl From<OPS_A> for bool {
    #[inline(always)]
    fn from(variant: OPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OPS`"]
pub type OPS_R = crate::R<bool, OPS_A>;
impl OPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPS_A {
        match self.bits {
            false => OPS_A::OPS_0,
            true => OPS_A::OPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OPS_0`"]
    #[inline(always)]
    pub fn is_ops_0(&self) -> bool {
        *self == OPS_A::OPS_0
    }
    #[doc = "Checks if the value of the field is `OPS_1`"]
    #[inline(always)]
    pub fn is_ops_1(&self) -> bool {
        *self == OPS_A::OPS_1
    }
}
#[doc = "Write proxy for field `OPS`"]
pub struct OPS_W<'a> {
    w: &'a mut W,
}
impl<'a> OPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "True polarity."]
    #[inline(always)]
    pub fn ops_0(self) -> &'a mut W {
        self.variant(OPS_A::OPS_0)
    }
    #[doc = "Inverted polarity."]
    #[inline(always)]
    pub fn ops_1(self) -> &'a mut W {
        self.variant(OPS_A::OPS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FORCE`"]
pub type FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE`"]
pub struct FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VAL`"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EEOF`"]
pub type EEOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EEOF`"]
pub struct EEOF_W<'a> {
    w: &'a mut W,
}
impl<'a> EEOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MSTR`"]
pub type MSTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSTR`"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Input Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTURE_MODE_A {
    #[doc = "0: Capture function is disabled"]
    CAPTURE_MODE_0 = 0,
    #[doc = "1: Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    CAPTURE_MODE_1 = 1,
    #[doc = "2: Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    CAPTURE_MODE_2 = 2,
    #[doc = "3: Load capture register on both edges of input"]
    CAPTURE_MODE_3 = 3,
}
impl From<CAPTURE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTURE_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CAPTURE_MODE`"]
pub type CAPTURE_MODE_R = crate::R<u8, CAPTURE_MODE_A>;
impl CAPTURE_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_MODE_A {
        match self.bits {
            0 => CAPTURE_MODE_A::CAPTURE_MODE_0,
            1 => CAPTURE_MODE_A::CAPTURE_MODE_1,
            2 => CAPTURE_MODE_A::CAPTURE_MODE_2,
            3 => CAPTURE_MODE_A::CAPTURE_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE_MODE_0`"]
    #[inline(always)]
    pub fn is_capture_mode_0(&self) -> bool {
        *self == CAPTURE_MODE_A::CAPTURE_MODE_0
    }
    #[doc = "Checks if the value of the field is `CAPTURE_MODE_1`"]
    #[inline(always)]
    pub fn is_capture_mode_1(&self) -> bool {
        *self == CAPTURE_MODE_A::CAPTURE_MODE_1
    }
    #[doc = "Checks if the value of the field is `CAPTURE_MODE_2`"]
    #[inline(always)]
    pub fn is_capture_mode_2(&self) -> bool {
        *self == CAPTURE_MODE_A::CAPTURE_MODE_2
    }
    #[doc = "Checks if the value of the field is `CAPTURE_MODE_3`"]
    #[inline(always)]
    pub fn is_capture_mode_3(&self) -> bool {
        *self == CAPTURE_MODE_A::CAPTURE_MODE_3
    }
}
#[doc = "Write proxy for field `CAPTURE_MODE`"]
pub struct CAPTURE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Capture function is disabled"]
    #[inline(always)]
    pub fn capture_mode_0(self) -> &'a mut W {
        self.variant(CAPTURE_MODE_A::CAPTURE_MODE_0)
    }
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    #[inline(always)]
    pub fn capture_mode_1(self) -> &'a mut W {
        self.variant(CAPTURE_MODE_A::CAPTURE_MODE_1)
    }
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    #[inline(always)]
    pub fn capture_mode_2(self) -> &'a mut W {
        self.variant(CAPTURE_MODE_A::CAPTURE_MODE_2)
    }
    #[doc = "Load capture register on both edges of input"]
    #[inline(always)]
    pub fn capture_mode_3(self) -> &'a mut W {
        self.variant(CAPTURE_MODE_A::CAPTURE_MODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `INPUT`"]
pub type INPUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPS`"]
pub type IPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPS`"]
pub struct IPS_W<'a> {
    w: &'a mut W,
}
impl<'a> IPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `IEFIE`"]
pub type IEFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEFIE`"]
pub struct IEFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IEFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `IEF`"]
pub type IEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEF`"]
pub struct IEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TOFIE`"]
pub type TOFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOFIE`"]
pub struct TOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TOF`"]
pub type TOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOF`"]
pub struct TOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TCFIE`"]
pub type TCFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCFIE`"]
pub struct TCFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCF`"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output Enable"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Polarity Select"]
    #[inline(always)]
    pub fn ops(&self) -> OPS_R {
        OPS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force OFLAG Output"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Forced OFLAG Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable External OFLAG Force"]
    #[inline(always)]
    pub fn eeof(&self) -> EEOF_R {
        EEOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Master Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Input Capture Mode"]
    #[inline(always)]
    pub fn capture_mode(&self) -> CAPTURE_MODE_R {
        CAPTURE_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - External Input Signal"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Input Polarity Select"]
    #[inline(always)]
    pub fn ips(&self) -> IPS_R {
        IPS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub fn iefie(&self) -> IEFIE_R {
        IEFIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input Edge Flag"]
    #[inline(always)]
    pub fn ief(&self) -> IEF_R {
        IEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub fn tofie(&self) -> TOFIE_R {
        TOFIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub fn tcfie(&self) -> TCFIE_R {
        TCFIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer Compare Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Enable"]
    #[inline(always)]
    pub fn oen(&mut self) -> OEN_W {
        OEN_W { w: self }
    }
    #[doc = "Bit 1 - Output Polarity Select"]
    #[inline(always)]
    pub fn ops(&mut self) -> OPS_W {
        OPS_W { w: self }
    }
    #[doc = "Bit 2 - Force OFLAG Output"]
    #[inline(always)]
    pub fn force(&mut self) -> FORCE_W {
        FORCE_W { w: self }
    }
    #[doc = "Bit 3 - Forced OFLAG Value"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Bit 4 - Enable External OFLAG Force"]
    #[inline(always)]
    pub fn eeof(&mut self) -> EEOF_W {
        EEOF_W { w: self }
    }
    #[doc = "Bit 5 - Master Mode"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    #[doc = "Bits 6:7 - Input Capture Mode"]
    #[inline(always)]
    pub fn capture_mode(&mut self) -> CAPTURE_MODE_W {
        CAPTURE_MODE_W { w: self }
    }
    #[doc = "Bit 9 - Input Polarity Select"]
    #[inline(always)]
    pub fn ips(&mut self) -> IPS_W {
        IPS_W { w: self }
    }
    #[doc = "Bit 10 - Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub fn iefie(&mut self) -> IEFIE_W {
        IEFIE_W { w: self }
    }
    #[doc = "Bit 11 - Input Edge Flag"]
    #[inline(always)]
    pub fn ief(&mut self) -> IEF_W {
        IEF_W { w: self }
    }
    #[doc = "Bit 12 - Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub fn tofie(&mut self) -> TOFIE_W {
        TOFIE_W { w: self }
    }
    #[doc = "Bit 13 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W {
        TOF_W { w: self }
    }
    #[doc = "Bit 14 - Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub fn tcfie(&mut self) -> TCFIE_W {
        TCFIE_W { w: self }
    }
    #[doc = "Bit 15 - Timer Compare Flag"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
}
