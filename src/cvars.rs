use crate::syscalls;

/// Register cvar
/// TODO: implement the other cvar functions
/// Keep vmCar_t instance private .. is it even needed for our purpose?
pub fn _register(cvar : &vmCvar_t, var_name : &str, value : &str, flags : i32) {
    let syscall = syscalls::get_syscall();
    
    let n = syscalls::convert_str_to_cstring(var_name);
    let v = syscalls::convert_str_to_cstring(value);

    syscall(syscalls::uiImport_t::UI_CVAR_REGISTER as isize, cvar, n.as_ptr(), v.as_ptr(), flags);
}


const MAX_CVAR_VALUE_STRING : usize = 256;

#[allow(non_camel_case_types)]
type cvarHandle_t = i32;

#[repr(C)]
#[allow(non_snake_case, dead_code)]
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