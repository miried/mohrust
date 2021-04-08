use crate::client as cl;
use libc::intptr_t;
use std::ffi::CStr;

use cl::uiImport_t;

pub fn _set(var_name : &str, value : &str) {
    let c_var_name = cl::create_cstring(var_name);
    let c_value = cl::create_cstring(value);
    unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_SET as intptr_t, c_var_name.as_ptr(), c_value.as_ptr())};
}

pub fn create(var_name : &str, value : &str, flags : i32) {
    let c_var_name = cl::create_cstring(var_name);
    let c_value = cl::create_cstring(value);
    unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_CREATE as intptr_t, c_var_name.as_ptr(), c_value.as_ptr(), flags)};
}

pub fn _variable_value(var_name : &str) -> f32 {
    let c_var_name = cl::create_cstring(var_name);
    let value = unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_SET as intptr_t, c_var_name.as_ptr())};
    let fi = cl::floatint_t { i : value as i32 };
    unsafe{ fi.f }
}

pub fn _variable_string_buffer(var_name : &str) -> String {
    let mut buffer = [0 as i8; MAX_CVAR_VALUE_STRING];
    let c_var_name = cl::create_cstring(var_name);

    let result = unsafe {
        cl::SYSCALL(uiImport_t::UI_CVAR_VARIABLESTRINGBUFFER as intptr_t, c_var_name.as_ptr(), buffer.as_mut_ptr(), MAX_CVAR_VALUE_STRING);
        CStr::from_ptr(buffer.as_ptr())
    };
    
    result.to_str().expect("CStr conversion failed.").to_owned()
}

bitflags! {
    struct CvarFlags: u32 {
        const ARCHIVE =      0b00000000001;
        const USERINFO =     0b00000000010;
        const SERVERINFO =   0b00000000100;
        const SYSTEMINFO =   0b00000001000;
        const INIT =         0b00000010000;
        const LATCH =        0b00000100000;
        const ROM =          0b00001000000;
        const USER_CREATED = 0b00010000000;
        const TEMP =         0b00100000000;
        const CHEAT =        0b01000000000;
        const NORESTART =    0b10000000000;
    }
}

/* CVAR definitions */
const MAX_CVAR_VALUE_STRING : usize = 256;

#[allow(non_camel_case_types)]
type cvarHandle_t = i32;

#[repr(C)]
#[allow(non_snake_case, non_camel_case_types, dead_code)]
pub struct vmCvar_t {
	handle : cvarHandle_t,
	modificationCount : i32,
	value : f32,
	integer : i32,
	string : [u8; MAX_CVAR_VALUE_STRING]
}

impl Default for vmCvar_t {
    fn default() -> Self {
        vmCvar_t {
            handle : 0,
            modificationCount : 0,
            value : 0.0,
            integer : 0,
            string : [0; MAX_CVAR_VALUE_STRING]
        }
    }
}