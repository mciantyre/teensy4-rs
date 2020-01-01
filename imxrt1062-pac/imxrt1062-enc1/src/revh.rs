#[doc = "Reader of register REVH"]
pub type R = crate::R<u16, super::REVH>;
#[doc = "Reader of field `REVH`"]
pub type REVH_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This read-only register contains a snapshot of the value of the REV register."]
    #[inline(always)]
    pub fn revh(&self) -> REVH_R {
        REVH_R::new((self.bits & 0xffff) as u16)
    }
}
