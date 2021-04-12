use std::sync::Mutex;
use menu::LoadedMenus;
use once_cell::sync::OnceCell;

mod menu;
mod urc;
mod draw;

use crate::client as cl;
use crate::ui_println;


pub const UI_APIVERSION : i32 = 6;

static LOADED_MENUS : OnceCell<Mutex<LoadedMenus>> = OnceCell::new();

fn set_menuconfig(mc : LoadedMenus) {
	let mutex = Mutex::new(mc);
	let _ = LOADED_MENUS.set(mutex);
}

pub fn init(_in_game_load : bool) -> i32 {
	cl::cvar::create("ui_wombat", "0", 0);

	let mc = menu::LoadedMenus::new();
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
		LOADED_MENUS.get()
		.expect("UI Refresh before Init.")
		.lock()
		.expect("UI Refresh lock could not be aquired.");

	menu_config
		.get_stack()
		.iter()
		.for_each(|m| draw::draw_menu(m));

	0
}

pub fn is_fullscreen() -> bool {
	let menu_config =
		LOADED_MENUS.get()
		.expect("UI is_fullscreen before Init.")
		.lock()
		.expect("UI is_fullscreen lock could not be aquired.");

	let top_menu = menu_config.get_stack().last();

	let top_menu_fullscreen =
		top_menu.filter(|_|cl::key::is_catch_ui())
		.map(|m|m.fullscreen)
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
