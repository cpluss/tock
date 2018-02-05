/* IOC - IO Configuration */

use kernel::common::VolatileCell;
use kernel::hil;

pub const IOC_PULL_CTL: u8 = 13;
pub const IOC_IE: u8 = 29;
pub const IOC_EDGE_DET: u8 = 16;
pub const IOC_EDGE_IRQ_EN: u8 = 18;

pub const IOC_UART0_RX_ID: u32 = 0xF;
pub const IOC_UART0_TX_ID: u32 = 0x10;

#[repr(C)]
pub struct IocRegisters {
    iocfg: [VolatileCell<u32>; 32],
}

const IOC_BASE: *mut IocRegisters = 0x4008_1000 as *mut IocRegisters;

pub struct IocfgPin {
    pin: usize,
}

impl IocfgPin {
    const fn new(pin: u8) -> IocfgPin {
        IocfgPin {
            pin: pin as usize,
        }
    }

    pub fn enable_gpio(&self) {
        let regs: &IocRegisters = unsafe { &*IOC_BASE };
        let pin_ioc = regs.iocfg[self.pin];

        // In order to configure the pin for GPIO we need to clear
        // the lower 6 bits.
        pin_ioc.set(pin_ioc.get() & !0x3F);
    }

    pub fn enable_uart_rx(&self) {
        let regs: &IocRegisters = unsafe { &*IOC_BASE };
        let pin_ioc = regs.iocfg[self.pin];

        pin_ioc.set(pin_ioc.get() | IOC_UART0_RX_ID);
        self.set_input_mode(hil::gpio::InputMode::PullNone);
        self.enable_input();
    }

    pub fn enable_uart_tx(&self) {
        let regs: &IocRegisters = unsafe { &*IOC_BASE };
        let pin_ioc = regs.iocfg[self.pin];

        pin_ioc.set(pin_ioc.get() | IOC_UART0_TX_ID);
        self.set_input_mode(hil::gpio::InputMode::PullNone);
        self.enable_output();
    }

    pub fn set_input_mode(&self, mode: hil::gpio::InputMode) {
        let regs: &IocRegisters = unsafe { &*IOC_BASE };
        let pin_ioc = regs.iocfg[self.pin];

        let conf = match mode {
            hil::gpio::InputMode::PullDown => 1,
            hil::gpio::InputMode::PullUp => 2,
            hil::gpio::InputMode::PullNone => 3,
        };

        pin_ioc.set(pin_ioc.get() & !(0b11 << IOC_PULL_CTL) | (conf << IOC_PULL_CTL));
    }

    pub fn enable_output(&self) {
        // Enable by disabling input
        let regs: &IocRegisters = unsafe { &*IOC_BASE };
        let pin_ioc = regs.iocfg[self.pin];
        pin_ioc.set(pin_ioc.get() & !(1 << IOC_IE));
    }

    pub fn enable_input(&self) {
        // Set IE (Input Enable) bit
        let regs: &IocRegisters = unsafe { &*IOC_BASE };
        let pin_ioc = regs.iocfg[self.pin];
        pin_ioc.set(pin_ioc.get() | 1 << IOC_IE);
    }

    pub fn enable_interrupt(&self, mode: hil::gpio::InterruptMode) {
        let regs: &IocRegisters = unsafe { &*IOC_BASE };
        let pin_ioc = regs.iocfg[self.pin];

        let ioc_edge_mode = match mode {
            hil::gpio::InterruptMode::FallingEdge => 1 << IOC_EDGE_DET,
            hil::gpio::InterruptMode::RisingEdge => 2 << IOC_EDGE_DET,
            hil::gpio::InterruptMode::EitherEdge => 3 << IOC_EDGE_DET,
        };

        pin_ioc.set(pin_ioc.get() & !(0b11 << IOC_EDGE_DET) | ioc_edge_mode | 1 << IOC_EDGE_IRQ_EN);
    }

    pub fn disable_interrupt(&self) {
        let regs: &IocRegisters = unsafe { &*IOC_BASE };
        let pin_ioc = regs.iocfg[self.pin];
        pin_ioc.set(pin_ioc.get() & !(1 << IOC_EDGE_IRQ_EN));
    }
}

pub static IOCFG: [IocfgPin; 32] = [
    IocfgPin::new(0),
    IocfgPin::new(1),
    IocfgPin::new(2),
    IocfgPin::new(3),
    IocfgPin::new(4),
    IocfgPin::new(5),
    IocfgPin::new(6),
    IocfgPin::new(7),
    IocfgPin::new(8),
    IocfgPin::new(9),
    IocfgPin::new(10),
    IocfgPin::new(11),
    IocfgPin::new(12),
    IocfgPin::new(13),
    IocfgPin::new(14),
    IocfgPin::new(15),
    IocfgPin::new(16),
    IocfgPin::new(17),
    IocfgPin::new(18),
    IocfgPin::new(19),
    IocfgPin::new(20),
    IocfgPin::new(21),
    IocfgPin::new(22),
    IocfgPin::new(23),
    IocfgPin::new(24),
    IocfgPin::new(25),
    IocfgPin::new(26),
    IocfgPin::new(27),
    IocfgPin::new(28),
    IocfgPin::new(29),
    IocfgPin::new(30),
    IocfgPin::new(31),
];
