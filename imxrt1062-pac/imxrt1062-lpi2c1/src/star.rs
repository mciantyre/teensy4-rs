#[doc = "Reader of register STAR"]
pub type R = crate::R<u32, super::STAR>;
#[doc = "Writer for register STAR"]
pub type W = crate::W<u32, super::STAR>;
#[doc = "Register STAR `reset()`'s with value 0"]
impl crate::ResetValue for super::STAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXNACK_A {
    #[doc = "0: Write a Transmit ACK for each received word"]
    TXNACK_0 = 0,
    #[doc = "1: Write a Transmit NACK for each received word"]
    TXNACK_1 = 1,
}
impl From<TXNACK_A> for bool {
    #[inline(always)]
    fn from(variant: TXNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXNACK`"]
pub type TXNACK_R = crate::R<bool, TXNACK_A>;
impl TXNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXNACK_A {
        match self.bits {
            false => TXNACK_A::TXNACK_0,
            true => TXNACK_A::TXNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXNACK_0`"]
    #[inline(always)]
    pub fn is_txnack_0(&self) -> bool {
        *self == TXNACK_A::TXNACK_0
    }
    #[doc = "Checks if the value of the field is `TXNACK_1`"]
    #[inline(always)]
    pub fn is_txnack_1(&self) -> bool {
        *self == TXNACK_A::TXNACK_1
    }
}
#[doc = "Write proxy for field `TXNACK`"]
pub struct TXNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write a Transmit ACK for each received word"]
    #[inline(always)]
    pub fn txnack_0(self) -> &'a mut W {
        self.variant(TXNACK_A::TXNACK_0)
    }
    #[doc = "Write a Transmit NACK for each received word"]
    #[inline(always)]
    pub fn txnack_1(self) -> &'a mut W {
        self.variant(TXNACK_A::TXNACK_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit NACK"]
    #[inline(always)]
    pub fn txnack(&self) -> TXNACK_R {
        TXNACK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit NACK"]
    #[inline(always)]
    pub fn txnack(&mut self) -> TXNACK_W {
        TXNACK_W { w: self }
    }
}
