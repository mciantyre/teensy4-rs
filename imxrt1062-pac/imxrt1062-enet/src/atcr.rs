#[doc = "Reader of register ATCR"]
pub type R = crate::R<u32, super::ATCR>;
#[doc = "Writer for register ATCR"]
pub type W = crate::W<u32, super::ATCR>;
#[doc = "Register ATCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ATCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: The timer stops at the current value."]
    EN_0 = 0,
    #[doc = "1: The timer starts incrementing."]
    EN_1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == EN_A::EN_1
    }
}
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer stops at the current value."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "The timer starts incrementing."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
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
#[doc = "Enable One-Shot Offset Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFEN_A {
    #[doc = "0: Disable."]
    OFFEN_0 = 0,
    #[doc = "1: The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    OFFEN_1 = 1,
}
impl From<OFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFFEN`"]
pub type OFFEN_R = crate::R<bool, OFFEN_A>;
impl OFFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFEN_A {
        match self.bits {
            false => OFFEN_A::OFFEN_0,
            true => OFFEN_A::OFFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OFFEN_0`"]
    #[inline(always)]
    pub fn is_offen_0(&self) -> bool {
        *self == OFFEN_A::OFFEN_0
    }
    #[doc = "Checks if the value of the field is `OFFEN_1`"]
    #[inline(always)]
    pub fn is_offen_1(&self) -> bool {
        *self == OFFEN_A::OFFEN_1
    }
}
#[doc = "Write proxy for field `OFFEN`"]
pub struct OFFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn offen_0(self) -> &'a mut W {
        self.variant(OFFEN_A::OFFEN_0)
    }
    #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    #[inline(always)]
    pub fn offen_1(self) -> &'a mut W {
        self.variant(OFFEN_A::OFFEN_1)
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
#[doc = "Reset Timer On Offset Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRST_A {
    #[doc = "0: The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    OFFRST_0 = 0,
    #[doc = "1: If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    OFFRST_1 = 1,
}
impl From<OFFRST_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFFRST`"]
pub type OFFRST_R = crate::R<bool, OFFRST_A>;
impl OFFRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRST_A {
        match self.bits {
            false => OFFRST_A::OFFRST_0,
            true => OFFRST_A::OFFRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `OFFRST_0`"]
    #[inline(always)]
    pub fn is_offrst_0(&self) -> bool {
        *self == OFFRST_A::OFFRST_0
    }
    #[doc = "Checks if the value of the field is `OFFRST_1`"]
    #[inline(always)]
    pub fn is_offrst_1(&self) -> bool {
        *self == OFFRST_A::OFFRST_1
    }
}
#[doc = "Write proxy for field `OFFRST`"]
pub struct OFFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    #[inline(always)]
    pub fn offrst_0(self) -> &'a mut W {
        self.variant(OFFRST_A::OFFRST_0)
    }
    #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    #[inline(always)]
    pub fn offrst_1(self) -> &'a mut W {
        self.variant(OFFRST_A::OFFRST_1)
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
#[doc = "Enable Periodical Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEREN_A {
    #[doc = "0: Disable."]
    PEREN_0 = 0,
    #[doc = "1: A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    PEREN_1 = 1,
}
impl From<PEREN_A> for bool {
    #[inline(always)]
    fn from(variant: PEREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEREN`"]
