#[doc = "Reader of register KPCR"]
pub type R = crate::R<u16, super::KPCR>;
#[doc = "Writer for register KPCR"]
pub type W = crate::W<u16, super::KPCR>;
#[doc = "Register KPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::KPCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Keypad Row Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KRE_A {
    #[doc = "0: Row is not included in the keypad key press detect."]
    KRE_0 = 0,
    #[doc = "1: Row is included in the keypad key press detect."]
    KRE_1 = 1,
}
impl From<KRE_A> for u8 {
    #[inline(always)]
    fn from(variant: KRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KRE`"]
pub type KRE_R = crate::R<u8, KRE_A>;
impl KRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KRE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KRE_A::KRE_0),
            1 => Val(KRE_A::KRE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KRE_0`"]
    #[inline(always)]
    pub fn is_kre_0(&self) -> bool {
        *self == KRE_A::KRE_0
    }
    #[doc = "Checks if the value of the field is `KRE_1`"]
    #[inline(always)]
    pub fn is_kre_1(&self) -> bool {
        *self == KRE_A::KRE_1
    }
}
#[doc = "Write proxy for field `KRE`"]
pub struct KRE_W<'a> {
    w: &'a mut W,
}
impl<'a> KRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Row is not included in the keypad key press detect."]
    #[inline(always)]
    pub fn kre_0(self) -> &'a mut W {
        self.variant(KRE_A::KRE_0)
    }
    #[doc = "Row is included in the keypad key press detect."]
    #[inline(always)]
    pub fn kre_1(self) -> &'a mut W {
        self.variant(KRE_A::KRE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Keypad Column Strobe Open-Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KCO_A {
    #[doc = "0: Column strobe output is totem pole drive."]
    TOTEM_POLE = 0,
    #[doc = "1: Column strobe output is open drain."]
    OPEN_DRAIN = 1,
}
impl From<KCO_A> for u8 {
    #[inline(always)]
    fn from(variant: KCO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KCO`"]
pub type KCO_R = crate::R<u8, KCO_A>;
impl KCO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KCO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KCO_A::TOTEM_POLE),
            1 => Val(KCO_A::OPEN_DRAIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TOTEM_POLE`"]
    #[inline(always)]
    pub fn is_totem_pole(&self) -> bool {
        *self == KCO_A::TOTEM_POLE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == KCO_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `KCO`"]
pub struct KCO_W<'a> {
    w: &'a mut W,
}
impl<'a> KCO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KCO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Column strobe output is totem pole drive."]
    #[inline(always)]
    pub fn totem_pole(self) -> &'a mut W {
        self.variant(KCO_A::TOTEM_POLE)
    }
    #[doc = "Column strobe output is open drain."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(KCO_A::OPEN_DRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Keypad Row Enable"]
    #[inline(always)]
    pub fn kre(&self) -> KRE_R {
        KRE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Keypad Column Strobe Open-Drain Enable"]
    #[inline(always)]
    pub fn kco(&self) -> KCO_R {
        KCO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Keypad Row Enable"]
    #[inline(always)]
    pub fn kre(&mut self) -> KRE_W {
        KRE_W { w: self }
    }
    #[doc = "Bits 8:15 - Keypad Column Strobe Open-Drain Enable"]
    #[inline(always)]
    pub fn kco(&mut self) -> KCO_W {
        KCO_W { w: self }
    }
}
