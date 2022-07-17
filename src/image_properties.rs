use leap_sys::LEAP_IMAGE_PROPERTIES;

use crate::{ImageFormat, ImageType};

crate::leap_ref_struct!(
    #[doc = " Properties of a sensor image."]
    #[doc = " @since 3.0.0"]
    ImageProperties,
    LEAP_IMAGE_PROPERTIES
);

impl<'a> ImageProperties<'a> {
    #[doc = " The type of this image. @since 3.0.0"]
    pub fn image_type(&self) -> ImageType {
        self.handle.type_.into()
    }

    #[doc = " The format of this image. @since 3.0.0"]
    pub fn image_format(&self) -> ImageFormat {
        self.handle.format.into()
    }

    #[doc = " The number of bytes per image pixel. @since 3.0.0"]
    pub fn bpp(&self) -> u32 {
        self.handle.bpp
    }

    #[doc = " The number of horizontal pixels in the image. @since 3.0.0"]
    pub fn width(&self) -> u32 {
        self.handle.width
    }

    #[doc = " The number of rows of pixels in the image. @since 3.0.0"]
    pub fn height(&self) -> u32 {
        self.handle.height
    }
}
