use prcm;
use kernel::common::VolatileCell;

pub const I2C_MCR_MFE: u32 = 0x10;
pub const I2C_MCTRL_RUN: u32 = 0x1;

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
}
