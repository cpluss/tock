use core::cell::Cell;
use kernel::common::VolatileCell;
use kernel::common::take_cell::TakeCell;
use kernel::hil::gpio::Pin;
use kernel::hil::uart;
use kernel;
use prcm;
use ioc::IocfgPin;
use gpio;
use peripheral_interrupts;
use cortexm3::nvic;

pub const UART_CTL_UARTEN: u32 = 1;
pub const UART_CTL_TXE: u32 = 1 << 8;
pub const UART_CTL_RXE: u32 = 1 << 9;
pub const UART_LCRH_FEN: u32 = 1 << 4;
pub const UART_FR_BUSY: u32 = 1 << 3;
pub const UART_INTERRUPT_ALL: u32 = 0x7F2;
pub const UART_FIFO_TX7_8: u32 = 0x04; // Transmit interrupt at 7/8 Full
pub const UART_FIFO_RX4_8: u32 = 0x10; // Receive interrupt at 1/2 Full
pub const UART_FR_TXFF: u32 = 0x20;

pub const UART_CONF_BAUD_RATE: u32 = 115200;
pub const UART_CONF_WLEN_8: u32 = 0x60;
pub const UART_INT_RX: u32 = 0x010;
pub const UART_INT_RT: u32 = 0x040; // Receive Timeout Interrupt Mask

pub const BOARD_IOID_UART_RX: u8 = 28;
pub const BOARD_IOID_UART_TX: u8 = 29;

pub const MCU_CLOCK: u32 = 48_000_000;

pub const UART_BASE: usize = 0x4000_1000;

#[repr(C)]
pub struct UART_REGS {
    pub dr: VolatileCell<u32>,
    pub rsr_ecr: VolatileCell<u32>,
    _reserved0: [VolatileCell<u8>; 0x10],
    pub fr: VolatileCell<u32>,
    _reserved1: [VolatileCell<u8>; 0x8],
    pub ibrd: VolatileCell<u32>,
    pub fbrd: VolatileCell<u32>,
    pub lcrh: VolatileCell<u32>,
    pub ctl: VolatileCell<u32>,
    pub ifls: VolatileCell<u32>,
    pub imsc: VolatileCell<u32>,
    pub ris: VolatileCell<u32>,
    pub mis: VolatileCell<u32>,
    pub icr: VolatileCell<u32>,
    pub dmactl: VolatileCell<u32>,
}

#[allow(non_snake_case)]
fn UART() -> &'static UART_REGS { unsafe { &*(UART_BASE as *const UART_REGS) } }

pub struct UART {
    client: Cell<Option<&'static uart::Client>>,
    buffer: TakeCell<'static, [u8]>,
    len: Cell<usize>,
    index: Cell<usize>,
}

pub static mut UART0: UART = UART::new();

impl UART {
    pub const fn new() -> UART {
        UART {
            client: Cell::new(None),
            buffer: TakeCell::empty(),
            len: Cell::new(0),
            index: Cell::new(0),
        }
    }

    pub fn enable(&self) {
        self.power_and_clock();
        self.disable();
        self.disable_interrupts();
        self.configure();
        self.enable_interrupts();
    }

    pub fn disable(&self) {
        self.fifo_disable();
        UART().ctl.set(UART().ctl.get() & !(UART_CTL_RXE | UART_CTL_TXE | UART_CTL_UARTEN));
    }

    fn fifo_enable(&self) {
        UART().lcrh.set(UART().lcrh.get() | UART_LCRH_FEN);
    }

    fn fifo_disable(&self) {
        UART().lcrh.set(UART().lcrh.get() & !UART_LCRH_FEN);
    }

    pub fn configure(&self) {
        let ctl_val = UART_CTL_UARTEN | UART_CTL_TXE | UART_CTL_RXE;

        /*
        * Make sure the TX pin is output / high before assigning it to UART control
        * to avoid falling edge glitches
        */
        unsafe {
            let tx_pin = &gpio::PORT[BOARD_IOID_UART_TX as usize];
            tx_pin.make_output();
            tx_pin.set();
        }

        // Map UART signals to IO pin
        let tx_pin: IocfgPin = IocfgPin::new(BOARD_IOID_UART_TX);
        tx_pin.enable_uart_tx();
        let rx_pin: IocfgPin = IocfgPin::new(BOARD_IOID_UART_RX);
        rx_pin.enable_uart_rx();

        // Disable the UART before configuring
        self.disable();

        // Fractional baud rate divider
        let div = (((MCU_CLOCK * 8) / UART_CONF_BAUD_RATE) + 1) / 2;

        // Set the baud rate
        UART().ibrd.set(div / 64);
        UART().fbrd.set(div % 64);

        // Set word length
        UART().lcrh.set(UART_CONF_WLEN_8);

        // Set fifo interrupt level
        UART().ifls.set(UART_FIFO_TX7_8 | UART_FIFO_RX4_8);
        self.fifo_enable();

        // Enable, TX, RT and UART
        UART().ctl.set(ctl_val);
    }

    fn power_and_clock(&self) {
        prcm::Power::enable_domain(prcm::PowerDomain::Serial);
        while !prcm::Power::is_enabled(prcm::PowerDomain::Serial) { };
        prcm::Clock::enable_uart_run();
    }

    pub fn disable_interrupts(&self) {
        unsafe {
            let uart0_int = nvic::Nvic::new(peripheral_interrupts::UART0);
            uart0_int.disable();
        }

        // Disable all UART module interrupts
        UART().imsc.set(UART().imsc.get() & !UART_INTERRUPT_ALL);

        // Clear all UART interrupts
        UART().icr.set(UART_INTERRUPT_ALL);
    }

    pub fn enable_interrupts(&self) {
        // Clear all UART interrupts
        UART().icr.set(UART_INTERRUPT_ALL);

        UART().imsc.set(UART().imsc.get() | UART_INT_RT | UART_INT_RX);

        unsafe {
            let uart0_int = nvic::Nvic::new(peripheral_interrupts::UART0);
            uart0_int.enable();
        }
    }

    pub fn put_char(&self, c: u8) {
        // Wait for space
        while (UART().fr.get() & UART_FR_TXFF) != 0 { }

        UART().dr.set(c as u32);
    }
}

impl kernel::hil::uart::UART for UART {
    fn set_client(&self, client: &'static kernel::hil::uart::Client) {
        self.client.set(Some(client));
    }

    fn init(&self, params: kernel::hil::uart::UARTParams) {
        self.enable();
    }

    #[allow(unused)]
    fn transmit(&self, tx_data: &'static mut [u8], tx_len: usize) {
    }

    #[allow(unused)]
    fn receive(&self, rx_buffer: &'static mut [u8], rx_len: usize) {}
}
