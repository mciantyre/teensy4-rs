#[doc = "Reader of register SMINTEN"]
pub type R = crate::R<u16, super::SMINTEN>;
#[doc = "Writer for register SMINTEN"]
pub type W = crate::W<u16, super::SMINTEN>;
#[doc = "Register SMINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SMINTEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Compare Interrupt Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPIE_A {
    #[doc = "0: The corresponding STS\\[CMPF\\]
bit will not cause an interrupt request."]
    CMPIE_0 = 0,
    #[doc = "1: The corresponding STS\\[CMPF\\]
bit will cause an interrupt request."]
    CMPIE_1 = 1,
}
impl From<CMPIE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPIE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMPIE`"]
pub type CMPIE_R = crate::R<u8, CMPIE_A>;
impl CMPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMPIE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMPIE_A::CMPIE_0),
            1 => Val(CMPIE_A::CMPIE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMPIE_0`"]
    #[inline(always)]
    pub fn is_cmpie_0(&self) -> bool {
        *self == CMPIE_A::CMPIE_0
    }
    #[doc = "Checks if the value of the field is `CMPIE_1`"]
    #[inline(always)]
    pub fn is_cmpie_1(&self) -> bool {
        *self == CMPIE_A::CMPIE_1
    }
}
#[doc = "Write proxy for field `CMPIE`"]
pub struct CMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding STS\\[CMPF\\]
bit will not cause an interrupt request."]
    #[inline(always)]
    pub fn cmpie_0(self) -> &'a mut W {
        self.variant(CMPIE_A::CMPIE_0)
    }
    #[doc = "The corresponding STS\\[CMPF\\]
