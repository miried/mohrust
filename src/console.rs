use crate::{client as cl, ui_print};
use crate::menu::LoadedMenus;

pub fn cmd_pushmenu( menu_config : &mut LoadedMenus ) -> bool {
    if cl::util::argc() == 2 {
        let name = cl::util::argv(1);
        menu_config.push_menu(&name)
    }
    else {
        ui_print!("Usage: pushmenu <menuname>\n")
    }
    true
}

pub fn cmd_popmenu( menu_config : &mut LoadedMenus ) -> bool {
    menu_config.pop_menu();
    true
}
