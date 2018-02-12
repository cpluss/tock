use prcm;
use kernel::common::VolatileCell;

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
    }
}
