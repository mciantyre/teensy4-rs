#[doc = "Reader of register TST"]
pub type R = crate::R<u16, super::TST>;
#[doc = "Writer for register TST"]
pub type W = crate::W<u16, super::TST>;
#[doc = "Register TST `reset()`'s with value 0"]
impl crate::ResetValue for super::TST {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEST_COUNT`"]
pub type TEST_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEST_COUNT`"]
pub struct TEST_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TEST_PERIOD`"]
pub type TEST_PERIOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEST_PERIOD`"]
pub struct TEST_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Quadrature Decoder Negative Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QDN_A {
    #[doc = "0: Leaves quadrature decoder signal in a positive direction"]
    QDN_0 = 0,
    #[doc = "1: Generates a negative quadrature decoder signal"]
    QDN_1 = 1,
}
impl From<QDN_A> for bool {
    #[inline(always)]
    fn from(variant: QDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QDN`"]
pub type QDN_R = crate::R<bool, QDN_A>;
impl QDN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QDN_A {
        match self.bits {
            false => QDN_A::QDN_0,
            true => QDN_A::QDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `QDN_0`"]
    #[inline(always)]
    pub fn is_qdn_0(&self) -> bool {
        *self == QDN_A::QDN_0
    }
    #[doc = "Checks if the value of the field is `QDN_1`"]
    #[inline(always)]
    pub fn is_qdn_1(&self) -> bool {
        *self == QDN_A::QDN_1
    }
}
#[doc = "Write proxy for field `QDN`"]
pub struct QDN_W<'a> {
    w: &'a mut W,
}
impl<'a> QDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QDN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Leaves quadrature decoder signal in a positive direction"]
    #[inline(always)]
    pub fn qdn_0(self) -> &'a mut W {
        self.variant(QDN_A::QDN_0)
    }
    #[doc = "Generates a negative quadrature decoder signal"]
    #[inline(always)]
    pub fn qdn_1(self) -> &'a mut W {
        self.variant(QDN_A::QDN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Test Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE_A {
    #[doc = "0: Test count is not enabled"]
    TCE_0 = 0,
    #[doc = "1: Test count is enabled"]
    TCE_1 = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCE`"]
pub type TCE_R = crate::R<bool, TCE_A>;
impl TCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::TCE_0,
            true => TCE_A::TCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCE_0`"]
    #[inline(always)]
    pub fn is_tce_0(&self) -> bool {
        *self == TCE_A::TCE_0
    }
    #[doc = "Checks if the value of the field is `TCE_1`"]
    #[inline(always)]
    pub fn is_tce_1(&self) -> bool {
        *self == TCE_A::TCE_1
    }
}
#[doc = "Write proxy for field `TCE`"]
pub struct TCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Test count is not enabled"]
    #[inline(always)]
    pub fn tce_0(self) -> &'a mut W {
        self.variant(TCE_A::TCE_0)
    }
    #[doc = "Test count is enabled"]
    #[inline(always)]
    pub fn tce_1(self) -> &'a mut W {
        self.variant(TCE_A::TCE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
    #[doc = "0: Test module is not enabled"]
    TEN_0 = 0,
    #[doc = "1: Test module is enabled"]
    TEN_1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEN`"]
pub type TEN_R = crate::R<bool, TEN_A>;
impl TEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::TEN_0,
            true => TEN_A::TEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEN_0`"]
    #[inline(always)]
    pub fn is_ten_0(&self) -> bool {
        *self == TEN_A::TEN_0
    }
    #[doc = "Checks if the value of the field is `TEN_1`"]
    #[inline(always)]
    pub fn is_ten_1(&self) -> bool {
        *self == TEN_A::TEN_1
    }
}
#[doc = "Write proxy for field `TEN`"]
pub struct TEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Test module is not enabled"]
    #[inline(always)]
    pub fn ten_0(self) -> &'a mut W {
        self.variant(TEN_A::TEN_0)
    }
    #[doc = "Test module is enabled"]
    #[inline(always)]
    pub fn ten_1(self) -> &'a mut W {
        self.variant(TEN_A::TEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits hold the number of quadrature advances to generate."]
    #[inline(always)]
    pub fn test_count(&self) -> TEST_COUNT_R {
        TEST_COUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - These bits hold the period of quadrature phase in IPBus clock cycles."]
    #[inline(always)]
    pub fn test_period(&self) -> TEST_PERIOD_R {
        TEST_PERIOD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Quadrature Decoder Negative Signal"]
    #[inline(always)]
    pub fn qdn(&self) -> QDN_R {
        QDN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Test Counter Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Test Mode Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits hold the number of quadrature advances to generate."]
    #[inline(always)]
    pub fn test_count(&mut self) -> TEST_COUNT_W {
        TEST_COUNT_W { w: self }
    }
    #[doc = "Bits 8:12 - These bits hold the period of quadrature phase in IPBus clock cycles."]
    #[inline(always)]
    pub fn test_period(&mut self) -> TEST_PERIOD_W {
        TEST_PERIOD_W { w: self }
    }
    #[doc = "Bit 13 - Quadrature Decoder Negative Signal"]
    #[inline(always)]
    pub fn qdn(&mut self) -> QDN_W {
        QDN_W { w: self }
    }
    #[doc = "Bit 14 - Test Counter Enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W {
        TCE_W { w: self }
    }
    #[doc = "Bit 15 - Test Mode Enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W { w: self }
    }
}
