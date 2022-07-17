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

    #[doc = " A version number for the distortion matrix. When the distortion matrix"]
    #[doc = " changes, this number is updated. This number is guaranteed not to repeat"]
    #[doc = " for the lifetime of the connection. This version number is also guaranteed"]
    #[doc = " to be distinct for each perspective of an image."]
    #[doc = ""]
    #[doc = " This value is guaranteed to be nonzero if it is valid."]
    #[doc = ""]
    #[doc = " The distortion matrix only changes when the streaming device changes or when the"]
    #[doc = " device orientation flips -- inverting the image and the distortion grid."]
    #[doc = " Since building a matrix to undistort an image can be a time-consuming task,"]
    #[doc = " you can optimize the process by only rebuilding this matrix (or whatever"]
    #[doc = " data type you use to correct image distortion) when the grid actually changes."]
    pub fn matrix_version(&self) -> u64 {
        self.handle.matrix_version
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
