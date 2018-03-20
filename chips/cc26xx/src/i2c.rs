use gpio;
use ioc;
use prcm;
use core::cell::Cell;
use kernel::common::regs::{ReadOnly, ReadWrite, WriteOnly};
use kernel::hil::gpio::Pin;

pub const I2C_MCR_MFE: u32 = 0x10;
pub const I2C_MCTRL_RUN: u32 = 0x1;

// I2C commands
const SINGLE_SEND: u32 = 0x7;
const BURST_SEND_ERROR_STOP: u32 = 0x4;
const BURST_RECEIVE_START: u32 = 0xb;
const BURST_RECEIVE_CONT: u32 = 0x9;
const BURST_SEND_START: u32 = 0x3;
const BURST_SEND_CONT: u32 = 0x1;
const BURST_SEND_FINISH: u32 = 0x5;
const BURST_RECEIVE_FINISH: u32 = 0x5;

// Pin configuration
pub const BOARD_IO_SDA: usize = 0x5;
pub const BOARD_IO_SCL: usize = 0x6;
pub const BOARD_IO_SDA_HP: usize = 0x8;
pub const BOARD_IO_SCL_HP: usize = 0x9;

pub const MCU_CLOCK: u32 = 48_000_000;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum I2cInterface {
    Interface0 = 0,
    Interface1 = 1,
    NoInterface = 2,
}

#[repr(C)]
pub struct Registers {
    pub soar: ReadWrite<u32>,
    pub sstat_sctl: ReadWrite<u32>,
    pub sdr: ReadWrite<u32>,
    pub simr: ReadWrite<u32>,
    pub sris: ReadOnly<u32>,
    pub smis: ReadOnly<u32>,
    pub sicr: WriteOnly<u32>,

    _reserved0: [u8; 0x7e4],

    pub msa: ReadWrite<u32>,
    pub mstat_mctrl: ReadWrite<u32, MasterStatusAndControl::Register>,
    pub mdr: ReadWrite<u32>,
    pub mtpr: ReadWrite<u32, TimerPeriod::Register>,
    pub mimr: ReadWrite<u32>,
    pub mris: ReadOnly<u32>,
    pub mmis: ReadOnly<u32>,
    pub micr: WriteOnly<u32>,
    pub mcr: ReadWrite<u32, MasterConfiguration::Register>,
}

register_bitfields![
    u32,
    TimerPeriod [
        SCL_CLOCK_PERIOD OFFSET(0) NUMBITS(7) []
    ],
    MasterConfiguration [
        MASTER_FUNCTION_ENABLE OFFSET(4) NUMBITS(1) []
    ],
    MasterStatusAndControl [
        // Control
        ENABLE_MASTER OFFSET(0) NUMBITS(1) [],
        // Status
        CONTROLLER_BUSY OFFSET(0) NUMBITS(1) [],
        ERROR OFFSET(1) NUMBITS(1) [],
        BUS_BUSY OFFSET(6) NUMBITS(1) [],
        ARBITRATION_LOST OFFSET(4) NUMBITS(1) [],
        DATA_NOT_ACKED OFFSET(3) NUMBITS(1) [],
        ADDRESS_NOT_ACKED OFFSET(2) NUMBITS(1) []
    ]
];

pub const I2C_BASE: *mut Registers = 0x4000_2000 as *mut Registers;

pub static mut I2C0: I2C = I2C::new();

pub struct I2C {
    regs: *mut Registers,
    slave_addr: Cell<u8>,
    interface: Cell<u8>,
}

impl I2C {
    pub const fn new() -> I2C {
        I2C {
            regs: I2C_BASE as *mut Registers,
            slave_addr: Cell::new(0),
            interface: Cell::new(I2cInterface::NoInterface as u8),
        }
    }

    pub fn wakeup(&self) {
        prcm::Power::enable_domain(prcm::PowerDomain::Serial);
        while !prcm::Power::is_enabled(prcm::PowerDomain::Serial) {}
        prcm::Clock::enable_i2c();

        self.configure(true);
    }

    #[allow(unused)]
    pub fn shutdown(&self) {
        // Not implemented
    }

