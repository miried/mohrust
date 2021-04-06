use std::sync::Mutex;
use menu::UrcCache;
use once_cell::sync::OnceCell;

mod menu;
mod urc;
mod draw;

use crate::client as cl;
use crate::ui_println;

use self::urc::UrcProperty;


pub const UI_APIVERSION : i32 = 6;

static MENUCONFIG : OnceCell<Mutex<UrcCache>> = OnceCell::new();

fn set_menuconfig(mc : UrcCache) {
	let mutex = Mutex::new(mc);
	let _ = MENUCONFIG.set(mutex);
}

pub fn init(_in_game_load : bool) -> i32 {
	cl::cvar::create("ui_wombat", "0", 0);

	let mc = menu::UrcCache::new();
	set_menuconfig(mc);

	ui_println!("UI init completed.");
	0
}

pub fn shutdown() -> i32 {
	MENUCONFIG.get()
	.expect("UI shutdown before Init.")
	.lock()
	.expect("UI shutdown lock could not be aquired.")
	.clear();
	0
}

pub fn key_event(_key : i32, _down : bool) -> i32 {
	0
}

pub fn mouse_event(_x : i32, _y : i32) -> i32 {
	0
}

pub fn refresh(_realtime : i32) -> i32 {

	if !cl::key::is_catch_ui() {
		return 0;
	}

	let menu_config =
		MENUCONFIG.get()
		.expect("UI Refresh before Init.")
		.lock()
		.expect("UI Refresh lock could not be aquired.");

	let _a =
		menu_config
		.get_stack()
		.iter()
		.map(|_x| ui_println!("a"));

	//ui_println!("{:?}", a);
	0
}

pub fn is_fullscreen() -> bool {
	let menu_config =
		MENUCONFIG.get()
		.expect("UI is_fullscreen before Init.")
		.lock()
		.expect("UI is_fullscreen lock could not be aquired.");

	let top_menu = menu_config.get_stack().last();

	let top_menu_fullscreen =
		top_menu.filter(|_|cl::key::is_catch_ui())
		.map(|m|m.properties.iter().find_map(UrcProperty::is_fullscreen))
		.flatten()
		.unwrap_or(false);

	top_menu_fullscreen
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
		uiMenuCommand_t::UIMENU_NONE => (),
		uiMenuCommand_t::UIMENU_MAIN => MENUCONFIG.get().unwrap().lock().unwrap().set_main_menu(),
		_ => ()
	}
	0
}

pub fn console_command(_realtime : i32) -> bool {
	false
}

pub fn draw_connect_screen(_overlay : bool) -> i32 {
	0
}

pub fn has_unique_cdkey() -> bool {
	false
}
