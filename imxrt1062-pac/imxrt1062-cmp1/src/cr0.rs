#[doc = "Reader of register CR0"]
pub type R = crate::R<u8, super::CR0>;
#[doc = "Writer for register CR0"]
pub type W = crate::W<u8, super::CR0>;
#[doc = "Register CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator hard block hysteresis control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYSTCTR_A {
    #[doc = "0: Level 0"]
    HYSTCTR_0 = 0,
    #[doc = "1: Level 1"]
    HYSTCTR_1 = 1,
    #[doc = "2: Level 2"]
    HYSTCTR_2 = 2,
    #[doc = "3: Level 3"]
    HYSTCTR_3 = 3,
}
impl From<HYSTCTR_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSTCTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HYSTCTR`"]
pub type HYSTCTR_R = crate::R<u8, HYSTCTR_A>;
impl HYSTCTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSTCTR_A {
        match self.bits {
            0 => HYSTCTR_A::HYSTCTR_0,
            1 => HYSTCTR_A::HYSTCTR_1,
            2 => HYSTCTR_A::HYSTCTR_2,
            3 => HYSTCTR_A::HYSTCTR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_0`"]
    #[inline(always)]
    pub fn is_hystctr_0(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_0
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_1`"]
    #[inline(always)]
    pub fn is_hystctr_1(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_1
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_2`"]
    #[inline(always)]
    pub fn is_hystctr_2(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_2
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_3`"]
    #[inline(always)]
    pub fn is_hystctr_3(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_3
    }
}
#[doc = "Write proxy for field `HYSTCTR`"]
pub struct HYSTCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> HYSTCTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYSTCTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level 0"]
    #[inline(always)]
    pub fn hystctr_0(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_0)
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn hystctr_1(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_1)
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn hystctr_2(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_2)
    }
    #[doc = "Level 3"]
    #[inline(always)]
    pub fn hystctr_3(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Filter Sample Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_CNT_A {
    #[doc = "0: Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    FILTER_CNT_0 = 0,
    #[doc = "1: One sample must agree. The comparator output is simply sampled."]
    FILTER_CNT_1 = 1,
    #[doc = "2: 2 consecutive samples must agree."]
    FILTER_CNT_2 = 2,
    #[doc = "3: 3 consecutive samples must agree."]
    FILTER_CNT_3 = 3,
    #[doc = "4: 4 consecutive samples must agree."]
    FILTER_CNT_4 = 4,
    #[doc = "5: 5 consecutive samples must agree."]
    FILTER_CNT_5 = 5,
    #[doc = "6: 6 consecutive samples must agree."]
    FILTER_CNT_6 = 6,
    #[doc = "7: 7 consecutive samples must agree."]
    FILTER_CNT_7 = 7,
}
impl From<FILTER_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FILTER_CNT`"]
pub type FILTER_CNT_R = crate::R<u8, FILTER_CNT_A>;
impl FILTER_CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_CNT_A {
        match self.bits {
            0 => FILTER_CNT_A::FILTER_CNT_0,
            1 => FILTER_CNT_A::FILTER_CNT_1,
            2 => FILTER_CNT_A::FILTER_CNT_2,
            3 => FILTER_CNT_A::FILTER_CNT_3,
            4 => FILTER_CNT_A::FILTER_CNT_4,
            5 => FILTER_CNT_A::FILTER_CNT_5,
            6 => FILTER_CNT_A::FILTER_CNT_6,
            7 => FILTER_CNT_A::FILTER_CNT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_0`"]
    #[inline(always)]
    pub fn is_filter_cnt_0(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_0
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_1`"]
    #[inline(always)]
    pub fn is_filter_cnt_1(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_1
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_2`"]
    #[inline(always)]
    pub fn is_filter_cnt_2(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_2
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_3`"]
    #[inline(always)]
    pub fn is_filter_cnt_3(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_3
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_4`"]
    #[inline(always)]
    pub fn is_filter_cnt_4(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_4
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_5`"]
    #[inline(always)]
    pub fn is_filter_cnt_5(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_5
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_6`"]
    #[inline(always)]
    pub fn is_filter_cnt_6(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_6
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_7`"]
    #[inline(always)]
    pub fn is_filter_cnt_7(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_7
    }
}
#[doc = "Write proxy for field `FILTER_CNT`"]
pub struct FILTER_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_CNT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    #[inline(always)]
    pub fn filter_cnt_0(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_0)
    }
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    #[inline(always)]
    pub fn filter_cnt_1(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_1)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_2(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_2)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_3(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_3)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_4(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_4)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_5(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_5)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_6(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_6)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_7(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u8) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline(always)]
    pub fn hystctr(&self) -> HYSTCTR_R {
        HYSTCTR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&self) -> FILTER_CNT_R {
        FILTER_CNT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline(always)]
    pub fn hystctr(&mut self) -> HYSTCTR_W {
        HYSTCTR_W { w: self }
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&mut self) -> FILTER_CNT_W {
        FILTER_CNT_W { w: self }
    }
}
