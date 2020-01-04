#[doc = "Reader of register IMR"]
pub type R = crate::R<u16, super::IMR>;
#[doc = "Reader of field `HOME`"]
pub type HOME_R = crate::R<bool, bool>;
#[doc = "Reader of field `INDEX`"]
pub type INDEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `PHB`"]
pub type PHB_R = crate::R<bool, bool>;
#[doc = "Reader of field `PHA`"]
pub type PHA_R = crate::R<bool, bool>;
#[doc = "Reader of field `FHOM`"]
pub type FHOM_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIND`"]
pub type FIND_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPHB`"]
pub type FPHB_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPHA`"]
pub type FPHA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - This is the raw HOME input."]
    #[inline(always)]
    pub fn home(&self) -> HOME_R {
        HOME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This is the raw INDEX input."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the raw PHASEB input."]
    #[inline(always)]
    pub fn phb(&self) -> PHB_R {
        PHB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This is the raw PHASEA input."]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This is the filtered version of HOME input."]
    #[inline(always)]
    pub fn fhom(&self) -> FHOM_R {
        FHOM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This is the filtered version of INDEX input."]
    #[inline(always)]
    pub fn find(&self) -> FIND_R {
        FIND_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This is the filtered version of PHASEB input."]
    #[inline(always)]
    pub fn fphb(&self) -> FPHB_R {
        FPHB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This is the filtered version of PHASEA input."]
    #[inline(always)]
    pub fn fpha(&self) -> FPHA_R {
        FPHA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
