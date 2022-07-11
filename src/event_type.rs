use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
pub enum EventType {
    #[doc = " No event has occurred within the timeout period specified when calling LeapPollConnection()."]
    #[doc = " @since 3.0.0"]
    None = _eLeapEventType_eLeapEventType_None,
    #[doc = " A connection to the Ultraleap Tracking Service has been established."]
    #[doc = " @since 3.0.0"]
    Connection = _eLeapEventType_eLeapEventType_Connection,
    #[doc = " The connection to the Ultraleap Tracking Service has been lost."]
    #[doc = " @since 3.0.0"]
    ConnectionLost = _eLeapEventType_eLeapEventType_ConnectionLost,
    #[doc = " A device has been detected or plugged-in."]
    #[doc = " A device event is dispatched after a connection is established for any"]
    #[doc = " devices already plugged in. (The system currently only supports one"]
    #[doc = " streaming device at a time.)"]
    #[doc = " @since 3.0.0"]
    Device = _eLeapEventType_eLeapEventType_Device,
    #[doc = " A device has failed."]
    #[doc = " Device failure could be caused by hardware failure, USB controller issues, or"]
    #[doc = " other system instability. Note that unplugging a device generates an"]
    #[doc = " eLeapEventType_DeviceLost event message, not a failure message."]
    #[doc = " @since 3.0.0"]
    DeviceFailure = _eLeapEventType_eLeapEventType_DeviceFailure,
    #[doc = " A policy change has occurred."]
    #[doc = " This can be due to setting a policy with LeapSetPolicyFlags() or due to changing"]
    #[doc = " or policy-related config settings, including images_mode."]
    #[doc = " (A user can also change these policies using the Ultraleap Tracking Control Panel.)"]
    #[doc = " @since 3.0.0"]
    Policy = _eLeapEventType_eLeapEventType_Policy,
    #[doc = " A tracking frame. The message contains the tracking data for the frame."]
    #[doc = " @since 3.0.0"]
    Tracking = _eLeapEventType_eLeapEventType_Tracking,
    #[doc = " The request for an image has failed."]
    #[doc = " The message contains information about the failure. The client application"]
    #[doc = " will not receive the requested image set."]
    #[doc = " @since 3.0.0"]
    ImageRequestError = _eLeapEventType_eLeapEventType_ImageRequestError,
    #[doc = " The request for an image is complete."]
    #[doc = " The image data has been completely written to the application-provided"]
    #[doc = " buffer."]
    #[doc = " @since 3.0.0"]
    ImageComplete = _eLeapEventType_eLeapEventType_ImageComplete,
    #[doc = " A system message. @since 3.0.0"]
    LogEvent = _eLeapEventType_eLeapEventType_LogEvent,
    #[doc = " The device connection has been lost."]
    #[doc = ""]
    #[doc = " This event is generally asserted when the device has been detached from the system, when the"]
    #[doc = " connection to the service has been lost, or if the device is closed while streaming. Generally,"]
    #[doc = " any event where the system can conclude no further frames will be received will result in this"]
    #[doc = " message. The DeviceEvent field will be filled with the id of the formerly attached device."]
    #[doc = " @since 3.0.0"]
    DeviceLost = _eLeapEventType_eLeapEventType_DeviceLost,
    #[doc = " The asynchronous response to a call to LeapRequestConfigValue()."]
    #[doc = " Contains the value of requested configuration item."]
    #[doc = " @since 3.0.0"]
    ConfigResponse = _eLeapEventType_eLeapEventType_ConfigResponse,
    #[doc = " The asynchronous response to a call to LeapSaveConfigValue()."]
    #[doc = " Reports whether the change succeeded or failed."]
    #[doc = " @since 3.0.0"]
    ConfigChange = _eLeapEventType_eLeapEventType_ConfigChange,
    #[doc = " Notification that a status change has been detected on an attached device"]
    #[doc = ""]
    #[doc = " @since 3.1.3"]
    DeviceStatusChange = _eLeapEventType_eLeapEventType_DeviceStatusChange,
    #[doc = " Notification that a status change has been detected on an attached device"]
    #[doc = ""]
    #[doc = " @since 3.1.3"]
    DroppedFrame = _eLeapEventType_eLeapEventType_DroppedFrame,
    #[doc = " Notification that an unrequested stereo image pair is available"]
    #[doc = ""]
    #[doc = " @since 4.0.0"]
    Image = _eLeapEventType_eLeapEventType_Image,
    #[doc = " Notification that point mapping has changed"]
    #[doc = ""]
    #[doc = " @since 4.0.0"]
    PointMappingChange = _eLeapEventType_eLeapEventType_PointMappingChange,
    #[doc = " A tracking mode change has occurred."]
    #[doc = " This can be due to changing the hmd or screentop policy with LeapSetPolicyFlags()."]
    #[doc = " or setting the tracking mode using LeapSetTrackingMode()."]
    #[doc = " @since 5.0.0"]
    TrackingMode = _eLeapEventType_eLeapEventType_TrackingMode,
    #[doc = " An array of system messages. @since 4.0.0"]
    LogEvents = _eLeapEventType_eLeapEventType_LogEvents,
    #[doc = " A head pose. The message contains the timestamped head position and orientation."]
    #[doc = " @since 4.1.0"]
    HeadPose = _eLeapEventType_eLeapEventType_HeadPose,
    #[doc = " Tracked eye positions. @since 4.1.0"]
    Eyes = _eLeapEventType_eLeapEventType_Eyes,
    #[doc = " An IMU reading. @since 4.1.0"]
    IMU = _eLeapEventType_eLeapEventType_IMU,
    #[num_enum(default)]
    NotAnEvent,
}
