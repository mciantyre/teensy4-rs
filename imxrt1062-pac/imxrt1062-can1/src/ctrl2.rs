#[doc = "Reader of register CTRL2"]
pub type R = crate::R<u32, super::CTRL2>;
#[doc = "Writer for register CTRL2"]
pub type W = crate::W<u32, super::CTRL2>;
#[doc = "Register CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EACEN_A {
    #[doc = "0: Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    EACEN_0 = 0,
    #[doc = "1: Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    EACEN_1 = 1,
}
impl From<EACEN_A> for bool {
    #[inline(always)]
    fn from(variant: EACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EACEN`"]
pub type EACEN_R = crate::R<bool, EACEN_A>;
impl EACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EACEN_A {
        match self.bits {
            false => EACEN_A::EACEN_0,
            true => EACEN_A::EACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EACEN_0`"]
    #[inline(always)]
    pub fn is_eacen_0(&self) -> bool {
        *self == EACEN_A::EACEN_0
    }
    #[doc = "Checks if the value of the field is `EACEN_1`"]
    #[inline(always)]
    pub fn is_eacen_1(&self) -> bool {
        *self == EACEN_A::EACEN_1
    }
}
#[doc = "Write proxy for field `EACEN`"]
pub struct EACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn eacen_0(self) -> &'a mut W {
        self.variant(EACEN_A::EACEN_0)
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn eacen_1(self) -> &'a mut W {
        self.variant(EACEN_A::EACEN_1)
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
#[doc = "If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRS_A {
    #[doc = "0: Remote Response Frame is generated"]
    RRS_0 = 0,
    #[doc = "1: Remote Request Frame is stored"]
    RRS_1 = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RRS`"]
pub type RRS_R = crate::R<bool, RRS_A>;
impl RRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::RRS_0,
            true => RRS_A::RRS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRS_0`"]
    #[inline(always)]
    pub fn is_rrs_0(&self) -> bool {
        *self == RRS_A::RRS_0
    }
    #[doc = "Checks if the value of the field is `RRS_1`"]
    #[inline(always)]
    pub fn is_rrs_1(&self) -> bool {
        *self == RRS_A::RRS_1
    }
}
#[doc = "Write proxy for field `RRS`"]
pub struct RRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Remote Response Frame is generated"]
    #[inline(always)]
    pub fn rrs_0(self) -> &'a mut W {
        self.variant(RRS_A::RRS_0)
    }
    #[doc = "Remote Request Frame is stored"]
    #[inline(always)]
    pub fn rrs_1(self) -> &'a mut W {
        self.variant(RRS_A::RRS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRP_A {
    #[doc = "0: Matching starts from Rx FIFO and continues on Mailboxes"]
    MRP_0 = 0,
    #[doc = "1: Matching starts from Mailboxes and continues on Rx FIFO"]
    MRP_1 = 1,
}
impl From<MRP_A> for bool {
    #[inline(always)]
    fn from(variant: MRP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MRP`"]
pub type MRP_R = crate::R<bool, MRP_A>;
impl MRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRP_A {
        match self.bits {
            false => MRP_A::MRP_0,
            true => MRP_A::MRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRP_0`"]
    #[inline(always)]
    pub fn is_mrp_0(&self) -> bool {
        *self == MRP_A::MRP_0
    }
    #[doc = "Checks if the value of the field is `MRP_1`"]
    #[inline(always)]
    pub fn is_mrp_1(&self) -> bool {
        *self == MRP_A::MRP_1
    }
}
#[doc = "Write proxy for field `MRP`"]
pub struct MRP_W<'a> {
    w: &'a mut W,
}
impl<'a> MRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes"]
    #[inline(always)]
    pub fn mrp_0(self) -> &'a mut W {
        self.variant(MRP_A::MRP_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO"]
    #[inline(always)]
    pub fn mrp_1(self) -> &'a mut W {
        self.variant(MRP_A::MRP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TASD`"]
pub type TASD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TASD`"]
pub struct TASD_W<'a> {
    w: &'a mut W,
}
impl<'a> TASD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `RFFN`"]
pub type RFFN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFFN`"]
pub struct RFFN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Enable unrestricted write access to FlexCAN memory in Freeze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRMFRZ_A {
    #[doc = "0: Keep the write access restricted in some regions of FlexCAN memory"]
    WRMFRZ_0 = 0,
    #[doc = "1: Enable unrestricted write access to FlexCAN memory"]
    WRMFRZ_1 = 1,
}
impl From<WRMFRZ_A> for bool {
    #[inline(always)]
    fn from(variant: WRMFRZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRMFRZ`"]
pub type WRMFRZ_R = crate::R<bool, WRMFRZ_A>;
impl WRMFRZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRMFRZ_A {
        match self.bits {
            false => WRMFRZ_A::WRMFRZ_0,
            true => WRMFRZ_A::WRMFRZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `WRMFRZ_0`"]
    #[inline(always)]
    pub fn is_wrmfrz_0(&self) -> bool {
        *self == WRMFRZ_A::WRMFRZ_0
    }
    #[doc = "Checks if the value of the field is `WRMFRZ_1`"]
    #[inline(always)]
    pub fn is_wrmfrz_1(&self) -> bool {
        *self == WRMFRZ_A::WRMFRZ_1
    }
}
#[doc = "Write proxy for field `WRMFRZ`"]
pub struct WRMFRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> WRMFRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRMFRZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Keep the write access restricted in some regions of FlexCAN memory"]
    #[inline(always)]
    pub fn wrmfrz_0(self) -> &'a mut W {
        self.variant(WRMFRZ_A::WRMFRZ_0)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory"]
    #[inline(always)]
    pub fn wrmfrz_1(self) -> &'a mut W {
        self.variant(WRMFRZ_A::WRMFRZ_1)
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
impl R {
    #[doc = "Bit 16 - This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    #[inline(always)]
    pub fn eacen(&self) -> EACEN_R {
        EACEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    #[inline(always)]
    pub fn mrp(&self) -> MRP_R {
        MRP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    #[inline(always)]
    pub fn tasd(&self) -> TASD_R {
        TASD_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - This 4-bit field defines the number of Rx FIFO filters according to"]
    #[inline(always)]
    pub fn rffn(&self) -> RFFN_R {
        RFFN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    #[inline(always)]
    pub fn wrmfrz(&self) -> WRMFRZ_R {
        WRMFRZ_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    #[inline(always)]
    pub fn eacen(&mut self) -> EACEN_W {
        EACEN_W { w: self }
    }
    #[doc = "Bit 17 - If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W {
        RRS_W { w: self }
    }
    #[doc = "Bit 18 - If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    #[inline(always)]
    pub fn mrp(&mut self) -> MRP_W {
        MRP_W { w: self }
    }
    #[doc = "Bits 19:23 - This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    #[inline(always)]
    pub fn tasd(&mut self) -> TASD_W {
        TASD_W { w: self }
    }
    #[doc = "Bits 24:27 - This 4-bit field defines the number of Rx FIFO filters according to"]
    #[inline(always)]
    pub fn rffn(&mut self) -> RFFN_W {
        RFFN_W { w: self }
    }
    #[doc = "Bit 28 - Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    #[inline(always)]
    pub fn wrmfrz(&mut self) -> WRMFRZ_W {
        WRMFRZ_W { w: self }
    }
}
