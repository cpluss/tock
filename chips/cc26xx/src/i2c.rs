use ioc;
use prcm;
use kernel::common::regs::{ReadOnly, ReadWrite, WriteOnly};
use kernel::hil::i2c::I2CMaster;

// I2C commands
const SINGLE_SEND: u32 = 0x7;
const BURST_SEND_START: u32 = 0x3;
const BURST_SEND_CONT: u32 = 0x1;
const BURST_SEND_FINISH: u32 = 0x5;
const BURST_SEND_ERROR_STOP: u32 = 0x4;
const BURST_RECEIVE_START: u32 = 0xb;
const BURST_RECEIVE_CONT: u32 = 0x9;
const BURST_RECEIVE_FINISH: u32 = 0x5;

pub const MCU_CLOCK: u32 = 48_000_000;

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
}

impl I2C {
    pub const fn new() -> I2C {
        I2C {
            regs: I2C_BASE as *mut Registers,
        }
    }

    pub fn init(&self, sda: u8, scl: u8) {
        self.wakeup();
        self.master_disable();
        ioc::IOCFG[sda as usize].enable_i2c_sda();
        ioc::IOCFG[scl as usize].enable_i2c_scl();
        self.master_enable();
        self.configure_clock(true);
    }

    pub fn wakeup(&self) {
        prcm::Power::enable_domain(prcm::PowerDomain::Serial);
        while !prcm::Power::is_enabled(prcm::PowerDomain::Serial) {}
        prcm::Clock::enable_i2c();
    }

    fn configure_clock(&self, fast: bool) {
        let freq;
        if fast { freq = 400_000; } else { freq = 100_000; }

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

    pub fn write_single(&self, addr: u8, data: u8) -> bool {
        self.set_master_slave_address(addr, false);
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

    pub fn read(&self, addr: u8, data: &mut [u8], len: u8) -> bool {
        self.set_master_slave_address(addr, true);

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

    pub fn write(&self, addr: u8, data: &[u8], len: u8) -> bool {
        self.set_master_slave_address(addr, false);

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

    pub fn write_read(&self,addr: u8, data: &mut [u8], write_len: u8, read_len: u8) -> bool {
        self.set_master_slave_address(addr, false);

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

        self.set_master_slave_address(addr, true);

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
}

impl I2CMaster for I2C {
    fn enable(&self) { self.master_enable(); }

    fn disable(&self) { self.master_disable(); }

    fn write_read(&self, addr: u8, data: &'static mut [u8], write_len: u8, read_len: u8) {
        self.write_read(addr, data, write_len, read_len);
    }

    fn write(&self, addr: u8, data: &'static mut [u8], len: u8) {
        self.write(addr, data, len);
    }

    fn read(&self, addr: u8, buffer: &'static mut [u8], len: u8) {
        self.read(addr, buffer, len);
    }
}
