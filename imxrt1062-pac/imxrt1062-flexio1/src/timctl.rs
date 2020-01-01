#[doc = "Reader of register TIMCTL[%s]"]
pub type R = crate::R<u32, super::TIMCTL>;
#[doc = "Writer for register TIMCTL[%s]"]
pub type W = crate::W<u32, super::TIMCTL>;
#[doc = "Register TIMCTL[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TIMCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMOD_A {
    #[doc = "0: Timer Disabled."]
    TIMOD_0 = 0,
    #[doc = "1: Dual 8-bit counters baud mode."]
    TIMOD_1 = 1,
    #[doc = "2: Dual 8-bit counters PWM high mode."]
    TIMOD_2 = 2,
    #[doc = "3: Single 16-bit counter mode."]
    TIMOD_3 = 3,
}
impl From<TIMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMOD`"]
pub type TIMOD_R = crate::R<u8, TIMOD_A>;
impl TIMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMOD_A {
        match self.bits {
            0 => TIMOD_A::TIMOD_0,
            1 => TIMOD_A::TIMOD_1,
            2 => TIMOD_A::TIMOD_2,
            3 => TIMOD_A::TIMOD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMOD_0`"]
    #[inline(always)]
    pub fn is_timod_0(&self) -> bool {
        *self == TIMOD_A::TIMOD_0
    }
    #[doc = "Checks if the value of the field is `TIMOD_1`"]
    #[inline(always)]
    pub fn is_timod_1(&self) -> bool {
        *self == TIMOD_A::TIMOD_1
    }
    #[doc = "Checks if the value of the field is `TIMOD_2`"]
    #[inline(always)]
    pub fn is_timod_2(&self) -> bool {
        *self == TIMOD_A::TIMOD_2
    }
    #[doc = "Checks if the value of the field is `TIMOD_3`"]
    #[inline(always)]
    pub fn is_timod_3(&self) -> bool {
        *self == TIMOD_A::TIMOD_3
    }
}
#[doc = "Write proxy for field `TIMOD`"]
pub struct TIMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer Disabled."]
    #[inline(always)]
    pub fn timod_0(self) -> &'a mut W {
        self.variant(TIMOD_A::TIMOD_0)
    }
    #[doc = "Dual 8-bit counters baud mode."]
    #[inline(always)]
    pub fn timod_1(self) -> &'a mut W {
        self.variant(TIMOD_A::TIMOD_1)
    }
    #[doc = "Dual 8-bit counters PWM high mode."]
    #[inline(always)]
    pub fn timod_2(self) -> &'a mut W {
        self.variant(TIMOD_A::TIMOD_2)
    }
    #[doc = "Single 16-bit counter mode."]
    #[inline(always)]
    pub fn timod_3(self) -> &'a mut W {
        self.variant(TIMOD_A::TIMOD_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Timer Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPOL_A {
    #[doc = "0: Pin is active high"]
    PINPOL_0 = 0,
    #[doc = "1: Pin is active low"]
    PINPOL_1 = 1,
}
impl From<PINPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PINPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINPOL`"]
pub type PINPOL_R = crate::R<bool, PINPOL_A>;
impl PINPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINPOL_A {
        match self.bits {
            false => PINPOL_A::PINPOL_0,
            true => PINPOL_A::PINPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PINPOL_0`"]
    #[inline(always)]
    pub fn is_pinpol_0(&self) -> bool {
        *self == PINPOL_A::PINPOL_0
    }
    #[doc = "Checks if the value of the field is `PINPOL_1`"]
    #[inline(always)]
    pub fn is_pinpol_1(&self) -> bool {
        *self == PINPOL_A::PINPOL_1
    }
}
#[doc = "Write proxy for field `PINPOL`"]
pub struct PINPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is active high"]
    #[inline(always)]
    pub fn pinpol_0(self) -> &'a mut W {
        self.variant(PINPOL_A::PINPOL_0)
    }
    #[doc = "Pin is active low"]
    #[inline(always)]
    pub fn pinpol_1(self) -> &'a mut W {
        self.variant(PINPOL_A::PINPOL_1)
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
#[doc = "Reader of field `PINSEL`"]
pub type PINSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PINSEL`"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Timer Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: Timer pin output disabled"]
    PINCFG_0 = 0,
    #[doc = "1: Timer pin open drain or bidirectional output enable"]
    PINCFG_1 = 1,
    #[doc = "2: Timer pin bidirectional output data"]
    PINCFG_2 = 2,
    #[doc = "3: Timer pin output"]
    PINCFG_3 = 3,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINCFG`"]
