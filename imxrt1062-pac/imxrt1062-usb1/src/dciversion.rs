#[doc = "Reader of register DCIVERSION"]
pub type R = crate::R<u16, super::DCIVERSION>;
#[doc = "Reader of field `DCIVERSION`"]
pub type DCIVERSION_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Device Controller Interface Version Number Default value is '01h', which means rev0.1."]
    #[inline(always)]
    pub fn dciversion(&self) -> DCIVERSION_R {
        DCIVERSION_R::new((self.bits & 0xffff) as u16)
    }
}
