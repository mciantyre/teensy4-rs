use crate::interrupt; // bring in interrupt variants for #[interrupt] macro
use core::fmt;
use teensy4_usb_sys as usbsys;

pub struct Logging(&'static mut Logger);

impl Logging {
    /// Initializes the logger
    pub fn init(self) {
        self.0.enabled = true;
        ::log::set_logger(self.0)
            .map(|_| ::log::set_max_level(::log::STATIC_MAX_LEVEL))
            .unwrap();
        unsafe {
            usbsys::usb_pll_start();
            usbsys::usb_init();
            cortex_m::peripheral::NVIC::unmask(crate::interrupt::USB_OTG1);
        }
    }

    /// # Safety
    ///
    /// This is only called once, when we're setting up peripherals.
    /// If `init()` is called, we will set the members of the struct
    /// into their state. There can only be one Logging struct, so
    /// there's only one reference to the logger singleton.
    pub(super) fn new() -> Self {
        unsafe { Logging(&mut LOGGER) }
    }
}

#[crate::rt::interrupt]
fn USB_OTG1() {
    unsafe {
        usbsys::isr();
    }
}

struct Logger {
    /// Tracks if we are (not) enabled
    enabled: bool,
}

static mut LOGGER: Logger = Logger { enabled: false };

impl ::log::Log for Logger {
    fn enabled(&self, metadata: &::log::Metadata) -> bool {
        metadata.level() <= ::log::max_level()
    }

    fn log(&self, record: &::log::Record) {
        if !self.enabled {
            return;
        }

        use core::fmt::Write;
        // Stack space for writing
        let mut buffer = [0; 256];
        let mut cursor = Cursor::new(&mut buffer);
        if self.enabled(record.metadata()) {
            // Dropping errors...
            let _ = write!(&mut cursor, "[{} ({})]: ", record.level(), record.target());
            usbsys::serial_write(&cursor);
            cursor.clear();
            let _ = write!(&mut cursor, "{}\r\n", record.args());
            usbsys::serial_write(&cursor);
            cursor.clear();
        }
    }

    fn flush(&self) {
        unsafe { usbsys::usb_serial_flush_output() }
    }
}

/// Custom cursor for writing into buffers
struct Cursor<'a> {
    buffer: &'a mut [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        Cursor { buffer, offset: 0 }
    }

    fn clear(&mut self) {
        self.offset = 0;
    }
}

impl<'a> fmt::Write for Cursor<'a> {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        let src = msg.as_bytes();
        let buf = &mut self.buffer[self.offset..];
        if buf.len() < src.len() {
            return Err(fmt::Error);
        }
        let buf = &mut buf[..src.len()];
        buf.copy_from_slice(src);
        self.offset += src.len();
        Ok(())
    }
}

impl<'a> AsRef<[u8]> for Cursor<'a> {
    fn as_ref(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
}
