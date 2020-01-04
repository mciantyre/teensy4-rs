#[doc = "Reader of register BFCRT23%s"]
pub type R = crate::R<u16, super::BFCRT23>;
#[doc = "Writer for register BFCRT23%s"]
pub type W = crate::W<u16, super::BFCRT23>;
#[doc = "Register BFCRT23%s `reset()`'s with value 0"]
impl crate::ResetValue for super::BFCRT23 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Product term 3, D input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT3_DC_A {
    #[doc = "0: Force the D input in this product term to a logical zero"]
    PT3_DC_0 = 0,
    #[doc = "1: Pass the D input in this product term"]
    PT3_DC_1 = 1,
    #[doc = "2: Complement the D input in this product term"]
    PT3_DC_2 = 2,
    #[doc = "3: Force the D input in this product term to a logical one"]
    PT3_DC_3 = 3,
}
impl From<PT3_DC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT3_DC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT3_DC`"]
pub type PT3_DC_R = crate::R<u8, PT3_DC_A>;
impl PT3_DC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT3_DC_A {
        match self.bits {
            0 => PT3_DC_A::PT3_DC_0,
            1 => PT3_DC_A::PT3_DC_1,
            2 => PT3_DC_A::PT3_DC_2,
            3 => PT3_DC_A::PT3_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_DC_0`"]
    #[inline(always)]
    pub fn is_pt3_dc_0(&self) -> bool {
        *self == PT3_DC_A::PT3_DC_0
    }
    #[doc = "Checks if the value of the field is `PT3_DC_1`"]
    #[inline(always)]
    pub fn is_pt3_dc_1(&self) -> bool {
        *self == PT3_DC_A::PT3_DC_1
    }
    #[doc = "Checks if the value of the field is `PT3_DC_2`"]
    #[inline(always)]
    pub fn is_pt3_dc_2(&self) -> bool {
        *self == PT3_DC_A::PT3_DC_2
    }
    #[doc = "Checks if the value of the field is `PT3_DC_3`"]
    #[inline(always)]
    pub fn is_pt3_dc_3(&self) -> bool {
        *self == PT3_DC_A::PT3_DC_3
    }
}
#[doc = "Write proxy for field `PT3_DC`"]
pub struct PT3_DC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT3_DC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT3_DC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt3_dc_0(self) -> &'a mut W {
        self.variant(PT3_DC_A::PT3_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline(always)]
    pub fn pt3_dc_1(self) -> &'a mut W {
        self.variant(PT3_DC_A::PT3_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline(always)]
    pub fn pt3_dc_2(self) -> &'a mut W {
        self.variant(PT3_DC_A::PT3_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt3_dc_3(self) -> &'a mut W {
        self.variant(PT3_DC_A::PT3_DC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Product term 3, C input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT3_CC_A {
    #[doc = "0: Force the C input in this product term to a logical zero"]
    PT3_CC_0 = 0,
    #[doc = "1: Pass the C input in this product term"]
    PT3_CC_1 = 1,
    #[doc = "2: Complement the C input in this product term"]
    PT3_CC_2 = 2,
    #[doc = "3: Force the C input in this product term to a logical one"]
    PT3_CC_3 = 3,
}
impl From<PT3_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT3_CC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT3_CC`"]
pub type PT3_CC_R = crate::R<u8, PT3_CC_A>;
impl PT3_CC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT3_CC_A {
        match self.bits {
            0 => PT3_CC_A::PT3_CC_0,
            1 => PT3_CC_A::PT3_CC_1,
            2 => PT3_CC_A::PT3_CC_2,
            3 => PT3_CC_A::PT3_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_CC_0`"]
    #[inline(always)]
    pub fn is_pt3_cc_0(&self) -> bool {
        *self == PT3_CC_A::PT3_CC_0
    }
    #[doc = "Checks if the value of the field is `PT3_CC_1`"]
    #[inline(always)]
    pub fn is_pt3_cc_1(&self) -> bool {
        *self == PT3_CC_A::PT3_CC_1
    }
    #[doc = "Checks if the value of the field is `PT3_CC_2`"]
    #[inline(always)]
    pub fn is_pt3_cc_2(&self) -> bool {
        *self == PT3_CC_A::PT3_CC_2
    }
    #[doc = "Checks if the value of the field is `PT3_CC_3`"]
    #[inline(always)]
    pub fn is_pt3_cc_3(&self) -> bool {
        *self == PT3_CC_A::PT3_CC_3
    }
}
#[doc = "Write proxy for field `PT3_CC`"]
pub struct PT3_CC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT3_CC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT3_CC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt3_cc_0(self) -> &'a mut W {
        self.variant(PT3_CC_A::PT3_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline(always)]
    pub fn pt3_cc_1(self) -> &'a mut W {
        self.variant(PT3_CC_A::PT3_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline(always)]
    pub fn pt3_cc_2(self) -> &'a mut W {
        self.variant(PT3_CC_A::PT3_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt3_cc_3(self) -> &'a mut W {
        self.variant(PT3_CC_A::PT3_CC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Product term 3, B input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT3_BC_A {
    #[doc = "0: Force the B input in this product term to a logical zero"]
    PT3_BC_0 = 0,
    #[doc = "1: Pass the B input in this product term"]
    PT3_BC_1 = 1,
    #[doc = "2: Complement the B input in this product term"]
    PT3_BC_2 = 2,
    #[doc = "3: Force the B input in this product term to a logical one"]
    PT3_BC_3 = 3,
}
impl From<PT3_BC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT3_BC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT3_BC`"]
pub type PT3_BC_R = crate::R<u8, PT3_BC_A>;
impl PT3_BC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT3_BC_A {
        match self.bits {
            0 => PT3_BC_A::PT3_BC_0,
            1 => PT3_BC_A::PT3_BC_1,
            2 => PT3_BC_A::PT3_BC_2,
            3 => PT3_BC_A::PT3_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_BC_0`"]
    #[inline(always)]
    pub fn is_pt3_bc_0(&self) -> bool {
        *self == PT3_BC_A::PT3_BC_0
    }
    #[doc = "Checks if the value of the field is `PT3_BC_1`"]
    #[inline(always)]
    pub fn is_pt3_bc_1(&self) -> bool {
        *self == PT3_BC_A::PT3_BC_1
    }
    #[doc = "Checks if the value of the field is `PT3_BC_2`"]
    #[inline(always)]
    pub fn is_pt3_bc_2(&self) -> bool {
        *self == PT3_BC_A::PT3_BC_2
    }
    #[doc = "Checks if the value of the field is `PT3_BC_3`"]
    #[inline(always)]
    pub fn is_pt3_bc_3(&self) -> bool {
        *self == PT3_BC_A::PT3_BC_3
    }
}
#[doc = "Write proxy for field `PT3_BC`"]
pub struct PT3_BC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT3_BC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT3_BC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt3_bc_0(self) -> &'a mut W {
        self.variant(PT3_BC_A::PT3_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline(always)]
    pub fn pt3_bc_1(self) -> &'a mut W {
        self.variant(PT3_BC_A::PT3_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline(always)]
    pub fn pt3_bc_2(self) -> &'a mut W {
        self.variant(PT3_BC_A::PT3_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt3_bc_3(self) -> &'a mut W {
        self.variant(PT3_BC_A::PT3_BC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Product term 3, A input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT3_AC_A {
    #[doc = "0: Force the A input in this product term to a logical zero"]
    PT3_AC_0 = 0,
    #[doc = "1: Pass the A input in this product term"]
    PT3_AC_1 = 1,
    #[doc = "2: Complement the A input in this product term"]
    PT3_AC_2 = 2,
    #[doc = "3: Force the A input in this product term to a logical one"]
    PT3_AC_3 = 3,
}
impl From<PT3_AC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT3_AC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT3_AC`"]
pub type PT3_AC_R = crate::R<u8, PT3_AC_A>;
impl PT3_AC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT3_AC_A {
        match self.bits {
            0 => PT3_AC_A::PT3_AC_0,
            1 => PT3_AC_A::PT3_AC_1,
            2 => PT3_AC_A::PT3_AC_2,
            3 => PT3_AC_A::PT3_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_AC_0`"]
    #[inline(always)]
    pub fn is_pt3_ac_0(&self) -> bool {
        *self == PT3_AC_A::PT3_AC_0
    }
    #[doc = "Checks if the value of the field is `PT3_AC_1`"]
    #[inline(always)]
    pub fn is_pt3_ac_1(&self) -> bool {
        *self == PT3_AC_A::PT3_AC_1
    }
    #[doc = "Checks if the value of the field is `PT3_AC_2`"]
    #[inline(always)]
    pub fn is_pt3_ac_2(&self) -> bool {
        *self == PT3_AC_A::PT3_AC_2
    }
    #[doc = "Checks if the value of the field is `PT3_AC_3`"]
    #[inline(always)]
    pub fn is_pt3_ac_3(&self) -> bool {
        *self == PT3_AC_A::PT3_AC_3
    }
}
#[doc = "Write proxy for field `PT3_AC`"]
pub struct PT3_AC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT3_AC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT3_AC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt3_ac_0(self) -> &'a mut W {
        self.variant(PT3_AC_A::PT3_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline(always)]
    pub fn pt3_ac_1(self) -> &'a mut W {
        self.variant(PT3_AC_A::PT3_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline(always)]
    pub fn pt3_ac_2(self) -> &'a mut W {
        self.variant(PT3_AC_A::PT3_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt3_ac_3(self) -> &'a mut W {
        self.variant(PT3_AC_A::PT3_AC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Product term 2, D input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT2_DC_A {
    #[doc = "0: Force the D input in this product term to a logical zero"]
    PT2_DC_0 = 0,
    #[doc = "1: Pass the D input in this product term"]
    PT2_DC_1 = 1,
    #[doc = "2: Complement the D input in this product term"]
    PT2_DC_2 = 2,
    #[doc = "3: Force the D input in this product term to a logical one"]
    PT2_DC_3 = 3,
}
impl From<PT2_DC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT2_DC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT2_DC`"]
pub type PT2_DC_R = crate::R<u8, PT2_DC_A>;
impl PT2_DC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT2_DC_A {
        match self.bits {
            0 => PT2_DC_A::PT2_DC_0,
            1 => PT2_DC_A::PT2_DC_1,
            2 => PT2_DC_A::PT2_DC_2,
            3 => PT2_DC_A::PT2_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_DC_0`"]
    #[inline(always)]
    pub fn is_pt2_dc_0(&self) -> bool {
        *self == PT2_DC_A::PT2_DC_0
    }
    #[doc = "Checks if the value of the field is `PT2_DC_1`"]
    #[inline(always)]
    pub fn is_pt2_dc_1(&self) -> bool {
        *self == PT2_DC_A::PT2_DC_1
    }
    #[doc = "Checks if the value of the field is `PT2_DC_2`"]
    #[inline(always)]
    pub fn is_pt2_dc_2(&self) -> bool {
        *self == PT2_DC_A::PT2_DC_2
    }
    #[doc = "Checks if the value of the field is `PT2_DC_3`"]
    #[inline(always)]
    pub fn is_pt2_dc_3(&self) -> bool {
        *self == PT2_DC_A::PT2_DC_3
    }
}
#[doc = "Write proxy for field `PT2_DC`"]
pub struct PT2_DC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT2_DC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT2_DC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt2_dc_0(self) -> &'a mut W {
        self.variant(PT2_DC_A::PT2_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline(always)]
    pub fn pt2_dc_1(self) -> &'a mut W {
        self.variant(PT2_DC_A::PT2_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline(always)]
    pub fn pt2_dc_2(self) -> &'a mut W {
        self.variant(PT2_DC_A::PT2_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt2_dc_3(self) -> &'a mut W {
        self.variant(PT2_DC_A::PT2_DC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Product term 2, C input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT2_CC_A {
    #[doc = "0: Force the C input in this product term to a logical zero"]
    PT2_CC_0 = 0,
    #[doc = "1: Pass the C input in this product term"]
    PT2_CC_1 = 1,
    #[doc = "2: Complement the C input in this product term"]
    PT2_CC_2 = 2,
    #[doc = "3: Force the C input in this product term to a logical one"]
    PT2_CC_3 = 3,
}
impl From<PT2_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT2_CC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT2_CC`"]
pub type PT2_CC_R = crate::R<u8, PT2_CC_A>;
impl PT2_CC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT2_CC_A {
        match self.bits {
            0 => PT2_CC_A::PT2_CC_0,
            1 => PT2_CC_A::PT2_CC_1,
            2 => PT2_CC_A::PT2_CC_2,
            3 => PT2_CC_A::PT2_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_CC_0`"]
    #[inline(always)]
    pub fn is_pt2_cc_0(&self) -> bool {
        *self == PT2_CC_A::PT2_CC_0
    }
    #[doc = "Checks if the value of the field is `PT2_CC_1`"]
    #[inline(always)]
    pub fn is_pt2_cc_1(&self) -> bool {
        *self == PT2_CC_A::PT2_CC_1
    }
    #[doc = "Checks if the value of the field is `PT2_CC_2`"]
    #[inline(always)]
    pub fn is_pt2_cc_2(&self) -> bool {
        *self == PT2_CC_A::PT2_CC_2
    }
    #[doc = "Checks if the value of the field is `PT2_CC_3`"]
    #[inline(always)]
    pub fn is_pt2_cc_3(&self) -> bool {
        *self == PT2_CC_A::PT2_CC_3
    }
}
#[doc = "Write proxy for field `PT2_CC`"]
pub struct PT2_CC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT2_CC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT2_CC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt2_cc_0(self) -> &'a mut W {
        self.variant(PT2_CC_A::PT2_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline(always)]
    pub fn pt2_cc_1(self) -> &'a mut W {
        self.variant(PT2_CC_A::PT2_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline(always)]
    pub fn pt2_cc_2(self) -> &'a mut W {
        self.variant(PT2_CC_A::PT2_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt2_cc_3(self) -> &'a mut W {
        self.variant(PT2_CC_A::PT2_CC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Product term 2, B input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT2_BC_A {
    #[doc = "0: Force the B input in this product term to a logical zero"]
    PT2_BC_0 = 0,
    #[doc = "1: Pass the B input in this product term"]
    PT2_BC_1 = 1,
    #[doc = "2: Complement the B input in this product term"]
    PT2_BC_2 = 2,
    #[doc = "3: Force the B input in this product term to a logical one"]
    PT2_BC_3 = 3,
}
impl From<PT2_BC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT2_BC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT2_BC`"]
pub type PT2_BC_R = crate::R<u8, PT2_BC_A>;
impl PT2_BC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT2_BC_A {
        match self.bits {
            0 => PT2_BC_A::PT2_BC_0,
            1 => PT2_BC_A::PT2_BC_1,
            2 => PT2_BC_A::PT2_BC_2,
            3 => PT2_BC_A::PT2_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_BC_0`"]
    #[inline(always)]
    pub fn is_pt2_bc_0(&self) -> bool {
        *self == PT2_BC_A::PT2_BC_0
    }
    #[doc = "Checks if the value of the field is `PT2_BC_1`"]
    #[inline(always)]
    pub fn is_pt2_bc_1(&self) -> bool {
        *self == PT2_BC_A::PT2_BC_1
    }
    #[doc = "Checks if the value of the field is `PT2_BC_2`"]
    #[inline(always)]
    pub fn is_pt2_bc_2(&self) -> bool {
        *self == PT2_BC_A::PT2_BC_2
    }
    #[doc = "Checks if the value of the field is `PT2_BC_3`"]
    #[inline(always)]
    pub fn is_pt2_bc_3(&self) -> bool {
        *self == PT2_BC_A::PT2_BC_3
    }
}
#[doc = "Write proxy for field `PT2_BC`"]
pub struct PT2_BC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT2_BC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT2_BC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt2_bc_0(self) -> &'a mut W {
        self.variant(PT2_BC_A::PT2_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline(always)]
    pub fn pt2_bc_1(self) -> &'a mut W {
        self.variant(PT2_BC_A::PT2_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline(always)]
    pub fn pt2_bc_2(self) -> &'a mut W {
        self.variant(PT2_BC_A::PT2_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt2_bc_3(self) -> &'a mut W {
        self.variant(PT2_BC_A::PT2_BC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Product term 2, A input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PT2_AC_A {
    #[doc = "0: Force the A input in this product term to a logical zero"]
    PT2_AC_0 = 0,
    #[doc = "1: Pass the A input in this product term"]
    PT2_AC_1 = 1,
    #[doc = "2: Complement the A input in this product term"]
    PT2_AC_2 = 2,
    #[doc = "3: Force the A input in this product term to a logical one"]
    PT2_AC_3 = 3,
}
impl From<PT2_AC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT2_AC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PT2_AC`"]
pub type PT2_AC_R = crate::R<u8, PT2_AC_A>;
impl PT2_AC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT2_AC_A {
        match self.bits {
            0 => PT2_AC_A::PT2_AC_0,
            1 => PT2_AC_A::PT2_AC_1,
            2 => PT2_AC_A::PT2_AC_2,
            3 => PT2_AC_A::PT2_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_AC_0`"]
    #[inline(always)]
    pub fn is_pt2_ac_0(&self) -> bool {
        *self == PT2_AC_A::PT2_AC_0
    }
    #[doc = "Checks if the value of the field is `PT2_AC_1`"]
    #[inline(always)]
    pub fn is_pt2_ac_1(&self) -> bool {
        *self == PT2_AC_A::PT2_AC_1
    }
    #[doc = "Checks if the value of the field is `PT2_AC_2`"]
    #[inline(always)]
    pub fn is_pt2_ac_2(&self) -> bool {
        *self == PT2_AC_A::PT2_AC_2
    }
    #[doc = "Checks if the value of the field is `PT2_AC_3`"]
    #[inline(always)]
    pub fn is_pt2_ac_3(&self) -> bool {
        *self == PT2_AC_A::PT2_AC_3
    }
}
#[doc = "Write proxy for field `PT2_AC`"]
pub struct PT2_AC_W<'a> {
    w: &'a mut W,
}
impl<'a> PT2_AC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT2_AC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt2_ac_0(self) -> &'a mut W {
        self.variant(PT2_AC_A::PT2_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline(always)]
    pub fn pt2_ac_1(self) -> &'a mut W {
        self.variant(PT2_AC_A::PT2_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline(always)]
    pub fn pt2_ac_2(self) -> &'a mut W {
        self.variant(PT2_AC_A::PT2_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt2_ac_3(self) -> &'a mut W {
        self.variant(PT2_AC_A::PT2_AC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Product term 3, D input configuration"]
    #[inline(always)]
    pub fn pt3_dc(&self) -> PT3_DC_R {
        PT3_DC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Product term 3, C input configuration"]
    #[inline(always)]
    pub fn pt3_cc(&self) -> PT3_CC_R {
        PT3_CC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Product term 3, B input configuration"]
    #[inline(always)]
    pub fn pt3_bc(&self) -> PT3_BC_R {
        PT3_BC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Product term 3, A input configuration"]
    #[inline(always)]
    pub fn pt3_ac(&self) -> PT3_AC_R {
        PT3_AC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Product term 2, D input configuration"]
    #[inline(always)]
    pub fn pt2_dc(&self) -> PT2_DC_R {
        PT2_DC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Product term 2, C input configuration"]
    #[inline(always)]
    pub fn pt2_cc(&self) -> PT2_CC_R {
        PT2_CC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Product term 2, B input configuration"]
    #[inline(always)]
    pub fn pt2_bc(&self) -> PT2_BC_R {
        PT2_BC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Product term 2, A input configuration"]
    #[inline(always)]
    pub fn pt2_ac(&self) -> PT2_AC_R {
        PT2_AC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Product term 3, D input configuration"]
    #[inline(always)]
    pub fn pt3_dc(&mut self) -> PT3_DC_W {
        PT3_DC_W { w: self }
    }
    #[doc = "Bits 2:3 - Product term 3, C input configuration"]
    #[inline(always)]
    pub fn pt3_cc(&mut self) -> PT3_CC_W {
        PT3_CC_W { w: self }
    }
    #[doc = "Bits 4:5 - Product term 3, B input configuration"]
    #[inline(always)]
    pub fn pt3_bc(&mut self) -> PT3_BC_W {
        PT3_BC_W { w: self }
    }
    #[doc = "Bits 6:7 - Product term 3, A input configuration"]
    #[inline(always)]
    pub fn pt3_ac(&mut self) -> PT3_AC_W {
        PT3_AC_W { w: self }
    }
    #[doc = "Bits 8:9 - Product term 2, D input configuration"]
    #[inline(always)]
    pub fn pt2_dc(&mut self) -> PT2_DC_W {
        PT2_DC_W { w: self }
    }
    #[doc = "Bits 10:11 - Product term 2, C input configuration"]
    #[inline(always)]
    pub fn pt2_cc(&mut self) -> PT2_CC_W {
        PT2_CC_W { w: self }
    }
    #[doc = "Bits 12:13 - Product term 2, B input configuration"]
    #[inline(always)]
    pub fn pt2_bc(&mut self) -> PT2_BC_W {
        PT2_BC_W { w: self }
    }
    #[doc = "Bits 14:15 - Product term 2, A input configuration"]
    #[inline(always)]
    pub fn pt2_ac(&mut self) -> PT2_AC_W {
        PT2_AC_W { w: self }
    }
}