pub type PEREN_R = crate::R<bool, PEREN_A>;
impl PEREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEREN_A {
        match self.bits {
            false => PEREN_A::PEREN_0,
            true => PEREN_A::PEREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PEREN_0`"]
    #[inline(always)]
    pub fn is_peren_0(&self) -> bool {
        *self == PEREN_A::PEREN_0
    }
    #[doc = "Checks if the value of the field is `PEREN_1`"]
    #[inline(always)]
    pub fn is_peren_1(&self) -> bool {
        *self == PEREN_A::PEREN_1
    }
}
#[doc = "Write proxy for field `PEREN`"]
pub struct PEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn peren_0(self) -> &'a mut W {
        self.variant(PEREN_A::PEREN_0)
    }
    #[doc = "A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    #[inline(always)]
    pub fn peren_1(self) -> &'a mut W {
        self.variant(PEREN_A::PEREN_1)
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
#[doc = "Enables event signal output assertion on period event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPER_A {
    #[doc = "0: Disable."]
    PINPER_0 = 0,
    #[doc = "1: Enable."]
    PINPER_1 = 1,
}
impl From<PINPER_A> for bool {
    #[inline(always)]
    fn from(variant: PINPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINPER`"]
pub type PINPER_R = crate::R<bool, PINPER_A>;
impl PINPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINPER_A {
        match self.bits {
            false => PINPER_A::PINPER_0,
            true => PINPER_A::PINPER_1,
        }
    }
    #[doc = "Checks if the value of the field is `PINPER_0`"]
    #[inline(always)]
    pub fn is_pinper_0(&self) -> bool {
        *self == PINPER_A::PINPER_0
    }
    #[doc = "Checks if the value of the field is `PINPER_1`"]
    #[inline(always)]
    pub fn is_pinper_1(&self) -> bool {
        *self == PINPER_A::PINPER_1
    }
}
#[doc = "Write proxy for field `PINPER`"]
pub struct PINPER_W<'a> {
    w: &'a mut W,
}
impl<'a> PINPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn pinper_0(self) -> &'a mut W {
        self.variant(PINPER_A::PINPER_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn pinper_1(self) -> &'a mut W {
        self.variant(PINPER_A::PINPER_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RESTART`"]
pub type RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESTART`"]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
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
#[doc = "Capture Timer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE_A {
    #[doc = "0: No effect."]
    CAPTURE_0 = 0,
    #[doc = "1: The current time is captured and can be read from the ATVR register."]
    CAPTURE_1 = 1,
}
impl From<CAPTURE_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTURE`"]
pub type CAPTURE_R = crate::R<bool, CAPTURE_A>;
impl CAPTURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_A {
        match self.bits {
            false => CAPTURE_A::CAPTURE_0,
            true => CAPTURE_A::CAPTURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE_0`"]
    #[inline(always)]
    pub fn is_capture_0(&self) -> bool {
        *self == CAPTURE_A::CAPTURE_0
    }
    #[doc = "Checks if the value of the field is `CAPTURE_1`"]
    #[inline(always)]
    pub fn is_capture_1(&self) -> bool {
        *self == CAPTURE_A::CAPTURE_1
    }
}
#[doc = "Write proxy for field `CAPTURE`"]
pub struct CAPTURE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn capture_0(self) -> &'a mut W {
        self.variant(CAPTURE_A::CAPTURE_0)
    }
    #[doc = "The current time is captured and can be read from the ATVR register."]
    #[inline(always)]
    pub fn capture_1(self) -> &'a mut W {
        self.variant(CAPTURE_A::CAPTURE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Enable Timer Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_A {
    #[doc = "0: The timer is active and all configuration fields in this register are relevant."]
    SLAVE_0 = 0,
    #[doc = "1: The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    SLAVE_1 = 1,
}
impl From<SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLAVE`"]
pub type SLAVE_R = crate::R<bool, SLAVE_A>;
impl SLAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_A {
        match self.bits {
            false => SLAVE_A::SLAVE_0,
            true => SLAVE_A::SLAVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_0`"]
    #[inline(always)]
    pub fn is_slave_0(&self) -> bool {
        *self == SLAVE_A::SLAVE_0
    }
    #[doc = "Checks if the value of the field is `SLAVE_1`"]
    #[inline(always)]
    pub fn is_slave_1(&self) -> bool {
        *self == SLAVE_A::SLAVE_1
    }
}
#[doc = "Write proxy for field `SLAVE`"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer is active and all configuration fields in this register are relevant."]
    #[inline(always)]
    pub fn slave_0(self) -> &'a mut W {
        self.variant(SLAVE_A::SLAVE_0)
    }
    #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    #[inline(always)]
    pub fn slave_1(self) -> &'a mut W {
        self.variant(SLAVE_A::SLAVE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable One-Shot Offset Event"]
    #[inline(always)]
    pub fn offen(&self) -> OFFEN_R {
        OFFEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset Timer On Offset Event"]
    #[inline(always)]
    pub fn offrst(&self) -> OFFRST_R {
        OFFRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Periodical Event"]
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables event signal output assertion on period event"]
    #[inline(always)]
    pub fn pinper(&self) -> PINPER_R {
        PINPER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset Timer"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Timer Value"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Timer Slave Mode"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable One-Shot Offset Event"]
    #[inline(always)]
    pub fn offen(&mut self) -> OFFEN_W {
        OFFEN_W { w: self }
    }
    #[doc = "Bit 3 - Reset Timer On Offset Event"]
    #[inline(always)]
    pub fn offrst(&mut self) -> OFFRST_W {
        OFFRST_W { w: self }
    }
    #[doc = "Bit 4 - Enable Periodical Event"]
    #[inline(always)]
    pub fn peren(&mut self) -> PEREN_W {
        PEREN_W { w: self }
    }
    #[doc = "Bit 7 - Enables event signal output assertion on period event"]
    #[inline(always)]
    pub fn pinper(&mut self) -> PINPER_W {
        PINPER_W { w: self }
    }
    #[doc = "Bit 9 - Reset Timer"]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bit 11 - Capture Timer Value"]
    #[inline(always)]
    pub fn capture(&mut self) -> CAPTURE_W {
        CAPTURE_W { w: self }
    }
    #[doc = "Bit 13 - Enable Timer Slave Mode"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
}
