use gssapi_sys;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::slice;

pub struct Buffer {
    buffer_desc: gssapi_sys::gss_buffer_desc,
}

impl Buffer {
    pub fn from_vec(buffer: Vec<u8>) -> Self {
        let buffer_desc = gssapi_sys::gss_buffer_desc {
            length: buffer.len(),
            value: buffer.as_ptr() as *mut c_void,
        };

        // Ownership has been moved into `Buffer`.
        mem::forget(buffer);

        Buffer {
            buffer_desc: buffer_desc,
        }
    }

    pub fn from_str(buffer: &str) -> Buffer {
        // Copy the buffer into a C string to make sure it has a trailing nul.
        let c_string = CString::new(buffer).unwrap();
        Buffer::from_vec(c_string.into_bytes_with_nul())
    }

    pub unsafe fn get_handle(&mut self) -> gssapi_sys::gss_buffer_t {
        &mut self.buffer_desc
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            let slice = slice::from_raw_parts(self.buffer_desc.value, self.buffer_desc.length);
            let _: Box<[u8]> = mem::transmute(slice);
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

