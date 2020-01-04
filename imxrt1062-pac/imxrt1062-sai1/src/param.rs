#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `DATALINE`"]
pub type DATALINE_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFO`"]
pub type FIFO_R = crate::R<u8, u8>;
#[doc = "Reader of field `FRAME`"]
pub type FRAME_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of Datalines"]
    #[inline(always)]
    pub fn dataline(&self) -> DATALINE_R {
        DATALINE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - FIFO Size"]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Frame Size"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
