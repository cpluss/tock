//! RF Core
//!
//! Provides communication with the core module of the radio.
//!     See page 1586 in the datasheet for more details.
//!
//! The radio is managed by an external Cortex-M0 running prioprietary code in order to manage
//! and set everything up. All stacks is implemented on this external MCU, and interaction
//! with it enables the radio to do all kinds of communication.
//!
//! In order to communicate, we send commands to the Cortex-M0 through something called
//! "Radio Doorbell", which has some fancy features attached to it.
//!
//! This abstraction allows us to perform complex radio instructions with a set of simple
//! packages and commands.
//!
//!

// RFC Commands are located at the bottom
use self::rfc_commands::*;
use cc26xx::prcm;
use cc26xx::rtc;
use core::cell::Cell;

use kernel::common::regs::{ReadOnly, ReadWrite};
use kernel::common::VolatileCell;

/*
    RFC commands can be of two types:
        * Direct Commands
        * Radio operation / Immediate
*/
pub enum RfcCommandType {
    Direct,
    Immediate,
}

/*
    Trait to implement custom RFC commands.
*/
pub trait RfcCommand {
    fn command_id(&self) -> &u16;
    fn command_status(&self) -> &u16;
    fn command_type(&self) -> RfcCommandType;

    /*
        A direct command structure of CMDR:
        bit  31                    16               8               2    0
            ----------------------------------------------------------------
            | Command ID (16 bits) | Opt. param     | Opt. ext      | 0  1 |
            ----------------------------------------------------------------
    */
    fn direct_command(&self) -> u32 {
        let cmd = *self.command_id() as u32;
        let par = *self.command_status() as u32;
        (cmd << 16) | (par & 0xFFFC) | 1
    }

    /*
        A radio op / immediate command structure of CMDR:
        bit  31                    16               8               2    0
            ----------------------------------------------------------------
            | Command ID (16 bits) | Opt. param     | Opt. ext      | 0  1 |
            ----------------------------------------------------------------
    */
    fn immediate_command(&self) -> *const Self {
        self as *const Self
    }
}

#[repr(C)]
pub struct RfcBellRegisters {
    cmdr: ReadWrite<u32>,
    cmdsta: ReadOnly<u32>,
    _rf_hw_interrupt_flags: ReadOnly<u32>,
    _rf_hw_interrupt_enable: ReadOnly<u32>,

    rf_cpe_interrupt_flags: ReadWrite<u32, RFCpeInterrupts::Register>,
    rf_cpe_interrupt_enable: ReadWrite<u32, RFCpeInterrupts::Register>,
    rf_cpe_interrupt_vector_sel: ReadWrite<u32, RFCpeInterrupts::Register>,

    rf_ack_interrupt_flag: ReadWrite<u32, RFAckInterruptFlag::Register>,

    _sys_gpo_control: ReadOnly<u32>,
}

register_bitfields![
    u32,
    RFCpeInterrupts [
        INTERNAL_ERROR      OFFSET(31) NUMBITS(1) [],
        BOOT_DONE           OFFSET(30) NUMBITS(1) [],
        MODULES_UNLOCKED    OFFSET(29) NUMBITS(1) [],
        SYNTH_NO_LOCK       OFFSET(28) NUMBITS(1) [],
        IRQ27               OFFSET(27) NUMBITS(1) [],
        RX_ABORTED          OFFSET(26) NUMBITS(1) [],
        RX_N_DATA_WRITTEN   OFFSET(25) NUMBITS(1) [],
        RX_DATA_WRITTEN     OFFSET(24) NUMBITS(1) [],
        RX_ENTRY_DONE       OFFSET(23) NUMBITS(1) [],
        RX_BUF_FULL         OFFSET(22) NUMBITS(1) [],
        RX_CTRL_ACK         OFFSET(21) NUMBITS(1) [],
        RX_CTRL             OFFSET(20) NUMBITS(1) [],
        RX_EMPTY            OFFSET(19) NUMBITS(1) [],
        RX_IGNORED          OFFSET(18) NUMBITS(1) [],
        RX_NOK              OFFSET(17) NUMBITS(1) [],
        RX_OK               OFFSET(16) NUMBITS(1) [],
        IRQ15               OFFSET(15) NUMBITS(1) [],
        IRQ14               OFFSET(14) NUMBITS(1) [],
        IRQ13               OFFSET(13) NUMBITS(1) [],
        IRQ12               OFFSET(12) NUMBITS(1) [],
        TX_BUFFER_CHANGED   OFFSET(11) NUMBITS(1) [],
        TX_ENTRY_DONE       OFFSET(10) NUMBITS(1) [],
        TX_RETRANS          OFFSET(9) NUMBITS(1) [],
        TX_CTRL_ACK_ACK     OFFSET(8) NUMBITS(1) [],
        TX_CTRL_ACK         OFFSET(7) NUMBITS(1) [],
        TX_CTRL             OFFSET(6) NUMBITS(1) [],
        TX_ACK              OFFSET(5) NUMBITS(1) [],
        TX_DONE             OFFSET(4) NUMBITS(1) [],
        LAST_FG_COMAND_DONE OFFSET(3) NUMBITS(1) [],
        FG_COMMAND_DONE     OFFSET(2) NUMBITS(1) [],
        LAST_COMMAND_DONE   OFFSET(1) NUMBITS(1) [],
        COMMAND_DONE        OFFSET(0) NUMBITS(1) []
    ],
    RFAckInterruptFlag [
        ACK OFFSET(0) NUMBITS(1) []
    ]
];

