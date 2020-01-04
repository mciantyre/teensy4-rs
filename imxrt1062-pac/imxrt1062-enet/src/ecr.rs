#[doc = "Reader of register ECR"]
pub type R = crate::R<u32, super::ECR>;
#[doc = "Writer for register ECR"]
pub type W = crate::W<u32, super::ECR>;
#[doc = "Register ECR `reset()`'s with value 0x7000_0000"]
impl crate::ResetValue for super::ECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7000_0000
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Ethernet Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHEREN_A {
    #[doc = "0: Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    ETHEREN_0 = 0,
    #[doc = "1: MAC is enabled, and reception and transmission are possible."]
    ETHEREN_1 = 1,
}
impl From<ETHEREN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHEREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHEREN`"]
pub type ETHEREN_R = crate::R<bool, ETHEREN_A>;
impl ETHEREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHEREN_A {
        match self.bits {
            false => ETHEREN_A::ETHEREN_0,
            true => ETHEREN_A::ETHEREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ETHEREN_0`"]
    #[inline(always)]
    pub fn is_etheren_0(&self) -> bool {
        *self == ETHEREN_A::ETHEREN_0
    }
    #[doc = "Checks if the value of the field is `ETHEREN_1`"]
    #[inline(always)]
    pub fn is_etheren_1(&self) -> bool {
        *self == ETHEREN_A::ETHEREN_1
    }
}
#[doc = "Write proxy for field `ETHEREN`"]
pub struct ETHEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHEREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHEREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    #[inline(always)]
    pub fn etheren_0(self) -> &'a mut W {
        self.variant(ETHEREN_A::ETHEREN_0)
    }
    #[doc = "MAC is enabled, and reception and transmission are possible."]
    #[inline(always)]
    pub fn etheren_1(self) -> &'a mut W {
        self.variant(ETHEREN_A::ETHEREN_1)
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
#[doc = "Magic Packet Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAGICEN_A {
    #[doc = "0: Magic detection logic disabled."]
    MAGICEN_0 = 0,
    #[doc = "1: The MAC core detects magic packets and asserts EIR\\[WAKEUP\\]
when a frame is detected."]
    MAGICEN_1 = 1,
}
impl From<MAGICEN_A> for bool {
    #[inline(always)]
    fn from(variant: MAGICEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAGICEN`"]
pub type MAGICEN_R = crate::R<bool, MAGICEN_A>;
impl MAGICEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAGICEN_A {
        match self.bits {
            false => MAGICEN_A::MAGICEN_0,
            true => MAGICEN_A::MAGICEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAGICEN_0`"]
    #[inline(always)]
    pub fn is_magicen_0(&self) -> bool {
        *self == MAGICEN_A::MAGICEN_0
    }
    #[doc = "Checks if the value of the field is `MAGICEN_1`"]
    #[inline(always)]
    pub fn is_magicen_1(&self) -> bool {
        *self == MAGICEN_A::MAGICEN_1
    }
}
#[doc = "Write proxy for field `MAGICEN`"]
pub struct MAGICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAGICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAGICEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Magic detection logic disabled."]
    #[inline(always)]
    pub fn magicen_0(self) -> &'a mut W {
        self.variant(MAGICEN_A::MAGICEN_0)
    }
    #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\]
when a frame is detected."]
    #[inline(always)]
    pub fn magicen_1(self) -> &'a mut W {
        self.variant(MAGICEN_A::MAGICEN_1)
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
#[doc = "Sleep Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Normal operating mode."]
    SLEEP_0 = 0,
    #[doc = "1: Sleep mode."]
    SLEEP_1 = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, SLEEP_A>;
impl SLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::SLEEP_0,
            true => SLEEP_A::SLEEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEP_0`"]
    #[inline(always)]
    pub fn is_sleep_0(&self) -> bool {
        *self == SLEEP_A::SLEEP_0
    }
    #[doc = "Checks if the value of the field is `SLEEP_1`"]
    #[inline(always)]
    pub fn is_sleep_1(&self) -> bool {
        *self == SLEEP_A::SLEEP_1
    }
}
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operating mode."]
    #[inline(always)]
    pub fn sleep_0(self) -> &'a mut W {
        self.variant(SLEEP_A::SLEEP_0)
    }
    #[doc = "Sleep mode."]
    #[inline(always)]
    pub fn sleep_1(self) -> &'a mut W {
        self.variant(SLEEP_A::SLEEP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "EN1588 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1588_A {
    #[doc = "0: Legacy FEC buffer descriptors and functions enabled."]
    EN1588_0 = 0,
    #[doc = "1: Enhanced frame time-stamping functions enabled."]
    EN1588_1 = 1,
}
impl From<EN1588_A> for bool {
    #[inline(always)]
    fn from(variant: EN1588_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN1588`"]
