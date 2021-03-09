use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;

use crate::client as cl;
use libc::intptr_t;

use cl::uiImport_t;

pub struct FileHandle {
    file_handle : fileHandle_t,
    length : usize,
}

impl TryFrom<&String> for FileHandle {
    type Error = &'static str;

    fn try_from(value : &String) -> Result<Self, Self::Error> {
        let mut file_handle : fileHandle_t = 0;
        let length = cl_fopen_file(value, &mut file_handle, fsMode_t::FS_READ);
        match file_handle {
            0 => Err("Could not open file."),
            _ => {
                let fh = FileHandle {
                    file_handle,
                    length,
                };
                Ok(fh)
            }
        }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        cl_fclose_file(self.file_handle);
    }
}

impl FileHandle {
    /// The returned buffer has the size of the file,
    /// but it has capacity for one more byte, in case
    /// we want to null-terminate it. 
    pub fn read(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.length+1);
        buffer.resize(self.length, 0);
        cl_read(buffer.as_mut_ptr(), self.length, self.file_handle);
        buffer
    }

    pub fn read_text(&self) -> String {
        let mut buffer = self.read();
        // we need a 0-byte at the end of the file buffer
        buffer.resize(self.length+1, 0);
        let cstr = CStr::from_bytes_with_nul(&buffer).expect("CStr conversion failed.");
        let converted = cstr.to_str().expect("str conversion failed.");
        converted.to_owned()
    }
}

#[allow(non_camel_case_types)]
type fileHandle_t = i32;

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
enum fsMode_t {
    FS_READ,
    FS_WRITE,
    FS_APPEND,
    FS_APPEND_SYNC
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
enum fsOrigin_t {
    FS_SEEK_CUR,
    FS_SEEK_END,
    FS_SEEK_SET
}

fn cl_fopen_file(qpath : &str, f : &mut fileHandle_t, mode : fsMode_t) -> usize {  
    let (_c_qpath, c_qpath_ptr) = cl::create_cstringptr(qpath);
    let length = unsafe{cl::SYSCALL(uiImport_t::UI_FS_FOPENFILE as intptr_t, c_qpath_ptr, f, mode)};
    length.try_into().expect("Returned file length negative.")
}

fn cl_fclose_file(f : fileHandle_t) {
    unsafe{cl::SYSCALL(uiImport_t::UI_FS_FCLOSEFILE as intptr_t, f)};
}

fn cl_read(buffer : *mut u8, len : usize, f : fileHandle_t) {
    unsafe{cl::SYSCALL(uiImport_t::UI_FS_READ as intptr_t, buffer, len, f)};
}
