#[doc = "Reader of register SM0STS"]
pub type R = crate::R<u16, super::SM0STS>;
#[doc = "Writer for register SM0STS"]
pub type W = crate::W<u16, super::SM0STS>;
#[doc = "Register SM0STS `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0STS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Compare Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPF_A {
    #[doc = "0: No compare event has occurred for a particular VALx value."]
    CMPF_0,
    #[doc = "1: A compare event has occurred for a particular VALx value."]
    CMPF_1,
}
impl From<CMPF_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPF_A) -> Self {
        match variant {
            CMPF_A::CMPF_0 => 0,
            CMPF_A::CMPF_1 => 1,
        }
    }
}
#[doc = "Reader of field `CMPF`"]
pub type CMPF_R = crate::R<u8, CMPF_A>;
impl CMPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMPF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMPF_A::CMPF_0),
            1 => Val(CMPF_A::CMPF_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMPF_0`"]
    #[inline(always)]
    pub fn is_cmpf_0(&self) -> bool {
        *self == CMPF_A::CMPF_0
    }
    #[doc = "Checks if the value of the field is `CMPF_1`"]
    #[inline(always)]
    pub fn is_cmpf_1(&self) -> bool {
        *self == CMPF_A::CMPF_1
    }
}
#[doc = "Write proxy for field `CMPF`"]
pub struct CMPF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No compare event has occurred for a particular VALx value."]
    #[inline(always)]
    pub fn cmpf_0(self) -> &'a mut W {
        self.variant(CMPF_A::CMPF_0)
    }
    #[doc = "A compare event has occurred for a particular VALx value."]
    #[inline(always)]
    pub fn cmpf_1(self) -> &'a mut W {
        self.variant(CMPF_A::CMPF_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CFX0`"]
pub type CFX0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFX0`"]
pub struct CFX0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFX0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CFX1`"]
pub type CFX1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFX1`"]
pub struct CFX1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFX1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CFB0`"]
pub type CFB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFB0`"]
pub struct CFB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFB0_W<'a> {
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
#[doc = "Reader of field `CFB1`"]
pub type CFB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFB1`"]
pub struct CFB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFB1_W<'a> {
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
#[doc = "Reader of field `CFA0`"]
pub type CFA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFA0`"]
pub struct CFA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFA0_W<'a> {
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
#[doc = "Reader of field `CFA1`"]
pub type CFA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFA1`"]
pub struct CFA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFA1_W<'a> {
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
#[doc = "Reload Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_A {
    #[doc = "0: No new reload cycle since last STS\\[RF\\] clearing"]
    RF_0,
    #[doc = "1: New reload cycle since last STS\\[RF\\] clearing"]
    RF_1,
}
impl From<RF_A> for bool {
    #[inline(always)]
    fn from(variant: RF_A) -> Self {
        match variant {
            RF_A::RF_0 => false,
            RF_A::RF_1 => true,
        }
    }
}
#[doc = "Reader of field `RF`"]
pub type RF_R = crate::R<bool, RF_A>;
impl RF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_A {
        match self.bits {
            false => RF_A::RF_0,
            true => RF_A::RF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RF_0`"]
    #[inline(always)]
    pub fn is_rf_0(&self) -> bool {
        *self == RF_A::RF_0
    }
    #[doc = "Checks if the value of the field is `RF_1`"]
    #[inline(always)]
    pub fn is_rf_1(&self) -> bool {
        *self == RF_A::RF_1
    }
}
#[doc = "Write proxy for field `RF`"]
pub struct RF_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No new reload cycle since last STS\\[RF\\] clearing"]
    #[inline(always)]
    pub fn rf_0(self) -> &'a mut W {
        self.variant(RF_A::RF_0)
    }
    #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
    #[inline(always)]
    pub fn rf_1(self) -> &'a mut W {
        self.variant(RF_A::RF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reload Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_A {
    #[doc = "0: No reload error occurred."]
    REF_0,
    #[doc = "1: Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
    REF_1,
}
impl From<REF_A> for bool {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        match variant {
            REF_A::REF_0 => false,
            REF_A::REF_1 => true,
        }
    }
}
#[doc = "Reader of field `REF`"]
pub type REF_R = crate::R<bool, REF_A>;
impl REF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_A {
        match self.bits {
            false => REF_A::REF_0,
            true => REF_A::REF_1,
        }
    }
    #[doc = "Checks if the value of the field is `REF_0`"]
    #[inline(always)]
    pub fn is_ref_0(&self) -> bool {
        *self == REF_A::REF_0
    }
    #[doc = "Checks if the value of the field is `REF_1`"]
    #[inline(always)]
    pub fn is_ref_1(&self) -> bool {
        *self == REF_A::REF_1
    }
}
#[doc = "Write proxy for field `REF`"]
pub struct REF_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reload error occurred."]
    #[inline(always)]
    pub fn ref_0(self) -> &'a mut W {
        self.variant(REF_A::REF_0)
    }
    #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
    #[inline(always)]
    pub fn ref_1(self) -> &'a mut W {
        self.variant(REF_A::REF_1)
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
#[doc = "Registers Updated Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUF_A {
    #[doc = "0: No register update has occurred since last reload."]
    RUF_0,
    #[doc = "1: At least one of the double buffered registers has been updated since the last reload."]
    RUF_1,
}
impl From<RUF_A> for bool {
    #[inline(always)]
    fn from(variant: RUF_A) -> Self {
        match variant {
            RUF_A::RUF_0 => false,
            RUF_A::RUF_1 => true,
        }
    }
}
#[doc = "Reader of field `RUF`"]
pub type RUF_R = crate::R<bool, RUF_A>;
impl RUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUF_A {
        match self.bits {
            false => RUF_A::RUF_0,
            true => RUF_A::RUF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RUF_0`"]
    #[inline(always)]
    pub fn is_ruf_0(&self) -> bool {
        *self == RUF_A::RUF_0
    }
    #[doc = "Checks if the value of the field is `RUF_1`"]
    #[inline(always)]
    pub fn is_ruf_1(&self) -> bool {
        *self == RUF_A::RUF_1
    }
}
impl R {
    #[doc = "Bits 0:5 - Compare Flags"]
    #[inline(always)]
    pub fn cmpf(&self) -> CMPF_R {
        CMPF_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Capture Flag X0"]
    #[inline(always)]
    pub fn cfx0(&self) -> CFX0_R {
        CFX0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Capture Flag X1"]
    #[inline(always)]
    pub fn cfx1(&self) -> CFX1_R {
        CFX1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture Flag B0"]
    #[inline(always)]
    pub fn cfb0(&self) -> CFB0_R {
        CFB0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture Flag B1"]
    #[inline(always)]
    pub fn cfb1(&self) -> CFB1_R {
        CFB1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture Flag A0"]
    #[inline(always)]
    pub fn cfa0(&self) -> CFA0_R {
        CFA0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Flag A1"]
    #[inline(always)]
    pub fn cfa1(&self) -> CFA1_R {
        CFA1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reload Flag"]
    #[inline(always)]
    pub fn rf(&self) -> RF_R {
        RF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reload Error Flag"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Registers Updated Flag"]
    #[inline(always)]
    pub fn ruf(&self) -> RUF_R {
        RUF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Compare Flags"]
    #[inline(always)]
    pub fn cmpf(&mut self) -> CMPF_W {
        CMPF_W { w: self }
    }
    #[doc = "Bit 6 - Capture Flag X0"]
    #[inline(always)]
    pub fn cfx0(&mut self) -> CFX0_W {
        CFX0_W { w: self }
    }
    #[doc = "Bit 7 - Capture Flag X1"]
    #[inline(always)]
    pub fn cfx1(&mut self) -> CFX1_W {
        CFX1_W { w: self }
    }
    #[doc = "Bit 8 - Capture Flag B0"]
    #[inline(always)]
    pub fn cfb0(&mut self) -> CFB0_W {
        CFB0_W { w: self }
    }
    #[doc = "Bit 9 - Capture Flag B1"]
    #[inline(always)]
    pub fn cfb1(&mut self) -> CFB1_W {
        CFB1_W { w: self }
    }
    #[doc = "Bit 10 - Capture Flag A0"]
    #[inline(always)]
    pub fn cfa0(&mut self) -> CFA0_W {
        CFA0_W { w: self }
    }
    #[doc = "Bit 11 - Capture Flag A1"]
    #[inline(always)]
    pub fn cfa1(&mut self) -> CFA1_W {
        CFA1_W { w: self }
    }
    #[doc = "Bit 12 - Reload Flag"]
    #[inline(always)]
    pub fn rf(&mut self) -> RF_W {
        RF_W { w: self }
    }
    #[doc = "Bit 13 - Reload Error Flag"]
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W {
        REF_W { w: self }
    }
}
