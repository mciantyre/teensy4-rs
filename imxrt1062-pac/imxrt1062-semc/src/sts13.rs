#[doc = "Reader of register STS13"]
pub type R = crate::R<u32, super::STS13>;
#[doc = "Reader of field `SLVLOCK`"]
pub type SLVLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `REFLOCK`"]
pub type REFLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLVSEL`"]
pub type SLVSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Sample clock slave delay line locked."]
    #[inline(always)]
    pub fn slvlock(&self) -> SLVLOCK_R {
        SLVLOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sample clock reference delay line locked."]
    #[inline(always)]
    pub fn reflock(&self) -> REFLOCK_R {
        REFLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - Sample clock slave delay line delay cell number selection ."]
    #[inline(always)]
    pub fn slvsel(&self) -> SLVSEL_R {
        SLVSEL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
