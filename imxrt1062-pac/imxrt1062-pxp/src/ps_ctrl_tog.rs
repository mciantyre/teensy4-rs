#[doc = "Reader of register PS_CTRL_TOG"]
pub type R = crate::R<u32, super::PS_CTRL_TOG>;
#[doc = "Writer for register PS_CTRL_TOG"]
pub type W = crate::W<u32, super::PS_CTRL_TOG>;
#[doc = "Register PS_CTRL_TOG `reset()`'s with value 0"]
impl crate::ResetValue for super::PS_CTRL_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "4: 32-bit pixels (unpacked 24-bit format)"]
    RGB888 = 4,
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
    #[doc = "30: 16-bit pixels (3-plane format)"]
    YUV422 = 30,
    #[doc = "31: 16-bit pixels (3-plane format)"]
    YUV420 = 31,
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
            4 => Val(FORMAT_A::RGB888),
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
            30 => Val(FORMAT_A::YUV422),
            31 => Val(FORMAT_A::YUV420),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == FORMAT_A::RGB888
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
    #[doc = "Checks if the value of the field is `YUV422`"]
    #[inline(always)]
    pub fn is_yuv422(&self) -> bool {
        *self == FORMAT_A::YUV422
    }
    #[doc = "Checks if the value of the field is `YUV420`"]
    #[inline(always)]
    pub fn is_yuv420(&self) -> bool {
        *self == FORMAT_A::YUV420
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
    #[doc = "32-bit pixels (unpacked 24-bit format)"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB888)
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
    #[doc = "16-bit pixels (3-plane format)"]
    #[inline(always)]
    pub fn yuv422(self) -> &'a mut W {
        self.variant(FORMAT_A::YUV422)
    }
    #[doc = "16-bit pixels (3-plane format)"]
    #[inline(always)]
    pub fn yuv420(self) -> &'a mut W {
        self.variant(FORMAT_A::YUV420)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `WB_SWAP`"]
pub type WB_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WB_SWAP`"]
pub struct WB_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> WB_SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Verticle pre decimation filter control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DECY_A {
    #[doc = "0: Disable pre-decimation filter."]
    DISABLE = 0,
    #[doc = "1: Decimate PS by 2."]
    DECY2 = 1,
    #[doc = "2: Decimate PS by 4."]
    DECY4 = 2,
    #[doc = "3: Decimate PS by 8."]
    DECY8 = 3,
}
impl From<DECY_A> for u8 {
    #[inline(always)]
    fn from(variant: DECY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DECY`"]
pub type DECY_R = crate::R<u8, DECY_A>;
impl DECY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECY_A {
        match self.bits {
            0 => DECY_A::DISABLE,
            1 => DECY_A::DECY2,
            2 => DECY_A::DECY4,
            3 => DECY_A::DECY8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DECY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DECY2`"]
    #[inline(always)]
    pub fn is_decy2(&self) -> bool {
        *self == DECY_A::DECY2
    }
    #[doc = "Checks if the value of the field is `DECY4`"]
    #[inline(always)]
    pub fn is_decy4(&self) -> bool {
        *self == DECY_A::DECY4
    }
    #[doc = "Checks if the value of the field is `DECY8`"]
    #[inline(always)]
    pub fn is_decy8(&self) -> bool {
        *self == DECY_A::DECY8
    }
}
#[doc = "Write proxy for field `DECY`"]
pub struct DECY_W<'a> {
    w: &'a mut W,
}
impl<'a> DECY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable pre-decimation filter."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DECY_A::DISABLE)
    }
    #[doc = "Decimate PS by 2."]
    #[inline(always)]
    pub fn decy2(self) -> &'a mut W {
        self.variant(DECY_A::DECY2)
    }
    #[doc = "Decimate PS by 4."]
    #[inline(always)]
    pub fn decy4(self) -> &'a mut W {
        self.variant(DECY_A::DECY4)
    }
    #[doc = "Decimate PS by 8."]
    #[inline(always)]
    pub fn decy8(self) -> &'a mut W {
        self.variant(DECY_A::DECY8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Horizontal pre decimation filter control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DECX_A {
    #[doc = "0: Disable pre-decimation filter."]
    DISABLE = 0,
    #[doc = "1: Decimate PS by 2."]
    DECX2 = 1,
    #[doc = "2: Decimate PS by 4."]
    DECX4 = 2,
    #[doc = "3: Decimate PS by 8."]
    DECX8 = 3,
}
impl From<DECX_A> for u8 {
    #[inline(always)]
    fn from(variant: DECX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DECX`"]
pub type DECX_R = crate::R<u8, DECX_A>;
impl DECX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECX_A {
        match self.bits {
            0 => DECX_A::DISABLE,
            1 => DECX_A::DECX2,
            2 => DECX_A::DECX4,
            3 => DECX_A::DECX8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DECX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DECX2`"]
    #[inline(always)]
    pub fn is_decx2(&self) -> bool {
        *self == DECX_A::DECX2
    }
    #[doc = "Checks if the value of the field is `DECX4`"]
    #[inline(always)]
    pub fn is_decx4(&self) -> bool {
        *self == DECX_A::DECX4
    }
    #[doc = "Checks if the value of the field is `DECX8`"]
    #[inline(always)]
    pub fn is_decx8(&self) -> bool {
        *self == DECX_A::DECX8
    }
}
#[doc = "Write proxy for field `DECX`"]
pub struct DECX_W<'a> {
    w: &'a mut W,
}
impl<'a> DECX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable pre-decimation filter."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DECX_A::DISABLE)
    }
    #[doc = "Decimate PS by 2."]
    #[inline(always)]
    pub fn decx2(self) -> &'a mut W {
        self.variant(DECX_A::DECX2)
    }
    #[doc = "Decimate PS by 4."]
    #[inline(always)]
    pub fn decx4(self) -> &'a mut W {
        self.variant(DECX_A::DECX4)
    }
    #[doc = "Decimate PS by 8."]
    #[inline(always)]
    pub fn decx8(self) -> &'a mut W {
        self.variant(DECX_A::DECX8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:4 - PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[inline(always)]
    pub fn wb_swap(&self) -> WB_SWAP_R {
        WB_SWAP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Verticle pre decimation filter control."]
    #[inline(always)]
    pub fn decy(&self) -> DECY_R {
        DECY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Horizontal pre decimation filter control."]
    #[inline(always)]
    pub fn decx(&self) -> DECX_R {
        DECX_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:4 - PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Bit 5 - Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[inline(always)]
    pub fn wb_swap(&mut self) -> WB_SWAP_W {
        WB_SWAP_W { w: self }
    }
    #[doc = "Bits 8:9 - Verticle pre decimation filter control."]
    #[inline(always)]
    pub fn decy(&mut self) -> DECY_W {
        DECY_W { w: self }
    }
    #[doc = "Bits 10:11 - Horizontal pre decimation filter control."]
    #[inline(always)]
    pub fn decx(&mut self) -> DECX_W {
        DECX_W { w: self }
    }
}
