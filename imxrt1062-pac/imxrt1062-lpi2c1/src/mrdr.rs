#[doc = "Reader of register MRDR"]
pub type R = crate::R<u32, super::MRDR>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u8, u8>;
#[doc = "RX Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTY_A {
    #[doc = "0: Receive FIFO is not empty"]
    RXEMPTY_0 = 0,
    #[doc = "1: Receive FIFO is empty"]
    RXEMPTY_1 = 1,
}
impl From<RXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEMPTY`"]
pub type RXEMPTY_R = crate::R<bool, RXEMPTY_A>;
impl RXEMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPTY_A {
        match self.bits {
            false => RXEMPTY_A::RXEMPTY_0,
            true => RXEMPTY_A::RXEMPTY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEMPTY_0`"]
    #[inline(always)]
    pub fn is_rxempty_0(&self) -> bool {
        *self == RXEMPTY_A::RXEMPTY_0
    }
    #[doc = "Checks if the value of the field is `RXEMPTY_1`"]
    #[inline(always)]
    pub fn is_rxempty_1(&self) -> bool {
        *self == RXEMPTY_A::RXEMPTY_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - RX Empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
