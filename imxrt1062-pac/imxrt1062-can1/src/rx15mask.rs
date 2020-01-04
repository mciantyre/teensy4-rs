#[doc = "Reader of register RX15MASK"]
pub type R = crate::R<u32, super::RX15MASK>;
#[doc = "Writer for register RX15MASK"]
pub type W = crate::W<u32, super::RX15MASK>;
#[doc = "Register RX15MASK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RX15MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RX15M_A {
    #[doc = "0: the corresponding bit in the filter is \"don't care\""]
    RX15M_0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    RX15M_1 = 1,
}
impl From<RX15M_A> for u32 {
    #[inline(always)]
    fn from(variant: RX15M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX15M`"]
pub type RX15M_R = crate::R<u32, RX15M_A>;
impl RX15M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RX15M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX15M_A::RX15M_0),
            1 => Val(RX15M_A::RX15M_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX15M_0`"]
    #[inline(always)]
    pub fn is_rx15m_0(&self) -> bool {
        *self == RX15M_A::RX15M_0
    }
    #[doc = "Checks if the value of the field is `RX15M_1`"]
    #[inline(always)]
    pub fn is_rx15m_1(&self) -> bool {
        *self == RX15M_A::RX15M_1
    }
}
#[doc = "Write proxy for field `RX15M`"]
pub struct RX15M_W<'a> {
    w: &'a mut W,
}
impl<'a> RX15M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX15M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn rx15m_0(self) -> &'a mut W {
        self.variant(RX15M_A::RX15M_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn rx15m_1(self) -> &'a mut W {
        self.variant(RX15M_A::RX15M_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[inline(always)]
    pub fn rx15m(&self) -> RX15M_R {
        RX15M_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[inline(always)]
    pub fn rx15m(&mut self) -> RX15M_W {
        RX15M_W { w: self }
    }
}
