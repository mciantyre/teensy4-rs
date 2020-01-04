#[doc = "Reader of register PIGEON_6_0"]
pub type R = crate::R<u32, super::PIGEON_6_0>;
#[doc = "Writer for register PIGEON_6_0"]
pub type W = crate::W<u32, super::PIGEON_6_0>;
#[doc = "Register PIGEON_6_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIGEON_6_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Polarity of signal output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_A {
    #[doc = "0: Normal Signal (Active high)"]
    ACTIVE_HIGH = 0,
    #[doc = "1: Inverted signal (Active low)"]
    ACTIVE_LOW = 1,
}
impl From<POL_A> for bool {
    #[inline(always)]
    fn from(variant: POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL`"]
pub type POL_R = crate::R<bool, POL_A>;
impl POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_A {
        match self.bits {
            false => POL_A::ACTIVE_HIGH,
            true => POL_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == POL_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == POL_A::ACTIVE_LOW
    }
}
#[doc = "Write proxy for field `POL`"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Signal (Active high)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(POL_A::ACTIVE_HIGH)
    }
    #[doc = "Inverted signal (Active low)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(POL_A::ACTIVE_LOW)
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
#[doc = "Event to incrment local counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INC_SEL_A {
    #[doc = "0: pclk"]
    PCLK = 0,
    #[doc = "1: Line start pulse"]
    LINE = 1,
    #[doc = "2: Frame start pulse"]
    FRAME = 2,
    #[doc = "3: Use another signal as tick event"]
    SIG_ANOTHER = 3,
}
impl From<INC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INC_SEL`"]
pub type INC_SEL_R = crate::R<u8, INC_SEL_A>;
impl INC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INC_SEL_A {
        match self.bits {
            0 => INC_SEL_A::PCLK,
            1 => INC_SEL_A::LINE,
            2 => INC_SEL_A::FRAME,
            3 => INC_SEL_A::SIG_ANOTHER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == INC_SEL_A::PCLK
    }
    #[doc = "Checks if the value of the field is `LINE`"]
    #[inline(always)]
    pub fn is_line(&self) -> bool {
        *self == INC_SEL_A::LINE
    }
    #[doc = "Checks if the value of the field is `FRAME`"]
    #[inline(always)]
    pub fn is_frame(&self) -> bool {
        *self == INC_SEL_A::FRAME
    }
    #[doc = "Checks if the value of the field is `SIG_ANOTHER`"]
    #[inline(always)]
    pub fn is_sig_another(&self) -> bool {
        *self == INC_SEL_A::SIG_ANOTHER
    }
}
#[doc = "Write proxy for field `INC_SEL`"]
pub struct INC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INC_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "pclk"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(INC_SEL_A::PCLK)
    }
    #[doc = "Line start pulse"]
    #[inline(always)]
    pub fn line(self) -> &'a mut W {
        self.variant(INC_SEL_A::LINE)
    }
    #[doc = "Frame start pulse"]
    #[inline(always)]
    pub fn frame(self) -> &'a mut W {
        self.variant(INC_SEL_A::FRAME)
    }
    #[doc = "Use another signal as tick event"]
    #[inline(always)]
    pub fn sig_another(self) -> &'a mut W {
        self.variant(INC_SEL_A::SIG_ANOTHER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `OFFSET`"]
pub type OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSET`"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "select global counters as mask condition, use together with MASK_CNT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASK_CNT_SEL_A {
    #[doc = "0: pclk counter within one hscan state"]
    HSTATE_CNT = 0,
    #[doc = "1: pclk cycle within one hscan state"]
    HSTATE_CYCLE = 1,
    #[doc = "2: line counter within one vscan state"]
    VSTATE_CNT = 2,
    #[doc = "3: line cycle within one vscan state"]
    VSTATE_CYCLE = 3,
    #[doc = "4: frame counter"]
    FRAME_CNT = 4,
    #[doc = "5: frame cycle"]
    FRAME_CYCLE = 5,
    #[doc = "6: horizontal counter (pclk counter within one line )"]
    HCNT = 6,
    #[doc = "7: vertical counter (line counter within one frame)"]
    VCNT = 7,
}
impl From<MASK_CNT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MASK_CNT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MASK_CNT_SEL`"]
pub type MASK_CNT_SEL_R = crate::R<u8, MASK_CNT_SEL_A>;
impl MASK_CNT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MASK_CNT_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MASK_CNT_SEL_A::HSTATE_CNT),
            1 => Val(MASK_CNT_SEL_A::HSTATE_CYCLE),
            2 => Val(MASK_CNT_SEL_A::VSTATE_CNT),
            3 => Val(MASK_CNT_SEL_A::VSTATE_CYCLE),
            4 => Val(MASK_CNT_SEL_A::FRAME_CNT),
            5 => Val(MASK_CNT_SEL_A::FRAME_CYCLE),
            6 => Val(MASK_CNT_SEL_A::HCNT),
            7 => Val(MASK_CNT_SEL_A::VCNT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSTATE_CNT`"]
    #[inline(always)]
    pub fn is_hstate_cnt(&self) -> bool {
        *self == MASK_CNT_SEL_A::HSTATE_CNT
    }
    #[doc = "Checks if the value of the field is `HSTATE_CYCLE`"]
    #[inline(always)]
    pub fn is_hstate_cycle(&self) -> bool {
        *self == MASK_CNT_SEL_A::HSTATE_CYCLE
    }
    #[doc = "Checks if the value of the field is `VSTATE_CNT`"]
    #[inline(always)]
    pub fn is_vstate_cnt(&self) -> bool {
        *self == MASK_CNT_SEL_A::VSTATE_CNT
    }
    #[doc = "Checks if the value of the field is `VSTATE_CYCLE`"]
    #[inline(always)]
    pub fn is_vstate_cycle(&self) -> bool {
        *self == MASK_CNT_SEL_A::VSTATE_CYCLE
    }
    #[doc = "Checks if the value of the field is `FRAME_CNT`"]
    #[inline(always)]
    pub fn is_frame_cnt(&self) -> bool {
        *self == MASK_CNT_SEL_A::FRAME_CNT
    }
    #[doc = "Checks if the value of the field is `FRAME_CYCLE`"]
    #[inline(always)]
    pub fn is_frame_cycle(&self) -> bool {
        *self == MASK_CNT_SEL_A::FRAME_CYCLE
    }
    #[doc = "Checks if the value of the field is `HCNT`"]
    #[inline(always)]
    pub fn is_hcnt(&self) -> bool {
        *self == MASK_CNT_SEL_A::HCNT
    }
    #[doc = "Checks if the value of the field is `VCNT`"]
    #[inline(always)]
    pub fn is_vcnt(&self) -> bool {
        *self == MASK_CNT_SEL_A::VCNT
    }
}
#[doc = "Write proxy for field `MASK_CNT_SEL`"]
pub struct MASK_CNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_CNT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_CNT_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pclk counter within one hscan state"]
    #[inline(always)]
    pub fn hstate_cnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SEL_A::HSTATE_CNT)
    }
    #[doc = "pclk cycle within one hscan state"]
    #[inline(always)]
    pub fn hstate_cycle(self) -> &'a mut W {
        self.variant(MASK_CNT_SEL_A::HSTATE_CYCLE)
    }
    #[doc = "line counter within one vscan state"]
    #[inline(always)]
    pub fn vstate_cnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SEL_A::VSTATE_CNT)
    }
    #[doc = "line cycle within one vscan state"]
    #[inline(always)]
    pub fn vstate_cycle(self) -> &'a mut W {
        self.variant(MASK_CNT_SEL_A::VSTATE_CYCLE)
    }
    #[doc = "frame counter"]
    #[inline(always)]
    pub fn frame_cnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SEL_A::FRAME_CNT)
    }
    #[doc = "frame cycle"]
    #[inline(always)]
    pub fn frame_cycle(self) -> &'a mut W {
        self.variant(MASK_CNT_SEL_A::FRAME_CYCLE)
    }
    #[doc = "horizontal counter (pclk counter within one line )"]
    #[inline(always)]
    pub fn hcnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SEL_A::HCNT)
    }
    #[doc = "vertical counter (line counter within one frame)"]
    #[inline(always)]
    pub fn vcnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SEL_A::VCNT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MASK_CNT`"]
pub type MASK_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MASK_CNT`"]
pub struct MASK_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | (((value as u32) & 0x0fff) << 12);
        self.w
    }
}
#[doc = "state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_MASK_A {
    #[doc = "1: FRAME SYNC"]
    FS = 1,
    #[doc = "2: FRAME BEGIN"]
    FB = 2,
    #[doc = "4: FRAME DATA"]
    FD = 4,
    #[doc = "8: FRAME END"]
    FE = 8,
    #[doc = "16: LINE SYNC"]
    LS = 16,
    #[doc = "32: LINE BEGIN"]
    LB = 32,
    #[doc = "64: LINE DATA"]
    LD = 64,
    #[doc = "128: LINE END"]
    LE = 128,
}
impl From<STATE_MASK_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_MASK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE_MASK`"]
pub type STATE_MASK_R = crate::R<u8, STATE_MASK_A>;
impl STATE_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_MASK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(STATE_MASK_A::FS),
            2 => Val(STATE_MASK_A::FB),
            4 => Val(STATE_MASK_A::FD),
            8 => Val(STATE_MASK_A::FE),
            16 => Val(STATE_MASK_A::LS),
            32 => Val(STATE_MASK_A::LB),
            64 => Val(STATE_MASK_A::LD),
            128 => Val(STATE_MASK_A::LE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == STATE_MASK_A::FS
    }
    #[doc = "Checks if the value of the field is `FB`"]
    #[inline(always)]
    pub fn is_fb(&self) -> bool {
        *self == STATE_MASK_A::FB
    }
    #[doc = "Checks if the value of the field is `FD`"]
    #[inline(always)]
    pub fn is_fd(&self) -> bool {
        *self == STATE_MASK_A::FD
    }
    #[doc = "Checks if the value of the field is `FE`"]
    #[inline(always)]
    pub fn is_fe(&self) -> bool {
        *self == STATE_MASK_A::FE
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == STATE_MASK_A::LS
    }
    #[doc = "Checks if the value of the field is `LB`"]
    #[inline(always)]
    pub fn is_lb(&self) -> bool {
        *self == STATE_MASK_A::LB
    }
    #[doc = "Checks if the value of the field is `LD`"]
    #[inline(always)]
    pub fn is_ld(&self) -> bool {
        *self == STATE_MASK_A::LD
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == STATE_MASK_A::LE
    }
}
#[doc = "Write proxy for field `STATE_MASK`"]
pub struct STATE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATE_MASK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FRAME SYNC"]
    #[inline(always)]
    pub fn fs(self) -> &'a mut W {
        self.variant(STATE_MASK_A::FS)
    }
    #[doc = "FRAME BEGIN"]
    #[inline(always)]
    pub fn fb(self) -> &'a mut W {
        self.variant(STATE_MASK_A::FB)
    }
    #[doc = "FRAME DATA"]
    #[inline(always)]
    pub fn fd(self) -> &'a mut W {
        self.variant(STATE_MASK_A::FD)
    }
    #[doc = "FRAME END"]
    #[inline(always)]
    pub fn fe(self) -> &'a mut W {
        self.variant(STATE_MASK_A::FE)
    }
    #[doc = "LINE SYNC"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(STATE_MASK_A::LS)
    }
    #[doc = "LINE BEGIN"]
    #[inline(always)]
    pub fn lb(self) -> &'a mut W {
        self.variant(STATE_MASK_A::LB)
    }
    #[doc = "LINE DATA"]
    #[inline(always)]
    pub fn ld(self) -> &'a mut W {
        self.variant(STATE_MASK_A::LD)
    }
    #[doc = "LINE END"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(STATE_MASK_A::LE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Polarity of signal output"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Event to incrment local counter"]
    #[inline(always)]
    pub fn inc_sel(&self) -> INC_SEL_R {
        INC_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - offset on pclk unit"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub fn mask_cnt_sel(&self) -> MASK_CNT_SEL_R {
        MASK_CNT_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:23 - When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub fn mask_cnt(&self) -> MASK_CNT_R {
        MASK_CNT_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub fn state_mask(&self) -> STATE_MASK_R {
        STATE_MASK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable pigeon Mode on this signal"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Polarity of signal output"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Bits 2:3 - Event to incrment local counter"]
    #[inline(always)]
    pub fn inc_sel(&mut self) -> INC_SEL_W {
        INC_SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - offset on pclk unit"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Bits 8:11 - select global counters as mask condition, use together with MASK_CNT"]
    #[inline(always)]
    pub fn mask_cnt_sel(&mut self) -> MASK_CNT_SEL_W {
        MASK_CNT_SEL_W { w: self }
    }
    #[doc = "Bits 12:23 - When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline(always)]
    pub fn mask_cnt(&mut self) -> MASK_CNT_W {
        MASK_CNT_W { w: self }
    }
    #[doc = "Bits 24:31 - state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline(always)]
    pub fn state_mask(&mut self) -> STATE_MASK_W {
        STATE_MASK_W { w: self }
    }
}
