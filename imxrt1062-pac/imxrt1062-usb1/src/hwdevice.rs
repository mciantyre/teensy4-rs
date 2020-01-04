#[doc = "Reader of register HWDEVICE"]
pub type R = crate::R<u32, super::HWDEVICE>;
#[doc = "Device Capable. Indicating whether device operation mode is supported or not.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_A {
    #[doc = "0: Not supported"]
    DC_0 = 0,
    #[doc = "1: Supported"]
    DC_1 = 1,
}
impl From<DC_A> for bool {
    #[inline(always)]
    fn from(variant: DC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DC`"]
pub type DC_R = crate::R<bool, DC_A>;
impl DC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DC_A {
        match self.bits {
            false => DC_A::DC_0,
            true => DC_A::DC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DC_0`"]
    #[inline(always)]
    pub fn is_dc_0(&self) -> bool {
        *self == DC_A::DC_0
    }
    #[doc = "Checks if the value of the field is `DC_1`"]
    #[inline(always)]
    pub fn is_dc_1(&self) -> bool {
        *self == DC_A::DC_1
    }
}
#[doc = "Reader of field `DEVEP`"]
pub type DEVEP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Device Capable. Indicating whether device operation mode is supported or not."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - Device Endpoint Number"]
    #[inline(always)]
    pub fn devep(&self) -> DEVEP_R {
        DEVEP_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
