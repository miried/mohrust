use std::{collections::HashMap, convert::TryFrom};

use crate::ui_println;
use crate::client::fs;

use super::urc;

#[derive(Debug)]
struct Menu {
    name : String,
}
#[derive(Debug)]
pub struct MenuConfig {
    parsed_menus : HashMap<String, Menu>,
    active_menus : Vec<String>,
}

impl MenuConfig {

    pub fn init () -> MenuConfig {
        MenuConfig {
            parsed_menus : HashMap::new(),
            active_menus : Vec::new(),
        }
    }
    
    fn push_menu(&mut self, name : &str) {
        let saved = self.parsed_menus.get(name);
        
        if saved.is_none() {
            let filename = format!("ui/{}.urc", name);
            let file = fs::FileHandle::try_from(&filename).unwrap();
            let urc_string = file.read_text();

            let _file_parse = urc::parse_urc( &urc_string );

            let menu = Menu {
                name : name.to_owned(),
            };
            self.parsed_menus.insert(name.to_owned(), menu);
        };

        
        self.active_menus.push(name.to_owned());
        ui_println!("Loaded menus:\n{:?}", self.active_menus);
    }
    
    pub fn set_main_menu(&mut self) -> i32 {
        if self.active_menus.is_empty() {
            self.push_menu("main");
        }
        0
    }    
}
