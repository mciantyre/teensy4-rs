#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `TF1BR0`"]
pub type TF1BR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF1BR1`"]
pub type TF1BR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF2BR0`"]
pub type TF2BR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF2BR1`"]
pub type TF2BR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF3BR0`"]
pub type TF3BR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF3BR1`"]
pub type TF3BR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF4BR0`"]
pub type TF4BR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF4BR1`"]
pub type TF4BR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF5BR0`"]
pub type TF5BR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF5BR1`"]
pub type TF5BR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF6PBR0`"]
pub type TF6PBR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TF6PBR1`"]
pub type TF6PBR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFSB`"]
pub type TFSB_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFLR`"]
pub type TFLR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFP`"]
pub type TFP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFMB`"]
pub type TFMB_R = crate::R<bool, bool>;
#[doc = "Reader of field `RETRY_CT`"]
pub type RETRY_CT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf1br0(&self) -> TF1BR0_R {
        TF1BR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf1br1(&self) -> TF1BR1_R {
        TF1BR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf2br0(&self) -> TF2BR0_R {
        TF2BR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf2br1(&self) -> TF2BR1_R {
        TF2BR1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf3br0(&self) -> TF3BR0_R {
        TF3BR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf3br1(&self) -> TF3BR1_R {
        TF3BR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf4br0(&self) -> TF4BR0_R {
        TF4BR0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf4br1(&self) -> TF4BR1_R {
        TF4BR1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf5br0(&self) -> TF5BR0_R {
        TF5BR0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf5br1(&self) -> TF5BR1_R {
        TF5BR1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Test Fail, 6 Plus Bit Run, Sampling 0s"]
    #[inline(always)]
    pub fn tf6pbr0(&self) -> TF6PBR0_R {
        TF6PBR0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Test Fail, 6 Plus Bit Run, Sampling 1s"]
    #[inline(always)]
    pub fn tf6pbr1(&self) -> TF6PBR1_R {
        TF6PBR1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
    #[inline(always)]
    pub fn tfsb(&self) -> TFSB_R {
        TFSB_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
    #[inline(always)]
    pub fn tflr(&self) -> TFLR_R {
        TFLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Test Fail, Poker. If TFP=1, the Poker Test has failed."]
    #[inline(always)]
    pub fn tfp(&self) -> TFP_R {
        TFP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
    #[inline(always)]
    pub fn tfmb(&self) -> TFMB_R {
        TFMB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn retry_ct(&self) -> RETRY_CT_R {
        RETRY_CT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
