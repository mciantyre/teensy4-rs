#[doc = "Reader of register CCGR7"]
pub type R = crate::R<u32, super::CCGR7>;
#[doc = "Writer for register CCGR7"]
pub type W = crate::W<u32, super::CCGR7>;
#[doc = "Register CCGR7 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CCGR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `CG0`"]
pub type CG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG0`"]
pub struct CG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CG1`"]
pub type CG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG1`"]
pub struct CG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CG2`"]
pub type CG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG2`"]
pub struct CG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CG3`"]
pub type CG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG3`"]
pub struct CG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CG4`"]
pub type CG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG4`"]
pub struct CG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CG5`"]
pub type CG5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG5`"]
pub struct CG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `CG6`"]
pub type CG6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG6`"]
pub struct CG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - enet2_clk_enable"]
    #[inline(always)]
    pub fn cg0(&self) -> CG0_R {
        CG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - flexspi2_clk_enable"]
    #[inline(always)]
    pub fn cg1(&self) -> CG1_R {
        CG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - axbs_l_clk_enable"]
    #[inline(always)]
    pub fn cg2(&self) -> CG2_R {
        CG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - can3_clk_enable"]
    #[inline(always)]
    pub fn cg3(&self) -> CG3_R {
        CG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - can3_serial_clk_enable"]
    #[inline(always)]
    pub fn cg4(&self) -> CG4_R {
        CG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - aips_lite_clk_enable"]
    #[inline(always)]
    pub fn cg5(&self) -> CG5_R {
        CG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - flexio3_clk_enable"]
    #[inline(always)]
    pub fn cg6(&self) -> CG6_R {
        CG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - enet2_clk_enable"]
    #[inline(always)]
    pub fn cg0(&mut self) -> CG0_W {
        CG0_W { w: self }
    }
    #[doc = "Bits 2:3 - flexspi2_clk_enable"]
    #[inline(always)]
    pub fn cg1(&mut self) -> CG1_W {
        CG1_W { w: self }
    }
    #[doc = "Bits 4:5 - axbs_l_clk_enable"]
    #[inline(always)]
    pub fn cg2(&mut self) -> CG2_W {
        CG2_W { w: self }
    }
    #[doc = "Bits 6:7 - can3_clk_enable"]
    #[inline(always)]
    pub fn cg3(&mut self) -> CG3_W {
        CG3_W { w: self }
    }
    #[doc = "Bits 8:9 - can3_serial_clk_enable"]
    #[inline(always)]
    pub fn cg4(&mut self) -> CG4_W {
        CG4_W { w: self }
    }
    #[doc = "Bits 10:11 - aips_lite_clk_enable"]
    #[inline(always)]
    pub fn cg5(&mut self) -> CG5_W {
        CG5_W { w: self }
    }
    #[doc = "Bits 12:13 - flexio3_clk_enable"]
    #[inline(always)]
    pub fn cg6(&mut self) -> CG6_W {
        CG6_W { w: self }
    }
}
