#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADICLK_A {
    #[doc = "0: IPG clock"]
    ADICLK_0 = 0,
    #[doc = "1: IPG clock divided by 2"]
    ADICLK_1 = 1,
    #[doc = "3: Asynchronous clock (ADACK)"]
    ADICLK_3 = 3,
}
impl From<ADICLK_A> for u8 {
    #[inline(always)]
    fn from(variant: ADICLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADICLK`"]
pub type ADICLK_R = crate::R<u8, ADICLK_A>;
impl ADICLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADICLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADICLK_A::ADICLK_0),
            1 => Val(ADICLK_A::ADICLK_1),
            3 => Val(ADICLK_A::ADICLK_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADICLK_0`"]
    #[inline(always)]
    pub fn is_adiclk_0(&self) -> bool {
        *self == ADICLK_A::ADICLK_0
    }
    #[doc = "Checks if the value of the field is `ADICLK_1`"]
    #[inline(always)]
    pub fn is_adiclk_1(&self) -> bool {
        *self == ADICLK_A::ADICLK_1
    }
    #[doc = "Checks if the value of the field is `ADICLK_3`"]
    #[inline(always)]
    pub fn is_adiclk_3(&self) -> bool {
        *self == ADICLK_A::ADICLK_3
    }
}
#[doc = "Write proxy for field `ADICLK`"]
pub struct ADICLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADICLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IPG clock"]
    #[inline(always)]
    pub fn adiclk_0(self) -> &'a mut W {
        self.variant(ADICLK_A::ADICLK_0)
    }
    #[doc = "IPG clock divided by 2"]
    #[inline(always)]
    pub fn adiclk_1(self) -> &'a mut W {
        self.variant(ADICLK_A::ADICLK_1)
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline(always)]
    pub fn adiclk_3(self) -> &'a mut W {
        self.variant(ADICLK_A::ADICLK_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Conversion Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 8-bit conversion"]
    MODE_0 = 0,
    #[doc = "1: 10-bit conversion"]
    MODE_1 = 1,
    #[doc = "2: 12-bit conversion"]
    MODE_2 = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::MODE_0),
            1 => Val(MODE_A::MODE_1),
            2 => Val(MODE_A::MODE_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == MODE_A::MODE_1
    }
    #[doc = "Checks if the value of the field is `MODE_2`"]
    #[inline(always)]
    pub fn is_mode_2(&self) -> bool {
        *self == MODE_A::MODE_2
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
    }
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn mode_2(self) -> &'a mut W {
        self.variant(MODE_A::MODE_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Long Sample Time Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLSMP_A {
    #[doc = "0: Short sample mode."]
    ADLSMP_0 = 0,
    #[doc = "1: Long sample mode."]
    ADLSMP_1 = 1,
}
impl From<ADLSMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADLSMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADLSMP`"]
pub type ADLSMP_R = crate::R<bool, ADLSMP_A>;
impl ADLSMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLSMP_A {
        match self.bits {
            false => ADLSMP_A::ADLSMP_0,
            true => ADLSMP_A::ADLSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADLSMP_0`"]
    #[inline(always)]
    pub fn is_adlsmp_0(&self) -> bool {
        *self == ADLSMP_A::ADLSMP_0
    }
    #[doc = "Checks if the value of the field is `ADLSMP_1`"]
    #[inline(always)]
    pub fn is_adlsmp_1(&self) -> bool {
        *self == ADLSMP_A::ADLSMP_1
    }
}
#[doc = "Write proxy for field `ADLSMP`"]
pub struct ADLSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLSMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADLSMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Short sample mode."]
    #[inline(always)]
    pub fn adlsmp_0(self) -> &'a mut W {
        self.variant(ADLSMP_A::ADLSMP_0)
    }
    #[doc = "Long sample mode."]
    #[inline(always)]
    pub fn adlsmp_1(self) -> &'a mut W {
        self.variant(ADLSMP_A::ADLSMP_1)
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
#[doc = "Clock Divide Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADIV_A {
    #[doc = "0: Input clock"]
    ADIV_0 = 0,
    #[doc = "1: Input clock / 2"]
    ADIV_1 = 1,
    #[doc = "2: Input clock / 4"]
    ADIV_2 = 2,
    #[doc = "3: Input clock / 8"]
    ADIV_3 = 3,
}
impl From<ADIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADIV`"]
pub type ADIV_R = crate::R<u8, ADIV_A>;
impl ADIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIV_A {
        match self.bits {
            0 => ADIV_A::ADIV_0,
            1 => ADIV_A::ADIV_1,
            2 => ADIV_A::ADIV_2,
            3 => ADIV_A::ADIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADIV_0`"]
    #[inline(always)]
    pub fn is_adiv_0(&self) -> bool {
        *self == ADIV_A::ADIV_0
    }
    #[doc = "Checks if the value of the field is `ADIV_1`"]
    #[inline(always)]
    pub fn is_adiv_1(&self) -> bool {
        *self == ADIV_A::ADIV_1
    }
    #[doc = "Checks if the value of the field is `ADIV_2`"]
    #[inline(always)]
    pub fn is_adiv_2(&self) -> bool {
        *self == ADIV_A::ADIV_2
    }
    #[doc = "Checks if the value of the field is `ADIV_3`"]
    #[inline(always)]
    pub fn is_adiv_3(&self) -> bool {
        *self == ADIV_A::ADIV_3
    }
}
#[doc = "Write proxy for field `ADIV`"]
pub struct ADIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input clock"]
    #[inline(always)]
    pub fn adiv_0(self) -> &'a mut W {
        self.variant(ADIV_A::ADIV_0)
    }
    #[doc = "Input clock / 2"]
    #[inline(always)]
    pub fn adiv_1(self) -> &'a mut W {
        self.variant(ADIV_A::ADIV_1)
    }
    #[doc = "Input clock / 4"]
    #[inline(always)]
    pub fn adiv_2(self) -> &'a mut W {
        self.variant(ADIV_A::ADIV_2)
    }
    #[doc = "Input clock / 8"]
    #[inline(always)]
    pub fn adiv_3(self) -> &'a mut W {
        self.variant(ADIV_A::ADIV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Low-Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLPC_A {
    #[doc = "0: ADC hard block not in low power mode."]
    ADLPC_0 = 0,
    #[doc = "1: ADC hard block in low power mode."]
    ADLPC_1 = 1,
}
impl From<ADLPC_A> for bool {
    #[inline(always)]
    fn from(variant: ADLPC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADLPC`"]
pub type ADLPC_R = crate::R<bool, ADLPC_A>;
impl ADLPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLPC_A {
        match self.bits {
            false => ADLPC_A::ADLPC_0,
            true => ADLPC_A::ADLPC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADLPC_0`"]
    #[inline(always)]
    pub fn is_adlpc_0(&self) -> bool {
        *self == ADLPC_A::ADLPC_0
    }
    #[doc = "Checks if the value of the field is `ADLPC_1`"]
    #[inline(always)]
    pub fn is_adlpc_1(&self) -> bool {
        *self == ADLPC_A::ADLPC_1
    }
}
#[doc = "Write proxy for field `ADLPC`"]
pub struct ADLPC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADLPC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC hard block not in low power mode."]
    #[inline(always)]
    pub fn adlpc_0(self) -> &'a mut W {
        self.variant(ADLPC_A::ADLPC_0)
    }
    #[doc = "ADC hard block in low power mode."]
    #[inline(always)]
    pub fn adlpc_1(self) -> &'a mut W {
        self.variant(ADLPC_A::ADLPC_1)
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
#[doc = "Defines the sample time duration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADSTS_A {
    #[doc = "0: Sample period (ADC clocks) = 2 if ADLSMP=0b Sample period (ADC clocks) = 12 if ADLSMP=1b"]
    ADSTS_0 = 0,
    #[doc = "1: Sample period (ADC clocks) = 4 if ADLSMP=0b Sample period (ADC clocks) = 16 if ADLSMP=1b"]
    ADSTS_1 = 1,
    #[doc = "2: Sample period (ADC clocks) = 6 if ADLSMP=0b Sample period (ADC clocks) = 20 if ADLSMP=1b"]
    ADSTS_2 = 2,
    #[doc = "3: Sample period (ADC clocks) = 8 if ADLSMP=0b Sample period (ADC clocks) = 24 if ADLSMP=1b"]
    ADSTS_3 = 3,
}
impl From<ADSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSTS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADSTS`"]
pub type ADSTS_R = crate::R<u8, ADSTS_A>;
impl ADSTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTS_A {
        match self.bits {
            0 => ADSTS_A::ADSTS_0,
            1 => ADSTS_A::ADSTS_1,
            2 => ADSTS_A::ADSTS_2,
            3 => ADSTS_A::ADSTS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADSTS_0`"]
    #[inline(always)]
    pub fn is_adsts_0(&self) -> bool {
        *self == ADSTS_A::ADSTS_0
    }
    #[doc = "Checks if the value of the field is `ADSTS_1`"]
    #[inline(always)]
    pub fn is_adsts_1(&self) -> bool {
        *self == ADSTS_A::ADSTS_1
    }
    #[doc = "Checks if the value of the field is `ADSTS_2`"]
    #[inline(always)]
    pub fn is_adsts_2(&self) -> bool {
        *self == ADSTS_A::ADSTS_2
    }
    #[doc = "Checks if the value of the field is `ADSTS_3`"]
    #[inline(always)]
    pub fn is_adsts_3(&self) -> bool {
        *self == ADSTS_A::ADSTS_3
    }
}
#[doc = "Write proxy for field `ADSTS`"]
pub struct ADSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSTS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Sample period (ADC clocks) = 2 if ADLSMP=0b Sample period (ADC clocks) = 12 if ADLSMP=1b"]
    #[inline(always)]
    pub fn adsts_0(self) -> &'a mut W {
        self.variant(ADSTS_A::ADSTS_0)
    }
    #[doc = "Sample period (ADC clocks) = 4 if ADLSMP=0b Sample period (ADC clocks) = 16 if ADLSMP=1b"]
    #[inline(always)]
    pub fn adsts_1(self) -> &'a mut W {
        self.variant(ADSTS_A::ADSTS_1)
    }
    #[doc = "Sample period (ADC clocks) = 6 if ADLSMP=0b Sample period (ADC clocks) = 20 if ADLSMP=1b"]
    #[inline(always)]
    pub fn adsts_2(self) -> &'a mut W {
        self.variant(ADSTS_A::ADSTS_2)
    }
    #[doc = "Sample period (ADC clocks) = 8 if ADLSMP=0b Sample period (ADC clocks) = 24 if ADLSMP=1b"]
    #[inline(always)]
    pub fn adsts_3(self) -> &'a mut W {
        self.variant(ADSTS_A::ADSTS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "High Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADHSC_A {
    #[doc = "0: Normal conversion selected."]
    ADHSC_0 = 0,
    #[doc = "1: High speed conversion selected."]
    ADHSC_1 = 1,
}
impl From<ADHSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADHSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADHSC`"]
pub type ADHSC_R = crate::R<bool, ADHSC_A>;
impl ADHSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADHSC_A {
        match self.bits {
            false => ADHSC_A::ADHSC_0,
            true => ADHSC_A::ADHSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADHSC_0`"]
    #[inline(always)]
    pub fn is_adhsc_0(&self) -> bool {
        *self == ADHSC_A::ADHSC_0
    }
    #[doc = "Checks if the value of the field is `ADHSC_1`"]
    #[inline(always)]
    pub fn is_adhsc_1(&self) -> bool {
        *self == ADHSC_A::ADHSC_1
    }
}
#[doc = "Write proxy for field `ADHSC`"]
pub struct ADHSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADHSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal conversion selected."]
    #[inline(always)]
    pub fn adhsc_0(self) -> &'a mut W {
        self.variant(ADHSC_A::ADHSC_0)
    }
    #[doc = "High speed conversion selected."]
    #[inline(always)]
    pub fn adhsc_1(self) -> &'a mut W {
        self.variant(ADHSC_A::ADHSC_1)
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
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Selects VREFH/VREFL as reference voltage."]
    REFSEL_0 = 0,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REFSEL_A::REFSEL_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REFSEL_0`"]
    #[inline(always)]
    pub fn is_refsel_0(&self) -> bool {
        *self == REFSEL_A::REFSEL_0
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects VREFH/VREFL as reference voltage."]
    #[inline(always)]
    pub fn refsel_0(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Conversion Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTRG_A {
    #[doc = "0: Software trigger selected"]
    ADTRG_0 = 0,
    #[doc = "1: Hardware trigger selected"]
    ADTRG_1 = 1,
}
impl From<ADTRG_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADTRG`"]
pub type ADTRG_R = crate::R<bool, ADTRG_A>;
impl ADTRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRG_A {
        match self.bits {
            false => ADTRG_A::ADTRG_0,
            true => ADTRG_A::ADTRG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADTRG_0`"]
    #[inline(always)]
    pub fn is_adtrg_0(&self) -> bool {
        *self == ADTRG_A::ADTRG_0
    }
    #[doc = "Checks if the value of the field is `ADTRG_1`"]
    #[inline(always)]
    pub fn is_adtrg_1(&self) -> bool {
        *self == ADTRG_A::ADTRG_1
    }
}
#[doc = "Write proxy for field `ADTRG`"]
pub struct ADTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADTRG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software trigger selected"]
    #[inline(always)]
    pub fn adtrg_0(self) -> &'a mut W {
        self.variant(ADTRG_A::ADTRG_0)
    }
    #[doc = "Hardware trigger selected"]
    #[inline(always)]
    pub fn adtrg_1(self) -> &'a mut W {
        self.variant(ADTRG_A::ADTRG_1)
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
#[doc = "Hardware Average select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AVGS_A {
    #[doc = "0: 4 samples averaged"]
    AVGS_0 = 0,
    #[doc = "1: 8 samples averaged"]
    AVGS_1 = 1,
    #[doc = "2: 16 samples averaged"]
    AVGS_2 = 2,
    #[doc = "3: 32 samples averaged"]
    AVGS_3 = 3,
}
impl From<AVGS_A> for u8 {
    #[inline(always)]
    fn from(variant: AVGS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AVGS`"]
pub type AVGS_R = crate::R<u8, AVGS_A>;
impl AVGS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGS_A {
        match self.bits {
            0 => AVGS_A::AVGS_0,
            1 => AVGS_A::AVGS_1,
            2 => AVGS_A::AVGS_2,
            3 => AVGS_A::AVGS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVGS_0`"]
    #[inline(always)]
    pub fn is_avgs_0(&self) -> bool {
        *self == AVGS_A::AVGS_0
    }
    #[doc = "Checks if the value of the field is `AVGS_1`"]
    #[inline(always)]
    pub fn is_avgs_1(&self) -> bool {
        *self == AVGS_A::AVGS_1
    }
    #[doc = "Checks if the value of the field is `AVGS_2`"]
    #[inline(always)]
    pub fn is_avgs_2(&self) -> bool {
        *self == AVGS_A::AVGS_2
    }
    #[doc = "Checks if the value of the field is `AVGS_3`"]
    #[inline(always)]
    pub fn is_avgs_3(&self) -> bool {
        *self == AVGS_A::AVGS_3
    }
}
#[doc = "Write proxy for field `AVGS`"]
pub struct AVGS_W<'a> {
    w: &'a mut W,
}
impl<'a> AVGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVGS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 samples averaged"]
    #[inline(always)]
    pub fn avgs_0(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_0)
    }
    #[doc = "8 samples averaged"]
    #[inline(always)]
    pub fn avgs_1(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_1)
    }
    #[doc = "16 samples averaged"]
    #[inline(always)]
    pub fn avgs_2(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_2)
    }
    #[doc = "32 samples averaged"]
    #[inline(always)]
    pub fn avgs_3(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Data Overwrite Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVWREN_A {
    #[doc = "0: Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
    OVWREN_0 = 0,
    #[doc = "1: Enable the overwriting."]
    OVWREN_1 = 1,
}
impl From<OVWREN_A> for bool {
    #[inline(always)]
    fn from(variant: OVWREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVWREN`"]
pub type OVWREN_R = crate::R<bool, OVWREN_A>;
impl OVWREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVWREN_A {
        match self.bits {
            false => OVWREN_A::OVWREN_0,
            true => OVWREN_A::OVWREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVWREN_0`"]
    #[inline(always)]
    pub fn is_ovwren_0(&self) -> bool {
        *self == OVWREN_A::OVWREN_0
    }
    #[doc = "Checks if the value of the field is `OVWREN_1`"]
    #[inline(always)]
    pub fn is_ovwren_1(&self) -> bool {
        *self == OVWREN_A::OVWREN_1
    }
}
#[doc = "Write proxy for field `OVWREN`"]
pub struct OVWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVWREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVWREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
    #[inline(always)]
    pub fn ovwren_0(self) -> &'a mut W {
        self.variant(OVWREN_A::OVWREN_0)
    }
    #[doc = "Enable the overwriting."]
    #[inline(always)]
    pub fn ovwren_1(self) -> &'a mut W {
        self.variant(OVWREN_A::OVWREN_1)
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
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&self) -> ADICLK_R {
        ADICLK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Conversion Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Long Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&self) -> ADLSMP_R {
        ADLSMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&self) -> ADIV_R {
        ADIV_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&self) -> ADLPC_R {
        ADLPC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Defines the sample time duration"]
    #[inline(always)]
    pub fn adsts(&self) -> ADSTS_R {
        ADSTS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - High Speed Configuration"]
    #[inline(always)]
    pub fn adhsc(&self) -> ADHSC_R {
        ADHSC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&self) -> ADTRG_R {
        ADTRG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Hardware Average select"]
    #[inline(always)]
    pub fn avgs(&self) -> AVGS_R {
        AVGS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Data Overwrite Enable"]
    #[inline(always)]
    pub fn ovwren(&self) -> OVWREN_R {
        OVWREN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&mut self) -> ADICLK_W {
        ADICLK_W { w: self }
    }
    #[doc = "Bits 2:3 - Conversion Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - Long Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&mut self) -> ADLSMP_W {
        ADLSMP_W { w: self }
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&mut self) -> ADIV_W {
        ADIV_W { w: self }
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&mut self) -> ADLPC_W {
        ADLPC_W { w: self }
    }
    #[doc = "Bits 8:9 - Defines the sample time duration"]
    #[inline(always)]
    pub fn adsts(&mut self) -> ADSTS_W {
        ADSTS_W { w: self }
    }
    #[doc = "Bit 10 - High Speed Configuration"]
    #[inline(always)]
    pub fn adhsc(&mut self) -> ADHSC_W {
        ADHSC_W { w: self }
    }
    #[doc = "Bits 11:12 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 13 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&mut self) -> ADTRG_W {
        ADTRG_W { w: self }
    }
    #[doc = "Bits 14:15 - Hardware Average select"]
    #[inline(always)]
    pub fn avgs(&mut self) -> AVGS_W {
        AVGS_W { w: self }
    }
    #[doc = "Bit 16 - Data Overwrite Enable"]
    #[inline(always)]
    pub fn ovwren(&mut self) -> OVWREN_W {
        OVWREN_W { w: self }
    }
}
