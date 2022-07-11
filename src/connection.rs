use std::mem;

use leap_sys::{LeapCloseConnection, LeapCreateConnection, LeapOpenConnection, LEAP_CONNECTION};

use crate::LeapRS;

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
    pub fn try_new() -> Result<Self, LeapRS> {
        let res: LeapRS;
        let mut leap_connection: LEAP_CONNECTION;
        unsafe {
            leap_connection = mem::zeroed();
            res = LeapCreateConnection(std::ptr::null(), &mut leap_connection).into();
        }

        if res != LeapRS::Success {
            return Err(res);
        }

        Ok(Self { leap_connection })
    }

    pub fn open(&mut self) -> Result<(), LeapRS> {
        let res: LeapRS;
        unsafe {
            res = LeapOpenConnection(self.leap_connection).into();
        }

        if res != LeapRS::Success {
            return Err(res);
        }

        Ok(())
    }
}
