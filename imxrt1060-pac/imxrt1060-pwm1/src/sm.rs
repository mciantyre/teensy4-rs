#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcnt](smcnt) module"]
pub type SMCNT = crate::Reg<u16, _SMCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCNT;
#[doc = "`read()` method returns [smcnt::R](smcnt::R) reader structure"]
impl crate::Readable for SMCNT {}
#[doc = "Counter Register"]
pub mod smcnt;
#[doc = "Initial Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sminit](sminit) module"]
pub type SMINIT = crate::Reg<u16, _SMINIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMINIT;
#[doc = "`read()` method returns [sminit::R](sminit::R) reader structure"]
impl crate::Readable for SMINIT {}
#[doc = "`write(|w| ..)` method takes [sminit::W](sminit::W) writer structure"]
impl crate::Writable for SMINIT {}
#[doc = "Initial Count Register"]
pub mod sminit;
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smctrl2](smctrl2) module"]
pub type SMCTRL2 = crate::Reg<u16, _SMCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCTRL2;
#[doc = "`read()` method returns [smctrl2::R](smctrl2::R) reader structure"]
impl crate::Readable for SMCTRL2 {}
#[doc = "`write(|w| ..)` method takes [smctrl2::W](smctrl2::W) writer structure"]
impl crate::Writable for SMCTRL2 {}
#[doc = "Control 2 Register"]
pub mod smctrl2;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smctrl](smctrl) module"]
pub type SMCTRL = crate::Reg<u16, _SMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCTRL;
#[doc = "`read()` method returns [smctrl::R](smctrl::R) reader structure"]
impl crate::Readable for SMCTRL {}
#[doc = "`write(|w| ..)` method takes [smctrl::W](smctrl::W) writer structure"]
impl crate::Writable for SMCTRL {}
#[doc = "Control Register"]
pub mod smctrl;
#[doc = "Value Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval0](smval0) module"]
pub type SMVAL0 = crate::Reg<u16, _SMVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMVAL0;
#[doc = "`read()` method returns [smval0::R](smval0::R) reader structure"]
impl crate::Readable for SMVAL0 {}
#[doc = "`write(|w| ..)` method takes [smval0::W](smval0::W) writer structure"]
impl crate::Writable for SMVAL0 {}
#[doc = "Value Register 0"]
pub mod smval0;
#[doc = "Fractional Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfracval1](smfracval1) module"]
pub type SMFRACVAL1 = crate::Reg<u16, _SMFRACVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMFRACVAL1;
#[doc = "`read()` method returns [smfracval1::R](smfracval1::R) reader structure"]
impl crate::Readable for SMFRACVAL1 {}
#[doc = "`write(|w| ..)` method takes [smfracval1::W](smfracval1::W) writer structure"]
impl crate::Writable for SMFRACVAL1 {}
#[doc = "Fractional Value Register 1"]
pub mod smfracval1;
#[doc = "Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval1](smval1) module"]
pub type SMVAL1 = crate::Reg<u16, _SMVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMVAL1;
#[doc = "`read()` method returns [smval1::R](smval1::R) reader structure"]
impl crate::Readable for SMVAL1 {}
#[doc = "`write(|w| ..)` method takes [smval1::W](smval1::W) writer structure"]
impl crate::Writable for SMVAL1 {}
#[doc = "Value Register 1"]
pub mod smval1;
#[doc = "Fractional Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfracval2](smfracval2) module"]
pub type SMFRACVAL2 = crate::Reg<u16, _SMFRACVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMFRACVAL2;
#[doc = "`read()` method returns [smfracval2::R](smfracval2::R) reader structure"]
impl crate::Readable for SMFRACVAL2 {}
#[doc = "`write(|w| ..)` method takes [smfracval2::W](smfracval2::W) writer structure"]
impl crate::Writable for SMFRACVAL2 {}
#[doc = "Fractional Value Register 2"]
pub mod smfracval2;
#[doc = "Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval2](smval2) module"]
pub type SMVAL2 = crate::Reg<u16, _SMVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMVAL2;
#[doc = "`read()` method returns [smval2::R](smval2::R) reader structure"]
impl crate::Readable for SMVAL2 {}
#[doc = "`write(|w| ..)` method takes [smval2::W](smval2::W) writer structure"]
impl crate::Writable for SMVAL2 {}
#[doc = "Value Register 2"]
pub mod smval2;
#[doc = "Fractional Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfracval3](smfracval3) module"]
pub type SMFRACVAL3 = crate::Reg<u16, _SMFRACVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMFRACVAL3;
#[doc = "`read()` method returns [smfracval3::R](smfracval3::R) reader structure"]
impl crate::Readable for SMFRACVAL3 {}
#[doc = "`write(|w| ..)` method takes [smfracval3::W](smfracval3::W) writer structure"]
impl crate::Writable for SMFRACVAL3 {}
#[doc = "Fractional Value Register 3"]
pub mod smfracval3;
#[doc = "Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval3](smval3) module"]
pub type SMVAL3 = crate::Reg<u16, _SMVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMVAL3;
#[doc = "`read()` method returns [smval3::R](smval3::R) reader structure"]
impl crate::Readable for SMVAL3 {}
#[doc = "`write(|w| ..)` method takes [smval3::W](smval3::W) writer structure"]
impl crate::Writable for SMVAL3 {}
#[doc = "Value Register 3"]
pub mod smval3;
#[doc = "Fractional Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfracval4](smfracval4) module"]
pub type SMFRACVAL4 = crate::Reg<u16, _SMFRACVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMFRACVAL4;
#[doc = "`read()` method returns [smfracval4::R](smfracval4::R) reader structure"]
impl crate::Readable for SMFRACVAL4 {}
#[doc = "`write(|w| ..)` method takes [smfracval4::W](smfracval4::W) writer structure"]
impl crate::Writable for SMFRACVAL4 {}
#[doc = "Fractional Value Register 4"]
pub mod smfracval4;
#[doc = "Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval4](smval4) module"]
pub type SMVAL4 = crate::Reg<u16, _SMVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMVAL4;
#[doc = "`read()` method returns [smval4::R](smval4::R) reader structure"]
impl crate::Readable for SMVAL4 {}
#[doc = "`write(|w| ..)` method takes [smval4::W](smval4::W) writer structure"]
impl crate::Writable for SMVAL4 {}
#[doc = "Value Register 4"]
pub mod smval4;
#[doc = "Fractional Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfracval5](smfracval5) module"]
pub type SMFRACVAL5 = crate::Reg<u16, _SMFRACVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMFRACVAL5;
#[doc = "`read()` method returns [smfracval5::R](smfracval5::R) reader structure"]
impl crate::Readable for SMFRACVAL5 {}
#[doc = "`write(|w| ..)` method takes [smfracval5::W](smfracval5::W) writer structure"]
impl crate::Writable for SMFRACVAL5 {}
#[doc = "Fractional Value Register 5"]
pub mod smfracval5;
#[doc = "Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval5](smval5) module"]
pub type SMVAL5 = crate::Reg<u16, _SMVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMVAL5;
#[doc = "`read()` method returns [smval5::R](smval5::R) reader structure"]
impl crate::Readable for SMVAL5 {}
#[doc = "`write(|w| ..)` method takes [smval5::W](smval5::W) writer structure"]
impl crate::Writable for SMVAL5 {}
#[doc = "Value Register 5"]
pub mod smval5;
#[doc = "Fractional Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfrctrl](smfrctrl) module"]
pub type SMFRCTRL = crate::Reg<u16, _SMFRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMFRCTRL;
#[doc = "`read()` method returns [smfrctrl::R](smfrctrl::R) reader structure"]
impl crate::Readable for SMFRCTRL {}
#[doc = "`write(|w| ..)` method takes [smfrctrl::W](smfrctrl::W) writer structure"]
impl crate::Writable for SMFRCTRL {}
#[doc = "Fractional Control Register"]
pub mod smfrctrl;
#[doc = "Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smoctrl](smoctrl) module"]
pub type SMOCTRL = crate::Reg<u16, _SMOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMOCTRL;
#[doc = "`read()` method returns [smoctrl::R](smoctrl::R) reader structure"]
impl crate::Readable for SMOCTRL {}
#[doc = "`write(|w| ..)` method takes [smoctrl::W](smoctrl::W) writer structure"]
impl crate::Writable for SMOCTRL {}
#[doc = "Output Control Register"]
pub mod smoctrl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smsts](smsts) module"]
pub type SMSTS = crate::Reg<u16, _SMSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMSTS;
#[doc = "`read()` method returns [smsts::R](smsts::R) reader structure"]
impl crate::Readable for SMSTS {}
#[doc = "`write(|w| ..)` method takes [smsts::W](smsts::W) writer structure"]
impl crate::Writable for SMSTS {}
#[doc = "Status Register"]
pub mod smsts;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sminten](sminten) module"]
pub type SMINTEN = crate::Reg<u16, _SMINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMINTEN;
#[doc = "`read()` method returns [sminten::R](sminten::R) reader structure"]
impl crate::Readable for SMINTEN {}
#[doc = "`write(|w| ..)` method takes [sminten::W](sminten::W) writer structure"]
impl crate::Writable for SMINTEN {}
#[doc = "Interrupt Enable Register"]
pub mod sminten;
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smdmaen](smdmaen) module"]
pub type SMDMAEN = crate::Reg<u16, _SMDMAEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMDMAEN;
#[doc = "`read()` method returns [smdmaen::R](smdmaen::R) reader structure"]
impl crate::Readable for SMDMAEN {}
#[doc = "`write(|w| ..)` method takes [smdmaen::W](smdmaen::W) writer structure"]
impl crate::Writable for SMDMAEN {}
#[doc = "DMA Enable Register"]
pub mod smdmaen;
#[doc = "Output Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smtctrl](smtctrl) module"]
pub type SMTCTRL = crate::Reg<u16, _SMTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMTCTRL;
#[doc = "`read()` method returns [smtctrl::R](smtctrl::R) reader structure"]
impl crate::Readable for SMTCTRL {}
#[doc = "`write(|w| ..)` method takes [smtctrl::W](smtctrl::W) writer structure"]
impl crate::Writable for SMTCTRL {}
#[doc = "Output Trigger Control Register"]
pub mod smtctrl;
#[doc = "Fault Disable Mapping Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smdismap0](smdismap0) module"]
pub type SMDISMAP0 = crate::Reg<u16, _SMDISMAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMDISMAP0;
#[doc = "`read()` method returns [smdismap0::R](smdismap0::R) reader structure"]
impl crate::Readable for SMDISMAP0 {}
#[doc = "`write(|w| ..)` method takes [smdismap0::W](smdismap0::W) writer structure"]
impl crate::Writable for SMDISMAP0 {}
#[doc = "Fault Disable Mapping Register 0"]
pub mod smdismap0;
#[doc = "Fault Disable Mapping Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smdismap1](smdismap1) module"]
pub type SMDISMAP1 = crate::Reg<u16, _SMDISMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMDISMAP1;
#[doc = "`read()` method returns [smdismap1::R](smdismap1::R) reader structure"]
impl crate::Readable for SMDISMAP1 {}
#[doc = "`write(|w| ..)` method takes [smdismap1::W](smdismap1::W) writer structure"]
impl crate::Writable for SMDISMAP1 {}
#[doc = "Fault Disable Mapping Register 1"]
pub mod smdismap1;
#[doc = "Deadtime Count Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smdtcnt0](smdtcnt0) module"]
pub type SMDTCNT0 = crate::Reg<u16, _SMDTCNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMDTCNT0;
#[doc = "`read()` method returns [smdtcnt0::R](smdtcnt0::R) reader structure"]
impl crate::Readable for SMDTCNT0 {}
#[doc = "`write(|w| ..)` method takes [smdtcnt0::W](smdtcnt0::W) writer structure"]
impl crate::Writable for SMDTCNT0 {}
#[doc = "Deadtime Count Register 0"]
pub mod smdtcnt0;
#[doc = "Deadtime Count Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smdtcnt1](smdtcnt1) module"]
pub type SMDTCNT1 = crate::Reg<u16, _SMDTCNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMDTCNT1;
#[doc = "`read()` method returns [smdtcnt1::R](smdtcnt1::R) reader structure"]
impl crate::Readable for SMDTCNT1 {}
#[doc = "`write(|w| ..)` method takes [smdtcnt1::W](smdtcnt1::W) writer structure"]
impl crate::Writable for SMDTCNT1 {}
#[doc = "Deadtime Count Register 1"]
pub mod smdtcnt1;
#[doc = "Capture Control A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptctrla](smcaptctrla) module"]
pub type SMCAPTCTRLA = crate::Reg<u16, _SMCAPTCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCAPTCTRLA;
#[doc = "`read()` method returns [smcaptctrla::R](smcaptctrla::R) reader structure"]
impl crate::Readable for SMCAPTCTRLA {}
#[doc = "`write(|w| ..)` method takes [smcaptctrla::W](smcaptctrla::W) writer structure"]
impl crate::Writable for SMCAPTCTRLA {}
#[doc = "Capture Control A Register"]
pub mod smcaptctrla;
#[doc = "Capture Compare A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptcompa](smcaptcompa) module"]
pub type SMCAPTCOMPA = crate::Reg<u16, _SMCAPTCOMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCAPTCOMPA;
#[doc = "`read()` method returns [smcaptcompa::R](smcaptcompa::R) reader structure"]
impl crate::Readable for SMCAPTCOMPA {}
#[doc = "`write(|w| ..)` method takes [smcaptcompa::W](smcaptcompa::W) writer structure"]
impl crate::Writable for SMCAPTCOMPA {}
#[doc = "Capture Compare A Register"]
pub mod smcaptcompa;
#[doc = "Capture Control B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptctrlb](smcaptctrlb) module"]
pub type SMCAPTCTRLB = crate::Reg<u16, _SMCAPTCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCAPTCTRLB;
#[doc = "`read()` method returns [smcaptctrlb::R](smcaptctrlb::R) reader structure"]
impl crate::Readable for SMCAPTCTRLB {}
#[doc = "`write(|w| ..)` method takes [smcaptctrlb::W](smcaptctrlb::W) writer structure"]
impl crate::Writable for SMCAPTCTRLB {}
#[doc = "Capture Control B Register"]
pub mod smcaptctrlb;
#[doc = "Capture Compare B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptcompb](smcaptcompb) module"]
pub type SMCAPTCOMPB = crate::Reg<u16, _SMCAPTCOMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCAPTCOMPB;
#[doc = "`read()` method returns [smcaptcompb::R](smcaptcompb::R) reader structure"]
impl crate::Readable for SMCAPTCOMPB {}
#[doc = "`write(|w| ..)` method takes [smcaptcompb::W](smcaptcompb::W) writer structure"]
impl crate::Writable for SMCAPTCOMPB {}
#[doc = "Capture Compare B Register"]
pub mod smcaptcompb;
#[doc = "Capture Control X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptctrlx](smcaptctrlx) module"]
pub type SMCAPTCTRLX = crate::Reg<u16, _SMCAPTCTRLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCAPTCTRLX;
#[doc = "`read()` method returns [smcaptctrlx::R](smcaptctrlx::R) reader structure"]
impl crate::Readable for SMCAPTCTRLX {}
#[doc = "`write(|w| ..)` method takes [smcaptctrlx::W](smcaptctrlx::W) writer structure"]
impl crate::Writable for SMCAPTCTRLX {}
#[doc = "Capture Control X Register"]
pub mod smcaptctrlx;
#[doc = "Capture Compare X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptcompx](smcaptcompx) module"]
pub type SMCAPTCOMPX = crate::Reg<u16, _SMCAPTCOMPX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCAPTCOMPX;
#[doc = "`read()` method returns [smcaptcompx::R](smcaptcompx::R) reader structure"]
impl crate::Readable for SMCAPTCOMPX {}
#[doc = "`write(|w| ..)` method takes [smcaptcompx::W](smcaptcompx::W) writer structure"]
impl crate::Writable for SMCAPTCOMPX {}
#[doc = "Capture Compare X Register"]
pub mod smcaptcompx;
#[doc = "Capture Value 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval0](smcval0) module"]
pub type SMCVAL0 = crate::Reg<u16, _SMCVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL0;
#[doc = "`read()` method returns [smcval0::R](smcval0::R) reader structure"]
impl crate::Readable for SMCVAL0 {}
#[doc = "Capture Value 0 Register"]
pub mod smcval0;
#[doc = "Capture Value 0 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval0cyc](smcval0cyc) module"]
pub type SMCVAL0CYC = crate::Reg<u16, _SMCVAL0CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL0CYC;
#[doc = "`read()` method returns [smcval0cyc::R](smcval0cyc::R) reader structure"]
impl crate::Readable for SMCVAL0CYC {}
#[doc = "Capture Value 0 Cycle Register"]
pub mod smcval0cyc;
#[doc = "Capture Value 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval1](smcval1) module"]
pub type SMCVAL1 = crate::Reg<u16, _SMCVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL1;
#[doc = "`read()` method returns [smcval1::R](smcval1::R) reader structure"]
impl crate::Readable for SMCVAL1 {}
#[doc = "Capture Value 1 Register"]
pub mod smcval1;
#[doc = "Capture Value 1 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval1cyc](smcval1cyc) module"]
pub type SMCVAL1CYC = crate::Reg<u16, _SMCVAL1CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL1CYC;
#[doc = "`read()` method returns [smcval1cyc::R](smcval1cyc::R) reader structure"]
impl crate::Readable for SMCVAL1CYC {}
#[doc = "Capture Value 1 Cycle Register"]
pub mod smcval1cyc;
#[doc = "Capture Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval2](smcval2) module"]
pub type SMCVAL2 = crate::Reg<u16, _SMCVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL2;
#[doc = "`read()` method returns [smcval2::R](smcval2::R) reader structure"]
impl crate::Readable for SMCVAL2 {}
#[doc = "Capture Value 2 Register"]
pub mod smcval2;
#[doc = "Capture Value 2 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval2cyc](smcval2cyc) module"]
pub type SMCVAL2CYC = crate::Reg<u16, _SMCVAL2CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL2CYC;
#[doc = "`read()` method returns [smcval2cyc::R](smcval2cyc::R) reader structure"]
impl crate::Readable for SMCVAL2CYC {}
#[doc = "Capture Value 2 Cycle Register"]
pub mod smcval2cyc;
#[doc = "Capture Value 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval3](smcval3) module"]
pub type SMCVAL3 = crate::Reg<u16, _SMCVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL3;
#[doc = "`read()` method returns [smcval3::R](smcval3::R) reader structure"]
impl crate::Readable for SMCVAL3 {}
#[doc = "Capture Value 3 Register"]
pub mod smcval3;
#[doc = "Capture Value 3 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval3cyc](smcval3cyc) module"]
pub type SMCVAL3CYC = crate::Reg<u16, _SMCVAL3CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL3CYC;
#[doc = "`read()` method returns [smcval3cyc::R](smcval3cyc::R) reader structure"]
impl crate::Readable for SMCVAL3CYC {}
#[doc = "Capture Value 3 Cycle Register"]
pub mod smcval3cyc;
#[doc = "Capture Value 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval4](smcval4) module"]
pub type SMCVAL4 = crate::Reg<u16, _SMCVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL4;
#[doc = "`read()` method returns [smcval4::R](smcval4::R) reader structure"]
impl crate::Readable for SMCVAL4 {}
#[doc = "Capture Value 4 Register"]
pub mod smcval4;
#[doc = "Capture Value 4 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval4cyc](smcval4cyc) module"]
pub type SMCVAL4CYC = crate::Reg<u16, _SMCVAL4CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL4CYC;
#[doc = "`read()` method returns [smcval4cyc::R](smcval4cyc::R) reader structure"]
impl crate::Readable for SMCVAL4CYC {}
#[doc = "Capture Value 4 Cycle Register"]
pub mod smcval4cyc;
#[doc = "Capture Value 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval5](smcval5) module"]
pub type SMCVAL5 = crate::Reg<u16, _SMCVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL5;
#[doc = "`read()` method returns [smcval5::R](smcval5::R) reader structure"]
impl crate::Readable for SMCVAL5 {}
#[doc = "Capture Value 5 Register"]
pub mod smcval5;
#[doc = "Capture Value 5 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval5cyc](smcval5cyc) module"]
pub type SMCVAL5CYC = crate::Reg<u16, _SMCVAL5CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCVAL5CYC;
#[doc = "`read()` method returns [smcval5cyc::R](smcval5cyc::R) reader structure"]
impl crate::Readable for SMCVAL5CYC {}
#[doc = "Capture Value 5 Cycle Register"]
pub mod smcval5cyc;
