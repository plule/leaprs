use std::{ffi::CString, mem};

use ::leap_sys::*;

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

    pub fn info(&mut self) -> Result<ConnectionInfo, Error> {
        let mut info = LEAP_CONNECTION_INFO {
            size: std::mem::size_of::<LEAP_CONNECTION_INFO>() as u32,
            status: 0,
        };
        unsafe {
            leap_try(LeapGetConnectionInfo(self.handle, &mut info))?;
        }

        Ok(ConnectionInfo(info))
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
        self.connection_message = Some(ConnectionMessage(msg));

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
        Ok(devices.into_iter().map(DeviceRef).collect())
    }

    #[doc = " Retrieves a list of Ultraleap Tracking camera devices currently attached to the system."]
    pub fn get_device_list(&mut self) -> Result<Vec<DeviceRef>, Error> {
        let mut count = 0;
        // First call to get the number of devices
        let _ = self.get_device_list_raw(&mut count)?;
        // Second call to get the list of devices
        self.get_device_list_raw(&mut count)
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

    #[doc = " Pauses the service"]
    #[doc = ""]
    #[doc = " Attempts to pause or unpause the service depending on the argument."]
    #[doc = " This is treated as a 'user pause', as though a user had requested a pause through the"]
    #[doc = " Leap Control Panel. The connection must have the AllowPauseResume policy set"]
    #[doc = " or it will fail with eLeapRS_InvalidArgument."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param pause Set to 'true' to pause, or 'false' to unpause."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 4.0.0"]
    pub fn set_pause(&mut self, pause: bool) -> Result<(), Error> {
        unsafe {
            leap_try(LeapSetPause(self.handle, pause))?;
        }
        Ok(())
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
                &mut event.0.sized,
                event.0.size() as u64,
            ))?;
            Ok(event)
        }
    }

    #[doc = " Provides the corrected camera ray intercepting the specified point on the image."]
    #[doc = ""]
    #[doc = " Given a point on the image, ``LeapPixelToRectilinear()`` corrects for camera distortion"]
    #[doc = " and returns the true direction from the camera to the source of that image point"]
    #[doc = " within the Ultraleap Tracking camera field of view."]
    #[doc = ""]
    #[doc = " This direction vector has an x and y component [x, y, 1], with the third element"]
    #[doc = " always 1. Note that this vector uses the 2D camera coordinate system"]
    #[doc = " where the x-axis parallels the longer (typically horizontal) dimension and"]
    #[doc = " the y-axis parallels the shorter (vertical) dimension. The camera coordinate"]
    #[doc = " system does not correlate to the 3D Ultraleap Tracking coordinate system."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param camera The camera to use, a member of the eLeapPerspectiveType enumeration"]
    #[doc = " @param pixel A Vector containing the position of a pixel in the image."]
    #[doc = " @returns A Vector containing the ray direction (the z-component of the vector is always 1)."]
    #[doc = " @since 3.1.3"]
    pub fn pixel_to_rectilinear<P: Into<[f32; 3]>>(
        &mut self,
        camera: PerspectiveType,
        pixel: P,
    ) -> [f32; 3] {
        unsafe {
            let leap_vector =
                LeapPixelToRectilinear(self.handle, camera.into(), build_leap_vector(pixel.into()));
            leap_vector.__bindgen_anon_1.v
        }
    }

    #[doc = " \\ingroup Functions"]
    #[doc = " Provides the point in the image corresponding to a ray projecting"]
    #[doc = " from the camera."]
    #[doc = ""]
    #[doc = " Given a ray projected from the camera in the specified direction, ``LeapRectilinearToPixel()``"]
    #[doc = " corrects for camera distortion and returns the corresponding pixel"]
    #[doc = " coordinates in the image."]
    #[doc = ""]
    #[doc = " The ray direction is specified in relationship to the camera. The first"]
    #[doc = " vector element is the tangent of the \"horizontal\" view angle; the second"]
    #[doc = " element is the tangent of the \"vertical\" view angle."]
    #[doc = ""]
    #[doc = " The ``LeapRectilinearToPixel()`` function returns pixel coordinates outside of the image bounds"]
    #[doc = " if you project a ray toward a point for which there is no recorded data."]
    #[doc = ""]
    #[doc = " ``LeapRectilinearToPixel()`` is typically not fast enough for realtime distortion correction."]
    #[doc = " For better performance, use a shader program executed on a GPU."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param camera The camera to use, a member of the eLeapPerspectiveType enumeration"]
    #[doc = " @param rectilinear A Vector containing the ray direction."]
    #[doc = " @returns A Vector containing the pixel coordinates [x, y, 1] (with z always 1)."]
    #[doc = " @since 3.1.3"]
    pub fn rectilinear_to_pixel<P: Into<[f32; 3]>>(
        &mut self,
        camera: PerspectiveType,
        rectilinear: P,
    ) -> [f32; 3] {
        unsafe {
            leap_vector_to_array(LeapRectilinearToPixel(
                self.handle,
                camera.into(),
                build_leap_vector(rectilinear),
            ))
        }
    }

    pub fn get_point_mapping_size(&mut self) -> Result<u64, Error> {
        let mut s = 0;
        unsafe {
            leap_try(LeapGetPointMappingSize(self.handle, &mut s))?;
        }
        Ok(s)
    }

    pub fn get_point_mapping(&mut self) -> Result<PointMapping, Error> {
        let mut size = self.get_point_mapping_size()?;
        unsafe {
            let mut mapping = PointMapping::new_uninitialized(size);
            leap_try(LeapGetPointMapping(
                self.handle,
                &mut mapping.handle.sized,
                &mut size,
            ))?;
            Ok(mapping)
        }
    }
}

