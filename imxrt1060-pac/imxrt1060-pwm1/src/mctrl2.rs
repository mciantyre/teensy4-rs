#[doc = "Reader of register MCTRL2"]
pub type R = crate::R<u16, super::MCTRL2>;
#[doc = "Writer for register MCTRL2"]
pub type W = crate::W<u16, super::MCTRL2>;
#[doc = "Register MCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTRL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Monitor PLL State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MONPLL_A {
    #[doc = "0: Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
    MONPLL_0 = 0,
    #[doc = "1: Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
    MONPLL_1 = 1,
    #[doc = "2: Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
    MONPLL_2 = 2,
    #[doc = "3: Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
    MONPLL_3 = 3,
}
impl From<MONPLL_A> for u8 {
    #[inline(always)]
    fn from(variant: MONPLL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MONPLL`"]
pub type MONPLL_R = crate::R<u8, MONPLL_A>;
impl MONPLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONPLL_A {
        match self.bits {
            0 => MONPLL_A::MONPLL_0,
            1 => MONPLL_A::MONPLL_1,
            2 => MONPLL_A::MONPLL_2,
            3 => MONPLL_A::MONPLL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MONPLL_0`"]
    #[inline(always)]
    pub fn is_monpll_0(&self) -> bool {
        *self == MONPLL_A::MONPLL_0
    }
    #[doc = "Checks if the value of the field is `MONPLL_1`"]
    #[inline(always)]
    pub fn is_monpll_1(&self) -> bool {
        *self == MONPLL_A::MONPLL_1
    }
    #[doc = "Checks if the value of the field is `MONPLL_2`"]
    #[inline(always)]
    pub fn is_monpll_2(&self) -> bool {
        *self == MONPLL_A::MONPLL_2
    }
    #[doc = "Checks if the value of the field is `MONPLL_3`"]
    #[inline(always)]
    pub fn is_monpll_3(&self) -> bool {
        *self == MONPLL_A::MONPLL_3
    }
}
#[doc = "Write proxy for field `MONPLL`"]
pub struct MONPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> MONPLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONPLL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
    #[inline(always)]
    pub fn monpll_0(self) -> &'a mut W {
        self.variant(MONPLL_A::MONPLL_0)
    }
    #[doc = "Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
    #[inline(always)]
    pub fn monpll_1(self) -> &'a mut W {
        self.variant(MONPLL_A::MONPLL_1)
    }
    #[doc = "Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
    #[inline(always)]
    pub fn monpll_2(self) -> &'a mut W {
        self.variant(MONPLL_A::MONPLL_2)
    }
    #[doc = "Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
    #[inline(always)]
    pub fn monpll_3(self) -> &'a mut W {
        self.variant(MONPLL_A::MONPLL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Monitor PLL State"]
    #[inline(always)]
    pub fn monpll(&self) -> MONPLL_R {
        MONPLL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Monitor PLL State"]
    #[inline(always)]
    pub fn monpll(&mut self) -> MONPLL_W {
        MONPLL_W { w: self }
    }
}
