#[doc = "Reader of register WICR"]
pub type R = crate::R<u16, super::WICR>;
#[doc = "Writer for register WICR"]
pub type W = crate::W<u16, super::WICR>;
#[doc = "Register WICR `reset()`'s with value 0x04"]
impl crate::ResetValue for super::WICR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "WICT\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WICT_A {
    #[doc = "0: WICT\\[7:0\\]
= Time duration between interrupt and time-out is 0 seconds."]
    WICT_0 = 0,
    #[doc = "1: WICT\\[7:0\\]
= Time duration between interrupt and time-out is 0.5 seconds."]
    WICT_1 = 1,
    #[doc = "4: WICT\\[7:0\\]
= Time duration between interrupt and time-out is 2 seconds (Default)."]
    WICT_4 = 4,
    #[doc = "255: WICT\\[7:0\\]
= Time duration between interrupt and time-out is 127.5 seconds."]
    WICT_255 = 255,
}
impl From<WICT_A> for u8 {
    #[inline(always)]
    fn from(variant: WICT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WICT`"]
pub type WICT_R = crate::R<u8, WICT_A>;
impl WICT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WICT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WICT_A::WICT_0),
            1 => Val(WICT_A::WICT_1),
            4 => Val(WICT_A::WICT_4),
            255 => Val(WICT_A::WICT_255),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WICT_0`"]
    #[inline(always)]
    pub fn is_wict_0(&self) -> bool {
        *self == WICT_A::WICT_0
    }
    #[doc = "Checks if the value of the field is `WICT_1`"]
    #[inline(always)]
    pub fn is_wict_1(&self) -> bool {
        *self == WICT_A::WICT_1
    }
    #[doc = "Checks if the value of the field is `WICT_4`"]
    #[inline(always)]
    pub fn is_wict_4(&self) -> bool {
        *self == WICT_A::WICT_4
    }
    #[doc = "Checks if the value of the field is `WICT_255`"]
    #[inline(always)]
    pub fn is_wict_255(&self) -> bool {
        *self == WICT_A::WICT_255
    }
}
#[doc = "Write proxy for field `WICT`"]
pub struct WICT_W<'a> {
    w: &'a mut W,
}
impl<'a> WICT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WICT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "WICT\\[7:0\\]
= Time duration between interrupt and time-out is 0 seconds."]
    #[inline(always)]
    pub fn wict_0(self) -> &'a mut W {
        self.variant(WICT_A::WICT_0)
    }
    #[doc = "WICT\\[7:0\\]
= Time duration between interrupt and time-out is 0.5 seconds."]
    #[inline(always)]
    pub fn wict_1(self) -> &'a mut W {
        self.variant(WICT_A::WICT_1)
    }
    #[doc = "WICT\\[7:0\\]
= Time duration between interrupt and time-out is 2 seconds (Default)."]
    #[inline(always)]
    pub fn wict_4(self) -> &'a mut W {
        self.variant(WICT_A::WICT_4)
    }
    #[doc = "WICT\\[7:0\\]
= Time duration between interrupt and time-out is 127.5 seconds."]
    #[inline(always)]
    pub fn wict_255(self) -> &'a mut W {
        self.variant(WICT_A::WICT_255)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "WTIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTIS_A {
    #[doc = "0: No interrupt has occurred (Default)."]
    WTIS_0 = 0,
    #[doc = "1: Interrupt has occurred"]
    WTIS_1 = 1,
}
impl From<WTIS_A> for bool {
    #[inline(always)]
    fn from(variant: WTIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTIS`"]
pub type WTIS_R = crate::R<bool, WTIS_A>;
impl WTIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTIS_A {
        match self.bits {
            false => WTIS_A::WTIS_0,
            true => WTIS_A::WTIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `WTIS_0`"]
    #[inline(always)]
    pub fn is_wtis_0(&self) -> bool {
        *self == WTIS_A::WTIS_0
    }
    #[doc = "Checks if the value of the field is `WTIS_1`"]
    #[inline(always)]
    pub fn is_wtis_1(&self) -> bool {
        *self == WTIS_A::WTIS_1
    }
}
#[doc = "Write proxy for field `WTIS`"]
pub struct WTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt has occurred (Default)."]
    #[inline(always)]
    pub fn wtis_0(self) -> &'a mut W {
        self.variant(WTIS_A::WTIS_0)
    }
    #[doc = "Interrupt has occurred"]
    #[inline(always)]
    pub fn wtis_1(self) -> &'a mut W {
        self.variant(WTIS_A::WTIS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "WIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIE_A {
    #[doc = "0: Disable Interrupt (Default)."]
    WIE_0 = 0,
    #[doc = "1: Enable Interrupt."]
    WIE_1 = 1,
}
impl From<WIE_A> for bool {
    #[inline(always)]
    fn from(variant: WIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WIE`"]
pub type WIE_R = crate::R<bool, WIE_A>;
impl WIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIE_A {
        match self.bits {
            false => WIE_A::WIE_0,
            true => WIE_A::WIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIE_0`"]
    #[inline(always)]
    pub fn is_wie_0(&self) -> bool {
        *self == WIE_A::WIE_0
    }
    #[doc = "Checks if the value of the field is `WIE_1`"]
    #[inline(always)]
    pub fn is_wie_1(&self) -> bool {
        *self == WIE_A::WIE_1
    }
}
#[doc = "Write proxy for field `WIE`"]
pub struct WIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Interrupt (Default)."]
    #[inline(always)]
    pub fn wie_0(self) -> &'a mut W {
        self.variant(WIE_A::WIE_0)
    }
    #[doc = "Enable Interrupt."]
    #[inline(always)]
    pub fn wie_1(self) -> &'a mut W {
        self.variant(WIE_A::WIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - WICT"]
    #[inline(always)]
    pub fn wict(&self) -> WICT_R {
        WICT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - WTIS"]
    #[inline(always)]
    pub fn wtis(&self) -> WTIS_R {
        WTIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - WIE"]
    #[inline(always)]
    pub fn wie(&self) -> WIE_R {
        WIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - WICT"]
    #[inline(always)]
    pub fn wict(&mut self) -> WICT_W {
        WICT_W { w: self }
    }
    #[doc = "Bit 14 - WTIS"]
    #[inline(always)]
    pub fn wtis(&mut self) -> WTIS_W {
        WTIS_W { w: self }
    }
    #[doc = "Bit 15 - WIE"]
    #[inline(always)]
    pub fn wie(&mut self) -> WIE_W {
        WIE_W { w: self }
    }
}
