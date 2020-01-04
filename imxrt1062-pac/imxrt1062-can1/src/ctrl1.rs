#[doc = "Reader of register CTRL1"]
pub type R = crate::R<u32, super::CTRL1>;
#[doc = "Writer for register CTRL1"]
pub type W = crate::W<u32, super::CTRL1>;
#[doc = "Register CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROPSEG`"]
pub type PROPSEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROPSEG`"]
pub struct PROPSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROPSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "This bit configures FLEXCAN to operate in Listen Only Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOM_A {
    #[doc = "0: Listen Only Mode is deactivated"]
    LOM_0 = 0,
    #[doc = "1: FLEXCAN module operates in Listen Only Mode"]
    LOM_1 = 1,
}
impl From<LOM_A> for bool {
    #[inline(always)]
    fn from(variant: LOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOM`"]
pub type LOM_R = crate::R<bool, LOM_A>;
impl LOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOM_A {
        match self.bits {
            false => LOM_A::LOM_0,
            true => LOM_A::LOM_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOM_0`"]
    #[inline(always)]
    pub fn is_lom_0(&self) -> bool {
        *self == LOM_A::LOM_0
    }
    #[doc = "Checks if the value of the field is `LOM_1`"]
    #[inline(always)]
    pub fn is_lom_1(&self) -> bool {
        *self == LOM_A::LOM_1
    }
}
#[doc = "Write proxy for field `LOM`"]
pub struct LOM_W<'a> {
    w: &'a mut W,
}
impl<'a> LOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Listen Only Mode is deactivated"]
    #[inline(always)]
    pub fn lom_0(self) -> &'a mut W {
        self.variant(LOM_A::LOM_0)
    }
    #[doc = "FLEXCAN module operates in Listen Only Mode"]
    #[inline(always)]
    pub fn lom_1(self) -> &'a mut W {
        self.variant(LOM_A::LOM_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "This bit defines the ordering mechanism for Message Buffer transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBUF_A {
    #[doc = "0: Buffer with highest priority is transmitted first"]
    LBUF_0 = 0,
    #[doc = "1: Lowest number buffer is transmitted first"]
    LBUF_1 = 1,
}
impl From<LBUF_A> for bool {
    #[inline(always)]
    fn from(variant: LBUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBUF`"]
pub type LBUF_R = crate::R<bool, LBUF_A>;
impl LBUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBUF_A {
        match self.bits {
            false => LBUF_A::LBUF_0,
            true => LBUF_A::LBUF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LBUF_0`"]
    #[inline(always)]
    pub fn is_lbuf_0(&self) -> bool {
        *self == LBUF_A::LBUF_0
    }
    #[doc = "Checks if the value of the field is `LBUF_1`"]
    #[inline(always)]
    pub fn is_lbuf_1(&self) -> bool {
        *self == LBUF_A::LBUF_1
    }
}
#[doc = "Write proxy for field `LBUF`"]
pub struct LBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> LBUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBUF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Buffer with highest priority is transmitted first"]
    #[inline(always)]
    pub fn lbuf_0(self) -> &'a mut W {
        self.variant(LBUF_A::LBUF_0)
    }
    #[doc = "Lowest number buffer is transmitted first"]
    #[inline(always)]
    pub fn lbuf_1(self) -> &'a mut W {
        self.variant(LBUF_A::LBUF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSYN_A {
    #[doc = "0: Timer Sync feature disabled"]
    TSYN_0 = 0,
    #[doc = "1: Timer Sync feature enabled"]
    TSYN_1 = 1,
}
impl From<TSYN_A> for bool {
    #[inline(always)]
    fn from(variant: TSYN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSYN`"]
pub type TSYN_R = crate::R<bool, TSYN_A>;
impl TSYN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSYN_A {
        match self.bits {
            false => TSYN_A::TSYN_0,
            true => TSYN_A::TSYN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TSYN_0`"]
    #[inline(always)]
    pub fn is_tsyn_0(&self) -> bool {
        *self == TSYN_A::TSYN_0
    }
    #[doc = "Checks if the value of the field is `TSYN_1`"]
    #[inline(always)]
    pub fn is_tsyn_1(&self) -> bool {
        *self == TSYN_A::TSYN_1
    }
}
#[doc = "Write proxy for field `TSYN`"]
pub struct TSYN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSYN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSYN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer Sync feature disabled"]
    #[inline(always)]
    pub fn tsyn_0(self) -> &'a mut W {
        self.variant(TSYN_A::TSYN_0)
    }
    #[doc = "Timer Sync feature enabled"]
    #[inline(always)]
    pub fn tsyn_1(self) -> &'a mut W {
        self.variant(TSYN_A::TSYN_1)
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
#[doc = "This bit defines how FLEXCAN recovers from Bus Off state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFREC_A {
    #[doc = "0: Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
    BOFFREC_0 = 0,
    #[doc = "1: Automatic recovering from Bus Off state disabled"]
    BOFFREC_1 = 1,
}
impl From<BOFFREC_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFREC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOFFREC`"]
pub type BOFFREC_R = crate::R<bool, BOFFREC_A>;
impl BOFFREC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFREC_A {
        match self.bits {
            false => BOFFREC_A::BOFFREC_0,
            true => BOFFREC_A::BOFFREC_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFREC_0`"]
    #[inline(always)]
    pub fn is_boffrec_0(&self) -> bool {
        *self == BOFFREC_A::BOFFREC_0
    }
    #[doc = "Checks if the value of the field is `BOFFREC_1`"]
    #[inline(always)]
    pub fn is_boffrec_1(&self) -> bool {
        *self == BOFFREC_A::BOFFREC_1
    }
}
#[doc = "Write proxy for field `BOFFREC`"]
pub struct BOFFREC_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFREC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
    #[inline(always)]
    pub fn boffrec_0(self) -> &'a mut W {
        self.variant(BOFFREC_A::BOFFREC_0)
    }
    #[doc = "Automatic recovering from Bus Off state disabled"]
    #[inline(always)]
    pub fn boffrec_1(self) -> &'a mut W {
        self.variant(BOFFREC_A::BOFFREC_1)
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
#[doc = "This bit defines the sampling mode of CAN bits at the FLEXCAN_RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMP_A {
    #[doc = "0: Just one sample is used to determine the bit value"]
    SMP_0 = 0,
    #[doc = "1: Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples, a majority rule is used"]
    SMP_1 = 1,
}
impl From<SMP_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMP`"]
pub type SMP_R = crate::R<bool, SMP_A>;
impl SMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            false => SMP_A::SMP_0,
            true => SMP_A::SMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMP_0`"]
    #[inline(always)]
    pub fn is_smp_0(&self) -> bool {
        *self == SMP_A::SMP_0
    }
    #[doc = "Checks if the value of the field is `SMP_1`"]
    #[inline(always)]
    pub fn is_smp_1(&self) -> bool {
        *self == SMP_A::SMP_1
    }
}
#[doc = "Write proxy for field `SMP`"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Just one sample is used to determine the bit value"]
    #[inline(always)]
    pub fn smp_0(self) -> &'a mut W {
        self.variant(SMP_A::SMP_0)
    }
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples, a majority rule is used"]
    #[inline(always)]
    pub fn smp_1(self) -> &'a mut W {
        self.variant(SMP_A::SMP_1)
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
#[doc = "This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNMSK_A {
    #[doc = "0: Rx Warning Interrupt disabled"]
    RWRNMSK_0 = 0,
    #[doc = "1: Rx Warning Interrupt enabled"]
    RWRNMSK_1 = 1,
}
impl From<RWRNMSK_A> for bool {
    #[inline(always)]
    fn from(variant: RWRNMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RWRNMSK`"]
pub type RWRNMSK_R = crate::R<bool, RWRNMSK_A>;
impl RWRNMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWRNMSK_A {
        match self.bits {
            false => RWRNMSK_A::RWRNMSK_0,
            true => RWRNMSK_A::RWRNMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWRNMSK_0`"]
    #[inline(always)]
    pub fn is_rwrnmsk_0(&self) -> bool {
        *self == RWRNMSK_A::RWRNMSK_0
    }
    #[doc = "Checks if the value of the field is `RWRNMSK_1`"]
    #[inline(always)]
    pub fn is_rwrnmsk_1(&self) -> bool {
        *self == RWRNMSK_A::RWRNMSK_1
    }
}
#[doc = "Write proxy for field `RWRNMSK`"]
pub struct RWRNMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RWRNMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWRNMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx Warning Interrupt disabled"]
    #[inline(always)]
    pub fn rwrnmsk_0(self) -> &'a mut W {
        self.variant(RWRNMSK_A::RWRNMSK_0)
    }
    #[doc = "Rx Warning Interrupt enabled"]
    #[inline(always)]
    pub fn rwrnmsk_1(self) -> &'a mut W {
        self.variant(RWRNMSK_A::RWRNMSK_1)
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
#[doc = "This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNMSK_A {
    #[doc = "0: Tx Warning Interrupt disabled"]
    TWRNMSK_0 = 0,
    #[doc = "1: Tx Warning Interrupt enabled"]
    TWRNMSK_1 = 1,
}
impl From<TWRNMSK_A> for bool {
    #[inline(always)]
    fn from(variant: TWRNMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TWRNMSK`"]
pub type TWRNMSK_R = crate::R<bool, TWRNMSK_A>;
impl TWRNMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRNMSK_A {
        match self.bits {
            false => TWRNMSK_A::TWRNMSK_0,
            true => TWRNMSK_A::TWRNMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TWRNMSK_0`"]
    #[inline(always)]
    pub fn is_twrnmsk_0(&self) -> bool {
        *self == TWRNMSK_A::TWRNMSK_0
    }
    #[doc = "Checks if the value of the field is `TWRNMSK_1`"]
    #[inline(always)]
    pub fn is_twrnmsk_1(&self) -> bool {
        *self == TWRNMSK_A::TWRNMSK_1
    }
}
#[doc = "Write proxy for field `TWRNMSK`"]
pub struct TWRNMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TWRNMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWRNMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx Warning Interrupt disabled"]
    #[inline(always)]
    pub fn twrnmsk_0(self) -> &'a mut W {
        self.variant(TWRNMSK_A::TWRNMSK_0)
    }
    #[doc = "Tx Warning Interrupt enabled"]
    #[inline(always)]
    pub fn twrnmsk_1(self) -> &'a mut W {
        self.variant(TWRNMSK_A::TWRNMSK_1)
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
#[doc = "This bit configures FlexCAN to operate in Loop-Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPB_A {
    #[doc = "0: Loop Back disabled"]
    LPB_0 = 0,
    #[doc = "1: Loop Back enabled"]
    LPB_1 = 1,
}
impl From<LPB_A> for bool {
    #[inline(always)]
    fn from(variant: LPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPB`"]
pub type LPB_R = crate::R<bool, LPB_A>;
impl LPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPB_A {
        match self.bits {
            false => LPB_A::LPB_0,
            true => LPB_A::LPB_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPB_0`"]
    #[inline(always)]
    pub fn is_lpb_0(&self) -> bool {
        *self == LPB_A::LPB_0
    }
    #[doc = "Checks if the value of the field is `LPB_1`"]
    #[inline(always)]
    pub fn is_lpb_1(&self) -> bool {
        *self == LPB_A::LPB_1
    }
}
#[doc = "Write proxy for field `LPB`"]
pub struct LPB_W<'a> {
    w: &'a mut W,
}
impl<'a> LPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loop Back disabled"]
    #[inline(always)]
    pub fn lpb_0(self) -> &'a mut W {
        self.variant(LPB_A::LPB_0)
    }
    #[doc = "Loop Back enabled"]
    #[inline(always)]
    pub fn lpb_1(self) -> &'a mut W {
        self.variant(LPB_A::LPB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "This bit provides a mask for the Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMSK_A {
    #[doc = "0: Error interrupt disabled"]
    ERRMSK_0 = 0,
    #[doc = "1: Error interrupt enabled"]
    ERRMSK_1 = 1,
}
impl From<ERRMSK_A> for bool {
    #[inline(always)]
    fn from(variant: ERRMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRMSK`"]
pub type ERRMSK_R = crate::R<bool, ERRMSK_A>;
impl ERRMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRMSK_A {
        match self.bits {
            false => ERRMSK_A::ERRMSK_0,
            true => ERRMSK_A::ERRMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRMSK_0`"]
    #[inline(always)]
    pub fn is_errmsk_0(&self) -> bool {
        *self == ERRMSK_A::ERRMSK_0
    }
    #[doc = "Checks if the value of the field is `ERRMSK_1`"]
    #[inline(always)]
    pub fn is_errmsk_1(&self) -> bool {
        *self == ERRMSK_A::ERRMSK_1
    }
}
#[doc = "Write proxy for field `ERRMSK`"]
pub struct ERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn errmsk_0(self) -> &'a mut W {
        self.variant(ERRMSK_A::ERRMSK_0)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn errmsk_1(self) -> &'a mut W {
        self.variant(ERRMSK_A::ERRMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "This bit provides a mask for the Bus Off Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFMSK_A {
    #[doc = "0: Bus Off interrupt disabled"]
    BOFFMSK_0 = 0,
    #[doc = "1: Bus Off interrupt enabled"]
    BOFFMSK_1 = 1,
}
impl From<BOFFMSK_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOFFMSK`"]
pub type BOFFMSK_R = crate::R<bool, BOFFMSK_A>;
impl BOFFMSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFMSK_A {
        match self.bits {
            false => BOFFMSK_A::BOFFMSK_0,
            true => BOFFMSK_A::BOFFMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFMSK_0`"]
    #[inline(always)]
    pub fn is_boffmsk_0(&self) -> bool {
        *self == BOFFMSK_A::BOFFMSK_0
    }
    #[doc = "Checks if the value of the field is `BOFFMSK_1`"]
    #[inline(always)]
    pub fn is_boffmsk_1(&self) -> bool {
        *self == BOFFMSK_A::BOFFMSK_1
    }
}
#[doc = "Write proxy for field `BOFFMSK`"]
pub struct BOFFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFMSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus Off interrupt disabled"]
    #[inline(always)]
    pub fn boffmsk_0(self) -> &'a mut W {
        self.variant(BOFFMSK_A::BOFFMSK_0)
    }
    #[doc = "Bus Off interrupt enabled"]
    #[inline(always)]
    pub fn boffmsk_1(self) -> &'a mut W {
        self.variant(BOFFMSK_A::BOFFMSK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PSEG2`"]
pub type PSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSEG2`"]
pub struct PSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `PSEG1`"]
pub type PSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSEG1`"]
pub struct PSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `RJW`"]
pub type RJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RJW`"]
pub struct RJW_W<'a> {
    w: &'a mut W,
}
impl<'a> RJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRESDIV`"]
pub type PRESDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESDIV`"]
pub struct PRESDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - This 3-bit field defines the length of the Propagation Segment in the bit time"]
    #[inline(always)]
    pub fn propseg(&self) -> PROPSEG_R {
        PROPSEG_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - This bit configures FLEXCAN to operate in Listen Only Mode"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit defines the ordering mechanism for Message Buffer transmission"]
    #[inline(always)]
    pub fn lbuf(&self) -> LBUF_R {
        LBUF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0"]
    #[inline(always)]
    pub fn tsyn(&self) -> TSYN_R {
        TSYN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit defines how FLEXCAN recovers from Bus Off state"]
    #[inline(always)]
    pub fn boffrec(&self) -> BOFFREC_R {
        BOFFREC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit defines the sampling mode of CAN bits at the FLEXCAN_RX"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register"]
    #[inline(always)]
    pub fn rwrnmsk(&self) -> RWRNMSK_R {
        RWRNMSK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register"]
    #[inline(always)]
    pub fn twrnmsk(&self) -> TWRNMSK_R {
        TWRNMSK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit configures FlexCAN to operate in Loop-Back Mode"]
    #[inline(always)]
    pub fn lpb(&self) -> LPB_R {
        LPB_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit provides a mask for the Error Interrupt."]
    #[inline(always)]
    pub fn errmsk(&self) -> ERRMSK_R {
        ERRMSK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit provides a mask for the Bus Off Interrupt."]
    #[inline(always)]
    pub fn boffmsk(&self) -> BOFFMSK_R {
        BOFFMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - This 3-bit field defines the length of Phase Buffer Segment 2 in the bit time"]
    #[inline(always)]
    pub fn pseg2(&self) -> PSEG2_R {
        PSEG2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - This 3-bit field defines the length of Phase Buffer Segment 1 in the bit time"]
    #[inline(always)]
    pub fn pseg1(&self) -> PSEG1_R {
        PSEG1_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 22:23 - This 2-bit field defines the maximum number of time quanta One time quantum is equal to the Sclock period"]
    #[inline(always)]
    pub fn rjw(&self) -> RJW_R {
        RJW_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:31 - This 8-bit field defines the ratio between the PE clock frequency and the Serial Clock (Sclock) frequency"]
    #[inline(always)]
    pub fn presdiv(&self) -> PRESDIV_R {
        PRESDIV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This 3-bit field defines the length of the Propagation Segment in the bit time"]
    #[inline(always)]
    pub fn propseg(&mut self) -> PROPSEG_W {
        PROPSEG_W { w: self }
    }
    #[doc = "Bit 3 - This bit configures FLEXCAN to operate in Listen Only Mode"]
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W {
        LOM_W { w: self }
    }
    #[doc = "Bit 4 - This bit defines the ordering mechanism for Message Buffer transmission"]
    #[inline(always)]
    pub fn lbuf(&mut self) -> LBUF_W {
        LBUF_W { w: self }
    }
    #[doc = "Bit 5 - This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0"]
    #[inline(always)]
    pub fn tsyn(&mut self) -> TSYN_W {
        TSYN_W { w: self }
    }
    #[doc = "Bit 6 - This bit defines how FLEXCAN recovers from Bus Off state"]
    #[inline(always)]
    pub fn boffrec(&mut self) -> BOFFREC_W {
        BOFFREC_W { w: self }
    }
    #[doc = "Bit 7 - This bit defines the sampling mode of CAN bits at the FLEXCAN_RX"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
    #[doc = "Bit 10 - This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register"]
    #[inline(always)]
    pub fn rwrnmsk(&mut self) -> RWRNMSK_W {
        RWRNMSK_W { w: self }
    }
    #[doc = "Bit 11 - This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register"]
    #[inline(always)]
    pub fn twrnmsk(&mut self) -> TWRNMSK_W {
        TWRNMSK_W { w: self }
    }
    #[doc = "Bit 12 - This bit configures FlexCAN to operate in Loop-Back Mode"]
    #[inline(always)]
    pub fn lpb(&mut self) -> LPB_W {
        LPB_W { w: self }
    }
    #[doc = "Bit 14 - This bit provides a mask for the Error Interrupt."]
    #[inline(always)]
    pub fn errmsk(&mut self) -> ERRMSK_W {
        ERRMSK_W { w: self }
    }
    #[doc = "Bit 15 - This bit provides a mask for the Bus Off Interrupt."]
    #[inline(always)]
    pub fn boffmsk(&mut self) -> BOFFMSK_W {
        BOFFMSK_W { w: self }
    }
    #[doc = "Bits 16:18 - This 3-bit field defines the length of Phase Buffer Segment 2 in the bit time"]
    #[inline(always)]
    pub fn pseg2(&mut self) -> PSEG2_W {
        PSEG2_W { w: self }
    }
    #[doc = "Bits 19:21 - This 3-bit field defines the length of Phase Buffer Segment 1 in the bit time"]
    #[inline(always)]
    pub fn pseg1(&mut self) -> PSEG1_W {
        PSEG1_W { w: self }
    }
    #[doc = "Bits 22:23 - This 2-bit field defines the maximum number of time quanta One time quantum is equal to the Sclock period"]
    #[inline(always)]
    pub fn rjw(&mut self) -> RJW_W {
        RJW_W { w: self }
    }
    #[doc = "Bits 24:31 - This 8-bit field defines the ratio between the PE clock frequency and the Serial Clock (Sclock) frequency"]
    #[inline(always)]
    pub fn presdiv(&mut self) -> PRESDIV_W {
        PRESDIV_W { w: self }
    }
}
