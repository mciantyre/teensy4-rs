//! Demonstrates how to use a SPI master.
//! Similar to the I2C example, we try to
//! read the WHO_AM_I register from an MPU9250.
//!
//! Pinout:
//!
//! - Teensy 4 Pin 13 (SCK) to MPU's SCL (Note that we lose the LED here)
//! - Teensy 4 Pin 11 (MOSI) to MPU's SDA/SDI
//! - Teensy 4 Pin 12 (MISO) to MPU's AD0/SDO
//! - Teensy 4 Pin 10 (PSC0) to MPU's NCS
//!
//! By default, the example utilizes the SPI's internal chip select pin, rather
//! than relying on an arbitrary GPIO to control the chip select. However, you
//! may consider using an arbitrary GPIO for chip select. The example supports
//! that as well.
//!
//! Success criteria: the SPI clock rate is 1MHz. We can read both the MPU9250's
//! `WHO_AM_I` register (returns 0x71), and also the AK8963's `WHO_AM_I` register
//! (returns 0x48).

#![no_std]
#![no_main]

mod systick;
mod usb_io;

use teensy4_panic as _;

use cortex_m_rt::entry;
use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin};
use teensy4_bsp as bsp;

const SPI_BAUD_RATE_HZ: u32 = 1_000_000;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);
    usb_io::init().unwrap();
    let pins = bsp::pins::t40::from_pads(peripherals.iomuxc);

    peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    systick.delay_ms(5000);
    log::info!("Initializing SPI4 clocks...");

    let (_, _, _, spi4_builder) = peripherals.spi.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::spi::ClockSelect::Pll2,
        bsp::hal::ccm::spi::PrescalarSelect::LPSPI_PODF_5,
    );

    log::info!("Constructing SPI4 peripheral...");
    let mut spi4 = spi4_builder.build(pins.p11, pins.p12, pins.p13);

    match spi4.set_clock_speed(bsp::hal::spi::ClockSpeed(SPI_BAUD_RATE_HZ)) {
        Ok(()) => {
            log::info!("Set clock speed to {}Hz", SPI_BAUD_RATE_HZ);
        }
        Err(err) => {
            panic!(
                "Unable to set clock speed to {}Hz: {:?}",
                SPI_BAUD_RATE_HZ, err
            );
        }
    };

    // We're using the SPI's default chip select pin. This uses a
    // dummy `OutputPin` that does nothing! If you'd rather use any
    // GPIO, replace this line to construct a GPIO from another pin.
    spi4.enable_chip_select_0(pins.p10);
    struct DummyChipSelect;
    impl embedded_hal::digital::v2::OutputPin for DummyChipSelect {
        type Error = core::convert::Infallible;
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    let mut cs4 = DummyChipSelect;
    log::info!("Waiting 5 seconds before querying MPU9250...");
    systick.delay_ms(4000);

    match ak8963_init(&mut systick, &mut spi4, &mut cs4) {
        Ok(()) => (),
        Err(err) => {
            log::warn!("Unable to initialize AK8963: {:?}", err);
            loop {
                core::hint::spin_loop()
            }
        }
    };
    loop {
        systick.delay_ms(1000);
        match who_am_i(&mut spi4, &mut cs4) {
            Ok(who) => log::info!("Received {:#X} for WHO_AM_I", who),
            Err(err) => log::warn!("Error when querying WHO_AM_I: {:?}", err),
        }
        match ak8963_who_am_i(&mut spi4, &mut cs4) {
            Ok(who) => log::info!("Received {:#X} for AK8963_WHO_AM_I", who),
            Err(err) => log::warn!("Error when querying AK8963_WHO_AM_I: {:?}", err),
        }
    }
}

/// Query the MPU9250 for its WHO_AM_I value. It should be `0x71`.
fn who_am_i<SPI: Transfer<u16>, CS: OutputPin>(
    spi: &mut SPI,
    cs: &mut CS,
) -> Result<u8, SPI::Error> {
    const WHO_AM_I: u8 = 0x75;
    let mut buffer: [u16; 1] = [read(WHO_AM_I)];
    transact(cs, || spi.transfer(&mut buffer))?;
    Ok((buffer[0] & 0xFF) as u8)
}

