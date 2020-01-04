#[doc = "Reader of register CTRL"]
pub type R = crate::R<u16, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u16, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Compare Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPIE_A {
    #[doc = "0: Compare interrupt is disabled"]
    CMPIE_0 = 0,
    #[doc = "1: Compare interrupt is enabled"]
    CMPIE_1 = 1,
}
impl From<CMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMPIE`"]
pub type CMPIE_R = crate::R<bool, CMPIE_A>;
impl CMPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIE_A {
        match self.bits {
            false => CMPIE_A::CMPIE_0,
            true => CMPIE_A::CMPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMPIE_0`"]
    #[inline(always)]
    pub fn is_cmpie_0(&self) -> bool {
        *self == CMPIE_A::CMPIE_0
    }
    #[doc = "Checks if the value of the field is `CMPIE_1`"]
    #[inline(always)]
    pub fn is_cmpie_1(&self) -> bool {
        *self == CMPIE_A::CMPIE_1
    }
}
#[doc = "Write proxy for field `CMPIE`"]
pub struct CMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare interrupt is disabled"]
    #[inline(always)]
    pub fn cmpie_0(self) -> &'a mut W {
        self.variant(CMPIE_A::CMPIE_0)
    }
    #[doc = "Compare interrupt is enabled"]
    #[inline(always)]
    pub fn cmpie_1(self) -> &'a mut W {
        self.variant(CMPIE_A::CMPIE_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Compare Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPIRQ_A {
    #[doc = "0: No match has occurred"]
    CMPIRQ_0 = 0,
    #[doc = "1: COMP match has occurred"]
    CMPIRQ_1 = 1,
}
impl From<CMPIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMPIRQ`"]
pub type CMPIRQ_R = crate::R<bool, CMPIRQ_A>;
impl CMPIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIRQ_A {
        match self.bits {
            false => CMPIRQ_A::CMPIRQ_0,
            true => CMPIRQ_A::CMPIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMPIRQ_0`"]
    #[inline(always)]
    pub fn is_cmpirq_0(&self) -> bool {
        *self == CMPIRQ_A::CMPIRQ_0
    }
    #[doc = "Checks if the value of the field is `CMPIRQ_1`"]
    #[inline(always)]
    pub fn is_cmpirq_1(&self) -> bool {
        *self == CMPIRQ_A::CMPIRQ_1
    }
}
#[doc = "Write proxy for field `CMPIRQ`"]
pub struct CMPIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPIRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No match has occurred"]
    #[inline(always)]
    pub fn cmpirq_0(self) -> &'a mut W {
        self.variant(CMPIRQ_A::CMPIRQ_0)
    }
    #[doc = "COMP match has occurred"]
    #[inline(always)]
    pub fn cmpirq_1(self) -> &'a mut W {
        self.variant(CMPIRQ_A::CMPIRQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Watchdog Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDE_A {
    #[doc = "0: Watchdog timer is disabled"]
    WDE_0 = 0,
    #[doc = "1: Watchdog timer is enabled"]
    WDE_1 = 1,
}
impl From<WDE_A> for bool {
    #[inline(always)]
    fn from(variant: WDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDE`"]
pub type WDE_R = crate::R<bool, WDE_A>;
impl WDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDE_A {
        match self.bits {
            false => WDE_A::WDE_0,
            true => WDE_A::WDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDE_0`"]
    #[inline(always)]
    pub fn is_wde_0(&self) -> bool {
        *self == WDE_A::WDE_0
    }
    #[doc = "Checks if the value of the field is `WDE_1`"]
    #[inline(always)]
    pub fn is_wde_1(&self) -> bool {
        *self == WDE_A::WDE_1
    }
}
#[doc = "Write proxy for field `WDE`"]
pub struct WDE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Watchdog timer is disabled"]
    #[inline(always)]
    pub fn wde_0(self) -> &'a mut W {
        self.variant(WDE_A::WDE_0)
    }
    #[doc = "Watchdog timer is enabled"]
    #[inline(always)]
    pub fn wde_1(self) -> &'a mut W {
        self.variant(WDE_A::WDE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Watchdog Timeout Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIE_A {
    #[doc = "0: Watchdog timer interrupt is disabled"]
    DIE_0 = 0,
    #[doc = "1: Watchdog timer interrupt is enabled"]
    DIE_1 = 1,
}
impl From<DIE_A> for bool {
    #[inline(always)]
    fn from(variant: DIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIE`"]
pub type DIE_R = crate::R<bool, DIE_A>;
impl DIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIE_A {
        match self.bits {
            false => DIE_A::DIE_0,
            true => DIE_A::DIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIE_0`"]
    #[inline(always)]
    pub fn is_die_0(&self) -> bool {
        *self == DIE_A::DIE_0
    }
    #[doc = "Checks if the value of the field is `DIE_1`"]
    #[inline(always)]
    pub fn is_die_1(&self) -> bool {
        *self == DIE_A::DIE_1
    }
}
#[doc = "Write proxy for field `DIE`"]
pub struct DIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Watchdog timer interrupt is disabled"]
    #[inline(always)]
    pub fn die_0(self) -> &'a mut W {
        self.variant(DIE_A::DIE_0)
    }
    #[doc = "Watchdog timer interrupt is enabled"]
    #[inline(always)]
    pub fn die_1(self) -> &'a mut W {
        self.variant(DIE_A::DIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Watchdog Timeout Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRQ_A {
    #[doc = "0: No interrupt has occurred"]
    DIRQ_0 = 0,
    #[doc = "1: Watchdog timeout interrupt has occurred"]
    DIRQ_1 = 1,
}
impl From<DIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRQ`"]
pub type DIRQ_R = crate::R<bool, DIRQ_A>;
impl DIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ_A {
        match self.bits {
            false => DIRQ_A::DIRQ_0,
            true => DIRQ_A::DIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRQ_0`"]
    #[inline(always)]
    pub fn is_dirq_0(&self) -> bool {
        *self == DIRQ_A::DIRQ_0
    }
    #[doc = "Checks if the value of the field is `DIRQ_1`"]
    #[inline(always)]
    pub fn is_dirq_1(&self) -> bool {
        *self == DIRQ_A::DIRQ_1
    }
}
#[doc = "Write proxy for field `DIRQ`"]
pub struct DIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt has occurred"]
    #[inline(always)]
    pub fn dirq_0(self) -> &'a mut W {
        self.variant(DIRQ_A::DIRQ_0)
    }
    #[doc = "Watchdog timeout interrupt has occurred"]
    #[inline(always)]
    pub fn dirq_1(self) -> &'a mut W {
        self.variant(DIRQ_A::DIRQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Use Negative Edge of INDEX Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XNE_A {
    #[doc = "0: Use positive transition edge of INDEX pulse"]
    XNE_0 = 0,
    #[doc = "1: Use negative transition edge of INDEX pulse"]
    XNE_1 = 1,
}
impl From<XNE_A> for bool {
    #[inline(always)]
    fn from(variant: XNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XNE`"]
pub type XNE_R = crate::R<bool, XNE_A>;
impl XNE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XNE_A {
        match self.bits {
            false => XNE_A::XNE_0,
            true => XNE_A::XNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNE_0`"]
    #[inline(always)]
    pub fn is_xne_0(&self) -> bool {
        *self == XNE_A::XNE_0
    }
    #[doc = "Checks if the value of the field is `XNE_1`"]
    #[inline(always)]
    pub fn is_xne_1(&self) -> bool {
        *self == XNE_A::XNE_1
    }
}
#[doc = "Write proxy for field `XNE`"]
pub struct XNE_W<'a> {
    w: &'a mut W,
}
impl<'a> XNE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XNE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use positive transition edge of INDEX pulse"]
    #[inline(always)]
    pub fn xne_0(self) -> &'a mut W {
        self.variant(XNE_A::XNE_0)
    }
    #[doc = "Use negative transition edge of INDEX pulse"]
    #[inline(always)]
    pub fn xne_1(self) -> &'a mut W {
        self.variant(XNE_A::XNE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIP_A {
    #[doc = "0: No action"]
    XIP_0 = 0,
    #[doc = "1: INDEX pulse initializes the position counter"]
    XIP_1 = 1,
}
impl From<XIP_A> for bool {
    #[inline(always)]
    fn from(variant: XIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XIP`"]
pub type XIP_R = crate::R<bool, XIP_A>;
impl XIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIP_A {
        match self.bits {
            false => XIP_A::XIP_0,
            true => XIP_A::XIP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIP_0`"]
    #[inline(always)]
    pub fn is_xip_0(&self) -> bool {
        *self == XIP_A::XIP_0
    }
    #[doc = "Checks if the value of the field is `XIP_1`"]
    #[inline(always)]
    pub fn is_xip_1(&self) -> bool {
        *self == XIP_A::XIP_1
    }
}
#[doc = "Write proxy for field `XIP`"]
pub struct XIP_W<'a> {
    w: &'a mut W,
}
impl<'a> XIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn xip_0(self) -> &'a mut W {
        self.variant(XIP_A::XIP_0)
    }
    #[doc = "INDEX pulse initializes the position counter"]
    #[inline(always)]
    pub fn xip_1(self) -> &'a mut W {
        self.variant(XIP_A::XIP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "INDEX Pulse Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIE_A {
    #[doc = "0: INDEX pulse interrupt is disabled"]
    XIE_0 = 0,
    #[doc = "1: INDEX pulse interrupt is enabled"]
    XIE_1 = 1,
}
impl From<XIE_A> for bool {
    #[inline(always)]
    fn from(variant: XIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XIE`"]
pub type XIE_R = crate::R<bool, XIE_A>;
impl XIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIE_A {
        match self.bits {
            false => XIE_A::XIE_0,
            true => XIE_A::XIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIE_0`"]
    #[inline(always)]
    pub fn is_xie_0(&self) -> bool {
        *self == XIE_A::XIE_0
    }
    #[doc = "Checks if the value of the field is `XIE_1`"]
    #[inline(always)]
    pub fn is_xie_1(&self) -> bool {
        *self == XIE_A::XIE_1
    }
}
#[doc = "Write proxy for field `XIE`"]
pub struct XIE_W<'a> {
    w: &'a mut W,
}
impl<'a> XIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "INDEX pulse interrupt is disabled"]
    #[inline(always)]
    pub fn xie_0(self) -> &'a mut W {
        self.variant(XIE_A::XIE_0)
    }
    #[doc = "INDEX pulse interrupt is enabled"]
    #[inline(always)]
    pub fn xie_1(self) -> &'a mut W {
        self.variant(XIE_A::XIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "INDEX Pulse Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIRQ_A {
    #[doc = "0: No interrupt has occurred"]
    XIRQ_0 = 0,
    #[doc = "1: INDEX pulse interrupt has occurred"]
    XIRQ_1 = 1,
}
impl From<XIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: XIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XIRQ`"]
pub type XIRQ_R = crate::R<bool, XIRQ_A>;
impl XIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIRQ_A {
        match self.bits {
            false => XIRQ_A::XIRQ_0,
            true => XIRQ_A::XIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIRQ_0`"]
    #[inline(always)]
    pub fn is_xirq_0(&self) -> bool {
        *self == XIRQ_A::XIRQ_0
    }
    #[doc = "Checks if the value of the field is `XIRQ_1`"]
    #[inline(always)]
    pub fn is_xirq_1(&self) -> bool {
        *self == XIRQ_A::XIRQ_1
    }
}
#[doc = "Write proxy for field `XIRQ`"]
pub struct XIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> XIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt has occurred"]
    #[inline(always)]
    pub fn xirq_0(self) -> &'a mut W {
        self.variant(XIRQ_A::XIRQ_0)
    }
    #[doc = "INDEX pulse interrupt has occurred"]
    #[inline(always)]
    pub fn xirq_1(self) -> &'a mut W {
        self.variant(XIRQ_A::XIRQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Enable Signal Phase Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PH1_A {
    #[doc = "0: Use standard quadrature decoder where PHASEA and PHASEB represent a two phase quadrature signal."]
    PH1_0 = 0,
    #[doc = "1: Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction. If CTRL\\[REV\\]
= 0, PHASEB = 0, then count up If CTRL\\[REV\\]
= 0, PHASEB = 1, then count down If CTRL\\[REV\\]
= 1, PHASEB = 0, then count down If CTRL\\[REV\\]
= 1, PHASEB = 1, then count up"]
    PH1_1 = 1,
}
impl From<PH1_A> for bool {
    #[inline(always)]
    fn from(variant: PH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PH1`"]
pub type PH1_R = crate::R<bool, PH1_A>;
impl PH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PH1_A {
        match self.bits {
            false => PH1_A::PH1_0,
            true => PH1_A::PH1_1,
        }
    }
    #[doc = "Checks if the value of the field is `PH1_0`"]
    #[inline(always)]
    pub fn is_ph1_0(&self) -> bool {
        *self == PH1_A::PH1_0
    }
    #[doc = "Checks if the value of the field is `PH1_1`"]
    #[inline(always)]
    pub fn is_ph1_1(&self) -> bool {
        *self == PH1_A::PH1_1
    }
}
#[doc = "Write proxy for field `PH1`"]
pub struct PH1_W<'a> {
    w: &'a mut W,
}
impl<'a> PH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use standard quadrature decoder where PHASEA and PHASEB represent a two phase quadrature signal."]
    #[inline(always)]
    pub fn ph1_0(self) -> &'a mut W {
        self.variant(PH1_A::PH1_0)
    }
    #[doc = "Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction. If CTRL\\[REV\\]
= 0, PHASEB = 0, then count up If CTRL\\[REV\\]
= 0, PHASEB = 1, then count down If CTRL\\[REV\\]
= 1, PHASEB = 0, then count down If CTRL\\[REV\\]
= 1, PHASEB = 1, then count up"]
    #[inline(always)]
    pub fn ph1_1(self) -> &'a mut W {
        self.variant(PH1_A::PH1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable Reverse Direction Counting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV_A {
    #[doc = "0: Count normally"]
    REV_0 = 0,
    #[doc = "1: Count in the reverse direction"]
    REV_1 = 1,
}
impl From<REV_A> for bool {
    #[inline(always)]
    fn from(variant: REV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<bool, REV_A>;
impl REV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV_A {
        match self.bits {
            false => REV_A::REV_0,
            true => REV_A::REV_1,
        }
    }
    #[doc = "Checks if the value of the field is `REV_0`"]
    #[inline(always)]
    pub fn is_rev_0(&self) -> bool {
        *self == REV_A::REV_0
    }
    #[doc = "Checks if the value of the field is `REV_1`"]
    #[inline(always)]
    pub fn is_rev_1(&self) -> bool {
        *self == REV_A::REV_1
    }
}
#[doc = "Write proxy for field `REV`"]
pub struct REV_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Count normally"]
    #[inline(always)]
    pub fn rev_0(self) -> &'a mut W {
        self.variant(REV_A::REV_0)
    }
    #[doc = "Count in the reverse direction"]
    #[inline(always)]
    pub fn rev_1(self) -> &'a mut W {
        self.variant(REV_A::REV_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Software Triggered Initialization of Position Counters UPOS and LPOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIP_A {
    #[doc = "0: No action"]
    SWIP_0 = 0,
    #[doc = "1: Initialize position counter"]
    SWIP_1 = 1,
}
impl From<SWIP_A> for bool {
    #[inline(always)]
    fn from(variant: SWIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWIP`"]
pub type SWIP_R = crate::R<bool, SWIP_A>;
impl SWIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWIP_A {
        match self.bits {
            false => SWIP_A::SWIP_0,
            true => SWIP_A::SWIP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWIP_0`"]
    #[inline(always)]
    pub fn is_swip_0(&self) -> bool {
        *self == SWIP_A::SWIP_0
    }
    #[doc = "Checks if the value of the field is `SWIP_1`"]
    #[inline(always)]
    pub fn is_swip_1(&self) -> bool {
        *self == SWIP_A::SWIP_1
    }
}
#[doc = "Write proxy for field `SWIP`"]
pub struct SWIP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn swip_0(self) -> &'a mut W {
        self.variant(SWIP_A::SWIP_0)
    }
    #[doc = "Initialize position counter"]
    #[inline(always)]
    pub fn swip_1(self) -> &'a mut W {
        self.variant(SWIP_A::SWIP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Use Negative Edge of HOME Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HNE_A {
    #[doc = "0: Use positive going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_0 = 0,
    #[doc = "1: Use negative going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_1 = 1,
}
impl From<HNE_A> for bool {
    #[inline(always)]
    fn from(variant: HNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HNE`"]
pub type HNE_R = crate::R<bool, HNE_A>;
impl HNE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HNE_A {
        match self.bits {
            false => HNE_A::HNE_0,
            true => HNE_A::HNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HNE_0`"]
    #[inline(always)]
    pub fn is_hne_0(&self) -> bool {
        *self == HNE_A::HNE_0
    }
    #[doc = "Checks if the value of the field is `HNE_1`"]
    #[inline(always)]
    pub fn is_hne_1(&self) -> bool {
        *self == HNE_A::HNE_1
    }
}
#[doc = "Write proxy for field `HNE`"]
pub struct HNE_W<'a> {
    w: &'a mut W,
}
impl<'a> HNE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HNE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use positive going edge-to-trigger initialization of position counters UPOS and LPOS"]
    #[inline(always)]
    pub fn hne_0(self) -> &'a mut W {
        self.variant(HNE_A::HNE_0)
    }
    #[doc = "Use negative going edge-to-trigger initialization of position counters UPOS and LPOS"]
    #[inline(always)]
    pub fn hne_1(self) -> &'a mut W {
        self.variant(HNE_A::HNE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Enable HOME to Initialize Position Counters UPOS and LPOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIP_A {
    #[doc = "0: No action"]
    HIP_0 = 0,
    #[doc = "1: HOME signal initializes the position counter"]
    HIP_1 = 1,
}
impl From<HIP_A> for bool {
    #[inline(always)]
    fn from(variant: HIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIP`"]
pub type HIP_R = crate::R<bool, HIP_A>;
impl HIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIP_A {
        match self.bits {
            false => HIP_A::HIP_0,
            true => HIP_A::HIP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIP_0`"]
    #[inline(always)]
    pub fn is_hip_0(&self) -> bool {
        *self == HIP_A::HIP_0
    }
    #[doc = "Checks if the value of the field is `HIP_1`"]
    #[inline(always)]
    pub fn is_hip_1(&self) -> bool {
        *self == HIP_A::HIP_1
    }
}
#[doc = "Write proxy for field `HIP`"]
pub struct HIP_W<'a> {
    w: &'a mut W,
}
impl<'a> HIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn hip_0(self) -> &'a mut W {
        self.variant(HIP_A::HIP_0)
    }
    #[doc = "HOME signal initializes the position counter"]
    #[inline(always)]
    pub fn hip_1(self) -> &'a mut W {
        self.variant(HIP_A::HIP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "HOME Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIE_A {
    #[doc = "0: Disable HOME interrupts"]
    HIE_0 = 0,
    #[doc = "1: Enable HOME interrupts"]
    HIE_1 = 1,
}
impl From<HIE_A> for bool {
    #[inline(always)]
    fn from(variant: HIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIE`"]
pub type HIE_R = crate::R<bool, HIE_A>;
impl HIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIE_A {
        match self.bits {
            false => HIE_A::HIE_0,
            true => HIE_A::HIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIE_0`"]
    #[inline(always)]
    pub fn is_hie_0(&self) -> bool {
        *self == HIE_A::HIE_0
    }
    #[doc = "Checks if the value of the field is `HIE_1`"]
    #[inline(always)]
    pub fn is_hie_1(&self) -> bool {
        *self == HIE_A::HIE_1
    }
}
#[doc = "Write proxy for field `HIE`"]
pub struct HIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable HOME interrupts"]
    #[inline(always)]
    pub fn hie_0(self) -> &'a mut W {
        self.variant(HIE_A::HIE_0)
    }
    #[doc = "Enable HOME interrupts"]
    #[inline(always)]
    pub fn hie_1(self) -> &'a mut W {
        self.variant(HIE_A::HIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "HOME Signal Transition Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIRQ_A {
    #[doc = "0: No interrupt"]
    HIRQ_0 = 0,
    #[doc = "1: HOME signal transition interrupt request"]
    HIRQ_1 = 1,
}
impl From<HIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: HIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIRQ`"]
pub type HIRQ_R = crate::R<bool, HIRQ_A>;
impl HIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRQ_A {
        match self.bits {
            false => HIRQ_A::HIRQ_0,
            true => HIRQ_A::HIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIRQ_0`"]
    #[inline(always)]
    pub fn is_hirq_0(&self) -> bool {
        *self == HIRQ_A::HIRQ_0
    }
    #[doc = "Checks if the value of the field is `HIRQ_1`"]
    #[inline(always)]
    pub fn is_hirq_1(&self) -> bool {
        *self == HIRQ_A::HIRQ_1
    }
}
#[doc = "Write proxy for field `HIRQ`"]
pub struct HIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn hirq_0(self) -> &'a mut W {
        self.variant(HIRQ_A::HIRQ_0)
    }
    #[doc = "HOME signal transition interrupt request"]
    #[inline(always)]
    pub fn hirq_1(self) -> &'a mut W {
        self.variant(HIRQ_A::HIRQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Compare Interrupt Enable"]
    #[inline(always)]
    pub fn cmpie(&self) -> CMPIE_R {
        CMPIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compare Interrupt Request"]
    #[inline(always)]
    pub fn cmpirq(&self) -> CMPIRQ_R {
        CMPIRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Watchdog Enable"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn die(&self) -> DIE_R {
        DIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog Timeout Interrupt Request"]
    #[inline(always)]
    pub fn dirq(&self) -> DIRQ_R {
        DIRQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Use Negative Edge of INDEX Pulse"]
    #[inline(always)]
    pub fn xne(&self) -> XNE_R {
        XNE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INDEX Pulse Interrupt Enable"]
    #[inline(always)]
    pub fn xie(&self) -> XIE_R {
        XIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - INDEX Pulse Interrupt Request"]
    #[inline(always)]
    pub fn xirq(&self) -> XIRQ_R {
        XIRQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Signal Phase Count Mode"]
    #[inline(always)]
    pub fn ph1(&self) -> PH1_R {
        PH1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Reverse Direction Counting"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn swip(&self) -> SWIP_R {
        SWIP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Use Negative Edge of HOME Input"]
    #[inline(always)]
    pub fn hne(&self) -> HNE_R {
        HNE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable HOME to Initialize Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn hip(&self) -> HIP_R {
        HIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - HOME Interrupt Enable"]
    #[inline(always)]
    pub fn hie(&self) -> HIE_R {
        HIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - HOME Signal Transition Interrupt Request"]
    #[inline(always)]
    pub fn hirq(&self) -> HIRQ_R {
        HIRQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Interrupt Enable"]
    #[inline(always)]
    pub fn cmpie(&mut self) -> CMPIE_W {
        CMPIE_W { w: self }
    }
    #[doc = "Bit 1 - Compare Interrupt Request"]
    #[inline(always)]
    pub fn cmpirq(&mut self) -> CMPIRQ_W {
        CMPIRQ_W { w: self }
    }
    #[doc = "Bit 2 - Watchdog Enable"]
    #[inline(always)]
    pub fn wde(&mut self) -> WDE_W {
        WDE_W { w: self }
    }
    #[doc = "Bit 3 - Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn die(&mut self) -> DIE_W {
        DIE_W { w: self }
    }
    #[doc = "Bit 4 - Watchdog Timeout Interrupt Request"]
    #[inline(always)]
    pub fn dirq(&mut self) -> DIRQ_W {
        DIRQ_W { w: self }
    }
    #[doc = "Bit 5 - Use Negative Edge of INDEX Pulse"]
    #[inline(always)]
    pub fn xne(&mut self) -> XNE_W {
        XNE_W { w: self }
    }
    #[doc = "Bit 6 - INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn xip(&mut self) -> XIP_W {
        XIP_W { w: self }
    }
    #[doc = "Bit 7 - INDEX Pulse Interrupt Enable"]
    #[inline(always)]
    pub fn xie(&mut self) -> XIE_W {
        XIE_W { w: self }
    }
    #[doc = "Bit 8 - INDEX Pulse Interrupt Request"]
    #[inline(always)]
    pub fn xirq(&mut self) -> XIRQ_W {
        XIRQ_W { w: self }
    }
    #[doc = "Bit 9 - Enable Signal Phase Count Mode"]
    #[inline(always)]
    pub fn ph1(&mut self) -> PH1_W {
        PH1_W { w: self }
    }
    #[doc = "Bit 10 - Enable Reverse Direction Counting"]
    #[inline(always)]
    pub fn rev(&mut self) -> REV_W {
        REV_W { w: self }
    }
    #[doc = "Bit 11 - Software Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn swip(&mut self) -> SWIP_W {
        SWIP_W { w: self }
    }
    #[doc = "Bit 12 - Use Negative Edge of HOME Input"]
    #[inline(always)]
    pub fn hne(&mut self) -> HNE_W {
        HNE_W { w: self }
    }
    #[doc = "Bit 13 - Enable HOME to Initialize Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn hip(&mut self) -> HIP_W {
        HIP_W { w: self }
    }
    #[doc = "Bit 14 - HOME Interrupt Enable"]
    #[inline(always)]
    pub fn hie(&mut self) -> HIE_W {
        HIE_W { w: self }
    }
    #[doc = "Bit 15 - HOME Signal Transition Interrupt Request"]
    #[inline(always)]
    pub fn hirq(&mut self) -> HIRQ_W {
        HIRQ_W { w: self }
    }
}
