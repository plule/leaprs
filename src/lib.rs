mod connection;
mod leap_rs;
pub use connection::*;
pub use leap_rs::*;

#[cfg(test)]
mod tests {
    use leap_sys::*;
    use std::mem;

    use crate::Connection;

    #[test]
    fn hello_leap() {
        let mut connection = Connection::try_new().expect("Failed to create connection");
        connection.open().expect("Failed to open the connection");
    }

    #[test]
    fn hello_leap_unsafe() {
        #[allow(nonstandard_style)]
        unsafe {
            let mut connection: LEAP_CONNECTION = mem::zeroed();
            let res = LeapCreateConnection(std::ptr::null(), &mut connection);
            assert_eq!(_eLeapRS_eLeapRS_Success, res, "LeapCreateConnection");

            let res = LeapOpenConnection(connection);
            assert_eq!(_eLeapRS_eLeapRS_Success, res, "LeapOpenConnection");
            let mut computed_array_size: u32 = 0;
            for _ in 0..=7 {
                let mut msg: LEAP_CONNECTION_MESSAGE = mem::zeroed();
                let return_code = LeapPollConnection(connection, 1000, &mut msg);

                if return_code == _eLeapRS_eLeapRS_NotConnected {
                    continue;
                }

                let return_code =
                    LeapGetDeviceList(connection, std::ptr::null_mut(), &mut computed_array_size);

                assert_eq!(return_code, _eLeapRS_eLeapRS_Success);

                println!("Number of devices available: {}", computed_array_size);

                if computed_array_size > 0 {
                    break;
                }
            }

            if computed_array_size > 0 {
                let mut leap_device_list: Vec<LEAP_DEVICE_REF> =
                    vec![mem::zeroed(); computed_array_size as usize];
                let res = LeapGetDeviceList(
                    connection,
                    leap_device_list.as_mut_ptr(),
                    &mut computed_array_size,
                );

                assert_eq!(res, _eLeapRS_eLeapRS_Success);

                for device in leap_device_list {
                    let id = device.id;
                    println!("id: {}", id);
                }
            }

            LeapCloseConnection(connection);
            LeapDestroyConnection(connection);
        }
    }
}
