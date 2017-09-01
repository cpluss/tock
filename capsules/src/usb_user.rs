//! USB system call interface
//!
//! This capsule provides a system call interface to the USB controller.
//!
//! ## Instantiation
//!
//! The `UsbSyscallDriver` must be created by passing a reference to something
//! that implements `hil::usb::Client` (that is, something that is connected to
//! the USBC), as well as a `Container` for managing application requests.  For
//! example:
//!
//! ```rust
//! // Configure the USB controller
//! let usb_client = static_init!(
//!     capsules::usbc_client::Client<'static, sam4l::usbc::Usbc<'static>>,
//!     capsules::usbc_client::Client::new(&sam4l::usbc::USBC));
//! sam4l::usbc::USBC.set_client(usb_client);
//!
//! // Configure the USB userspace driver
//! let usb_driver = static_init!(
//!     capsules::usb_user::UsbSyscallDriver<'static,
//!         capsules::usbc_client::Client<'static, sam4l::usbc::Usbc<'static>>>,
//!     capsules::usb_user::UsbSyscallDriver::new(
//!         usb_client, kernel::Container::create()));
//! ```

use core::cell::Cell;
use kernel::{AppId, Container, Callback, Driver, ReturnCode};
use kernel::hil;

#[derive(Default)]
pub struct App {
    callback: Option<Callback>,
    awaiting: Option<Request>,
}

pub struct UsbSyscallDriver<'a, C: hil::usb::Client + 'a> {
    usbc_client: &'a C,
    apps: Container<App>,
    serving_app: Cell<Option<AppId>>,
}

impl<'a, C> UsbSyscallDriver<'a, C>
    where C: hil::usb::Client
{
    pub fn new(usbc_client: &'a C, apps: Container<App>) -> Self {
        UsbSyscallDriver {
            usbc_client: usbc_client,
            apps: apps,
            serving_app: Cell::new(None),
        }
    }

    fn serve_waiting_apps(&self) {
        if self.serving_app.get().is_some() {
            // An operation on the USBC client is in progress
            return;
        }

        // Find a waiting app and start its requested computation
        let mut found = false;
        for app in self.apps.iter() {
            app.enter(|app, _| {
                if let Some(request) = app.awaiting {
                    found = true;
                    match request {
                        Request::EnableAndAttach => {
                            // Enable and attach (synchronously)
                            self.usbc_client.enable();
                            self.usbc_client.attach();

                            // Schedule a callback immediately
                            if let Some(mut callback) = app.callback {
                                callback.schedule(From::from(ReturnCode::SUCCESS), 0, 0);
                            }
                            app.awaiting = None;
                        }
                    }
                }
            });
            if found {
                break;
            }
        }

        if !found {
            // No userspace requests pending at this time
        }
    }
}

#[derive(Copy, Clone)]
enum Request {
    EnableAndAttach,
}

impl<'a, C> Driver for UsbSyscallDriver<'a, C>
    where C: hil::usb::Client
{
    fn subscribe(&self, subscribe_num: usize, callback: Callback) -> ReturnCode {
        match subscribe_num {
            // Set callback for result
            0 => {
                self.apps
                    .enter(callback.app_id(), |app, _| {
                        app.callback = Some(callback);
                        ReturnCode::SUCCESS
                    })
                    .unwrap_or_else(|err| err.into())
            }
            _ => ReturnCode::ENOSUPPORT,
        }
    }

    fn command(&self, command_num: usize, _arg: usize, appid: AppId) -> ReturnCode {
        match command_num {
            // This driver is present
            0 => ReturnCode::SUCCESS,

            // Enable USB controller, attach to bus, and service default control endpoint
            1 => {
                let result = self.apps
                    .enter(appid, |app, _| {
                        if app.awaiting.is_some() {
                            // Each app may make only one request at a time
                            ReturnCode::EBUSY
                        } else {
                            if app.callback.is_some() {
                                app.awaiting = Some(Request::EnableAndAttach);
                                ReturnCode::SUCCESS
                            } else {
                                ReturnCode::EINVAL
                            }
                        }
                    })
                    .unwrap_or_else(|err| err.into());

                if result == ReturnCode::SUCCESS {
                    self.serve_waiting_apps();
                }
                result
            }

            _ => ReturnCode::ENOSUPPORT,
        }
    }
}
