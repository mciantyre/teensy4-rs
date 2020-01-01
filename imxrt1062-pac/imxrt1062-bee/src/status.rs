#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRQ_VEC`"]
pub type IRQ_VEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQ_VEC`"]
pub struct IRQ_VEC_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_VEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BEE_IDLE`"]
pub type BEE_IDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
    #[inline(always)]
    pub fn irq_vec(&self) -> IRQ_VEC_R {
        IRQ_VEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1'b1: BEE is idle; 1'b0: BEE is active"]
    #[inline(always)]
    pub fn bee_idle(&self) -> BEE_IDLE_R {
        BEE_IDLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - bit 7: Protected region-3 access violation bit 6: Protected region-2 access violation bit 5: Protected region-1 access violation bit 4: Protected region-0 access violation bit 3: Region-1 read channel security violation bit 2: Read channel illegal access detected bit 1: Region-0 read channel security violation bit 0: Disable abort"]
    #[inline(always)]
    pub fn irq_vec(&mut self) -> IRQ_VEC_W {
        IRQ_VEC_W { w: self }
    }
}
