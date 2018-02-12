use prcm;
use kernel::hil;
use kernel::common::VolatileCell;

pub const I2C_MCR_MFE: u32 = 0x10;
pub const I2C_MCTRL_RUN: u32 = 0x1;

pub const I2C_MASTER_CMD_SINGLE_SEND: u32 = 0x7;
pub const I2C_MASTER_CMD_BURST_SEND_ERROR_STOP: u32 = 0x4;
pub const I2C_MASTER_CMD_BURST_RECEIVE_START: u32 = 0xb;
pub const I2C_MASTER_CMD_BURST_RECEIVE_CONT: u32 = 0x9;
pub const I2C_MASTER_CMD_BURST_SEND_START: u32 = 0x3;
pub const I2C_MASTER_CMD_BURST_SEND_CONT: u32 = 0x1;
pub const I2C_MASTER_CMD_BURST_SEND_FINISH: u32 = 0x5;

pub const I2C_MSTAT_ERR: u32 = 0x2;
pub const I2C_MSTAT_BUSY: u32 = 0x1;
pub const I2C_MSTAT_BUSBSY: u32 = 0x40;
pub const I2C_MSTAT_ARBLST: u32 = 0x10;
pub const I2C_MSTAT_DATACK_N: u32 = 0x8;
pub const I2C_MSTAT_ADRACK_N: u32 = 0x4;
pub const I2C_MSTAT_DATACK_N_M: u32 = 0x8;
pub const I2C_MSTAT_ADRACK_N_M: u32 = 0x4;

pub const MCU_CLOCK: u32 = 48_000_000;

#[repr(C)]
pub struct I2CRegisters {
    pub soar: VolatileCell<u32>,
    pub sstat_sctl: VolatileCell<u32>,
    pub sdr: VolatileCell<u32>,
    pub simr: VolatileCell<u32>,
    pub sris: VolatileCell<u32>,
    pub smis: VolatileCell<u32>,
    pub sicr: VolatileCell<u32>,

    _reserved0: [u8; 0x7e4],

    pub msa: VolatileCell<u32>,
    pub mstat_mctrl: VolatileCell<u32>,
    pub mdr: VolatileCell<u32>,
    pub mtpr: VolatileCell<u32>,
    pub mimr: VolatileCell<u32>,
    pub mmis: VolatileCell<u32>,
    pub micr: VolatileCell<u32>,
    pub mcr: VolatileCell<u32>,
}

pub const I2C_BASE: *mut I2CRegisters = 0x4000_2000 as *mut I2CRegisters;

pub struct I2C {
    regs: *mut I2CRegisters
}

impl I2C {
    pub fn wakeup(&self) {
        prcm::Power::enable_domain(prcm::PowerDomain::Serial);
        while !prcm::Power::is_enabled(prcm::PowerDomain::Serial) { };
        prcm::Clock::enable_i2c();

        self.configure(true);
    }

    fn configure(&self, fast: bool) {
        self.master_enable();

        let freq;
        if fast { freq = 400_000; } else { freq = 100_000; }

        // Compute SCL (serial clock) period
        let tpr = ((MCU_CLOCK + (2 * 10 * freq) - 1) / (2 * 10 * freq)) - 1;
        let regs: &I2CRegisters = unsafe { &*self.regs };
        regs.mtpr.set(tpr);
    }

    fn master_enable(&self) {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        // Set as master
        regs.mcr.set(regs.mcr.get() | I2C_MCR_MFE);
        // Enable master to transfer/receive data
        regs.mstat_mctrl.set(I2C_MCTRL_RUN);
    }

    fn write_single(&self, addr: u8, data: u8) -> bool {
        self.set_master_slave_address(addr, false);
        self.master_put_data(data);

        if !self.busy_wait_master_bus() { return false; }

        self.master_control(I2C_MASTER_CMD_SINGLE_SEND);
        if !self.busy_wait_master() { return false; }

        self.status()
    }

    fn read(&self, addr: u8, data: &'static mut [u8], len: u8) {
        self.set_master_slave_address(addr, true);

        self.busy_wait_master_bus();

        self.master_control(I2C_MASTER_CMD_BURST_RECEIVE_START);

        let mut i = 0;
        let mut success = true;
        while i < (len - 1) && success {
            self.busy_wait_master();
            success = self.status();
            if success {
                data[i as usize] = self.master_get_data() as u8;
                self.master_control(I2C_MASTER_CMD_BURST_RECEIVE_CONT);
                i += 1;
            }
        }
    }

    fn write(&self, addr: u8, data: &'static mut [u8], len: u8) -> bool {
        self.set_master_slave_address(addr, false);

        self.master_put_data(data[0]);

        self.busy_wait_master_bus();

        self.master_control(I2C_MASTER_CMD_BURST_SEND_START);
        self.busy_wait_master();
        let mut success = self.status();

        for i in 0..len {
            if !success { break; }
            self.master_put_data(data[i as usize]);
            if i < len - 1 {
                self.master_control(I2C_MASTER_CMD_BURST_SEND_CONT);
                self.busy_wait_master();
                success = self.status();
            }
        }

        if success {
            self.master_control(I2C_MASTER_CMD_BURST_SEND_FINISH);
            self.busy_wait_master();
            success = self.status();
            self.busy_wait_master_bus();
        }

        success
    }

    fn set_master_slave_address(&self, addr: u8, receive: bool) {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        regs.msa.set(((addr as u32) << 1) | (receive as u32));
    }

    fn master_put_data(&self, data: u8) {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        regs.mdr.set(data as u32);
    }

    fn master_get_data(&self) -> u32 {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        regs.mdr.get()
    }

    fn master_bus_busy(&self) -> bool {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        (regs.mstat_mctrl.get() & I2C_MSTAT_BUSBSY) != 0
    }

    fn master_busy(&self) -> bool {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        (regs.mstat_mctrl.get() & I2C_MSTAT_BUSY) != 0
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
        let regs: &I2CRegisters = unsafe { &*self.regs };
        regs.mstat_mctrl.set(cmd);
    }

    fn status(&self) -> bool {
        let status = self.master_err();

        if (status & (I2C_MSTAT_DATACK_N_M | I2C_MSTAT_ADRACK_N_M)) != 0 {
            self.master_control(I2C_MASTER_CMD_BURST_SEND_ERROR_STOP);
        }

        status == 0
    }

    fn master_err(&self) -> u32 {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        let err = regs.mstat_mctrl.get();

        // If the master is busy there is not error to report
        if (err & I2C_MSTAT_BUSY) == 1 {
            return 0;
        }

        // Check for errors
        if err & (I2C_MSTAT_ERR | I2C_MSTAT_ARBLST) != 0 {
            return err & (I2C_MSTAT_ARBLST | I2C_MSTAT_DATACK_N | I2C_MSTAT_ADRACK_N);
        } else {
                return 0;
        }
    }

}

impl hil::i2c::I2CMaster for I2C {
    /// This enables the entire I2C peripheral
    #[allow(unused)]
    fn enable(&self) {
    }

    /// This disables the entire I2C peripheral
    #[allow(unused)]
    fn disable(&self) {
    }

    #[allow(unused)]
    fn write(&self, addr: u8, data: &'static mut [u8], len: u8) {
    }

    fn read(&self, addr: u8, data: &'static mut [u8], len: u8) {
        self.read(addr, data, len);
    }

    #[allow(unused)]
    fn write_read(&self, addr: u8, data: &'static mut [u8], write_len: u8, read_len: u8) {
    }
}
