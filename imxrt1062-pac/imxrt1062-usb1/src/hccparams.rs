#[doc = "Reader of register HCCPARAMS"]
pub type R = crate::R<u32, super::HCCPARAMS>;
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFL`"]
pub type PFL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASP`"]
pub type ASP_R = crate::R<bool, bool>;
#[doc = "Reader of field `IST`"]
pub type IST_R = crate::R<u8, u8>;
#[doc = "Reader of field `EECP`"]
pub type EECP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - 64-bit Addressing Capability This bit is set '0b' in all controller core, no 64-bit addressing capability is supported"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Programmable Frame List Flag If this bit is set to zero, then the system software must use a frame list length of 1024 elements with this host controller"]
    #[inline(always)]
    pub fn pfl(&self) -> PFL_R {
        PFL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Asynchronous Schedule Park Capability If this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule"]
    #[inline(always)]
    pub fn asp(&self) -> ASP_R {
        ASP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Isochronous Scheduling Threshold"]
    #[inline(always)]
    pub fn ist(&self) -> IST_R {
        IST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - EHCI Extended Capabilities Pointer"]
    #[inline(always)]
    pub fn eecp(&self) -> EECP_R {
        EECP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
