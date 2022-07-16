use leap_sys::LEAP_IMAGE;

use crate::{DistortionMatrix, ImageProperties};

crate::leap_ref_struct!(Image, LEAP_IMAGE);

impl<'a> Image<'a> {
    pub fn properties(&self) -> ImageProperties {
        (&self.handle.properties).into()
    }

    pub fn matrix_version(&self) -> u64 {
        self.handle.matrix_version
    }

    pub fn distorion_matrix(&self) -> DistortionMatrix {
        let distortion_matrix;

        unsafe {
            distortion_matrix = &*self.handle.distortion_matrix;
        }

        distortion_matrix.into()
    }

    pub fn data(&self) -> &[u8] {
        let width = self.handle.properties.width;
        let height = self.handle.properties.height;
        let size = (width * height) as usize;
        unsafe {
            let start = self.handle.data.offset(self.handle.offset as isize) as *const u8;
            std::slice::from_raw_parts(start, size)
        }
    }
}