pub type EN1588_R = crate::R<bool, EN1588_A>;
impl EN1588_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1588_A {
        match self.bits {
            false => EN1588_A::EN1588_0,
            true => EN1588_A::EN1588_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN1588_0`"]
    #[inline(always)]
    pub fn is_en1588_0(&self) -> bool {
        *self == EN1588_A::EN1588_0
    }
    #[doc = "Checks if the value of the field is `EN1588_1`"]
    #[inline(always)]
    pub fn is_en1588_1(&self) -> bool {
        *self == EN1588_A::EN1588_1
    }
}
#[doc = "Write proxy for field `EN1588`"]
pub struct EN1588_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1588_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN1588_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Legacy FEC buffer descriptors and functions enabled."]
    #[inline(always)]
    pub fn en1588_0(self) -> &'a mut W {
        self.variant(EN1588_A::EN1588_0)
    }
    #[doc = "Enhanced frame time-stamping functions enabled."]
    #[inline(always)]
    pub fn en1588_1(self) -> &'a mut W {
        self.variant(EN1588_A::EN1588_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_A {
    #[doc = "0: MAC continues operation in debug mode."]
    DBGEN_0 = 0,
    #[doc = "1: MAC enters hardware freeze mode when the processor is in debug mode."]
    DBGEN_1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBGEN`"]
pub type DBGEN_R = crate::R<bool, DBGEN_A>;
impl DBGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::DBGEN_0,
            true => DBGEN_A::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline(always)]
    pub fn is_dbgen_0(&self) -> bool {
        *self == DBGEN_A::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline(always)]
    pub fn is_dbgen_1(&self) -> bool {
        *self == DBGEN_A::DBGEN_1
    }
}
#[doc = "Write proxy for field `DBGEN`"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MAC continues operation in debug mode."]
    #[inline(always)]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_0)
    }
    #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
    #[inline(always)]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Descriptor Byte Swapping Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBSWP_A {
    #[doc = "0: The buffer descriptor bytes are not swapped to support big-endian devices."]
    DBSWP_0 = 0,
    #[doc = "1: The buffer descriptor bytes are swapped to support little-endian devices."]
    DBSWP_1 = 1,
}
impl From<DBSWP_A> for bool {
    #[inline(always)]
    fn from(variant: DBSWP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBSWP`"]
pub type DBSWP_R = crate::R<bool, DBSWP_A>;
impl DBSWP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBSWP_A {
        match self.bits {
            false => DBSWP_A::DBSWP_0,
            true => DBSWP_A::DBSWP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBSWP_0`"]
    #[inline(always)]
    pub fn is_dbswp_0(&self) -> bool {
        *self == DBSWP_A::DBSWP_0
    }
    #[doc = "Checks if the value of the field is `DBSWP_1`"]
    #[inline(always)]
    pub fn is_dbswp_1(&self) -> bool {
        *self == DBSWP_A::DBSWP_1
    }
}
#[doc = "Write proxy for field `DBSWP`"]
pub struct DBSWP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBSWP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
    #[inline(always)]
    pub fn dbswp_0(self) -> &'a mut W {
        self.variant(DBSWP_A::DBSWP_0)
    }
    #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
    #[inline(always)]
    pub fn dbswp_1(self) -> &'a mut W {
        self.variant(DBSWP_A::DBSWP_1)
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
impl R {
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline(always)]
    pub fn etheren(&self) -> ETHEREN_R {
        ETHEREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn magicen(&self) -> MAGICEN_R {
        MAGICEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline(always)]
    pub fn en1588(&self) -> EN1588_R {
        EN1588_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub fn dbswp(&self) -> DBSWP_R {
        DBSWP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline(always)]
    pub fn etheren(&mut self) -> ETHEREN_W {
        ETHEREN_W { w: self }
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn magicen(&mut self) -> MAGICEN_W {
        MAGICEN_W { w: self }
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline(always)]
    pub fn en1588(&mut self) -> EN1588_W {
        EN1588_W { w: self }
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub fn dbswp(&mut self) -> DBSWP_W {
        DBSWP_W { w: self }
    }
}
