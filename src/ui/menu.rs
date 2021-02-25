use crate::ui_println;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

static MENUSTACK : OnceCell<Mutex<Vec<u32>>> = OnceCell::new();

pub fn init () {
    let result = MENUSTACK.set(Mutex::new(Vec::new()));
    result.expect("Could not initialize MENUSTACK, already done before.");
}

fn push_menu(name : &str) {
    MENUSTACK.get().unwrap().lock().unwrap().push(1);
    ui_println!("Loaded menu {}", name);
}

pub fn set_main_menu() -> i32 {
    if MENUSTACK.get().unwrap().lock().unwrap().is_empty() {
        push_menu("main");
    }
    0
}
