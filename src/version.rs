use leap_sys::LEAP_VERSION;

crate::leap_struct!(
    #[doc = " Version information."]
    #[doc = ""]
    #[doc = " The members can be converted to a version string using the format:"]
    #[doc = ""]
    #[doc = " major.minor.patch.build"]
    #[doc = ""]
    #[doc = " @since 5.2.0"]
    Version,
    LEAP_VERSION
);

impl Version {
    #[doc = " The major version."]
    #[doc = " @since 5.2.0"]
    pub fn major(&self) -> i32 {
        self.handle.major
    }

    #[doc = " The minor version."]
    #[doc = " @since 5.2.0"]
    pub fn minor(&self) -> i32 {
        self.handle.minor
    }

    #[doc = " The patch version."]
    #[doc = " @since 5.2.0"]
    pub fn patch(&self) -> i32 {
        self.handle.patch
    }
}
