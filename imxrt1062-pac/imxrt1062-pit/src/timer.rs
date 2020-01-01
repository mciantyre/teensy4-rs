#[doc = "Timer Load Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldval](ldval) module"]
pub type LDVAL = crate::Reg<u32, _LDVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDVAL;
#[doc = "`read()` method returns [ldval::R](ldval::R) reader structure"]
impl crate::Readable for LDVAL {}
#[doc = "`write(|w| ..)` method takes [ldval::W](ldval::W) writer structure"]
impl crate::Writable for LDVAL {}
#[doc = "Timer Load Value Register"]
pub mod ldval;
#[doc = "Current Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cval](cval) module"]
pub type CVAL = crate::Reg<u32, _CVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVAL;
#[doc = "`read()` method returns [cval::R](cval::R) reader structure"]
impl crate::Readable for CVAL {}
#[doc = "Current Timer Value Register"]
pub mod cval;
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl](tctrl) module"]
pub type TCTRL = crate::Reg<u32, _TCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTRL;
#[doc = "`read()` method returns [tctrl::R](tctrl::R) reader structure"]
impl crate::Readable for TCTRL {}
#[doc = "`write(|w| ..)` method takes [tctrl::W](tctrl::W) writer structure"]
impl crate::Writable for TCTRL {}
#[doc = "Timer Control Register"]
pub mod tctrl;
#[doc = "Timer Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tflg](tflg) module"]
pub type TFLG = crate::Reg<u32, _TFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFLG;
#[doc = "`read()` method returns [tflg::R](tflg::R) reader structure"]
impl crate::Readable for TFLG {}
#[doc = "`write(|w| ..)` method takes [tflg::W](tflg::W) writer structure"]
impl crate::Writable for TFLG {}
#[doc = "Timer Flag Register"]
pub mod tflg;
