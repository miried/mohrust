use crate::client as cl;
use libc::intptr_t;
use std::ffi::CStr;

use cl::uiImport_t;

bitflags! {
    pub struct CvarFlags: u32 {
        const ARCHIVE      = 0b00000000001;
        const USERINFO     = 0b00000000010;
        const SERVERINFO   = 0b00000000100;
        const SYSTEMINFO   = 0b00000001000;
        const INIT         = 0b00000010000;
        const LATCH        = 0b00000100000;
        const ROM          = 0b00001000000;
        const USER_CREATED = 0b00010000000;
        const TEMP         = 0b00100000000;
        const CHEAT        = 0b01000000000;
        const NORESTART    = 0b10000000000;
    }
}

/* CVAR definitions */
const MAX_CVAR_VALUE_STRING : usize = 256;

#[allow(non_camel_case_types)]
type cvarHandle_t = i32;

#[derive(Default, Debug)]
pub struct Cvar {
    last_mod_count : i32,
    value_string : String,
    vm_cvar : vmCvar,
}

impl Cvar {
    pub fn _register( name : &str, value : &str, flags : Option<CvarFlags>) -> Self {
        let mut cvar = Self::default();
        let flagbits = flags.unwrap_or(CvarFlags::empty());

        _register(&mut cvar.vm_cvar, name, value, flagbits.bits);
        cvar
    }

    pub fn _value_str(&self) -> String {
        let cstr = unsafe{CStr::from_ptr(self.vm_cvar.string.as_ptr())};
        cstr.to_str().expect("CStr conversion failed.").to_owned()
    }

    pub fn _update(&mut self) {
        _update(&mut self.vm_cvar);
    }

    pub fn _has_changed(&mut self) -> bool {
        if self.vm_cvar.modificationCount != self.last_mod_count {
            self.last_mod_count = self.vm_cvar.modificationCount;
            return true
        }

        false
    }
}
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[derive(Debug)]
struct vmCvar {
	handle : cvarHandle_t,
	modificationCount : i32,
	value : f32,
	integer : i32,
	string : [i8; MAX_CVAR_VALUE_STRING]
}

impl Default for vmCvar {
    fn default() -> Self {
        Self {
            handle : 0,
            modificationCount : 0,
            value : 0.0,
            integer : 0,
            string : [0; MAX_CVAR_VALUE_STRING]
        }
    }
}

fn _register(cvar : &mut vmCvar, var_name : &str, value : &str, flags : u32) {
    let c_var_name = cl::create_cstring(var_name);
    let c_value = cl::create_cstring(value);
    unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_REGISTER as intptr_t, cvar, c_var_name.as_ptr(), c_value.as_ptr(), flags)};
}

fn _update(cvar : &mut vmCvar) {
    unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_UPDATE as intptr_t, cvar)};
}

pub fn _set(var_name : &str, value : &str) {
    let c_var_name = cl::create_cstring(var_name);
    let c_value = cl::create_cstring(value);
    unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_SET as intptr_t, c_var_name.as_ptr(), c_value.as_ptr())};
}

pub fn _create(var_name : &str, value : &str, flags : i32) {
    let c_var_name = cl::create_cstring(var_name);
    let c_value = cl::create_cstring(value);
    unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_CREATE as intptr_t, c_var_name.as_ptr(), c_value.as_ptr(), flags)};
}

pub fn value_float(var_name : &str) -> f32 {
    let c_var_name = cl::create_cstring(var_name);
    let value = unsafe{cl::SYSCALL(uiImport_t::UI_CVAR_VARIABLEVALUE as intptr_t, c_var_name.as_ptr())};
    let fi = cl::floatint_t { i : value as i32 };
    unsafe{ fi.f }
}

fn _value_string( var_name : &str ) -> String {
    let mut buffer = [0 as i8; MAX_CVAR_VALUE_STRING];
    let c_var_name = cl::create_cstring(var_name);

    let result = unsafe {
        cl::SYSCALL(uiImport_t::UI_CVAR_VARIABLESTRINGBUFFER as intptr_t, c_var_name.as_ptr(), buffer.as_mut_ptr(), buffer.len());
        CStr::from_ptr(buffer.as_ptr())
    };
    
    result.to_str().expect("CStr conversion failed.").to_owned()
}
