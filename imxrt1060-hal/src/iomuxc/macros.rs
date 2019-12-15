macro_rules! alt0 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt0 setting
        pub fn alt0(self) -> $Pad<$crate::iomuxc::Alt0> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT0)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt1 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt1 setting
        pub fn alt1(self) -> $Pad<$crate::iomuxc::Alt1> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT1)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt2 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt2 setting
        pub fn alt2(self) -> $Pad<$crate::iomuxc::Alt2> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT2)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt3 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt3 setting
        pub fn alt3(self) -> $Pad<$crate::iomuxc::Alt3> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT3)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt4 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt4 setting
        pub fn alt4(self) -> $Pad<$crate::iomuxc::Alt4> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT4)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt5 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt5 setting
        pub fn alt5(self) -> $Pad<$crate::iomuxc::Alt5> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT5)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt6 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt6 setting
        pub fn alt6(self) -> $Pad<$crate::iomuxc::Alt6> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT6)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt7 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt7 setting
        pub fn alt7(self) -> $Pad<$crate::iomuxc::Alt7> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT7)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt8 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt8 setting
        pub fn alt8(self) -> $Pad<$crate::iomuxc::Alt8> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT8)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! alt9 {
    ($Pad:ident, $mux_mod:ident) => {
        /// Converts the pad into its Alt9 setting
        pub fn alt9(self) -> $Pad<$crate::iomuxc::Alt9> {
            unsafe {
                (*self.iomuxc).$mux_mod.write(|w| {
                    w.mux_mode()
                        .variant($crate::pac::iomuxc::$mux_mod::MUX_MODE_A::ALT9)
                })
            };
            $Pad {
                _alt: core::marker::PhantomData,
                iomuxc: self.iomuxc,
            }
        }
    };
}

macro_rules! pad {
    ($Pad:ident, $mux_mod:ident, [$($alt_macro:ident),+]) => {
        pub struct $Pad<Alt> {
            _alt: core::marker::PhantomData<Alt>,
            iomuxc: *const $crate::pac::iomuxc::RegisterBlock,
        }
        impl<Alt> $Pad<Alt> {
            $(
                $alt_macro!($Pad, $mux_mod);
            )+

            #[allow(dead_code)] // TODO Remove once all pads are exposed in IOMUXC, and these are properly used
            pub(crate) fn new(iomuxc: &$crate::pac::iomuxc::RegisterBlock) -> Self {
                Self {
                    _alt: core::marker::PhantomData,
                    iomuxc,
                }
            }
        }
    };
}
