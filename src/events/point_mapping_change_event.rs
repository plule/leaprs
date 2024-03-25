use derive_deref::Deref;
use leap_sys::LEAP_POINT_MAPPING_CHANGE_EVENT;

/// # Fields
/// Available via dereference: [LEAP_POINT_MAPPING_CHANGE_EVENT].
#[derive(Deref, Clone, Copy)]
pub struct PointMappingChangeEventRef<'a>(pub(crate) &'a LEAP_POINT_MAPPING_CHANGE_EVENT);