#[cfg(feature = "gemini")]
impl Connection {
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
        Ok(Version(version))
    }

    #[doc = " Sets or clears one or more policy flags for a particular device."]
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
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param set A bitwise combination of flags to be set. Set to 0 if not setting any flags."]
    #[doc = " @param clear A bitwise combination of flags to be cleared. Set to 0 if not clearing any flags."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    pub fn set_policy_flags_ex(
        &mut self,
        device: &Device,
        set: PolicyFlags,
        clear: PolicyFlags,
    ) -> Result<(), Error> {
        unsafe {
            leap_try(LeapSetPolicyFlagsEx(
                self.handle,
                device.handle,
                set.bits(),
                clear.bits(),
            ))
        }
    }

    #[doc = " \\ingroup Functions"]
    #[doc = " Requests the currently set tracking mode."]
    #[doc = ""]
    #[doc = " Requesting the current tracking mode is asynchronous. After you call this function, a subsequent"]
    #[doc = " call to LeapPollConnection provides a LEAP_TRACKING_MODE_EVENT containing the current"]
    #[doc = " tracking mode, reflecting any changes."]
    #[doc = ""]
    #[doc = " The eLeapTrackingMode enumeration defines the tracking mode."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.0.0"]
    pub fn get_tracking_mode(&mut self) -> Result<(), Error> {
        unsafe { leap_try(LeapGetTrackingMode(self.handle)) }
    }

    #[doc = " \\ingroup Functions"]
    #[doc = " Requests the currently set tracking mode for a particular device."]
    #[doc = ""]
    #[doc = " Requesting the current tracking mode is asynchronous. After you call this function, a subsequent"]
    #[doc = " call to LeapPollConnection provides a LEAP_TRACKING_MODE_EVENT containing the current"]
    #[doc = " tracking mode, reflecting any changes."]
    #[doc = ""]
    #[doc = " The eLeapTrackingMode enumeration defines the tracking mode."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    pub fn get_tracking_mode_ex(&mut self, device: &Device) -> Result<(), Error> {
        unsafe { leap_try(LeapGetTrackingModeEx(self.handle, device.handle)) }
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

    #[doc = " \\ingroup Functions"]
    #[doc = " Requests a tracking mode for a particular device."]
    #[doc = ""]
    #[doc = " Changing tracking modes is asynchronous. After you call this function, a subsequent"]
    #[doc = " call to LeapPollConnection provides a LEAP_POLICY_EVENT containing the current"]
    #[doc = " policies, reflecting any changes."]
    #[doc = ""]
    #[doc = " The eLeapTrackingMode enumeration defines the tracking mode."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param mode The enum value specifying the requested tracking mode."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    pub fn set_tracking_mode_ex(
        &mut self,
        device: &Device,
        mode: TrackingMode,
    ) -> Result<(), Error> {
        unsafe {
            leap_try(LeapSetTrackingModeEx(
                self.handle,
                device.handle,
                mode.into(),
            ))
        }
    }

    #[doc = " For a multi-device aware client, sets the device to use in the context of"]
    #[doc = " non-\"Ex\" API functions which are logically device-specific but don't provide"]
    #[doc = " a device parameter."]
    #[doc = ""]
    #[doc = " Automatically subscribes to the specified device (see LeapSubscribeEvents()),"]
    #[doc = " and if \\p unsubscribeOthers is \\c true, then unsubscribes from all other devices"]
    #[doc = " as well (see LeapUnsubscribeEvents())."]
    #[doc = ""]
    #[doc = " Affects future invocations of the following functions:"]
    #[doc = "  - LeapCameraMatrix()"]
    #[doc = "  - LeapDistortionCoeffs()"]
    #[doc = "  - LeapGetFrameSize()"]
    #[doc = "  - LeapInterpolateFrame()"]
    #[doc = "  - LeapInterpolateFrameFromTime()"]
    #[doc = "  - LeapPixelToRectilinear()"]
    #[doc = "  - LeapRectilinearToPixel()"]
    #[doc = ""]
    #[doc = " It is not necessary to call this function from a client that does not claim"]
    #[doc = " to be multi-device-aware (see ::eLeapConnectionConfig and"]
    #[doc = " ::LeapCreateConnection)."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param unsubscribeOthers If \\c true, unsubscribe from all other devices."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    pub fn set_primary_device(
        &mut self,
        device: &Device,
        unsubscribe_others: bool,
    ) -> Result<(), Error> {
        unsafe {
            leap_try(LeapSetPrimaryDevice(
                self.handle,
                device.handle,
                unsubscribe_others,
            ))?;
        }
        Ok(())
    }

    #[doc = " Subscribe to event messages based on device."]
    #[doc = ""]
    #[doc = " If events from multiple devices are being sent from a service, this function"]
    #[doc = " allows the client to receive events from the specified device. Clients that"]
    #[doc = " claim to be multi-device-aware (see ::eLeapConnectionConfig and"]
    #[doc = " ::LeapCreateConnection) must subscribe to a device to receive various"]
    #[doc = " device-specific events."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A handle to the device for which events are desired."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    pub fn subscribe_events(&mut self, device: &Device) -> Result<(), Error> {
        unsafe {
            leap_try(LeapSubscribeEvents(self.handle, device.handle))?;
        }
        Ok(())
    }

    #[doc = " Unsubscribe from event messages based on device."]
    #[doc = ""]
    #[doc = " If events from multiple devices are being sent from a service, this function"]
    #[doc = " prevents receiving further events from the specified device that had"]
    #[doc = " previously been enabled using a call to LeapSubscribeEvents()."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A handle to the device for which events are desired."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    pub fn unsubscribe_events(&mut self, device: &Device) -> Result<(), Error> {
        unsafe {
            leap_try(LeapUnsubscribeEvents(self.handle, device.handle))?;
        }
        Ok(())
    }

    #[doc = " \\ingroup Functions"]
    #[doc = " Retrieves the number of bytes required to allocate an interpolated frame at the specified time"]
    #[doc = " for a particular device."]
    #[doc = ""]
    #[doc = " Use this function to determine the size of the buffer to allocate when calling"]
    #[doc = " LeapInterpolateFrameEx()."]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param timestamp The timestamp of the frame whose size is to be queried."]
    #[doc = " @param[out] pncbEvent A pointer that receives the number of bytes required to store the specified frame."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    pub fn get_frame_size_ex(&mut self, device: &Device, timestamp: i64) -> Result<u64, Error> {
        let mut result: u64 = 0;
        unsafe {
            leap_try(LeapGetFrameSizeEx(
                self.handle,
                device.handle,
                timestamp,
                &mut result,
            ))?;
        }
        Ok(result)
    }

    #[doc = " \\ingroup Functions"]
    #[doc = " Retrieves the number of bytes required to allocate an interpolated frame at the specified time"]
    #[doc = " for a particular device."]
    #[doc = ""]
    #[doc = " Use this function to determine the size of the buffer to allocate when calling"]
    #[doc = " LeapInterpolateFrameEx()."]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param timestamp The timestamp of the frame whose size is to be queried."]
    #[doc = " @param[out] pncbEvent A pointer that receives the number of bytes required to store the specified frame."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 5.4.0"]
    pub fn interpolate_frame_ex(
        &mut self,
        device: &Device,
        timestamp: i64,
        requested_frame_size: u64,
    ) -> Result<InterpolationTrackingEvent, Error> {
        // LEAP_TRACKING_EVENT with more size to account for the dynamic frame data.
        unsafe {
            let mut event = InterpolationTrackingEvent::new_uninitialized(requested_frame_size);

            leap_try(LeapInterpolateFrameEx(
                self.handle,
                device.handle,
                timestamp,
                &mut event.0.sized,
                event.0.size() as u64,
            ))?;
            Ok(event)
        }
    }

    #[doc = " Provides the corrected camera ray intercepting the specified point"]
    #[doc = " on the image for a particular device."]
    #[doc = ""]
    #[doc = " Given a point on the image, ``LeapPixelToRectilinearEx()`` corrects for camera distortion"]
    #[doc = " and returns the true direction from the camera to the source of that image point"]
    #[doc = " within the Devices field of view."]
    #[doc = ""]
    #[doc = " This direction vector has an x and y component [x, y, 1], with the third element"]
    #[doc = " always 1. Note that this vector uses the 2D camera coordinate system"]
    #[doc = " where the x-axis parallels the longer (typically horizontal) dimension and"]
    #[doc = " the y-axis parallels the shorter (vertical) dimension. The camera coordinate"]
    #[doc = " system does not correlate to the 3D Ultraleap coordinate system."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param camera The camera to use, a member of the eLeapPerspectiveType enumeration"]
    #[doc = " @param pixel A Vector containing the position of a pixel in the image."]
    #[doc = " @returns A Vector containing the ray direction (the z-component of the vector is always 1)."]
    #[doc = " @since 5.4.0"]
    pub fn pixel_to_rectilinear_ex<P: Into<[f32; 3]>>(
        &mut self,
        device: &Device,
        camera: PerspectiveType,
        pixel: P,
    ) -> [f32; 3] {
        unsafe {
            leap_vector_to_array(LeapPixelToRectilinearEx(
                self.handle,
                device.handle,
                camera.into(),
                build_leap_vector(pixel),
            ))
        }
    }

    #[doc = " Provides the point in the image corresponding to a ray projecting"]
    #[doc = " from the camera for a particular device."]
    #[doc = ""]
    #[doc = " Given a ray projected from the camera in the specified direction, ``LeapRectilinearToPixelEx()``"]
    #[doc = " corrects for camera distortion and returns the corresponding pixel"]
    #[doc = " coordinates in the image."]
    #[doc = ""]
    #[doc = " The ray direction is specified in relationship to the camera. The first"]
    #[doc = " vector element is the tangent of the \"horizontal\" view angle; the second"]
    #[doc = " element is the tangent of the \"vertical\" view angle."]
    #[doc = ""]
    #[doc = " The ``LeapRectilinearToPixelEx()`` function returns pixel coordinates outside of the image bounds"]
    #[doc = " if you project a ray toward a point for which there is no recorded data."]
    #[doc = ""]
    #[doc = " ``LeapRectilinearToPixelEx()`` is typically not fast enough for realtime distortion correction."]
    #[doc = " For better performance, use a shader program executed on a GPU."]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param camera The camera to use, a member of the eLeapPerspectiveType enumeration"]
    #[doc = " @param rectilinear A Vector containing the ray direction."]
    #[doc = " @returns A Vector containing the pixel coordinates [x, y, 1] (with z always 1)."]
    #[doc = " @since 5.4.0"]
    pub fn rectilinear_to_pixel_ex<P: Into<[f32; 3]>>(
        &mut self,
        device: &Device,
        camera: PerspectiveType,
        rectilinear: P,
    ) -> [f32; 3] {
        unsafe {
            leap_vector_to_array(LeapRectilinearToPixelEx(
                self.handle,
                device.handle,
                camera.into(),
                build_leap_vector(rectilinear),
            ))
        }
    }

    #[doc = " Returns an OpenCV-compatible camera matrix for a particular device."]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param camera The camera to use, a member of the eLeapPerspectiveType enumeration"]
    #[doc = " @param[out] dest A pointer to a single-precision float array of size 9"]
    #[doc = " @since 5.4.0"]
    pub fn camera_matrix_ex(
        &mut self,
        device: &Device,
        camera: PerspectiveType,
        dest: &mut [f32; 9],
    ) {
        unsafe { LeapCameraMatrixEx(self.handle, device.handle, camera.into(), dest.as_mut_ptr()) }
    }

    #[doc = " Returns an OpenCV-compatible lens distortion for a particular device, using"]
    #[doc = " the 8-parameter rational model."]
    #[doc = ""]
    #[doc = " The order of the returned array is: [k1, k2, p1, p2, k3, k4, k5, k6]"]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param hDevice A device handle returned by LeapOpenDevice()."]
    #[doc = " @param camera The camera to use, a member of the eLeapPerspectiveType enumeration"]
    #[doc = " @param[out] dest A pointer to a single-precision float array of size 8."]
    #[doc = " @since 5.4.0"]
    pub fn distortion_coeffs_ex(
        &mut self,
        device: &Device,
        camera: PerspectiveType,
        dest: &mut [f32; 8],
    ) {
        unsafe {
            LeapDistortionCoeffsEx(self.handle, device.handle, camera.into(), dest.as_mut_ptr())
        }
    }

    #[doc = " Returns an OpenCV-compatible camera matrix."]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param camera The camera to use, a member of the eLeapPerspectiveType enumeration"]
    #[doc = " @param[out] dest A pointer to a single-precision float array of size 9"]
    #[doc = " @since 3.2.1"] // Actually not present in orion LeapC
    pub fn camera_matrix(&mut self, camera: PerspectiveType, dest: &mut [f32; 9]) {
        unsafe { LeapCameraMatrix(self.handle, camera.into(), dest.as_mut_ptr()) }
    }

    #[doc = " Returns an OpenCV-compatible lens distortion using the 8-parameter rational"]
    #[doc = " model."]
    #[doc = ""]
    #[doc = " The order of the returned array is: [k1, k2, p1, p2, k3, k4, k5, k6]"]
    #[doc = ""]
    #[doc = " @param hConnection The connection handle created by LeapCreateConnection()."]
    #[doc = " @param camera The camera to use, a member of the eLeapPerspectiveType enumeration"]
    #[doc = " @param[out] dest A pointer to a single-precision float array of size 8."]
    #[doc = " @since 3.2.1"] // Actually not present in orion LeapC
    pub fn distortion_coeffs(&mut self, camera: PerspectiveType, dest: &mut [f32; 8]) {
        unsafe { LeapDistortionCoeffs(self.handle, camera.into(), dest.as_mut_ptr()) }
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
        assert!(!devices.is_empty(), "No devices, tests will not run.");
    }

    #[cfg(feature = "gemini")]
    mod gemini {
        use std::time::{Duration, SystemTime};

        use crate::tests::*;
        use crate::*;

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
        fn set_primary_device_test() {
            let (mut connection, first_device) = initialize_connection_ex();

            connection.set_primary_device(&first_device, true).unwrap();
            let _ = connection.get_frame_size(0);
        }

        #[test]
        fn pause_resume() {
            let (mut connection, first_device) = initialize_connection_ex();

            connection
                .set_policy_flags_ex(
                    &first_device,
                    PolicyFlags::ALLOW_PAUSE_RESUME,
                    PolicyFlags::empty(),
                )
                .expect("Failed to allow pause");

            connection
                .wait_for(|e| match e {
                    EventRef::Policy(_) => Some(()),
                    _ => None,
                })
                .expect("Did not receive policy change");

            connection.set_pause(true).expect("Failed to set pause");
            connection.set_pause(false).expect("Failed to unset pause");

            connection
                .set_policy_flags_ex(
                    &first_device,
                    PolicyFlags::empty(),
                    PolicyFlags::ALLOW_PAUSE_RESUME,
                )
                .expect("Failed to remove pause allow");

            let pause_err = connection
                .set_pause(true)
                .expect_err("succeded to pause even though it was forbidden");
            assert_eq!(pause_err, Error::InvalidArgument);
        }

        #[test]
        fn safety_sanity() {
            let (mut connection, device) = initialize_connection_ex();

            let mut camera_matrix = [0.0; 9];
            connection.camera_matrix_ex(&device, PerspectiveType::StereoLeft, &mut camera_matrix);

            let mut distortion_coeffs = [0.0; 8];
            connection.distortion_coeffs_ex(
                &device,
                PerspectiveType::StereoRight,
                &mut distortion_coeffs,
            );

            let mut camera_matrix = [0.0; 9];
            connection.camera_matrix(PerspectiveType::StereoLeft, &mut camera_matrix);

            let mut distortion_coeffs = [0.0; 8];
            connection.distortion_coeffs(PerspectiveType::StereoRight, &mut distortion_coeffs);

            let leap_vector = [0.0, 0.0, 1.0];
            connection.pixel_to_rectilinear_ex(&device, PerspectiveType::StereoLeft, leap_vector);
            connection.rectilinear_to_pixel_ex(&device, PerspectiveType::StereoRight, leap_vector);
        }

        #[test]
        fn frame_interpolation() {
            let start = SystemTime::now();

            let (mut connection, device) = initialize_connection_ex();
            let mut clock_synchronizer =
                ClockRebaser::create().expect("Failed to create clock sync");

            for _ in 0..10 {
                // Note: If events are not polled, the frame interpolation fails with "is not seer"
                connection
                    .wait_for(|e| match e {
                        EventRef::Tracking(_) => Some(()),
                        _ => None,
                    })
                    .expect("no tracking data");
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
                    .get_frame_size_ex(&device, target_frame_time)
                    .expect("Failed to get requested size");

                let frame = connection
                    .interpolate_frame_ex(&device, target_frame_time, requested_size)
                    .expect("Failed to interpolate frame");
                let _hands = frame.hands();
            }
        }
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
                config_key,
                CString::new("hello").unwrap().into(),
                Some(&mut request_id),
            )
            .expect("Failed to save a config value");
    }

    /// Basebone test just going through otherwise untested unsafe code
    #[test]
    fn safety_sanity() {
        let mut connection = initialize_connection();

        let leap_vector = [0.0, 0.0, 1.0];
        connection.pixel_to_rectilinear(PerspectiveType::StereoLeft, leap_vector);
        connection.rectilinear_to_pixel(PerspectiveType::StereoRight, leap_vector);
    }

    #[test]
    #[ignore = "Unsupported in gemini"]
    fn point_mapping() {
        let mut connection = initialize_connection();
        let point_mapping = connection.get_point_mapping().unwrap();
        let _pid_vec = point_mapping.pids().to_vec();
        let _points = point_mapping.points();
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
        connection
            .wait_for(|e| match e {
                EventRef::ConfigResponse(c) => {
                    if c.requestID != request_id {
                        None
                    } else if let Variant::Boolean(robust_mode_enabled) = c.value() {
                        Some(robust_mode_enabled)
                    } else {
                        panic!("Type mismatch for the configuration value.")
                    }
                }
                _ => None,
            })
            .expect("Did not receive the config");
    }

    #[test]
    fn frame_interpolation() {
        let start = SystemTime::now();

        let mut connection = initialize_connection();
        let mut clock_synchronizer = ClockRebaser::create().expect("Failed to create clock sync");

        for _ in 0..10 {
            // Note: If events are not polled, the frame interpolation fails with "is not seer"
            connection
                .wait_for(|e| match e {
                    EventRef::Tracking(_) => Some(()),
                    _ => None,
                })
                .expect("no tracking data");
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

    #[test]
    fn pause_resume() {
        let mut connection = initialize_connection();

        connection
            .set_policy_flags(PolicyFlags::ALLOW_PAUSE_RESUME, PolicyFlags::empty())
            .expect("Failed to allow pause");

        connection
            .wait_for(|e| match e {
                EventRef::Policy(_) => Some(()),
                _ => None,
            })
            .expect("Did not receive policy change");

        connection.set_pause(true).expect("Failed to set pause");
        connection.set_pause(false).expect("Failed to unset pause");

        connection
            .set_policy_flags(PolicyFlags::empty(), PolicyFlags::ALLOW_PAUSE_RESUME)
            .expect("Failed to remove pause allow");

        let pause_err = connection
            .set_pause(true)
            .expect_err("succeded to pause even though it was forbidden");
        assert_eq!(pause_err, Error::InvalidArgument);
    }
}