const RFC_DBELL_BASE: *mut RfcBellRegisters = 0x4004_1000 as *mut RfcBellRegisters;
const RFC_PWR_BASE: *mut VolatileCell<u32> = 0x4004_0000 as *mut VolatileCell<u32>;

/*
    RFC Immediate commands
*/
const RFC_CMD0: u16 = 0x607;
const RFC_PING: u16 = 0x406;
const RFC_BUS_REQUEST: u16 = 0x40E;
const RFC_START_RAT_TIMER: u16 = 0x080A;
const RFC_SETUP: u16 = 0x0802;

/*
    Power masks in order to enable certain clocks in the RFC
*/
const RFC_PWR_RFC: u32 = 0x01; // Main module
                               // Command and Packet Engine (CPE)
const RFC_PWR_CPE: u32 = 0x02;
const RFC_PWR_CPERAM: u32 = 0x04;
// Modem module
const RFC_PWR_MDM: u32 = 0x08;
const RFC_PWR_MDMRAM: u32 = 0x10;
// RF Engine (RFE)
const RFC_PWR_RFE: u32 = 0x20;
const RFC_PWR_RFERAM: u32 = 0x40;
// Radio Timer (RAT)
const RFC_PWR_RAT: u32 = 0x80;
// Packet Handling Accelerator (PHA)
const RFC_PWR_PHA: u32 = 0x100;
// Frequence Synthesizer Calibration Accelerator (FCSCA)
const RFC_PWR_FSCA: u32 = 0x200;

pub enum RfcResult {
    Ok,
    Error(u32),
}

pub enum RfcInterrupt {
    Cpe0,
    Cpe1,
    CmdAck,
    Hardware,
}

#[derive(PartialEq, Clone, Copy)]
pub enum RfcMode {
    BLE = 0x00,
    IEEE802154 = 0x01,
    Unchanged = 0xFF,
}

pub struct RFCore {
    bell_regs: *mut RfcBellRegisters,
    pwr_ctl: *mut VolatileCell<u32>,
    client: Cell<Option<&'static RFCoreClient>>,
    mode: Cell<Option<RfcMode>>,
}

/*
    RFCoreClient - Client to interface
    with protocol, to get callbacks when a command has been processed.
*/
pub trait RFCoreClient {
    fn command_done(&self);
}

impl RFCore {
    pub const fn new() -> RFCore {
        RFCore {
            bell_regs: RFC_DBELL_BASE,
            pwr_ctl: RFC_PWR_BASE,
            client: Cell::new(None),
            mode: Cell::new(None),
        }
    }

    pub fn is_enabled(&self) -> bool {
        prcm::Power::is_enabled(prcm::PowerDomain::RFC)
    }

    pub fn current_mode(&self) -> Option<RfcMode> {
        self.mode.get()
    }

    pub fn set_mode(&self, mode: RfcMode) {
        let rf_mode = match mode {
            RfcMode::BLE => 0x01,
            _ => panic!("No other mode than BLE is currently supported for RF!\r")
        };

        // Redirect power to the correct module
        prcm::rf_mode_sel(rf_mode);

        self.mode.set(Some(mode))
    }

