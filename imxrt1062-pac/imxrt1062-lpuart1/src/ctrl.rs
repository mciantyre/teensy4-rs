#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT_A {
    #[doc = "0: Even parity."]
    PT_0 = 0,
    #[doc = "1: Odd parity."]
    PT_1 = 1,
}
impl From<PT_A> for bool {
    #[inline(always)]
    fn from(variant: PT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<bool, PT_A>;
impl PT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT_A {
        match self.bits {
            false => PT_A::PT_0,
            true => PT_A::PT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PT_0`"]
    #[inline(always)]
    pub fn is_pt_0(&self) -> bool {
        *self == PT_A::PT_0
    }
    #[doc = "Checks if the value of the field is `PT_1`"]
    #[inline(always)]
    pub fn is_pt_1(&self) -> bool {
        *self == PT_A::PT_1
    }
}
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Even parity."]
    #[inline(always)]
    pub fn pt_0(self) -> &'a mut W {
        self.variant(PT_A::PT_0)
    }
    #[doc = "Odd parity."]
    #[inline(always)]
    pub fn pt_1(self) -> &'a mut W {
        self.variant(PT_A::PT_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: No hardware parity generation or checking."]
    PE_0 = 0,
    #[doc = "1: Parity enabled."]
    PE_1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::PE_0,
            true => PE_A::PE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PE_0`"]
    #[inline(always)]
    pub fn is_pe_0(&self) -> bool {
        *self == PE_A::PE_0
    }
    #[doc = "Checks if the value of the field is `PE_1`"]
    #[inline(always)]
    pub fn is_pe_1(&self) -> bool {
        *self == PE_A::PE_1
    }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No hardware parity generation or checking."]
    #[inline(always)]
    pub fn pe_0(self) -> &'a mut W {
        self.variant(PE_A::PE_0)
    }
    #[doc = "Parity enabled."]
    #[inline(always)]
    pub fn pe_1(self) -> &'a mut W {
        self.variant(PE_A::PE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Idle Line Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILT_A {
    #[doc = "0: Idle character bit count starts after start bit."]
    ILT_0 = 0,
    #[doc = "1: Idle character bit count starts after stop bit."]
    ILT_1 = 1,
}
impl From<ILT_A> for bool {
    #[inline(always)]
    fn from(variant: ILT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILT`"]
pub type ILT_R = crate::R<bool, ILT_A>;
impl ILT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILT_A {
        match self.bits {
            false => ILT_A::ILT_0,
            true => ILT_A::ILT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ILT_0`"]
    #[inline(always)]
    pub fn is_ilt_0(&self) -> bool {
        *self == ILT_A::ILT_0
    }
    #[doc = "Checks if the value of the field is `ILT_1`"]
    #[inline(always)]
    pub fn is_ilt_1(&self) -> bool {
        *self == ILT_A::ILT_1
    }
}
#[doc = "Write proxy for field `ILT`"]
pub struct ILT_W<'a> {
    w: &'a mut W,
}
impl<'a> ILT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Idle character bit count starts after start bit."]
    #[inline(always)]
    pub fn ilt_0(self) -> &'a mut W {
        self.variant(ILT_A::ILT_0)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline(always)]
    pub fn ilt_1(self) -> &'a mut W {
        self.variant(ILT_A::ILT_1)
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
#[doc = "Receiver Wakeup Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_A {
    #[doc = "0: Configures RWU for idle-line wakeup."]
    WAKE_0 = 0,
    #[doc = "1: Configures RWU with address-mark wakeup."]
    WAKE_1 = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKE`"]
pub type WAKE_R = crate::R<bool, WAKE_A>;
impl WAKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::WAKE_0,
            true => WAKE_A::WAKE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKE_0`"]
    #[inline(always)]
    pub fn is_wake_0(&self) -> bool {
        *self == WAKE_A::WAKE_0
    }
    #[doc = "Checks if the value of the field is `WAKE_1`"]
    #[inline(always)]
    pub fn is_wake_1(&self) -> bool {
        *self == WAKE_A::WAKE_1
    }
}
#[doc = "Write proxy for field `WAKE`"]
pub struct WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configures RWU for idle-line wakeup."]
    #[inline(always)]
    pub fn wake_0(self) -> &'a mut W {
        self.variant(WAKE_A::WAKE_0)
    }
    #[doc = "Configures RWU with address-mark wakeup."]
    #[inline(always)]
    pub fn wake_1(self) -> &'a mut W {
        self.variant(WAKE_A::WAKE_1)
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
#[doc = "9-Bit or 8-Bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_A {
    #[doc = "0: Receiver and transmitter use 8-bit data characters."]
    M_0 = 0,
    #[doc = "1: Receiver and transmitter use 9-bit data characters."]
    M_1 = 1,
}
impl From<M_A> for bool {
    #[inline(always)]
    fn from(variant: M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `M`"]
pub type M_R = crate::R<bool, M_A>;
impl M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_A {
        match self.bits {
            false => M_A::M_0,
            true => M_A::M_1,
        }
    }
    #[doc = "Checks if the value of the field is `M_0`"]
    #[inline(always)]
    pub fn is_m_0(&self) -> bool {
        *self == M_A::M_0
    }
    #[doc = "Checks if the value of the field is `M_1`"]
    #[inline(always)]
    pub fn is_m_1(&self) -> bool {
        *self == M_A::M_1
    }
}
#[doc = "Write proxy for field `M`"]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    #[inline(always)]
    pub fn m_0(self) -> &'a mut W {
        self.variant(M_A::M_0)
    }
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    #[inline(always)]
    pub fn m_1(self) -> &'a mut W {
        self.variant(M_A::M_1)
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
#[doc = "Receiver Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSRC_A {
    #[doc = "0: Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    RSRC_0 = 0,
    #[doc = "1: Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    RSRC_1 = 1,
}
impl From<RSRC_A> for bool {
    #[inline(always)]
    fn from(variant: RSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSRC`"]
pub type RSRC_R = crate::R<bool, RSRC_A>;
impl RSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSRC_A {
        match self.bits {
            false => RSRC_A::RSRC_0,
            true => RSRC_A::RSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSRC_0`"]
    #[inline(always)]
    pub fn is_rsrc_0(&self) -> bool {
        *self == RSRC_A::RSRC_0
    }
    #[doc = "Checks if the value of the field is `RSRC_1`"]
    #[inline(always)]
    pub fn is_rsrc_1(&self) -> bool {
        *self == RSRC_A::RSRC_1
    }
}
#[doc = "Write proxy for field `RSRC`"]
pub struct RSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    #[inline(always)]
    pub fn rsrc_0(self) -> &'a mut W {
        self.variant(RSRC_A::RSRC_0)
    }
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    #[inline(always)]
    pub fn rsrc_1(self) -> &'a mut W {
        self.variant(RSRC_A::RSRC_1)
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
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEEN_A {
    #[doc = "0: LPUART is enabled in Doze mode."]
    DOZEEN_0 = 0,
    #[doc = "1: LPUART is disabled in Doze mode."]
    DOZEEN_1 = 1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOZEEN`"]
pub type DOZEEN_R = crate::R<bool, DOZEEN_A>;
impl DOZEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::DOZEEN_0,
            true => DOZEEN_A::DOZEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEEN_0`"]
    #[inline(always)]
    pub fn is_dozeen_0(&self) -> bool {
        *self == DOZEEN_A::DOZEEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEEN_1`"]
    #[inline(always)]
    pub fn is_dozeen_1(&self) -> bool {
        *self == DOZEEN_A::DOZEEN_1
    }
}
#[doc = "Write proxy for field `DOZEEN`"]
pub struct DOZEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPUART is enabled in Doze mode."]
    #[inline(always)]
    pub fn dozeen_0(self) -> &'a mut W {
        self.variant(DOZEEN_A::DOZEEN_0)
    }
    #[doc = "LPUART is disabled in Doze mode."]
    #[inline(always)]
    pub fn dozeen_1(self) -> &'a mut W {
        self.variant(DOZEEN_A::DOZEEN_1)
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
#[doc = "Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPS_A {
    #[doc = "0: Normal operation - RXD and TXD use separate pins."]
    LOOPS_0 = 0,
    #[doc = "1: Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    LOOPS_1 = 1,
}
impl From<LOOPS_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOOPS`"]
pub type LOOPS_R = crate::R<bool, LOOPS_A>;
impl LOOPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPS_A {
        match self.bits {
            false => LOOPS_A::LOOPS_0,
            true => LOOPS_A::LOOPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOOPS_0`"]
    #[inline(always)]
    pub fn is_loops_0(&self) -> bool {
        *self == LOOPS_A::LOOPS_0
    }
    #[doc = "Checks if the value of the field is `LOOPS_1`"]
    #[inline(always)]
    pub fn is_loops_1(&self) -> bool {
        *self == LOOPS_A::LOOPS_1
    }
}
#[doc = "Write proxy for field `LOOPS`"]
pub struct LOOPS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation - RXD and TXD use separate pins."]
    #[inline(always)]
    pub fn loops_0(self) -> &'a mut W {
        self.variant(LOOPS_A::LOOPS_0)
    }
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    #[inline(always)]
    pub fn loops_1(self) -> &'a mut W {
        self.variant(LOOPS_A::LOOPS_1)
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
#[doc = "Idle Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDLECFG_A {
    #[doc = "0: 1 idle character"]
    IDLECFG_0 = 0,
    #[doc = "1: 2 idle characters"]
    IDLECFG_1 = 1,
    #[doc = "2: 4 idle characters"]
    IDLECFG_2 = 2,
    #[doc = "3: 8 idle characters"]
    IDLECFG_3 = 3,
    #[doc = "4: 16 idle characters"]
    IDLECFG_4 = 4,
    #[doc = "5: 32 idle characters"]
    IDLECFG_5 = 5,
    #[doc = "6: 64 idle characters"]
    IDLECFG_6 = 6,
    #[doc = "7: 128 idle characters"]
    IDLECFG_7 = 7,
}
impl From<IDLECFG_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IDLECFG`"]
pub type IDLECFG_R = crate::R<u8, IDLECFG_A>;
impl IDLECFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLECFG_A {
        match self.bits {
            0 => IDLECFG_A::IDLECFG_0,
            1 => IDLECFG_A::IDLECFG_1,
            2 => IDLECFG_A::IDLECFG_2,
            3 => IDLECFG_A::IDLECFG_3,
            4 => IDLECFG_A::IDLECFG_4,
            5 => IDLECFG_A::IDLECFG_5,
            6 => IDLECFG_A::IDLECFG_6,
            7 => IDLECFG_A::IDLECFG_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLECFG_0`"]
    #[inline(always)]
    pub fn is_idlecfg_0(&self) -> bool {
        *self == IDLECFG_A::IDLECFG_0
    }
    #[doc = "Checks if the value of the field is `IDLECFG_1`"]
    #[inline(always)]
    pub fn is_idlecfg_1(&self) -> bool {
        *self == IDLECFG_A::IDLECFG_1
    }
    #[doc = "Checks if the value of the field is `IDLECFG_2`"]
    #[inline(always)]
    pub fn is_idlecfg_2(&self) -> bool {
        *self == IDLECFG_A::IDLECFG_2
    }
    #[doc = "Checks if the value of the field is `IDLECFG_3`"]
    #[inline(always)]
    pub fn is_idlecfg_3(&self) -> bool {
        *self == IDLECFG_A::IDLECFG_3
    }
    #[doc = "Checks if the value of the field is `IDLECFG_4`"]
    #[inline(always)]
    pub fn is_idlecfg_4(&self) -> bool {
        *self == IDLECFG_A::IDLECFG_4
    }
    #[doc = "Checks if the value of the field is `IDLECFG_5`"]
    #[inline(always)]
    pub fn is_idlecfg_5(&self) -> bool {
        *self == IDLECFG_A::IDLECFG_5
    }
    #[doc = "Checks if the value of the field is `IDLECFG_6`"]
    #[inline(always)]
    pub fn is_idlecfg_6(&self) -> bool {
        *self == IDLECFG_A::IDLECFG_6
    }
    #[doc = "Checks if the value of the field is `IDLECFG_7`"]
    #[inline(always)]
    pub fn is_idlecfg_7(&self) -> bool {
        *self == IDLECFG_A::IDLECFG_7
    }
}
#[doc = "Write proxy for field `IDLECFG`"]
pub struct IDLECFG_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 idle character"]
    #[inline(always)]
    pub fn idlecfg_0(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLECFG_0)
    }
    #[doc = "2 idle characters"]
    #[inline(always)]
    pub fn idlecfg_1(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLECFG_1)
    }
    #[doc = "4 idle characters"]
    #[inline(always)]
    pub fn idlecfg_2(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLECFG_2)
    }
    #[doc = "8 idle characters"]
    #[inline(always)]
    pub fn idlecfg_3(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLECFG_3)
    }
    #[doc = "16 idle characters"]
    #[inline(always)]
    pub fn idlecfg_4(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLECFG_4)
    }
    #[doc = "32 idle characters"]
    #[inline(always)]
    pub fn idlecfg_5(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLECFG_5)
    }
    #[doc = "64 idle characters"]
    #[inline(always)]
    pub fn idlecfg_6(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLECFG_6)
    }
    #[doc = "128 idle characters"]
    #[inline(always)]
    pub fn idlecfg_7(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLECFG_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "7-Bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7_A {
    #[doc = "0: Receiver and transmitter use 8-bit to 10-bit data characters."]
    M7_0 = 0,
    #[doc = "1: Receiver and transmitter use 7-bit data characters."]
    M7_1 = 1,
}
impl From<M7_A> for bool {
    #[inline(always)]
    fn from(variant: M7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `M7`"]
pub type M7_R = crate::R<bool, M7_A>;
impl M7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7_A {
        match self.bits {
            false => M7_A::M7_0,
            true => M7_A::M7_1,
        }
    }
    #[doc = "Checks if the value of the field is `M7_0`"]
    #[inline(always)]
    pub fn is_m7_0(&self) -> bool {
        *self == M7_A::M7_0
    }
    #[doc = "Checks if the value of the field is `M7_1`"]
    #[inline(always)]
    pub fn is_m7_1(&self) -> bool {
        *self == M7_A::M7_1
    }
}
#[doc = "Write proxy for field `M7`"]
pub struct M7_W<'a> {
    w: &'a mut W,
}
impl<'a> M7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."]
    #[inline(always)]
    pub fn m7_0(self) -> &'a mut W {
        self.variant(M7_A::M7_0)
    }
    #[doc = "Receiver and transmitter use 7-bit data characters."]
    #[inline(always)]
    pub fn m7_1(self) -> &'a mut W {
        self.variant(M7_A::M7_1)
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
#[doc = "Match 2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA2IE_A {
    #[doc = "0: MA2F interrupt disabled"]
    MA2IE_0 = 0,
    #[doc = "1: MA2F interrupt enabled"]
    MA2IE_1 = 1,
}
impl From<MA2IE_A> for bool {
    #[inline(always)]
    fn from(variant: MA2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MA2IE`"]
pub type MA2IE_R = crate::R<bool, MA2IE_A>;
impl MA2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA2IE_A {
        match self.bits {
            false => MA2IE_A::MA2IE_0,
            true => MA2IE_A::MA2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MA2IE_0`"]
    #[inline(always)]
    pub fn is_ma2ie_0(&self) -> bool {
        *self == MA2IE_A::MA2IE_0
    }
    #[doc = "Checks if the value of the field is `MA2IE_1`"]
    #[inline(always)]
    pub fn is_ma2ie_1(&self) -> bool {
        *self == MA2IE_A::MA2IE_1
    }
}
#[doc = "Write proxy for field `MA2IE`"]
pub struct MA2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MA2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MA2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MA2F interrupt disabled"]
    #[inline(always)]
    pub fn ma2ie_0(self) -> &'a mut W {
        self.variant(MA2IE_A::MA2IE_0)
    }
    #[doc = "MA2F interrupt enabled"]
    #[inline(always)]
    pub fn ma2ie_1(self) -> &'a mut W {
        self.variant(MA2IE_A::MA2IE_1)
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
#[doc = "Match 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA1IE_A {
    #[doc = "0: MA1F interrupt disabled"]
    MA1IE_0 = 0,
    #[doc = "1: MA1F interrupt enabled"]
    MA1IE_1 = 1,
}
impl From<MA1IE_A> for bool {
    #[inline(always)]
    fn from(variant: MA1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MA1IE`"]
pub type MA1IE_R = crate::R<bool, MA1IE_A>;
impl MA1IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA1IE_A {
        match self.bits {
            false => MA1IE_A::MA1IE_0,
            true => MA1IE_A::MA1IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MA1IE_0`"]
    #[inline(always)]
    pub fn is_ma1ie_0(&self) -> bool {
        *self == MA1IE_A::MA1IE_0
    }
    #[doc = "Checks if the value of the field is `MA1IE_1`"]
    #[inline(always)]
    pub fn is_ma1ie_1(&self) -> bool {
        *self == MA1IE_A::MA1IE_1
    }
}
#[doc = "Write proxy for field `MA1IE`"]
pub struct MA1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MA1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MA1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MA1F interrupt disabled"]
    #[inline(always)]
    pub fn ma1ie_0(self) -> &'a mut W {
        self.variant(MA1IE_A::MA1IE_0)
    }
    #[doc = "MA1F interrupt enabled"]
    #[inline(always)]
    pub fn ma1ie_1(self) -> &'a mut W {
        self.variant(MA1IE_A::MA1IE_1)
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
#[doc = "Send Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBK_A {
    #[doc = "0: Normal transmitter operation."]
    SBK_0 = 0,
    #[doc = "1: Queue break character(s) to be sent."]
    SBK_1 = 1,
}
impl From<SBK_A> for bool {
    #[inline(always)]
    fn from(variant: SBK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBK`"]
pub type SBK_R = crate::R<bool, SBK_A>;
impl SBK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBK_A {
        match self.bits {
            false => SBK_A::SBK_0,
            true => SBK_A::SBK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBK_0`"]
    #[inline(always)]
    pub fn is_sbk_0(&self) -> bool {
        *self == SBK_A::SBK_0
    }
    #[doc = "Checks if the value of the field is `SBK_1`"]
    #[inline(always)]
    pub fn is_sbk_1(&self) -> bool {
        *self == SBK_A::SBK_1
    }
}
#[doc = "Write proxy for field `SBK`"]
pub struct SBK_W<'a> {
    w: &'a mut W,
}
impl<'a> SBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal transmitter operation."]
    #[inline(always)]
    pub fn sbk_0(self) -> &'a mut W {
        self.variant(SBK_A::SBK_0)
    }
    #[doc = "Queue break character(s) to be sent."]
    #[inline(always)]
    pub fn sbk_1(self) -> &'a mut W {
        self.variant(SBK_A::SBK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Receiver Wakeup Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWU_A {
    #[doc = "0: Normal receiver operation."]
    RWU_0 = 0,
    #[doc = "1: LPUART receiver in standby waiting for wakeup condition."]
    RWU_1 = 1,
}
impl From<RWU_A> for bool {
    #[inline(always)]
    fn from(variant: RWU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RWU`"]
pub type RWU_R = crate::R<bool, RWU_A>;
impl RWU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWU_A {
        match self.bits {
            false => RWU_A::RWU_0,
            true => RWU_A::RWU_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWU_0`"]
    #[inline(always)]
    pub fn is_rwu_0(&self) -> bool {
        *self == RWU_A::RWU_0
    }
    #[doc = "Checks if the value of the field is `RWU_1`"]
    #[inline(always)]
    pub fn is_rwu_1(&self) -> bool {
        *self == RWU_A::RWU_1
    }
}
#[doc = "Write proxy for field `RWU`"]
pub struct RWU_W<'a> {
    w: &'a mut W,
}
impl<'a> RWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal receiver operation."]
    #[inline(always)]
    pub fn rwu_0(self) -> &'a mut W {
        self.variant(RWU_A::RWU_0)
    }
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    #[inline(always)]
    pub fn rwu_1(self) -> &'a mut W {
        self.variant(RWU_A::RWU_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: Receiver disabled."]
    RE_0 = 0,
    #[doc = "1: Receiver enabled."]
    RE_1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, RE_A>;
impl RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::RE_0,
            true => RE_A::RE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RE_0`"]
    #[inline(always)]
    pub fn is_re_0(&self) -> bool {
        *self == RE_A::RE_0
    }
    #[doc = "Checks if the value of the field is `RE_1`"]
    #[inline(always)]
    pub fn is_re_1(&self) -> bool {
        *self == RE_A::RE_1
    }
}
#[doc = "Write proxy for field `RE`"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver disabled."]
    #[inline(always)]
    pub fn re_0(self) -> &'a mut W {
        self.variant(RE_A::RE_0)
    }
    #[doc = "Receiver enabled."]
    #[inline(always)]
    pub fn re_1(self) -> &'a mut W {
        self.variant(RE_A::RE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    #[doc = "0: Transmitter disabled."]
    TE_0 = 0,
    #[doc = "1: Transmitter enabled."]
    TE_1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TE`"]
pub type TE_R = crate::R<bool, TE_A>;
impl TE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::TE_0,
            true => TE_A::TE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TE_0`"]
    #[inline(always)]
    pub fn is_te_0(&self) -> bool {
        *self == TE_A::TE_0
    }
    #[doc = "Checks if the value of the field is `TE_1`"]
    #[inline(always)]
    pub fn is_te_1(&self) -> bool {
        *self == TE_A::TE_1
    }
}
#[doc = "Write proxy for field `TE`"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmitter disabled."]
    #[inline(always)]
    pub fn te_0(self) -> &'a mut W {
        self.variant(TE_A::TE_0)
    }
    #[doc = "Transmitter enabled."]
    #[inline(always)]
    pub fn te_1(self) -> &'a mut W {
        self.variant(TE_A::TE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Idle Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIE_A {
    #[doc = "0: Hardware interrupts from IDLE disabled; use polling."]
    ILIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when IDLE flag is 1."]
    ILIE_1 = 1,
}
impl From<ILIE_A> for bool {
    #[inline(always)]
    fn from(variant: ILIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILIE`"]
pub type ILIE_R = crate::R<bool, ILIE_A>;
impl ILIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIE_A {
        match self.bits {
            false => ILIE_A::ILIE_0,
            true => ILIE_A::ILIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ILIE_0`"]
    #[inline(always)]
    pub fn is_ilie_0(&self) -> bool {
        *self == ILIE_A::ILIE_0
    }
    #[doc = "Checks if the value of the field is `ILIE_1`"]
    #[inline(always)]
    pub fn is_ilie_1(&self) -> bool {
        *self == ILIE_A::ILIE_1
    }
}
#[doc = "Write proxy for field `ILIE`"]
pub struct ILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    #[inline(always)]
    pub fn ilie_0(self) -> &'a mut W {
        self.variant(ILIE_A::ILIE_0)
    }
    #[doc = "Hardware interrupt requested when IDLE flag is 1."]
    #[inline(always)]
    pub fn ilie_1(self) -> &'a mut W {
        self.variant(ILIE_A::ILIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Receiver Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIE_A {
    #[doc = "0: Hardware interrupts from RDRF disabled; use polling."]
    RIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when RDRF flag is 1."]
    RIE_1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RIE`"]
pub type RIE_R = crate::R<bool, RIE_A>;
impl RIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::RIE_0,
            true => RIE_A::RIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RIE_0`"]
    #[inline(always)]
    pub fn is_rie_0(&self) -> bool {
        *self == RIE_A::RIE_0
    }
    #[doc = "Checks if the value of the field is `RIE_1`"]
    #[inline(always)]
    pub fn is_rie_1(&self) -> bool {
        *self == RIE_A::RIE_1
    }
}
#[doc = "Write proxy for field `RIE`"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from RDRF disabled; use polling."]
    #[inline(always)]
    pub fn rie_0(self) -> &'a mut W {
        self.variant(RIE_A::RIE_0)
    }
    #[doc = "Hardware interrupt requested when RDRF flag is 1."]
    #[inline(always)]
    pub fn rie_1(self) -> &'a mut W {
        self.variant(RIE_A::RIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Transmission Complete Interrupt Enable for\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: Hardware interrupts from TC disabled; use polling."]
    TCIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when TC flag is 1."]
    TCIE_1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIE`"]
pub type TCIE_R = crate::R<bool, TCIE_A>;
impl TCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::TCIE_0,
            true => TCIE_A::TCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCIE_0`"]
    #[inline(always)]
    pub fn is_tcie_0(&self) -> bool {
        *self == TCIE_A::TCIE_0
    }
    #[doc = "Checks if the value of the field is `TCIE_1`"]
    #[inline(always)]
    pub fn is_tcie_1(&self) -> bool {
        *self == TCIE_A::TCIE_1
    }
}
#[doc = "Write proxy for field `TCIE`"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from TC disabled; use polling."]
    #[inline(always)]
    pub fn tcie_0(self) -> &'a mut W {
        self.variant(TCIE_A::TCIE_0)
    }
    #[doc = "Hardware interrupt requested when TC flag is 1."]
    #[inline(always)]
    pub fn tcie_1(self) -> &'a mut W {
        self.variant(TCIE_A::TCIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Hardware interrupts from TDRE disabled; use polling."]
    TIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when TDRE flag is 1."]
    TIE_1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::TIE_0,
            true => TIE_A::TIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIE_0`"]
    #[inline(always)]
    pub fn is_tie_0(&self) -> bool {
        *self == TIE_A::TIE_0
    }
    #[doc = "Checks if the value of the field is `TIE_1`"]
    #[inline(always)]
    pub fn is_tie_1(&self) -> bool {
        *self == TIE_A::TIE_1
    }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from TDRE disabled; use polling."]
    #[inline(always)]
    pub fn tie_0(self) -> &'a mut W {
        self.variant(TIE_A::TIE_0)
    }
    #[doc = "Hardware interrupt requested when TDRE flag is 1."]
    #[inline(always)]
    pub fn tie_1(self) -> &'a mut W {
        self.variant(TIE_A::TIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIE_A {
    #[doc = "0: PF interrupts disabled; use polling)."]
    PEIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when PF is set."]
    PEIE_1 = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEIE`"]
pub type PEIE_R = crate::R<bool, PEIE_A>;
impl PEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::PEIE_0,
            true => PEIE_A::PEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PEIE_0`"]
    #[inline(always)]
    pub fn is_peie_0(&self) -> bool {
        *self == PEIE_A::PEIE_0
    }
    #[doc = "Checks if the value of the field is `PEIE_1`"]
    #[inline(always)]
    pub fn is_peie_1(&self) -> bool {
        *self == PEIE_A::PEIE_1
    }
}
#[doc = "Write proxy for field `PEIE`"]
pub struct PEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PF interrupts disabled; use polling)."]
    #[inline(always)]
    pub fn peie_0(self) -> &'a mut W {
        self.variant(PEIE_A::PEIE_0)
    }
    #[doc = "Hardware interrupt requested when PF is set."]
    #[inline(always)]
    pub fn peie_1(self) -> &'a mut W {
        self.variant(PEIE_A::PEIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: FE interrupts disabled; use polling."]
    FEIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when FE is set."]
    FEIE_1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEIE`"]
pub type FEIE_R = crate::R<bool, FEIE_A>;
impl FEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::FEIE_0,
            true => FEIE_A::FEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEIE_0`"]
    #[inline(always)]
    pub fn is_feie_0(&self) -> bool {
        *self == FEIE_A::FEIE_0
    }
    #[doc = "Checks if the value of the field is `FEIE_1`"]
    #[inline(always)]
    pub fn is_feie_1(&self) -> bool {
        *self == FEIE_A::FEIE_1
    }
}
#[doc = "Write proxy for field `FEIE`"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FE interrupts disabled; use polling."]
    #[inline(always)]
    pub fn feie_0(self) -> &'a mut W {
        self.variant(FEIE_A::FEIE_0)
    }
    #[doc = "Hardware interrupt requested when FE is set."]
    #[inline(always)]
    pub fn feie_1(self) -> &'a mut W {
        self.variant(FEIE_A::FEIE_1)
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
#[doc = "Noise Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEIE_A {
    #[doc = "0: NF interrupts disabled; use polling."]
    NEIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when NF is set."]
    NEIE_1 = 1,
}
impl From<NEIE_A> for bool {
    #[inline(always)]
    fn from(variant: NEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NEIE`"]
pub type NEIE_R = crate::R<bool, NEIE_A>;
impl NEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEIE_A {
        match self.bits {
            false => NEIE_A::NEIE_0,
            true => NEIE_A::NEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `NEIE_0`"]
    #[inline(always)]
    pub fn is_neie_0(&self) -> bool {
        *self == NEIE_A::NEIE_0
    }
    #[doc = "Checks if the value of the field is `NEIE_1`"]
    #[inline(always)]
    pub fn is_neie_1(&self) -> bool {
        *self == NEIE_A::NEIE_1
    }
}
#[doc = "Write proxy for field `NEIE`"]
pub struct NEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NF interrupts disabled; use polling."]
    #[inline(always)]
    pub fn neie_0(self) -> &'a mut W {
        self.variant(NEIE_A::NEIE_0)
    }
    #[doc = "Hardware interrupt requested when NF is set."]
    #[inline(always)]
    pub fn neie_1(self) -> &'a mut W {
        self.variant(NEIE_A::NEIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORIE_A {
    #[doc = "0: OR interrupts disabled; use polling."]
    ORIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when OR is set."]
    ORIE_1 = 1,
}
impl From<ORIE_A> for bool {
    #[inline(always)]
    fn from(variant: ORIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ORIE`"]
pub type ORIE_R = crate::R<bool, ORIE_A>;
impl ORIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORIE_A {
        match self.bits {
            false => ORIE_A::ORIE_0,
            true => ORIE_A::ORIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ORIE_0`"]
    #[inline(always)]
    pub fn is_orie_0(&self) -> bool {
        *self == ORIE_A::ORIE_0
    }
    #[doc = "Checks if the value of the field is `ORIE_1`"]
    #[inline(always)]
    pub fn is_orie_1(&self) -> bool {
        *self == ORIE_A::ORIE_1
    }
}
#[doc = "Write proxy for field `ORIE`"]
pub struct ORIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ORIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ORIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OR interrupts disabled; use polling."]
    #[inline(always)]
    pub fn orie_0(self) -> &'a mut W {
        self.variant(ORIE_A::ORIE_0)
    }
    #[doc = "Hardware interrupt requested when OR is set."]
    #[inline(always)]
    pub fn orie_1(self) -> &'a mut W {
        self.variant(ORIE_A::ORIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Transmit Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINV_A {
    #[doc = "0: Transmit data not inverted."]
    TXINV_0 = 0,
    #[doc = "1: Transmit data inverted."]
    TXINV_1 = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXINV`"]
pub type TXINV_R = crate::R<bool, TXINV_A>;
impl TXINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::TXINV_0,
            true => TXINV_A::TXINV_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXINV_0`"]
    #[inline(always)]
    pub fn is_txinv_0(&self) -> bool {
        *self == TXINV_A::TXINV_0
    }
    #[doc = "Checks if the value of the field is `TXINV_1`"]
    #[inline(always)]
    pub fn is_txinv_1(&self) -> bool {
        *self == TXINV_A::TXINV_1
    }
}
#[doc = "Write proxy for field `TXINV`"]
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit data not inverted."]
    #[inline(always)]
    pub fn txinv_0(self) -> &'a mut W {
        self.variant(TXINV_A::TXINV_0)
    }
    #[doc = "Transmit data inverted."]
    #[inline(always)]
    pub fn txinv_1(self) -> &'a mut W {
        self.variant(TXINV_A::TXINV_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "TXD Pin Direction in Single-Wire Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIR_A {
    #[doc = "0: TXD pin is an input in single-wire mode."]
    TXDIR_0 = 0,
    #[doc = "1: TXD pin is an output in single-wire mode."]
    TXDIR_1 = 1,
}
impl From<TXDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TXDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDIR`"]
pub type TXDIR_R = crate::R<bool, TXDIR_A>;
impl TXDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDIR_A {
        match self.bits {
            false => TXDIR_A::TXDIR_0,
            true => TXDIR_A::TXDIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDIR_0`"]
    #[inline(always)]
    pub fn is_txdir_0(&self) -> bool {
        *self == TXDIR_A::TXDIR_0
    }
    #[doc = "Checks if the value of the field is `TXDIR_1`"]
    #[inline(always)]
    pub fn is_txdir_1(&self) -> bool {
        *self == TXDIR_A::TXDIR_1
    }
}
#[doc = "Write proxy for field `TXDIR`"]
pub struct TXDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXD pin is an input in single-wire mode."]
    #[inline(always)]
    pub fn txdir_0(self) -> &'a mut W {
        self.variant(TXDIR_A::TXDIR_0)
    }
    #[doc = "TXD pin is an output in single-wire mode."]
    #[inline(always)]
    pub fn txdir_1(self) -> &'a mut W {
        self.variant(TXDIR_A::TXDIR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `R9T8`"]
pub type R9T8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R9T8`"]
pub struct R9T8_W<'a> {
    w: &'a mut W,
}
impl<'a> R9T8_W<'a> {
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
#[doc = "Reader of field `R8T9`"]
pub type R8T9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R8T9`"]
pub struct R8T9_W<'a> {
    w: &'a mut W,
}
impl<'a> R8T9_W<'a> {
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
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&self) -> ILT_R {
        ILT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&self) -> RSRC_R {
        RSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&self) -> LOOPS_R {
        LOOPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline(always)]
    pub fn idlecfg(&self) -> IDLECFG_R {
        IDLECFG_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - 7-Bit Mode Select"]
    #[inline(always)]
    pub fn m7(&self) -> M7_R {
        M7_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ma2ie(&self) -> MA2IE_R {
        MA2IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ma1ie(&self) -> MA1IE_R {
        MA1IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&self) -> ILIE_R {
        ILIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&self) -> NEIE_R {
        NEIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> ORIE_R {
        ORIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TXD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub fn txdir(&self) -> TXDIR_R {
        TXDIR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    pub fn r9t8(&self) -> R9T8_R {
        R9T8_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    pub fn r8t9(&self) -> R8T9_R {
        R8T9_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&mut self) -> ILT_W {
        ILT_W { w: self }
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W {
        WAKE_W { w: self }
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&mut self) -> RSRC_W {
        RSRC_W { w: self }
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&mut self) -> DOZEEN_W {
        DOZEEN_W { w: self }
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&mut self) -> LOOPS_W {
        LOOPS_W { w: self }
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline(always)]
    pub fn idlecfg(&mut self) -> IDLECFG_W {
        IDLECFG_W { w: self }
    }
    #[doc = "Bit 11 - 7-Bit Mode Select"]
    #[inline(always)]
    pub fn m7(&mut self) -> M7_W {
        M7_W { w: self }
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ma2ie(&mut self) -> MA2IE_W {
        MA2IE_W { w: self }
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ma1ie(&mut self) -> MA1IE_W {
        MA1IE_W { w: self }
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SBK_W {
        SBK_W { w: self }
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&mut self) -> RWU_W {
        RWU_W { w: self }
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&mut self) -> ILIE_W {
        ILIE_W { w: self }
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W {
        PEIE_W { w: self }
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&mut self) -> NEIE_W {
        NEIE_W { w: self }
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&mut self) -> ORIE_W {
        ORIE_W { w: self }
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    #[doc = "Bit 29 - TXD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub fn txdir(&mut self) -> TXDIR_W {
        TXDIR_W { w: self }
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    pub fn r9t8(&mut self) -> R9T8_W {
        R9T8_W { w: self }
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    pub fn r8t9(&mut self) -> R8T9_W {
        R8T9_W { w: self }
    }
}
