#[doc = "Reader of register TRIG4_RESULT_3_2"]
pub type R = crate::R<u32, super::TRIG4_RESULT_3_2>;
#[doc = "Reader of field `DATA2`"]
pub type DATA2_R = crate::R<u16, u16>;
#[doc = "Reader of field `DATA3`"]
pub type DATA3_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Result DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Result DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
