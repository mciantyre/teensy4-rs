#[doc = "Reader of register SRFM"]
pub type R = crate::R<u32, super::SRFM>;
#[doc = "Reader of field `FreqMeas`"]
pub type FREQMEAS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Frequency measurement data"]
    #[inline(always)]
    pub fn freq_meas(&self) -> FREQMEAS_R {
        FREQMEAS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