bit will cause an interrupt request."]
    #[inline(always)]
    pub fn cmpie_1(self) -> &'a mut W {
        self.variant(CMPIE_A::CMPIE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Capture X 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CX0IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFX0\\]."]
    CX0IE_0 = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFX0\\]."]
    CX0IE_1 = 1,
}
impl From<CX0IE_A> for bool {
    #[inline(always)]
    fn from(variant: CX0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CX0IE`"]
pub type CX0IE_R = crate::R<bool, CX0IE_A>;
impl CX0IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CX0IE_A {
        match self.bits {
            false => CX0IE_A::CX0IE_0,
            true => CX0IE_A::CX0IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CX0IE_0`"]
    #[inline(always)]
    pub fn is_cx0ie_0(&self) -> bool {
        *self == CX0IE_A::CX0IE_0
    }
    #[doc = "Checks if the value of the field is `CX0IE_1`"]
    #[inline(always)]
    pub fn is_cx0ie_1(&self) -> bool {
        *self == CX0IE_A::CX0IE_1
    }
}
#[doc = "Write proxy for field `CX0IE`"]
pub struct CX0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CX0IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CX0IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request disabled for STS\\[CFX0\\]."]
    #[inline(always)]
    pub fn cx0ie_0(self) -> &'a mut W {
        self.variant(CX0IE_A::CX0IE_0)
    }
    #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
    #[inline(always)]
    pub fn cx0ie_1(self) -> &'a mut W {
        self.variant(CX0IE_A::CX0IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Capture X 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CX1IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFX1\\]."]
    CX1IE_0 = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFX1\\]."]
    CX1IE_1 = 1,
}
impl From<CX1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CX1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CX1IE`"]
pub type CX1IE_R = crate::R<bool, CX1IE_A>;
impl CX1IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CX1IE_A {
        match self.bits {
            false => CX1IE_A::CX1IE_0,
            true => CX1IE_A::CX1IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CX1IE_0`"]
    #[inline(always)]
    pub fn is_cx1ie_0(&self) -> bool {
        *self == CX1IE_A::CX1IE_0
    }
    #[doc = "Checks if the value of the field is `CX1IE_1`"]
    #[inline(always)]
    pub fn is_cx1ie_1(&self) -> bool {
        *self == CX1IE_A::CX1IE_1
    }
}
#[doc = "Write proxy for field `CX1IE`"]
pub struct CX1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CX1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CX1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request disabled for STS\\[CFX1\\]."]
    #[inline(always)]
    pub fn cx1ie_0(self) -> &'a mut W {
        self.variant(CX1IE_A::CX1IE_0)
    }
    #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
    #[inline(always)]
    pub fn cx1ie_1(self) -> &'a mut W {
        self.variant(CX1IE_A::CX1IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Capture B 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CB0IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFB0\\]."]
    CB0IE_0 = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFB0\\]."]
    CB0IE_1 = 1,
}
impl From<CB0IE_A> for bool {
    #[inline(always)]
    fn from(variant: CB0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CB0IE`"]
pub type CB0IE_R = crate::R<bool, CB0IE_A>;
impl CB0IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CB0IE_A {
        match self.bits {
            false => CB0IE_A::CB0IE_0,
            true => CB0IE_A::CB0IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CB0IE_0`"]
    #[inline(always)]
    pub fn is_cb0ie_0(&self) -> bool {
        *self == CB0IE_A::CB0IE_0
    }
    #[doc = "Checks if the value of the field is `CB0IE_1`"]
    #[inline(always)]
    pub fn is_cb0ie_1(&self) -> bool {
        *self == CB0IE_A::CB0IE_1
    }
}
#[doc = "Write proxy for field `CB0IE`"]
pub struct CB0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CB0IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CB0IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request disabled for STS\\[CFB0\\]."]
    #[inline(always)]
    pub fn cb0ie_0(self) -> &'a mut W {
        self.variant(CB0IE_A::CB0IE_0)
    }
    #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
    #[inline(always)]
    pub fn cb0ie_1(self) -> &'a mut W {
        self.variant(CB0IE_A::CB0IE_1)
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
#[doc = "Capture B 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CB1IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFB1\\]."]
    CB1IE_0 = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFB1\\]."]
    CB1IE_1 = 1,
}
impl From<CB1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CB1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CB1IE`"]
pub type CB1IE_R = crate::R<bool, CB1IE_A>;
impl CB1IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CB1IE_A {
        match self.bits {
            false => CB1IE_A::CB1IE_0,
            true => CB1IE_A::CB1IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CB1IE_0`"]
    #[inline(always)]
    pub fn is_cb1ie_0(&self) -> bool {
        *self == CB1IE_A::CB1IE_0
    }
    #[doc = "Checks if the value of the field is `CB1IE_1`"]
    #[inline(always)]
    pub fn is_cb1ie_1(&self) -> bool {
        *self == CB1IE_A::CB1IE_1
    }
}
#[doc = "Write proxy for field `CB1IE`"]
pub struct CB1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CB1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CB1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request disabled for STS\\[CFB1\\]."]
    #[inline(always)]
    pub fn cb1ie_0(self) -> &'a mut W {
        self.variant(CB1IE_A::CB1IE_0)
    }
    #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
    #[inline(always)]
    pub fn cb1ie_1(self) -> &'a mut W {
        self.variant(CB1IE_A::CB1IE_1)
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
#[doc = "Capture A 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CA0IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFA0\\]."]
    CA0IE_0 = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFA0\\]."]
    CA0IE_1 = 1,
}
impl From<CA0IE_A> for bool {
    #[inline(always)]
    fn from(variant: CA0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CA0IE`"]
pub type CA0IE_R = crate::R<bool, CA0IE_A>;
impl CA0IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CA0IE_A {
        match self.bits {
            false => CA0IE_A::CA0IE_0,
            true => CA0IE_A::CA0IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CA0IE_0`"]
    #[inline(always)]
    pub fn is_ca0ie_0(&self) -> bool {
        *self == CA0IE_A::CA0IE_0
    }
    #[doc = "Checks if the value of the field is `CA0IE_1`"]
    #[inline(always)]
    pub fn is_ca0ie_1(&self) -> bool {
        *self == CA0IE_A::CA0IE_1
    }
}
#[doc = "Write proxy for field `CA0IE`"]
pub struct CA0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CA0IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CA0IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request disabled for STS\\[CFA0\\]."]
    #[inline(always)]
    pub fn ca0ie_0(self) -> &'a mut W {
        self.variant(CA0IE_A::CA0IE_0)
    }
    #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
    #[inline(always)]
    pub fn ca0ie_1(self) -> &'a mut W {
        self.variant(CA0IE_A::CA0IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Capture A 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CA1IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFA1\\]."]
    CA1IE_0 = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFA1\\]."]
    CA1IE_1 = 1,
}
impl From<CA1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CA1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CA1IE`"]
pub type CA1IE_R = crate::R<bool, CA1IE_A>;
impl CA1IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CA1IE_A {
        match self.bits {
            false => CA1IE_A::CA1IE_0,
            true => CA1IE_A::CA1IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CA1IE_0`"]
    #[inline(always)]
    pub fn is_ca1ie_0(&self) -> bool {
        *self == CA1IE_A::CA1IE_0
    }
    #[doc = "Checks if the value of the field is `CA1IE_1`"]
    #[inline(always)]
    pub fn is_ca1ie_1(&self) -> bool {
        *self == CA1IE_A::CA1IE_1
    }
}
#[doc = "Write proxy for field `CA1IE`"]
pub struct CA1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CA1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CA1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt request disabled for STS\\[CFA1\\]."]
    #[inline(always)]
    pub fn ca1ie_0(self) -> &'a mut W {
        self.variant(CA1IE_A::CA1IE_0)
    }
    #[doc = "Interrupt request enabled for STS\\[CFA1\\]."]
    #[inline(always)]
    pub fn ca1ie_1(self) -> &'a mut W {
        self.variant(CA1IE_A::CA1IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reload Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIE_A {
    #[doc = "0: STS\\[RF\\]
CPU interrupt requests disabled"]
    RIE_0 = 0,
    #[doc = "1: STS\\[RF\\]
CPU interrupt requests enabled"]
    RIE_1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RIE`"]
pub type RIE_R = crate::R<bool, RIE_A>;
impl RIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::RIE_0,
            true => RIE_A::RIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RIE_0`"]
    #[inline(always)]
    pub fn is_rie_0(&self) -> bool {
        *self == RIE_A::RIE_0
    }
    #[doc = "Checks if the value of the field is `RIE_1`"]
    #[inline(always)]
    pub fn is_rie_1(&self) -> bool {
        *self == RIE_A::RIE_1
    }
}
#[doc = "Write proxy for field `RIE`"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "STS\\[RF\\]
CPU interrupt requests disabled"]
    #[inline(always)]
    pub fn rie_0(self) -> &'a mut W {
        self.variant(RIE_A::RIE_0)
    }
    #[doc = "STS\\[RF\\]
