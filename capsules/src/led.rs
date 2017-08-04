//! Provides userspace access to LEDs on a board.
//!
//! This allows for much more cross platform controlling of LEDs without having
//! to know which of the GPIO pins exposed across the syscall interface are
//! LEDs.
//!
//! Usage
//! -----
//!
//! ```rust
//! let led_pins = static_init!(
//!     [(&'static sam4l::gpio::GPIOPin, capsules::led::ActivationMode); 3],
//!     [(&sam4l::gpio::PA[13], capsules::led::ActivationMode::ActiveLow),   // Red
//!      (&sam4l::gpio::PA[15], capsules::led::ActivationMode::ActiveLow),   // Green
//!      (&sam4l::gpio::PA[14], capsules::led::ActivationMode::ActiveLow)]); // Blue
//! let led = static_init!(
//!     capsules::led::LED<'static, sam4l::gpio::GPIOPin>,
//!     capsules::led::LED::new(led_pins));
//! ```
//!
//! Syscall Interface
//! -----------------
//!
//! - Stability: 2 - Stable
//!
//! ### Command
//!
//! All LED operations are synchronous, so this capsule only uses the `command`
//! syscall.
//!
//! #### `command_num`
//!
//! - `0`: Return the number of LEDs on this platform.
//!   - `data`: Unused.
//!   - Return: Number of LEDs.
//! - `1`: Turn the LED on.
//!   - `data`: The index of the LED. Starts at 0.
//!   - Return: `SUCCESS` if the LED index was valid, `EINVAL` otherwise.
//! - `2`: Turn the LED off.
//!   - `data`: The index of the LED. Starts at 0.
//!   - Return: `SUCCESS` if the LED index was valid, `EINVAL` otherwise.
//! - `3`: Toggle the on/off state of the LED.
//!   - `data`: The index of the LED. Starts at 0.
//!   - Return: `SUCCESS` if the LED index was valid, `EINVAL` otherwise.

use kernel::{AppId, Driver, ReturnCode};
use kernel::hil;

/// Syscall driver number.
pub const DRIVER_NUM: usize = 0x00000002;

/// Whether the LEDs are active high or active low on this platform.
#[derive(Clone,Copy)]
pub enum ActivationMode {
    ActiveHigh,
    ActiveLow,
}

/// Holds the array of GPIO pins attached to the LEDs.
pub struct LED<'a, G: hil::gpio::Pin + 'a> {
    pins_init: &'a [(&'a G, ActivationMode)],
}

impl<'a, G: hil::gpio::Pin + hil::gpio::PinCtl> LED<'a, G> {
    pub fn new(pins_init: &'a [(&'a G, ActivationMode)]) -> LED<'a, G> {
        // Make all pins output and off
        for &(pin, mode) in pins_init.as_ref().iter() {
            pin.make_output();
            match mode {
                ActivationMode::ActiveHigh => pin.clear(),
                ActivationMode::ActiveLow => pin.set(),
            }
        }

        LED { pins_init: pins_init }
    }
}

impl<'a, G: hil::gpio::Pin + hil::gpio::PinCtl> Driver for LED<'a, G> {
    /// Control the LEDs
    ///
    /// ### `command_num`
    ///
    /// - `0`: Get number of LEDs.
    /// - `1`: Turn LED on.
    /// - `2`: Turn LED off.
    /// - `3`: Toggle an LED.
    fn command(&self, command_num: usize, data: usize, _: AppId) -> ReturnCode {
        let pins_init = self.pins_init.as_ref();
        match command_num {
            // get number of LEDs
            0 => ReturnCode::SuccessWithValue { value: pins_init.len() as usize },

            // on
            1 => {
                if data >= pins_init.len() {
                    ReturnCode::EINVAL /* impossible pin */
                } else {
                    let (pin, mode) = pins_init[data];
                    match mode {
                        ActivationMode::ActiveHigh => pin.set(),
                        ActivationMode::ActiveLow => pin.clear(),
                    }
                    ReturnCode::SUCCESS
                }
            }

            // off
            2 => {
                if data >= pins_init.len() {
                    ReturnCode::EINVAL /* impossible pin */
                } else {
                    let (pin, mode) = pins_init[data];
                    match mode {
                        ActivationMode::ActiveHigh => pin.clear(),
                        ActivationMode::ActiveLow => pin.set(),
                    }
                    ReturnCode::SUCCESS
                }
            }

            // toggle
            3 => {
                if data >= pins_init.len() {
                    ReturnCode::EINVAL /* impossible pin */
                } else {
                    let (pin, _) = pins_init[data];
                    pin.toggle();
                    ReturnCode::SUCCESS
                }
            }

            // default
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}
