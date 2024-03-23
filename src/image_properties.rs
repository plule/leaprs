use derive_deref::Deref;
use leap_sys::LEAP_IMAGE_PROPERTIES;

use crate::{ImageFormat, ImageType};

#[doc = " Properties of a sensor image."]
#[derive(Deref)]
pub struct ImagePropertiesRef<'a>(pub(crate) &'a LEAP_IMAGE_PROPERTIES);

impl<'a> ImagePropertiesRef<'a> {
    #[doc = " The type of this image. @since 3.0.0"]
    pub fn image_type(&self) -> ImageType {
        self.type_.into()
    }

    #[doc = " The format of this image. @since 3.0.0"]
    pub fn image_format(&self) -> ImageFormat {
        self.format.into()
    }
}
