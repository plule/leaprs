use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
pub enum DroppedFrameType {
    PreprocessingQueue = _eLeapDroppedFrameType_eLeapDroppedFrameType_PreprocessingQueue,
    TrackingQueue = _eLeapDroppedFrameType_eLeapDroppedFrameType_TrackingQueue,
    #[num_enum(default)]
    Other = _eLeapDroppedFrameType_eLeapDroppedFrameType_Other,
}
