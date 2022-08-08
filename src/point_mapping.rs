use leap_sys::LEAP_POINT_MAPPING;

use crate::{sized_with_trailing_data::SizedWithTrailingData, LeapVector};

pub struct PointMapping {
    /// Store a boxed dynamic sized event
    /// The size is only known at runtime
    pub(crate) handle: Box<SizedWithTrailingData<LEAP_POINT_MAPPING>>,
}

impl PointMapping {
    /// Allocate a LEAP_POINT_MAPPING with more data contiguous to it.
    /// Unsafe: inner struct is uninitialized
    pub(crate) unsafe fn new_uninitialized(point_mapping_size: u64) -> Self {
        let trailing_size = point_mapping_size as usize - std::mem::size_of::<LEAP_POINT_MAPPING>();
        Self {
            handle: SizedWithTrailingData::allocate(std::mem::zeroed(), trailing_size),
        }
    }

    #[doc = " The ID of the frame corresponding to the source of the currently tracked points. @since 4.0.0"]
    pub fn frame_id(&self) -> i64 {
        self.handle.sized.frame_id
    }

    #[doc = " The timestamp of the frame, in microseconds, referenced against LeapGetNow(). @since 4.0.0"]
    pub fn timestamp(&self) -> i64 {
        self.handle.sized.timestamp
    }

    #[doc = " The number of points being tracked. @since 4.0.0"]
    pub fn n_points(&self) -> u32 {
        self.handle.sized.nPoints
    }

    #[doc = " The 3D points being mapped. @since 4.0.0"]
    pub fn points(&self) -> Vec<LeapVector> {
        unsafe {
            (0..self.handle.sized.nPoints as isize)
                .map(|i| LeapVector::from(*self.handle.sized.pPoints.offset(i)))
                .collect()
        }
    }

    #[doc = " The IDs of the 3D points being mapped. @since 4.0.0"]
    pub fn pids(&self) -> &[u32] {
        unsafe {
            std::slice::from_raw_parts(self.handle.sized.pIDs, self.handle.sized.nPoints as usize)
        }
    }
}
