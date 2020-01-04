#[doc = "Reader of register VERSION"]
pub type R = crate::R<u32, super::VERSION>;
#[doc = "Reader of field `STEP`"]
pub type STEP_R = crate::R<u16, u16>;
#[doc = "Reader of field `MINOR`"]
pub type MINOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Fixed read-only value reflecting the stepping of the RTL version."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Fixed read-only value reflecting the MINOR field of the RTL version."]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fixed read-only value reflecting the MAJOR field of the RTL version."]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
