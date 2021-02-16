use crate::client as cl;
use libc::intptr_t;

use cl::uiImport_t;

#[repr(C)]
#[allow(dead_code)]
union floatint_t {
    f: f32,
    i: i32,
    ui: u32
}

pub fn _set(var_name : &str, value : &str) {
    let (_c_var_name, c_var_name_ptr) = cl::create_cstringptr(var_name);
    let (_c_value, c_value_ptr) = cl::create_cstringptr(value);
    unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_SET as intptr_t, c_var_name_ptr, c_value_ptr)};
}

pub fn create(var_name : &str, value : &str, flags : i32) {
    let (_c_var_name, c_var_name_ptr) = cl::create_cstringptr(var_name);
    let (_c_value, c_value_ptr) = cl::create_cstringptr(value);
    unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_CREATE as intptr_t, c_var_name_ptr, c_value_ptr, flags)};
}

pub fn _variable_value(var_name : &str) -> f32 {
    let (_c_var_name, c_var_name_ptr) = cl::create_cstringptr(var_name);
    let value = unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_SET as intptr_t, c_var_name_ptr)};
    let fi = floatint_t { i : value as i32 };
    unsafe{ fi.f }
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