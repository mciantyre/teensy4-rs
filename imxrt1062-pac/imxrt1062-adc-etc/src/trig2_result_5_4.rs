#[doc = "Reader of register TRIG2_RESULT_5_4"]
pub type R = crate::R<u32, super::TRIG2_RESULT_5_4>;
#[doc = "Reader of field `DATA4`"]
pub type DATA4_R = crate::R<u16, u16>;
#[doc = "Reader of field `DATA5`"]
pub type DATA5_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Result DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Result DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
