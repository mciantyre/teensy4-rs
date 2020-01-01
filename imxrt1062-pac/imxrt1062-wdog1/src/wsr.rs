#[doc = "Reader of register WSR"]
pub type R = crate::R<u16, super::WSR>;
#[doc = "Writer for register WSR"]
pub type W = crate::W<u16, super::WSR>;
#[doc = "Register WSR `reset()`'s with value 0"]
impl crate::ResetValue for super::WSR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum WSR_A {
    #[doc = "21845: Write to the Watchdog Service Register (WDOG_WSR)."]
    WSR_21845 = 21845,
    #[doc = "43690: Write to the Watchdog Service Register (WDOG_WSR)."]
    WSR_43690 = 43690,
}
impl From<WSR_A> for u16 {
    #[inline(always)]
    fn from(variant: WSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WSR`"]
pub type WSR_R = crate::R<u16, WSR_A>;
impl WSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, WSR_A> {
        use crate::Variant::*;
        match self.bits {
            21845 => Val(WSR_A::WSR_21845),
            43690 => Val(WSR_A::WSR_43690),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WSR_21845`"]
    #[inline(always)]
    pub fn is_wsr_21845(&self) -> bool {
        *self == WSR_A::WSR_21845
    }
    #[doc = "Checks if the value of the field is `WSR_43690`"]
    #[inline(always)]
    pub fn is_wsr_43690(&self) -> bool {
        *self == WSR_A::WSR_43690
    }
}
#[doc = "Write proxy for field `WSR`"]
pub struct WSR_W<'a> {
    w: &'a mut W,
}
impl<'a> WSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    #[inline(always)]
    pub fn wsr_21845(self) -> &'a mut W {
        self.variant(WSR_A::WSR_21845)
    }
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    #[inline(always)]
    pub fn wsr_43690(self) -> &'a mut W {
        self.variant(WSR_A::WSR_43690)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - WSR"]
    #[inline(always)]
    pub fn wsr(&self) -> WSR_R {
        WSR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WSR"]
    #[inline(always)]
    pub fn wsr(&mut self) -> WSR_W {
        WSR_W { w: self }
    }
}
