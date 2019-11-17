#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: GPT is disabled."]
    EN_0,
    #[doc = "1: GPT is enabled."]
    EN_1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        match variant {
            EN_A::EN_0 => false,
            EN_A::EN_1 => true,
        }
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
    #[doc = "GPT is disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "GPT is enabled."]
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
#[doc = "GPT Enable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENMOD_A {
    #[doc = "0: GPT counter will retain its value when it is disabled."]
    ENMOD_0,
    #[doc = "1: GPT counter value is reset to 0 when it is disabled."]
    ENMOD_1,
}
impl From<ENMOD_A> for bool {
    #[inline(always)]
    fn from(variant: ENMOD_A) -> Self {
        match variant {
            ENMOD_A::ENMOD_0 => false,
            ENMOD_A::ENMOD_1 => true,
        }
    }
}
#[doc = "Reader of field `ENMOD`"]
pub type ENMOD_R = crate::R<bool, ENMOD_A>;
impl ENMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENMOD_A {
        match self.bits {
            false => ENMOD_A::ENMOD_0,
            true => ENMOD_A::ENMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENMOD_0`"]
    #[inline(always)]
    pub fn is_enmod_0(&self) -> bool {
        *self == ENMOD_A::ENMOD_0
    }
    #[doc = "Checks if the value of the field is `ENMOD_1`"]
    #[inline(always)]
    pub fn is_enmod_1(&self) -> bool {
        *self == ENMOD_A::ENMOD_1
    }
}
#[doc = "Write proxy for field `ENMOD`"]
pub struct ENMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPT counter will retain its value when it is disabled."]
    #[inline(always)]
    pub fn enmod_0(self) -> &'a mut W {
        self.variant(ENMOD_A::ENMOD_0)
    }
    #[doc = "GPT counter value is reset to 0 when it is disabled."]
    #[inline(always)]
    pub fn enmod_1(self) -> &'a mut W {
        self.variant(ENMOD_A::ENMOD_1)
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
#[doc = "GPT debug mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_A {
    #[doc = "0: GPT is disabled in debug mode."]
    DBGEN_0,
    #[doc = "1: GPT is enabled in debug mode."]
    DBGEN_1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        match variant {
            DBGEN_A::DBGEN_0 => false,
            DBGEN_A::DBGEN_1 => true,
        }
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
    #[doc = "GPT is disabled in debug mode."]
    #[inline(always)]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_0)
    }
    #[doc = "GPT is enabled in debug mode."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "GPT Wait Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITEN_A {
    #[doc = "0: GPT is disabled in wait mode."]
    WAITEN_0,
    #[doc = "1: GPT is enabled in wait mode."]
    WAITEN_1,
}
impl From<WAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITEN_A) -> Self {
        match variant {
            WAITEN_A::WAITEN_0 => false,
            WAITEN_A::WAITEN_1 => true,
        }
    }
}
#[doc = "Reader of field `WAITEN`"]
pub type WAITEN_R = crate::R<bool, WAITEN_A>;
impl WAITEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITEN_A {
        match self.bits {
            false => WAITEN_A::WAITEN_0,
            true => WAITEN_A::WAITEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAITEN_0`"]
    #[inline(always)]
    pub fn is_waiten_0(&self) -> bool {
        *self == WAITEN_A::WAITEN_0
    }
    #[doc = "Checks if the value of the field is `WAITEN_1`"]
    #[inline(always)]
    pub fn is_waiten_1(&self) -> bool {
        *self == WAITEN_A::WAITEN_1
    }
}
#[doc = "Write proxy for field `WAITEN`"]
pub struct WAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPT is disabled in wait mode."]
    #[inline(always)]
    pub fn waiten_0(self) -> &'a mut W {
        self.variant(WAITEN_A::WAITEN_0)
    }
    #[doc = "GPT is enabled in wait mode."]
    #[inline(always)]
    pub fn waiten_1(self) -> &'a mut W {
        self.variant(WAITEN_A::WAITEN_1)
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
#[doc = "GPT Doze Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEEN_A {
    #[doc = "0: GPT is disabled in doze mode."]
    DOZEEN_0,
    #[doc = "1: GPT is enabled in doze mode."]
    DOZEEN_1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        match variant {
            DOZEEN_A::DOZEEN_0 => false,
            DOZEEN_A::DOZEEN_1 => true,
        }
    }
}
#[doc = "Reader of field `DOZEEN`"]
pub type DOZEEN_R = crate::R<bool, DOZEEN_A>;
impl DOZEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::DOZEEN_0,
            true => DOZEEN_A::DOZEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEEN_0`"]
    #[inline(always)]
    pub fn is_dozeen_0(&self) -> bool {
        *self == DOZEEN_A::DOZEEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEEN_1`"]
    #[inline(always)]
    pub fn is_dozeen_1(&self) -> bool {
        *self == DOZEEN_A::DOZEEN_1
    }
}
#[doc = "Write proxy for field `DOZEEN`"]
pub struct DOZEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPT is disabled in doze mode."]
    #[inline(always)]
    pub fn dozeen_0(self) -> &'a mut W {
        self.variant(DOZEEN_A::DOZEEN_0)
    }
    #[doc = "GPT is enabled in doze mode."]
    #[inline(always)]
    pub fn dozeen_1(self) -> &'a mut W {
        self.variant(DOZEEN_A::DOZEEN_1)
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
#[doc = "GPT Stop Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPEN_A {
    #[doc = "0: GPT is disabled in Stop mode."]
    STOPEN_0,
    #[doc = "1: GPT is enabled in Stop mode."]
    STOPEN_1,
}
impl From<STOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOPEN_A) -> Self {
        match variant {
            STOPEN_A::STOPEN_0 => false,
            STOPEN_A::STOPEN_1 => true,
        }
    }
}
#[doc = "Reader of field `STOPEN`"]
pub type STOPEN_R = crate::R<bool, STOPEN_A>;
impl STOPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPEN_A {
        match self.bits {
            false => STOPEN_A::STOPEN_0,
            true => STOPEN_A::STOPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STOPEN_0`"]
    #[inline(always)]
    pub fn is_stopen_0(&self) -> bool {
        *self == STOPEN_A::STOPEN_0
    }
    #[doc = "Checks if the value of the field is `STOPEN_1`"]
    #[inline(always)]
    pub fn is_stopen_1(&self) -> bool {
        *self == STOPEN_A::STOPEN_1
    }
}
#[doc = "Write proxy for field `STOPEN`"]
pub struct STOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPT is disabled in Stop mode."]
    #[inline(always)]
    pub fn stopen_0(self) -> &'a mut W {
        self.variant(STOPEN_A::STOPEN_0)
    }
    #[doc = "GPT is enabled in Stop mode."]
    #[inline(always)]
    pub fn stopen_1(self) -> &'a mut W {
        self.variant(STOPEN_A::STOPEN_1)
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
#[doc = "Clock Source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_A {
    #[doc = "0: No clock"]
    CLKSRC_0,
    #[doc = "1: Peripheral Clock (ipg_clk)"]
    CLKSRC_1,
    #[doc = "2: High Frequency Reference Clock (ipg_clk_highfreq)"]
    CLKSRC_2,
    #[doc = "3: External Clock"]
    CLKSRC_3,
    #[doc = "4: Low Frequency Reference Clock (ipg_clk_32k)"]
    CLKSRC_4,
    #[doc = "5: Crystal oscillator as Reference Clock (ipg_clk_24M)"]
    CLKSRC_5,
}
impl From<CLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        match variant {
            CLKSRC_A::CLKSRC_0 => 0,
            CLKSRC_A::CLKSRC_1 => 1,
            CLKSRC_A::CLKSRC_2 => 2,
            CLKSRC_A::CLKSRC_3 => 3,
            CLKSRC_A::CLKSRC_4 => 4,
            CLKSRC_A::CLKSRC_5 => 5,
        }
    }
}
#[doc = "Reader of field `CLKSRC`"]
pub type CLKSRC_R = crate::R<u8, CLKSRC_A>;
impl CLKSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSRC_A::CLKSRC_0),
            1 => Val(CLKSRC_A::CLKSRC_1),
            2 => Val(CLKSRC_A::CLKSRC_2),
            3 => Val(CLKSRC_A::CLKSRC_3),
            4 => Val(CLKSRC_A::CLKSRC_4),
            5 => Val(CLKSRC_A::CLKSRC_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_0`"]
    #[inline(always)]
    pub fn is_clksrc_0(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_1`"]
    #[inline(always)]
    pub fn is_clksrc_1(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_1
    }
    #[doc = "Checks if the value of the field is `CLKSRC_2`"]
    #[inline(always)]
    pub fn is_clksrc_2(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_2
    }
    #[doc = "Checks if the value of the field is `CLKSRC_3`"]
    #[inline(always)]
    pub fn is_clksrc_3(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_3
    }
    #[doc = "Checks if the value of the field is `CLKSRC_4`"]
    #[inline(always)]
    pub fn is_clksrc_4(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_4
    }
    #[doc = "Checks if the value of the field is `CLKSRC_5`"]
    #[inline(always)]
    pub fn is_clksrc_5(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_5
    }
}
#[doc = "Write proxy for field `CLKSRC`"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn clksrc_0(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_0)
    }
    #[doc = "Peripheral Clock (ipg_clk)"]
    #[inline(always)]
    pub fn clksrc_1(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_1)
    }
    #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)"]
    #[inline(always)]
    pub fn clksrc_2(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_2)
    }
    #[doc = "External Clock"]
    #[inline(always)]
    pub fn clksrc_3(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_3)
    }
    #[doc = "Low Frequency Reference Clock (ipg_clk_32k)"]
    #[inline(always)]
    pub fn clksrc_4(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_4)
    }
    #[doc = "Crystal oscillator as Reference Clock (ipg_clk_24M)"]
    #[inline(always)]
    pub fn clksrc_5(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Free-Run or Restart mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRR_A {
    #[doc = "0: Restart mode"]
    FRR_0,
    #[doc = "1: Free-Run mode"]
    FRR_1,
}
impl From<FRR_A> for bool {
    #[inline(always)]
    fn from(variant: FRR_A) -> Self {
        match variant {
            FRR_A::FRR_0 => false,
            FRR_A::FRR_1 => true,
        }
    }
}
#[doc = "Reader of field `FRR`"]
pub type FRR_R = crate::R<bool, FRR_A>;
impl FRR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRR_A {
        match self.bits {
            false => FRR_A::FRR_0,
            true => FRR_A::FRR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRR_0`"]
    #[inline(always)]
    pub fn is_frr_0(&self) -> bool {
        *self == FRR_A::FRR_0
    }
    #[doc = "Checks if the value of the field is `FRR_1`"]
    #[inline(always)]
    pub fn is_frr_1(&self) -> bool {
        *self == FRR_A::FRR_1
    }
}
#[doc = "Write proxy for field `FRR`"]
pub struct FRR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Restart mode"]
    #[inline(always)]
    pub fn frr_0(self) -> &'a mut W {
        self.variant(FRR_A::FRR_0)
    }
    #[doc = "Free-Run mode"]
    #[inline(always)]
    pub fn frr_1(self) -> &'a mut W {
        self.variant(FRR_A::FRR_1)
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
#[doc = "Enable 24 MHz clock input from crystal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_24M_A {
    #[doc = "0: 24M clock disabled"]
    EN_24M_0,
    #[doc = "1: 24M clock enabled"]
    EN_24M_1,
}
impl From<EN_24M_A> for bool {
    #[inline(always)]
    fn from(variant: EN_24M_A) -> Self {
        match variant {
            EN_24M_A::EN_24M_0 => false,
            EN_24M_A::EN_24M_1 => true,
        }
    }
}
#[doc = "Reader of field `EN_24M`"]
pub type EN_24M_R = crate::R<bool, EN_24M_A>;
impl EN_24M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_24M_A {
        match self.bits {
            false => EN_24M_A::EN_24M_0,
            true => EN_24M_A::EN_24M_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_24M_0`"]
    #[inline(always)]
    pub fn is_en_24m_0(&self) -> bool {
        *self == EN_24M_A::EN_24M_0
    }
    #[doc = "Checks if the value of the field is `EN_24M_1`"]
    #[inline(always)]
    pub fn is_en_24m_1(&self) -> bool {
        *self == EN_24M_A::EN_24M_1
    }
}
#[doc = "Write proxy for field `EN_24M`"]
pub struct EN_24M_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_24M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_24M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "24M clock disabled"]
    #[inline(always)]
    pub fn en_24m_0(self) -> &'a mut W {
        self.variant(EN_24M_A::EN_24M_0)
    }
    #[doc = "24M clock enabled"]
    #[inline(always)]
    pub fn en_24m_1(self) -> &'a mut W {
        self.variant(EN_24M_A::EN_24M_1)
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
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR_A {
    #[doc = "0: GPT is not in reset state"]
    SWR_0,
    #[doc = "1: GPT is in reset state"]
    SWR_1,
}
impl From<SWR_A> for bool {
    #[inline(always)]
    fn from(variant: SWR_A) -> Self {
        match variant {
            SWR_A::SWR_0 => false,
            SWR_A::SWR_1 => true,
        }
    }
}
#[doc = "Reader of field `SWR`"]
pub type SWR_R = crate::R<bool, SWR_A>;
impl SWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWR_A {
        match self.bits {
            false => SWR_A::SWR_0,
            true => SWR_A::SWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWR_0`"]
    #[inline(always)]
    pub fn is_swr_0(&self) -> bool {
        *self == SWR_A::SWR_0
    }
    #[doc = "Checks if the value of the field is `SWR_1`"]
    #[inline(always)]
    pub fn is_swr_1(&self) -> bool {
        *self == SWR_A::SWR_1
    }
}
#[doc = "Write proxy for field `SWR`"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPT is not in reset state"]
    #[inline(always)]
    pub fn swr_0(self) -> &'a mut W {
        self.variant(SWR_A::SWR_0)
    }
    #[doc = "GPT is in reset state"]
    #[inline(always)]
    pub fn swr_1(self) -> &'a mut W {
        self.variant(SWR_A::SWR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `IM1`"]
pub type IM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IM1`"]
pub struct IM1_W<'a> {
    w: &'a mut W,
}
impl<'a> IM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM2_A {
    #[doc = "0: capture disabled"]
    IM2_0,
    #[doc = "1: capture on rising edge only"]
    IM2_1,
    #[doc = "2: capture on falling edge only"]
    IM2_2,
    #[doc = "3: capture on both edges"]
    IM2_3,
}
impl From<IM2_A> for u8 {
    #[inline(always)]
    fn from(variant: IM2_A) -> Self {
        match variant {
            IM2_A::IM2_0 => 0,
            IM2_A::IM2_1 => 1,
            IM2_A::IM2_2 => 2,
            IM2_A::IM2_3 => 3,
        }
    }
}
#[doc = "Reader of field `IM2`"]
pub type IM2_R = crate::R<u8, IM2_A>;
impl IM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM2_A {
        match self.bits {
            0 => IM2_A::IM2_0,
            1 => IM2_A::IM2_1,
            2 => IM2_A::IM2_2,
            3 => IM2_A::IM2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IM2_0`"]
    #[inline(always)]
    pub fn is_im2_0(&self) -> bool {
        *self == IM2_A::IM2_0
    }
    #[doc = "Checks if the value of the field is `IM2_1`"]
    #[inline(always)]
    pub fn is_im2_1(&self) -> bool {
        *self == IM2_A::IM2_1
    }
    #[doc = "Checks if the value of the field is `IM2_2`"]
    #[inline(always)]
    pub fn is_im2_2(&self) -> bool {
        *self == IM2_A::IM2_2
    }
    #[doc = "Checks if the value of the field is `IM2_3`"]
    #[inline(always)]
    pub fn is_im2_3(&self) -> bool {
        *self == IM2_A::IM2_3
    }
}
#[doc = "Write proxy for field `IM2`"]
pub struct IM2_W<'a> {
    w: &'a mut W,
}
impl<'a> IM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IM2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "capture disabled"]
    #[inline(always)]
    pub fn im2_0(self) -> &'a mut W {
        self.variant(IM2_A::IM2_0)
    }
    #[doc = "capture on rising edge only"]
    #[inline(always)]
    pub fn im2_1(self) -> &'a mut W {
        self.variant(IM2_A::IM2_1)
    }
    #[doc = "capture on falling edge only"]
    #[inline(always)]
    pub fn im2_2(self) -> &'a mut W {
        self.variant(IM2_A::IM2_2)
    }
    #[doc = "capture on both edges"]
    #[inline(always)]
    pub fn im2_3(self) -> &'a mut W {
        self.variant(IM2_A::IM2_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `OM1`"]
pub type OM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OM1`"]
pub struct OM1_W<'a> {
    w: &'a mut W,
}
impl<'a> OM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `OM2`"]
pub type OM2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OM2`"]
pub struct OM2_W<'a> {
    w: &'a mut W,
}
impl<'a> OM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OM3_A {
    #[doc = "0: Output disconnected. No response on pin."]
    OM3_0,
    #[doc = "1: Toggle output pin"]
    OM3_1,
    #[doc = "2: Clear output pin"]
    OM3_2,
    #[doc = "3: Set output pin"]
    OM3_3,
    #[doc = "4: Generate an active low pulse (that is one input clock wide) on the output pin."]
    OM3_4,
}
impl From<OM3_A> for u8 {
    #[inline(always)]
    fn from(variant: OM3_A) -> Self {
        match variant {
            OM3_A::OM3_0 => 0,
            OM3_A::OM3_1 => 1,
            OM3_A::OM3_2 => 2,
            OM3_A::OM3_3 => 3,
            OM3_A::OM3_4 => 4,
        }
    }
}
#[doc = "Reader of field `OM3`"]
pub type OM3_R = crate::R<u8, OM3_A>;
impl OM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OM3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OM3_A::OM3_0),
            1 => Val(OM3_A::OM3_1),
            2 => Val(OM3_A::OM3_2),
            3 => Val(OM3_A::OM3_3),
            4 => Val(OM3_A::OM3_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OM3_0`"]
    #[inline(always)]
    pub fn is_om3_0(&self) -> bool {
        *self == OM3_A::OM3_0
    }
    #[doc = "Checks if the value of the field is `OM3_1`"]
    #[inline(always)]
    pub fn is_om3_1(&self) -> bool {
        *self == OM3_A::OM3_1
    }
    #[doc = "Checks if the value of the field is `OM3_2`"]
    #[inline(always)]
    pub fn is_om3_2(&self) -> bool {
        *self == OM3_A::OM3_2
    }
    #[doc = "Checks if the value of the field is `OM3_3`"]
    #[inline(always)]
    pub fn is_om3_3(&self) -> bool {
        *self == OM3_A::OM3_3
    }
    #[doc = "Checks if the value of the field is `OM3_4`"]
    #[inline(always)]
    pub fn is_om3_4(&self) -> bool {
        *self == OM3_A::OM3_4
    }
}
#[doc = "Write proxy for field `OM3`"]
pub struct OM3_W<'a> {
    w: &'a mut W,
}
impl<'a> OM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output disconnected. No response on pin."]
    #[inline(always)]
    pub fn om3_0(self) -> &'a mut W {
        self.variant(OM3_A::OM3_0)
    }
    #[doc = "Toggle output pin"]
    #[inline(always)]
    pub fn om3_1(self) -> &'a mut W {
        self.variant(OM3_A::OM3_1)
    }
    #[doc = "Clear output pin"]
    #[inline(always)]
    pub fn om3_2(self) -> &'a mut W {
        self.variant(OM3_A::OM3_2)
    }
    #[doc = "Set output pin"]
    #[inline(always)]
    pub fn om3_3(self) -> &'a mut W {
        self.variant(OM3_A::OM3_3)
    }
    #[doc = "Generate an active low pulse (that is one input clock wide) on the output pin."]
    #[inline(always)]
    pub fn om3_4(self) -> &'a mut W {
        self.variant(OM3_A::OM3_4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Reader of field `FO1`"]
pub type FO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FO1`"]
pub struct FO1_W<'a> {
    w: &'a mut W,
}
impl<'a> FO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `FO2`"]
pub type FO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FO2`"]
pub struct FO2_W<'a> {
    w: &'a mut W,
}
impl<'a> FO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FO3_A {
    #[doc = "0: Writing a 0 has no effect."]
    FO3_0,
    #[doc = "1: Causes the programmed pin action on the timer Output Compare n pin; the OFn flag is not set."]
    FO3_1,
}
impl From<FO3_A> for bool {
    #[inline(always)]
    fn from(variant: FO3_A) -> Self {
        match variant {
            FO3_A::FO3_0 => false,
            FO3_A::FO3_1 => true,
        }
    }
}
#[doc = "Reader of field `FO3`"]
pub type FO3_R = crate::R<bool, FO3_A>;
impl FO3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FO3_A {
        match self.bits {
            false => FO3_A::FO3_0,
            true => FO3_A::FO3_1,
        }
    }
    #[doc = "Checks if the value of the field is `FO3_0`"]
    #[inline(always)]
    pub fn is_fo3_0(&self) -> bool {
        *self == FO3_A::FO3_0
    }
    #[doc = "Checks if the value of the field is `FO3_1`"]
    #[inline(always)]
    pub fn is_fo3_1(&self) -> bool {
        *self == FO3_A::FO3_1
    }
}
#[doc = "Write proxy for field `FO3`"]
pub struct FO3_W<'a> {
    w: &'a mut W,
}
impl<'a> FO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FO3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing a 0 has no effect."]
    #[inline(always)]
    pub fn fo3_0(self) -> &'a mut W {
        self.variant(FO3_A::FO3_0)
    }
    #[doc = "Causes the programmed pin action on the timer Output Compare n pin; the OFn flag is not set."]
    #[inline(always)]
    pub fn fo3_1(self) -> &'a mut W {
        self.variant(FO3_A::FO3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPT Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPT Enable mode"]
    #[inline(always)]
    pub fn enmod(&self) -> ENMOD_R {
        ENMOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPT debug mode enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPT Wait Mode enable"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPT Doze Mode Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPT Stop Mode enable"]
    #[inline(always)]
    pub fn stopen(&self) -> STOPEN_R {
        STOPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Clock Source select"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Free-Run or Restart mode"]
    #[inline(always)]
    pub fn frr(&self) -> FRR_R {
        FRR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable 24 MHz clock input from crystal"]
    #[inline(always)]
    pub fn en_24m(&self) -> EN_24M_R {
        EN_24M_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - See IM2"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - See OM3"]
    #[inline(always)]
    pub fn om1(&self) -> OM1_R {
        OM1_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - See OM3"]
    #[inline(always)]
    pub fn om2(&self) -> OM2_R {
        OM2_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline(always)]
    pub fn om3(&self) -> OM3_R {
        OM3_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bit 29 - See F03"]
    #[inline(always)]
    pub fn fo1(&self) -> FO1_R {
        FO1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - See F03"]
    #[inline(always)]
    pub fn fo2(&self) -> FO2_R {
        FO2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[inline(always)]
    pub fn fo3(&self) -> FO3_R {
        FO3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPT Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - GPT Enable mode"]
    #[inline(always)]
    pub fn enmod(&mut self) -> ENMOD_W {
        ENMOD_W { w: self }
    }
    #[doc = "Bit 2 - GPT debug mode enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 3 - GPT Wait Mode enable"]
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W {
        WAITEN_W { w: self }
    }
    #[doc = "Bit 4 - GPT Doze Mode Enable"]
    #[inline(always)]
    pub fn dozeen(&mut self) -> DOZEEN_W {
        DOZEEN_W { w: self }
    }
    #[doc = "Bit 5 - GPT Stop Mode enable"]
    #[inline(always)]
    pub fn stopen(&mut self) -> STOPEN_W {
        STOPEN_W { w: self }
    }
    #[doc = "Bits 6:8 - Clock Source select"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Bit 9 - Free-Run or Restart mode"]
    #[inline(always)]
    pub fn frr(&mut self) -> FRR_W {
        FRR_W { w: self }
    }
    #[doc = "Bit 10 - Enable 24 MHz clock input from crystal"]
    #[inline(always)]
    pub fn en_24m(&mut self) -> EN_24M_W {
        EN_24M_W { w: self }
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bits 16:17 - See IM2"]
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W {
        IM1_W { w: self }
    }
    #[doc = "Bits 18:19 - IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W {
        IM2_W { w: self }
    }
    #[doc = "Bits 20:22 - See OM3"]
    #[inline(always)]
    pub fn om1(&mut self) -> OM1_W {
        OM1_W { w: self }
    }
    #[doc = "Bits 23:25 - See OM3"]
    #[inline(always)]
    pub fn om2(&mut self) -> OM2_W {
        OM2_W { w: self }
    }
    #[doc = "Bits 26:28 - OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline(always)]
    pub fn om3(&mut self) -> OM3_W {
        OM3_W { w: self }
    }
    #[doc = "Bit 29 - See F03"]
    #[inline(always)]
    pub fn fo1(&mut self) -> FO1_W {
        FO1_W { w: self }
    }
    #[doc = "Bit 30 - See F03"]
    #[inline(always)]
    pub fn fo2(&mut self) -> FO2_W {
        FO2_W { w: self }
    }
    #[doc = "Bit 31 - FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[inline(always)]
    pub fn fo3(&mut self) -> FO3_W {
        FO3_W { w: self }
    }
}
