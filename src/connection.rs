use std::mem;

use leap_sys::*;

use crate::{leap_try, DeviceRef, Error};

#[doc = " \\ingroup Structs"]
#[doc = " \\struct LEAP_CONNECTION"]
#[doc = " A handle to the Leap connection object."]
#[doc = " Use this handle to specify the connection for an operation."]
#[doc = " @since 3.0.0"]
pub struct Connection {
    leap_connection: LEAP_CONNECTION,
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            LeapCloseConnection(self.leap_connection);
        }
    }
}

impl Connection {
    pub fn create() -> Result<Self, Error> {
        let mut leap_connection: LEAP_CONNECTION;
        unsafe {
            leap_connection = mem::zeroed();
            leap_try(LeapCreateConnection(std::ptr::null(), &mut leap_connection))?;
        }

        Ok(Self { leap_connection })
    }

    pub fn open(&mut self) -> Result<(), Error> {
        unsafe {
            leap_try(LeapOpenConnection(self.leap_connection))?;
        }

        Ok(())
    }

    pub fn close(&mut self) {
        unsafe { LeapCloseConnection(self.leap_connection) }
    }

    pub fn poll(&mut self, timeout: u32) -> Result<(), Error> {
        unsafe {
            let mut msg: LEAP_CONNECTION_MESSAGE = mem::zeroed();
            leap_try(LeapPollConnection(self.leap_connection, timeout, &mut msg))?;
        }

        Ok(())
    }

    pub fn get_device_list2(&mut self) -> Result<Vec<DeviceRef>, Error> {
        let mut computed_array_size: u32 = 0;
        unsafe {
            leap_try(LeapGetDeviceList(
                self.leap_connection,
                std::ptr::null_mut(),
                &mut computed_array_size,
            ))?;
        }

        if computed_array_size == 0 {
            return Ok(vec![]);
        }

        let mut devices: Vec<LEAP_DEVICE_REF>;
        unsafe {
            devices = vec![std::mem::zeroed(); computed_array_size as usize];
            leap_try(LeapGetDeviceList(
                self.leap_connection,
                devices.as_mut_ptr(),
                &mut computed_array_size,
            ))?;
        }

        let devices: Vec<DeviceRef> = devices.into_iter().map(|d| d.into()).collect();

        Ok(devices)
    }

    pub fn get_device_list_count(&mut self) -> Result<u32, Error> {
        let mut computed_array_size: u32 = 0;
        unsafe {
            leap_try(LeapGetDeviceList(
                self.leap_connection,
                std::ptr::null_mut(),
                &mut computed_array_size,
            ))?;
        }
        Ok(computed_array_size)
    }

    pub fn get_device_list(&mut self, count: u32) -> Result<Vec<DeviceRef>, Error> {
        let mut received = count;
        let mut devices: Vec<LEAP_DEVICE_REF>;
        unsafe {
            devices = vec![std::mem::zeroed(); count as usize];
            leap_try(LeapGetDeviceList(
                self.leap_connection,
                devices.as_mut_ptr(),
                &mut received,
            ))?;
        }
        let devices: Vec<DeviceRef> = devices
            .into_iter()
            .take(received as usize)
            .map(|d| d.into())
            .collect();
        Ok(devices)
    }
}
