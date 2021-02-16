use crate::client as cl;

pub const UI_APIVERSION : i32 = 6;

pub fn init(_in_game_load : bool) -> i32 {
	let a = cl::util::milliseconds();
	let b = format!("Rusty UI_INIT at {}ms.\n", a);
	cl::util::print(&b);

	cl::cvar::create("ui_wombat", "0", 0);

	0
}

pub fn shutdown() -> i32 {
	0
}

pub fn key_event(_key : i32, _down : bool) -> i32 {
	0
}
pub fn mouse_event(_x : i32, _y : i32) -> i32 {
	0
}
pub fn refresh(_realtime : i32) -> i32 {
	0
}
pub fn is_fullscreen() -> i32 {
	0
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
enum uiMenuCommand_t {
	UIMENU_NONE,
	UIMENU_MAIN,
	UIMENU_INGAME,
	UIMENU_NEED_CD,
	UIMENU_BAD_CD_KEY,
	UIMENU_TEAM,
	UIMENU_POSTGAME
}

pub fn set_active_menu(menu : i32) -> i32 {
	let _menu_command : uiMenuCommand_t = unsafe { std::mem::transmute(menu) };
	0
}

pub fn console_command(_realtime : i32) -> i32 {
	0
}

pub fn draw_connect_screen(_overlay : bool) -> i32 {
	0
}

pub fn has_unique_cdkey() -> i32 {
	0
}
