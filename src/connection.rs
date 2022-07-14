use std::mem;

use leap_sys::*;

use crate::{leap_try, ConnectionMessage, Error, PolicyFlags, VersionPart};

#[doc = " \\ingroup Structs"]
#[doc = " \\struct LEAP_CONNECTION"]
#[doc = " A handle to the Leap connection object."]
#[doc = " Use this handle to specify the connection for an operation."]
#[doc = " @since 3.0.0"]
pub struct Connection {
    handle: LEAP_CONNECTION,
    // Each call to call() invalidates the connection message pointer,
    // and it is distroy on the connection drop.
    // Only distribute non mutable references of this one.
    connection_message: Option<ConnectionMessage>,
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            LeapCloseConnection(self.handle);
        }
    }
}

unsafe impl Send for Connection {}

impl Connection {
    pub fn create() -> Result<Self, Error> {
        let mut leap_connection: LEAP_CONNECTION;
        unsafe {
            leap_connection = mem::zeroed();
            leap_try(LeapCreateConnection(std::ptr::null(), &mut leap_connection))?;
        }

        Ok(Self {
            handle: leap_connection,
            connection_message: None,
        })
    }

    pub fn open(&mut self) -> Result<(), Error> {
        unsafe {
            leap_try(LeapOpenConnection(self.handle))?;
        }

        Ok(())
    }

    pub fn close(&mut self) {
        unsafe { LeapCloseConnection(self.handle) }
    }

    pub fn poll(&mut self, timeout: u32) -> Result<&ConnectionMessage, Error> {
        // The code after will invalidate it.
        self.connection_message = None;
        let mut msg: LEAP_CONNECTION_MESSAGE;
        unsafe {
            msg = mem::zeroed();
            leap_try(LeapPollConnection(self.handle, timeout, &mut msg))?;
        }
        self.connection_message = Some(msg.into());

        Ok(self.connection_message.as_ref().unwrap())
    }

    pub fn get_device_list_raw(&mut self, count: &mut u32) -> Result<Vec<LEAP_DEVICE_REF>, Error> {
        let mut devices;
        unsafe {
            // Initialize enough space for the devices
            devices = vec![std::mem::zeroed(); *count as usize];
            let devices_ptr = if *count > 0 {
                devices.as_mut_ptr()
            } else {
                std::ptr::null_mut()
            };
            // Attempt to fill with the devices
            leap_try(LeapGetDeviceList(self.handle, devices_ptr, count))?;

            // Truncate the null devices if less than asked were received.
            devices.truncate(*count as usize);
        }
        Ok(devices)
    }

    pub fn get_device_list(&mut self) -> Result<Vec<LEAP_DEVICE_REF>, Error> {
        let mut count = 0;
        // First call to get the number of devices
        let _ = self.get_device_list_raw(&mut count)?;
        // Second call to get the list of devices
        self.get_device_list_raw(&mut count)
    }

    pub fn get_version(&mut self, part: VersionPart) -> Result<LEAP_VERSION, Error> {
        let mut version: LEAP_VERSION;
        unsafe {
            version = std::mem::zeroed();
            leap_try(LeapGetVersion(self.handle, part.into(), &mut version))?;
        }
        Ok(version)
    }

    pub fn set_policy_flags(&mut self, set: PolicyFlags, clear: PolicyFlags) -> Result<(), Error> {
        unsafe { leap_try(LeapSetPolicyFlags(self.handle, set.bits(), clear.bits())) }
    }
}
