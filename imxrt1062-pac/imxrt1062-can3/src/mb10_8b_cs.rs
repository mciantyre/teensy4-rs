#[doc = "Reader of register MB10_8B_CS"]
pub type R = crate::R<u32, super::MB10_8B_CS>;
#[doc = "Writer for register MB10_8B_CS"]
pub type W = crate::W<u32, super::MB10_8B_CS>;
#[doc = "Register MB10_8B_CS `reset()`'s with value 0"]
impl crate::ResetValue for super::MB10_8B_CS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIME_STAMP`"]
pub type TIME_STAMP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIME_STAMP`"]
pub struct TIME_STAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_STAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DLC`"]
pub type DLC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLC`"]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTR`"]
pub type RTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTR`"]
pub struct RTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `IDE`"]
pub type IDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDE`"]
pub struct IDE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SRR`"]
pub type SRR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRR`"]
pub struct SRR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CODE`"]
pub type CODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CODE`"]
pub struct CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ESI`"]
pub type ESI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESI`"]
pub struct ESI_W<'a> {
    w: &'a mut W,
}
impl<'a> ESI_W<'a> {
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
#[doc = "Reader of field `BRS`"]
pub type BRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRS`"]
pub struct BRS_W<'a> {
    w: &'a mut W,
}
impl<'a> BRS_W<'a> {
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
#[doc = "Reader of field `EDL`"]
pub type EDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDL`"]
pub struct EDL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDL_W<'a> {
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
    #[doc = "Bits 0:15 - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub fn time_stamp(&self) -> TIME_STAMP_R {
        TIME_STAMP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub fn srr(&self) -> SRR_R {
        SRR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub fn esi(&self) -> ESI_R {
        ESI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub fn brs(&self) -> BRS_R {
        BRS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub fn edl(&self) -> EDL_R {
        EDL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub fn time_stamp(&mut self) -> TIME_STAMP_W {
        TIME_STAMP_W { w: self }
    }
    #[doc = "Bits 16:19 - Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
    #[doc = "Bit 20 - Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W {
        RTR_W { w: self }
    }
    #[doc = "Bit 21 - ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W {
        IDE_W { w: self }
    }
    #[doc = "Bit 22 - Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub fn srr(&mut self) -> SRR_W {
        SRR_W { w: self }
    }
    #[doc = "Bits 24:27 - Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub fn code(&mut self) -> CODE_W {
        CODE_W { w: self }
    }
    #[doc = "Bit 29 - Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub fn esi(&mut self) -> ESI_W {
        ESI_W { w: self }
    }
    #[doc = "Bit 30 - Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub fn brs(&mut self) -> BRS_W {
        BRS_W { w: self }
    }
    #[doc = "Bit 31 - Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub fn edl(&mut self) -> EDL_W {
        EDL_W { w: self }
    }
}
