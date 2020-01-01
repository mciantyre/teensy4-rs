#[doc = "Reader of register VID1"]
pub type R = crate::R<u32, super::VID1>;
#[doc = "Shows the IP's Minor revision of the TRNG.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MIN_REV_A {
    #[doc = "0: Minor revision number for TRNG."]
    MIN_REV_0 = 0,
}
impl From<MIN_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: MIN_REV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MIN_REV`"]
pub type MIN_REV_R = crate::R<u8, MIN_REV_A>;
impl MIN_REV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MIN_REV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MIN_REV_A::MIN_REV_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MIN_REV_0`"]
    #[inline(always)]
    pub fn is_min_rev_0(&self) -> bool {
        *self == MIN_REV_A::MIN_REV_0
    }
}
#[doc = "Shows the IP's Major revision of the TRNG.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAJ_REV_A {
    #[doc = "1: Major revision number for TRNG."]
    MAJ_REV_1 = 1,
}
impl From<MAJ_REV_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJ_REV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAJ_REV`"]
pub type MAJ_REV_R = crate::R<u8, MAJ_REV_A>;
impl MAJ_REV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAJ_REV_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(MAJ_REV_A::MAJ_REV_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAJ_REV_1`"]
    #[inline(always)]
    pub fn is_maj_rev_1(&self) -> bool {
        *self == MAJ_REV_A::MAJ_REV_1
    }
}
#[doc = "Shows the IP ID.\n\nValue on reset: 48"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum IP_ID_A {
    #[doc = "48: ID for TRNG."]
    IP_ID_48 = 48,
}
impl From<IP_ID_A> for u16 {
    #[inline(always)]
    fn from(variant: IP_ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IP_ID`"]
pub type IP_ID_R = crate::R<u16, IP_ID_A>;
impl IP_ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, IP_ID_A> {
        use crate::Variant::*;
        match self.bits {
            48 => Val(IP_ID_A::IP_ID_48),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IP_ID_48`"]
    #[inline(always)]
    pub fn is_ip_id_48(&self) -> bool {
        *self == IP_ID_A::IP_ID_48
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the IP's Minor revision of the TRNG."]
    #[inline(always)]
    pub fn min_rev(&self) -> MIN_REV_R {
        MIN_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the IP's Major revision of the TRNG."]
    #[inline(always)]
    pub fn maj_rev(&self) -> MAJ_REV_R {
        MAJ_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Shows the IP ID."]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
