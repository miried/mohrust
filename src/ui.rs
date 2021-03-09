use std::sync::Mutex;
use menu::MenuConfig;
use once_cell::sync::OnceCell;

mod menu;
mod urc;

use crate::client::cvar;
use crate::ui_println;


pub const UI_APIVERSION : i32 = 6;

static MENUCONFIG : OnceCell<Mutex<MenuConfig>> = OnceCell::new();

fn set_menuconfig(mc : MenuConfig) {
	let result = MENUCONFIG.set(Mutex::new(mc));
	result.expect("Could not initialize MENUCONFIG, already done before.");
	// TODO: this crashes on vid_restart. we need a proper shutdown sequence, to gracefully handle a restart.
}

pub fn init(_in_game_load : bool) -> i32 {
	cvar::create("ui_wombat", "0", 0);

	set_menuconfig(menu::MenuConfig::init());

	ui_println!("UI init completed.");
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

#[allow(non_camel_case_types, dead_code)]
#[repr(C)]
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
	let menu_command : uiMenuCommand_t = unsafe { std::mem::transmute(menu) };
	match menu_command {
		uiMenuCommand_t::UIMENU_NONE => 0,
		uiMenuCommand_t::UIMENU_MAIN => MENUCONFIG.get().unwrap().lock().unwrap().set_main_menu(),
		_ => -1
	}
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
