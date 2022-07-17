use std::ffi::CString;

use bitflags::bitflags;
use leap_sys::*;

bitflags! {
    pub struct ConnectionConfigFlags: _eLeapConnectionConfig {
        #[doc = " The client is aware of how to handle multiple devices through the API."]
        #[doc = " @since 4.1.0"]
        const MULTI_DEVICE_AWARE = _eLeapConnectionConfig_eLeapConnectionConfig_MultiDeviceAware;
    }
}

#[doc = " Specifies the configuration for a connection."]
#[doc = " @since 3.0.0"]
pub struct ConnectionConfig {
    // Safety: the handle contains a pointer to the server_namespace
    pub(crate) handle: LEAP_CONNECTION_CONFIG,
    server_namespace: Option<CString>,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self::new(ConnectionConfigFlags::empty(), None)
    }
}

impl ConnectionConfig {
    pub fn new(flags: ConnectionConfigFlags, namespace: Option<CString>) -> Self {
        let mut config = ConnectionConfig {
            handle: LEAP_CONNECTION_CONFIG {
                size: std::mem::size_of::<LEAP_CONNECTION_CONFIG>() as u32,
                flags: flags.bits() as u32,
                server_namespace: std::ptr::null(),
            },
            server_namespace: namespace,
        };

        if let Some(namespace) = &config.server_namespace {
            config.handle.server_namespace = namespace.as_ptr()
        }

        config
    }
}
