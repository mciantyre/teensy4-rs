#[doc = "Reader of register TCR2"]
pub type R = crate::R<u32, super::TCR2>;
#[doc = "Writer for register TCR2"]
pub type W = crate::W<u32, super::TCR2>;
#[doc = "Register TCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Bit Clock Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCD_A {
    #[doc = "0: Bit clock is generated externally in Slave mode."]
    BCD_0 = 0,
    #[doc = "1: Bit clock is generated internally in Master mode."]
    BCD_1 = 1,
}
impl From<BCD_A> for bool {
    #[inline(always)]
    fn from(variant: BCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCD`"]
pub type BCD_R = crate::R<bool, BCD_A>;
impl BCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCD_A {
        match self.bits {
            false => BCD_A::BCD_0,
            true => BCD_A::BCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCD_0`"]
    #[inline(always)]
    pub fn is_bcd_0(&self) -> bool {
        *self == BCD_A::BCD_0
    }
    #[doc = "Checks if the value of the field is `BCD_1`"]
    #[inline(always)]
    pub fn is_bcd_1(&self) -> bool {
        *self == BCD_A::BCD_1
    }
}
#[doc = "Write proxy for field `BCD`"]
pub struct BCD_W<'a> {
    w: &'a mut W,
}
impl<'a> BCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit clock is generated externally in Slave mode."]
    #[inline(always)]
    pub fn bcd_0(self) -> &'a mut W {
        self.variant(BCD_A::BCD_0)
    }
    #[doc = "Bit clock is generated internally in Master mode."]
    #[inline(always)]
    pub fn bcd_1(self) -> &'a mut W {
        self.variant(BCD_A::BCD_1)
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
#[doc = "Bit Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCP_A {
    #[doc = "0: Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    BCP_0 = 0,
    #[doc = "1: Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    BCP_1 = 1,
}
impl From<BCP_A> for bool {
    #[inline(always)]
    fn from(variant: BCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCP`"]
pub type BCP_R = crate::R<bool, BCP_A>;
impl BCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCP_A {
        match self.bits {
            false => BCP_A::BCP_0,
            true => BCP_A::BCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCP_0`"]
    #[inline(always)]
    pub fn is_bcp_0(&self) -> bool {
        *self == BCP_A::BCP_0
    }
    #[doc = "Checks if the value of the field is `BCP_1`"]
    #[inline(always)]
    pub fn is_bcp_1(&self) -> bool {
        *self == BCP_A::BCP_1
    }
}
#[doc = "Write proxy for field `BCP`"]
pub struct BCP_W<'a> {
    w: &'a mut W,
}
impl<'a> BCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    #[inline(always)]
    pub fn bcp_0(self) -> &'a mut W {
        self.variant(BCP_A::BCP_0)
    }
    #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    #[inline(always)]
    pub fn bcp_1(self) -> &'a mut W {
        self.variant(BCP_A::BCP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "MCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: Bus Clock selected."]
    MSEL_0 = 0,
    #[doc = "1: Master Clock (MCLK) 1 option selected."]
    MSEL_1 = 1,
    #[doc = "2: Master Clock (MCLK) 2 option selected."]
    MSEL_2 = 2,
    #[doc = "3: Master Clock (MCLK) 3 option selected."]
    MSEL_3 = 3,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, MSEL_A>;
