/* BASIC ENGINE FUNCTIONS */
use crate::client as cl;
use libc::intptr_t;
use std::ffi::CStr;

use cl::uiImport_t;

#[macro_export]
macro_rules! ui_print {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        $crate::client::util::print(&res);
    }}
}

#[macro_export]
macro_rules! ui_println {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        $crate::client::util::println(&res);
    }}
}

/// Print error message and quit the program.
pub fn _error(text: &str) {
    let c_text = cl::create_cstring(text);
    unsafe{cl::SYSCALL(uiImport_t::UI_ERROR as intptr_t,c_text.as_ptr())};
    panic!("Unrecoverable error occurred.")
}

/// Print console message.
pub fn print(text: &str) {
    let c_text = cl::create_cstring(text);
    unsafe{cl::SYSCALL(uiImport_t::UI_PRINT as intptr_t,c_text.as_ptr())};
}

/// Print console message line.
pub fn println(text: &str) {
    let text_line = format!("{}\n", text);
    print(&text_line);
}

/// Execution time.
pub fn _milliseconds() -> isize {
    unsafe{cl::SYSCALL(uiImport_t::UI_MILLISECONDS as intptr_t)}
}

pub fn argc() -> isize {
    unsafe{cl::SYSCALL(uiImport_t::UI_ARGC as intptr_t)}
}

pub fn argv( n : i32 ) -> String {
    let mut buffer = [0 as i8; 1024];

    let result = unsafe{
        cl::SYSCALL(uiImport_t::UI_ARGV as intptr_t, n, buffer.as_mut_ptr(), buffer.len());
        CStr::from_ptr(buffer.as_ptr())
    };
    
    result.to_str().expect("CStr conversion failed.").to_owned()
}

pub fn cmd_execute( text : &str ) {
    cmd_executetext(cbufExec_t::EXEC_APPEND, text)
}

fn cmd_executetext( exec_when : cbufExec_t, text : &str) {
    let c_text = cl::create_cstring(text);
    let ew = exec_when as i32;
    unsafe{cl::SYSCALL(uiImport_t::UI_CMD_EXECUTETEXT as intptr_t, ew, c_text.as_ptr())};
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]// parameters for command buffer stuffing
enum cbufExec_t {
    EXEC_NOW,                       // don't return until completed, a VM should NEVER use this,
                                            // because some commands might cause the VM to be unloaded...
    EXEC_INSERT,            // insert at current position, but don't run yet
    EXEC_APPEND                     // add to end of the command buffer (normal case)
}
