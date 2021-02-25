use std::convert::TryFrom;

use crate::ui_println;
use crate::client::fs;

#[derive(Debug)]
pub struct MenuConfig {
    menustack : Vec<u32>,
}

impl MenuConfig {

    pub fn init () -> MenuConfig {
        MenuConfig {
            menustack : Vec::new(),
        }
    }
    
    fn push_menu(&mut self, name : &str) {
        let filename = format!("ui/{}.urc", name);
        let file = fs::FileHandle::try_from(&filename).unwrap();
        self.menustack.push(1);
        ui_println!("Loaded menu {}", name);
        let _urc_string = file.readt();
    }
    
    pub fn set_main_menu(&mut self) -> i32 {
        if self.menustack.is_empty() {
            self.push_menu("main");
        }
        0
    }    
}
