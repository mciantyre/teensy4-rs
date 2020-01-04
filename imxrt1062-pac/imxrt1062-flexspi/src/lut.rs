#[doc = "Reader of register LUT[%s]"]
pub type R = crate::R<u32, super::LUT>;
#[doc = "Writer for register LUT[%s]"]
pub type W = crate::W<u32, super::LUT>;
#[doc = "Register LUT[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::LUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OPERAND0`"]
pub type OPERAND0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPERAND0`"]
pub struct OPERAND0_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERAND0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NUM_PADS0`"]
pub type NUM_PADS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_PADS0`"]
pub struct NUM_PADS0_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_PADS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `OPCODE0`"]
pub type OPCODE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPCODE0`"]
pub struct OPCODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `OPERAND1`"]
pub type OPERAND1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPERAND1`"]
pub struct OPERAND1_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERAND1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `NUM_PADS1`"]
pub type NUM_PADS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_PADS1`"]
pub struct NUM_PADS1_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_PADS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `OPCODE1`"]
pub type OPCODE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPCODE1`"]
pub struct OPCODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - OPERAND0"]
    #[inline(always)]
    pub fn operand0(&self) -> OPERAND0_R {
        OPERAND0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - NUM_PADS0"]
    #[inline(always)]
    pub fn num_pads0(&self) -> NUM_PADS0_R {
        NUM_PADS0_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:15 - OPCODE"]
    #[inline(always)]
    pub fn opcode0(&self) -> OPCODE0_R {
        OPCODE0_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - OPERAND1"]
    #[inline(always)]
    pub fn operand1(&self) -> OPERAND1_R {
        OPERAND1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - NUM_PADS1"]
    #[inline(always)]
    pub fn num_pads1(&self) -> NUM_PADS1_R {
        NUM_PADS1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:31 - OPCODE1"]
    #[inline(always)]
    pub fn opcode1(&self) -> OPCODE1_R {
        OPCODE1_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - OPERAND0"]
    #[inline(always)]
    pub fn operand0(&mut self) -> OPERAND0_W {
        OPERAND0_W { w: self }
    }
    #[doc = "Bits 8:9 - NUM_PADS0"]
    #[inline(always)]
    pub fn num_pads0(&mut self) -> NUM_PADS0_W {
        NUM_PADS0_W { w: self }
    }
    #[doc = "Bits 10:15 - OPCODE"]
    #[inline(always)]
    pub fn opcode0(&mut self) -> OPCODE0_W {
        OPCODE0_W { w: self }
    }
    #[doc = "Bits 16:23 - OPERAND1"]
    #[inline(always)]
    pub fn operand1(&mut self) -> OPERAND1_W {
        OPERAND1_W { w: self }
    }
    #[doc = "Bits 24:25 - NUM_PADS1"]
    #[inline(always)]
    pub fn num_pads1(&mut self) -> NUM_PADS1_W {
        NUM_PADS1_W { w: self }
    }
    #[doc = "Bits 26:31 - OPCODE1"]
    #[inline(always)]
    pub fn opcode1(&mut self) -> OPCODE1_W {
        OPCODE1_W { w: self }
    }
}