    pub fn enable(&self) {
        // Enable power & clock
        prcm::Power::enable_domain(prcm::PowerDomain::RFC);
        prcm::Clock::enable_rfc();

        unsafe {
            rtc::RTC.set_upd_en(true);
        }

        // Wait for the power domain to be up
        while !prcm::Power::is_enabled(prcm::PowerDomain::RFC) {}

        let bell_regs: &RfcBellRegisters = unsafe { &*self.bell_regs };

        // Setup clocks and allow CPE to boot
        let pwr_ctl: &VolatileCell<u32> = unsafe { &*self.pwr_ctl };
        pwr_ctl.set(
            RFC_PWR_RFC | RFC_PWR_CPE | RFC_PWR_CPERAM | RFC_PWR_FSCA | RFC_PWR_PHA | RFC_PWR_RAT
                | RFC_PWR_RFE | RFC_PWR_RFERAM | RFC_PWR_MDM | RFC_PWR_MDMRAM,
        );

        bell_regs.rf_ack_interrupt_flag.set(0);

        // All interrupts to Cpe0 except INTERNAL_ERROR which is routed to Cpe1
        bell_regs.rf_cpe_interrupt_vector_sel.write(RFCpeInterrupts::INTERNAL_ERROR::SET);
        // Enable INTERNAL_ERROR and LOAD_DONE
        bell_regs.rf_cpe_interrupt_enable.write(
            RFCpeInterrupts::INTERNAL_ERROR::SET
                + RFCpeInterrupts::COMMAND_DONE::SET
                + RFCpeInterrupts::BOOT_DONE::SET
        );
        // Clear interrupt flags that might've been set by the init commands
        bell_regs.rf_cpe_interrupt_flags.set(0x00);

        self.send_ensure(&DirectCommand::new(RFC_CMD0, 0x10 | 0x40));

        // Request the bus
        self.send_ensure(&DirectCommand::new(RFC_BUS_REQUEST, 1));

        // Send a ping command to verify that the core is ready and alive
        self.send_ensure(&DirectCommand::new(RFC_PING, 0));
    }

    pub fn setup(&self, reg_override: u32) {
        let mode = self.mode
            .get()
            .unwrap_or_else(|| {
                panic!("No RF mode selected, can not setup.\r")
            });

        let setup_cmd = RfcCommandRadioSetup {
            command_no: RFC_SETUP,
            status: 0,
            p_nextop: 0,
            ratmr: 0,
            start_trigger: 0,
            condition: {
                let mut cond = RfcCondition(0);
                cond.set_rule(0x01); // COND_NEVER
                cond
            },
            mode: mode as u8,
            lo_divider: 0,
            config: {
                let mut cfg = RfcSetupConfig(0);
                cfg.set_frontend_mode(0); // Differential mode
                cfg.set_bias_mode(false); // Internal bias
                cfg
            },
            tx_power: 0x9330,
            reg_override: reg_override,
        };

        self.send_ensure(&setup_cmd);
    }

    pub fn start_rat(&self) {
        let cmd = RfcCommandStartRat {
            command_no: RFC_START_RAT_TIMER,
            status: 0,
            next_op: 0,
            start_time: 0,
            start_trigger: 0,
            condition: {
                let mut cond = RfcCondition(0);
                cond.set_rule(0x01); // COND_NEVER
                cond
            },
            _reserved: 0,
            rat0: 0,
        };

        self.send_ensure(&cmd);
    }

    pub fn send_ensure<C: RfcCommand>(&self, cmd: &C) {
        match self.send(cmd) {
            RfcResult::Error(status) => panic!("Error occurred during send_and_wait cmdsta=0x{:x}\r", status),
            RfcResult::Ok => (),
        }
    }

    pub fn send<C: RfcCommand>(&self, cmd: &C) -> RfcResult {
        let command: u32 = match cmd.command_type() {
            RfcCommandType::Direct => cmd.direct_command(),
            RfcCommandType::Immediate => cmd.immediate_command() as u32,
        };

        // Check if the radio is accessible or not
        if !prcm::Power::is_enabled(prcm::PowerDomain::RFC) {
            panic!("RFC power domain is off.\r");
        }

        let bell_regs: &RfcBellRegisters = unsafe { &*self.bell_regs };

        // CMDR is only writeable once it is zeroed
        while bell_regs.cmdr.get() != 0 {}

        // Set the command
        bell_regs.cmdr.set(command);

        // Wait for ACK from the radio MCU
        let mut timeout: u32 = 0;
        let mut status = 0;
        const MAX_TIMEOUT: u32 = 0x2FFFFFF;
        while timeout < MAX_TIMEOUT {
            status = bell_regs.cmdsta.get();
            if (status & 0xFF) == 0x01 {
                return RfcResult::Ok;
            }

            timeout += 1;
        }

        RfcResult::Error(status)
    }

