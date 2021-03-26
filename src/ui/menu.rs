use std::{collections::HashMap, convert::TryFrom};

use crate::ui_println;
use crate::client::fs;

use super::urc;

#[derive(Debug)]
pub struct UrcMenus {
    parsed_menus : HashMap<String, urc::UrcResource>,
    active_menus : Vec<String>,
}

impl UrcMenus {

    pub fn new () -> Self {
        Self {
            parsed_menus : HashMap::new(),
            active_menus : Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.active_menus.clear();
        self.parsed_menus.clear();
    }
    
    fn push_menu(&mut self, name : &str) {
        let saved = self.parsed_menus.get(name);
        
        if saved.is_none() {
            let filename = format!("ui/{}.urc", name);
            let file = fs::FileHandle::try_from(&filename).unwrap();
            let urc_string = file.read_text();

            let menu = urc::UrcResource::parse_urc( &urc_string );

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
