/* BASIC ENGINE FUNCTIONS */
use crate::client as cl;
use libc::intptr_t;

use cl::uiImport_t;

/// Print error message and quit the program.
pub fn _error(text: &str) {
    let (_c_text, c_text_ptr) = cl::create_cstringptr(text);
    unsafe{cl::SYSCALL(uiImport_t::UI_ERROR as intptr_t,c_text_ptr)};
    panic!("Unrecoverable error occurred.")
}

/// Print console message.
pub fn print(text: &str) {
    let (_c_text, c_text_ptr) = cl::create_cstringptr(text);
    unsafe{cl::SYSCALL(uiImport_t::UI_PRINT as intptr_t,c_text_ptr)};
}

/// Execution time.
pub fn milliseconds() -> isize {
    unsafe{cl::SYSCALL(uiImport_t::UI_MILLISECONDS as intptr_t)}
}

pub fn _argc() -> isize {
    unsafe{cl::SYSCALL(uiImport_t::UI_ARGC as intptr_t)}
}

