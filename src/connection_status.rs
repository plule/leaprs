use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
#[doc = " The connection status codes."]
#[doc = " These codes can be read from the LEAP_CONNECTION_INFO struct created by"]
#[doc = " a call to LeapGetConnectionInfo()."]
#[doc = " @since 3.0.0"]
pub enum ConnectionStatus {
    #[doc = " The connection is not open."]
    #[doc = " Call LeapOpenConnection() to open a connection to the Ultraleap Tracking Service."]
    #[doc = " @since 3.0.0"]
    NotConnected = _eLeapConnectionStatus_eLeapConnectionStatus_NotConnected,
    #[doc = " The connection is open."]
    #[doc = " @since 3.0.0"]
    Connected = _eLeapConnectionStatus_eLeapConnectionStatus_Connected,
    #[doc = " Opening the connection is underway, but not complete."]
    #[doc = " @since 3.0.0"]
    HandshakeIncomplete = _eLeapConnectionStatus_eLeapConnectionStatus_HandshakeIncomplete,
    #[doc = " The connection could not be opened because the Ultraleap Tracking Service does not"]
    #[doc = " appear to be running."]
    #[doc = " @since 3.0.0"]
    NotRunning = _eLeapConnectionStatus_eLeapConnectionStatus_NotRunning,
    #[num_enum(default)]
    Unknown,
}
