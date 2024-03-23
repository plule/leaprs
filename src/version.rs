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
