use prcm;
use kernel::common::VolatileCell;

pub const I2C_MCR_MFE: u32 = 0x10;
pub const I2C_MSTAT_BUSY: u32 = 0x1;
pub const I2C_MCTRL_RUN: u32 = 0x1;
pub const I2C_MASTER_CMD_SINGLE_SEND: u32 = 0x7;

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
        if !self.busy_wait_master_bus() { return false; }

        true // Need to check for errors before returning true
    }

    fn set_master_slave_address(&self, addr: u8, receive: bool) {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        regs.msa.set(((addr as u32) << 1) | (receive as u32));
    }

    fn master_put_data(&self, data: u8) {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        regs.mdr.set(data as u32);
    }

    fn master_bus_busy(&self) -> bool {
        let regs: &I2CRegisters = unsafe { &*self.regs };
        (regs.mstat_mctrl.get() & I2C_MSTAT_BUSY) != 0
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

}
