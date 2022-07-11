use std::mem;

use leap_sys::*;

use crate::{leap_try, ConnectionMessage, Error};

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

    pub fn poll<'a>(&'a mut self, timeout: u32) -> Result<&'a ConnectionMessage, Error> {
        unsafe {
            let mut msg: LEAP_CONNECTION_MESSAGE = mem::zeroed();
            leap_try(LeapPollConnection(self.handle, timeout, &mut msg))?;
            self.connection_message = Some(msg.into());
        }

        Ok(self.connection_message.as_ref().unwrap())
    }

    pub fn get_device_list(&mut self, count: u32) -> Result<Vec<LEAP_DEVICE_REF>, Error> {
        unsafe {
            let mut devices = vec![std::mem::zeroed(); count as usize];
            let mut received = count;
            leap_try(LeapGetDeviceList(
                self.handle,
                devices.as_mut_ptr(),
                &mut received,
            ))?;
            devices.truncate(received as usize);
            Ok(devices)
        }
    }

    pub fn get_device_list_count(&mut self) -> Result<u32, Error> {
        let mut computed_array_size: u32 = 0;
        unsafe {
            leap_try(LeapGetDeviceList(
                self.handle,
                std::ptr::null_mut(),
                &mut computed_array_size,
            ))?;
        }
        Ok(computed_array_size)
    }
}