impl MSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::MSEL_0,
            1 => MSEL_A::MSEL_1,
            2 => MSEL_A::MSEL_2,
            3 => MSEL_A::MSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        *self == MSEL_A::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        *self == MSEL_A::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        *self == MSEL_A::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        *self == MSEL_A::MSEL_3
    }
}
#[doc = "Write proxy for field `MSEL`"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus Clock selected."]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_0)
    }
    #[doc = "Master Clock (MCLK) 1 option selected."]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_1)
    }
    #[doc = "Master Clock (MCLK) 2 option selected."]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_2)
    }
    #[doc = "Master Clock (MCLK) 3 option selected."]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Bit Clock Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCI_A {
    #[doc = "0: No effect."]
    BCI_0 = 0,
    #[doc = "1: Internal logic is clocked as if bit clock was externally generated."]
    BCI_1 = 1,
}
impl From<BCI_A> for bool {
    #[inline(always)]
    fn from(variant: BCI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCI`"]
pub type BCI_R = crate::R<bool, BCI_A>;
impl BCI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCI_A {
        match self.bits {
            false => BCI_A::BCI_0,
            true => BCI_A::BCI_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCI_0`"]
    #[inline(always)]
    pub fn is_bci_0(&self) -> bool {
        *self == BCI_A::BCI_0
    }
    #[doc = "Checks if the value of the field is `BCI_1`"]
    #[inline(always)]
    pub fn is_bci_1(&self) -> bool {
        *self == BCI_A::BCI_1
    }
}
#[doc = "Write proxy for field `BCI`"]
pub struct BCI_W<'a> {
    w: &'a mut W,
}
impl<'a> BCI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn bci_0(self) -> &'a mut W {
        self.variant(BCI_A::BCI_0)
    }
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    #[inline(always)]
    pub fn bci_1(self) -> &'a mut W {
        self.variant(BCI_A::BCI_1)
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
#[doc = "Bit Clock Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCS_A {
    #[doc = "0: Use the normal bit clock source."]
    BCS_0 = 0,
    #[doc = "1: Swap the bit clock source."]
    BCS_1 = 1,
}
impl From<BCS_A> for bool {
    #[inline(always)]
    fn from(variant: BCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCS`"]
pub type BCS_R = crate::R<bool, BCS_A>;
impl BCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCS_A {
        match self.bits {
            false => BCS_A::BCS_0,
            true => BCS_A::BCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCS_0`"]
    #[inline(always)]
    pub fn is_bcs_0(&self) -> bool {
        *self == BCS_A::BCS_0
    }
    #[doc = "Checks if the value of the field is `BCS_1`"]
    #[inline(always)]
    pub fn is_bcs_1(&self) -> bool {
        *self == BCS_A::BCS_1
    }
}
#[doc = "Write proxy for field `BCS`"]
pub struct BCS_W<'a> {
    w: &'a mut W,
}
impl<'a> BCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use the normal bit clock source."]
    #[inline(always)]
    pub fn bcs_0(self) -> &'a mut W {
        self.variant(BCS_A::BCS_0)
    }
    #[doc = "Swap the bit clock source."]
    #[inline(always)]
    pub fn bcs_1(self) -> &'a mut W {
        self.variant(BCS_A::BCS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Synchronous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC_A {
    #[doc = "0: Asynchronous mode."]
    SYNC_0 = 0,
    #[doc = "1: Synchronous with receiver."]
    SYNC_1 = 1,
}
impl From<SYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<u8, SYNC_A>;
impl SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNC_A::SYNC_0),
            1 => Val(SYNC_A::SYNC_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_0`"]
    #[inline(always)]
    pub fn is_sync_0(&self) -> bool {
        *self == SYNC_A::SYNC_0
    }
    #[doc = "Checks if the value of the field is `SYNC_1`"]
    #[inline(always)]
    pub fn is_sync_1(&self) -> bool {
        *self == SYNC_A::SYNC_1
    }
}
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn sync_0(self) -> &'a mut W {
        self.variant(SYNC_A::SYNC_0)
    }
    #[doc = "Synchronous with receiver."]
    #[inline(always)]
    pub fn sync_1(self) -> &'a mut W {
        self.variant(SYNC_A::SYNC_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline(always)]
    pub fn bcd(&self) -> BCD_R {
        BCD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline(always)]
    pub fn bci(&self) -> BCI_R {
        BCI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline(always)]
    pub fn bcs(&self) -> BCS_R {
        BCS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline(always)]
    pub fn bcd(&mut self) -> BCD_W {
        BCD_W { w: self }
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline(always)]
    pub fn bcp(&mut self) -> BCP_W {
        BCP_W { w: self }
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline(always)]
    pub fn bci(&mut self) -> BCI_W {
        BCI_W { w: self }
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline(always)]
    pub fn bcs(&mut self) -> BCS_W {
        BCS_W { w: self }
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
}
