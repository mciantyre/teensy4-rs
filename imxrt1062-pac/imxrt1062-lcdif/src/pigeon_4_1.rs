#[doc = "Reader of register PIGEON_4_1"]
pub type R = crate::R<u32, super::PIGEON_4_1>;
#[doc = "Writer for register PIGEON_4_1"]
pub type W = crate::W<u32, super::PIGEON_4_1>;
#[doc = "Register PIGEON_4_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIGEON_4_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Assert signal output when counter match this value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SET_CNT_A {
    #[doc = "0: Start as active"]
    START_ACTIVE = 0,
}
impl From<SET_CNT_A> for u16 {
    #[inline(always)]
    fn from(variant: SET_CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SET_CNT`"]
pub type SET_CNT_R = crate::R<u16, SET_CNT_A>;
impl SET_CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SET_CNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SET_CNT_A::START_ACTIVE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `START_ACTIVE`"]
    #[inline(always)]
    pub fn is_start_active(&self) -> bool {
        *self == SET_CNT_A::START_ACTIVE
    }
}
#[doc = "Write proxy for field `SET_CNT`"]
pub struct SET_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_CNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Start as active"]
    #[inline(always)]
    pub fn start_active(self) -> &'a mut W {
        self.variant(SET_CNT_A::START_ACTIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Deassert signal output when counter match this value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CLR_CNT_A {
    #[doc = "0: Keep active until mask off"]
    CLEAR_USING_MASK = 0,
}
impl From<CLR_CNT_A> for u16 {
    #[inline(always)]
    fn from(variant: CLR_CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLR_CNT`"]
pub type CLR_CNT_R = crate::R<u16, CLR_CNT_A>;
impl CLR_CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CLR_CNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLR_CNT_A::CLEAR_USING_MASK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_USING_MASK`"]
    #[inline(always)]
    pub fn is_clear_using_mask(&self) -> bool {
        *self == CLR_CNT_A::CLEAR_USING_MASK
    }
}
#[doc = "Write proxy for field `CLR_CNT`"]
pub struct CLR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_CNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Keep active until mask off"]
    #[inline(always)]
    pub fn clear_using_mask(self) -> &'a mut W {
        self.variant(CLR_CNT_A::CLEAR_USING_MASK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Assert signal output when counter match this value"]
    #[inline(always)]
    pub fn set_cnt(&self) -> SET_CNT_R {
        SET_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Deassert signal output when counter match this value"]
    #[inline(always)]
    pub fn clr_cnt(&self) -> CLR_CNT_R {
        CLR_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Assert signal output when counter match this value"]
    #[inline(always)]
    pub fn set_cnt(&mut self) -> SET_CNT_W {
        SET_CNT_W { w: self }
    }
    #[doc = "Bits 16:31 - Deassert signal output when counter match this value"]
    #[inline(always)]
    pub fn clr_cnt(&mut self) -> CLR_CNT_W {
        CLR_CNT_W { w: self }
    }
}
