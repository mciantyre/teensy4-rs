#[doc = "Reader of register GPR3"]
pub type R = crate::R<u32, super::GPR3>;
#[doc = "Writer for register GPR3"]
pub type W = crate::W<u32, super::GPR3>;
#[doc = "Register GPR3 `reset()`'s with value 0xf0f0"]
impl crate::ResetValue for super::GPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf0f0
    }
}
#[doc = "Reader of field `OCRAM_CTL`"]
pub type OCRAM_CTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCRAM_CTL`"]
pub struct OCRAM_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Select 128-bit dcp key from 256-bit key from snvs/ocotp\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCP_KEY_SEL_A {
    #[doc = "0: Select \\[127:0\\]
from snvs/ocotp key as dcp key"]
    DCP_KEY_SEL_0 = 0,
    #[doc = "1: Select \\[255:128\\]
from snvs/ocotp key as dcp key"]
    DCP_KEY_SEL_1 = 1,
}
impl From<DCP_KEY_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DCP_KEY_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCP_KEY_SEL`"]
pub type DCP_KEY_SEL_R = crate::R<bool, DCP_KEY_SEL_A>;
impl DCP_KEY_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCP_KEY_SEL_A {
        match self.bits {
            false => DCP_KEY_SEL_A::DCP_KEY_SEL_0,
            true => DCP_KEY_SEL_A::DCP_KEY_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCP_KEY_SEL_0`"]
    #[inline(always)]
    pub fn is_dcp_key_sel_0(&self) -> bool {
        *self == DCP_KEY_SEL_A::DCP_KEY_SEL_0
    }
    #[doc = "Checks if the value of the field is `DCP_KEY_SEL_1`"]
    #[inline(always)]
    pub fn is_dcp_key_sel_1(&self) -> bool {
        *self == DCP_KEY_SEL_A::DCP_KEY_SEL_1
    }
}
#[doc = "Write proxy for field `DCP_KEY_SEL`"]
pub struct DCP_KEY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCP_KEY_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCP_KEY_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select \\[127:0\\]
from snvs/ocotp key as dcp key"]
    #[inline(always)]
    pub fn dcp_key_sel_0(self) -> &'a mut W {
        self.variant(DCP_KEY_SEL_A::DCP_KEY_SEL_0)
    }
    #[doc = "Select \\[255:128\\]
from snvs/ocotp key as dcp key"]
    #[inline(always)]
    pub fn dcp_key_sel_1(self) -> &'a mut W {
        self.variant(DCP_KEY_SEL_A::DCP_KEY_SEL_1)
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
#[doc = "Reader of field `OCRAM2_CTL`"]
pub type OCRAM2_CTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCRAM2_CTL`"]
pub struct OCRAM2_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM2_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Request to halt axbs_l\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_L_HALT_REQ_A {
    #[doc = "0: axbs_l normal run"]
    AXBS_L_HALT_REQ_0 = 0,
    #[doc = "1: request to halt axbs_l"]
    AXBS_L_HALT_REQ_1 = 1,
}
impl From<AXBS_L_HALT_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_L_HALT_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_L_HALT_REQ`"]
pub type AXBS_L_HALT_REQ_R = crate::R<bool, AXBS_L_HALT_REQ_A>;
impl AXBS_L_HALT_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_L_HALT_REQ_A {
        match self.bits {
            false => AXBS_L_HALT_REQ_A::AXBS_L_HALT_REQ_0,
            true => AXBS_L_HALT_REQ_A::AXBS_L_HALT_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_L_HALT_REQ_0`"]
    #[inline(always)]
    pub fn is_axbs_l_halt_req_0(&self) -> bool {
        *self == AXBS_L_HALT_REQ_A::AXBS_L_HALT_REQ_0
    }
    #[doc = "Checks if the value of the field is `AXBS_L_HALT_REQ_1`"]
    #[inline(always)]
    pub fn is_axbs_l_halt_req_1(&self) -> bool {
        *self == AXBS_L_HALT_REQ_A::AXBS_L_HALT_REQ_1
    }
}
#[doc = "Write proxy for field `AXBS_L_HALT_REQ`"]
pub struct AXBS_L_HALT_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AXBS_L_HALT_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXBS_L_HALT_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "axbs_l normal run"]
    #[inline(always)]
    pub fn axbs_l_halt_req_0(self) -> &'a mut W {
        self.variant(AXBS_L_HALT_REQ_A::AXBS_L_HALT_REQ_0)
    }
    #[doc = "request to halt axbs_l"]
    #[inline(always)]
    pub fn axbs_l_halt_req_1(self) -> &'a mut W {
        self.variant(AXBS_L_HALT_REQ_A::AXBS_L_HALT_REQ_1)
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
#[doc = "Reader of field `OCRAM_STATUS`"]
pub type OCRAM_STATUS_R = crate::R<u8, u8>;
#[doc = "Reader of field `OCRAM2_STATUS`"]
pub type OCRAM2_STATUS_R = crate::R<u8, u8>;
#[doc = "This bit shows the status of axbs_l\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXBS_L_HALTED_A {
    #[doc = "0: axbs_l is not halted"]
    AXBS_L_HALTED_0 = 0,
    #[doc = "1: axbs_l is in halted status"]
    AXBS_L_HALTED_1 = 1,
}
impl From<AXBS_L_HALTED_A> for bool {
    #[inline(always)]
    fn from(variant: AXBS_L_HALTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXBS_L_HALTED`"]
pub type AXBS_L_HALTED_R = crate::R<bool, AXBS_L_HALTED_A>;
impl AXBS_L_HALTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXBS_L_HALTED_A {
        match self.bits {
            false => AXBS_L_HALTED_A::AXBS_L_HALTED_0,
            true => AXBS_L_HALTED_A::AXBS_L_HALTED_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXBS_L_HALTED_0`"]
    #[inline(always)]
    pub fn is_axbs_l_halted_0(&self) -> bool {
        *self == AXBS_L_HALTED_A::AXBS_L_HALTED_0
    }
    #[doc = "Checks if the value of the field is `AXBS_L_HALTED_1`"]
    #[inline(always)]
    pub fn is_axbs_l_halted_1(&self) -> bool {
        *self == AXBS_L_HALTED_A::AXBS_L_HALTED_1
    }
}
#[doc = "Write proxy for field `AXBS_L_HALTED`"]
pub struct AXBS_L_HALTED_W<'a> {
    w: &'a mut W,
}
impl<'a> AXBS_L_HALTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXBS_L_HALTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "axbs_l is not halted"]
    #[inline(always)]
    pub fn axbs_l_halted_0(self) -> &'a mut W {
        self.variant(AXBS_L_HALTED_A::AXBS_L_HALTED_0)
    }
    #[doc = "axbs_l is in halted status"]
    #[inline(always)]
    pub fn axbs_l_halted_1(self) -> &'a mut W {
        self.variant(AXBS_L_HALTED_A::AXBS_L_HALTED_1)
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
    #[doc = "Bits 0:3 - OCRAM_CTL\\[3\\]
