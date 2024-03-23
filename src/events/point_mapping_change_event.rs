use derive_deref::Deref;
use leap_sys::LEAP_POINT_MAPPING_CHANGE_EVENT;

#[derive(Deref)]
pub struct PointMappingChangeEvent<'a>(pub(crate) &'a LEAP_POINT_MAPPING_CHANGE_EVENT);
