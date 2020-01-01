#[doc = "Reader of register WMCR"]
pub type R = crate::R<u16, super::WMCR>;
#[doc = "Writer for register WMCR"]
pub type W = crate::W<u16, super::WMCR>;
#[doc = "Register WMCR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::WMCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "PDE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDE_A {
    #[doc = "0: Power Down Counter of WDOG is disabled."]
    PDE_0 = 0,
    #[doc = "1: Power Down Counter of WDOG is enabled (Default)."]
    PDE_1 = 1,
}
impl From<PDE_A> for bool {
    #[inline(always)]
    fn from(variant: PDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDE`"]
pub type PDE_R = crate::R<bool, PDE_A>;
impl PDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDE_A {
        match self.bits {
            false => PDE_A::PDE_0,
            true => PDE_A::PDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDE_0`"]
    #[inline(always)]
    pub fn is_pde_0(&self) -> bool {
        *self == PDE_A::PDE_0
    }
    #[doc = "Checks if the value of the field is `PDE_1`"]
    #[inline(always)]
    pub fn is_pde_1(&self) -> bool {
        *self == PDE_A::PDE_1
    }
}
#[doc = "Write proxy for field `PDE`"]
pub struct PDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power Down Counter of WDOG is disabled."]
    #[inline(always)]
    pub fn pde_0(self) -> &'a mut W {
        self.variant(PDE_A::PDE_0)
    }
    #[doc = "Power Down Counter of WDOG is enabled (Default)."]
    #[inline(always)]
    pub fn pde_1(self) -> &'a mut W {
        self.variant(PDE_A::PDE_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDE"]
    #[inline(always)]
    pub fn pde(&self) -> PDE_R {
        PDE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDE"]
    #[inline(always)]
    pub fn pde(&mut self) -> PDE_W {
        PDE_W { w: self }
    }
}