- write address pipeline control bit"]
    #[inline(always)]
    pub fn ocram_ctl(&self) -> OCRAM_CTL_R {
        OCRAM_CTL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Select 128-bit dcp key from 256-bit key from snvs/ocotp"]
    #[inline(always)]
    pub fn dcp_key_sel(&self) -> DCP_KEY_SEL_R {
        DCP_KEY_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - OCRAM2_CTL\\[3\\]
- write address pipeline control bit"]
    #[inline(always)]
    pub fn ocram2_ctl(&self) -> OCRAM2_CTL_R {
        OCRAM2_CTL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Request to halt axbs_l"]
    #[inline(always)]
    pub fn axbs_l_halt_req(&self) -> AXBS_L_HALT_REQ_R {
        AXBS_L_HALT_REQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - This field shows the OCRAM pipeline settings status, controlled by OCRAM_CTL bits respectively"]
    #[inline(always)]
    pub fn ocram_status(&self) -> OCRAM_STATUS_R {
        OCRAM_STATUS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - This field shows the OCRAM2 pipeline settings status, controlled by OCRAM2_CTL bits respectively"]
    #[inline(always)]
    pub fn ocram2_status(&self) -> OCRAM2_STATUS_R {
        OCRAM2_STATUS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - This bit shows the status of axbs_l"]
    #[inline(always)]
    pub fn axbs_l_halted(&self) -> AXBS_L_HALTED_R {
        AXBS_L_HALTED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - OCRAM_CTL\\[3\\]
- write address pipeline control bit"]
    #[inline(always)]
    pub fn ocram_ctl(&mut self) -> OCRAM_CTL_W {
        OCRAM_CTL_W { w: self }
    }
    #[doc = "Bit 4 - Select 128-bit dcp key from 256-bit key from snvs/ocotp"]
    #[inline(always)]
    pub fn dcp_key_sel(&mut self) -> DCP_KEY_SEL_W {
        DCP_KEY_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - OCRAM2_CTL\\[3\\]
- write address pipeline control bit"]
    #[inline(always)]
    pub fn ocram2_ctl(&mut self) -> OCRAM2_CTL_W {
        OCRAM2_CTL_W { w: self }
    }
    #[doc = "Bit 15 - Request to halt axbs_l"]
    #[inline(always)]
    pub fn axbs_l_halt_req(&mut self) -> AXBS_L_HALT_REQ_W {
        AXBS_L_HALT_REQ_W { w: self }
    }
    #[doc = "Bit 31 - This bit shows the status of axbs_l"]
    #[inline(always)]
    pub fn axbs_l_halted(&mut self) -> AXBS_L_HALTED_W {
        AXBS_L_HALTED_W { w: self }
    }
}
