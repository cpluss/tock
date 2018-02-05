use core::fmt::{write, Arguments, Write};
use kernel::hil::uart::{self, UART};
use kernel::hil::gpio::Pin;
use cc2650;

pub struct Writer {
    initialized: bool,
}

pub static mut WRITER: Writer = Writer { initialized: false };

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let uart = unsafe { &mut cc2650::uart::UART0 };
        if !self.initialized {
            self.initialized = true;
            uart.enable();
        }
        for c in s.bytes() {
            unsafe {
                uart.send_byte(c);
            }
            while !uart.tx_ready() {}
        }
        Ok(())
    }
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub unsafe extern "C" fn rust_begin_unwind(
    _args: Arguments,
    _file: &'static str,
    _line: usize,
) -> ! {

    let writer = &mut WRITER;
    writer.write_fmt(format_args!(
        "\r\nKernel panic at {}:{}:\r\n\t\"",
        _file, _line
    ));

    let led0 = &cc2650::gpio::PORT[10];
    let led1 = &cc2650::gpio::PORT[15];

    led0.make_output();
    led1.make_output();
    loop {
        for _ in 0..1000000 {
            led0.clear();
            led1.clear();
        }
        for _ in 0..100000 {
            led0.set();
            led1.set();
        }
        for _ in 0..1000000 {
            led0.clear();
            led1.clear();
        }
        for _ in 0..500000 {
            led0.set();
            led1.set();
        }
    }
}
