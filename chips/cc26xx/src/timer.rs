use kernel::common::VolatileCell;
use core::cell::Cell;
use prcm;

#[repr(C)]
pub struct Registers {
   pub cfg: VolatileCell<u32>,
   pub tamr: VolatileCell<u32>,
   pub tbmr: VolatileCell<u32>,
   pub ctl: VolatileCell<u32>,
   pub sync: VolatileCell<u32>,

   _reserved0: [u8; 0x4],

   pub imr: VolatileCell<u32>,
   pub ris: VolatileCell<u32>,
   pub mis: VolatileCell<u32>,
   pub iclr: VolatileCell<u32>,
   pub tailr: VolatileCell<u32>,
   pub tbilr: VolatileCell<u32>,
   pub tamatchr: VolatileCell<u32>,
   pub tbmatchr: VolatileCell<u32>,
   pub tapr: VolatileCell<u32>,
   pub tbpr: VolatileCell<u32>,
   pub tapmr: VolatileCell<u32>,
   pub tbpmr: VolatileCell<u32>,
   pub tar: VolatileCell<u32>,
   pub tbr: VolatileCell<u32>,
   pub tav: VolatileCell<u32>,
   pub tbv: VolatileCell<u32>,

   _reserved1: [u8; 0x4],

   pub taps: VolatileCell<u32>,
   pub tbps: VolatileCell<u32>,
   pub tapv: VolatileCell<u32>,
   pub tbpv: VolatileCell<u32>,
   pub dmaev: VolatileCell<u32>,

   _reserved2: [u8; 0xF40],

   pub version: VolatileCell<u32>,
   pub andccp: VolatileCell<u32>,
}

pub const GPT_CFG_32_BIT: u32 = 0x0;
pub const GPT_ONE_SHOT: u32 = 0x1;
pub const GPT_REG_BIT: u32 = 0x1;

#[derive(Copy,Clone,PartialEq)]
pub enum TimerBase {
   GPT0 = 0x4001_0000,
   GPT1 = 0x4001_1000,
   GPT2 = 0x4001_2000,
   GPT3 = 0x4001_3000,
}

pub static mut GPT0: Timer = Timer::new(TimerBase::GPT0);
pub static mut GPT1: Timer = Timer::new(TimerBase::GPT1);
pub static mut GPT2: Timer = Timer::new(TimerBase::GPT2);
pub static mut GPT3: Timer = Timer::new(TimerBase::GPT3);

pub struct Timer {
   regs: *const Registers,
   reg_bit: u32,
   client: Cell<Option<&'static TimerClient>>,
}

trait TimerClient {
   fn fired(&self);
}

pub fn power_on_timers() {
    prcm::Power::enable_domain(prcm::PowerDomain::Serial);
    while !prcm::Power::is_enabled(prcm::PowerDomain::Serial) { }
    prcm::Clock::enable_gpt();
}

impl Timer {
   const fn new(gpt_base: TimerBase) -> Timer {
      Timer {
         regs: (gpt_base as u32) as *const Registers,
         reg_bit: GPT_REG_BIT,
         client: Cell::new(None),
      }
   }

   pub fn one_shot(&self, value: u32) {
       let regs: &Registers = unsafe { &*self.regs };

       // Disable timer before configuration
       regs.ctl.set(regs.ctl.get() & !self.reg_bit);
       regs.cfg.set(GPT_CFG_32_BIT);

       // Set type and initial timer value
       regs.tamr.set(GPT_ONE_SHOT);
       regs.tailr.set(value);

       // Enable interrupts and start the timer
       regs.imr.set(self.reg_bit);
       regs.ctl.set(regs.ctl.get() | self.reg_bit);
   }

   pub fn has_fired(&self) -> bool {
     let regs: &Registers = unsafe { &*self.regs };
     (regs.mis.get() & self.reg_bit) != 0
   }

   pub fn handle_interrupt(&self) {
      let regs: &Registers = unsafe { &*self.regs };
      regs.iclr.set(regs.iclr.get() | self.reg_bit);
      self.client.get().map(|client| {
         client.fired();
      });
   }
}
