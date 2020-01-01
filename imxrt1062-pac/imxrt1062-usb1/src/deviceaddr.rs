#[doc = "Reader of register DEVICEADDR"]
pub type R = crate::R<u32, super::DEVICEADDR>;
#[doc = "Writer for register DEVICEADDR"]
pub type W = crate::W<u32, super::DEVICEADDR>;
#[doc = "Register DEVICEADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVICEADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBADRA`"]
pub type USBADRA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBADRA`"]
pub struct USBADRA_W<'a> {
    w: &'a mut W,
}
impl<'a> USBADRA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `USBADR`"]
pub type USBADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBADR`"]
pub struct USBADR_W<'a> {
    w: &'a mut W,
}
impl<'a> USBADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Device Address Advance"]
    #[inline(always)]
    pub fn usbadra(&self) -> USBADRA_R {
        USBADRA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:31 - Device Address. These bits correspond to the USB device address"]
    #[inline(always)]
    pub fn usbadr(&self) -> USBADR_R {
        USBADR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Device Address Advance"]
    #[inline(always)]
    pub fn usbadra(&mut self) -> USBADRA_W {
        USBADRA_W { w: self }
    }
    #[doc = "Bits 25:31 - Device Address. These bits correspond to the USB device address"]
    #[inline(always)]
    pub fn usbadr(&mut self) -> USBADR_W {
        USBADR_W { w: self }
    }
}
