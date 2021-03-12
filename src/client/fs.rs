use std::convert::{TryFrom, TryInto};

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
    pub fn read(&self) -> Vec<u8> {
        cl_read(self.length, self.file_handle)
    }

    pub fn read_text(&self) -> String {
        let buffer = self.read();
        String::from_utf8(buffer).expect("String conversion from file failed.")
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
    let c_qpath = cl::create_cstring(qpath);
    let length = unsafe{cl::SYSCALL(uiImport_t::UI_FS_FOPENFILE as intptr_t, c_qpath.as_ptr(), f, mode)};
    length.try_into().expect("Returned file length negative.")
}

fn cl_fclose_file(f : fileHandle_t) {
    unsafe{cl::SYSCALL(uiImport_t::UI_FS_FCLOSEFILE as intptr_t, f)};
}

fn cl_read(len : usize, f : fileHandle_t) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(len);

    unsafe{
        cl::SYSCALL(uiImport_t::UI_FS_READ as intptr_t, buffer.as_mut_ptr(), len, f);
        // FIXME: the engine does not tell us how many bytes were actually read!
        buffer.set_len(len);
    }
    buffer
}
