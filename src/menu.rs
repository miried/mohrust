use std::{collections::HashMap, convert::TryFrom};
use std::sync::Arc;

use crate::ui_println;
use crate::client as cl;

use cl::render::Shader;
use cl::key::KeyNum;

mod urc;
mod widget;


pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct LoadedMenus {
    cache : HashMap<String, Arc<urc::Menu>>,
    stack : Vec<Arc<urc::Menu>>,
    cursor : Shader,
    cursor_posx : i32,
    cursor_posy : i32,
}

impl Draw for LoadedMenus {
    fn draw(&self) {
        self.stack.iter().for_each(|m| m.draw());

        self.cursor.draw(self.cursor_posx, self.cursor_posy, 32, 32);
    }
}

fn clamp_pos(pos : i32, delta : i32) -> i32{
    let result = pos + delta;

    if result < 0 { 0 }
    else if result > 640 { 640 }
    else { result }
}

impl LoadedMenus {

    pub fn new () -> Self {
        Self {
            cache : HashMap::new(),
            stack : Vec::new(),
            cursor : Shader::register("mouse"),
            cursor_posx : 0,
            cursor_posy : 0,
        }
    }

    pub fn clear(&mut self) {
        Vec::clear(&mut self.stack);
        HashMap::clear(&mut self.cache);
    }

    pub fn is_fullscreen(&self) -> bool {
        let top_menu = self.stack.last();

        let top_menu_fullscreen =
            top_menu.filter(|_|cl::key::is_catch_ui())
            .map(|m|m.is_fullscreen())
            .unwrap_or(false);

        top_menu_fullscreen
    }

    pub fn key_event(&mut self, keycode : i32, down : bool) {

        if self.stack.is_empty() {
            return
        }

        let key : KeyNum = unsafe { std::mem::transmute(keycode) };

        if !down {
            return
        }

        match key {
            KeyNum::K_ESCAPE => self.pop_menu(),
            KeyNum::K_MOUSE1 => self.mouse_clicked(),
            _ => {},
        }
    }

    fn mouse_clicked(&self) {
        if let Some(m) = self.stack.last() {
            m.mouse_click();
        }
    }

    pub fn mouse_moved(&mut self, dx: i32, dy : i32 ) {

        if self.stack.is_empty() {
            return
        }

        self.cursor_posx = clamp_pos(self.cursor_posx, dx);
        self.cursor_posy = clamp_pos(self.cursor_posy, dy);

        if let Some(m) = self.stack.last() {
            m.mouse_move(self.cursor_posx, self.cursor_posy);
        }
    }

    fn load_menu(name : &str) -> Option<Arc<urc::Menu>> {
        let filename   = format!("ui/{}.urc", name);
        match cl::fs::FileHandle::try_from(&filename) {
            Ok(file) => {
                let urc_string = file.read_text();

                let menu = urc::Menu::parse( &urc_string );
                Some(Arc::new(menu))
            },
            Err(_) => {
                ui_println!("Menu not found: {}", filename);
                None
            }
        }
    }
    
    pub fn push_menu(&mut self, name : &str) {
        let from_cache =
            self.cache
            .get(name)
            .map(Arc::clone).
            or_else(|| {
                LoadedMenus::load_menu(name).map(|arc_menu| {
                    let arc_clone = Arc::clone(&arc_menu);
                    self.cache.insert(name.to_owned(), arc_clone);
                    arc_menu
                })
            });
        
        if let Some(m) = from_cache {
            self.stack.push(m);
            cl::key::catch_ui();
        }
    }

    pub fn pop_menu(&mut self) {
        self.stack.pop();
    }
    
    pub fn set_main_menu(&mut self) {
        if self.stack.is_empty() {
            self.push_menu("main");
        }
    }    
}
