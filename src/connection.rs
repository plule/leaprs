use std::{ffi::CString, mem};

use leap_sys::*;

use crate::*;

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
            LeapDestroyConnection(self.handle);
        }
    }
}

impl Connection {
    #[doc = " Creates a new LEAP_CONNECTION object."]
    #[doc = ""]
    #[doc = " Pass the LEAP_CONNECTION pointer to LeapOpenConnection() to establish a"]
    #[doc = " connection to the Ultraleap Tracking Service; and to subsequent operations"]
    #[doc = " on the same connection."]
    #[doc = ""]
    #[doc = " @param pConfig The configuration to be used with the newly created connection."]
    #[doc = " If pConfig is null, a connection is created with a default configuration."]
    #[doc = " @param[out] phConnection Receives a pointer to the connection object, set to invalid on failure"]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn create(config: ConnectionConfig) -> Result<Self, Error> {
        let mut leap_connection: LEAP_CONNECTION;
        unsafe {
            leap_connection = mem::zeroed();
            leap_try(LeapCreateConnection(&config.handle, &mut leap_connection))?;
        }

        Ok(Self {
            handle: leap_connection,
            connection_message: None,
        })
    }

    #[doc = " Opens a connection to the service."]
    #[doc = ""]
    #[doc = " This routine will not block. A connection to the service will not be established until the first"]
    #[doc = " invocation of LeapPollConnection."]
    #[doc = ""]
    #[doc = " @param hConnection A handle to the connection object, created by LeapCreateConnection()."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn open(&mut self) -> Result<(), Error> {
        unsafe {
            leap_try(LeapOpenConnection(self.handle))?;
        }

        Ok(())
    }

    #[doc = " Polls the connection for a new event."]
    #[doc = ""]
    #[doc = " The specific types of events that may be received are not configurable in this entrypoint. Configure"]
    #[doc = " the device or connection object directly to change what events will be received."]
    #[doc = ""]
    #[doc = " Pointers in the retrieved event message structure will be valid until the associated connection or device is"]
    #[doc = " closed, or the next call to LeapPollConnection()."]
    #[doc = ""]
    #[doc = " Calling this method concurrently will return eLeapRS_ConcurrentPoll."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param timeout The maximum amount of time to wait, in milliseconds. If this value is zero,"]
    #[doc = " the evt pointer references the next queued message, if there is one, and returns immediately."]
    #[doc = " @param[out] evt A pointer to a structure that is filled with event information. This structure will be valid"]
    #[doc = " as long as the LEAP_CONNECTION object is valid."]
    #[doc = ""]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration. If the operation"]
    #[doc = " times out, this method will return eLeapRS_Timeout. The evt pointer will reference a"]
    #[doc = " message of type eLeapEventType_None."]
    #[doc = " @since 3.0.0"]
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

    #[doc = " Retrieves a list of Ultraleap Tracking camera devices currently attached to the system."]
    #[doc = ""]
    #[doc = " To get the number of connected devices, call this function with the pArray parameter"]
    #[doc = " set to null. The number of devices is written to the memory specified by pnArray."]
    #[doc = " Use the device count to create an array of LEAP_DEVICE_REF structs large enough to"]
    #[doc = " hold the number of connected devices. Finally, call LeapGetDeviceList() with this"]
    #[doc = " array and known count to get the list of Leap devices. A device must be opened with"]
    #[doc = " LeapOpenDevice() before device properties can be queried."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param[out] pArray A pointer to an array that LeapC fills with the device list."]
    #[doc = " @param[in,out] pnArray On input, set to the number of elements in pArray; on output,"]
    #[doc = " LeapC sets this to the number of valid device handles."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn get_device_list_raw(&mut self, count: &mut u32) -> Result<Vec<DeviceRef>, Error> {
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
        Ok(devices.into_iter().map(|r| r.into()).collect())
    }

    #[doc = " Retrieves a list of Ultraleap Tracking camera devices currently attached to the system."]
    pub fn get_device_list(&mut self) -> Result<Vec<DeviceRef>, Error> {
        let mut count = 0;
        // First call to get the number of devices
        let _ = self.get_device_list_raw(&mut count)?;
        // Second call to get the list of devices
        self.get_device_list_raw(&mut count)
    }

    #[doc = " Returns the version of a specified part of the system."]
    #[doc = ""]
    #[doc = " If an invalid connection handle is provided only the version details of the client will be available."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param versionPart The version part to return, this will reference one part of the system."]
    #[doc = " @param[out] pVersion A pointer to a struct used to store the version number."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.2.x"]
    pub fn get_version(&mut self, part: VersionPart) -> Result<Version, Error> {
        let mut version: LEAP_VERSION;
        unsafe {
            version = std::mem::zeroed();
            leap_try(LeapGetVersion(self.handle, part.into(), &mut version))?;
        }
        Ok(version.into())
    }

    #[doc = " Sets or clears one or more policy flags."]
    #[doc = ""]
    #[doc = " Changing policies is asynchronous. After you call this function, a subsequent"]
    #[doc = " call to LeapPollConnection provides a LEAP_POLICY_EVENT containing the current"]
    #[doc = " policies, reflecting any changes."]
    #[doc = ""]
    #[doc = " To get the current policies without changes, specify zero for both the set"]
    #[doc = " and clear parameters. When ready, LeapPollConnection() provides a LEAP_POLICY_EVENT"]
    #[doc = " containing the current settings."]
    #[doc = ""]
    #[doc = " The eLeapPolicyFlag enumeration defines the policy flags."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param set A bitwise combination of flags to be set. Set to 0 if not setting any flags."]
    #[doc = " @param clear A bitwise combination of flags to be cleared. Set to 0 to if not clearing any flags."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn set_policy_flags(&mut self, set: PolicyFlags, clear: PolicyFlags) -> Result<(), Error> {
        unsafe { leap_try(LeapSetPolicyFlags(self.handle, set.bits(), clear.bits())) }
    }

    #[doc = " Requests a tracking mode."]
    #[doc = ""]
    #[doc = " Changing tracking modes is asynchronous. After you call this function, a subsequent"]
    #[doc = " call to LeapPollConnection provides a LEAP_POLICY_EVENT containing the current"]
    #[doc = " policies, reflecting any changes."]
    #[doc = ""]
    #[doc = " The eLeapTrackingMode enumeration defines the tracking mode."]
    #[doc = "."]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param mode The enum value specifying the requested tracking mode"]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.0.0"]
    pub fn set_tracking_mode(&mut self, mode: TrackingMode) -> Result<(), Error> {
        unsafe { leap_try(LeapSetTrackingMode(self.handle, mode.into())) }
    }

    #[doc = "**Warning**: Does not appear to work."]
    #[doc = ""]
    #[doc = " Causes the client to commit a configuration change to the Ultraleap Tracking Service."]
    #[doc = ""]
    #[doc = " The change is performed asynchronously -- and may fail. LeapPollConnection()"]
    #[doc = " returns this event structure when the request has been processed. Use the pRequestID"]
    #[doc = " value to correlate the response to the originating request."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param key The key of the configuration to commit."]
    #[doc = " @param value The value of the configuration to commit."]
    #[doc = " @param[out] pRequestID A pointer to a memory location to which the id for this request is written, or nullptr if this value is not needed."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn save_config_value(
        &mut self,
        key: CString,
        value: Variant,
        request_id: Option<&mut u32>,
    ) -> Result<(), Error> {
        let request_id: *mut u32 = match request_id {
            Some(id) => id,
            None => std::ptr::null_mut(),
        };
        let value: LEAP_VARIANT = value.into();
        unsafe {
            leap_try(LeapSaveConfigValue(
                self.handle,
                key.as_ptr(),
                &value,
                request_id,
            ))?;
        }
        Ok(())
    }

    #[doc = "**Warning**: Does not appear to work."]
    #[doc = ""]
    #[doc = " Requests the current value of a service configuration setting."]
    #[doc = " The value is fetched asynchronously since it requires a service transaction. LeapPollConnection()"]
    #[doc = " returns this event structure when the request has been processed. Use the pRequestID"]
    #[doc = " value to correlate the response to the originating request."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param key The key of the configuration to request"]
    #[doc = " @param[out] pRequestID A pointer to a memory location to which the id for this request is written."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    pub fn request_config_value(
        &mut self,
        key: CString,
        request_id: Option<&mut u32>,
    ) -> Result<(), Error> {
        let request_id: *mut u32 = match request_id {
            Some(id) => id,
            None => std::ptr::null_mut(),
        };
        unsafe {
            leap_try(LeapRequestConfigValue(
                self.handle,
                key.as_ptr(),
                request_id,
            ))?;
        }
        Ok(())
    }

    #[doc = " Retrieves the number of bytes required to allocate an interpolated frame at the specified time."]
    #[doc = ""]
    #[doc = " Use this function to determine the size of the buffer to allocate when calling"]
    #[doc = " LeapInterpolateFrame()."]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param timestamp The timestamp of the frame whose size is to be queried."]
    #[doc = " @param[out] pncbEvent A pointer that receives the number of bytes required to store the specified frame."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.1.1"]
    pub fn get_frame_size(&mut self, timestamp: i64) -> Result<u64, Error> {
        let mut result: u64 = 0;
        unsafe {
            leap_try(LeapGetFrameSize(self.handle, timestamp, &mut result))?;
        }
        Ok(result)
    }

    #[doc = " \\ingroup Functions"]
    #[doc = " Constructs a frame at the specified timestamp by interpolating between measured"]
    #[doc = " frames."]
    #[doc = ""]
    #[doc = " Caller is responsible for allocating a buffer large enough to hold the data of the frame."]
    #[doc = " Use LeapGetFrameSize() to calculate the minimum size of this buffer."]
    #[doc = ""]
    #[doc = " Use LeapCreateClockRebaser(), LeapUpdateRebase(), and LeapRebaseClock() to"]
    #[doc = " synchronize time measurements in the application with time measurements in"]
    #[doc = " the Ultraleap Tracking Service. This process is required to achieve accurate, smooth"]
    #[doc = " interpolation."]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param timestamp The timestamp at which to interpolate the frame data."]
    #[doc = " @param[out] pEvent A pointer to a flat buffer which is filled with an interpolated frame."]
    #[doc = " @param ncbEvent The number of bytes pointed to by pEvent."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.1.1"]
    pub fn interpolate_frame(
        &mut self,
        timestamp: i64,
        requested_frame_size: u64,
    ) -> Result<InterpolationTrackingEvent, Error> {
        // LEAP_TRACKING_EVENT with more size to account for the dynamic frame data.
        unsafe {
            let mut event = InterpolationTrackingEvent::new_uninitialized(requested_frame_size);

            leap_try(LeapInterpolateFrame(
                self.handle,
                timestamp,
                &mut event.handle.sized,
                event.handle.size() as u64,
            ))?;
            Ok(event)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::time::{Duration, SystemTime};

    use crate::tests::*;
    use crate::*;

    #[test]
    fn get_device_list() {
        let devices = initialize_connection()
            .get_device_list()
            .expect("Failed to list devices");
        assert!(devices.len() > 0, "No devices, tests will not run.");
    }

    #[test]
    fn get_version() {
        let mut connection = initialize_connection();
        connection
            .get_version(VersionPart::ClientLibrary)
            .expect("Failed to get client library version");
        connection
            .get_version(VersionPart::ClientProtocol)
            .expect("Failed to get client protocol version");
        connection
            .get_version(VersionPart::ServerLibrary)
            .expect("Failed to get server library version");
        connection
            .get_version(VersionPart::ServerProtocol)
            .expect("Failed to get server protocol version");
    }

    #[test]
    fn config_value() {
        let mut connection = initialize_connection();
        let mut request_id: u32 = 0;
        let config_key = CString::new("fake_config").unwrap();
        connection
            .request_config_value(config_key.clone(), Some(&mut request_id))
            .expect("Failed to request the config value");
        // Note: No response event are received: useless?

        connection
            .save_config_value(config_key.clone(), true.into(), Some(&mut request_id))
            .expect("Failed to save a config value");
        connection
            .save_config_value(config_key.clone(), 0.into(), Some(&mut request_id))
            .expect("Failed to save a config value");
        connection
            .save_config_value(config_key.clone(), 0.0.into(), Some(&mut request_id))
            .expect("Failed to save a config value");
        connection
            .save_config_value(
                config_key.clone(),
                CString::new("hello").unwrap().into(),
                Some(&mut request_id),
            )
            .expect("Failed to save a config value");
    }

    #[test]
    #[ignore = "Does not work"]
    fn set_config_value() {
        let mut connection = initialize_connection();
        let mut request_id: u32 = 0;
        let config_key = CString::new("robust_mode_enabled").unwrap();
        connection
            .request_config_value(config_key, Some(&mut request_id))
            .expect("Failed to request the config value");
        connection.expect_event("Did not receive the config".to_string(), |e| match e {
            Event::ConfigResponse(c) => {
                if c.request_id() != request_id {
                    None
                } else {
                    if let Variant::Boolean(robust_mode_enabled) = c.value() {
                        Some(robust_mode_enabled)
                    } else {
                        panic!("Type mismatch for the configuration value.")
                    }
                }
            }
            _ => None,
        });
    }

    #[test]
    fn frame_interpolation() {
        let start = SystemTime::now();

        let mut connection = initialize_connection();
        let mut clock_synchronizer = ClockRebaser::create().expect("Failed to create clock sync");

        for _ in 0..10 {
            // Note: If events are not polled, the frame interpolation fails with "is not seer"
            connection.expect_event("no tracking data".to_string(), |e| match e {
                Event::Tracking(_) => Some(()),
                _ => None,
            });
            let cpu_time = SystemTime::now().duration_since(start).unwrap().as_micros() as i64;
            clock_synchronizer
                .update_rebase(cpu_time, leap_get_now())
                .expect("Failed to update rebase");

            std::thread::sleep(Duration::from_millis(10));

            let cpu_time = SystemTime::now().duration_since(start).unwrap().as_micros() as i64;

            let target_frame_time = clock_synchronizer
                .rebase_clock(cpu_time)
                .expect("Failed to rebase clock");

            let requested_size = connection
                .get_frame_size(target_frame_time)
                .expect("Failed to get requested size");

            let frame = connection
                .interpolate_frame(target_frame_time, requested_size)
                .expect("Failed to interpolate frame");
            let _hands = frame.hands();
        }
    }
}
