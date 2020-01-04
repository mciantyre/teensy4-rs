#[doc = "Reader of register CTRL2_SET"]
pub type R = crate::R<u32, super::CTRL2_SET>;
#[doc = "Writer for register CTRL2_SET"]
pub type W = crate::W<u32, super::CTRL2_SET>;
#[doc = "Register CTRL2_SET `reset()`'s with value 0x0020_0000"]
impl crate::ResetValue for super::CTRL2_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0020_0000
    }
}
#[doc = "This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EVEN_LINE_PATTERN_A {
    #[doc = "0: RGB"]
    RGB = 0,
    #[doc = "1: RBG"]
    RBG = 1,
    #[doc = "2: GBR"]
    GBR = 2,
    #[doc = "3: GRB"]
    GRB = 3,
    #[doc = "4: BRG"]
    BRG = 4,
    #[doc = "5: BGR"]
    BGR = 5,
}
impl From<EVEN_LINE_PATTERN_A> for u8 {
    #[inline(always)]
    fn from(variant: EVEN_LINE_PATTERN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EVEN_LINE_PATTERN`"]
pub type EVEN_LINE_PATTERN_R = crate::R<u8, EVEN_LINE_PATTERN_A>;
impl EVEN_LINE_PATTERN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EVEN_LINE_PATTERN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EVEN_LINE_PATTERN_A::RGB),
            1 => Val(EVEN_LINE_PATTERN_A::RBG),
            2 => Val(EVEN_LINE_PATTERN_A::GBR),
            3 => Val(EVEN_LINE_PATTERN_A::GRB),
            4 => Val(EVEN_LINE_PATTERN_A::BRG),
            5 => Val(EVEN_LINE_PATTERN_A::BGR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == EVEN_LINE_PATTERN_A::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        *self == EVEN_LINE_PATTERN_A::RBG
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        *self == EVEN_LINE_PATTERN_A::GBR
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        *self == EVEN_LINE_PATTERN_A::GRB
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        *self == EVEN_LINE_PATTERN_A::BRG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        *self == EVEN_LINE_PATTERN_A::BGR
    }
}
#[doc = "Write proxy for field `EVEN_LINE_PATTERN`"]
pub struct EVEN_LINE_PATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVEN_LINE_PATTERN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVEN_LINE_PATTERN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERN_A::RGB)
    }
    #[doc = "RBG"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERN_A::RBG)
    }
    #[doc = "GBR"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERN_A::GBR)
    }
    #[doc = "GRB"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERN_A::GRB)
    }
    #[doc = "BRG"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERN_A::BRG)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERN_A::BGR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ODD_LINE_PATTERN_A {
    #[doc = "0: RGB"]
    RGB = 0,
    #[doc = "1: RBG"]
    RBG = 1,
    #[doc = "2: GBR"]
    GBR = 2,
    #[doc = "3: GRB"]
    GRB = 3,
    #[doc = "4: BRG"]
    BRG = 4,
    #[doc = "5: BGR"]
    BGR = 5,
}
impl From<ODD_LINE_PATTERN_A> for u8 {
    #[inline(always)]
    fn from(variant: ODD_LINE_PATTERN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ODD_LINE_PATTERN`"]
pub type ODD_LINE_PATTERN_R = crate::R<u8, ODD_LINE_PATTERN_A>;
impl ODD_LINE_PATTERN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ODD_LINE_PATTERN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ODD_LINE_PATTERN_A::RGB),
            1 => Val(ODD_LINE_PATTERN_A::RBG),
            2 => Val(ODD_LINE_PATTERN_A::GBR),
            3 => Val(ODD_LINE_PATTERN_A::GRB),
            4 => Val(ODD_LINE_PATTERN_A::BRG),
            5 => Val(ODD_LINE_PATTERN_A::BGR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == ODD_LINE_PATTERN_A::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        *self == ODD_LINE_PATTERN_A::RBG
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        *self == ODD_LINE_PATTERN_A::GBR
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        *self == ODD_LINE_PATTERN_A::GRB
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        *self == ODD_LINE_PATTERN_A::BRG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        *self == ODD_LINE_PATTERN_A::BGR
    }
}
#[doc = "Write proxy for field `ODD_LINE_PATTERN`"]
pub struct ODD_LINE_PATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODD_LINE_PATTERN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODD_LINE_PATTERN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERN_A::RGB)
    }
    #[doc = "RBG"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERN_A::RBG)
    }
    #[doc = "GBR"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERN_A::GBR)
    }
    #[doc = "GRB"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERN_A::GRB)
    }
    #[doc = "BRG"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERN_A::BRG)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERN_A::BGR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `BURST_LEN_8`"]
