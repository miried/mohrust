/* BASIC ENGINE FUNCTIONS */
use crate::client as cl;
use libc::intptr_t;

use cl::uiImport_t;

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

/// Execution time.
pub fn _milliseconds() -> isize {
    unsafe{cl::SYSCALL(uiImport_t::UI_MILLISECONDS as intptr_t)}
}

pub fn _argc() -> isize {
    unsafe{cl::SYSCALL(uiImport_t::UI_ARGC as intptr_t)}
}

