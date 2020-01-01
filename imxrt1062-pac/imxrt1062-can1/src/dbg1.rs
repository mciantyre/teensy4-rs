#[doc = "Reader of register DBG1"]
pub type R = crate::R<u32, super::DBG1>;
#[doc = "Reader of field `CFSM`"]
pub type CFSM_R = crate::R<u8, u8>;
#[doc = "Reader of field `CBN`"]
pub type CBN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - CAN Finite State Machine"]
    #[inline(always)]
    pub fn cfsm(&self) -> CFSM_R {
        CFSM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - CAN Bit Number"]
    #[inline(always)]
    pub fn cbn(&self) -> CBN_R {
        CBN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
