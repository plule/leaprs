use derive_deref::Deref;
use leap_sys::LEAP_IMAGE;

use crate::{DistortionMatrix, ImageProperties};

#[doc = " An image associated with a frame of data."]
#[doc = " @since 4.0.0"]
#[derive(Deref)]
pub struct Image<'a>(pub(crate) &'a LEAP_IMAGE);

impl<'a> Image<'a> {
    #[doc = " The properties of the received image."]
    pub fn properties(&self) -> ImageProperties {
        ImageProperties(&self.properties)
    }

    #[doc = " Pointers to the camera's distortion matrix."]
    pub fn distorion_matrix(&self) -> DistortionMatrix {
        let distortion_matrix;

        unsafe {
            distortion_matrix = &*self.distortion_matrix;
        }

        DistortionMatrix(distortion_matrix)
    }

    #[doc = " A pointer to the image data."]
    pub fn data(&self) -> &[u8] {
        let width = self.properties.width;
        let height = self.properties.height;
        let size = (width * height) as usize;
        unsafe {
            let start = self.data.offset(self.offset as isize) as *const u8;
            std::slice::from_raw_parts(start, size)
        }
    }
}