    pub fn handle_interrupt(&self, int: RfcInterrupt) {
        let bell_regs: &RfcBellRegisters = unsafe { &*self.bell_regs };
        match int {
            RfcInterrupt::CmdAck => {
                // Clear the interrupt
                bell_regs.rf_ack_interrupt_flag.set(0);
            }
            RfcInterrupt::Cpe0 => {
                let rfcpeifg = bell_regs.rf_cpe_interrupt_flags.get();

                bell_regs.rf_cpe_interrupt_flags.set(0);

                if (rfcpeifg & 0x1) != 0 {
                    self.client.get().map(|client| client.command_done());
                }
            }
            RfcInterrupt::Cpe1 => {
                bell_regs.rf_cpe_interrupt_flags.set(0x7FFFFFFF);
                panic!("Internal occurred during radio command!\r");
            }
            _ => panic!("Unhandled RFC interrupt: {}\r", int as u8),
        }
    }

    pub fn set_client(&self, client: &'static RFCoreClient) {
        self.client.set(Some(client));
    }
}

pub mod rfc_commands {
    use radio::rfc::{RfcCommand, RfcCommandType};

    /* Basic direct command */
    pub struct DirectCommand {
        pub command_id: u16,
        pub parameters: u16,
    }

    impl DirectCommand {
        pub const fn new(command: u16, param: u16) -> DirectCommand {
            DirectCommand {
                command_id: command,
                parameters: param,
            }
        }
    }

    impl RfcCommand for DirectCommand {
        fn command_id(&self) -> &u16 {
            &self.command_id
        }
        fn command_status(&self) -> &u16 {
            &self.parameters
        }
        fn command_type(&self) -> RfcCommandType {
            RfcCommandType::Direct
        }
    }

    /* Basic immediate command */
    #[repr(C)]
    pub struct ImmediateCommand {
        // These fields below are always the first bytes in any rfc command
        // which is a radio operation or a immediate command.
        pub command_no: u16,
        pub status: u16,
        pub next_op: u32,
        pub start_time: u32,
        pub start_trigger: u8,
        pub condition: RfcCondition,
    }

    impl RfcCommand for ImmediateCommand {
        fn command_id(&self) -> &u16 {
            &self.command_no
        }
        fn command_status(&self) -> &u16 {
            &self.status
        }
        fn command_type(&self) -> RfcCommandType {
            RfcCommandType::Immediate
        }
    }

    /* In order to properly setup the radio mode (e.g BLE or IEEE) */
    #[repr(C)]
    pub struct RfcCommandRadioSetup {
        pub command_no: u16,
        pub status: u16,
        pub p_nextop: u32,
        pub ratmr: u32,
        pub start_trigger: u8,
        pub condition: RfcCondition,
        pub mode: u8,
        pub lo_divider: u8,
        pub config: RfcSetupConfig,
        pub tx_power: u16,
        pub reg_override: u32,
    }

    impl RfcCommand for RfcCommandRadioSetup {
        fn command_id(&self) -> &u16 {
            &self.command_no
        }
        fn command_status(&self) -> &u16 {
            &self.status
        }
        fn command_type(&self) -> RfcCommandType {
            RfcCommandType::Immediate
        }
    }

    #[repr(C)]
    pub struct RfcCommandStartRat {
        pub command_no: u16,
        pub status: u16,
        pub next_op: u32,
        pub start_time: u32,
        pub start_trigger: u8,
        pub condition: RfcCondition,
        pub _reserved: u16,
        pub rat0: u32,
    }

    impl RfcCommand for RfcCommandStartRat {
        fn command_id(&self) -> &u16 {
            &self.command_no
        }
        fn command_status(&self) -> &u16 {
            &self.status
        }
        fn command_type(&self) -> RfcCommandType {
            RfcCommandType::Immediate
        }
    }

    /* Bitfields used by many commands */
    bitfield!{
        #[derive(Copy, Clone)]
        pub struct RfcTrigger(u8);
        impl Debug;
        pub _trigger_type, _set_trigger_type  : 3, 0;
        pub _enable_cmd, _set_enable_cmd      : 4;
        pub _trigger_no, _set_trigger_no      : 6, 5;
        pub _past_trigger, _set_past_trigger  : 7;
    }

    bitfield!{
        #[derive(Copy, Clone)]
        pub struct RfcCondition(u8);
        impl Debug;
        pub _rule, set_rule : 3, 0;
        pub _skip, _set_skip : 7, 4;
    }

    bitfield!{
        #[derive(Copy, Clone)]
        pub struct RfcSetupConfig(u16);
        impl Debug;
        pub _frontend_mode, set_frontend_mode: 2, 0;
        pub _bias_mode, set_bias_mode: 3;
        pub _analog_cfg_mode, _set_analog_config_mode: 9, 4;
        pub _no_fs_powerup, _set_no_fs_powerup: 10;
    }
}
