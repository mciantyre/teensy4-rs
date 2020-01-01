#[doc = "Reader of register TRIG0_RESULT_1_0"]
pub type R = crate::R<u32, super::TRIG0_RESULT_1_0>;
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<u16, u16>;
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Result DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Result DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