    fn configure(&self, fast: bool) {
        self.master_enable();

        let freq;
        if fast {
            freq = 400_000;
        } else {
            freq = 100_000;
        }

        // Compute SCL (serial clock) period
        let tpr = ((MCU_CLOCK + (2 * 10 * freq) - 1) / (2 * 10 * freq)) - 1;
        let regs: &Registers = unsafe { &*self.regs };
        regs.mtpr.write(TimerPeriod::SCL_CLOCK_PERIOD.val(tpr));
    }

    fn master_enable(&self) {
        let regs: &Registers = unsafe { &*self.regs };
        // Set as master
        regs.mcr.modify(MasterConfiguration::MASTER_FUNCTION_ENABLE::SET);
        // Enable master to transfer/receive data
        regs.mstat_mctrl.write(MasterStatusAndControl::ENABLE_MASTER::SET);
    }

    fn master_disable(&self) {
        let regs: &Registers = unsafe { &*self.regs };
        regs.mstat_mctrl.set(0);
        regs.mcr.modify(MasterConfiguration::MASTER_FUNCTION_ENABLE::CLEAR);
    }

    pub fn write_single(&self, data: u8) -> bool {
        self.set_master_slave_address(self.slave_addr.get(), false);
        self.master_put_data(data);

        if !self.busy_wait_master_bus() {
            return false;
        }

        self.master_control(SINGLE_SEND);
        if !self.busy_wait_master() {
            return false;
        }

        self.status()
    }

    pub fn read(&self, data: &mut [u8], len: u8) -> bool {
        self.set_master_slave_address(self.slave_addr.get(), true);

        self.busy_wait_master_bus();

        self.master_control(BURST_RECEIVE_START);

        let mut i = 0;
        let mut success = true;
        while i < (len - 1) && success {
            self.busy_wait_master();
            success = self.status();
            if success {
                data[i as usize] = self.master_get_data() as u8;
                self.master_control(BURST_RECEIVE_CONT);
                i += 1;
            }
        }

        if success {
            self.master_control(BURST_RECEIVE_FINISH);
            self.busy_wait_master();
            success = self.status();
            if success {
                data[(len - 1) as usize] = self.master_get_data() as u8;
                self.busy_wait_master_bus();
            }
        }

        success
    }

    pub fn write(&self, data: &[u8], len: u8) -> bool {
        self.set_master_slave_address(self.slave_addr.get(), false);

        self.master_put_data(data[0]);

        self.busy_wait_master_bus();

        self.master_control(BURST_SEND_START);
        self.busy_wait_master();
        let mut success = self.status();

        for i in 1..len {
            if !success {
                break;
            }
            self.master_put_data(data[i as usize]);
            if i < len - 1 {
                self.master_control(BURST_SEND_CONT);
                self.busy_wait_master();
                success = self.status();
            }
        }

        if success {
            self.master_control(BURST_SEND_FINISH);
            self.busy_wait_master();
            success = self.status();
            self.busy_wait_master_bus();
        }

        success
    }

    pub fn write_read(&self, data: &mut [u8], write_len: u8, read_len: u8) -> bool {
        self.set_master_slave_address(self.slave_addr.get(), false);

        self.master_put_data(data[0]);

        self.busy_wait_master_bus();

        self.master_control(BURST_SEND_START);
        self.busy_wait_master();
        let mut success = self.status();

        for i in 1..write_len {
            if !success {
                break;
            }

            self.master_put_data(data[i as usize]);

            self.master_control(BURST_SEND_CONT);
            self.busy_wait_master();
            success = self.status();
        }

        if !success {
            return false;
        }

        self.set_master_slave_address(self.slave_addr.get(), true);

        self.master_control(BURST_RECEIVE_START);

        let mut i = 0;
        while i < (read_len - 1) && success {
            self.busy_wait_master();
            success = self.status();
            if success {
                data[i as usize] = self.master_get_data() as u8;
                self.master_control(BURST_RECEIVE_CONT);
                i += 1;
            }
        }

        if success {
            self.master_control(BURST_RECEIVE_FINISH);
            self.busy_wait_master();
            success = self.status();
            if success {
                data[(read_len - 1) as usize] = self.master_get_data() as u8;
                self.busy_wait_master_bus();
            }
        }

        success
    }

