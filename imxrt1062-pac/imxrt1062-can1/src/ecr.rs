#[doc = "Reader of register ECR"]
pub type R = crate::R<u32, super::ECR>;
#[doc = "Writer for register ECR"]
pub type W = crate::W<u32, super::ECR>;
#[doc = "Register ECR `reset()`'s with value 0"]
impl crate::ResetValue for super::ECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_ERR_COUNTER`"]
pub type TX_ERR_COUNTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_ERR_COUNTER`"]
pub struct TX_ERR_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ERR_COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RX_ERR_COUNTER`"]
pub type RX_ERR_COUNTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_ERR_COUNTER`"]
pub struct RX_ERR_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ERR_COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Tx_Err_Counter"]
    #[inline(always)]
    pub fn tx_err_counter(&self) -> TX_ERR_COUNTER_R {
        TX_ERR_COUNTER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rx_Err_Counter"]
    #[inline(always)]
    pub fn rx_err_counter(&self) -> RX_ERR_COUNTER_R {
        RX_ERR_COUNTER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tx_Err_Counter"]
    #[inline(always)]
    pub fn tx_err_counter(&mut self) -> TX_ERR_COUNTER_W {
        TX_ERR_COUNTER_W { w: self }
    }
    #[doc = "Bits 8:15 - Rx_Err_Counter"]
    #[inline(always)]
    pub fn rx_err_counter(&mut self) -> RX_ERR_COUNTER_W {
        RX_ERR_COUNTER_W { w: self }
    }
}
