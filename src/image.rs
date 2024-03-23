use leap_sys::LEAP_IMAGE;

use crate::{DistortionMatrix, ImageProperties};

crate::leap_ref_struct!(
    #[doc = " An image associated with a frame of data."]
    #[doc = " @since 4.0.0"]
    Image,
    LEAP_IMAGE
);

impl<'a> Image<'a> {
    #[doc = " The properties of the received image."]
    pub fn properties(&self) -> ImageProperties {
        (&self.handle.properties).into()
    }

    #[doc = " Pointers to the camera's distortion matrix."]
    pub fn distorion_matrix(&self) -> DistortionMatrix {
        let distortion_matrix;

        unsafe {
            distortion_matrix = &*self.handle.distortion_matrix;
        }

        distortion_matrix.into()
    }

    #[doc = " A pointer to the image data."]
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
