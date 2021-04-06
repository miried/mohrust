use std::{collections::HashMap, convert::TryFrom};
use std::sync::Arc;

use urc::UrcResource;

use crate::ui_println;
use crate::client as cl;

use super::urc;

#[derive(Debug)]
pub struct UrcCache {
    loaded_cache : HashMap<String, Arc<urc::UrcResource>>,
    pub active_stack : Vec<Arc<urc::UrcResource>>,
}

impl UrcCache {

    pub fn new () -> Self {
        Self {
            loaded_cache : HashMap::new(),
            active_stack : Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        Vec::clear(&mut self.active_stack);
        HashMap::clear(&mut self.loaded_cache);
    }

    pub fn get_stack(&self) -> &Vec<Arc<urc::UrcResource>> {
        &self.active_stack
    }
}

impl UrcCache {

    fn load_menu(name : &str) -> Arc<UrcResource> {
        let filename   = format!("ui/{}.urc", name);
        let file   = cl::fs::FileHandle::try_from(&filename).unwrap();
        let urc_string = file.read_text();

        let menu = urc::UrcResource::parse_urc( &urc_string );
        Arc::new(menu)
    }
    
    pub fn push_menu(&mut self, name : &str) {
        let from_cache =
            self.loaded_cache
            .get(name)
            .map(Arc::clone).
            unwrap_or_else(|| {
            let arc_menu = UrcCache::load_menu(name);
            let arc_clone = Arc::clone(&arc_menu);
            self.loaded_cache.insert(name.to_owned(), arc_clone);
            arc_menu
        });
        
        self.active_stack.push(from_cache);
        cl::key::catch_ui();
        ui_println!("Loaded menus:\n{:?}", self.active_stack);
    }
    
    pub fn set_main_menu(&mut self) {
        if self.active_stack.is_empty() {
            self.push_menu("main");
        }
    }    
}
