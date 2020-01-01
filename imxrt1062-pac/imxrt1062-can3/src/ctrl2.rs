#[doc = "Reader of register CTRL2"]
pub type R = crate::R<u32, super::CTRL2>;
#[doc = "Writer for register CTRL2"]
pub type W = crate::W<u32, super::CTRL2>;
#[doc = "Register CTRL2 `reset()`'s with value 0x0080_0000"]
impl crate::ResetValue for super::CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_0000
    }
}
#[doc = "Time Stamp Capture Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSTAMPCAP_A {
    #[doc = "0: The high resolution time stamp capture is disabled"]
    TSTAMPCAP_0 = 0,
    #[doc = "1: The high resolution time stamp is captured in the end of the CAN frame"]
    TSTAMPCAP_1 = 1,
    #[doc = "2: The high resolution time stamp is captured in the start of the CAN frame"]
    TSTAMPCAP_2 = 2,
    #[doc = "3: The high resolution time stamp is captured in the start of frame for classical CAN frames and in res bit for CAN FD frames"]
    TSTAMPCAP_3 = 3,
}
impl From<TSTAMPCAP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTAMPCAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSTAMPCAP`"]
pub type TSTAMPCAP_R = crate::R<u8, TSTAMPCAP_A>;
impl TSTAMPCAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTAMPCAP_A {
        match self.bits {
            0 => TSTAMPCAP_A::TSTAMPCAP_0,
            1 => TSTAMPCAP_A::TSTAMPCAP_1,
            2 => TSTAMPCAP_A::TSTAMPCAP_2,
            3 => TSTAMPCAP_A::TSTAMPCAP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TSTAMPCAP_0`"]
    #[inline(always)]
    pub fn is_tstampcap_0(&self) -> bool {
        *self == TSTAMPCAP_A::TSTAMPCAP_0
    }
    #[doc = "Checks if the value of the field is `TSTAMPCAP_1`"]
    #[inline(always)]
    pub fn is_tstampcap_1(&self) -> bool {
        *self == TSTAMPCAP_A::TSTAMPCAP_1
    }
    #[doc = "Checks if the value of the field is `TSTAMPCAP_2`"]
    #[inline(always)]
    pub fn is_tstampcap_2(&self) -> bool {
        *self == TSTAMPCAP_A::TSTAMPCAP_2
    }
    #[doc = "Checks if the value of the field is `TSTAMPCAP_3`"]
    #[inline(always)]
    pub fn is_tstampcap_3(&self) -> bool {
        *self == TSTAMPCAP_A::TSTAMPCAP_3
    }
}
#[doc = "Write proxy for field `TSTAMPCAP`"]
pub struct TSTAMPCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTAMPCAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTAMPCAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The high resolution time stamp capture is disabled"]
    #[inline(always)]
    pub fn tstampcap_0(self) -> &'a mut W {
        self.variant(TSTAMPCAP_A::TSTAMPCAP_0)
    }
    #[doc = "The high resolution time stamp is captured in the end of the CAN frame"]
    #[inline(always)]
    pub fn tstampcap_1(self) -> &'a mut W {
        self.variant(TSTAMPCAP_A::TSTAMPCAP_1)
    }
    #[doc = "The high resolution time stamp is captured in the start of the CAN frame"]
    #[inline(always)]
    pub fn tstampcap_2(self) -> &'a mut W {
        self.variant(TSTAMPCAP_A::TSTAMPCAP_2)
    }
    #[doc = "The high resolution time stamp is captured in the start of frame for classical CAN frames and in res bit for CAN FD frames"]
    #[inline(always)]
    pub fn tstampcap_3(self) -> &'a mut W {
        self.variant(TSTAMPCAP_A::TSTAMPCAP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Message Buffer Time Stamp Base\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MBTSBASE_A {
    #[doc = "0: Message Buffer Time Stamp base is CAN_TIMER"]
    MBTSBASE_0 = 0,
    #[doc = "1: Message Buffer Time Stamp base is lower 16-bits of high resolution timer"]
    MBTSBASE_1 = 1,
    #[doc = "2: Message Buffer Time Stamp base is upper 16-bits of high resolution timerT"]
    MBTSBASE_2 = 2,
}
impl From<MBTSBASE_A> for u8 {
    #[inline(always)]
    fn from(variant: MBTSBASE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MBTSBASE`"]
pub type MBTSBASE_R = crate::R<u8, MBTSBASE_A>;
impl MBTSBASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MBTSBASE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MBTSBASE_A::MBTSBASE_0),
            1 => Val(MBTSBASE_A::MBTSBASE_1),
            2 => Val(MBTSBASE_A::MBTSBASE_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MBTSBASE_0`"]
    #[inline(always)]
    pub fn is_mbtsbase_0(&self) -> bool {
        *self == MBTSBASE_A::MBTSBASE_0
    }
    #[doc = "Checks if the value of the field is `MBTSBASE_1`"]
    #[inline(always)]
    pub fn is_mbtsbase_1(&self) -> bool {
        *self == MBTSBASE_A::MBTSBASE_1
    }
    #[doc = "Checks if the value of the field is `MBTSBASE_2`"]
    #[inline(always)]
    pub fn is_mbtsbase_2(&self) -> bool {
        *self == MBTSBASE_A::MBTSBASE_2
    }
}
#[doc = "Write proxy for field `MBTSBASE`"]
pub struct MBTSBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> MBTSBASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBTSBASE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Message Buffer Time Stamp base is CAN_TIMER"]
    #[inline(always)]
    pub fn mbtsbase_0(self) -> &'a mut W {
        self.variant(MBTSBASE_A::MBTSBASE_0)
    }
    #[doc = "Message Buffer Time Stamp base is lower 16-bits of high resolution timer"]
    #[inline(always)]
    pub fn mbtsbase_1(self) -> &'a mut W {
        self.variant(MBTSBASE_A::MBTSBASE_1)
    }
    #[doc = "Message Buffer Time Stamp base is upper 16-bits of high resolution timerT"]
    #[inline(always)]
    pub fn mbtsbase_2(self) -> &'a mut W {
        self.variant(MBTSBASE_A::MBTSBASE_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Edge Filter Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDFLTDIS_A {
    #[doc = "0: Edge Filter is enabled"]
    EDFLTDIS_0 = 0,
    #[doc = "1: Edge Filter is disabled"]
    EDFLTDIS_1 = 1,
}
impl From<EDFLTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: EDFLTDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDFLTDIS`"]
pub type EDFLTDIS_R = crate::R<bool, EDFLTDIS_A>;
impl EDFLTDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDFLTDIS_A {
        match self.bits {
            false => EDFLTDIS_A::EDFLTDIS_0,
            true => EDFLTDIS_A::EDFLTDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDFLTDIS_0`"]
    #[inline(always)]
    pub fn is_edfltdis_0(&self) -> bool {
        *self == EDFLTDIS_A::EDFLTDIS_0
    }
    #[doc = "Checks if the value of the field is `EDFLTDIS_1`"]
    #[inline(always)]
    pub fn is_edfltdis_1(&self) -> bool {
        *self == EDFLTDIS_A::EDFLTDIS_1
    }
}
#[doc = "Write proxy for field `EDFLTDIS`"]
pub struct EDFLTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EDFLTDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDFLTDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge Filter is enabled"]
    #[inline(always)]
    pub fn edfltdis_0(self) -> &'a mut W {
        self.variant(EDFLTDIS_A::EDFLTDIS_0)
    }
    #[doc = "Edge Filter is disabled"]
    #[inline(always)]
    pub fn edfltdis_1(self) -> &'a mut W {
        self.variant(EDFLTDIS_A::EDFLTDIS_1)
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
#[doc = "ISO CAN FD Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOCANFDEN_A {
    #[doc = "0: FlexCAN operates using the non-ISO CAN FD protocol."]
    ISOCANFDEN_0 = 0,
    #[doc = "1: FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    ISOCANFDEN_1 = 1,
}
impl From<ISOCANFDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ISOCANFDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISOCANFDEN`"]
pub type ISOCANFDEN_R = crate::R<bool, ISOCANFDEN_A>;
impl ISOCANFDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOCANFDEN_A {
        match self.bits {
            false => ISOCANFDEN_A::ISOCANFDEN_0,
            true => ISOCANFDEN_A::ISOCANFDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ISOCANFDEN_0`"]
    #[inline(always)]
    pub fn is_isocanfden_0(&self) -> bool {
        *self == ISOCANFDEN_A::ISOCANFDEN_0
    }
    #[doc = "Checks if the value of the field is `ISOCANFDEN_1`"]
    #[inline(always)]
    pub fn is_isocanfden_1(&self) -> bool {
        *self == ISOCANFDEN_A::ISOCANFDEN_1
    }
}
#[doc = "Write proxy for field `ISOCANFDEN`"]
pub struct ISOCANFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOCANFDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOCANFDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
    #[inline(always)]
    pub fn isocanfden_0(self) -> &'a mut W {
        self.variant(ISOCANFDEN_A::ISOCANFDEN_0)
    }
    #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    #[inline(always)]
    pub fn isocanfden_1(self) -> &'a mut W {
        self.variant(ISOCANFDEN_A::ISOCANFDEN_1)
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
#[doc = "Bit Timing Expansion enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTE_A {
    #[doc = "0: CAN Bit timing expansion is disabled."]
    BTE_0 = 0,
    #[doc = "1: CAN bit timing expansion is enabled."]
    BTE_1 = 1,
}
impl From<BTE_A> for bool {
    #[inline(always)]
    fn from(variant: BTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTE`"]
pub type BTE_R = crate::R<bool, BTE_A>;
impl BTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTE_A {
        match self.bits {
            false => BTE_A::BTE_0,
            true => BTE_A::BTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BTE_0`"]
    #[inline(always)]
    pub fn is_bte_0(&self) -> bool {
        *self == BTE_A::BTE_0
    }
    #[doc = "Checks if the value of the field is `BTE_1`"]
    #[inline(always)]
    pub fn is_bte_1(&self) -> bool {
        *self == BTE_A::BTE_1
    }
}
#[doc = "Write proxy for field `BTE`"]
pub struct BTE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAN Bit timing expansion is disabled."]
    #[inline(always)]
    pub fn bte_0(self) -> &'a mut W {
        self.variant(BTE_A::BTE_0)
    }
    #[doc = "CAN bit timing expansion is enabled."]
    #[inline(always)]
    pub fn bte_1(self) -> &'a mut W {
        self.variant(BTE_A::BTE_1)
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
#[doc = "Protocol Exception Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREXCEN_A {
    #[doc = "0: Protocol Exception is disabled."]
    PREXCEN_0 = 0,
    #[doc = "1: Protocol Exception is enabled."]
    PREXCEN_1 = 1,
}
impl From<PREXCEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREXCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREXCEN`"]
pub type PREXCEN_R = crate::R<bool, PREXCEN_A>;
impl PREXCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREXCEN_A {
        match self.bits {
            false => PREXCEN_A::PREXCEN_0,
            true => PREXCEN_A::PREXCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PREXCEN_0`"]
    #[inline(always)]
    pub fn is_prexcen_0(&self) -> bool {
        *self == PREXCEN_A::PREXCEN_0
    }
    #[doc = "Checks if the value of the field is `PREXCEN_1`"]
    #[inline(always)]
    pub fn is_prexcen_1(&self) -> bool {
        *self == PREXCEN_A::PREXCEN_1
    }
}
#[doc = "Write proxy for field `PREXCEN`"]
pub struct PREXCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREXCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREXCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protocol Exception is disabled."]
    #[inline(always)]
    pub fn prexcen_0(self) -> &'a mut W {
        self.variant(PREXCEN_A::PREXCEN_0)
    }
    #[doc = "Protocol Exception is enabled."]
    #[inline(always)]
    pub fn prexcen_1(self) -> &'a mut W {
        self.variant(PREXCEN_A::PREXCEN_1)
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
#[doc = "Timer Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SRC_A {
    #[doc = "0: The Free Running Timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    TIMER_SRC_0 = 0,
    #[doc = "1: The Free Running Timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device specific section for details about the external time tick."]
    TIMER_SRC_1 = 1,
}
impl From<TIMER_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER_SRC`"]
pub type TIMER_SRC_R = crate::R<bool, TIMER_SRC_A>;
impl TIMER_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SRC_A {
        match self.bits {
            false => TIMER_SRC_A::TIMER_SRC_0,
            true => TIMER_SRC_A::TIMER_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SRC_0`"]
    #[inline(always)]
    pub fn is_timer_src_0(&self) -> bool {
        *self == TIMER_SRC_A::TIMER_SRC_0
    }
    #[doc = "Checks if the value of the field is `TIMER_SRC_1`"]
    #[inline(always)]
    pub fn is_timer_src_1(&self) -> bool {
        *self == TIMER_SRC_A::TIMER_SRC_1
    }
}
#[doc = "Write proxy for field `TIMER_SRC`"]
pub struct TIMER_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Free Running Timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    #[inline(always)]
    pub fn timer_src_0(self) -> &'a mut W {
        self.variant(TIMER_SRC_A::TIMER_SRC_0)
    }
    #[doc = "The Free Running Timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device specific section for details about the external time tick."]
    #[inline(always)]
    pub fn timer_src_1(self) -> &'a mut W {
        self.variant(TIMER_SRC_A::TIMER_SRC_1)
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
#[doc = "Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes\n\nValue on reset: 0"]
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
#[doc = "Remote Request Storing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRS_A {
    #[doc = "0: Remote Response Frame is generated."]
    RRS_0 = 0,
    #[doc = "1: Remote Request Frame is stored."]
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
    #[doc = "Remote Response Frame is generated."]
    #[inline(always)]
    pub fn rrs_0(self) -> &'a mut W {
        self.variant(RRS_A::RRS_0)
    }
    #[doc = "Remote Request Frame is stored."]
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
#[doc = "Mailboxes Reception Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRP_A {
    #[doc = "0: Matching starts from Legacy Rx FIFO or Enhanced Rx FIFO and continues on Mailboxes."]
    MRP_0 = 0,
    #[doc = "1: Matching starts from Mailboxes and continues on Legacy Rx FIFO or Enhanced Rx FIFO ."]
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
    #[doc = "Matching starts from Legacy Rx FIFO or Enhanced Rx FIFO and continues on Mailboxes."]
    #[inline(always)]
    pub fn mrp_0(self) -> &'a mut W {
        self.variant(MRP_A::MRP_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Legacy Rx FIFO or Enhanced Rx FIFO ."]
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
#[doc = "Bus Off Done Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFDONEMSK_A {
    #[doc = "0: Bus Off Done interrupt disabled."]
    BOFFDONEMSK_0 = 0,
    #[doc = "1: Bus Off Done interrupt enabled."]
    BOFFDONEMSK_1 = 1,
}
impl From<BOFFDONEMSK_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFDONEMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOFFDONEMSK`"]
pub type BOFFDONEMSK_R = crate::R<bool, BOFFDONEMSK_A>;
impl BOFFDONEMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFDONEMSK_A {
        match self.bits {
            false => BOFFDONEMSK_A::BOFFDONEMSK_0,
            true => BOFFDONEMSK_A::BOFFDONEMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFDONEMSK_0`"]
    #[inline(always)]
    pub fn is_boffdonemsk_0(&self) -> bool {
        *self == BOFFDONEMSK_A::BOFFDONEMSK_0
    }
    #[doc = "Checks if the value of the field is `BOFFDONEMSK_1`"]
    #[inline(always)]
    pub fn is_boffdonemsk_1(&self) -> bool {
        *self == BOFFDONEMSK_A::BOFFDONEMSK_1
    }
}
#[doc = "Write proxy for field `BOFFDONEMSK`"]
pub struct BOFFDONEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFDONEMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFDONEMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus Off Done interrupt disabled."]
    #[inline(always)]
    pub fn boffdonemsk_0(self) -> &'a mut W {
        self.variant(BOFFDONEMSK_A::BOFFDONEMSK_0)
    }
    #[doc = "Bus Off Done interrupt enabled."]
    #[inline(always)]
    pub fn boffdonemsk_1(self) -> &'a mut W {
        self.variant(BOFFDONEMSK_A::BOFFDONEMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Error Interrupt Mask for errors detected in the Data Phase of fast CAN FD frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMSK_FAST_A {
    #[doc = "0: ERRINT_FAST Error interrupt disabled."]
    ERRMSK_FAST_0 = 0,
    #[doc = "1: ERRINT_FAST Error interrupt enabled."]
    ERRMSK_FAST_1 = 1,
}
impl From<ERRMSK_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: ERRMSK_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRMSK_FAST`"]
pub type ERRMSK_FAST_R = crate::R<bool, ERRMSK_FAST_A>;
impl ERRMSK_FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRMSK_FAST_A {
        match self.bits {
            false => ERRMSK_FAST_A::ERRMSK_FAST_0,
            true => ERRMSK_FAST_A::ERRMSK_FAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRMSK_FAST_0`"]
    #[inline(always)]
    pub fn is_errmsk_fast_0(&self) -> bool {
        *self == ERRMSK_FAST_A::ERRMSK_FAST_0
    }
    #[doc = "Checks if the value of the field is `ERRMSK_FAST_1`"]
    #[inline(always)]
    pub fn is_errmsk_fast_1(&self) -> bool {
        *self == ERRMSK_FAST_A::ERRMSK_FAST_1
    }
}
#[doc = "Write proxy for field `ERRMSK_FAST`"]
pub struct ERRMSK_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRMSK_FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRMSK_FAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ERRINT_FAST Error interrupt disabled."]
    #[inline(always)]
    pub fn errmsk_fast_0(self) -> &'a mut W {
        self.variant(ERRMSK_FAST_A::ERRMSK_FAST_0)
    }
    #[doc = "ERRINT_FAST Error interrupt enabled."]
    #[inline(always)]
    pub fn errmsk_fast_1(self) -> &'a mut W {
        self.variant(ERRMSK_FAST_A::ERRMSK_FAST_1)
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
    #[doc = "Bits 6:7 - Time Stamp Capture Point"]
    #[inline(always)]
    pub fn tstampcap(&self) -> TSTAMPCAP_R {
        TSTAMPCAP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Message Buffer Time Stamp Base"]
    #[inline(always)]
    pub fn mbtsbase(&self) -> MBTSBASE_R {
        MBTSBASE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Edge Filter Disable"]
    #[inline(always)]
    pub fn edfltdis(&self) -> EDFLTDIS_R {
        EDFLTDIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ISO CAN FD Enable"]
    #[inline(always)]
    pub fn isocanfden(&self) -> ISOCANFDEN_R {
        ISOCANFDEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Bit Timing Expansion enable"]
    #[inline(always)]
    pub fn bte(&self) -> BTE_R {
        BTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Enable"]
    #[inline(always)]
    pub fn prexcen(&self) -> PREXCEN_R {
        PREXCEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer Source"]
    #[inline(always)]
    pub fn timer_src(&self) -> TIMER_SRC_R {
        TIMER_SRC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&self) -> EACEN_R {
        EACEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&self) -> MRP_R {
        MRP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&self) -> TASD_R {
        TASD_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Number Of Legacy Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&self) -> RFFN_R {
        RFFN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Bus Off Done Interrupt Mask"]
    #[inline(always)]
    pub fn boffdonemsk(&self) -> BOFFDONEMSK_R {
        BOFFDONEMSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Error Interrupt Mask for errors detected in the Data Phase of fast CAN FD frames"]
    #[inline(always)]
    pub fn errmsk_fast(&self) -> ERRMSK_FAST_R {
        ERRMSK_FAST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - Time Stamp Capture Point"]
    #[inline(always)]
    pub fn tstampcap(&mut self) -> TSTAMPCAP_W {
        TSTAMPCAP_W { w: self }
    }
    #[doc = "Bits 8:9 - Message Buffer Time Stamp Base"]
    #[inline(always)]
    pub fn mbtsbase(&mut self) -> MBTSBASE_W {
        MBTSBASE_W { w: self }
    }
    #[doc = "Bit 11 - Edge Filter Disable"]
    #[inline(always)]
    pub fn edfltdis(&mut self) -> EDFLTDIS_W {
        EDFLTDIS_W { w: self }
    }
    #[doc = "Bit 12 - ISO CAN FD Enable"]
    #[inline(always)]
    pub fn isocanfden(&mut self) -> ISOCANFDEN_W {
        ISOCANFDEN_W { w: self }
    }
    #[doc = "Bit 13 - Bit Timing Expansion enable"]
    #[inline(always)]
    pub fn bte(&mut self) -> BTE_W {
        BTE_W { w: self }
    }
    #[doc = "Bit 14 - Protocol Exception Enable"]
    #[inline(always)]
    pub fn prexcen(&mut self) -> PREXCEN_W {
        PREXCEN_W { w: self }
    }
    #[doc = "Bit 15 - Timer Source"]
    #[inline(always)]
    pub fn timer_src(&mut self) -> TIMER_SRC_W {
        TIMER_SRC_W { w: self }
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&mut self) -> EACEN_W {
        EACEN_W { w: self }
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W {
        RRS_W { w: self }
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&mut self) -> MRP_W {
        MRP_W { w: self }
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&mut self) -> TASD_W {
        TASD_W { w: self }
    }
    #[doc = "Bits 24:27 - Number Of Legacy Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&mut self) -> RFFN_W {
        RFFN_W { w: self }
    }
    #[doc = "Bit 30 - Bus Off Done Interrupt Mask"]
    #[inline(always)]
    pub fn boffdonemsk(&mut self) -> BOFFDONEMSK_W {
        BOFFDONEMSK_W { w: self }
    }
    #[doc = "Bit 31 - Error Interrupt Mask for errors detected in the Data Phase of fast CAN FD frames"]
    #[inline(always)]
    pub fn errmsk_fast(&mut self) -> ERRMSK_FAST_W {
        ERRMSK_FAST_W { w: self }
    }
}
