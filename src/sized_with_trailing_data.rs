use std::alloc::Layout;

/// A DST with a sized part and unsized remainder.
#[repr(C, packed)]
pub(crate) struct SizedWithTrailingData<T> {
    pub sized: T,
    pub trailing: [u8],
}

impl<T> SizedWithTrailingData<T> {
    /// Allocate a SizedWithTrailingData with a given trailing size.
    /// From: https://stackoverflow.com/questions/64120001/how-to-create-a-smart-pointer-to-an-unsized-type-with-an-embedded-slice
    pub fn allocate(sized: T, trailing_size: usize) -> Box<Self> {
        // Create a layout of an `Inner` followed by the array
        let (layout, arr_base) = Layout::array::<usize>(trailing_size)
            .and_then(|arr_layout| Layout::new::<T>().extend(arr_layout))
            .unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        // At this point, `ptr` is `*mut u8` and the compiler doesn't know the size of the allocation
        if ptr.is_null() {
            panic!("Internal allocation error");
        }

        unsafe {
            ptr.cast::<T>().write(sized);
            let tmp_ptr = ptr.add(arr_base).cast::<usize>();
            // Initialize the array elements, in this case to 0
            (0..trailing_size).for_each(|i| tmp_ptr.add(i).write(0));

            // At this point everything is initialized and can safely be converted to `Box`
            Box::from_raw(
                std::ptr::slice_from_raw_parts_mut(ptr as *mut usize, trailing_size)
                    as *mut SizedWithTrailingData<T>,
            )
        }
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<T>() + std::mem::size_of_val(&self.trailing)
    }
}

#[cfg(test)]
mod tests {
    use leap_sys::LEAP_TRACKING_EVENT;

    use super::*;

    #[test]
    pub fn event_with_trailing() {
        let mut tracking_event: LEAP_TRACKING_EVENT;

        unsafe {
            tracking_event = std::mem::zeroed();
            tracking_event.tracking_frame_id = 42;
        }
        let mut dst = SizedWithTrailingData::allocate(tracking_event, 4);
        dst.sized.tracking_frame_id = 42;
        dst.trailing[0] = 1;
        dst.trailing[1] = 3;
        dst.trailing[2] = 5;
        dst.trailing[3] = 7;

        let sized = dst.sized;
        let frame = sized.tracking_frame_id;
        assert_eq!(frame, 42);
        assert_eq!(dst.trailing[0], 1);
        assert_eq!(dst.trailing[1], 3);
        assert_eq!(dst.trailing[2], 5);
        assert_eq!(dst.trailing[3], 7);
    }

    #[test]
    pub fn string_with_trailing() {
        let mut tracking_event: LEAP_TRACKING_EVENT;

        unsafe {
            tracking_event = std::mem::zeroed();
            tracking_event.tracking_frame_id = 42;
        }
        let mut dst = SizedWithTrailingData::allocate("hello".to_string(), 4);
        dst.trailing[0] = 1;
        dst.trailing[1] = 3;
        dst.trailing[2] = 5;
        dst.trailing[3] = 7;

        let sized = dst.sized;
        assert_eq!(sized, "hello".to_string());
        assert_eq!(dst.trailing[0], 1);
        assert_eq!(dst.trailing[1], 3);
        assert_eq!(dst.trailing[2], 5);
        assert_eq!(dst.trailing[3], 7);
    }
}