    fn set_master_slave_address(&self, addr: u8, receive: bool) {
        let regs: &Registers = unsafe { &*self.regs };
        regs.msa.set(((addr as u32) << 1) | (receive as u32));
    }

    fn master_put_data(&self, data: u8) {
        let regs: &Registers = unsafe { &*self.regs };
        regs.mdr.set(data as u32);
    }

    fn master_get_data(&self) -> u32 {
        let regs: &Registers = unsafe { &*self.regs };
        regs.mdr.get()
    }

    fn master_bus_busy(&self) -> bool {
        let regs: &Registers = unsafe { &*self.regs };
        regs.mstat_mctrl.is_set(MasterStatusAndControl::BUS_BUSY)
    }

    fn master_busy(&self) -> bool {
        let regs: &Registers = unsafe { &*self.regs };
        regs.mstat_mctrl.is_set(MasterStatusAndControl::CONTROLLER_BUSY)
    }

    // Limited busy wait for the master
    fn busy_wait_master(&self) -> bool {
        let delay = 0xFFFFFF;
        for _ in 0..delay {
            if !self.master_busy() {
                return true;
            }
        }
        false
    }

    // Limited busy wait for the master bus
    fn busy_wait_master_bus(&self) -> bool {
        let delay = 0xFFFFFF;
        for _ in 0..delay {
            if !self.master_bus_busy() {
                return true;
            }
        }
        false
    }

    fn master_control(&self, cmd: u32) {
        let regs: &Registers = unsafe { &*self.regs };
        regs.mstat_mctrl.set(cmd);
    }

    fn status(&self) -> bool {
        //let status = self.master_err();
        let regs: &Registers = unsafe { &*self.regs };
        let mut success = true;

        // If the master is busy there is no error to report
        let busy = regs.mstat_mctrl.is_set(MasterStatusAndControl::CONTROLLER_BUSY);

        if !busy {
            if regs.mstat_mctrl.is_set(MasterStatusAndControl::ERROR) {
                // See if message was not acknowledged
                if regs.mstat_mctrl.is_set(MasterStatusAndControl::DATA_NOT_ACKED)
                    || regs.mstat_mctrl.is_set(MasterStatusAndControl::ADDRESS_NOT_ACKED) {
                    self.master_control(BURST_SEND_ERROR_STOP);
                    success = false;
                }
            } else if regs.mstat_mctrl.is_set(MasterStatusAndControl::ARBITRATION_LOST) {
                success = false;
            }
        }

        success
    }

    fn accessible(&self) -> bool {
        if !prcm::Power::is_enabled(prcm::PowerDomain::Serial) {
            return false;
        }

        if !prcm::Clock::i2c_run_clk_enabled() {
            return false;
        }

        true
    }

    pub fn select(&self, new_interface: I2cInterface, addr: u8) {
        self.slave_addr.set(addr);

        if !self.accessible() {
            self.wakeup();
        }

        let interface = new_interface as u8;
        if interface != self.interface.get() as u8 {
            self.interface.set(interface);

            self.master_disable();

            if interface == I2cInterface::Interface0 as u8 {
                unsafe {
                    ioc::IOCFG[BOARD_IO_SDA].enable_i2c_sda();
                    ioc::IOCFG[BOARD_IO_SCL].enable_i2c_scl();
                    gpio::PORT[BOARD_IO_SDA_HP].make_input();
                    gpio::PORT[BOARD_IO_SCL_HP].make_input();
                }
            } else if interface == I2cInterface::Interface1 as u8 {
                unsafe {
                    ioc::IOCFG[BOARD_IO_SDA_HP].enable_i2c_sda();
                    ioc::IOCFG[BOARD_IO_SCL_HP].enable_i2c_scl();
                    gpio::PORT[BOARD_IO_SDA].make_input();
                    gpio::PORT[BOARD_IO_SCL].make_input();
                }
            }

            self.configure(true);
        }
    }
}
