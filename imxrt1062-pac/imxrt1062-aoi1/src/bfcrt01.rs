#[doc = "Reader of register BFCRT01%s"]
pub type R = crate::R<u16, super::BFCRT01>;
#[doc = "Writer for register BFCRT01%s"]
pub type W = crate::W<u16, super::BFCRT01>;
#[doc = "Register BFCRT01%s `reset()`'s with value 0"]
impl crate::ResetValue for super::BFCRT01 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Product term 1, D input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT1_DC_A {
    #[doc = "0: Force the D input in this product term to a logical zero"]
    PT1_DC_0 = 0,
    #[doc = "1: Pass the D input in this product term"]
    PT1_DC_1 = 1,
    #[doc = "2: Complement the D input in this product term"]
    PT1_DC_2 = 2,
    #[doc = "3: Force the D input in this product term to a logical one"]
    PT1_DC_3 = 3,
}
impl From<PT1_DC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT1_DC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT1_DC`"]
pub type PT1_DC_R = crate::R<u8, PT1_DC_A>;
impl PT1_DC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT1_DC_A {
        match self.bits {
            0 => PT1_DC_A::PT1_DC_0,
            1 => PT1_DC_A::PT1_DC_1,
            2 => PT1_DC_A::PT1_DC_2,
            3 => PT1_DC_A::PT1_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_DC_0`"]
    #[inline(always)]
    pub fn is_pt1_dc_0(&self) -> bool {
        *self == PT1_DC_A::PT1_DC_0
    }
    #[doc = "Checks if the value of the field is `PT1_DC_1`"]
    #[inline(always)]
    pub fn is_pt1_dc_1(&self) -> bool {
        *self == PT1_DC_A::PT1_DC_1
    }
    #[doc = "Checks if the value of the field is `PT1_DC_2`"]
    #[inline(always)]
    pub fn is_pt1_dc_2(&self) -> bool {
        *self == PT1_DC_A::PT1_DC_2
    }
    #[doc = "Checks if the value of the field is `PT1_DC_3`"]
    #[inline(always)]
    pub fn is_pt1_dc_3(&self) -> bool {
        *self == PT1_DC_A::PT1_DC_3
    }
}
#[doc = "Write proxy for field `PT1_DC`"]
pub struct PT1_DC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT1_DC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT1_DC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt1_dc_0(self) -> &'a mut W {
        self.variant(PT1_DC_A::PT1_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline(always)]
    pub fn pt1_dc_1(self) -> &'a mut W {
        self.variant(PT1_DC_A::PT1_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline(always)]
    pub fn pt1_dc_2(self) -> &'a mut W {
        self.variant(PT1_DC_A::PT1_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt1_dc_3(self) -> &'a mut W {
        self.variant(PT1_DC_A::PT1_DC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Product term 1, C input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT1_CC_A {
    #[doc = "0: Force the C input in this product term to a logical zero"]
    PT1_CC_0 = 0,
    #[doc = "1: Pass the C input in this product term"]
    PT1_CC_1 = 1,
    #[doc = "2: Complement the C input in this product term"]
    PT1_CC_2 = 2,
    #[doc = "3: Force the C input in this product term to a logical one"]
    PT1_CC_3 = 3,
}
impl From<PT1_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT1_CC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT1_CC`"]
pub type PT1_CC_R = crate::R<u8, PT1_CC_A>;
impl PT1_CC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT1_CC_A {
        match self.bits {
            0 => PT1_CC_A::PT1_CC_0,
            1 => PT1_CC_A::PT1_CC_1,
            2 => PT1_CC_A::PT1_CC_2,
            3 => PT1_CC_A::PT1_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_CC_0`"]
    #[inline(always)]
    pub fn is_pt1_cc_0(&self) -> bool {
        *self == PT1_CC_A::PT1_CC_0
    }
    #[doc = "Checks if the value of the field is `PT1_CC_1`"]
    #[inline(always)]
    pub fn is_pt1_cc_1(&self) -> bool {
        *self == PT1_CC_A::PT1_CC_1
    }
    #[doc = "Checks if the value of the field is `PT1_CC_2`"]
    #[inline(always)]
    pub fn is_pt1_cc_2(&self) -> bool {
        *self == PT1_CC_A::PT1_CC_2
    }
    #[doc = "Checks if the value of the field is `PT1_CC_3`"]
    #[inline(always)]
    pub fn is_pt1_cc_3(&self) -> bool {
        *self == PT1_CC_A::PT1_CC_3
    }
}
#[doc = "Write proxy for field `PT1_CC`"]
pub struct PT1_CC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT1_CC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT1_CC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt1_cc_0(self) -> &'a mut W {
        self.variant(PT1_CC_A::PT1_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline(always)]
    pub fn pt1_cc_1(self) -> &'a mut W {
        self.variant(PT1_CC_A::PT1_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline(always)]
    pub fn pt1_cc_2(self) -> &'a mut W {
        self.variant(PT1_CC_A::PT1_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt1_cc_3(self) -> &'a mut W {
        self.variant(PT1_CC_A::PT1_CC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Product term 1, B input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT1_BC_A {
    #[doc = "0: Force the B input in this product term to a logical zero"]
    PT1_BC_0 = 0,
    #[doc = "1: Pass the B input in this product term"]
    PT1_BC_1 = 1,
    #[doc = "2: Complement the B input in this product term"]
    PT1_BC_2 = 2,
    #[doc = "3: Force the B input in this product term to a logical one"]
    PT1_BC_3 = 3,
}
impl From<PT1_BC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT1_BC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT1_BC`"]
pub type PT1_BC_R = crate::R<u8, PT1_BC_A>;
impl PT1_BC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT1_BC_A {
        match self.bits {
            0 => PT1_BC_A::PT1_BC_0,
            1 => PT1_BC_A::PT1_BC_1,
            2 => PT1_BC_A::PT1_BC_2,
            3 => PT1_BC_A::PT1_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_BC_0`"]
    #[inline(always)]
    pub fn is_pt1_bc_0(&self) -> bool {
        *self == PT1_BC_A::PT1_BC_0
    }
    #[doc = "Checks if the value of the field is `PT1_BC_1`"]
    #[inline(always)]
    pub fn is_pt1_bc_1(&self) -> bool {
        *self == PT1_BC_A::PT1_BC_1
    }
    #[doc = "Checks if the value of the field is `PT1_BC_2`"]
    #[inline(always)]
    pub fn is_pt1_bc_2(&self) -> bool {
        *self == PT1_BC_A::PT1_BC_2
    }
    #[doc = "Checks if the value of the field is `PT1_BC_3`"]
    #[inline(always)]
    pub fn is_pt1_bc_3(&self) -> bool {
        *self == PT1_BC_A::PT1_BC_3
    }
}
#[doc = "Write proxy for field `PT1_BC`"]
pub struct PT1_BC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT1_BC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT1_BC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt1_bc_0(self) -> &'a mut W {
        self.variant(PT1_BC_A::PT1_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline(always)]
    pub fn pt1_bc_1(self) -> &'a mut W {
        self.variant(PT1_BC_A::PT1_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline(always)]
    pub fn pt1_bc_2(self) -> &'a mut W {
        self.variant(PT1_BC_A::PT1_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt1_bc_3(self) -> &'a mut W {
        self.variant(PT1_BC_A::PT1_BC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Product term 1, A input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT1_AC_A {
    #[doc = "0: Force the A input in this product term to a logical zero"]
    PT1_AC_0 = 0,
    #[doc = "1: Pass the A input in this product term"]
    PT1_AC_1 = 1,
    #[doc = "2: Complement the A input in this product term"]
    PT1_AC_2 = 2,
    #[doc = "3: Force the A input in this product term to a logical one"]
    PT1_AC_3 = 3,
}
impl From<PT1_AC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT1_AC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT1_AC`"]
pub type PT1_AC_R = crate::R<u8, PT1_AC_A>;
impl PT1_AC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT1_AC_A {
        match self.bits {
            0 => PT1_AC_A::PT1_AC_0,
            1 => PT1_AC_A::PT1_AC_1,
            2 => PT1_AC_A::PT1_AC_2,
            3 => PT1_AC_A::PT1_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_AC_0`"]
    #[inline(always)]
    pub fn is_pt1_ac_0(&self) -> bool {
        *self == PT1_AC_A::PT1_AC_0
    }
    #[doc = "Checks if the value of the field is `PT1_AC_1`"]
    #[inline(always)]
    pub fn is_pt1_ac_1(&self) -> bool {
        *self == PT1_AC_A::PT1_AC_1
    }
    #[doc = "Checks if the value of the field is `PT1_AC_2`"]
    #[inline(always)]
    pub fn is_pt1_ac_2(&self) -> bool {
        *self == PT1_AC_A::PT1_AC_2
    }
    #[doc = "Checks if the value of the field is `PT1_AC_3`"]
    #[inline(always)]
    pub fn is_pt1_ac_3(&self) -> bool {
        *self == PT1_AC_A::PT1_AC_3
    }
}
#[doc = "Write proxy for field `PT1_AC`"]
pub struct PT1_AC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT1_AC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT1_AC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt1_ac_0(self) -> &'a mut W {
        self.variant(PT1_AC_A::PT1_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline(always)]
    pub fn pt1_ac_1(self) -> &'a mut W {
        self.variant(PT1_AC_A::PT1_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline(always)]
    pub fn pt1_ac_2(self) -> &'a mut W {
        self.variant(PT1_AC_A::PT1_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt1_ac_3(self) -> &'a mut W {
        self.variant(PT1_AC_A::PT1_AC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Product term 0, D input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT0_DC_A {
    #[doc = "0: Force the D input in this product term to a logical zero"]
    PT0_DC_0 = 0,
    #[doc = "1: Pass the D input in this product term"]
    PT0_DC_1 = 1,
    #[doc = "2: Complement the D input in this product term"]
    PT0_DC_2 = 2,
    #[doc = "3: Force the D input in this product term to a logical one"]
    PT0_DC_3 = 3,
}
impl From<PT0_DC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT0_DC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT0_DC`"]
pub type PT0_DC_R = crate::R<u8, PT0_DC_A>;
impl PT0_DC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT0_DC_A {
        match self.bits {
            0 => PT0_DC_A::PT0_DC_0,
            1 => PT0_DC_A::PT0_DC_1,
            2 => PT0_DC_A::PT0_DC_2,
            3 => PT0_DC_A::PT0_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_DC_0`"]
    #[inline(always)]
    pub fn is_pt0_dc_0(&self) -> bool {
        *self == PT0_DC_A::PT0_DC_0
    }
    #[doc = "Checks if the value of the field is `PT0_DC_1`"]
    #[inline(always)]
    pub fn is_pt0_dc_1(&self) -> bool {
        *self == PT0_DC_A::PT0_DC_1
    }
    #[doc = "Checks if the value of the field is `PT0_DC_2`"]
    #[inline(always)]
    pub fn is_pt0_dc_2(&self) -> bool {
        *self == PT0_DC_A::PT0_DC_2
    }
    #[doc = "Checks if the value of the field is `PT0_DC_3`"]
    #[inline(always)]
    pub fn is_pt0_dc_3(&self) -> bool {
        *self == PT0_DC_A::PT0_DC_3
    }
}
#[doc = "Write proxy for field `PT0_DC`"]
pub struct PT0_DC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT0_DC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT0_DC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt0_dc_0(self) -> &'a mut W {
        self.variant(PT0_DC_A::PT0_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline(always)]
    pub fn pt0_dc_1(self) -> &'a mut W {
        self.variant(PT0_DC_A::PT0_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline(always)]
    pub fn pt0_dc_2(self) -> &'a mut W {
        self.variant(PT0_DC_A::PT0_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt0_dc_3(self) -> &'a mut W {
        self.variant(PT0_DC_A::PT0_DC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Product term 0, C input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT0_CC_A {
    #[doc = "0: Force the C input in this product term to a logical zero"]
    PT0_CC_0 = 0,
    #[doc = "1: Pass the C input in this product term"]
    PT0_CC_1 = 1,
    #[doc = "2: Complement the C input in this product term"]
    PT0_CC_2 = 2,
    #[doc = "3: Force the C input in this product term to a logical one"]
    PT0_CC_3 = 3,
}
impl From<PT0_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT0_CC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT0_CC`"]
pub type PT0_CC_R = crate::R<u8, PT0_CC_A>;
impl PT0_CC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT0_CC_A {
        match self.bits {
            0 => PT0_CC_A::PT0_CC_0,
            1 => PT0_CC_A::PT0_CC_1,
            2 => PT0_CC_A::PT0_CC_2,
            3 => PT0_CC_A::PT0_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_CC_0`"]
    #[inline(always)]
    pub fn is_pt0_cc_0(&self) -> bool {
        *self == PT0_CC_A::PT0_CC_0
    }
    #[doc = "Checks if the value of the field is `PT0_CC_1`"]
    #[inline(always)]
    pub fn is_pt0_cc_1(&self) -> bool {
        *self == PT0_CC_A::PT0_CC_1
    }
    #[doc = "Checks if the value of the field is `PT0_CC_2`"]
    #[inline(always)]
    pub fn is_pt0_cc_2(&self) -> bool {
        *self == PT0_CC_A::PT0_CC_2
    }
    #[doc = "Checks if the value of the field is `PT0_CC_3`"]
    #[inline(always)]
    pub fn is_pt0_cc_3(&self) -> bool {
        *self == PT0_CC_A::PT0_CC_3
    }
}
#[doc = "Write proxy for field `PT0_CC`"]
pub struct PT0_CC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT0_CC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT0_CC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt0_cc_0(self) -> &'a mut W {
        self.variant(PT0_CC_A::PT0_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline(always)]
    pub fn pt0_cc_1(self) -> &'a mut W {
        self.variant(PT0_CC_A::PT0_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline(always)]
    pub fn pt0_cc_2(self) -> &'a mut W {
        self.variant(PT0_CC_A::PT0_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt0_cc_3(self) -> &'a mut W {
        self.variant(PT0_CC_A::PT0_CC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Product term 0, B input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT0_BC_A {
    #[doc = "0: Force the B input in this product term to a logical zero"]
    PT0_BC_0 = 0,
    #[doc = "1: Pass the B input in this product term"]
    PT0_BC_1 = 1,
    #[doc = "2: Complement the B input in this product term"]
    PT0_BC_2 = 2,
    #[doc = "3: Force the B input in this product term to a logical one"]
    PT0_BC_3 = 3,
}
impl From<PT0_BC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT0_BC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT0_BC`"]
pub type PT0_BC_R = crate::R<u8, PT0_BC_A>;
impl PT0_BC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT0_BC_A {
        match self.bits {
            0 => PT0_BC_A::PT0_BC_0,
            1 => PT0_BC_A::PT0_BC_1,
            2 => PT0_BC_A::PT0_BC_2,
            3 => PT0_BC_A::PT0_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_BC_0`"]
    #[inline(always)]
    pub fn is_pt0_bc_0(&self) -> bool {
        *self == PT0_BC_A::PT0_BC_0
    }
    #[doc = "Checks if the value of the field is `PT0_BC_1`"]
    #[inline(always)]
    pub fn is_pt0_bc_1(&self) -> bool {
        *self == PT0_BC_A::PT0_BC_1
    }
    #[doc = "Checks if the value of the field is `PT0_BC_2`"]
    #[inline(always)]
    pub fn is_pt0_bc_2(&self) -> bool {
        *self == PT0_BC_A::PT0_BC_2
    }
    #[doc = "Checks if the value of the field is `PT0_BC_3`"]
    #[inline(always)]
    pub fn is_pt0_bc_3(&self) -> bool {
        *self == PT0_BC_A::PT0_BC_3
    }
}
#[doc = "Write proxy for field `PT0_BC`"]
pub struct PT0_BC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT0_BC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT0_BC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt0_bc_0(self) -> &'a mut W {
        self.variant(PT0_BC_A::PT0_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline(always)]
    pub fn pt0_bc_1(self) -> &'a mut W {
        self.variant(PT0_BC_A::PT0_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline(always)]
    pub fn pt0_bc_2(self) -> &'a mut W {
        self.variant(PT0_BC_A::PT0_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt0_bc_3(self) -> &'a mut W {
        self.variant(PT0_BC_A::PT0_BC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Product term 0, A input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT0_AC_A {
    #[doc = "0: Force the A input in this product term to a logical zero"]
    PT0_AC_0 = 0,
    #[doc = "1: Pass the A input in this product term"]
    PT0_AC_1 = 1,
    #[doc = "2: Complement the A input in this product term"]
    PT0_AC_2 = 2,
    #[doc = "3: Force the A input in this product term to a logical one"]
    PT0_AC_3 = 3,
}
impl From<PT0_AC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT0_AC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT0_AC`"]
pub type PT0_AC_R = crate::R<u8, PT0_AC_A>;
impl PT0_AC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT0_AC_A {
        match self.bits {
            0 => PT0_AC_A::PT0_AC_0,
            1 => PT0_AC_A::PT0_AC_1,
            2 => PT0_AC_A::PT0_AC_2,
            3 => PT0_AC_A::PT0_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_AC_0`"]
    #[inline(always)]
    pub fn is_pt0_ac_0(&self) -> bool {
        *self == PT0_AC_A::PT0_AC_0
    }
    #[doc = "Checks if the value of the field is `PT0_AC_1`"]
    #[inline(always)]
    pub fn is_pt0_ac_1(&self) -> bool {
        *self == PT0_AC_A::PT0_AC_1
    }
    #[doc = "Checks if the value of the field is `PT0_AC_2`"]
    #[inline(always)]
    pub fn is_pt0_ac_2(&self) -> bool {
        *self == PT0_AC_A::PT0_AC_2
    }
    #[doc = "Checks if the value of the field is `PT0_AC_3`"]
    #[inline(always)]
    pub fn is_pt0_ac_3(&self) -> bool {
        *self == PT0_AC_A::PT0_AC_3
    }
}
#[doc = "Write proxy for field `PT0_AC`"]
pub struct PT0_AC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT0_AC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT0_AC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt0_ac_0(self) -> &'a mut W {
        self.variant(PT0_AC_A::PT0_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline(always)]
    pub fn pt0_ac_1(self) -> &'a mut W {
        self.variant(PT0_AC_A::PT0_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline(always)]
    pub fn pt0_ac_2(self) -> &'a mut W {
        self.variant(PT0_AC_A::PT0_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt0_ac_3(self) -> &'a mut W {
        self.variant(PT0_AC_A::PT0_AC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Product term 1, D input configuration"]
    #[inline(always)]
    pub fn pt1_dc(&self) -> PT1_DC_R {
        PT1_DC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Product term 1, C input configuration"]
    #[inline(always)]
    pub fn pt1_cc(&self) -> PT1_CC_R {
        PT1_CC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Product term 1, B input configuration"]
    #[inline(always)]
    pub fn pt1_bc(&self) -> PT1_BC_R {
        PT1_BC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Product term 1, A input configuration"]
    #[inline(always)]
    pub fn pt1_ac(&self) -> PT1_AC_R {
        PT1_AC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Product term 0, D input configuration"]
    #[inline(always)]
    pub fn pt0_dc(&self) -> PT0_DC_R {
        PT0_DC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Product term 0, C input configuration"]
    #[inline(always)]
    pub fn pt0_cc(&self) -> PT0_CC_R {
        PT0_CC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Product term 0, B input configuration"]
    #[inline(always)]
    pub fn pt0_bc(&self) -> PT0_BC_R {
        PT0_BC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Product term 0, A input configuration"]
    #[inline(always)]
    pub fn pt0_ac(&self) -> PT0_AC_R {
        PT0_AC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Product term 1, D input configuration"]
    #[inline(always)]
    pub fn pt1_dc(&mut self) -> PT1_DC_W {
        PT1_DC_W { w: self }
    }
    #[doc = "Bits 2:3 - Product term 1, C input configuration"]
    #[inline(always)]
    pub fn pt1_cc(&mut self) -> PT1_CC_W {
        PT1_CC_W { w: self }
    }
    #[doc = "Bits 4:5 - Product term 1, B input configuration"]
    #[inline(always)]
    pub fn pt1_bc(&mut self) -> PT1_BC_W {
        PT1_BC_W { w: self }
    }
    #[doc = "Bits 6:7 - Product term 1, A input configuration"]
    #[inline(always)]
    pub fn pt1_ac(&mut self) -> PT1_AC_W {
        PT1_AC_W { w: self }
    }
    #[doc = "Bits 8:9 - Product term 0, D input configuration"]
    #[inline(always)]
    pub fn pt0_dc(&mut self) -> PT0_DC_W {
        PT0_DC_W { w: self }
    }
    #[doc = "Bits 10:11 - Product term 0, C input configuration"]
    #[inline(always)]
    pub fn pt0_cc(&mut self) -> PT0_CC_W {
        PT0_CC_W { w: self }
    }
    #[doc = "Bits 12:13 - Product term 0, B input configuration"]
    #[inline(always)]
    pub fn pt0_bc(&mut self) -> PT0_BC_W {
        PT0_BC_W { w: self }
    }
    #[doc = "Bits 14:15 - Product term 0, A input configuration"]
    #[inline(always)]
    pub fn pt0_ac(&mut self) -> PT0_AC_W {
        PT0_AC_W { w: self }
    }
}
