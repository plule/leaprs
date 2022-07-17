use leap_sys::*;
use num_enum::IntoPrimitive;

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive)]
#[repr(i32)]
#[doc = " Defines the parameters used to access version information."]
#[doc = " @since 5.2.x"]
pub enum VersionPart {
    #[doc = " The parameter for requesting the version of the client."]
    #[doc = " @since 5.2.x"]
    ClientLibrary = _eLeapVersionPart_eLeapVersionPart_ClientLibrary,
    #[doc = " The parameter for requesting the protocol version of the client."]
    #[doc = " @since 5.2.x"]
    ClientProtocol = _eLeapVersionPart_eLeapVersionPart_ClientProtocol,
    #[doc = " The parameter for requesting the version of the server."]
    #[doc = " @since 5.2.x"]
    ServerLibrary = _eLeapVersionPart_eLeapVersionPart_ServerLibrary,
    #[doc = " The parameter for requesting the protocol version of the server."]
    #[doc = " @since 5.2.x"]
    ServerProtocol = _eLeapVersionPart_eLeapVersionPart_ServerProtocol,
}
