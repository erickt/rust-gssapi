use gssapi_sys;
use std::ffi::CString;
use std::mem;
use std::ops;
use std::os::raw::c_void;
use std::ptr;
use std::slice;

#[derive(Debug)]
pub struct Buffer {
    buffer_desc: gssapi_sys::gss_buffer_desc,
    kind: BufferKind,
}

#[derive(Debug)]
enum BufferKind {
    Rust {
        capacity: usize,
    },
    //GSSAPI,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            buffer_desc: gssapi_sys::gss_buffer_desc {
                length: 0,
                value: ptr::null_mut(),
            },
            kind: BufferKind::Rust {
                capacity: 0,
            },
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self.kind {
            BufferKind::Rust { capacity } => {
                unsafe {
                    Vec::from_raw_parts(
                        self.buffer_desc.value as *mut u8,
                        self.buffer_desc.length,
                        capacity,
                    )
                }
            }
            /*
            BufferKind::GSSAPI => {
                let vec = unsafe {
                    slice::from_raw_parts(
                        self.buffer_desc.value as *mut u8,
                        self.buffer_desc.length
                    ).to_owned()
                };

                let mut minor_status = 0;

                let major_status = unsafe {
                    gssapi_sys::gss_release_buffer(
                        &mut minor_status,
                        &mut self.buffer_desc)
                };

                if major_status == gssapi_sys::GSS_S_COMPLETE {
                    Ok(vec)
                } else {
                    Err(Error::new(major_status, minor_status))
                }
            }
            */
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self.kind {
            BufferKind::Rust { .. } => {
                unsafe {
                    slice::from_raw_parts(
                        self.buffer_desc.value as *mut u8,
                        self.buffer_desc.length,
                    )
                }
            }
            /*
            BufferKind::GSSAPI => {
                let vec = unsafe {
                    slice::from_raw_parts(
                        self.buffer_desc.value as *mut u8,
                        self.buffer_desc.length
                    ).to_owned()
                };

                let mut minor_status = 0;

                let major_status = unsafe {
                    gssapi_sys::gss_release_buffer(
                        &mut minor_status,
                        &mut self.buffer_desc)
                };

                if major_status == gssapi_sys::GSS_S_COMPLETE {
                    Ok(vec)
                } else {
                    Err(Error::new(major_status, minor_status))
                }
            }
            */
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        match self.kind {
            BufferKind::Rust { .. } => {
                unsafe {
                    let slice = slice::from_raw_parts(
                        self.buffer_desc.value,
                        self.buffer_desc.length,
                    );
                    let _: Box<[u8]> = mem::transmute(slice);
                }
            }
            /*
            BufferKind::GSSAPI => {
                let mut min_stat = 0;
                let maj_stat = unsafe {
                    gssapi_sys::gss_release_buffer(&mut min_stat, &mut self.buffer_desc)
                };

                if maj_stat != gssapi_sys::GSS_S_COMPLETE {
                    panic!("failed to release buffer");
                }
            }
            */
        }
    }
}

impl ops::Deref for Buffer {
    type Target = BufferRef;

    fn deref(&self) -> &BufferRef {
        unsafe {
            mem::transmute(&self.buffer_desc)
        }
    }
}

impl ops::DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut BufferRef {
        unsafe {
            mem::transmute(&mut self.buffer_desc)
        }
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(buffer: Vec<u8>) -> Self {
        let buffer_desc = gssapi_sys::gss_buffer_desc {
            length: buffer.len(),
            value: buffer.as_ptr() as *mut c_void,
        };
        let capacity = buffer.capacity();

        // Ownership has been moved into `Buffer`.
        mem::forget(buffer);

        Buffer {
            buffer_desc: buffer_desc,
            kind: BufferKind::Rust { capacity: capacity },
        }
    }
}

impl From<CString> for Buffer {
    fn from(buffer: CString) -> Self {
        Buffer::from(buffer.into_bytes())
    }
}

impl From<String> for Buffer {
    fn from(buffer: String) -> Self {
        Buffer::from(buffer.into_bytes())
    }
}

pub struct BufferRef {
    buffer_desc: gssapi_sys::gss_buffer_desc,
}

impl BufferRef {
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

/*
impl<'a> From<&'a [u8]> for BufferRef<'a> {
    fn from(buffer: &'a [u8]) -> Self {
        let buffer_desc = gssapi_sys::gss_buffer_desc {
            length: buffer.len(),
            value: buffer.as_ptr() as *mut c_void,
        };

        BufferRef {
            buffer_desc: buffer_desc,
            _phantom_data: PhantomData,
        }
    }
}


impl<'a> From<&'a str> for BufferRef {
    fn from(buffer: &'a str) -> Buffer {
        Buffer::from(buffer.as_slice())
    }
}

/*
pub struct BufferRef {
    buffer: gssapi_sys::gss_buffer_t,
}

impl BufferRef {
    pub fn new(buffer: gssapi_sys::gss_buffer_t) -> BufferRef {
        BufferRef {
            buffer: buffer,
        }
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
*/
*/
