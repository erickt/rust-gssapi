use gssapi_sys;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::slice;

pub struct Buffer {
    buffer_desc: gssapi_sys::gss_buffer_desc,
    owned_by_rust: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            buffer_desc: gssapi_sys::gss_buffer_desc {
                length: 0,
                value: ptr::null_mut(),
            },
            owned_by_rust: false,
        }
    }

    pub fn len(&self) -> usize {
        self.buffer_desc.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub unsafe fn get_handle(&mut self) -> gssapi_sys::gss_buffer_t {
        &mut self.buffer_desc
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(buffer: Vec<u8>) -> Self {
        let buffer_desc = gssapi_sys::gss_buffer_desc {
            length: buffer.len(),
            value: buffer.as_ptr() as *mut c_void,
        };

        // Ownership has been moved into `Buffer`.
        mem::forget(buffer);

        Buffer {
            buffer_desc: buffer_desc,
            owned_by_rust: true,
        }
    }
}

impl From<CString> for Buffer {
    fn from(buffer: CString) -> Self {
        Buffer::from(buffer.into_bytes_with_nul())
    }
}

impl<'a> From<&'a str> for Buffer {
    fn from(buffer: &'a str) -> Buffer {
        // Copy the buffer into a C string to make sure it has a trailing nul.
        Buffer::from(CString::new(buffer).unwrap())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.owned_by_rust {
            unsafe {
                let slice = slice::from_raw_parts(self.buffer_desc.value, self.buffer_desc.length);
                let _: Box<[u8]> = mem::transmute(slice);
            }
        } else {
            let mut min_stat = 0;
            let maj_stat = unsafe {
                gssapi_sys::gss_release_buffer(&mut min_stat, &mut self.buffer_desc)
            };

            if maj_stat != gssapi_sys::GSS_S_COMPLETE {
                panic!("failed to release buffer");
            }
        }
    }
}

pub struct BufferRef {
    buffer: gssapi_sys::gss_buffer_t,
}

impl BufferRef {
    pub fn new(buffer: gssapi_sys::gss_buffer_t) -> BufferRef {
        BufferRef {
            buffer: buffer,
        }
    }

    pub unsafe fn get_handle(&mut self) -> gssapi_sys::gss_buffer_t {
        self.buffer
    }
}

impl Drop for BufferRef {
    fn drop(&mut self) {
        let mut min_stat = 0;
        let maj_stat = unsafe {
            gssapi_sys::gss_release_buffer(&mut min_stat, self.buffer)
        };

        if maj_stat != gssapi_sys::GSS_S_COMPLETE {
            panic!("failed to release buffer");
        }
    }
}

