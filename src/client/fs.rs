use std::convert::TryFrom;

use crate::client as cl;
use crate::ui_println;
use libc::intptr_t;

use cl::uiImport_t;

pub struct FileHandle {
    fp : fileHandle_t,
    pub length : isize,
}

impl TryFrom<&String> for FileHandle {
    type Error = &'static str;

    fn try_from(value : &String) -> Result<Self, Self::Error> {
        let mut f : fileHandle_t = 0;
        let length = fopen_file(value, &mut f, fsMode_t::FS_READ);
        match f {
            0 => Err("Could not open file."),
            _ => {
                let fh = FileHandle {
                    fp: f,
                    length,
                };
                ui_println!("opened file {} (len {}).", f, length);
                Ok(fh)
            }
        }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        fclose_file(self.fp);
    }
}

#[allow(non_camel_case_types)]
type fileHandle_t = i32;

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
pub enum fsMode_t {
    FS_READ,
    FS_WRITE,
    FS_APPEND,
    FS_APPEND_SYNC
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
pub enum fsOrigin_t {
    FS_SEEK_CUR,
    FS_SEEK_END,
    FS_SEEK_SET
}

fn fopen_file(qpath : &str, f : &mut fileHandle_t, mode : fsMode_t) -> isize {  
    let (_c_qpath, c_qpath_ptr) = cl::create_cstringptr(qpath);
    let length = unsafe{cl::SYSCALL(uiImport_t::UI_FS_FOPENFILE as intptr_t, c_qpath_ptr, f, mode)};
    length
}

fn fclose_file(f : fileHandle_t) {
    unsafe{cl::SYSCALL(uiImport_t::UI_FS_FCLOSEFILE as intptr_t, f)};
    ui_println!("closed file {}.", f);
}

fn read(buffer : *mut u8, len : isize, f : fileHandle_t) {
    unsafe{cl::SYSCALL(uiImport_t::UI_FS_READ as intptr_t, buffer, len, f)};
}
