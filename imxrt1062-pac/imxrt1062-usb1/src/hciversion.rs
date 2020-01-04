#[doc = "Reader of register HCIVERSION"]
pub type R = crate::R<u16, super::HCIVERSION>;
#[doc = "Reader of field `HCIVERSION`"]
pub type HCIVERSION_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host Controller Interface Version Number Default value is '10h', which means EHCI rev1.0."]
    #[inline(always)]
    pub fn hciversion(&self) -> HCIVERSION_R {
        HCIVERSION_R::new((self.bits & 0xffff) as u16)
    }
}
