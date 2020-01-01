#[doc = "Reader of register STS2"]
pub type R = crate::R<u32, super::STS2>;
#[doc = "Reader of field `ASLVLOCK`"]
pub type ASLVLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `AREFLOCK`"]
pub type AREFLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASLVSEL`"]
pub type ASLVSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `AREFSEL`"]
pub type AREFSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `BSLVLOCK`"]
pub type BSLVLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `BREFLOCK`"]
pub type BREFLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSLVSEL`"]
pub type BSLVSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `BREFSEL`"]
pub type BREFSEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Flash A sample clock slave delay line locked."]
    #[inline(always)]
    pub fn aslvlock(&self) -> ASLVLOCK_R {
        ASLVLOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash A sample clock reference delay line locked."]
    #[inline(always)]
    pub fn areflock(&self) -> AREFLOCK_R {
        AREFLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - Flash A sample clock slave delay line delay cell number selection ."]
    #[inline(always)]
    pub fn aslvsel(&self) -> ASLVSEL_R {
        ASLVSEL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Flash A sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn arefsel(&self) -> AREFSEL_R {
        AREFSEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Flash B sample clock slave delay line locked."]
    #[inline(always)]
    pub fn bslvlock(&self) -> BSLVLOCK_R {
        BSLVLOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Flash B sample clock reference delay line locked."]
    #[inline(always)]
    pub fn breflock(&self) -> BREFLOCK_R {
        BREFLOCK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:23 - Flash B sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub fn bslvsel(&self) -> BSLVSEL_R {
        BSLVSEL_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Flash B sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn brefsel(&self) -> BREFSEL_R {
        BREFSEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
