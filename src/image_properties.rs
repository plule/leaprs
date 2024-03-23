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
}
