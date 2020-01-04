#[doc = "Reader of register CM7_AHBSCR"]
pub type R = crate::R<u32, super::CM7_AHBSCR>;
#[doc = "Writer for register CM7_AHBSCR"]
pub type W = crate::W<u32, super::CM7_AHBSCR>;
#[doc = "Register CM7_AHBSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CM7_AHBSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AHBS prioritization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTL_A {
    #[doc = "0: AHBS access priority demoted. This is the reset value."]
    CTL_0 = 0,
    #[doc = "1: Software access priority demoted."]
    CTL_1 = 1,
    #[doc = "2: AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR\\[INITCOUNT\\]
value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR\\[TPRI\\]."]
    CTL_2 = 2,
    #[doc = "3: AHBSPRI signal has control of access priority."]
    CTL_3 = 3,
}
impl From<CTL_A> for u8 {
    #[inline(always)]
    fn from(variant: CTL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTL`"]
pub type CTL_R = crate::R<u8, CTL_A>;
impl CTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_A {
        match self.bits {
            0 => CTL_A::CTL_0,
            1 => CTL_A::CTL_1,
            2 => CTL_A::CTL_2,
            3 => CTL_A::CTL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTL_0`"]
    #[inline(always)]
    pub fn is_ctl_0(&self) -> bool {
        *self == CTL_A::CTL_0
    }
    #[doc = "Checks if the value of the field is `CTL_1`"]
    #[inline(always)]
    pub fn is_ctl_1(&self) -> bool {
        *self == CTL_A::CTL_1
    }
    #[doc = "Checks if the value of the field is `CTL_2`"]
    #[inline(always)]
    pub fn is_ctl_2(&self) -> bool {
        *self == CTL_A::CTL_2
    }
    #[doc = "Checks if the value of the field is `CTL_3`"]
    #[inline(always)]
    pub fn is_ctl_3(&self) -> bool {
        *self == CTL_A::CTL_3
    }
}
#[doc = "Write proxy for field `CTL`"]
pub struct CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "AHBS access priority demoted. This is the reset value."]
    #[inline(always)]
    pub fn ctl_0(self) -> &'a mut W {
        self.variant(CTL_A::CTL_0)
    }
    #[doc = "Software access priority demoted."]
    #[inline(always)]
    pub fn ctl_1(self) -> &'a mut W {
        self.variant(CTL_A::CTL_1)
    }
    #[doc = "AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR\\[INITCOUNT\\]
value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR\\[TPRI\\]."]
    #[inline(always)]
    pub fn ctl_2(self) -> &'a mut W {
        self.variant(CTL_A::CTL_2)
    }
    #[doc = "AHBSPRI signal has control of access priority."]
    #[inline(always)]
    pub fn ctl_3(self) -> &'a mut W {
        self.variant(CTL_A::CTL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TPRI`"]
pub type TPRI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TPRI`"]
pub struct TPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> TPRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | (((value as u32) & 0x01ff) << 2);
        self.w
    }
}
#[doc = "Reader of field `INITCOUNT`"]
pub type INITCOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INITCOUNT`"]
pub struct INITCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> INITCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AHBS prioritization control."]
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:10 - Threshold execution priority for AHBS traffic demotion."]
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:15 - Fairness counter initialization value."]
    #[inline(always)]
    pub fn initcount(&self) -> INITCOUNT_R {
        INITCOUNT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHBS prioritization control."]
    #[inline(always)]
    pub fn ctl(&mut self) -> CTL_W {
        CTL_W { w: self }
    }
    #[doc = "Bits 2:10 - Threshold execution priority for AHBS traffic demotion."]
    #[inline(always)]
    pub fn tpri(&mut self) -> TPRI_W {
        TPRI_W { w: self }
    }
    #[doc = "Bits 11:15 - Fairness counter initialization value."]
    #[inline(always)]
    pub fn initcount(&mut self) -> INITCOUNT_W {
        INITCOUNT_W { w: self }
    }
}
