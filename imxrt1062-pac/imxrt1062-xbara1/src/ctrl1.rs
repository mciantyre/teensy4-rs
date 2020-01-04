#[doc = "Reader of register CTRL1"]
pub type R = crate::R<u16, super::CTRL1>;
#[doc = "Writer for register CTRL1"]
pub type W = crate::W<u16, super::CTRL1>;
#[doc = "Register CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Enable for XBAR_OUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN2_A {
    #[doc = "0: DMA disabled"]
    DEN2_0 = 0,
    #[doc = "1: DMA enabled"]
    DEN2_1 = 1,
}
impl From<DEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEN2`"]
pub type DEN2_R = crate::R<bool, DEN2_A>;
impl DEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN2_A {
        match self.bits {
            false => DEN2_A::DEN2_0,
            true => DEN2_A::DEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN2_0`"]
    #[inline(always)]
    pub fn is_den2_0(&self) -> bool {
        *self == DEN2_A::DEN2_0
    }
    #[doc = "Checks if the value of the field is `DEN2_1`"]
    #[inline(always)]
    pub fn is_den2_1(&self) -> bool {
        *self == DEN2_A::DEN2_1
    }
}
#[doc = "Write proxy for field `DEN2`"]
pub struct DEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn den2_0(self) -> &'a mut W {
        self.variant(DEN2_A::DEN2_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn den2_1(self) -> &'a mut W {
        self.variant(DEN2_A::DEN2_1)
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
#[doc = "Interrupt Enable for XBAR_OUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN2_A {
    #[doc = "0: Interrupt disabled"]
    IEN2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    IEN2_1 = 1,
}
impl From<IEN2_A> for bool {
    #[inline(always)]
    fn from(variant: IEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IEN2`"]
pub type IEN2_R = crate::R<bool, IEN2_A>;
impl IEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN2_A {
        match self.bits {
            false => IEN2_A::IEN2_0,
            true => IEN2_A::IEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN2_0`"]
    #[inline(always)]
    pub fn is_ien2_0(&self) -> bool {
        *self == IEN2_A::IEN2_0
    }
    #[doc = "Checks if the value of the field is `IEN2_1`"]
    #[inline(always)]
    pub fn is_ien2_1(&self) -> bool {
        *self == IEN2_A::IEN2_1
    }
}
#[doc = "Write proxy for field `IEN2`"]
pub struct IEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ien2_0(self) -> &'a mut W {
        self.variant(IEN2_A::IEN2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ien2_1(self) -> &'a mut W {
        self.variant(IEN2_A::IEN2_1)
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
#[doc = "Active edge for edge detection on XBAR_OUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGE2_A {
    #[doc = "0: STS2 never asserts"]
    EDGE2_0 = 0,
    #[doc = "1: STS2 asserts on rising edges of XBAR_OUT2"]
    EDGE2_1 = 1,
    #[doc = "2: STS2 asserts on falling edges of XBAR_OUT2"]
    EDGE2_2 = 2,
    #[doc = "3: STS2 asserts on rising and falling edges of XBAR_OUT2"]
    EDGE2_3 = 3,
}
impl From<EDGE2_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGE2`"]
pub type EDGE2_R = crate::R<u8, EDGE2_A>;
impl EDGE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE2_A {
        match self.bits {
            0 => EDGE2_A::EDGE2_0,
            1 => EDGE2_A::EDGE2_1,
            2 => EDGE2_A::EDGE2_2,
            3 => EDGE2_A::EDGE2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE2_0`"]
    #[inline(always)]
    pub fn is_edge2_0(&self) -> bool {
        *self == EDGE2_A::EDGE2_0
    }
    #[doc = "Checks if the value of the field is `EDGE2_1`"]
    #[inline(always)]
    pub fn is_edge2_1(&self) -> bool {
        *self == EDGE2_A::EDGE2_1
    }
    #[doc = "Checks if the value of the field is `EDGE2_2`"]
    #[inline(always)]
    pub fn is_edge2_2(&self) -> bool {
        *self == EDGE2_A::EDGE2_2
    }
    #[doc = "Checks if the value of the field is `EDGE2_3`"]
    #[inline(always)]
    pub fn is_edge2_3(&self) -> bool {
        *self == EDGE2_A::EDGE2_3
    }
}
#[doc = "Write proxy for field `EDGE2`"]
pub struct EDGE2_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "STS2 never asserts"]
    #[inline(always)]
    pub fn edge2_0(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE2_0)
    }
    #[doc = "STS2 asserts on rising edges of XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2_1(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE2_1)
    }
    #[doc = "STS2 asserts on falling edges of XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2_2(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE2_2)
    }
    #[doc = "STS2 asserts on rising and falling edges of XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2_3(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE2_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Edge detection status for XBAR_OUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS2_A {
    #[doc = "0: Active edge not yet detected on XBAR_OUT2"]
    STS2_0 = 0,
    #[doc = "1: Active edge detected on XBAR_OUT2"]
    STS2_1 = 1,
}
impl From<STS2_A> for bool {
    #[inline(always)]
    fn from(variant: STS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STS2`"]
pub type STS2_R = crate::R<bool, STS2_A>;
impl STS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS2_A {
        match self.bits {
            false => STS2_A::STS2_0,
            true => STS2_A::STS2_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS2_0`"]
    #[inline(always)]
    pub fn is_sts2_0(&self) -> bool {
        *self == STS2_A::STS2_0
    }
    #[doc = "Checks if the value of the field is `STS2_1`"]
    #[inline(always)]
    pub fn is_sts2_1(&self) -> bool {
        *self == STS2_A::STS2_1
    }
}
#[doc = "Write proxy for field `STS2`"]
pub struct STS2_W<'a> {
    w: &'a mut W,
}
impl<'a> STS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active edge not yet detected on XBAR_OUT2"]
    #[inline(always)]
    pub fn sts2_0(self) -> &'a mut W {
        self.variant(STS2_A::STS2_0)
    }
    #[doc = "Active edge detected on XBAR_OUT2"]
    #[inline(always)]
    pub fn sts2_1(self) -> &'a mut W {
        self.variant(STS2_A::STS2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "DMA Enable for XBAR_OUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN3_A {
    #[doc = "0: DMA disabled"]
    DEN3_0 = 0,
    #[doc = "1: DMA enabled"]
    DEN3_1 = 1,
}
impl From<DEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEN3`"]
pub type DEN3_R = crate::R<bool, DEN3_A>;
impl DEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN3_A {
        match self.bits {
            false => DEN3_A::DEN3_0,
            true => DEN3_A::DEN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN3_0`"]
    #[inline(always)]
    pub fn is_den3_0(&self) -> bool {
        *self == DEN3_A::DEN3_0
    }
    #[doc = "Checks if the value of the field is `DEN3_1`"]
    #[inline(always)]
    pub fn is_den3_1(&self) -> bool {
        *self == DEN3_A::DEN3_1
    }
}
#[doc = "Write proxy for field `DEN3`"]
pub struct DEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn den3_0(self) -> &'a mut W {
        self.variant(DEN3_A::DEN3_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn den3_1(self) -> &'a mut W {
        self.variant(DEN3_A::DEN3_1)
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
#[doc = "Interrupt Enable for XBAR_OUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN3_A {
    #[doc = "0: Interrupt disabled"]
    IEN3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    IEN3_1 = 1,
}
impl From<IEN3_A> for bool {
    #[inline(always)]
    fn from(variant: IEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IEN3`"]
pub type IEN3_R = crate::R<bool, IEN3_A>;
impl IEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN3_A {
        match self.bits {
            false => IEN3_A::IEN3_0,
            true => IEN3_A::IEN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN3_0`"]
    #[inline(always)]
    pub fn is_ien3_0(&self) -> bool {
        *self == IEN3_A::IEN3_0
    }
    #[doc = "Checks if the value of the field is `IEN3_1`"]
    #[inline(always)]
    pub fn is_ien3_1(&self) -> bool {
        *self == IEN3_A::IEN3_1
    }
}
#[doc = "Write proxy for field `IEN3`"]
pub struct IEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ien3_0(self) -> &'a mut W {
        self.variant(IEN3_A::IEN3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ien3_1(self) -> &'a mut W {
        self.variant(IEN3_A::IEN3_1)
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
#[doc = "Active edge for edge detection on XBAR_OUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGE3_A {
    #[doc = "0: STS3 never asserts"]
    EDGE3_0 = 0,
    #[doc = "1: STS3 asserts on rising edges of XBAR_OUT3"]
    EDGE3_1 = 1,
    #[doc = "2: STS3 asserts on falling edges of XBAR_OUT3"]
    EDGE3_2 = 2,
    #[doc = "3: STS3 asserts on rising and falling edges of XBAR_OUT3"]
    EDGE3_3 = 3,
}
impl From<EDGE3_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGE3`"]
pub type EDGE3_R = crate::R<u8, EDGE3_A>;
impl EDGE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE3_A {
        match self.bits {
            0 => EDGE3_A::EDGE3_0,
            1 => EDGE3_A::EDGE3_1,
            2 => EDGE3_A::EDGE3_2,
            3 => EDGE3_A::EDGE3_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE3_0`"]
    #[inline(always)]
    pub fn is_edge3_0(&self) -> bool {
        *self == EDGE3_A::EDGE3_0
    }
    #[doc = "Checks if the value of the field is `EDGE3_1`"]
    #[inline(always)]
    pub fn is_edge3_1(&self) -> bool {
        *self == EDGE3_A::EDGE3_1
    }
    #[doc = "Checks if the value of the field is `EDGE3_2`"]
    #[inline(always)]
    pub fn is_edge3_2(&self) -> bool {
        *self == EDGE3_A::EDGE3_2
    }
    #[doc = "Checks if the value of the field is `EDGE3_3`"]
    #[inline(always)]
    pub fn is_edge3_3(&self) -> bool {
        *self == EDGE3_A::EDGE3_3
    }
}
#[doc = "Write proxy for field `EDGE3`"]
pub struct EDGE3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "STS3 never asserts"]
    #[inline(always)]
    pub fn edge3_0(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE3_0)
    }
    #[doc = "STS3 asserts on rising edges of XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3_1(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE3_1)
    }
    #[doc = "STS3 asserts on falling edges of XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3_2(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE3_2)
    }
    #[doc = "STS3 asserts on rising and falling edges of XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3_3(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE3_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Edge detection status for XBAR_OUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS3_A {
    #[doc = "0: Active edge not yet detected on XBAR_OUT3"]
    STS3_0 = 0,
    #[doc = "1: Active edge detected on XBAR_OUT3"]
    STS3_1 = 1,
}
impl From<STS3_A> for bool {
    #[inline(always)]
    fn from(variant: STS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STS3`"]
pub type STS3_R = crate::R<bool, STS3_A>;
impl STS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS3_A {
        match self.bits {
            false => STS3_A::STS3_0,
            true => STS3_A::STS3_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS3_0`"]
    #[inline(always)]
    pub fn is_sts3_0(&self) -> bool {
        *self == STS3_A::STS3_0
    }
    #[doc = "Checks if the value of the field is `STS3_1`"]
    #[inline(always)]
    pub fn is_sts3_1(&self) -> bool {
        *self == STS3_A::STS3_1
    }
}
#[doc = "Write proxy for field `STS3`"]
pub struct STS3_W<'a> {
    w: &'a mut W,
}
impl<'a> STS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active edge not yet detected on XBAR_OUT3"]
    #[inline(always)]
    pub fn sts3_0(self) -> &'a mut W {
        self.variant(STS3_A::STS3_0)
    }
    #[doc = "Active edge detected on XBAR_OUT3"]
    #[inline(always)]
    pub fn sts3_1(self) -> &'a mut W {
        self.variant(STS3_A::STS3_1)
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
impl R {
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT2"]
    #[inline(always)]
    pub fn den2(&self) -> DEN2_R {
        DEN2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT2"]
    #[inline(always)]
    pub fn ien2(&self) -> IEN2_R {
        IEN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT2"]
    #[inline(always)]
    pub fn sts2(&self) -> STS2_R {
        STS2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT3"]
    #[inline(always)]
    pub fn den3(&self) -> DEN3_R {
        DEN3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT3"]
    #[inline(always)]
    pub fn ien3(&self) -> IEN3_R {
        IEN3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT3"]
    #[inline(always)]
    pub fn sts3(&self) -> STS3_R {
        STS3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT2"]
    #[inline(always)]
    pub fn den2(&mut self) -> DEN2_W {
        DEN2_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT2"]
    #[inline(always)]
    pub fn ien2(&mut self) -> IEN2_W {
        IEN2_W { w: self }
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2(&mut self) -> EDGE2_W {
        EDGE2_W { w: self }
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT2"]
    #[inline(always)]
    pub fn sts2(&mut self) -> STS2_W {
        STS2_W { w: self }
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT3"]
    #[inline(always)]
    pub fn den3(&mut self) -> DEN3_W {
        DEN3_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT3"]
    #[inline(always)]
    pub fn ien3(&mut self) -> IEN3_W {
        IEN3_W { w: self }
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3(&mut self) -> EDGE3_W {
        EDGE3_W { w: self }
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT3"]
    #[inline(always)]
    pub fn sts3(&mut self) -> STS3_W {
        STS3_W { w: self }
    }
}