/// Perform a SPI transaction while the chip select is active
fn transact<'a, O: OutputPin, F: FnOnce() -> R + 'a, R>(cs: &'a mut O, act: F) -> R {
    let _ = cs.set_low();
    let res = act();
    let _ = cs.set_high();
    res
}

/// Creates a read instruction for the MPU9250
const fn read(address: u8) -> u16 {
    ((address as u16) | (1 << 7)) << 8
}

/// Creates a write instruction for the MPU9250
const fn write(address: u8, value: u8) -> u16 {
    ((address as u16) << 8) | (value as u16)
}

/// Initialize the MPU9250's on-board magnetometer (the AK8963)
fn ak8963_init<SPI: Transfer<u16>, CS: OutputPin>(
    systick: &mut systick::SysTick, // TODO could be embedded_hal's DelayMs trait
    spi: &mut SPI,
    cs: &mut CS,
) -> Result<(), SPI::Error> {
    const USER_CTRL: u8 = 0x6A;
    const I2C_MST_CTRL: u8 = 0x24;
    const USER_CTRL_I2C_MST_RST: u8 = 1 << 1;
    const USER_CTRL_I2C_IF_DIS: u8 = 1 << 4;
    const USER_CTRL_I2C_MST_EN: u8 = 1 << 5;
    systick.delay_ms(100);
    transact(cs, || {
        spi.transfer(&mut [write(
            USER_CTRL,
            USER_CTRL_I2C_IF_DIS | USER_CTRL_I2C_MST_EN | USER_CTRL_I2C_MST_RST,
        )])
        .map(|_| ())
    })?;
    systick.delay_ms(100);
    transact(cs, || {
        spi.transfer(&mut [write(I2C_MST_CTRL, 0x0D)]) // 400KHz
            .map(|_| ())
    })?;
    systick.delay_ms(100);
    Ok(())
}

/// Query the on-board magnetometer's (the AK8963's) WHO_AM_I
/// register. Should return `0x48`.
fn ak8963_who_am_i<SPI: Transfer<u16>, CS: OutputPin>(
    spi: &mut SPI,
    cs: &mut CS,
) -> Result<u8, SPI::Error> {
    const AK8963_ADDRESS: u8 = 0x0C;
    const AK8964_WHO_AM_I: u8 = 0x00;

    const I2C_SLV4_ADDR: u8 = 0x31;
    const I2C_SLV4_REG: u8 = 0x32;
    const I2C_SLV4_CTRL: u8 = 0x34;
    const I2C_SLV4_DI: u8 = 0x35;

    const I2C_MST_STATUS: u8 = 0x36;
    const I2C_SLV4_DONE: u8 = 1 << 6;
    const I2C_LOST_ARB: u8 = 1 << 5;
    const I2C_SLV4_NACK: u8 = 1 << 4;
    const I2C_SLV_EN: u8 = 1 << 7;

    transact(cs, || {
        spi.transfer(&mut [write(I2C_SLV4_ADDR, AK8963_ADDRESS | (1 << 7))])
            .map(|_| ())
    })?;
    transact(cs, || {
        spi.transfer(&mut [write(I2C_SLV4_REG, AK8964_WHO_AM_I)])
            .map(|_| ())
    })?;
    transact(cs, || {
        spi.transfer(&mut [write(I2C_SLV4_CTRL, I2C_SLV_EN)])
            .map(|_| ())
    })?;

    let mut attempts = 0;
    loop {
        let mut buffer = [read(I2C_MST_STATUS)];
        transact(cs, || spi.transfer(&mut buffer))?;
        let status = (buffer[0] & 0xFF) as u8;
        if status & I2C_SLV4_DONE > 0 {
            break;
        } else if status & I2C_SLV4_NACK > 0 {
            log::warn!("I2C_SLV4_NACK");
            break;
        } else if status & I2C_LOST_ARB > 0 {
            log::warn!("I2C_LOST_ARB");
            break;
        }
        attempts += 1;
        if attempts >= 10_000 {
            log::warn!("Tried too many times");
            return Ok(0);
        }
    }

    let mut buffer = [read(I2C_SLV4_DI)];
    transact(cs, || spi.transfer(&mut buffer))?;
    Ok((buffer[0] & 0xFF) as u8)
}
