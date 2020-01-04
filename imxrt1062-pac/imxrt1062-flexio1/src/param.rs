#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `SHIFTER`"]
pub type SHIFTER_R = crate::R<u8, u8>;
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<u8, u8>;
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIGGER`"]
pub type TRIGGER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Shifter Number"]
    #[inline(always)]
    pub fn shifter(&self) -> SHIFTER_R {
        SHIFTER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer Number"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pin Number"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Trigger Number"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
