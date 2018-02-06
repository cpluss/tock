#![no_std]
#![no_main]
#![feature(lang_items, compiler_builtins_lib, asm)]

extern crate capsules;
extern crate compiler_builtins;

extern crate cc2650;
#[allow(unused_imports)]
#[macro_use(debug, debug_gpio, static_init)]
extern crate kernel;

use cc2650::prcm;
use cc2650::aon;

#[macro_use]
pub mod io;

// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::process::FaultResponse = kernel::process::FaultResponse::Panic;

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 2;
//
static mut PROCESSES: [Option<kernel::Process<'static>>; NUM_PROCS] = [None, None];

#[link_section = ".app_memory"]
static mut APP_MEMORY: [u8; 10240] = [0; 10240];

pub struct Platform {
    gpio: &'static capsules::gpio::GPIO<'static, cc2650::gpio::GPIOPin>,
    led: &'static capsules::led::LED<'static, cc2650::gpio::GPIOPin>,
    button: &'static capsules::button::Button<'static, cc2650::gpio::GPIOPin>,
    console: &'static capsules::console::Console<'static, cc2650::uart::UART>,
    alarm: &'static capsules::alarm::AlarmDriver<
        'static,
        capsules::virtual_alarm::VirtualMuxAlarm<'static, cc2650::rtc::Rtc>,
    >,
    rng: &'static capsules::rng::SimpleRng<'static, cc2650::trng::Trng>,
}

impl kernel::Platform for Platform {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&kernel::Driver>) -> R,
    {
        match driver_num {
            capsules::console::DRIVER_NUM => f(Some(self.console)),
            capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            capsules::led::DRIVER_NUM => f(Some(self.led)),
            capsules::button::DRIVER_NUM => f(Some(self.button)),
            capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            capsules::rng::DRIVER_NUM => f(Some(self.rng)),
            _ => f(None),
        }
    }
}

