use std::sync::Mutex;
use crate::menu::LoadedMenus;
use once_cell::sync::OnceCell;

use crate::client as cl;
use crate::ui_println;
use crate::menu::Draw;

pub const UI_APIVERSION : i32 = 6;

static LOADED_MENUS : OnceCell<Mutex<LoadedMenus>> = OnceCell::new();

fn set_menuconfig(mc : LoadedMenus) {
	let mutex = Mutex::new(mc);
	let _ = LOADED_MENUS.set(mutex);
}

pub fn init(_in_game_load : bool) -> i32 {
	cl::cvar::create("ui_wombat", "0", 0);

	let mc = LoadedMenus::new();
	set_menuconfig(mc);

	ui_println!("UI init completed.");
	0
}

pub fn shutdown() -> i32 {
	LOADED_MENUS.get()
	.expect("UI shutdown before Init.")
	.lock()
	.expect("UI shutdown lock could not be aquired.")
	.clear();
	0
}

pub fn key_event(key : i32, down : bool) -> i32 {

	LOADED_MENUS.get()
		.expect("UI Refresh before Init.")
		.lock()
		.expect("UI Refresh lock could not be aquired.")
		.key_event(key, down);

	0
}

pub fn mouse_event(dx : i32, dy : i32) -> i32 {

	LOADED_MENUS.get()
		.expect("UI Refresh before Init.")
		.lock()
		.expect("UI Refresh lock could not be aquired.")
		.mouse_event(dx, dy);

	0
}

pub fn refresh(_realtime : i32) -> i32 {

	if !cl::key::is_catch_ui() {
		return 0;
	}

	let menu_config =
		LOADED_MENUS.get()
		.expect("UI Refresh before Init.")
		.lock()
		.expect("UI Refresh lock could not be aquired.");

	menu_config.draw();

	0
}

pub fn is_fullscreen() -> bool {
	let menu_config =
		LOADED_MENUS.get()
		.expect("UI is_fullscreen before Init.")
		.lock()
		.expect("UI is_fullscreen lock could not be aquired.");

	menu_config.is_fullscreen()
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
		uiMenuCommand_t::UIMENU_MAIN => {
			let mut menu_config =
				LOADED_MENUS.get()
				.expect("UI set_active_menu before Init.")
				.lock()
				.expect("UI set_active_menu lock could not be aquired.");
			menu_config.set_main_menu()},
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
