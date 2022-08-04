use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};
use thiserror::Error;

/// Convert a Leap return code into a possible error.
pub(crate) fn leap_try(leap_rs: i32) -> Result<(), Error> {
    if leap_rs == _eLeapRS_eLeapRS_Success {
        return Ok(());
    }
    Err(leap_rs.into())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive, Error)]
#[repr(i32)]
#[doc = " Defines the codes returned by all LeapC functions."]
pub enum Error {
    #[error("An undetermined error has occurred.")]
    #[doc = " This is usually the result of an abnormal operating condition in LeapC,"]
    #[doc = " the Ultraleap Tracking Service, or the host computer itself."]
    #[doc = " @since 3.0.0"]
    UnknownError = _eLeapRS_eLeapRS_UnknownError,
    #[error("An invalid argument was specified.")]
    #[doc = " @since 3.0.0"]
    InvalidArgument = _eLeapRS_eLeapRS_InvalidArgument,
    #[error(" Insufficient resources existed to complete the request.")]
    #[doc = " @since 3.0.0"]
    InsufficientResources = _eLeapRS_eLeapRS_InsufficientResources,
    #[error(" The specified buffer was not large enough to complete the request.")]
    #[doc = " @since 3.0.0"]
    InsufficientBuffer = _eLeapRS_eLeapRS_InsufficientBuffer,
    #[error("The requested operation has timed out.")]
    #[doc = " @since 3.0.0"]
    Timeout = _eLeapRS_eLeapRS_Timeout,
    #[error(" The operation is invalid because there is no current connection.")]
    #[doc = " @since 3.0.0"]
    NotConnected = _eLeapRS_eLeapRS_NotConnected,
    #[error(" The operation is invalid because the connection is not complete.")]
    #[doc = " @since 3.0.0"]
    HandshakeIncomplete = _eLeapRS_eLeapRS_HandshakeIncomplete,
    #[error(" The specified buffer size is too large.")]
    #[doc = " @since 3.0.0"]
    BufferSizeOverflow = _eLeapRS_eLeapRS_BufferSizeOverflow,
    #[error(" A communications protocol error occurred.")]
    #[doc = " @since 3.0.0"]
    ProtocolError = _eLeapRS_eLeapRS_ProtocolError,
    #[error(" The server incorrectly specified zero as a client ID.")]
    #[doc = " @since 3.0.0"]
    InvalidClientID = _eLeapRS_eLeapRS_InvalidClientID,
    #[error(" The connection to the service was unexpectedly closed while reading or writing a message.")]
    #[doc = " The server may have terminated."]
    #[doc = " @since 3.0.0"]
    UnexpectedClosed = _eLeapRS_eLeapRS_UnexpectedClosed,
    #[error(" The specified request token does not appear to be valid")]
    #[doc = ""]
    #[doc = " Provided that the token value which identifies the request itself was, at one point, valid, this"]
    #[doc = " error condition occurs when the request to which the token refers has already been satisfied or"]
    #[doc = " is currently being satisfied."]
    #[doc = " @since 3.0.0"]
    UnknownImageFrameRequest = _eLeapRS_eLeapRS_UnknownImageFrameRequest,
    #[error(" The specified frame ID is not valid or is no longer valid")]
    #[doc = ""]
    #[doc = " Provided that frame ID was, at one point, valid, this error condition occurs when the identifier"]
    #[doc = " refers to a frame that occurred further in the past than is currently recorded in the rolling"]
    #[doc = " frame window."]
    #[doc = " @since 3.0.0"]
    UnknownTrackingFrameID = _eLeapRS_eLeapRS_UnknownTrackingFrameID,
    #[error(" The specified timestamp references a future point in time")]
    #[doc = ""]
    #[doc = " The related routine can only operate on time points having occurred in the past, and the"]
    #[doc = " provided timestamp occurs in the future."]
    #[doc = " @since 3.1.2"]
    RoutineIsNotSeer = _eLeapRS_eLeapRS_RoutineIsNotSeer,
    #[error(" The specified timestamp references a point too far in the past")]
    #[doc = ""]
    #[doc = " The related routine can only operate on time points occurring within its immediate record of"]
    #[doc = " the past."]
    #[doc = " @since 3.1.2"]
    TimestampTooEarly = _eLeapRS_eLeapRS_TimestampTooEarly,
    #[error(" LeapPollConnection is called concurrently.")]
    #[doc = " @since 3.1.2"]
    ConcurrentPoll = _eLeapRS_eLeapRS_ConcurrentPoll,
    #[error(" A connection to the Ultraleap Tracking Service could not be established.")]
    #[doc = "@since 3.0.0"]
    NotAvailable = _eLeapRS_eLeapRS_NotAvailable,
    #[error(" The requested operation can only be performed while the device is sending data.")]
    #[doc = " @since 3.0.0"]
    NotStreaming = _eLeapRS_eLeapRS_NotStreaming,
    #[error(
        " The specified device could not be opened. It is possible that the device identifier"
    )]
    #[doc = " is invalid, or that the device has been disconnected since being enumerated."]
    #[doc = " @since 3.0.0"]
    CannotOpenDevice = _eLeapRS_eLeapRS_CannotOpenDevice,
    #[error(" The request is not supported by this version of the service.")]
    #[doc = " @since 5.4.0"]
    Unsupported = _eLeapRS_eLeapRS_Unsupported,

    /// The value is not a code. This is likely a bug or a version mixup.
    #[num_enum(default)]
    #[error(
        "The returned value is not a code. This is likely a LeapRS bug or a LeapSDK version mismatch."
    )]
    Unknown,
}
