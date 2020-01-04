#[doc = "Reader of register CFGR0"]
pub type R = crate::R<u32, super::CFGR0>;
#[doc = "Writer for register CFGR0"]
pub type W = crate::W<u32, super::CFGR0>;
#[doc = "Register CFGR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Host Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HREN_A {
    #[doc = "0: Host request is disabled"]
    HREN_0 = 0,
    #[doc = "1: Host request is enabled"]
    HREN_1 = 1,
}
impl From<HREN_A> for bool {
    #[inline(always)]
    fn from(variant: HREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HREN`"]
pub type HREN_R = crate::R<bool, HREN_A>;
impl HREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HREN_A {
        match self.bits {
            false => HREN_A::HREN_0,
            true => HREN_A::HREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HREN_0`"]
    #[inline(always)]
    pub fn is_hren_0(&self) -> bool {
        *self == HREN_A::HREN_0
    }
    #[doc = "Checks if the value of the field is `HREN_1`"]
    #[inline(always)]
    pub fn is_hren_1(&self) -> bool {
        *self == HREN_A::HREN_1
    }
}
#[doc = "Write proxy for field `HREN`"]
pub struct HREN_W<'a> {
    w: &'a mut W,
}
impl<'a> HREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Host request is disabled"]
    #[inline(always)]
    pub fn hren_0(self) -> &'a mut W {
        self.variant(HREN_A::HREN_0)
    }
    #[doc = "Host request is enabled"]
    #[inline(always)]
    pub fn hren_1(self) -> &'a mut W {
        self.variant(HREN_A::HREN_1)
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
#[doc = "Host Request Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRPOL_A {
    #[doc = "0: Active low"]
    HRPOL_0 = 0,
    #[doc = "1: Active high"]
    HRPOL_1 = 1,
}
impl From<HRPOL_A> for bool {
    #[inline(always)]
    fn from(variant: HRPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRPOL`"]
pub type HRPOL_R = crate::R<bool, HRPOL_A>;
impl HRPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRPOL_A {
        match self.bits {
            false => HRPOL_A::HRPOL_0,
            true => HRPOL_A::HRPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRPOL_0`"]
    #[inline(always)]
    pub fn is_hrpol_0(&self) -> bool {
        *self == HRPOL_A::HRPOL_0
    }
    #[doc = "Checks if the value of the field is `HRPOL_1`"]
    #[inline(always)]
    pub fn is_hrpol_1(&self) -> bool {
        *self == HRPOL_A::HRPOL_1
    }
}
#[doc = "Write proxy for field `HRPOL`"]
pub struct HRPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> HRPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn hrpol_0(self) -> &'a mut W {
        self.variant(HRPOL_A::HRPOL_0)
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn hrpol_1(self) -> &'a mut W {
        self.variant(HRPOL_A::HRPOL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Host Request Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRSEL_A {
    #[doc = "0: Host request input is the LPSPI_HREQ pin"]
    HRSEL_0 = 0,
    #[doc = "1: Host request input is the input trigger"]
    HRSEL_1 = 1,
}
impl From<HRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRSEL`"]
pub type HRSEL_R = crate::R<bool, HRSEL_A>;
impl HRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRSEL_A {
        match self.bits {
            false => HRSEL_A::HRSEL_0,
            true => HRSEL_A::HRSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRSEL_0`"]
    #[inline(always)]
    pub fn is_hrsel_0(&self) -> bool {
        *self == HRSEL_A::HRSEL_0
    }
    #[doc = "Checks if the value of the field is `HRSEL_1`"]
    #[inline(always)]
    pub fn is_hrsel_1(&self) -> bool {
        *self == HRSEL_A::HRSEL_1
    }
}
#[doc = "Write proxy for field `HRSEL`"]
pub struct HRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Host request input is the LPSPI_HREQ pin"]
    #[inline(always)]
    pub fn hrsel_0(self) -> &'a mut W {
        self.variant(HRSEL_A::HRSEL_0)
    }
    #[doc = "Host request input is the input trigger"]
    #[inline(always)]
    pub fn hrsel_1(self) -> &'a mut W {
        self.variant(HRSEL_A::HRSEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Circular FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRFIFO_A {
    #[doc = "0: Circular FIFO is disabled"]
    CIRFIFO_0 = 0,
    #[doc = "1: Circular FIFO is enabled"]
    CIRFIFO_1 = 1,
}
impl From<CIRFIFO_A> for bool {
    #[inline(always)]
    fn from(variant: CIRFIFO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CIRFIFO`"]
pub type CIRFIFO_R = crate::R<bool, CIRFIFO_A>;
impl CIRFIFO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIRFIFO_A {
        match self.bits {
            false => CIRFIFO_A::CIRFIFO_0,
            true => CIRFIFO_A::CIRFIFO_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIRFIFO_0`"]
    #[inline(always)]
    pub fn is_cirfifo_0(&self) -> bool {
        *self == CIRFIFO_A::CIRFIFO_0
    }
    #[doc = "Checks if the value of the field is `CIRFIFO_1`"]
    #[inline(always)]
    pub fn is_cirfifo_1(&self) -> bool {
        *self == CIRFIFO_A::CIRFIFO_1
    }
}
#[doc = "Write proxy for field `CIRFIFO`"]
pub struct CIRFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CIRFIFO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIRFIFO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Circular FIFO is disabled"]
    #[inline(always)]
    pub fn cirfifo_0(self) -> &'a mut W {
        self.variant(CIRFIFO_A::CIRFIFO_0)
    }
    #[doc = "Circular FIFO is enabled"]
    #[inline(always)]
    pub fn cirfifo_1(self) -> &'a mut W {
        self.variant(CIRFIFO_A::CIRFIFO_1)
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
#[doc = "Receive Data Match Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMO_A {
    #[doc = "0: Received data is stored in the receive FIFO as in normal operations"]
    RDMO_0 = 0,
    #[doc = "1: Received data is discarded unless the Data Match Flag (DMF) is set"]
    RDMO_1 = 1,
}
impl From<RDMO_A> for bool {
    #[inline(always)]
    fn from(variant: RDMO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDMO`"]
pub type RDMO_R = crate::R<bool, RDMO_A>;
impl RDMO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMO_A {
        match self.bits {
            false => RDMO_A::RDMO_0,
            true => RDMO_A::RDMO_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDMO_0`"]
    #[inline(always)]
    pub fn is_rdmo_0(&self) -> bool {
        *self == RDMO_A::RDMO_0
    }
    #[doc = "Checks if the value of the field is `RDMO_1`"]
    #[inline(always)]
    pub fn is_rdmo_1(&self) -> bool {
        *self == RDMO_A::RDMO_1
    }
}
#[doc = "Write proxy for field `RDMO`"]
pub struct RDMO_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Received data is stored in the receive FIFO as in normal operations"]
    #[inline(always)]
    pub fn rdmo_0(self) -> &'a mut W {
        self.variant(RDMO_A::RDMO_0)
    }
    #[doc = "Received data is discarded unless the Data Match Flag (DMF) is set"]
    #[inline(always)]
    pub fn rdmo_1(self) -> &'a mut W {
        self.variant(RDMO_A::RDMO_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline(always)]
    pub fn hren(&self) -> HREN_R {
        HREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline(always)]
    pub fn hrpol(&self) -> HRPOL_R {
        HRPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline(always)]
    pub fn hrsel(&self) -> HRSEL_R {
        HRSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline(always)]
    pub fn cirfifo(&self) -> CIRFIFO_R {
        CIRFIFO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline(always)]
    pub fn rdmo(&self) -> RDMO_R {
        RDMO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline(always)]
    pub fn hren(&mut self) -> HREN_W {
        HREN_W { w: self }
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline(always)]
    pub fn hrpol(&mut self) -> HRPOL_W {
        HRPOL_W { w: self }
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline(always)]
    pub fn hrsel(&mut self) -> HRSEL_W {
        HRSEL_W { w: self }
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline(always)]
    pub fn cirfifo(&mut self) -> CIRFIFO_W {
        CIRFIFO_W { w: self }
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline(always)]
    pub fn rdmo(&mut self) -> RDMO_W {
        RDMO_W { w: self }
    }
}
