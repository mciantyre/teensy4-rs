#[doc = "Reader of register CSICR18"]
pub type R = crate::R<u32, super::CSICR18>;
#[doc = "Writer for register CSICR18"]
pub type W = crate::W<u32, super::CSICR18>;
#[doc = "Register CSICR18 `reset()`'s with value 0x0002_d000"]
impl crate::ResetValue for super::CSICR18 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_d000
    }
}
#[doc = "This bit is used to select the output method When input is standard CCIR656 video.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEINTERLACE_EN_A {
    #[doc = "0: Deinterlace disabled"]
    DEINTERLACE_EN_0 = 0,
    #[doc = "1: Deinterlace enabled"]
    DEINTERLACE_EN_1 = 1,
}
impl From<DEINTERLACE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DEINTERLACE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEINTERLACE_EN`"]
pub type DEINTERLACE_EN_R = crate::R<bool, DEINTERLACE_EN_A>;
impl DEINTERLACE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEINTERLACE_EN_A {
        match self.bits {
            false => DEINTERLACE_EN_A::DEINTERLACE_EN_0,
            true => DEINTERLACE_EN_A::DEINTERLACE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEINTERLACE_EN_0`"]
    #[inline(always)]
    pub fn is_deinterlace_en_0(&self) -> bool {
        *self == DEINTERLACE_EN_A::DEINTERLACE_EN_0
    }
    #[doc = "Checks if the value of the field is `DEINTERLACE_EN_1`"]
    #[inline(always)]
    pub fn is_deinterlace_en_1(&self) -> bool {
        *self == DEINTERLACE_EN_A::DEINTERLACE_EN_1
    }
}
#[doc = "Write proxy for field `DEINTERLACE_EN`"]
pub struct DEINTERLACE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEINTERLACE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEINTERLACE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deinterlace disabled"]
    #[inline(always)]
    pub fn deinterlace_en_0(self) -> &'a mut W {
        self.variant(DEINTERLACE_EN_A::DEINTERLACE_EN_0)
    }
    #[doc = "Deinterlace enabled"]
    #[inline(always)]
    pub fn deinterlace_en_1(self) -> &'a mut W {
        self.variant(DEINTERLACE_EN_A::DEINTERLACE_EN_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PARALLEL24_EN`"]
pub type PARALLEL24_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARALLEL24_EN`"]
pub struct PARALLEL24_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARALLEL24_EN_W<'a> {
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
#[doc = "Reader of field `BASEADDR_SWITCH_EN`"]
pub type BASEADDR_SWITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BASEADDR_SWITCH_EN`"]
pub struct BASEADDR_SWITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEADDR_SWITCH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASEADDR_SWITCH_SEL_A {
    #[doc = "0: Switching base address at the edge of the vsync"]
    BASEADDR_SWITCH_SEL_0 = 0,
    #[doc = "1: Switching base address at the edge of the first data of each frame"]
    BASEADDR_SWITCH_SEL_1 = 1,
}
impl From<BASEADDR_SWITCH_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: BASEADDR_SWITCH_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BASEADDR_SWITCH_SEL`"]
pub type BASEADDR_SWITCH_SEL_R = crate::R<bool, BASEADDR_SWITCH_SEL_A>;
impl BASEADDR_SWITCH_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BASEADDR_SWITCH_SEL_A {
        match self.bits {
            false => BASEADDR_SWITCH_SEL_A::BASEADDR_SWITCH_SEL_0,
            true => BASEADDR_SWITCH_SEL_A::BASEADDR_SWITCH_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `BASEADDR_SWITCH_SEL_0`"]
    #[inline(always)]
    pub fn is_baseaddr_switch_sel_0(&self) -> bool {
        *self == BASEADDR_SWITCH_SEL_A::BASEADDR_SWITCH_SEL_0
    }
    #[doc = "Checks if the value of the field is `BASEADDR_SWITCH_SEL_1`"]
    #[inline(always)]
    pub fn is_baseaddr_switch_sel_1(&self) -> bool {
        *self == BASEADDR_SWITCH_SEL_A::BASEADDR_SWITCH_SEL_1
    }
}
#[doc = "Write proxy for field `BASEADDR_SWITCH_SEL`"]
pub struct BASEADDR_SWITCH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEADDR_SWITCH_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BASEADDR_SWITCH_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Switching base address at the edge of the vsync"]
    #[inline(always)]
    pub fn baseaddr_switch_sel_0(self) -> &'a mut W {
        self.variant(BASEADDR_SWITCH_SEL_A::BASEADDR_SWITCH_SEL_0)
    }
    #[doc = "Switching base address at the edge of the first data of each frame"]
    #[inline(always)]
    pub fn baseaddr_switch_sel_1(self) -> &'a mut W {
        self.variant(BASEADDR_SWITCH_SEL_A::BASEADDR_SWITCH_SEL_1)
    }
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
#[doc = "In interlace mode, fileld 0 means interrupt enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELD0_DONE_IE_A {
    #[doc = "0: Interrupt disabled"]
    FIELD0_DONE_IE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    FIELD0_DONE_IE_1 = 1,
}
impl From<FIELD0_DONE_IE_A> for bool {
    #[inline(always)]
    fn from(variant: FIELD0_DONE_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIELD0_DONE_IE`"]
pub type FIELD0_DONE_IE_R = crate::R<bool, FIELD0_DONE_IE_A>;
impl FIELD0_DONE_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELD0_DONE_IE_A {
        match self.bits {
            false => FIELD0_DONE_IE_A::FIELD0_DONE_IE_0,
            true => FIELD0_DONE_IE_A::FIELD0_DONE_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIELD0_DONE_IE_0`"]
    #[inline(always)]
    pub fn is_field0_done_ie_0(&self) -> bool {
        *self == FIELD0_DONE_IE_A::FIELD0_DONE_IE_0
    }
    #[doc = "Checks if the value of the field is `FIELD0_DONE_IE_1`"]
    #[inline(always)]
    pub fn is_field0_done_ie_1(&self) -> bool {
        *self == FIELD0_DONE_IE_A::FIELD0_DONE_IE_1
    }
}
#[doc = "Write proxy for field `FIELD0_DONE_IE`"]
pub struct FIELD0_DONE_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELD0_DONE_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELD0_DONE_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn field0_done_ie_0(self) -> &'a mut W {
        self.variant(FIELD0_DONE_IE_A::FIELD0_DONE_IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn field0_done_ie_1(self) -> &'a mut W {
        self.variant(FIELD0_DONE_IE_A::FIELD0_DONE_IE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "When in interlace mode, field 1 done interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_FIELD1_DONE_IE_A {
    #[doc = "0: Interrupt disabled"]
    DMA_FIELD1_DONE_IE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    DMA_FIELD1_DONE_IE_1 = 1,
}
impl From<DMA_FIELD1_DONE_IE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_FIELD1_DONE_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_FIELD1_DONE_IE`"]
pub type DMA_FIELD1_DONE_IE_R = crate::R<bool, DMA_FIELD1_DONE_IE_A>;
impl DMA_FIELD1_DONE_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_FIELD1_DONE_IE_A {
        match self.bits {
            false => DMA_FIELD1_DONE_IE_A::DMA_FIELD1_DONE_IE_0,
            true => DMA_FIELD1_DONE_IE_A::DMA_FIELD1_DONE_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_FIELD1_DONE_IE_0`"]
    #[inline(always)]
    pub fn is_dma_field1_done_ie_0(&self) -> bool {
        *self == DMA_FIELD1_DONE_IE_A::DMA_FIELD1_DONE_IE_0
    }
    #[doc = "Checks if the value of the field is `DMA_FIELD1_DONE_IE_1`"]
    #[inline(always)]
    pub fn is_dma_field1_done_ie_1(&self) -> bool {
        *self == DMA_FIELD1_DONE_IE_A::DMA_FIELD1_DONE_IE_1
    }
}
#[doc = "Write proxy for field `DMA_FIELD1_DONE_IE`"]
pub struct DMA_FIELD1_DONE_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_FIELD1_DONE_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_FIELD1_DONE_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn dma_field1_done_ie_0(self) -> &'a mut W {
        self.variant(DMA_FIELD1_DONE_IE_A::DMA_FIELD1_DONE_IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn dma_field1_done_ie_1(self) -> &'a mut W {
        self.variant(DMA_FIELD1_DONE_IE_A::DMA_FIELD1_DONE_IE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Choosing the last DMA request condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_DMA_REQ_SEL_A {
    #[doc = "0: fifo_full_level"]
    LAST_DMA_REQ_SEL_0 = 0,
    #[doc = "1: hburst_length"]
    LAST_DMA_REQ_SEL_1 = 1,
}
impl From<LAST_DMA_REQ_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_DMA_REQ_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LAST_DMA_REQ_SEL`"]
pub type LAST_DMA_REQ_SEL_R = crate::R<bool, LAST_DMA_REQ_SEL_A>;
impl LAST_DMA_REQ_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAST_DMA_REQ_SEL_A {
        match self.bits {
            false => LAST_DMA_REQ_SEL_A::LAST_DMA_REQ_SEL_0,
            true => LAST_DMA_REQ_SEL_A::LAST_DMA_REQ_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LAST_DMA_REQ_SEL_0`"]
    #[inline(always)]
    pub fn is_last_dma_req_sel_0(&self) -> bool {
        *self == LAST_DMA_REQ_SEL_A::LAST_DMA_REQ_SEL_0
    }
    #[doc = "Checks if the value of the field is `LAST_DMA_REQ_SEL_1`"]
    #[inline(always)]
    pub fn is_last_dma_req_sel_1(&self) -> bool {
        *self == LAST_DMA_REQ_SEL_A::LAST_DMA_REQ_SEL_1
    }
}
#[doc = "Write proxy for field `LAST_DMA_REQ_SEL`"]
pub struct LAST_DMA_REQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_DMA_REQ_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LAST_DMA_REQ_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fifo_full_level"]
    #[inline(always)]
    pub fn last_dma_req_sel_0(self) -> &'a mut W {
        self.variant(LAST_DMA_REQ_SEL_A::LAST_DMA_REQ_SEL_0)
    }
    #[doc = "hburst_length"]
    #[inline(always)]
    pub fn last_dma_req_sel_1(self) -> &'a mut W {
        self.variant(LAST_DMA_REQ_SEL_A::LAST_DMA_REQ_SEL_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `BASEADDR_CHANGE_ERROR_IE`"]
pub type BASEADDR_CHANGE_ERROR_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BASEADDR_CHANGE_ERROR_IE`"]
pub struct BASEADDR_CHANGE_ERROR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEADDR_CHANGE_ERROR_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Output is 32-bit format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGB888A_FORMAT_SEL_A {
    #[doc = "0: {8'h0, data\\[23:0\\]}"]
    RGB888A_FORMAT_SEL_0 = 0,
    #[doc = "1: {data\\[23:0\\], 8'h0}"]
    RGB888A_FORMAT_SEL_1 = 1,
}
impl From<RGB888A_FORMAT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: RGB888A_FORMAT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RGB888A_FORMAT_SEL`"]
pub type RGB888A_FORMAT_SEL_R = crate::R<bool, RGB888A_FORMAT_SEL_A>;
impl RGB888A_FORMAT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB888A_FORMAT_SEL_A {
        match self.bits {
            false => RGB888A_FORMAT_SEL_A::RGB888A_FORMAT_SEL_0,
            true => RGB888A_FORMAT_SEL_A::RGB888A_FORMAT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RGB888A_FORMAT_SEL_0`"]
    #[inline(always)]
    pub fn is_rgb888a_format_sel_0(&self) -> bool {
        *self == RGB888A_FORMAT_SEL_A::RGB888A_FORMAT_SEL_0
    }
    #[doc = "Checks if the value of the field is `RGB888A_FORMAT_SEL_1`"]
    #[inline(always)]
    pub fn is_rgb888a_format_sel_1(&self) -> bool {
        *self == RGB888A_FORMAT_SEL_A::RGB888A_FORMAT_SEL_1
    }
}
#[doc = "Write proxy for field `RGB888A_FORMAT_SEL`"]
pub struct RGB888A_FORMAT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB888A_FORMAT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGB888A_FORMAT_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "{8'h0, data\\[23:0\\]}"]
    #[inline(always)]
    pub fn rgb888a_format_sel_0(self) -> &'a mut W {
        self.variant(RGB888A_FORMAT_SEL_A::RGB888A_FORMAT_SEL_0)
    }
    #[doc = "{data\\[23:0\\], 8'h0}"]
    #[inline(always)]
    pub fn rgb888a_format_sel_1(self) -> &'a mut W {
        self.variant(RGB888A_FORMAT_SEL_A::RGB888A_FORMAT_SEL_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `AHB_HPROT`"]
pub type AHB_HPROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHB_HPROT`"]
pub struct AHB_HPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_HPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "These bits used to choose the method to mask the CSI input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASK_OPTION_A {
    #[doc = "0: Writing to memory from first completely frame, when using this option, the CSI_ENABLE should be 1."]
    MASK_OPTION_0 = 0,
    #[doc = "1: Writing to memory when CSI_ENABLE is 1."]
    MASK_OPTION_1 = 1,
    #[doc = "2: Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1."]
    MASK_OPTION_2 = 2,
    #[doc = "3: Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0."]
    MASK_OPTION_3 = 3,
}
impl From<MASK_OPTION_A> for u8 {
    #[inline(always)]
    fn from(variant: MASK_OPTION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MASK_OPTION`"]
pub type MASK_OPTION_R = crate::R<u8, MASK_OPTION_A>;
impl MASK_OPTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_OPTION_A {
        match self.bits {
            0 => MASK_OPTION_A::MASK_OPTION_0,
            1 => MASK_OPTION_A::MASK_OPTION_1,
            2 => MASK_OPTION_A::MASK_OPTION_2,
            3 => MASK_OPTION_A::MASK_OPTION_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MASK_OPTION_0`"]
    #[inline(always)]
    pub fn is_mask_option_0(&self) -> bool {
        *self == MASK_OPTION_A::MASK_OPTION_0
    }
    #[doc = "Checks if the value of the field is `MASK_OPTION_1`"]
    #[inline(always)]
    pub fn is_mask_option_1(&self) -> bool {
        *self == MASK_OPTION_A::MASK_OPTION_1
    }
    #[doc = "Checks if the value of the field is `MASK_OPTION_2`"]
    #[inline(always)]
    pub fn is_mask_option_2(&self) -> bool {
        *self == MASK_OPTION_A::MASK_OPTION_2
    }
    #[doc = "Checks if the value of the field is `MASK_OPTION_3`"]
    #[inline(always)]
    pub fn is_mask_option_3(&self) -> bool {
        *self == MASK_OPTION_A::MASK_OPTION_3
    }
}
#[doc = "Write proxy for field `MASK_OPTION`"]
pub struct MASK_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_OPTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_OPTION_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Writing to memory from first completely frame, when using this option, the CSI_ENABLE should be 1."]
    #[inline(always)]
    pub fn mask_option_0(self) -> &'a mut W {
        self.variant(MASK_OPTION_A::MASK_OPTION_0)
    }
    #[doc = "Writing to memory when CSI_ENABLE is 1."]
    #[inline(always)]
    pub fn mask_option_1(self) -> &'a mut W {
        self.variant(MASK_OPTION_A::MASK_OPTION_1)
    }
    #[doc = "Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1."]
    #[inline(always)]
    pub fn mask_option_2(self) -> &'a mut W {
        self.variant(MASK_OPTION_A::MASK_OPTION_2)
    }
    #[doc = "Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0."]
    #[inline(always)]
    pub fn mask_option_3(self) -> &'a mut W {
        self.variant(MASK_OPTION_A::MASK_OPTION_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `CSI_ENABLE`"]
pub type CSI_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSI_ENABLE`"]
pub struct CSI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSI_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - This bit is used to select the output method When input is standard CCIR656 video."]
    #[inline(always)]
    pub fn deinterlace_en(&self) -> DEINTERLACE_EN_R {
        DEINTERLACE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When input is parallel rgb888/yuv444 24bit, this bit can be enabled."]
    #[inline(always)]
    pub fn parallel24_en(&self) -> PARALLEL24_EN_R {
        PARALLEL24_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than atomically by DMA completed"]
    #[inline(always)]
    pub fn baseaddr_switch_en(&self) -> BASEADDR_SWITCH_EN_R {
        BASEADDR_SWITCH_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1."]
    #[inline(always)]
    pub fn baseaddr_switch_sel(&self) -> BASEADDR_SWITCH_SEL_R {
        BASEADDR_SWITCH_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - In interlace mode, fileld 0 means interrupt enabled."]
    #[inline(always)]
    pub fn field0_done_ie(&self) -> FIELD0_DONE_IE_R {
        FIELD0_DONE_IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When in interlace mode, field 1 done interrupt enable."]
    #[inline(always)]
    pub fn dma_field1_done_ie(&self) -> DMA_FIELD1_DONE_IE_R {
        DMA_FIELD1_DONE_IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Choosing the last DMA request condition."]
    #[inline(always)]
    pub fn last_dma_req_sel(&self) -> LAST_DMA_REQ_SEL_R {
        LAST_DMA_REQ_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Base address change error interrupt enable signal."]
    #[inline(always)]
    pub fn baseaddr_change_error_ie(&self) -> BASEADDR_CHANGE_ERROR_IE_R {
        BASEADDR_CHANGE_ERROR_IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output is 32-bit format."]
    #[inline(always)]
    pub fn rgb888a_format_sel(&self) -> RGB888A_FORMAT_SEL_R {
        RGB888A_FORMAT_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Hprot value in AHB bus protocol."]
    #[inline(always)]
    pub fn ahb_hprot(&self) -> AHB_HPROT_R {
        AHB_HPROT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - These bits used to choose the method to mask the CSI input."]
    #[inline(always)]
    pub fn mask_option(&self) -> MASK_OPTION_R {
        MASK_OPTION_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 31 - CSI global enable signal"]
    #[inline(always)]
    pub fn csi_enable(&self) -> CSI_ENABLE_R {
        CSI_ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit is used to select the output method When input is standard CCIR656 video."]
    #[inline(always)]
    pub fn deinterlace_en(&mut self) -> DEINTERLACE_EN_W {
        DEINTERLACE_EN_W { w: self }
    }
    #[doc = "Bit 3 - When input is parallel rgb888/yuv444 24bit, this bit can be enabled."]
    #[inline(always)]
    pub fn parallel24_en(&mut self) -> PARALLEL24_EN_W {
        PARALLEL24_EN_W { w: self }
    }
    #[doc = "Bit 4 - When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than atomically by DMA completed"]
    #[inline(always)]
    pub fn baseaddr_switch_en(&mut self) -> BASEADDR_SWITCH_EN_W {
        BASEADDR_SWITCH_EN_W { w: self }
    }
    #[doc = "Bit 5 - CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1."]
    #[inline(always)]
    pub fn baseaddr_switch_sel(&mut self) -> BASEADDR_SWITCH_SEL_W {
        BASEADDR_SWITCH_SEL_W { w: self }
    }
    #[doc = "Bit 6 - In interlace mode, fileld 0 means interrupt enabled."]
    #[inline(always)]
    pub fn field0_done_ie(&mut self) -> FIELD0_DONE_IE_W {
        FIELD0_DONE_IE_W { w: self }
    }
    #[doc = "Bit 7 - When in interlace mode, field 1 done interrupt enable."]
    #[inline(always)]
    pub fn dma_field1_done_ie(&mut self) -> DMA_FIELD1_DONE_IE_W {
        DMA_FIELD1_DONE_IE_W { w: self }
    }
    #[doc = "Bit 8 - Choosing the last DMA request condition."]
    #[inline(always)]
    pub fn last_dma_req_sel(&mut self) -> LAST_DMA_REQ_SEL_W {
        LAST_DMA_REQ_SEL_W { w: self }
    }
    #[doc = "Bit 9 - Base address change error interrupt enable signal."]
    #[inline(always)]
    pub fn baseaddr_change_error_ie(&mut self) -> BASEADDR_CHANGE_ERROR_IE_W {
        BASEADDR_CHANGE_ERROR_IE_W { w: self }
    }
    #[doc = "Bit 10 - Output is 32-bit format."]
    #[inline(always)]
    pub fn rgb888a_format_sel(&mut self) -> RGB888A_FORMAT_SEL_W {
        RGB888A_FORMAT_SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - Hprot value in AHB bus protocol."]
    #[inline(always)]
    pub fn ahb_hprot(&mut self) -> AHB_HPROT_W {
        AHB_HPROT_W { w: self }
    }
    #[doc = "Bits 18:19 - These bits used to choose the method to mask the CSI input."]
    #[inline(always)]
    pub fn mask_option(&mut self) -> MASK_OPTION_W {
        MASK_OPTION_W { w: self }
    }
    #[doc = "Bit 31 - CSI global enable signal"]
    #[inline(always)]
    pub fn csi_enable(&mut self) -> CSI_ENABLE_W {
        CSI_ENABLE_W { w: self }
    }
}
