#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::board;
use bsp::hal::{gpio, iomuxc};

// Hardware struct for HD44780 character LCD
pub struct LcdHw {
    // Represent GPIO offsets; the 'n' in GPIO2[n].
    d4_offset: u32,
    d5_offset: u32,
    d6_offset: u32,
    d7_offset: u32,
    gpio2: gpio::Port<2>,

    // For demonstration, I'll pretend that
    // the RS and EN pins require a different
    // GPIO port.
    en_offset: u32,
    rs_offset: u32,
    gpio3: gpio::Port<3>,
}

impl LcdHw {
    /// This constructor statically guarantees that all of the pads you're providing
    /// can be controlled by the GPIO2 or GPIO3 port.
    pub fn new<D4, D5, D6, D7, RS, EN>(
        gpio2: gpio::Port<2>,
        gpio3: gpio::Port<3>,
        _d4: D4,
        _d5: D5,
        _d6: D6,
        _d7: D7,
        _rs: RS,
        _en: EN,
    ) -> Self
    where
        // GPIO2 pins.
        D4: iomuxc::gpio::Pin<2>,
        D5: iomuxc::gpio::Pin<2>,
        D6: iomuxc::gpio::Pin<2>,
        D7: iomuxc::gpio::Pin<2>,
        // GPIO3 pins
        RS: iomuxc::gpio::Pin<3>,
        EN: iomuxc::gpio::Pin<3>,
    {
        // This ends up dropping the pads. But from the caller's
        // perspective, we own the pads. This doesn't necessarily
        // work if we need to give the user those pads back
        // sometime later...
        LcdHw {
            d4_offset: D4::OFFSET,
            d5_offset: D5::OFFSET,
            d6_offset: D6::OFFSET,
            d7_offset: D7::OFFSET,
            gpio2,

            rs_offset: RS::OFFSET,
            en_offset: EN::OFFSET,
            gpio3,
        }
    }
}

impl lcd::Hardware for LcdHw {
    fn rw(&mut self, bit: bool) {
        // imxrt-hal provides separate APIs for configuring GPIO inputs and outputs
        // when no pad object is available. Simply constructing the proper Output /
        // input object is enough to set the pin state.
        if bit {
            gpio::Input::<()>::without_pin(&mut self.gpio2, self.d4_offset);
            gpio::Input::<()>::without_pin(&mut self.gpio2, self.d5_offset);
            gpio::Input::<()>::without_pin(&mut self.gpio2, self.d6_offset);
            gpio::Input::<()>::without_pin(&mut self.gpio2, self.d7_offset);
        } else {
            gpio::Output::<()>::without_pin(&mut self.gpio2, self.d4_offset);
            gpio::Output::<()>::without_pin(&mut self.gpio2, self.d5_offset);
            gpio::Output::<()>::without_pin(&mut self.gpio2, self.d6_offset);
            gpio::Output::<()>::without_pin(&mut self.gpio2, self.d7_offset);
        }
    }
    fn rs(&mut self, bit: bool) {
        let rs = gpio::Output::<()>::without_pin(&mut self.gpio3, self.rs_offset);
        if bit {
            rs.set();
        } else {
            rs.clear();
        }
    }
    fn enable(&mut self, bit: bool) {
        let en = gpio::Output::<()>::without_pin(&mut self.gpio3, self.en_offset);
        if bit {
            en.set();
        } else {
            en.clear();
        }
    }
    fn data(&mut self, data: u8) {
        todo!("Set / clear dn pins based on data bits {}", data);
    }
    fn mode(&self) -> lcd::FunctionMode {
        lcd::FunctionMode::Bit4
    }
}

/////////////////////////////////////////////////////////////////////////

/// The same as the above, but it tracks the six pins in
/// the type system. Still assumes that pins are either GPIO2
/// and GPIO3 compatible.
///
/// Using `pub` instead of adding a constructor.
pub struct PinKnowingLdcHw<D4, D5, D6, D7, RS, EN>
where
    // GPIO2 pins.
    D4: iomuxc::gpio::Pin<2>,
    D5: iomuxc::gpio::Pin<2>,
    D6: iomuxc::gpio::Pin<2>,
    D7: iomuxc::gpio::Pin<2>,
    // GPIO3 pins
    RS: iomuxc::gpio::Pin<3>,
    EN: iomuxc::gpio::Pin<3>,
{
    pub d4: D4,
    pub d5: D5,
    pub d6: D6,
    pub d7: D7,
    pub gpio2: gpio::Port<2>,

    pub en: EN,
    pub rs: RS,
    pub gpio3: gpio::Port<3>,
}

impl<D4, D5, D6, D7, RS, EN> lcd::Hardware for PinKnowingLdcHw<D4, D5, D6, D7, RS, EN>
where
    // GPIO2 pins.
    D4: iomuxc::gpio::Pin<2>,
    D5: iomuxc::gpio::Pin<2>,
    D6: iomuxc::gpio::Pin<2>,
    D7: iomuxc::gpio::Pin<2>,
    // GPIO3 pins
    RS: iomuxc::gpio::Pin<3>,
    EN: iomuxc::gpio::Pin<3>,
{
    fn rw(&mut self, bit: bool) {
        // The GPIO port output / input calls take ownership of the provided
        // pad objects. But, we only have a &mut self, so we can't give the
        // port ownership of (our own) pads. So we still need to pretend that
        // we don't have the pads...
        //
        // Other approaches include an enum that states if each data pin is
        // currently an Output or an Input, and we switch variants here.
        if bit {
            gpio::Input::<()>::without_pin(&mut self.gpio2, D4::OFFSET);
            gpio::Input::<()>::without_pin(&mut self.gpio2, D5::OFFSET);
            gpio::Input::<()>::without_pin(&mut self.gpio2, D6::OFFSET);
            gpio::Input::<()>::without_pin(&mut self.gpio2, D7::OFFSET);
        } else {
            gpio::Output::<()>::without_pin(&mut self.gpio2, D4::OFFSET);
            gpio::Output::<()>::without_pin(&mut self.gpio2, D5::OFFSET);
            gpio::Output::<()>::without_pin(&mut self.gpio2, D6::OFFSET);
            gpio::Output::<()>::without_pin(&mut self.gpio2, D7::OFFSET);
        }
    }
    fn rs(&mut self, _bit: bool) {
        todo!();
    }
    fn enable(&mut self, _bit: bool) {
        todo!();
    }
    fn data(&mut self, data: u8) {
        todo!("Set / clear dn pins based on data bits {}", data);
    }
    fn mode(&self) -> lcd::FunctionMode {
        lcd::FunctionMode::Bit4
    }
}

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pins,
        mut gpio2,
        gpio3,
        ..
    } = board::t40(board::instances());
    let led = board::led(&mut gpio2, pins.p13);

    // NOTE: I'm picking arbitrary pins that I know are either GPIO2 and GPIO3 compatible.
    // And all of this is for a type check test.
    let pin_knowing = PinKnowingLdcHw {
        d4: pins.p6,
        d5: pins.p7,
        d6: pins.p8,
        d7: pins.p9,
        gpio2,
        rs: pins.p28,
        en: pins.p30,
        gpio3,
    };

    let lcd_hw = LcdHw::new(
        pin_knowing.gpio2,
        pin_knowing.gpio3,
        pin_knowing.d4,
        pin_knowing.d5,
        pin_knowing.d6,
        pin_knowing.d7,
        pin_knowing.rs,
        pin_knowing.en,
    );
    loop {
        led.set();
    }
}
