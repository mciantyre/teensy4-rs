#[doc = "Reader of register CCGR4"]
pub type R = crate::R<u32, super::CCGR4>;
#[doc = "Writer for register CCGR4"]
pub type W = crate::W<u32, super::CCGR4>;
#[doc = "Register CCGR4 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CCGR4 {
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
#[doc = "Reader of field `CG7`"]
pub type CG7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG7`"]
pub struct CG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `CG8`"]
pub type CG8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG8`"]
pub struct CG8_W<'a> {
    w: &'a mut W,
}
impl<'a> CG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CG9`"]
pub type CG9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG9`"]
pub struct CG9_W<'a> {
    w: &'a mut W,
}
impl<'a> CG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `CG10`"]
pub type CG10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG10`"]
pub struct CG10_W<'a> {
    w: &'a mut W,
}
impl<'a> CG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `CG11`"]
pub type CG11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG11`"]
pub struct CG11_W<'a> {
    w: &'a mut W,
}
impl<'a> CG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `CG12`"]
pub type CG12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG12`"]
pub struct CG12_W<'a> {
    w: &'a mut W,
}
impl<'a> CG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CG13`"]
pub type CG13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG13`"]
pub struct CG13_W<'a> {
    w: &'a mut W,
}
impl<'a> CG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `CG14`"]
pub type CG14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG14`"]
pub struct CG14_W<'a> {
    w: &'a mut W,
}
impl<'a> CG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `CG15`"]
pub type CG15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CG15`"]
pub struct CG15_W<'a> {
    w: &'a mut W,
}
impl<'a> CG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - sim_m7 register access clock (sim_m7_mainclk_r_enable)"]
    #[inline(always)]
    pub fn cg0(&self) -> CG0_R {
        CG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - iomuxc clock (iomuxc_clk_enable)"]
    #[inline(always)]
    pub fn cg1(&self) -> CG1_R {
        CG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    #[inline(always)]
    pub fn cg2(&self) -> CG2_R {
        CG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - bee clock(bee_clk_enable)"]
    #[inline(always)]
    pub fn cg3(&self) -> CG3_R {
        CG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - sim_m7 clock (sim_m7_clk_enable)"]
    #[inline(always)]
    pub fn cg4(&self) -> CG4_R {
        CG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - tsc_dig clock (tsc_clk_enable)"]
    #[inline(always)]
    pub fn cg5(&self) -> CG5_R {
        CG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - sim_m clocks (sim_m_clk_enable)"]
    #[inline(always)]
    pub fn cg6(&self) -> CG6_R {
        CG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - sim_ems clocks (sim_ems_clk_enable)"]
    #[inline(always)]
    pub fn cg7(&self) -> CG7_R {
        CG7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - pwm1 clocks (pwm1_clk_enable)"]
    #[inline(always)]
    pub fn cg8(&self) -> CG8_R {
        CG8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - pwm2 clocks (pwm2_clk_enable)"]
    #[inline(always)]
    pub fn cg9(&self) -> CG9_R {
        CG9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - pwm3 clocks (pwm3_clk_enable)"]
    #[inline(always)]
    pub fn cg10(&self) -> CG10_R {
        CG10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - pwm4 clocks (pwm4_clk_enable)"]
    #[inline(always)]
    pub fn cg11(&self) -> CG11_R {
        CG11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - enc1 clocks (enc1_clk_enable)"]
    #[inline(always)]
    pub fn cg12(&self) -> CG12_R {
        CG12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - enc2 clocks (enc2_clk_enable)"]
    #[inline(always)]
    pub fn cg13(&self) -> CG13_R {
        CG13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - enc3 clocks (enc3_clk_enable)"]
    #[inline(always)]
    pub fn cg14(&self) -> CG14_R {
        CG14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - enc4 clocks (enc4_clk_enable)"]
    #[inline(always)]
    pub fn cg15(&self) -> CG15_R {
        CG15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - sim_m7 register access clock (sim_m7_mainclk_r_enable)"]
    #[inline(always)]
    pub fn cg0(&mut self) -> CG0_W {
        CG0_W { w: self }
    }
    #[doc = "Bits 2:3 - iomuxc clock (iomuxc_clk_enable)"]
    #[inline(always)]
    pub fn cg1(&mut self) -> CG1_W {
        CG1_W { w: self }
    }
    #[doc = "Bits 4:5 - iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    #[inline(always)]
    pub fn cg2(&mut self) -> CG2_W {
        CG2_W { w: self }
    }
    #[doc = "Bits 6:7 - bee clock(bee_clk_enable)"]
    #[inline(always)]
    pub fn cg3(&mut self) -> CG3_W {
        CG3_W { w: self }
    }
    #[doc = "Bits 8:9 - sim_m7 clock (sim_m7_clk_enable)"]
    #[inline(always)]
    pub fn cg4(&mut self) -> CG4_W {
        CG4_W { w: self }
    }
    #[doc = "Bits 10:11 - tsc_dig clock (tsc_clk_enable)"]
    #[inline(always)]
    pub fn cg5(&mut self) -> CG5_W {
        CG5_W { w: self }
    }
    #[doc = "Bits 12:13 - sim_m clocks (sim_m_clk_enable)"]
    #[inline(always)]
    pub fn cg6(&mut self) -> CG6_W {
        CG6_W { w: self }
    }
    #[doc = "Bits 14:15 - sim_ems clocks (sim_ems_clk_enable)"]
    #[inline(always)]
    pub fn cg7(&mut self) -> CG7_W {
        CG7_W { w: self }
    }
    #[doc = "Bits 16:17 - pwm1 clocks (pwm1_clk_enable)"]
    #[inline(always)]
    pub fn cg8(&mut self) -> CG8_W {
        CG8_W { w: self }
    }
    #[doc = "Bits 18:19 - pwm2 clocks (pwm2_clk_enable)"]
    #[inline(always)]
    pub fn cg9(&mut self) -> CG9_W {
        CG9_W { w: self }
    }
    #[doc = "Bits 20:21 - pwm3 clocks (pwm3_clk_enable)"]
    #[inline(always)]
    pub fn cg10(&mut self) -> CG10_W {
        CG10_W { w: self }
    }
    #[doc = "Bits 22:23 - pwm4 clocks (pwm4_clk_enable)"]
    #[inline(always)]
    pub fn cg11(&mut self) -> CG11_W {
        CG11_W { w: self }
    }
    #[doc = "Bits 24:25 - enc1 clocks (enc1_clk_enable)"]
    #[inline(always)]
    pub fn cg12(&mut self) -> CG12_W {
        CG12_W { w: self }
    }
    #[doc = "Bits 26:27 - enc2 clocks (enc2_clk_enable)"]
    #[inline(always)]
    pub fn cg13(&mut self) -> CG13_W {
        CG13_W { w: self }
    }
    #[doc = "Bits 28:29 - enc3 clocks (enc3_clk_enable)"]
    #[inline(always)]
    pub fn cg14(&mut self) -> CG14_W {
        CG14_W { w: self }
    }
    #[doc = "Bits 30:31 - enc4 clocks (enc4_clk_enable)"]
    #[inline(always)]
    pub fn cg15(&mut self) -> CG15_W {
        CG15_W { w: self }
    }
}
