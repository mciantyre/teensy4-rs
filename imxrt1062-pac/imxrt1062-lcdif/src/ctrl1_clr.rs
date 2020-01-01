#[doc = "Reader of register CTRL1_CLR"]
pub type R = crate::R<u32, super::CTRL1_CLR>;
#[doc = "Writer for register CTRL1_CLR"]
pub type W = crate::W<u32, super::CTRL1_CLR>;
#[doc = "Register CTRL1_CLR `reset()`'s with value 0x000f_0000"]
impl crate::ResetValue for super::CTRL1_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000f_0000
    }
}
#[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSYNC_EDGE_IRQ_A {
    #[doc = "0: No Interrupt Request Pending."]
    NO_REQUEST = 0,
    #[doc = "1: Interrupt Request Pending."]
    REQUEST = 1,
}
impl From<VSYNC_EDGE_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_EDGE_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VSYNC_EDGE_IRQ`"]
pub type VSYNC_EDGE_IRQ_R = crate::R<bool, VSYNC_EDGE_IRQ_A>;
impl VSYNC_EDGE_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSYNC_EDGE_IRQ_A {
        match self.bits {
            false => VSYNC_EDGE_IRQ_A::NO_REQUEST,
            true => VSYNC_EDGE_IRQ_A::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == VSYNC_EDGE_IRQ_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == VSYNC_EDGE_IRQ_A::REQUEST
    }
}
#[doc = "Write proxy for field `VSYNC_EDGE_IRQ`"]
pub struct VSYNC_EDGE_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_EDGE_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VSYNC_EDGE_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(VSYNC_EDGE_IRQ_A::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(VSYNC_EDGE_IRQ_A::REQUEST)
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
#[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CUR_FRAME_DONE_IRQ_A {
    #[doc = "0: No Interrupt Request Pending."]
    NO_REQUEST = 0,
    #[doc = "1: Interrupt Request Pending."]
    REQUEST = 1,
}
impl From<CUR_FRAME_DONE_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CUR_FRAME_DONE_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CUR_FRAME_DONE_IRQ`"]
pub type CUR_FRAME_DONE_IRQ_R = crate::R<bool, CUR_FRAME_DONE_IRQ_A>;
impl CUR_FRAME_DONE_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CUR_FRAME_DONE_IRQ_A {
        match self.bits {
            false => CUR_FRAME_DONE_IRQ_A::NO_REQUEST,
            true => CUR_FRAME_DONE_IRQ_A::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == CUR_FRAME_DONE_IRQ_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == CUR_FRAME_DONE_IRQ_A::REQUEST
    }
}
#[doc = "Write proxy for field `CUR_FRAME_DONE_IRQ`"]
pub struct CUR_FRAME_DONE_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_FRAME_DONE_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CUR_FRAME_DONE_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(CUR_FRAME_DONE_IRQ_A::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(CUR_FRAME_DONE_IRQ_A::REQUEST)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFLOW_IRQ_A {
    #[doc = "0: No Interrupt Request Pending."]
    NO_REQUEST = 0,
    #[doc = "1: Interrupt Request Pending."]
    REQUEST = 1,
}
impl From<UNDERFLOW_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNDERFLOW_IRQ`"]
pub type UNDERFLOW_IRQ_R = crate::R<bool, UNDERFLOW_IRQ_A>;
impl UNDERFLOW_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_IRQ_A {
        match self.bits {
            false => UNDERFLOW_IRQ_A::NO_REQUEST,
            true => UNDERFLOW_IRQ_A::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == UNDERFLOW_IRQ_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == UNDERFLOW_IRQ_A::REQUEST
    }
}
#[doc = "Write proxy for field `UNDERFLOW_IRQ`"]
pub struct UNDERFLOW_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDERFLOW_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(UNDERFLOW_IRQ_A::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(UNDERFLOW_IRQ_A::REQUEST)
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
#[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_IRQ_A {
    #[doc = "0: No Interrupt Request Pending."]
    NO_REQUEST = 0,
    #[doc = "1: Interrupt Request Pending."]
    REQUEST = 1,
}
impl From<OVERFLOW_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVERFLOW_IRQ`"]
pub type OVERFLOW_IRQ_R = crate::R<bool, OVERFLOW_IRQ_A>;
impl OVERFLOW_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_IRQ_A {
        match self.bits {
            false => OVERFLOW_IRQ_A::NO_REQUEST,
            true => OVERFLOW_IRQ_A::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == OVERFLOW_IRQ_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == OVERFLOW_IRQ_A::REQUEST
    }
}
#[doc = "Write proxy for field `OVERFLOW_IRQ`"]
pub struct OVERFLOW_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(OVERFLOW_IRQ_A::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(OVERFLOW_IRQ_A::REQUEST)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `VSYNC_EDGE_IRQ_EN`"]
pub type VSYNC_EDGE_IRQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNC_EDGE_IRQ_EN`"]
pub struct VSYNC_EDGE_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_EDGE_IRQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CUR_FRAME_DONE_IRQ_EN`"]
pub type CUR_FRAME_DONE_IRQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CUR_FRAME_DONE_IRQ_EN`"]
pub struct CUR_FRAME_DONE_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_FRAME_DONE_IRQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `UNDERFLOW_IRQ_EN`"]
pub type UNDERFLOW_IRQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDERFLOW_IRQ_EN`"]
pub struct UNDERFLOW_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_IRQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `OVERFLOW_IRQ_EN`"]
pub type OVERFLOW_IRQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERFLOW_IRQ_EN`"]
pub struct OVERFLOW_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_IRQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `BYTE_PACKING_FORMAT`"]
pub type BYTE_PACKING_FORMAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE_PACKING_FORMAT`"]
pub struct BYTE_PACKING_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_PACKING_FORMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `IRQ_ON_ALTERNATE_FIELDS`"]
pub type IRQ_ON_ALTERNATE_FIELDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_ON_ALTERNATE_FIELDS`"]
pub struct IRQ_ON_ALTERNATE_FIELDS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_ON_ALTERNATE_FIELDS_W<'a> {
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
#[doc = "Reader of field `FIFO_CLEAR`"]
pub type FIFO_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_CLEAR`"]
pub struct FIFO_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `START_INTERLACE_FROM_SECOND_FIELD`"]
pub type START_INTERLACE_FROM_SECOND_FIELD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_INTERLACE_FROM_SECOND_FIELD`"]
pub struct START_INTERLACE_FROM_SECOND_FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> START_INTERLACE_FROM_SECOND_FIELD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `INTERLACE_FIELDS`"]
pub type INTERLACE_FIELDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERLACE_FIELDS`"]
pub struct INTERLACE_FIELDS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERLACE_FIELDS_W<'a> {
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
#[doc = "Reader of field `RECOVER_ON_UNDERFLOW`"]
pub type RECOVER_ON_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RECOVER_ON_UNDERFLOW`"]
pub struct RECOVER_ON_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RECOVER_ON_UNDERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "This bit is set to indicate that an interrupt is requested by the LCDIF block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM_ERROR_IRQ_A {
    #[doc = "0: No Interrupt Request Pending."]
    NO_REQUEST = 0,
    #[doc = "1: Interrupt Request Pending."]
    REQUEST = 1,
}
impl From<BM_ERROR_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: BM_ERROR_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BM_ERROR_IRQ`"]
pub type BM_ERROR_IRQ_R = crate::R<bool, BM_ERROR_IRQ_A>;
impl BM_ERROR_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM_ERROR_IRQ_A {
        match self.bits {
            false => BM_ERROR_IRQ_A::NO_REQUEST,
            true => BM_ERROR_IRQ_A::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == BM_ERROR_IRQ_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == BM_ERROR_IRQ_A::REQUEST
    }
}
#[doc = "Write proxy for field `BM_ERROR_IRQ`"]
pub struct BM_ERROR_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BM_ERROR_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BM_ERROR_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(BM_ERROR_IRQ_A::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(BM_ERROR_IRQ_A::REQUEST)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `BM_ERROR_IRQ_EN`"]
pub type BM_ERROR_IRQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BM_ERROR_IRQ_EN`"]
pub struct BM_ERROR_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BM_ERROR_IRQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CS_OUT_SELECT`"]
pub type CS_OUT_SELECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS_OUT_SELECT`"]
pub struct CS_OUT_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_OUT_SELECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `IMAGE_DATA_SELECT`"]
pub type IMAGE_DATA_SELECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMAGE_DATA_SELECT`"]
pub struct IMAGE_DATA_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAGE_DATA_SELECT_W<'a> {
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
    #[doc = "Bit 8 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn vsync_edge_irq(&self) -> VSYNC_EDGE_IRQ_R {
        VSYNC_EDGE_IRQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn cur_frame_done_irq(&self) -> CUR_FRAME_DONE_IRQ_R {
        CUR_FRAME_DONE_IRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn underflow_irq(&self) -> UNDERFLOW_IRQ_R {
        UNDERFLOW_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn overflow_irq(&self) -> OVERFLOW_IRQ_R {
        OVERFLOW_IRQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[inline(always)]
    pub fn vsync_edge_irq_en(&self) -> VSYNC_EDGE_IRQ_EN_R {
        VSYNC_EDGE_IRQ_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[inline(always)]
    pub fn cur_frame_done_irq_en(&self) -> CUR_FRAME_DONE_IRQ_EN_R {
        CUR_FRAME_DONE_IRQ_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub fn underflow_irq_en(&self) -> UNDERFLOW_IRQ_EN_R {
        UNDERFLOW_IRQ_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub fn overflow_irq_en(&self) -> OVERFLOW_IRQ_EN_R {
        OVERFLOW_IRQ_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[inline(always)]
    pub fn byte_packing_format(&self) -> BYTE_PACKING_FORMAT_R {
        BYTE_PACKING_FORMAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[inline(always)]
    pub fn irq_on_alternate_fields(&self) -> IRQ_ON_ALTERNATE_FIELDS_R {
        IRQ_ON_ALTERNATE_FIELDS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[inline(always)]
    pub fn fifo_clear(&self) -> FIFO_CLEAR_R {
        FIFO_CLEAR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The default is to grab the odd lines first and then the even lines"]
    #[inline(always)]
    pub fn start_interlace_from_second_field(&self) -> START_INTERLACE_FROM_SECOND_FIELD_R {
        START_INTERLACE_FROM_SECOND_FIELD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[inline(always)]
    pub fn interlace_fields(&self) -> INTERLACE_FIELDS_R {
        INTERLACE_FIELDS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[inline(always)]
    pub fn recover_on_underflow(&self) -> RECOVER_ON_UNDERFLOW_R {
        RECOVER_ON_UNDERFLOW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn bm_error_irq(&self) -> BM_ERROR_IRQ_R {
        BM_ERROR_IRQ_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[inline(always)]
    pub fn bm_error_irq_en(&self) -> BM_ERROR_IRQ_EN_R {
        BM_ERROR_IRQ_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit is CS0/CS1 valid select signals"]
    #[inline(always)]
    pub fn cs_out_select(&self) -> CS_OUT_SELECT_R {
        CS_OUT_SELECT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Command Mode MIPI image data select bit"]
    #[inline(always)]
    pub fn image_data_select(&self) -> IMAGE_DATA_SELECT_R {
        IMAGE_DATA_SELECT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn vsync_edge_irq(&mut self) -> VSYNC_EDGE_IRQ_W {
        VSYNC_EDGE_IRQ_W { w: self }
    }
    #[doc = "Bit 9 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn cur_frame_done_irq(&mut self) -> CUR_FRAME_DONE_IRQ_W {
        CUR_FRAME_DONE_IRQ_W { w: self }
    }
    #[doc = "Bit 10 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn underflow_irq(&mut self) -> UNDERFLOW_IRQ_W {
        UNDERFLOW_IRQ_W { w: self }
    }
    #[doc = "Bit 11 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn overflow_irq(&mut self) -> OVERFLOW_IRQ_W {
        OVERFLOW_IRQ_W { w: self }
    }
    #[doc = "Bit 12 - This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[inline(always)]
    pub fn vsync_edge_irq_en(&mut self) -> VSYNC_EDGE_IRQ_EN_W {
        VSYNC_EDGE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 13 - This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[inline(always)]
    pub fn cur_frame_done_irq_en(&mut self) -> CUR_FRAME_DONE_IRQ_EN_W {
        CUR_FRAME_DONE_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 14 - This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub fn underflow_irq_en(&mut self) -> UNDERFLOW_IRQ_EN_W {
        UNDERFLOW_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 15 - This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[inline(always)]
    pub fn overflow_irq_en(&mut self) -> OVERFLOW_IRQ_EN_W {
        OVERFLOW_IRQ_EN_W { w: self }
    }
    #[doc = "Bits 16:19 - This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[inline(always)]
    pub fn byte_packing_format(&mut self) -> BYTE_PACKING_FORMAT_W {
        BYTE_PACKING_FORMAT_W { w: self }
    }
    #[doc = "Bit 20 - If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[inline(always)]
    pub fn irq_on_alternate_fields(&mut self) -> IRQ_ON_ALTERNATE_FIELDS_W {
        IRQ_ON_ALTERNATE_FIELDS_W { w: self }
    }
    #[doc = "Bit 21 - Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[inline(always)]
    pub fn fifo_clear(&mut self) -> FIFO_CLEAR_W {
        FIFO_CLEAR_W { w: self }
    }
    #[doc = "Bit 22 - The default is to grab the odd lines first and then the even lines"]
    #[inline(always)]
    pub fn start_interlace_from_second_field(&mut self) -> START_INTERLACE_FROM_SECOND_FIELD_W {
        START_INTERLACE_FROM_SECOND_FIELD_W { w: self }
    }
    #[doc = "Bit 23 - Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[inline(always)]
    pub fn interlace_fields(&mut self) -> INTERLACE_FIELDS_W {
        INTERLACE_FIELDS_W { w: self }
    }
    #[doc = "Bit 24 - Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[inline(always)]
    pub fn recover_on_underflow(&mut self) -> RECOVER_ON_UNDERFLOW_W {
        RECOVER_ON_UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 25 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline(always)]
    pub fn bm_error_irq(&mut self) -> BM_ERROR_IRQ_W {
        BM_ERROR_IRQ_W { w: self }
    }
    #[doc = "Bit 26 - This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[inline(always)]
    pub fn bm_error_irq_en(&mut self) -> BM_ERROR_IRQ_EN_W {
        BM_ERROR_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 30 - This bit is CS0/CS1 valid select signals"]
    #[inline(always)]
    pub fn cs_out_select(&mut self) -> CS_OUT_SELECT_W {
        CS_OUT_SELECT_W { w: self }
    }
    #[doc = "Bit 31 - Command Mode MIPI image data select bit"]
    #[inline(always)]
    pub fn image_data_select(&mut self) -> IMAGE_DATA_SELECT_W {
        IMAGE_DATA_SELECT_W { w: self }
    }
}