CPU interrupt requests enabled"]
    #[inline(always)]
    pub fn rie_1(self) -> &'a mut W {
        self.variant(RIE_A::RIE_1)
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
#[doc = "Reload Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIE_A {
    #[doc = "0: STS\\[REF\\]
CPU interrupt requests disabled"]
    REIE_0 = 0,
    #[doc = "1: STS\\[REF\\]
CPU interrupt requests enabled"]
    REIE_1 = 1,
}
impl From<REIE_A> for bool {
    #[inline(always)]
    fn from(variant: REIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REIE`"]
pub type REIE_R = crate::R<bool, REIE_A>;
impl REIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REIE_A {
        match self.bits {
            false => REIE_A::REIE_0,
            true => REIE_A::REIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `REIE_0`"]
    #[inline(always)]
    pub fn is_reie_0(&self) -> bool {
        *self == REIE_A::REIE_0
    }
    #[doc = "Checks if the value of the field is `REIE_1`"]
    #[inline(always)]
    pub fn is_reie_1(&self) -> bool {
        *self == REIE_A::REIE_1
    }
}
#[doc = "Write proxy for field `REIE`"]
pub struct REIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "STS\\[REF\\]
CPU interrupt requests disabled"]
    #[inline(always)]
    pub fn reie_0(self) -> &'a mut W {
        self.variant(REIE_A::REIE_0)
    }
    #[doc = "STS\\[REF\\]
CPU interrupt requests enabled"]
    #[inline(always)]
    pub fn reie_1(self) -> &'a mut W {
        self.variant(REIE_A::REIE_1)
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
impl R {
    #[doc = "Bits 0:5 - Compare Interrupt Enables"]
    #[inline(always)]
    pub fn cmpie(&self) -> CMPIE_R {
        CMPIE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cx0ie(&self) -> CX0IE_R {
        CX0IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cx1ie(&self) -> CX1IE_R {
        CX1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cb0ie(&self) -> CB0IE_R {
        CB0IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cb1ie(&self) -> CB1IE_R {
        CB1IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ca0ie(&self) -> CA0IE_R {
        CA0IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ca1ie(&self) -> CA1IE_R {
        CA1IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reload Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reload Error Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Compare Interrupt Enables"]
    #[inline(always)]
    pub fn cmpie(&mut self) -> CMPIE_W {
        CMPIE_W { w: self }
    }
    #[doc = "Bit 6 - Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cx0ie(&mut self) -> CX0IE_W {
        CX0IE_W { w: self }
    }
    #[doc = "Bit 7 - Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cx1ie(&mut self) -> CX1IE_W {
        CX1IE_W { w: self }
    }
    #[doc = "Bit 8 - Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cb0ie(&mut self) -> CB0IE_W {
        CB0IE_W { w: self }
    }
    #[doc = "Bit 9 - Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cb1ie(&mut self) -> CB1IE_W {
        CB1IE_W { w: self }
    }
    #[doc = "Bit 10 - Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ca0ie(&mut self) -> CA0IE_W {
        CA0IE_W { w: self }
    }
    #[doc = "Bit 11 - Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ca1ie(&mut self) -> CA1IE_W {
        CA1IE_W { w: self }
    }
    #[doc = "Bit 12 - Reload Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 13 - Reload Error Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&mut self) -> REIE_W {
        REIE_W { w: self }
    }
}
