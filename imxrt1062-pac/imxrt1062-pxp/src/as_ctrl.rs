#[doc = "Reader of register AS_CTRL"]
pub type R = crate::R<u32, super::AS_CTRL>;
#[doc = "Writer for register AS_CTRL"]
pub type W = crate::W<u32, super::AS_CTRL>;
#[doc = "Register AS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::AS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<bool, bool>;
#[doc = "Determines how the alpha value is constructed for this alpha surface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALPHA_CTRL_A {
    #[doc = "0: Indicates that the AS pixel alpha value will be used to blend the AS with PS. The ALPHA field is ignored."]
    EMBEDDED = 0,
    #[doc = "1: Indicates that the value in the ALPHA field should be used instead of the alpha values present in the input pixels."]
    OVERRIDE = 1,
    #[doc = "2: Indicates that the value in the ALPHA field should be used to scale all pixel alpha values. Each pixel alpha is multiplied by the value in the ALPHA field."]
    MULTIPLY = 2,
    #[doc = "3: Enable ROPs. The ROP field indicates an operation to be performed on the alpha surface and PS pixels."]
    ROPS = 3,
}
impl From<ALPHA_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: ALPHA_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALPHA_CTRL`"]
pub type ALPHA_CTRL_R = crate::R<u8, ALPHA_CTRL_A>;
impl ALPHA_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALPHA_CTRL_A {
        match self.bits {
            0 => ALPHA_CTRL_A::EMBEDDED,
            1 => ALPHA_CTRL_A::OVERRIDE,
            2 => ALPHA_CTRL_A::MULTIPLY,
            3 => ALPHA_CTRL_A::ROPS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMBEDDED`"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == ALPHA_CTRL_A::EMBEDDED
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override_(&self) -> bool {
        *self == ALPHA_CTRL_A::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `MULTIPLY`"]
    #[inline(always)]
    pub fn is_multiply(&self) -> bool {
        *self == ALPHA_CTRL_A::MULTIPLY
    }
    #[doc = "Checks if the value of the field is `ROPS`"]
    #[inline(always)]
    pub fn is_rops(&self) -> bool {
        *self == ALPHA_CTRL_A::ROPS
    }
}
#[doc = "Write proxy for field `ALPHA_CTRL`"]
pub struct ALPHA_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALPHA_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALPHA_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Indicates that the AS pixel alpha value will be used to blend the AS with PS. The ALPHA field is ignored."]
    #[inline(always)]
    pub fn embedded(self) -> &'a mut W {
        self.variant(ALPHA_CTRL_A::EMBEDDED)
    }
    #[doc = "Indicates that the value in the ALPHA field should be used instead of the alpha values present in the input pixels."]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(ALPHA_CTRL_A::OVERRIDE)
    }
    #[doc = "Indicates that the value in the ALPHA field should be used to scale all pixel alpha values. Each pixel alpha is multiplied by the value in the ALPHA field."]
    #[inline(always)]
    pub fn multiply(self) -> &'a mut W {
        self.variant(ALPHA_CTRL_A::MULTIPLY)
    }
    #[doc = "Enable ROPs. The ROP field indicates an operation to be performed on the alpha surface and PS pixels."]
    #[inline(always)]
    pub fn rops(self) -> &'a mut W {
        self.variant(ALPHA_CTRL_A::ROPS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_COLORKEY`"]
pub type ENABLE_COLORKEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_COLORKEY`"]
pub struct ENABLE_COLORKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_COLORKEY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Indicates the input buffer format for AS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: 32-bit pixels with alpha"]
    ARGB8888 = 0,
    #[doc = "4: 32-bit pixels without alpha (unpacked 24-bit format)"]
    RGB888 = 4,
    #[doc = "8: 16-bit pixels with alpha"]
    ARGB1555 = 8,
    #[doc = "9: 16-bit pixels with alpha"]
    ARGB4444 = 9,
    #[doc = "12: 16-bit pixels without alpha"]
    RGB555 = 12,
    #[doc = "13: 16-bit pixels without alpha"]
    RGB444 = 13,
    #[doc = "14: 16-bit pixels without alpha"]
    RGB565 = 14,
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
            8 => Val(FORMAT_A::ARGB1555),
            9 => Val(FORMAT_A::ARGB4444),
            12 => Val(FORMAT_A::RGB555),
            13 => Val(FORMAT_A::RGB444),
            14 => Val(FORMAT_A::RGB565),
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
    #[doc = "32-bit pixels with alpha"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(FORMAT_A::ARGB8888)
    }
    #[doc = "32-bit pixels without alpha (unpacked 24-bit format)"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB888)
    }
    #[doc = "16-bit pixels with alpha"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(FORMAT_A::ARGB1555)
    }
    #[doc = "16-bit pixels with alpha"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(FORMAT_A::ARGB4444)
    }
    #[doc = "16-bit pixels without alpha"]
    #[inline(always)]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB555)
    }
    #[doc = "16-bit pixels without alpha"]
    #[inline(always)]
    pub fn rgb444(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB444)
    }
    #[doc = "16-bit pixels without alpha"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB565)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
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
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Indicates a raster operation to perform when enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ROP_A {
    #[doc = "0: AS AND PS"]
    MASKAS = 0,
    #[doc = "1: nAS AND PS"]
    MASKNOTAS = 1,
    #[doc = "2: AS AND nPS"]
    MASKASNOT = 2,
    #[doc = "3: AS OR PS"]
    MERGEAS = 3,
    #[doc = "4: nAS OR PS"]
    MERGENOTAS = 4,
    #[doc = "5: AS OR nPS"]
    MERGEASNOT = 5,
    #[doc = "6: nAS"]
    NOTCOPYAS = 6,
    #[doc = "7: nPS"]
    NOT = 7,
    #[doc = "8: AS NAND PS"]
    NOTMASKAS = 8,
    #[doc = "9: AS NOR PS"]
    NOTMERGEAS = 9,
    #[doc = "10: AS XOR PS"]
    XORAS = 10,
    #[doc = "11: AS XNOR PS"]
    NOTXORAS = 11,
}
impl From<ROP_A> for u8 {
    #[inline(always)]
    fn from(variant: ROP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ROP`"]
pub type ROP_R = crate::R<u8, ROP_A>;
impl ROP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ROP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ROP_A::MASKAS),
            1 => Val(ROP_A::MASKNOTAS),
            2 => Val(ROP_A::MASKASNOT),
            3 => Val(ROP_A::MERGEAS),
            4 => Val(ROP_A::MERGENOTAS),
            5 => Val(ROP_A::MERGEASNOT),
            6 => Val(ROP_A::NOTCOPYAS),
            7 => Val(ROP_A::NOT),
            8 => Val(ROP_A::NOTMASKAS),
            9 => Val(ROP_A::NOTMERGEAS),
            10 => Val(ROP_A::XORAS),
            11 => Val(ROP_A::NOTXORAS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKAS`"]
    #[inline(always)]
    pub fn is_maskas(&self) -> bool {
        *self == ROP_A::MASKAS
    }
    #[doc = "Checks if the value of the field is `MASKNOTAS`"]
    #[inline(always)]
    pub fn is_masknotas(&self) -> bool {
        *self == ROP_A::MASKNOTAS
    }
    #[doc = "Checks if the value of the field is `MASKASNOT`"]
    #[inline(always)]
    pub fn is_maskasnot(&self) -> bool {
        *self == ROP_A::MASKASNOT
    }
    #[doc = "Checks if the value of the field is `MERGEAS`"]
    #[inline(always)]
    pub fn is_mergeas(&self) -> bool {
        *self == ROP_A::MERGEAS
    }
    #[doc = "Checks if the value of the field is `MERGENOTAS`"]
    #[inline(always)]
    pub fn is_mergenotas(&self) -> bool {
        *self == ROP_A::MERGENOTAS
    }
    #[doc = "Checks if the value of the field is `MERGEASNOT`"]
    #[inline(always)]
    pub fn is_mergeasnot(&self) -> bool {
        *self == ROP_A::MERGEASNOT
    }
    #[doc = "Checks if the value of the field is `NOTCOPYAS`"]
    #[inline(always)]
    pub fn is_notcopyas(&self) -> bool {
        *self == ROP_A::NOTCOPYAS
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ROP_A::NOT
    }
    #[doc = "Checks if the value of the field is `NOTMASKAS`"]
    #[inline(always)]
    pub fn is_notmaskas(&self) -> bool {
        *self == ROP_A::NOTMASKAS
    }
    #[doc = "Checks if the value of the field is `NOTMERGEAS`"]
    #[inline(always)]
    pub fn is_notmergeas(&self) -> bool {
        *self == ROP_A::NOTMERGEAS
    }
    #[doc = "Checks if the value of the field is `XORAS`"]
    #[inline(always)]
    pub fn is_xoras(&self) -> bool {
        *self == ROP_A::XORAS
    }
    #[doc = "Checks if the value of the field is `NOTXORAS`"]
    #[inline(always)]
    pub fn is_notxoras(&self) -> bool {
        *self == ROP_A::NOTXORAS
    }
}
#[doc = "Write proxy for field `ROP`"]
pub struct ROP_W<'a> {
    w: &'a mut W,
}
impl<'a> ROP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AS AND PS"]
    #[inline(always)]
    pub fn maskas(self) -> &'a mut W {
        self.variant(ROP_A::MASKAS)
    }
    #[doc = "nAS AND PS"]
    #[inline(always)]
    pub fn masknotas(self) -> &'a mut W {
        self.variant(ROP_A::MASKNOTAS)
    }
    #[doc = "AS AND nPS"]
    #[inline(always)]
    pub fn maskasnot(self) -> &'a mut W {
        self.variant(ROP_A::MASKASNOT)
    }
    #[doc = "AS OR PS"]
    #[inline(always)]
    pub fn mergeas(self) -> &'a mut W {
        self.variant(ROP_A::MERGEAS)
    }
    #[doc = "nAS OR PS"]
    #[inline(always)]
    pub fn mergenotas(self) -> &'a mut W {
        self.variant(ROP_A::MERGENOTAS)
    }
    #[doc = "AS OR nPS"]
    #[inline(always)]
    pub fn mergeasnot(self) -> &'a mut W {
        self.variant(ROP_A::MERGEASNOT)
    }
    #[doc = "nAS"]
    #[inline(always)]
    pub fn notcopyas(self) -> &'a mut W {
        self.variant(ROP_A::NOTCOPYAS)
    }
    #[doc = "nPS"]
    #[inline(always)]
    pub fn not(self) -> &'a mut W {
        self.variant(ROP_A::NOT)
    }
    #[doc = "AS NAND PS"]
    #[inline(always)]
    pub fn notmaskas(self) -> &'a mut W {
        self.variant(ROP_A::NOTMASKAS)
    }
    #[doc = "AS NOR PS"]
    #[inline(always)]
    pub fn notmergeas(self) -> &'a mut W {
        self.variant(ROP_A::NOTMERGEAS)
    }
    #[doc = "AS XOR PS"]
    #[inline(always)]
    pub fn xoras(self) -> &'a mut W {
        self.variant(ROP_A::XORAS)
    }
    #[doc = "AS XNOR PS"]
    #[inline(always)]
    pub fn notxoras(self) -> &'a mut W {
        self.variant(ROP_A::NOTXORAS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ALPHA_INVERT`"]
pub type ALPHA_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALPHA_INVERT`"]
pub struct ALPHA_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALPHA_INVERT_W<'a> {
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
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Determines how the alpha value is constructed for this alpha surface"]
    #[inline(always)]
    pub fn alpha_ctrl(&self) -> ALPHA_CTRL_R {
        ALPHA_CTRL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Indicates that colorkey functionality is enabled for this alpha surface"]
    #[inline(always)]
    pub fn enable_colorkey(&self) -> ENABLE_COLORKEY_R {
        ENABLE_COLORKEY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Indicates the input buffer format for AS."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in PXP_AS_CTRL\\[ALPHA_CTRL\\]"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Indicates a raster operation to perform when enabled"]
    #[inline(always)]
    pub fn rop(&self) -> ROP_R {
        ROP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Setting this bit to logic 0 will not alter the alpha value"]
    #[inline(always)]
    pub fn alpha_invert(&self) -> ALPHA_INVERT_R {
        ALPHA_INVERT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:2 - Determines how the alpha value is constructed for this alpha surface"]
    #[inline(always)]
    pub fn alpha_ctrl(&mut self) -> ALPHA_CTRL_W {
        ALPHA_CTRL_W { w: self }
    }
    #[doc = "Bit 3 - Indicates that colorkey functionality is enabled for this alpha surface"]
    #[inline(always)]
    pub fn enable_colorkey(&mut self) -> ENABLE_COLORKEY_W {
        ENABLE_COLORKEY_W { w: self }
    }
    #[doc = "Bits 4:7 - Indicates the input buffer format for AS."]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Bits 8:15 - Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in PXP_AS_CTRL\\[ALPHA_CTRL\\]"]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W {
        ALPHA_W { w: self }
    }
    #[doc = "Bits 16:19 - Indicates a raster operation to perform when enabled"]
    #[inline(always)]
    pub fn rop(&mut self) -> ROP_W {
        ROP_W { w: self }
    }
    #[doc = "Bit 20 - Setting this bit to logic 0 will not alter the alpha value"]
    #[inline(always)]
    pub fn alpha_invert(&mut self) -> ALPHA_INVERT_W {
        ALPHA_INVERT_W { w: self }
    }
}
