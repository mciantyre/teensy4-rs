#[doc = "Reader of register KPSR"]
pub type R = crate::R<u16, super::KPSR>;
#[doc = "Writer for register KPSR"]
pub type W = crate::W<u16, super::KPSR>;
#[doc = "Register KPSR `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::KPSR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Keypad Key Depress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KPKD_A {
    #[doc = "0: No key presses detected"]
    KPKD_0 = 0,
    #[doc = "1: A key has been depressed"]
    KPKD_1 = 1,
}
impl From<KPKD_A> for bool {
    #[inline(always)]
    fn from(variant: KPKD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KPKD`"]
pub type KPKD_R = crate::R<bool, KPKD_A>;
impl KPKD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KPKD_A {
        match self.bits {
            false => KPKD_A::KPKD_0,
            true => KPKD_A::KPKD_1,
        }
    }
    #[doc = "Checks if the value of the field is `KPKD_0`"]
    #[inline(always)]
    pub fn is_kpkd_0(&self) -> bool {
        *self == KPKD_A::KPKD_0
    }
    #[doc = "Checks if the value of the field is `KPKD_1`"]
    #[inline(always)]
    pub fn is_kpkd_1(&self) -> bool {
        *self == KPKD_A::KPKD_1
    }
}
#[doc = "Write proxy for field `KPKD`"]
pub struct KPKD_W<'a> {
    w: &'a mut W,
}
impl<'a> KPKD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KPKD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No key presses detected"]
    #[inline(always)]
    pub fn kpkd_0(self) -> &'a mut W {
        self.variant(KPKD_A::KPKD_0)
    }
    #[doc = "A key has been depressed"]
    #[inline(always)]
    pub fn kpkd_1(self) -> &'a mut W {
        self.variant(KPKD_A::KPKD_1)
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
#[doc = "Keypad Key Release\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KPKR_A {
    #[doc = "0: No key release detected"]
    KPKR_0 = 0,
    #[doc = "1: All keys have been released"]
    KPKR_1 = 1,
}
impl From<KPKR_A> for bool {
    #[inline(always)]
    fn from(variant: KPKR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KPKR`"]
pub type KPKR_R = crate::R<bool, KPKR_A>;
impl KPKR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KPKR_A {
        match self.bits {
            false => KPKR_A::KPKR_0,
            true => KPKR_A::KPKR_1,
        }
    }
    #[doc = "Checks if the value of the field is `KPKR_0`"]
    #[inline(always)]
    pub fn is_kpkr_0(&self) -> bool {
        *self == KPKR_A::KPKR_0
    }
    #[doc = "Checks if the value of the field is `KPKR_1`"]
    #[inline(always)]
    pub fn is_kpkr_1(&self) -> bool {
        *self == KPKR_A::KPKR_1
    }
}
#[doc = "Write proxy for field `KPKR`"]
pub struct KPKR_W<'a> {
    w: &'a mut W,
}
impl<'a> KPKR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KPKR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No key release detected"]
    #[inline(always)]
    pub fn kpkr_0(self) -> &'a mut W {
        self.variant(KPKR_A::KPKR_0)
    }
    #[doc = "All keys have been released"]
    #[inline(always)]
    pub fn kpkr_1(self) -> &'a mut W {
        self.variant(KPKR_A::KPKR_1)
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
#[doc = "Key Depress Synchronizer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KDSC_A {
    #[doc = "0: No effect"]
    KDSC_0 = 0,
    #[doc = "1: Set bits that clear the keypad depress synchronizer chain"]
    KDSC_1 = 1,
}
impl From<KDSC_A> for bool {
    #[inline(always)]
    fn from(variant: KDSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KDSC`"]
pub type KDSC_R = crate::R<bool, KDSC_A>;
impl KDSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KDSC_A {
        match self.bits {
            false => KDSC_A::KDSC_0,
            true => KDSC_A::KDSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `KDSC_0`"]
    #[inline(always)]
    pub fn is_kdsc_0(&self) -> bool {
        *self == KDSC_A::KDSC_0
    }
    #[doc = "Checks if the value of the field is `KDSC_1`"]
    #[inline(always)]
    pub fn is_kdsc_1(&self) -> bool {
        *self == KDSC_A::KDSC_1
    }
}
#[doc = "Write proxy for field `KDSC`"]
pub struct KDSC_W<'a> {
    w: &'a mut W,
}
impl<'a> KDSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KDSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn kdsc_0(self) -> &'a mut W {
        self.variant(KDSC_A::KDSC_0)
    }
    #[doc = "Set bits that clear the keypad depress synchronizer chain"]
    #[inline(always)]
    pub fn kdsc_1(self) -> &'a mut W {
        self.variant(KDSC_A::KDSC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Key Release Synchronizer Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KRSS_A {
    #[doc = "0: No effect"]
    KRSS_0 = 0,
    #[doc = "1: Set bits which sets keypad release synchronizer chain"]
    KRSS_1 = 1,
}
impl From<KRSS_A> for bool {
    #[inline(always)]
    fn from(variant: KRSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KRSS`"]
pub type KRSS_R = crate::R<bool, KRSS_A>;
impl KRSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRSS_A {
        match self.bits {
            false => KRSS_A::KRSS_0,
            true => KRSS_A::KRSS_1,
        }
    }
    #[doc = "Checks if the value of the field is `KRSS_0`"]
    #[inline(always)]
    pub fn is_krss_0(&self) -> bool {
        *self == KRSS_A::KRSS_0
    }
    #[doc = "Checks if the value of the field is `KRSS_1`"]
    #[inline(always)]
    pub fn is_krss_1(&self) -> bool {
        *self == KRSS_A::KRSS_1
    }
}
#[doc = "Write proxy for field `KRSS`"]
pub struct KRSS_W<'a> {
    w: &'a mut W,
}
impl<'a> KRSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KRSS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn krss_0(self) -> &'a mut W {
        self.variant(KRSS_A::KRSS_0)
    }
    #[doc = "Set bits which sets keypad release synchronizer chain"]
    #[inline(always)]
    pub fn krss_1(self) -> &'a mut W {
        self.variant(KRSS_A::KRSS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Keypad Key Depress Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KDIE_A {
    #[doc = "0: No interrupt request is generated when KPKD is set."]
    KDIE_0 = 0,
    #[doc = "1: An interrupt request is generated when KPKD is set."]
    KDIE_1 = 1,
}
impl From<KDIE_A> for bool {
    #[inline(always)]
    fn from(variant: KDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KDIE`"]
pub type KDIE_R = crate::R<bool, KDIE_A>;
impl KDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KDIE_A {
        match self.bits {
            false => KDIE_A::KDIE_0,
            true => KDIE_A::KDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `KDIE_0`"]
    #[inline(always)]
    pub fn is_kdie_0(&self) -> bool {
        *self == KDIE_A::KDIE_0
    }
    #[doc = "Checks if the value of the field is `KDIE_1`"]
    #[inline(always)]
    pub fn is_kdie_1(&self) -> bool {
        *self == KDIE_A::KDIE_1
    }
}
#[doc = "Write proxy for field `KDIE`"]
pub struct KDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> KDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated when KPKD is set."]
    #[inline(always)]
    pub fn kdie_0(self) -> &'a mut W {
        self.variant(KDIE_A::KDIE_0)
    }
    #[doc = "An interrupt request is generated when KPKD is set."]
    #[inline(always)]
    pub fn kdie_1(self) -> &'a mut W {
        self.variant(KDIE_A::KDIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Keypad Release Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KRIE_A {
    #[doc = "0: No interrupt request is generated when KPKR is set."]
    KRIE_0 = 0,
    #[doc = "1: An interrupt request is generated when KPKR is set."]
    KRIE_1 = 1,
}
impl From<KRIE_A> for bool {
    #[inline(always)]
    fn from(variant: KRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KRIE`"]
pub type KRIE_R = crate::R<bool, KRIE_A>;
impl KRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRIE_A {
        match self.bits {
            false => KRIE_A::KRIE_0,
            true => KRIE_A::KRIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `KRIE_0`"]
    #[inline(always)]
    pub fn is_krie_0(&self) -> bool {
        *self == KRIE_A::KRIE_0
    }
    #[doc = "Checks if the value of the field is `KRIE_1`"]
    #[inline(always)]
    pub fn is_krie_1(&self) -> bool {
        *self == KRIE_A::KRIE_1
    }
}
#[doc = "Write proxy for field `KRIE`"]
pub struct KRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> KRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated when KPKR is set."]
    #[inline(always)]
    pub fn krie_0(self) -> &'a mut W {
        self.variant(KRIE_A::KRIE_0)
    }
    #[doc = "An interrupt request is generated when KPKR is set."]
    #[inline(always)]
    pub fn krie_1(self) -> &'a mut W {
        self.variant(KRIE_A::KRIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Keypad Key Depress"]
    #[inline(always)]
    pub fn kpkd(&self) -> KPKD_R {
        KPKD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Keypad Key Release"]
    #[inline(always)]
    pub fn kpkr(&self) -> KPKR_R {
        KPKR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Key Depress Synchronizer Clear"]
    #[inline(always)]
    pub fn kdsc(&self) -> KDSC_R {
        KDSC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Key Release Synchronizer Set"]
    #[inline(always)]
    pub fn krss(&self) -> KRSS_R {
        KRSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Keypad Key Depress Interrupt Enable"]
    #[inline(always)]
    pub fn kdie(&self) -> KDIE_R {
        KDIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Keypad Release Interrupt Enable"]
    #[inline(always)]
    pub fn krie(&self) -> KRIE_R {
        KRIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keypad Key Depress"]
    #[inline(always)]
    pub fn kpkd(&mut self) -> KPKD_W {
        KPKD_W { w: self }
    }
    #[doc = "Bit 1 - Keypad Key Release"]
    #[inline(always)]
    pub fn kpkr(&mut self) -> KPKR_W {
        KPKR_W { w: self }
    }
    #[doc = "Bit 2 - Key Depress Synchronizer Clear"]
    #[inline(always)]
    pub fn kdsc(&mut self) -> KDSC_W {
        KDSC_W { w: self }
    }
    #[doc = "Bit 3 - Key Release Synchronizer Set"]
    #[inline(always)]
    pub fn krss(&mut self) -> KRSS_W {
        KRSS_W { w: self }
    }
    #[doc = "Bit 8 - Keypad Key Depress Interrupt Enable"]
    #[inline(always)]
    pub fn kdie(&mut self) -> KDIE_W {
        KDIE_W { w: self }
    }
    #[doc = "Bit 9 - Keypad Release Interrupt Enable"]
    #[inline(always)]
    pub fn krie(&mut self) -> KRIE_W {
        KRIE_W { w: self }
    }
}
