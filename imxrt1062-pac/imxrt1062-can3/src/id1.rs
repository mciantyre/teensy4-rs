#[doc = "Reader of register ID1"]
pub type R = crate::R<u32, super::ID1>;
#[doc = "Writer for register ID1"]
pub type W = crate::W<u32, super::ID1>;
#[doc = "Register ID1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ID1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EXT`"]
pub struct EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
#[doc = "Reader of field `STD`"]
pub type STD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STD`"]
pub struct STD_W<'a> {
    w: &'a mut W,
}
impl<'a> STD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | (((value as u32) & 0x07ff) << 18);
        self.w
    }
}
#[doc = "Reader of field `PRIO`"]
pub type PRIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIO`"]
pub struct PRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub fn std(&self) -> STD_R {
        STD_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 29:31 - Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub fn ext(&mut self) -> EXT_W {
        EXT_W { w: self }
    }
    #[doc = "Bits 18:28 - Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub fn std(&mut self) -> STD_W {
        STD_W { w: self }
    }
    #[doc = "Bits 29:31 - Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W {
        PRIO_W { w: self }
    }
}
