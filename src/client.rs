pub mod util;
pub mod cvar;
pub mod fs;

use libc::intptr_t;
use std::ffi::CString;

/*
SYSCALL LOGIC

OK, I know this is totally against everything Rust was designed for.
But we try to be compatible to the original engine interface here.

There is no usable local state to keep the syscallptr, so it is global.
Since its lifetime is from first call to dllEntry to the end, it is
essentially 'static lifetime.

Suggestions for improvement welcome.
*/

type SyscallFn = extern "C" fn(intptr_t, ...) -> intptr_t;

/// In case we get called before a valid syscallptr has arrived.
fn uninit_syscall(_ : intptr_t) -> intptr_t {
    panic!("UI syscall done before syscallptr was set.")
}

static UNINIT_SYSCALLPTR : fn(isize)->isize = uninit_syscall;

/// The engine will give us the syscall function pointer before we can use it.
/// 
/// We rely on the engine behaving correctly.
pub static mut SYSCALL : SyscallFn = unsafe{std::mem::transmute(UNINIT_SYSCALLPTR)};

/// Set the syscall function address.
pub unsafe fn set_syscallptr(syscallptr : intptr_t) {
    SYSCALL = std::mem::transmute(syscallptr);
}

/* HELPER FUNCTIONS */

/// This is a helper to get a pointer to a C string.
pub fn create_cstring(input : &str) -> CString {
    CString::new(input).expect("Could not convert text to CString.")
}


#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
pub enum uiImport_t {
    UI_ERROR,
    UI_PRINT,
    UI_MILLISECONDS,
    UI_CVAR_SET,
    UI_CVAR_VARIABLEVALUE,
    UI_CVAR_VARIABLESTRINGBUFFER,
    UI_CVAR_SETVALUE,
    UI_CVAR_RESET,
    UI_CVAR_CREATE,
    UI_CVAR_INFOSTRINGBUFFER,
    UI_ARGC,
    UI_ARGV,
    UI_CMD_EXECUTETEXT,
    UI_FS_FOPENFILE,
    UI_FS_READ,
    UI_FS_WRITE,
    UI_FS_FCLOSEFILE,
    UI_FS_GETFILELIST,
    UI_R_REGISTERMODEL,
    UI_R_REGISTERSKIN,
    UI_R_REGISTERSHADERNOMIP,
    UI_R_CLEARSCENE,
    UI_R_ADDREFENTITYTOSCENE,
    UI_R_ADDPOLYTOSCENE,
    UI_R_ADDLIGHTTOSCENE,
    UI_R_RENDERSCENE,
    UI_R_SETCOLOR,
    UI_R_DRAWSTRETCHPIC,
    UI_UPDATESCREEN,
    UI_CM_LERPTAG,
    UI_CM_LOADMODEL,
    UI_S_REGISTERSOUND,
    UI_S_STARTLOCALSOUND,
    UI_KEY_KEYNUMTOSTRINGBUF,
    UI_KEY_GETBINDINGBUF,
    UI_KEY_SETBINDING,
    UI_KEY_ISDOWN,
    UI_KEY_GETOVERSTRIKEMODE,
    UI_KEY_SETOVERSTRIKEMODE,
    UI_KEY_CLEARSTATES,
    UI_KEY_GETCATCHER,
    UI_KEY_SETCATCHER,
    UI_GETCLIPBOARDDATA,
    UI_GETGLCONFIG,
    UI_GETCLIENTSTATE,
    UI_GETCONFIGSTRING,
    UI_LAN_GETPINGQUEUECOUNT,
    UI_LAN_CLEARPING,
    UI_LAN_GETPING,
    UI_LAN_GETPINGINFO,
    UI_CVAR_REGISTER,
    UI_CVAR_UPDATE,
    UI_MEMORY_REMAINING,
    UI_GET_CDKEY,
    UI_SET_CDKEY,
    UI_R_REGISTERFONT,
    UI_R_MODELBOUNDS,
    UI_PC_ADD_GLOBAL_DEFINE,
    UI_PC_LOAD_SOURCE,
    UI_PC_FREE_SOURCE,
    UI_PC_READ_TOKEN,
    UI_PC_SOURCE_FILE_AND_LINE,
    UI_S_STOPBACKGROUNDTRACK,
    UI_S_STARTBACKGROUNDTRACK,
    UI_REAL_TIME,
    UI_LAN_GETSERVERCOUNT,
    UI_LAN_GETSERVERADDRESSSTRING,
    UI_LAN_GETSERVERINFO,
    UI_LAN_MARKSERVERVISIBLE,
    UI_LAN_UPDATEVISIBLEPINGS,
    UI_LAN_RESETPINGS,
    UI_LAN_LOADCACHEDSERVERS,
    UI_LAN_SAVECACHEDSERVERS,
    UI_LAN_ADDSERVER,
    UI_LAN_REMOVESERVER,
    UI_CIN_PLAYCINEMATIC,
    UI_CIN_STOPCINEMATIC,
    UI_CIN_RUNCINEMATIC,
    UI_CIN_DRAWCINEMATIC,
    UI_CIN_SETEXTENTS,
    UI_R_REMAP_SHADER,
    UI_VERIFY_CDKEY,
    UI_LAN_SERVERSTATUS,
    UI_LAN_GETSERVERPING,
    UI_LAN_SERVERISVISIBLE,
    UI_LAN_COMPARESERVERS,
    // 1.32
    UI_FS_SEEK,
    UI_SET_PBCLSTATUS,

    UI_MEMSET = 100,
    UI_MEMCPY,
    UI_STRNCPY,
    UI_SIN,
    UI_COS,
    UI_ATAN2,
    UI_SQRT,
    UI_FLOOR,
    UI_CEIL
}