pub type PINCFG_R = crate::R<u8, PINCFG_A>;
impl PINCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::PINCFG_0,
            1 => PINCFG_A::PINCFG_1,
            2 => PINCFG_A::PINCFG_2,
            3 => PINCFG_A::PINCFG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PINCFG_0`"]
    #[inline(always)]
    pub fn is_pincfg_0(&self) -> bool {
        *self == PINCFG_A::PINCFG_0
    }
    #[doc = "Checks if the value of the field is `PINCFG_1`"]
    #[inline(always)]
    pub fn is_pincfg_1(&self) -> bool {
        *self == PINCFG_A::PINCFG_1
    }
    #[doc = "Checks if the value of the field is `PINCFG_2`"]
    #[inline(always)]
    pub fn is_pincfg_2(&self) -> bool {
        *self == PINCFG_A::PINCFG_2
    }
    #[doc = "Checks if the value of the field is `PINCFG_3`"]
    #[inline(always)]
    pub fn is_pincfg_3(&self) -> bool {
        *self == PINCFG_A::PINCFG_3
    }
}
#[doc = "Write proxy for field `PINCFG`"]
pub struct PINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer pin output disabled"]
    #[inline(always)]
    pub fn pincfg_0(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_0)
    }
    #[doc = "Timer pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn pincfg_1(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_1)
    }
    #[doc = "Timer pin bidirectional output data"]
    #[inline(always)]
    pub fn pincfg_2(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_2)
    }
    #[doc = "Timer pin output"]
    #[inline(always)]
    pub fn pincfg_3(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSRC_A {
    #[doc = "0: External trigger selected"]
    TRGSRC_0 = 0,
    #[doc = "1: Internal trigger selected"]
    TRGSRC_1 = 1,
}
impl From<TRGSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TRGSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRGSRC`"]
pub type TRGSRC_R = crate::R<bool, TRGSRC_A>;
impl TRGSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSRC_A {
        match self.bits {
            false => TRGSRC_A::TRGSRC_0,
            true => TRGSRC_A::TRGSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRGSRC_0`"]
    #[inline(always)]
    pub fn is_trgsrc_0(&self) -> bool {
        *self == TRGSRC_A::TRGSRC_0
    }
    #[doc = "Checks if the value of the field is `TRGSRC_1`"]
    #[inline(always)]
    pub fn is_trgsrc_1(&self) -> bool {
        *self == TRGSRC_A::TRGSRC_1
    }
}
#[doc = "Write proxy for field `TRGSRC`"]
pub struct TRGSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External trigger selected"]
    #[inline(always)]
    pub fn trgsrc_0(self) -> &'a mut W {
        self.variant(TRGSRC_A::TRGSRC_0)
    }
    #[doc = "Internal trigger selected"]
    #[inline(always)]
    pub fn trgsrc_1(self) -> &'a mut W {
        self.variant(TRGSRC_A::TRGSRC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Trigger Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPOL_A {
    #[doc = "0: Trigger active high"]
    TRGPOL_0 = 0,
    #[doc = "1: Trigger active low"]
    TRGPOL_1 = 1,
}
impl From<TRGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRGPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRGPOL`"]
pub type TRGPOL_R = crate::R<bool, TRGPOL_A>;
impl TRGPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGPOL_A {
        match self.bits {
            false => TRGPOL_A::TRGPOL_0,
            true => TRGPOL_A::TRGPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRGPOL_0`"]
    #[inline(always)]
    pub fn is_trgpol_0(&self) -> bool {
        *self == TRGPOL_A::TRGPOL_0
    }
    #[doc = "Checks if the value of the field is `TRGPOL_1`"]
    #[inline(always)]
    pub fn is_trgpol_1(&self) -> bool {
        *self == TRGPOL_A::TRGPOL_1
    }
}
#[doc = "Write proxy for field `TRGPOL`"]
pub struct TRGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger active high"]
    #[inline(always)]
    pub fn trgpol_0(self) -> &'a mut W {
        self.variant(TRGPOL_A::TRGPOL_0)
    }
    #[doc = "Trigger active low"]
    #[inline(always)]
    pub fn trgpol_1(self) -> &'a mut W {
        self.variant(TRGPOL_A::TRGPOL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TRGSEL`"]
pub type TRGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRGSEL`"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn timod(&self) -> TIMOD_R {
        TIMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&self) -> PINPOL_R {
        PINPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Timer Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&self) -> TRGPOL_R {
        TRGPOL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn timod(&mut self) -> TIMOD_W {
        TIMOD_W { w: self }
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&mut self) -> PINPOL_W {
        PINPOL_W { w: self }
    }
    #[doc = "Bits 8:11 - Timer Pin Select"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PINCFG_W {
        PINCFG_W { w: self }
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W {
        TRGSRC_W { w: self }
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&mut self) -> TRGPOL_W {
        TRGPOL_W { w: self }
    }
    #[doc = "Bits 24:28 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
}
