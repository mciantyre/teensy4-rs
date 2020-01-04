#[doc = "Reader of register OUT_CTRL"]
pub type R = crate::R<u32, super::OUT_CTRL>;
#[doc = "Writer for register OUT_CTRL"]
pub type W = crate::W<u32, super::OUT_CTRL>;
#[doc = "Register OUT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output framebuffer format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: 32-bit pixels"]
    ARGB8888 = 0,
    #[doc = "4: 32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    RGB888 = 4,
    #[doc = "5: 24-bit pixels (packed 24-bit format)"]
    RGB888P = 5,
    #[doc = "8: 16-bit pixels"]
    ARGB1555 = 8,
    #[doc = "9: 16-bit pixels"]
    ARGB4444 = 9,
    #[doc = "12: 16-bit pixels"]
    RGB555 = 12,
    #[doc = "13: 16-bit pixels"]
    RGB444 = 13,
    #[doc = "14: 16-bit pixels"]
    RGB565 = 14,
    #[doc = "16: 32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 16,
    #[doc = "18: 16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 18,
    #[doc = "19: 16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 19,
    #[doc = "20: 8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 20,
    #[doc = "21: 4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 21,
    #[doc = "24: 16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 24,
    #[doc = "25: 16-bit pixels (2-plane UV)"]
    YUV2P420 = 25,
    #[doc = "26: 16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 26,
    #[doc = "27: 16-bit pixels (2-plane VU)"]
    YVU2P420 = 27,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORMAT`"]
pub type FORMAT_R = crate::R<u8, FORMAT_A>;
impl FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FORMAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FORMAT_A::ARGB8888),
            4 => Val(FORMAT_A::RGB888),
            5 => Val(FORMAT_A::RGB888P),
            8 => Val(FORMAT_A::ARGB1555),
            9 => Val(FORMAT_A::ARGB4444),
            12 => Val(FORMAT_A::RGB555),
            13 => Val(FORMAT_A::RGB444),
            14 => Val(FORMAT_A::RGB565),
            16 => Val(FORMAT_A::YUV1P444),
            18 => Val(FORMAT_A::UYVY1P422),
            19 => Val(FORMAT_A::VYUY1P422),
            20 => Val(FORMAT_A::Y8),
            21 => Val(FORMAT_A::Y4),
            24 => Val(FORMAT_A::YUV2P422),
            25 => Val(FORMAT_A::YUV2P420),
            26 => Val(FORMAT_A::YVU2P422),
            27 => Val(FORMAT_A::YVU2P420),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARGB8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == FORMAT_A::ARGB8888
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == FORMAT_A::RGB888
    }
    #[doc = "Checks if the value of the field is `RGB888P`"]
    #[inline(always)]
    pub fn is_rgb888p(&self) -> bool {
        *self == FORMAT_A::RGB888P
    }
    #[doc = "Checks if the value of the field is `ARGB1555`"]
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == FORMAT_A::ARGB1555
    }
    #[doc = "Checks if the value of the field is `ARGB4444`"]
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == FORMAT_A::ARGB4444
    }
    #[doc = "Checks if the value of the field is `RGB555`"]
    #[inline(always)]
    pub fn is_rgb555(&self) -> bool {
        *self == FORMAT_A::RGB555
    }
    #[doc = "Checks if the value of the field is `RGB444`"]
    #[inline(always)]
    pub fn is_rgb444(&self) -> bool {
        *self == FORMAT_A::RGB444
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == FORMAT_A::RGB565
    }
    #[doc = "Checks if the value of the field is `YUV1P444`"]
    #[inline(always)]
    pub fn is_yuv1p444(&self) -> bool {
        *self == FORMAT_A::YUV1P444
    }
    #[doc = "Checks if the value of the field is `UYVY1P422`"]
    #[inline(always)]
    pub fn is_uyvy1p422(&self) -> bool {
        *self == FORMAT_A::UYVY1P422
    }
    #[doc = "Checks if the value of the field is `VYUY1P422`"]
    #[inline(always)]
    pub fn is_vyuy1p422(&self) -> bool {
        *self == FORMAT_A::VYUY1P422
    }
    #[doc = "Checks if the value of the field is `Y8`"]
    #[inline(always)]
    pub fn is_y8(&self) -> bool {
        *self == FORMAT_A::Y8
    }
    #[doc = "Checks if the value of the field is `Y4`"]
    #[inline(always)]
    pub fn is_y4(&self) -> bool {
        *self == FORMAT_A::Y4
    }
    #[doc = "Checks if the value of the field is `YUV2P422`"]
    #[inline(always)]
    pub fn is_yuv2p422(&self) -> bool {
        *self == FORMAT_A::YUV2P422
    }
    #[doc = "Checks if the value of the field is `YUV2P420`"]
    #[inline(always)]
    pub fn is_yuv2p420(&self) -> bool {
        *self == FORMAT_A::YUV2P420
    }
    #[doc = "Checks if the value of the field is `YVU2P422`"]
    #[inline(always)]
    pub fn is_yvu2p422(&self) -> bool {
        *self == FORMAT_A::YVU2P422
    }
    #[doc = "Checks if the value of the field is `YVU2P420`"]
    #[inline(always)]
    pub fn is_yvu2p420(&self) -> bool {
        *self == FORMAT_A::YVU2P420
    }
}
#[doc = "Write proxy for field `FORMAT`"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORMAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32-bit pixels"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(FORMAT_A::ARGB8888)
    }
    #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB888)
    }
    #[doc = "24-bit pixels (packed 24-bit format)"]
    #[inline(always)]
    pub fn rgb888p(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB888P)
    }
    #[doc = "16-bit pixels"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(FORMAT_A::ARGB1555)
    }
    #[doc = "16-bit pixels"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(FORMAT_A::ARGB4444)
    }
    #[doc = "16-bit pixels"]
    #[inline(always)]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB555)
    }
    #[doc = "16-bit pixels"]
    #[inline(always)]
    pub fn rgb444(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB444)
    }
    #[doc = "16-bit pixels"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB565)
    }
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    #[inline(always)]
    pub fn yuv1p444(self) -> &'a mut W {
        self.variant(FORMAT_A::YUV1P444)
    }
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    #[inline(always)]
    pub fn uyvy1p422(self) -> &'a mut W {
        self.variant(FORMAT_A::UYVY1P422)
    }
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    #[inline(always)]
    pub fn vyuy1p422(self) -> &'a mut W {
        self.variant(FORMAT_A::VYUY1P422)
    }
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    #[inline(always)]
    pub fn y8(self) -> &'a mut W {
        self.variant(FORMAT_A::Y8)
    }
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    #[inline(always)]
    pub fn y4(self) -> &'a mut W {
        self.variant(FORMAT_A::Y4)
    }
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    #[inline(always)]
    pub fn yuv2p422(self) -> &'a mut W {
        self.variant(FORMAT_A::YUV2P422)
    }
    #[doc = "16-bit pixels (2-plane UV)"]
    #[inline(always)]
    pub fn yuv2p420(self) -> &'a mut W {
        self.variant(FORMAT_A::YUV2P420)
    }
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    #[inline(always)]
    pub fn yvu2p422(self) -> &'a mut W {
        self.variant(FORMAT_A::YVU2P422)
    }
    #[doc = "16-bit pixels (2-plane VU)"]
    #[inline(always)]
    pub fn yvu2p420(self) -> &'a mut W {
        self.variant(FORMAT_A::YVU2P420)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Determines how the PXP writes it's output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTERLACED_OUTPUT_A {
    #[doc = "0: All data written in progressive format to the OUTBUF Pointer."]
    PROGRESSIVE = 0,
    #[doc = "1: Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    FIELD0 = 1,
    #[doc = "2: Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    FIELD1 = 2,
    #[doc = "3: Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    INTERLACED = 3,
}
impl From<INTERLACED_OUTPUT_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERLACED_OUTPUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTERLACED_OUTPUT`"]
pub type INTERLACED_OUTPUT_R = crate::R<u8, INTERLACED_OUTPUT_A>;
impl INTERLACED_OUTPUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERLACED_OUTPUT_A {
        match self.bits {
            0 => INTERLACED_OUTPUT_A::PROGRESSIVE,
            1 => INTERLACED_OUTPUT_A::FIELD0,
            2 => INTERLACED_OUTPUT_A::FIELD1,
            3 => INTERLACED_OUTPUT_A::INTERLACED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PROGRESSIVE`"]
    #[inline(always)]
    pub fn is_progressive(&self) -> bool {
        *self == INTERLACED_OUTPUT_A::PROGRESSIVE
    }
    #[doc = "Checks if the value of the field is `FIELD0`"]
    #[inline(always)]
    pub fn is_field0(&self) -> bool {
        *self == INTERLACED_OUTPUT_A::FIELD0
    }
    #[doc = "Checks if the value of the field is `FIELD1`"]
    #[inline(always)]
    pub fn is_field1(&self) -> bool {
        *self == INTERLACED_OUTPUT_A::FIELD1
    }
    #[doc = "Checks if the value of the field is `INTERLACED`"]
    #[inline(always)]
    pub fn is_interlaced(&self) -> bool {
        *self == INTERLACED_OUTPUT_A::INTERLACED
    }
}
#[doc = "Write proxy for field `INTERLACED_OUTPUT`"]
pub struct INTERLACED_OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERLACED_OUTPUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERLACED_OUTPUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "All data written in progressive format to the OUTBUF Pointer."]
    #[inline(always)]
    pub fn progressive(self) -> &'a mut W {
        self.variant(INTERLACED_OUTPUT_A::PROGRESSIVE)
    }
    #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    #[inline(always)]
    pub fn field0(self) -> &'a mut W {
        self.variant(INTERLACED_OUTPUT_A::FIELD0)
    }
    #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    #[inline(always)]
    pub fn field1(self) -> &'a mut W {
        self.variant(INTERLACED_OUTPUT_A::FIELD1)
    }
    #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    #[inline(always)]
    pub fn interlaced(self) -> &'a mut W {
        self.variant(INTERLACED_OUTPUT_A::INTERLACED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u16, u16>;
#[doc = "Reader of field `ALPHA_OUTPUT`"]
pub type ALPHA_OUTPUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALPHA_OUTPUT`"]
pub struct ALPHA_OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALPHA_OUTPUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ALPHA`"]
pub type ALPHA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALPHA`"]
pub struct ALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> ALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Output framebuffer format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Determines how the PXP writes it's output data"]
    #[inline(always)]
    pub fn interlaced_output(&self) -> INTERLACED_OUTPUT_R {
        INTERLACED_OUTPUT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:22 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 10) & 0x1fff) as u16)
    }
    #[doc = "Bit 23 - Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[inline(always)]
    pub fn alpha_output(&self) -> ALPHA_OUTPUT_R {
        ALPHA_OUTPUT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Output framebuffer format"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Bits 8:9 - Determines how the PXP writes it's output data"]
    #[inline(always)]
    pub fn interlaced_output(&mut self) -> INTERLACED_OUTPUT_W {
        INTERLACED_OUTPUT_W { w: self }
    }
    #[doc = "Bit 23 - Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[inline(always)]
    pub fn alpha_output(&mut self) -> ALPHA_OUTPUT_W {
        ALPHA_OUTPUT_W { w: self }
    }
    #[doc = "Bits 24:31 - When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W {
        ALPHA_W { w: self }
    }
}