pub type BURST_LEN_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURST_LEN_8`"]
pub struct BURST_LEN_8_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_LEN_8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTSTANDING_REQS_A {
    #[doc = "0: REQ_1"]
    REQ_1 = 0,
    #[doc = "1: REQ_2"]
    REQ_2 = 1,
    #[doc = "2: REQ_4"]
    REQ_4 = 2,
    #[doc = "3: REQ_8"]
    REQ_8 = 3,
    #[doc = "4: REQ_16"]
    REQ_16 = 4,
}
impl From<OUTSTANDING_REQS_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTSTANDING_REQS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTSTANDING_REQS`"]
pub type OUTSTANDING_REQS_R = crate::R<u8, OUTSTANDING_REQS_A>;
impl OUTSTANDING_REQS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OUTSTANDING_REQS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OUTSTANDING_REQS_A::REQ_1),
            1 => Val(OUTSTANDING_REQS_A::REQ_2),
            2 => Val(OUTSTANDING_REQS_A::REQ_4),
            3 => Val(OUTSTANDING_REQS_A::REQ_8),
            4 => Val(OUTSTANDING_REQS_A::REQ_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REQ_1`"]
    #[inline(always)]
    pub fn is_req_1(&self) -> bool {
        *self == OUTSTANDING_REQS_A::REQ_1
    }
    #[doc = "Checks if the value of the field is `REQ_2`"]
    #[inline(always)]
    pub fn is_req_2(&self) -> bool {
        *self == OUTSTANDING_REQS_A::REQ_2
    }
    #[doc = "Checks if the value of the field is `REQ_4`"]
    #[inline(always)]
    pub fn is_req_4(&self) -> bool {
        *self == OUTSTANDING_REQS_A::REQ_4
    }
    #[doc = "Checks if the value of the field is `REQ_8`"]
    #[inline(always)]
    pub fn is_req_8(&self) -> bool {
        *self == OUTSTANDING_REQS_A::REQ_8
    }
    #[doc = "Checks if the value of the field is `REQ_16`"]
    #[inline(always)]
    pub fn is_req_16(&self) -> bool {
        *self == OUTSTANDING_REQS_A::REQ_16
    }
}
#[doc = "Write proxy for field `OUTSTANDING_REQS`"]
pub struct OUTSTANDING_REQS_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSTANDING_REQS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSTANDING_REQS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "REQ_1"]
    #[inline(always)]
    pub fn req_1(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQS_A::REQ_1)
    }
    #[doc = "REQ_2"]
    #[inline(always)]
    pub fn req_2(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQS_A::REQ_2)
    }
    #[doc = "REQ_4"]
    #[inline(always)]
    pub fn req_4(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQS_A::REQ_4)
    }
    #[doc = "REQ_8"]
    #[inline(always)]
    pub fn req_8(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQS_A::REQ_8)
    }
    #[doc = "REQ_16"]
    #[inline(always)]
    pub fn req_16(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQS_A::REQ_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:14 - This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[inline(always)]
    pub fn even_line_pattern(&self) -> EVEN_LINE_PATTERN_R {
        EVEN_LINE_PATTERN_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[inline(always)]
    pub fn odd_line_pattern(&self) -> ODD_LINE_PATTERN_R {
        ODD_LINE_PATTERN_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[inline(always)]
    pub fn burst_len_8(&self) -> BURST_LEN_8_R {
        BURST_LEN_8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:23 - This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[inline(always)]
    pub fn outstanding_reqs(&self) -> OUTSTANDING_REQS_R {
        OUTSTANDING_REQS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[inline(always)]
    pub fn even_line_pattern(&mut self) -> EVEN_LINE_PATTERN_W {
        EVEN_LINE_PATTERN_W { w: self }
    }
    #[doc = "Bits 16:18 - This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[inline(always)]
    pub fn odd_line_pattern(&mut self) -> ODD_LINE_PATTERN_W {
        ODD_LINE_PATTERN_W { w: self }
    }
    #[doc = "Bit 20 - By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[inline(always)]
    pub fn burst_len_8(&mut self) -> BURST_LEN_8_W {
        BURST_LEN_8_W { w: self }
    }
    #[doc = "Bits 21:23 - This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[inline(always)]
    pub fn outstanding_reqs(&mut self) -> OUTSTANDING_REQS_W {
        OUTSTANDING_REQS_W { w: self }
    }
}
