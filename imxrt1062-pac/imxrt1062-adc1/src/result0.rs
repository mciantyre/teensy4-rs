#[doc = "Reader of register RESULT0"]
pub type R = crate::R<u32, super::RESULT0>;
#[doc = "Reader of field `CDATA`"]
pub type CDATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Data (result of an ADC conversion)"]
    #[inline(always)]
    pub fn cdata(&self) -> CDATA_R {
        CDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
