#[doc = "Reader of register RXMGMASK"]
pub type R = crate::R<u32, super::RXMGMASK>;
#[doc = "Writer for register RXMGMASK"]
pub type W = crate::W<u32, super::RXMGMASK>;
#[doc = "Register RXMGMASK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RXMGMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "These bits mask the Mailbox filter bits as shown in the figure above\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum MG_A {
    #[doc = "0: the corresponding bit in the filter is \"don't care\""]
    MG_0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked against the one received"]
    MG_1 = 1,
}
impl From<MG_A> for u32 {
    #[inline(always)]
    fn from(variant: MG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MG`"]
pub type MG_R = crate::R<u32, MG_A>;
impl MG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, MG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MG_A::MG_0),
            1 => Val(MG_A::MG_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MG_0`"]
    #[inline(always)]
    pub fn is_mg_0(&self) -> bool {
        *self == MG_A::MG_0
    }
    #[doc = "Checks if the value of the field is `MG_1`"]
    #[inline(always)]
    pub fn is_mg_1(&self) -> bool {
        *self == MG_A::MG_1
    }
}
#[doc = "Write proxy for field `MG`"]
pub struct MG_W<'a> {
    w: &'a mut W,
}
impl<'a> MG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn mg_0(self) -> &'a mut W {
        self.variant(MG_A::MG_0)
    }
    #[doc = "The corresponding bit in the filter is checked against the one received"]
    #[inline(always)]
    pub fn mg_1(self) -> &'a mut W {
        self.variant(MG_A::MG_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - These bits mask the Mailbox filter bits as shown in the figure above"]
    #[inline(always)]
    pub fn mg(&self) -> MG_R {
        MG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits mask the Mailbox filter bits as shown in the figure above"]
    #[inline(always)]
    pub fn mg(&mut self) -> MG_W {
        MG_W { w: self }
    }
}
