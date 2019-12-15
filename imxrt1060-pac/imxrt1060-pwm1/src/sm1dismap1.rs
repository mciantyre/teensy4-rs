#[doc = "Reader of register SM1DISMAP1"]
pub type R = crate::R<u16, super::SM1DISMAP1>;
#[doc = "Writer for register SM1DISMAP1"]
pub type W = crate::W<u16, super::SM1DISMAP1>;
#[doc = "Register SM1DISMAP1 `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::SM1DISMAP1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `DIS1A`"]
pub type DIS1A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIS1A`"]
pub struct DIS1A_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS1A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DIS1B`"]
pub type DIS1B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIS1B`"]
pub struct DIS1B_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS1B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIS1X`"]
pub type DIS1X_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIS1X`"]
pub struct DIS1X_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM_A Fault Disable Mask 1"]
    #[inline(always)]
    pub fn dis1a(&self) -> DIS1A_R {
        DIS1A_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PWM_B Fault Disable Mask 1"]
    #[inline(always)]
    pub fn dis1b(&self) -> DIS1B_R {
        DIS1B_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PWM_X Fault Disable Mask 1"]
    #[inline(always)]
    pub fn dis1x(&self) -> DIS1X_R {
        DIS1X_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_A Fault Disable Mask 1"]
    #[inline(always)]
    pub fn dis1a(&mut self) -> DIS1A_W {
        DIS1A_W { w: self }
    }
    #[doc = "Bits 4:7 - PWM_B Fault Disable Mask 1"]
    #[inline(always)]
    pub fn dis1b(&mut self) -> DIS1B_W {
        DIS1B_W { w: self }
    }
    #[doc = "Bits 8:11 - PWM_X Fault Disable Mask 1"]
    #[inline(always)]
    pub fn dis1x(&mut self) -> DIS1X_W {
        DIS1X_W { w: self }
    }
}
