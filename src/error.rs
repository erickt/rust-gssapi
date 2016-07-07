use buffer::Buffer;
use gssapi_sys;
use std::error;
use std::fmt;
use std::ptr;
use std::result;

#[derive(Debug)]
pub struct Error {
    major_status: u32,
    minor_status: u32,
}

impl Error {
    pub fn new(major_status: u32, minor_status: u32) -> Self {
        Error {
            major_status: major_status,
            minor_status: minor_status,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "GSSAPI Error"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut message_context = 0;

        loop {
            let mut minor_status = 0;
            let status_value = 0;
            let status_type = gssapi_sys::GSS_C_GSS_CODE;
            let mech_type = ptr::null_mut();
            let mut status_string = Buffer::new();

            let major_status = unsafe {
                gssapi_sys::gss_display_status(
                    &mut minor_status,
                    status_value,
                    status_type,
                    mech_type,
                    &mut message_context,
                    status_string.get_handle(),
                )
            };

            assert_eq!(major_status, gssapi_sys::GSS_S_COMPLETE);

            if let Err(err) = write!(f, "{:?}\n", self) {
                return Err(err);
            }

            if message_context != 0 {
                break;
            }
        }

        Ok(())
    }
}

pub type Result<T> = result::Result<T, Error>;
