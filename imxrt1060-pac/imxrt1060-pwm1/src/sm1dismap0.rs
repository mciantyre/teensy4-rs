#[doc = "Reader of register SM1DISMAP0"]
pub type R = crate::R<u16, super::SM1DISMAP0>;
#[doc = "Writer for register SM1DISMAP0"]
pub type W = crate::W<u16, super::SM1DISMAP0>;
#[doc = "Register SM1DISMAP0 `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::SM1DISMAP0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `DIS0A`"]
pub type DIS0A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIS0A`"]
pub struct DIS0A_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS0A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DIS0B`"]
pub type DIS0B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIS0B`"]
pub struct DIS0B_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS0B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIS0X`"]
pub type DIS0X_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIS0X`"]
pub struct DIS0X_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS0X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0a(&self) -> DIS0A_R {
        DIS0A_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0b(&self) -> DIS0B_R {
        DIS0B_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0x(&self) -> DIS0X_R {
        DIS0X_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0a(&mut self) -> DIS0A_W {
        DIS0A_W { w: self }
    }
    #[doc = "Bits 4:7 - PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0b(&mut self) -> DIS0B_W {
        DIS0B_W { w: self }
    }
    #[doc = "Bits 8:11 - PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0x(&mut self) -> DIS0X_W {
        DIS0X_W { w: self }
    }
}
