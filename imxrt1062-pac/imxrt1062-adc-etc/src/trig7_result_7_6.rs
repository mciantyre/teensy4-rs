#[doc = "Reader of register TRIG7_RESULT_7_6"]
pub type R = crate::R<u32, super::TRIG7_RESULT_7_6>;
#[doc = "Reader of field `DATA6`"]
pub type DATA6_R = crate::R<u16, u16>;
#[doc = "Reader of field `DATA7`"]
pub type DATA7_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Result DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Result DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