#[no_mangle]
pub unsafe fn reset_handler() {
    cc2650::init();

    // Setup AON event defaults
    aon::AON_EVENT.setup();

    // Power on peripherals (eg. GPIO)
    prcm::Power::enable_domain(prcm::PowerDomain::Peripherals);

    // Wait for it to turn on until we continue
    while !prcm::Power::is_enabled(prcm::PowerDomain::Peripherals) {}

    // Enable the GPIO clocks
    prcm::Clock::enable_gpio();

    // LEDs
    let led_pins = static_init!(
        [(
            &'static cc2650::gpio::GPIOPin,
            capsules::led::ActivationMode
        ); 2],
        [
            (
                &cc2650::gpio::PORT[10],
                capsules::led::ActivationMode::ActiveHigh
            ), // Red
            (
                &cc2650::gpio::PORT[15],
                capsules::led::ActivationMode::ActiveHigh
            ) // Green
        ]
    );
    let led = static_init!(
        capsules::led::LED<'static, cc2650::gpio::GPIOPin>,
        capsules::led::LED::new(led_pins)
    );

    // BUTTONs
    let button_pins = static_init!(
        [(&'static cc2650::gpio::GPIOPin, capsules::button::GpioMode); 2],
        [
            (
                &cc2650::gpio::PORT[0],
                capsules::button::GpioMode::LowWhenPressed
            ), // Button 2
            (
                &cc2650::gpio::PORT[4],
                capsules::button::GpioMode::LowWhenPressed
            ) // Button 1
        ]
    );
    let button = static_init!(
        capsules::button::Button<'static, cc2650::gpio::GPIOPin>,
        capsules::button::Button::new(button_pins, kernel::Grant::create())
    );
    for &(btn, _) in button_pins.iter() {
        btn.set_client(button);
    }

    let console = static_init!(
        capsules::console::Console<cc2650::uart::UART>,
        capsules::console::Console::new(
        &cc2650::uart::UART0,
        115200,
        &mut capsules::console::WRITE_BUF,
        kernel::Grant::create()
        )
    );
    kernel::hil::uart::UART::set_client(&cc2650::uart::UART0, console);
    console.initialize();

    // Attach the kernel debug interface to this console
    let kc = static_init!(capsules::console::App, capsules::console::App::default());
    kernel::debug::assign_console_driver(Some(console), kc);

    // Setup for remaining GPIO pins
    let gpio_pins = static_init!(
        [&'static cc2650::gpio::GPIOPin; 26],
        [
            &cc2650::gpio::PORT[1],
            &cc2650::gpio::PORT[2],
            &cc2650::gpio::PORT[3],
            &cc2650::gpio::PORT[5],
            &cc2650::gpio::PORT[6],
            &cc2650::gpio::PORT[7],
            &cc2650::gpio::PORT[8],
            &cc2650::gpio::PORT[9],
            &cc2650::gpio::PORT[11],
            &cc2650::gpio::PORT[12],
            &cc2650::gpio::PORT[13],
            &cc2650::gpio::PORT[14],
            &cc2650::gpio::PORT[16],
            &cc2650::gpio::PORT[17],
            &cc2650::gpio::PORT[18],
            &cc2650::gpio::PORT[19],
            &cc2650::gpio::PORT[20],
            &cc2650::gpio::PORT[21],
            &cc2650::gpio::PORT[22],
            &cc2650::gpio::PORT[23],
            &cc2650::gpio::PORT[24],
            &cc2650::gpio::PORT[25],
            &cc2650::gpio::PORT[26],
            &cc2650::gpio::PORT[27],
            &cc2650::gpio::PORT[30],
            &cc2650::gpio::PORT[31]
        ]
    );
    let gpio = static_init!(
        capsules::gpio::GPIO<'static, cc2650::gpio::GPIOPin>,
        capsules::gpio::GPIO::new(gpio_pins)
    );
    for pin in gpio_pins.iter() {
        pin.set_client(gpio);
    }

    let rtc = &cc2650::rtc::RTC;
    rtc.start();

    let mux_alarm = static_init!(
        capsules::virtual_alarm::MuxAlarm<'static, cc2650::rtc::Rtc>,
        capsules::virtual_alarm::MuxAlarm::new(&cc2650::rtc::RTC)
    );
    rtc.set_client(mux_alarm);

    let virtual_alarm1 = static_init!(
        capsules::virtual_alarm::VirtualMuxAlarm<'static, cc2650::rtc::Rtc>,
        capsules::virtual_alarm::VirtualMuxAlarm::new(mux_alarm)
    );
    let alarm = static_init!(
        capsules::alarm::AlarmDriver<
            'static,
            capsules::virtual_alarm::VirtualMuxAlarm<'static, cc2650::rtc::Rtc>,
        >,
        capsules::alarm::AlarmDriver::new(virtual_alarm1, kernel::Grant::create())
    );
    virtual_alarm1.set_client(alarm);

    cc2650::trng::TRNG.enable();
    let rng = static_init!(
        capsules::rng::SimpleRng<'static, cc2650::trng::Trng>,
        capsules::rng::SimpleRng::new(&cc2650::trng::TRNG, kernel::Grant::create())
    );
    cc2650::trng::TRNG.set_client(rng);

    let sensortag = Platform {
        gpio,
        led,
        button,
        console,
        alarm,
        rng: rng,
    };

    // Emit some nice RNG numbers now
    for i in 0..10 {
        debug!("{}: {}\r\n", i, cc2650::trng::TRNG.read_number());
    }

    let mut chip = cc2650::chip::Cc2650::new();

    debug!("Initialization complete. Entering main loop\r\n");
    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
    }

    kernel::process::load_processes(
        &_sapps as *const u8,
        &mut APP_MEMORY,
        &mut PROCESSES,
        FAULT_RESPONSE,
    );

    kernel::main(
        &sensortag,
        &mut chip,
        &mut PROCESSES,
        &kernel::ipc::IPC::new(),
    );
}
