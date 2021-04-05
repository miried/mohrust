use std::{collections::HashMap, convert::TryFrom};
use std::sync::Arc;

use urc::UrcResource;

use crate::ui_println;
use crate::client::fs;

use super::urc;

#[derive(Debug)]
pub struct UrcCache {
    loaded_cache : HashMap<String, Arc<urc::UrcResource>>,
    active_stack : Vec<Arc<urc::UrcResource>>,
}

impl UrcCache {

    pub fn new () -> Self {
        Self {
            loaded_cache : HashMap::new(),
            active_stack : Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.active_stack.clear();
        self.loaded_cache.clear();
    }

    fn load_menu(name : &str) -> Arc<UrcResource> {
        let filename   = format!("ui/{}.urc", name);
        let file   = fs::FileHandle::try_from(&filename).unwrap();
        let urc_string = file.read_text();

        let menu = urc::UrcResource::parse_urc( &urc_string );
        Arc::new(menu)
    }
    
    fn push_menu(&mut self, name : &str) {
        let from_cache =
            self.loaded_cache
            .get(name)
            .map(Arc::clone).
            unwrap_or_else(|| {
            let arc_menu = UrcCache::load_menu(name);
            self.loaded_cache.insert(name.to_owned(), arc_menu.clone());
            arc_menu
        });
        
        self.active_stack.push(from_cache);
        ui_println!("Loaded menus:\n{:?}", self.active_stack);
    }
    
    pub fn set_main_menu(&mut self) -> i32 {
        if self.active_stack.is_empty() {
            self.push_menu("main");
        }
        0
    }    
}
