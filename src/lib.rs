extern crate libc;
use libc::{c_int, intptr_t};
#[macro_use]
extern crate bitflags;

mod client;
mod q3ui;
mod menu;
mod q3common;
mod console;

/// When loading the library, the engine will first call dllEntry
/// So that we know the syscallptr to call functions from the library.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn dllEntry( syscallptr : intptr_t ) {
	client::set_syscallptr(syscallptr)
}

/// This is the gateway function for the engine to trigger events in the library.
#[allow(non_snake_case, unused_variables)]
#[no_mangle]
pub extern "C" fn vmMain( command: c_int, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int, arg9: c_int, arg10: c_int, arg11: c_int) -> intptr_t {
	let cmd : uiExport_t = unsafe { std::mem::transmute(command) };

	//ui_println!("cmd {:?}", cmd);
	
	let result = match cmd {
		uiExport_t::UI_GETAPIVERSION => q3ui::UI_APIVERSION,
		uiExport_t::UI_INIT => q3ui::init(arg0 != 0),
		uiExport_t::UI_SHUTDOWN => q3ui::shutdown(),
		uiExport_t::UI_KEY_EVENT => q3ui::key_event(arg0, arg1 != 0),
		uiExport_t::UI_MOUSE_EVENT => q3ui::mouse_event(arg0, arg1),
		uiExport_t::UI_REFRESH => q3ui::refresh(arg0),
		uiExport_t::UI_IS_FULLSCREEN => q3ui::is_fullscreen() as i32,
		uiExport_t::UI_SET_ACTIVE_MENU => q3ui::set_active_menu(arg0),
		uiExport_t::UI_CONSOLE_COMMAND => q3ui::console_command(arg0) as i32,
		uiExport_t::UI_DRAW_CONNECT_SCREEN => q3ui::draw_connect_screen(arg0 != 0),
		uiExport_t::UI_HASUNIQUECDKEY => q3ui::has_unique_cdkey() as i32,
	};
	result as intptr_t
}

#[allow(non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug)]
enum uiExport_t {
	UI_GETAPIVERSION = 0,	// system reserved

	UI_INIT,
//	void	UI_Init( void );

	UI_SHUTDOWN,
//	void	UI_Shutdown( void );

	UI_KEY_EVENT,
//	void	UI_KeyEvent( int key );

	UI_MOUSE_EVENT,
//	void	UI_MouseEvent( int dx, int dy );

	UI_REFRESH,
//	void	UI_Refresh( int time );

	UI_IS_FULLSCREEN,
//	qboolean UI_IsFullscreen( void );

	UI_SET_ACTIVE_MENU,
//	void	UI_SetActiveMenu( uiMenuCommand_t menu );

	UI_CONSOLE_COMMAND,
//	qboolean UI_ConsoleCommand( int realTime );

	UI_DRAW_CONNECT_SCREEN,
//	void	UI_DrawConnectScreen( qboolean overlay );
	UI_HASUNIQUECDKEY
// if !overlay, the background will be drawn, otherwise it will be
// overlayed over whatever the cgame has drawn.
// a GetClientState syscall will be made to get the current strings
}
