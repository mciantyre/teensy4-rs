#[doc = "Reader of register RFR[%s]"]
pub type R = crate::R<u32, super::RFR>;
#[doc = "Reader of field `RFP`"]
pub type RFP_R = crate::R<u8, u8>;
#[doc = "Receive Channel Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCP_A {
    #[doc = "0: No effect."]
    RCP_0 = 0,
    #[doc = "1: FIFO combine is enabled for FIFO reads and this FIFO will be read on the next FIFO read."]
    RCP_1 = 1,
}
impl From<RCP_A> for bool {
    #[inline(always)]
    fn from(variant: RCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RCP`"]
pub type RCP_R = crate::R<bool, RCP_A>;
impl RCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCP_A {
        match self.bits {
            false => RCP_A::RCP_0,
            true => RCP_A::RCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `RCP_0`"]
    #[inline(always)]
    pub fn is_rcp_0(&self) -> bool {
        *self == RCP_A::RCP_0
    }
    #[doc = "Checks if the value of the field is `RCP_1`"]
    #[inline(always)]
    pub fn is_rcp_1(&self) -> bool {
        *self == RCP_A::RCP_1
    }
}
#[doc = "Reader of field `WFP`"]
pub type WFP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Read FIFO Pointer"]
    #[inline(always)]
    pub fn rfp(&self) -> RFP_R {
        RFP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Receive Channel Pointer"]
    #[inline(always)]
    pub fn rcp(&self) -> RCP_R {
        RCP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Write FIFO Pointer"]
    #[inline(always)]
    pub fn wfp(&self) -> WFP_R {
        WFP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
