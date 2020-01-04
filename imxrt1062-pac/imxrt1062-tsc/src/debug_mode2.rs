#[doc = "Reader of register DEBUG_MODE2"]
pub type R = crate::R<u32, super::DEBUG_MODE2>;
#[doc = "Writer for register DEBUG_MODE2"]
pub type W = crate::W<u32, super::DEBUG_MODE2>;
#[doc = "Register DEBUG_MODE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUG_MODE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "XPUL Wire Pull Down Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XPUL_PULL_DOWN_A {
    #[doc = "0: Close the switch"]
    XPUL_PULL_DOWN_0 = 0,
    #[doc = "1: Open up the switch"]
    XPUL_PULL_DOWN_1 = 1,
}
impl From<XPUL_PULL_DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: XPUL_PULL_DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XPUL_PULL_DOWN`"]
pub type XPUL_PULL_DOWN_R = crate::R<bool, XPUL_PULL_DOWN_A>;
impl XPUL_PULL_DOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XPUL_PULL_DOWN_A {
        match self.bits {
            false => XPUL_PULL_DOWN_A::XPUL_PULL_DOWN_0,
            true => XPUL_PULL_DOWN_A::XPUL_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `XPUL_PULL_DOWN_0`"]
    #[inline(always)]
    pub fn is_xpul_pull_down_0(&self) -> bool {
        *self == XPUL_PULL_DOWN_A::XPUL_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `XPUL_PULL_DOWN_1`"]
    #[inline(always)]
    pub fn is_xpul_pull_down_1(&self) -> bool {
        *self == XPUL_PULL_DOWN_A::XPUL_PULL_DOWN_1
    }
}
#[doc = "Write proxy for field `XPUL_PULL_DOWN`"]
pub struct XPUL_PULL_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> XPUL_PULL_DOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XPUL_PULL_DOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn xpul_pull_down_0(self) -> &'a mut W {
        self.variant(XPUL_PULL_DOWN_A::XPUL_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn xpul_pull_down_1(self) -> &'a mut W {
        self.variant(XPUL_PULL_DOWN_A::XPUL_PULL_DOWN_1)
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
#[doc = "XPUL Wire Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XPUL_PULL_UP_A {
    #[doc = "0: Close the switch"]
    XPUL_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    XPUL_PULL_UP_1 = 1,
}
impl From<XPUL_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: XPUL_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XPUL_PULL_UP`"]
pub type XPUL_PULL_UP_R = crate::R<bool, XPUL_PULL_UP_A>;
impl XPUL_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XPUL_PULL_UP_A {
        match self.bits {
            false => XPUL_PULL_UP_A::XPUL_PULL_UP_0,
            true => XPUL_PULL_UP_A::XPUL_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XPUL_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_xpul_pull_up_0(&self) -> bool {
        *self == XPUL_PULL_UP_A::XPUL_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `XPUL_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_xpul_pull_up_1(&self) -> bool {
        *self == XPUL_PULL_UP_A::XPUL_PULL_UP_1
    }
}
#[doc = "Write proxy for field `XPUL_PULL_UP`"]
pub struct XPUL_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> XPUL_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XPUL_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn xpul_pull_up_0(self) -> &'a mut W {
        self.variant(XPUL_PULL_UP_A::XPUL_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn xpul_pull_up_1(self) -> &'a mut W {
        self.variant(XPUL_PULL_UP_A::XPUL_PULL_UP_1)
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
#[doc = "XPUL Wire 200K Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XPUL_200K_PULL_UP_A {
    #[doc = "0: Close the switch"]
    XPUL_200K_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    XPUL_200K_PULL_UP_1 = 1,
}
impl From<XPUL_200K_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: XPUL_200K_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XPUL_200K_PULL_UP`"]
pub type XPUL_200K_PULL_UP_R = crate::R<bool, XPUL_200K_PULL_UP_A>;
impl XPUL_200K_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XPUL_200K_PULL_UP_A {
        match self.bits {
            false => XPUL_200K_PULL_UP_A::XPUL_200K_PULL_UP_0,
            true => XPUL_200K_PULL_UP_A::XPUL_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XPUL_200K_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_xpul_200k_pull_up_0(&self) -> bool {
        *self == XPUL_200K_PULL_UP_A::XPUL_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `XPUL_200K_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_xpul_200k_pull_up_1(&self) -> bool {
        *self == XPUL_200K_PULL_UP_A::XPUL_200K_PULL_UP_1
    }
}
#[doc = "Write proxy for field `XPUL_200K_PULL_UP`"]
pub struct XPUL_200K_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> XPUL_200K_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XPUL_200K_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn xpul_200k_pull_up_0(self) -> &'a mut W {
        self.variant(XPUL_200K_PULL_UP_A::XPUL_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn xpul_200k_pull_up_1(self) -> &'a mut W {
        self.variant(XPUL_200K_PULL_UP_A::XPUL_200K_PULL_UP_1)
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
#[doc = "XNUR Wire Pull Down Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XNUR_PULL_DOWN_A {
    #[doc = "0: Close the switch"]
    XNUR_PULL_DOWN_0 = 0,
    #[doc = "1: Open up the switch"]
    XNUR_PULL_DOWN_1 = 1,
}
impl From<XNUR_PULL_DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: XNUR_PULL_DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XNUR_PULL_DOWN`"]
pub type XNUR_PULL_DOWN_R = crate::R<bool, XNUR_PULL_DOWN_A>;
impl XNUR_PULL_DOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XNUR_PULL_DOWN_A {
        match self.bits {
            false => XNUR_PULL_DOWN_A::XNUR_PULL_DOWN_0,
            true => XNUR_PULL_DOWN_A::XNUR_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNUR_PULL_DOWN_0`"]
    #[inline(always)]
    pub fn is_xnur_pull_down_0(&self) -> bool {
        *self == XNUR_PULL_DOWN_A::XNUR_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `XNUR_PULL_DOWN_1`"]
    #[inline(always)]
    pub fn is_xnur_pull_down_1(&self) -> bool {
        *self == XNUR_PULL_DOWN_A::XNUR_PULL_DOWN_1
    }
}
#[doc = "Write proxy for field `XNUR_PULL_DOWN`"]
pub struct XNUR_PULL_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> XNUR_PULL_DOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XNUR_PULL_DOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn xnur_pull_down_0(self) -> &'a mut W {
        self.variant(XNUR_PULL_DOWN_A::XNUR_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn xnur_pull_down_1(self) -> &'a mut W {
        self.variant(XNUR_PULL_DOWN_A::XNUR_PULL_DOWN_1)
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
#[doc = "XNUR Wire Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XNUR_PULL_UP_A {
    #[doc = "0: Close the switch"]
    XNUR_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    XNUR_PULL_UP_1 = 1,
}
impl From<XNUR_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: XNUR_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XNUR_PULL_UP`"]
pub type XNUR_PULL_UP_R = crate::R<bool, XNUR_PULL_UP_A>;
impl XNUR_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XNUR_PULL_UP_A {
        match self.bits {
            false => XNUR_PULL_UP_A::XNUR_PULL_UP_0,
            true => XNUR_PULL_UP_A::XNUR_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNUR_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_xnur_pull_up_0(&self) -> bool {
        *self == XNUR_PULL_UP_A::XNUR_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `XNUR_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_xnur_pull_up_1(&self) -> bool {
        *self == XNUR_PULL_UP_A::XNUR_PULL_UP_1
    }
}
#[doc = "Write proxy for field `XNUR_PULL_UP`"]
pub struct XNUR_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> XNUR_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XNUR_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn xnur_pull_up_0(self) -> &'a mut W {
        self.variant(XNUR_PULL_UP_A::XNUR_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn xnur_pull_up_1(self) -> &'a mut W {
        self.variant(XNUR_PULL_UP_A::XNUR_PULL_UP_1)
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
#[doc = "XNUR Wire 200K Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XNUR_200K_PULL_UP_A {
    #[doc = "0: Close the switch"]
    XNUR_200K_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    XNUR_200K_PULL_UP_1 = 1,
}
impl From<XNUR_200K_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: XNUR_200K_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XNUR_200K_PULL_UP`"]
pub type XNUR_200K_PULL_UP_R = crate::R<bool, XNUR_200K_PULL_UP_A>;
impl XNUR_200K_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XNUR_200K_PULL_UP_A {
        match self.bits {
            false => XNUR_200K_PULL_UP_A::XNUR_200K_PULL_UP_0,
            true => XNUR_200K_PULL_UP_A::XNUR_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNUR_200K_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_xnur_200k_pull_up_0(&self) -> bool {
        *self == XNUR_200K_PULL_UP_A::XNUR_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `XNUR_200K_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_xnur_200k_pull_up_1(&self) -> bool {
        *self == XNUR_200K_PULL_UP_A::XNUR_200K_PULL_UP_1
    }
}
#[doc = "Write proxy for field `XNUR_200K_PULL_UP`"]
pub struct XNUR_200K_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> XNUR_200K_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XNUR_200K_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn xnur_200k_pull_up_0(self) -> &'a mut W {
        self.variant(XNUR_200K_PULL_UP_A::XNUR_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn xnur_200k_pull_up_1(self) -> &'a mut W {
        self.variant(XNUR_200K_PULL_UP_A::XNUR_200K_PULL_UP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "YPLL Wire Pull Down Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YPLL_PULL_DOWN_A {
    #[doc = "0: Close the switch"]
    YPLL_PULL_DOWN_0 = 0,
    #[doc = "1: Open up the switch"]
    YPLL_PULL_DOWN_1 = 1,
}
impl From<YPLL_PULL_DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: YPLL_PULL_DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `YPLL_PULL_DOWN`"]
pub type YPLL_PULL_DOWN_R = crate::R<bool, YPLL_PULL_DOWN_A>;
impl YPLL_PULL_DOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YPLL_PULL_DOWN_A {
        match self.bits {
            false => YPLL_PULL_DOWN_A::YPLL_PULL_DOWN_0,
            true => YPLL_PULL_DOWN_A::YPLL_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `YPLL_PULL_DOWN_0`"]
    #[inline(always)]
    pub fn is_ypll_pull_down_0(&self) -> bool {
        *self == YPLL_PULL_DOWN_A::YPLL_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `YPLL_PULL_DOWN_1`"]
    #[inline(always)]
    pub fn is_ypll_pull_down_1(&self) -> bool {
        *self == YPLL_PULL_DOWN_A::YPLL_PULL_DOWN_1
    }
}
#[doc = "Write proxy for field `YPLL_PULL_DOWN`"]
pub struct YPLL_PULL_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> YPLL_PULL_DOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YPLL_PULL_DOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn ypll_pull_down_0(self) -> &'a mut W {
        self.variant(YPLL_PULL_DOWN_A::YPLL_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn ypll_pull_down_1(self) -> &'a mut W {
        self.variant(YPLL_PULL_DOWN_A::YPLL_PULL_DOWN_1)
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
#[doc = "YPLL Wire Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YPLL_PULL_UP_A {
    #[doc = "0: Close the switch"]
    YPLL_PULL_UP_0 = 0,
    #[doc = "1: Open the switch"]
    YPLL_PULL_UP_1 = 1,
}
impl From<YPLL_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: YPLL_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `YPLL_PULL_UP`"]
pub type YPLL_PULL_UP_R = crate::R<bool, YPLL_PULL_UP_A>;
impl YPLL_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YPLL_PULL_UP_A {
        match self.bits {
            false => YPLL_PULL_UP_A::YPLL_PULL_UP_0,
            true => YPLL_PULL_UP_A::YPLL_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `YPLL_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_ypll_pull_up_0(&self) -> bool {
        *self == YPLL_PULL_UP_A::YPLL_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `YPLL_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_ypll_pull_up_1(&self) -> bool {
        *self == YPLL_PULL_UP_A::YPLL_PULL_UP_1
    }
}
#[doc = "Write proxy for field `YPLL_PULL_UP`"]
pub struct YPLL_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> YPLL_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YPLL_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn ypll_pull_up_0(self) -> &'a mut W {
        self.variant(YPLL_PULL_UP_A::YPLL_PULL_UP_0)
    }
    #[doc = "Open the switch"]
    #[inline(always)]
    pub fn ypll_pull_up_1(self) -> &'a mut W {
        self.variant(YPLL_PULL_UP_A::YPLL_PULL_UP_1)
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
#[doc = "YPLL Wire 200K Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YPLL_200K_PULL_UP_A {
    #[doc = "0: Close the switch"]
    YPLL_200K_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    YPLL_200K_PULL_UP_1 = 1,
}
impl From<YPLL_200K_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: YPLL_200K_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `YPLL_200K_PULL_UP`"]
pub type YPLL_200K_PULL_UP_R = crate::R<bool, YPLL_200K_PULL_UP_A>;
impl YPLL_200K_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YPLL_200K_PULL_UP_A {
        match self.bits {
            false => YPLL_200K_PULL_UP_A::YPLL_200K_PULL_UP_0,
            true => YPLL_200K_PULL_UP_A::YPLL_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `YPLL_200K_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_ypll_200k_pull_up_0(&self) -> bool {
        *self == YPLL_200K_PULL_UP_A::YPLL_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `YPLL_200K_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_ypll_200k_pull_up_1(&self) -> bool {
        *self == YPLL_200K_PULL_UP_A::YPLL_200K_PULL_UP_1
    }
}
#[doc = "Write proxy for field `YPLL_200K_PULL_UP`"]
pub struct YPLL_200K_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> YPLL_200K_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YPLL_200K_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn ypll_200k_pull_up_0(self) -> &'a mut W {
        self.variant(YPLL_200K_PULL_UP_A::YPLL_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn ypll_200k_pull_up_1(self) -> &'a mut W {
        self.variant(YPLL_200K_PULL_UP_A::YPLL_200K_PULL_UP_1)
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
#[doc = "YNLR Wire Pull Down Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YNLR_PULL_DOWN_A {
    #[doc = "0: Close the switch"]
    YNLR_PULL_DOWN_0 = 0,
    #[doc = "1: Open up the switch"]
    YNLR_PULL_DOWN_1 = 1,
}
impl From<YNLR_PULL_DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: YNLR_PULL_DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `YNLR_PULL_DOWN`"]
pub type YNLR_PULL_DOWN_R = crate::R<bool, YNLR_PULL_DOWN_A>;
impl YNLR_PULL_DOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YNLR_PULL_DOWN_A {
        match self.bits {
            false => YNLR_PULL_DOWN_A::YNLR_PULL_DOWN_0,
            true => YNLR_PULL_DOWN_A::YNLR_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `YNLR_PULL_DOWN_0`"]
    #[inline(always)]
    pub fn is_ynlr_pull_down_0(&self) -> bool {
        *self == YNLR_PULL_DOWN_A::YNLR_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `YNLR_PULL_DOWN_1`"]
    #[inline(always)]
    pub fn is_ynlr_pull_down_1(&self) -> bool {
        *self == YNLR_PULL_DOWN_A::YNLR_PULL_DOWN_1
    }
}
#[doc = "Write proxy for field `YNLR_PULL_DOWN`"]
pub struct YNLR_PULL_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> YNLR_PULL_DOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YNLR_PULL_DOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn ynlr_pull_down_0(self) -> &'a mut W {
        self.variant(YNLR_PULL_DOWN_A::YNLR_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn ynlr_pull_down_1(self) -> &'a mut W {
        self.variant(YNLR_PULL_DOWN_A::YNLR_PULL_DOWN_1)
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
#[doc = "YNLR Wire Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YNLR_PULL_UP_A {
    #[doc = "0: Close the switch"]
    YNLR_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    YNLR_PULL_UP_1 = 1,
}
impl From<YNLR_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: YNLR_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `YNLR_PULL_UP`"]
pub type YNLR_PULL_UP_R = crate::R<bool, YNLR_PULL_UP_A>;
impl YNLR_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YNLR_PULL_UP_A {
        match self.bits {
            false => YNLR_PULL_UP_A::YNLR_PULL_UP_0,
            true => YNLR_PULL_UP_A::YNLR_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `YNLR_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_ynlr_pull_up_0(&self) -> bool {
        *self == YNLR_PULL_UP_A::YNLR_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `YNLR_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_ynlr_pull_up_1(&self) -> bool {
        *self == YNLR_PULL_UP_A::YNLR_PULL_UP_1
    }
}
#[doc = "Write proxy for field `YNLR_PULL_UP`"]
pub struct YNLR_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> YNLR_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YNLR_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn ynlr_pull_up_0(self) -> &'a mut W {
        self.variant(YNLR_PULL_UP_A::YNLR_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn ynlr_pull_up_1(self) -> &'a mut W {
        self.variant(YNLR_PULL_UP_A::YNLR_PULL_UP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "YNLR Wire 200K Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YNLR_200K_PULL_UP_A {
    #[doc = "0: Close the switch"]
    YNLR_200K_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    YNLR_200K_PULL_UP_1 = 1,
}
impl From<YNLR_200K_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: YNLR_200K_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `YNLR_200K_PULL_UP`"]
pub type YNLR_200K_PULL_UP_R = crate::R<bool, YNLR_200K_PULL_UP_A>;
impl YNLR_200K_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YNLR_200K_PULL_UP_A {
        match self.bits {
            false => YNLR_200K_PULL_UP_A::YNLR_200K_PULL_UP_0,
            true => YNLR_200K_PULL_UP_A::YNLR_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `YNLR_200K_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_ynlr_200k_pull_up_0(&self) -> bool {
        *self == YNLR_200K_PULL_UP_A::YNLR_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `YNLR_200K_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_ynlr_200k_pull_up_1(&self) -> bool {
        *self == YNLR_200K_PULL_UP_A::YNLR_200K_PULL_UP_1
    }
}
#[doc = "Write proxy for field `YNLR_200K_PULL_UP`"]
pub struct YNLR_200K_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> YNLR_200K_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YNLR_200K_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn ynlr_200k_pull_up_0(self) -> &'a mut W {
        self.variant(YNLR_200K_PULL_UP_A::YNLR_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn ynlr_200k_pull_up_1(self) -> &'a mut W {
        self.variant(YNLR_200K_PULL_UP_A::YNLR_200K_PULL_UP_1)
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
#[doc = "Wiper Wire Pull Down Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPER_PULL_DOWN_A {
    #[doc = "0: Close the switch"]
    WIPER_PULL_DOWN_0 = 0,
    #[doc = "1: Open up the switch"]
    WIPER_PULL_DOWN_1 = 1,
}
impl From<WIPER_PULL_DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: WIPER_PULL_DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WIPER_PULL_DOWN`"]
pub type WIPER_PULL_DOWN_R = crate::R<bool, WIPER_PULL_DOWN_A>;
impl WIPER_PULL_DOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIPER_PULL_DOWN_A {
        match self.bits {
            false => WIPER_PULL_DOWN_A::WIPER_PULL_DOWN_0,
            true => WIPER_PULL_DOWN_A::WIPER_PULL_DOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIPER_PULL_DOWN_0`"]
    #[inline(always)]
    pub fn is_wiper_pull_down_0(&self) -> bool {
        *self == WIPER_PULL_DOWN_A::WIPER_PULL_DOWN_0
    }
    #[doc = "Checks if the value of the field is `WIPER_PULL_DOWN_1`"]
    #[inline(always)]
    pub fn is_wiper_pull_down_1(&self) -> bool {
        *self == WIPER_PULL_DOWN_A::WIPER_PULL_DOWN_1
    }
}
#[doc = "Write proxy for field `WIPER_PULL_DOWN`"]
pub struct WIPER_PULL_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIPER_PULL_DOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIPER_PULL_DOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn wiper_pull_down_0(self) -> &'a mut W {
        self.variant(WIPER_PULL_DOWN_A::WIPER_PULL_DOWN_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn wiper_pull_down_1(self) -> &'a mut W {
        self.variant(WIPER_PULL_DOWN_A::WIPER_PULL_DOWN_1)
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
#[doc = "Wiper Wire Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPER_PULL_UP_A {
    #[doc = "0: Close the switch"]
    WIPER_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    WIPER_PULL_UP_1 = 1,
}
impl From<WIPER_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: WIPER_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WIPER_PULL_UP`"]
pub type WIPER_PULL_UP_R = crate::R<bool, WIPER_PULL_UP_A>;
impl WIPER_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIPER_PULL_UP_A {
        match self.bits {
            false => WIPER_PULL_UP_A::WIPER_PULL_UP_0,
            true => WIPER_PULL_UP_A::WIPER_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIPER_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_wiper_pull_up_0(&self) -> bool {
        *self == WIPER_PULL_UP_A::WIPER_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `WIPER_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_wiper_pull_up_1(&self) -> bool {
        *self == WIPER_PULL_UP_A::WIPER_PULL_UP_1
    }
}
#[doc = "Write proxy for field `WIPER_PULL_UP`"]
pub struct WIPER_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> WIPER_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIPER_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn wiper_pull_up_0(self) -> &'a mut W {
        self.variant(WIPER_PULL_UP_A::WIPER_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn wiper_pull_up_1(self) -> &'a mut W {
        self.variant(WIPER_PULL_UP_A::WIPER_PULL_UP_1)
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
#[doc = "Wiper Wire 200K Pull Up Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPER_200K_PULL_UP_A {
    #[doc = "0: Close the switch"]
    WIPER_200K_PULL_UP_0 = 0,
    #[doc = "1: Open up the switch"]
    WIPER_200K_PULL_UP_1 = 1,
}
impl From<WIPER_200K_PULL_UP_A> for bool {
    #[inline(always)]
    fn from(variant: WIPER_200K_PULL_UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WIPER_200K_PULL_UP`"]
pub type WIPER_200K_PULL_UP_R = crate::R<bool, WIPER_200K_PULL_UP_A>;
impl WIPER_200K_PULL_UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIPER_200K_PULL_UP_A {
        match self.bits {
            false => WIPER_200K_PULL_UP_A::WIPER_200K_PULL_UP_0,
            true => WIPER_200K_PULL_UP_A::WIPER_200K_PULL_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIPER_200K_PULL_UP_0`"]
    #[inline(always)]
    pub fn is_wiper_200k_pull_up_0(&self) -> bool {
        *self == WIPER_200K_PULL_UP_A::WIPER_200K_PULL_UP_0
    }
    #[doc = "Checks if the value of the field is `WIPER_200K_PULL_UP_1`"]
    #[inline(always)]
    pub fn is_wiper_200k_pull_up_1(&self) -> bool {
        *self == WIPER_200K_PULL_UP_A::WIPER_200K_PULL_UP_1
    }
}
#[doc = "Write proxy for field `WIPER_200K_PULL_UP`"]
pub struct WIPER_200K_PULL_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> WIPER_200K_PULL_UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIPER_200K_PULL_UP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Close the switch"]
    #[inline(always)]
    pub fn wiper_200k_pull_up_0(self) -> &'a mut W {
        self.variant(WIPER_200K_PULL_UP_A::WIPER_200K_PULL_UP_0)
    }
    #[doc = "Open up the switch"]
    #[inline(always)]
    pub fn wiper_200k_pull_up_1(self) -> &'a mut W {
        self.variant(WIPER_200K_PULL_UP_A::WIPER_200K_PULL_UP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Detect Four Wire\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_FOUR_WIRE_A {
    #[doc = "0: No detect signal"]
    DETECT_FOUR_WIRE_0 = 0,
    #[doc = "1: Yes, there is a detect on the touch screen."]
    DETECT_FOUR_WIRE_1 = 1,
}
impl From<DETECT_FOUR_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: DETECT_FOUR_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETECT_FOUR_WIRE`"]
pub type DETECT_FOUR_WIRE_R = crate::R<bool, DETECT_FOUR_WIRE_A>;
impl DETECT_FOUR_WIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECT_FOUR_WIRE_A {
        match self.bits {
            false => DETECT_FOUR_WIRE_A::DETECT_FOUR_WIRE_0,
            true => DETECT_FOUR_WIRE_A::DETECT_FOUR_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_FOUR_WIRE_0`"]
    #[inline(always)]
    pub fn is_detect_four_wire_0(&self) -> bool {
        *self == DETECT_FOUR_WIRE_A::DETECT_FOUR_WIRE_0
    }
    #[doc = "Checks if the value of the field is `DETECT_FOUR_WIRE_1`"]
    #[inline(always)]
    pub fn is_detect_four_wire_1(&self) -> bool {
        *self == DETECT_FOUR_WIRE_A::DETECT_FOUR_WIRE_1
    }
}
#[doc = "Detect Five Wire\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_FIVE_WIRE_A {
    #[doc = "0: No detect signal"]
    DETECT_FIVE_WIRE_0 = 0,
    #[doc = "1: Yes, there is a detect on the touch screen."]
    DETECT_FIVE_WIRE_1 = 1,
}
impl From<DETECT_FIVE_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: DETECT_FIVE_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETECT_FIVE_WIRE`"]
pub type DETECT_FIVE_WIRE_R = crate::R<bool, DETECT_FIVE_WIRE_A>;
impl DETECT_FIVE_WIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECT_FIVE_WIRE_A {
        match self.bits {
            false => DETECT_FIVE_WIRE_A::DETECT_FIVE_WIRE_0,
            true => DETECT_FIVE_WIRE_A::DETECT_FIVE_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_FIVE_WIRE_0`"]
    #[inline(always)]
    pub fn is_detect_five_wire_0(&self) -> bool {
        *self == DETECT_FIVE_WIRE_A::DETECT_FIVE_WIRE_0
    }
    #[doc = "Checks if the value of the field is `DETECT_FIVE_WIRE_1`"]
    #[inline(always)]
    pub fn is_detect_five_wire_1(&self) -> bool {
        *self == DETECT_FIVE_WIRE_A::DETECT_FIVE_WIRE_1
    }
}
#[doc = "State Machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_MACHINE_A {
    #[doc = "0: Idle"]
    STATE_MACHINE_0 = 0,
    #[doc = "1: Pre-charge"]
    STATE_MACHINE_1 = 1,
    #[doc = "2: Detect"]
    STATE_MACHINE_2 = 2,
    #[doc = "3: X-measure"]
    STATE_MACHINE_3 = 3,
    #[doc = "4: Y-measure"]
    STATE_MACHINE_4 = 4,
    #[doc = "5: Pre-charge"]
    STATE_MACHINE_5 = 5,
    #[doc = "6: Detect"]
    STATE_MACHINE_6 = 6,
}
impl From<STATE_MACHINE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_MACHINE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE_MACHINE`"]
pub type STATE_MACHINE_R = crate::R<u8, STATE_MACHINE_A>;
impl STATE_MACHINE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_MACHINE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_MACHINE_A::STATE_MACHINE_0),
            1 => Val(STATE_MACHINE_A::STATE_MACHINE_1),
            2 => Val(STATE_MACHINE_A::STATE_MACHINE_2),
            3 => Val(STATE_MACHINE_A::STATE_MACHINE_3),
            4 => Val(STATE_MACHINE_A::STATE_MACHINE_4),
            5 => Val(STATE_MACHINE_A::STATE_MACHINE_5),
            6 => Val(STATE_MACHINE_A::STATE_MACHINE_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_0`"]
    #[inline(always)]
    pub fn is_state_machine_0(&self) -> bool {
        *self == STATE_MACHINE_A::STATE_MACHINE_0
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_1`"]
    #[inline(always)]
    pub fn is_state_machine_1(&self) -> bool {
        *self == STATE_MACHINE_A::STATE_MACHINE_1
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_2`"]
    #[inline(always)]
    pub fn is_state_machine_2(&self) -> bool {
        *self == STATE_MACHINE_A::STATE_MACHINE_2
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_3`"]
    #[inline(always)]
    pub fn is_state_machine_3(&self) -> bool {
        *self == STATE_MACHINE_A::STATE_MACHINE_3
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_4`"]
    #[inline(always)]
    pub fn is_state_machine_4(&self) -> bool {
        *self == STATE_MACHINE_A::STATE_MACHINE_4
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_5`"]
    #[inline(always)]
    pub fn is_state_machine_5(&self) -> bool {
        *self == STATE_MACHINE_A::STATE_MACHINE_5
    }
    #[doc = "Checks if the value of the field is `STATE_MACHINE_6`"]
    #[inline(always)]
    pub fn is_state_machine_6(&self) -> bool {
        *self == STATE_MACHINE_A::STATE_MACHINE_6
    }
}
#[doc = "Intermediate State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERMEDIATE_A {
    #[doc = "0: Not in intermedia"]
    INTERMEDIATE_0 = 0,
    #[doc = "1: Intermedia"]
    INTERMEDIATE_1 = 1,
}
impl From<INTERMEDIATE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERMEDIATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTERMEDIATE`"]
pub type INTERMEDIATE_R = crate::R<bool, INTERMEDIATE_A>;
impl INTERMEDIATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERMEDIATE_A {
        match self.bits {
            false => INTERMEDIATE_A::INTERMEDIATE_0,
            true => INTERMEDIATE_A::INTERMEDIATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE_0`"]
    #[inline(always)]
    pub fn is_intermediate_0(&self) -> bool {
        *self == INTERMEDIATE_A::INTERMEDIATE_0
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE_1`"]
    #[inline(always)]
    pub fn is_intermediate_1(&self) -> bool {
        *self == INTERMEDIATE_A::INTERMEDIATE_1
    }
}
#[doc = "Detect Enable Four Wire\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_ENABLE_FOUR_WIRE_A {
    #[doc = "0: Do not read four wire detect value, read default value from analogue"]
    DETECT_ENABLE_FOUR_WIRE_0 = 0,
    #[doc = "1: Read four wire detect status from analogue"]
    DETECT_ENABLE_FOUR_WIRE_1 = 1,
}
impl From<DETECT_ENABLE_FOUR_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: DETECT_ENABLE_FOUR_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETECT_ENABLE_FOUR_WIRE`"]
pub type DETECT_ENABLE_FOUR_WIRE_R = crate::R<bool, DETECT_ENABLE_FOUR_WIRE_A>;
impl DETECT_ENABLE_FOUR_WIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECT_ENABLE_FOUR_WIRE_A {
        match self.bits {
            false => DETECT_ENABLE_FOUR_WIRE_A::DETECT_ENABLE_FOUR_WIRE_0,
            true => DETECT_ENABLE_FOUR_WIRE_A::DETECT_ENABLE_FOUR_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_ENABLE_FOUR_WIRE_0`"]
    #[inline(always)]
    pub fn is_detect_enable_four_wire_0(&self) -> bool {
        *self == DETECT_ENABLE_FOUR_WIRE_A::DETECT_ENABLE_FOUR_WIRE_0
    }
    #[doc = "Checks if the value of the field is `DETECT_ENABLE_FOUR_WIRE_1`"]
    #[inline(always)]
    pub fn is_detect_enable_four_wire_1(&self) -> bool {
        *self == DETECT_ENABLE_FOUR_WIRE_A::DETECT_ENABLE_FOUR_WIRE_1
    }
}
#[doc = "Write proxy for field `DETECT_ENABLE_FOUR_WIRE`"]
pub struct DETECT_ENABLE_FOUR_WIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECT_ENABLE_FOUR_WIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DETECT_ENABLE_FOUR_WIRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not read four wire detect value, read default value from analogue"]
    #[inline(always)]
    pub fn detect_enable_four_wire_0(self) -> &'a mut W {
        self.variant(DETECT_ENABLE_FOUR_WIRE_A::DETECT_ENABLE_FOUR_WIRE_0)
    }
    #[doc = "Read four wire detect status from analogue"]
    #[inline(always)]
    pub fn detect_enable_four_wire_1(self) -> &'a mut W {
        self.variant(DETECT_ENABLE_FOUR_WIRE_A::DETECT_ENABLE_FOUR_WIRE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Detect Enable Five Wire\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECT_ENABLE_FIVE_WIRE_A {
    #[doc = "0: Do not read five wire detect value, read default value from analogue"]
    DETECT_ENABLE_FIVE_WIRE_0 = 0,
    #[doc = "1: Read five wire detect status from analogue"]
    DETECT_ENABLE_FIVE_WIRE_1 = 1,
}
impl From<DETECT_ENABLE_FIVE_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: DETECT_ENABLE_FIVE_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DETECT_ENABLE_FIVE_WIRE`"]
pub type DETECT_ENABLE_FIVE_WIRE_R = crate::R<bool, DETECT_ENABLE_FIVE_WIRE_A>;
impl DETECT_ENABLE_FIVE_WIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECT_ENABLE_FIVE_WIRE_A {
        match self.bits {
            false => DETECT_ENABLE_FIVE_WIRE_A::DETECT_ENABLE_FIVE_WIRE_0,
            true => DETECT_ENABLE_FIVE_WIRE_A::DETECT_ENABLE_FIVE_WIRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_ENABLE_FIVE_WIRE_0`"]
    #[inline(always)]
    pub fn is_detect_enable_five_wire_0(&self) -> bool {
        *self == DETECT_ENABLE_FIVE_WIRE_A::DETECT_ENABLE_FIVE_WIRE_0
    }
    #[doc = "Checks if the value of the field is `DETECT_ENABLE_FIVE_WIRE_1`"]
    #[inline(always)]
    pub fn is_detect_enable_five_wire_1(&self) -> bool {
        *self == DETECT_ENABLE_FIVE_WIRE_A::DETECT_ENABLE_FIVE_WIRE_1
    }
}
#[doc = "Write proxy for field `DETECT_ENABLE_FIVE_WIRE`"]
pub struct DETECT_ENABLE_FIVE_WIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECT_ENABLE_FIVE_WIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DETECT_ENABLE_FIVE_WIRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not read five wire detect value, read default value from analogue"]
    #[inline(always)]
    pub fn detect_enable_five_wire_0(self) -> &'a mut W {
        self.variant(DETECT_ENABLE_FIVE_WIRE_A::DETECT_ENABLE_FIVE_WIRE_0)
    }
    #[doc = "Read five wire detect status from analogue"]
    #[inline(always)]
    pub fn detect_enable_five_wire_1(self) -> &'a mut W {
        self.variant(DETECT_ENABLE_FIVE_WIRE_A::DETECT_ENABLE_FIVE_WIRE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "This field indicates glitch threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DE_GLITCH_A {
    #[doc = "0: Normal function: 0x1fff ipg clock cycles; Low power mode: 0x9 low power clock cycles"]
    DE_GLITCH_0 = 0,
    #[doc = "1: Normal function: 0xfff ipg clock cycles; Low power mode: :0x7 low power clock cycles"]
    DE_GLITCH_1 = 1,
    #[doc = "2: Normal function: 0x7ff ipg clock cycles; Low power mode:0x5 low power clock cycles"]
    DE_GLITCH_2 = 2,
    #[doc = "3: Normal function: 0x3 ipg clock cycles; Low power mode:0x3 low power clock cycles"]
    DE_GLITCH_3 = 3,
}
impl From<DE_GLITCH_A> for u8 {
    #[inline(always)]
    fn from(variant: DE_GLITCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DE_GLITCH`"]
pub type DE_GLITCH_R = crate::R<u8, DE_GLITCH_A>;
impl DE_GLITCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DE_GLITCH_A {
        match self.bits {
            0 => DE_GLITCH_A::DE_GLITCH_0,
            1 => DE_GLITCH_A::DE_GLITCH_1,
            2 => DE_GLITCH_A::DE_GLITCH_2,
            3 => DE_GLITCH_A::DE_GLITCH_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DE_GLITCH_0`"]
    #[inline(always)]
    pub fn is_de_glitch_0(&self) -> bool {
        *self == DE_GLITCH_A::DE_GLITCH_0
    }
    #[doc = "Checks if the value of the field is `DE_GLITCH_1`"]
    #[inline(always)]
    pub fn is_de_glitch_1(&self) -> bool {
        *self == DE_GLITCH_A::DE_GLITCH_1
    }
    #[doc = "Checks if the value of the field is `DE_GLITCH_2`"]
    #[inline(always)]
    pub fn is_de_glitch_2(&self) -> bool {
        *self == DE_GLITCH_A::DE_GLITCH_2
    }
    #[doc = "Checks if the value of the field is `DE_GLITCH_3`"]
    #[inline(always)]
    pub fn is_de_glitch_3(&self) -> bool {
        *self == DE_GLITCH_A::DE_GLITCH_3
    }
}
impl R {
    #[doc = "Bit 0 - XPUL Wire Pull Down Switch"]
    #[inline(always)]
    pub fn xpul_pull_down(&self) -> XPUL_PULL_DOWN_R {
        XPUL_PULL_DOWN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XPUL Wire Pull Up Switch"]
    #[inline(always)]
    pub fn xpul_pull_up(&self) -> XPUL_PULL_UP_R {
        XPUL_PULL_UP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XPUL Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn xpul_200k_pull_up(&self) -> XPUL_200K_PULL_UP_R {
        XPUL_200K_PULL_UP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XNUR Wire Pull Down Switch"]
    #[inline(always)]
    pub fn xnur_pull_down(&self) -> XNUR_PULL_DOWN_R {
        XNUR_PULL_DOWN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XNUR Wire Pull Up Switch"]
    #[inline(always)]
    pub fn xnur_pull_up(&self) -> XNUR_PULL_UP_R {
        XNUR_PULL_UP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XNUR Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn xnur_200k_pull_up(&self) -> XNUR_200K_PULL_UP_R {
        XNUR_200K_PULL_UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - YPLL Wire Pull Down Switch"]
    #[inline(always)]
    pub fn ypll_pull_down(&self) -> YPLL_PULL_DOWN_R {
        YPLL_PULL_DOWN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - YPLL Wire Pull Up Switch"]
    #[inline(always)]
    pub fn ypll_pull_up(&self) -> YPLL_PULL_UP_R {
        YPLL_PULL_UP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - YPLL Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn ypll_200k_pull_up(&self) -> YPLL_200K_PULL_UP_R {
        YPLL_200K_PULL_UP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - YNLR Wire Pull Down Switch"]
    #[inline(always)]
    pub fn ynlr_pull_down(&self) -> YNLR_PULL_DOWN_R {
        YNLR_PULL_DOWN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - YNLR Wire Pull Up Switch"]
    #[inline(always)]
    pub fn ynlr_pull_up(&self) -> YNLR_PULL_UP_R {
        YNLR_PULL_UP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - YNLR Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn ynlr_200k_pull_up(&self) -> YNLR_200K_PULL_UP_R {
        YNLR_200K_PULL_UP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wiper Wire Pull Down Switch"]
    #[inline(always)]
    pub fn wiper_pull_down(&self) -> WIPER_PULL_DOWN_R {
        WIPER_PULL_DOWN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wiper Wire Pull Up Switch"]
    #[inline(always)]
    pub fn wiper_pull_up(&self) -> WIPER_PULL_UP_R {
        WIPER_PULL_UP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wiper Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn wiper_200k_pull_up(&self) -> WIPER_200K_PULL_UP_R {
        WIPER_200K_PULL_UP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Detect Four Wire"]
    #[inline(always)]
    pub fn detect_four_wire(&self) -> DETECT_FOUR_WIRE_R {
        DETECT_FOUR_WIRE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Detect Five Wire"]
    #[inline(always)]
    pub fn detect_five_wire(&self) -> DETECT_FIVE_WIRE_R {
        DETECT_FIVE_WIRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - State Machine"]
    #[inline(always)]
    pub fn state_machine(&self) -> STATE_MACHINE_R {
        STATE_MACHINE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Intermediate State"]
    #[inline(always)]
    pub fn intermediate(&self) -> INTERMEDIATE_R {
        INTERMEDIATE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Detect Enable Four Wire"]
    #[inline(always)]
    pub fn detect_enable_four_wire(&self) -> DETECT_ENABLE_FOUR_WIRE_R {
        DETECT_ENABLE_FOUR_WIRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Detect Enable Five Wire"]
    #[inline(always)]
    pub fn detect_enable_five_wire(&self) -> DETECT_ENABLE_FIVE_WIRE_R {
        DETECT_ENABLE_FIVE_WIRE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - This field indicates glitch threshold"]
    #[inline(always)]
    pub fn de_glitch(&self) -> DE_GLITCH_R {
        DE_GLITCH_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XPUL Wire Pull Down Switch"]
    #[inline(always)]
    pub fn xpul_pull_down(&mut self) -> XPUL_PULL_DOWN_W {
        XPUL_PULL_DOWN_W { w: self }
    }
    #[doc = "Bit 1 - XPUL Wire Pull Up Switch"]
    #[inline(always)]
    pub fn xpul_pull_up(&mut self) -> XPUL_PULL_UP_W {
        XPUL_PULL_UP_W { w: self }
    }
    #[doc = "Bit 2 - XPUL Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn xpul_200k_pull_up(&mut self) -> XPUL_200K_PULL_UP_W {
        XPUL_200K_PULL_UP_W { w: self }
    }
    #[doc = "Bit 3 - XNUR Wire Pull Down Switch"]
    #[inline(always)]
    pub fn xnur_pull_down(&mut self) -> XNUR_PULL_DOWN_W {
        XNUR_PULL_DOWN_W { w: self }
    }
    #[doc = "Bit 4 - XNUR Wire Pull Up Switch"]
    #[inline(always)]
    pub fn xnur_pull_up(&mut self) -> XNUR_PULL_UP_W {
        XNUR_PULL_UP_W { w: self }
    }
    #[doc = "Bit 5 - XNUR Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn xnur_200k_pull_up(&mut self) -> XNUR_200K_PULL_UP_W {
        XNUR_200K_PULL_UP_W { w: self }
    }
    #[doc = "Bit 6 - YPLL Wire Pull Down Switch"]
    #[inline(always)]
    pub fn ypll_pull_down(&mut self) -> YPLL_PULL_DOWN_W {
        YPLL_PULL_DOWN_W { w: self }
    }
    #[doc = "Bit 7 - YPLL Wire Pull Up Switch"]
    #[inline(always)]
    pub fn ypll_pull_up(&mut self) -> YPLL_PULL_UP_W {
        YPLL_PULL_UP_W { w: self }
    }
    #[doc = "Bit 8 - YPLL Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn ypll_200k_pull_up(&mut self) -> YPLL_200K_PULL_UP_W {
        YPLL_200K_PULL_UP_W { w: self }
    }
    #[doc = "Bit 9 - YNLR Wire Pull Down Switch"]
    #[inline(always)]
    pub fn ynlr_pull_down(&mut self) -> YNLR_PULL_DOWN_W {
        YNLR_PULL_DOWN_W { w: self }
    }
    #[doc = "Bit 10 - YNLR Wire Pull Up Switch"]
    #[inline(always)]
    pub fn ynlr_pull_up(&mut self) -> YNLR_PULL_UP_W {
        YNLR_PULL_UP_W { w: self }
    }
    #[doc = "Bit 11 - YNLR Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn ynlr_200k_pull_up(&mut self) -> YNLR_200K_PULL_UP_W {
        YNLR_200K_PULL_UP_W { w: self }
    }
    #[doc = "Bit 12 - Wiper Wire Pull Down Switch"]
    #[inline(always)]
    pub fn wiper_pull_down(&mut self) -> WIPER_PULL_DOWN_W {
        WIPER_PULL_DOWN_W { w: self }
    }
    #[doc = "Bit 13 - Wiper Wire Pull Up Switch"]
    #[inline(always)]
    pub fn wiper_pull_up(&mut self) -> WIPER_PULL_UP_W {
        WIPER_PULL_UP_W { w: self }
    }
    #[doc = "Bit 14 - Wiper Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub fn wiper_200k_pull_up(&mut self) -> WIPER_200K_PULL_UP_W {
        WIPER_200K_PULL_UP_W { w: self }
    }
    #[doc = "Bit 24 - Detect Enable Four Wire"]
    #[inline(always)]
    pub fn detect_enable_four_wire(&mut self) -> DETECT_ENABLE_FOUR_WIRE_W {
        DETECT_ENABLE_FOUR_WIRE_W { w: self }
    }
    #[doc = "Bit 28 - Detect Enable Five Wire"]
    #[inline(always)]
    pub fn detect_enable_five_wire(&mut self) -> DETECT_ENABLE_FIVE_WIRE_W {
        DETECT_ENABLE_FIVE_WIRE_W { w: self }
    }
}
