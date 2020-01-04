#[doc = "Reader of register HS"]
pub type R = crate::R<u32, super::HS>;
#[doc = "Reader of field `COCO0`"]
pub type COCO0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Conversion Complete Flag"]
    #[inline(always)]
    pub fn coco0(&self) -> COCO0_R {
        COCO0_R::new((self.bits & 0x01) != 0)
    }
}
