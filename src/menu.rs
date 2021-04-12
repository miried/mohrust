use std::{collections::HashMap, convert::TryFrom};
use std::sync::Arc;

use crate::ui_println;
use crate::client as cl;

mod urc;
mod widget;

#[derive(Debug)]
pub struct LoadedMenus {
    cache : HashMap<String, Arc<urc::Menu>>,
    stack : Vec<Arc<urc::Menu>>,
}

impl LoadedMenus {

    pub fn new () -> Self {
        Self {
            cache : HashMap::new(),
            stack : Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        Vec::clear(&mut self.stack);
        HashMap::clear(&mut self.cache);
    }

    pub fn get_stack(&self) -> &Vec<Arc<urc::Menu>> {
        &self.stack
    }
}

impl LoadedMenus {

    fn load_menu(name : &str) -> Arc<urc::Menu> {
        let filename   = format!("ui/{}.urc", name);
        let file   = cl::fs::FileHandle::try_from(&filename).unwrap();
        let urc_string = file.read_text();

        let menu = urc::Menu::parse( &urc_string );
        Arc::new(menu)
    }
    
    pub fn push_menu(&mut self, name : &str) {
        let from_cache =
            self.cache
            .get(name)
            .map(Arc::clone).
            unwrap_or_else(|| {
            let arc_menu = LoadedMenus::load_menu(name);
            let arc_clone = Arc::clone(&arc_menu);
            self.cache.insert(name.to_owned(), arc_clone);
            arc_menu
        });
        
        self.stack.push(from_cache);
        cl::key::catch_ui();
        ui_println!("Loaded menus:\n{:?}", self.stack);
    }
    
    pub fn set_main_menu(&mut self) {
        if self.stack.is_empty() {
            self.push_menu("main");
        }
    }    
}
