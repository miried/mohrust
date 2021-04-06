use libc::intptr_t;
use crate::client as cl;
use cl::uiImport_t;


pub fn is_catch_ui() -> bool {
    (get_catcher() & KEYCATCH_UI) != 0
}

pub fn catch_ui() {
    set_catcher(KEYCATCH_UI);
}

pub fn _remove_ui() {
    let catcher = get_catcher();
    set_catcher(catcher & !KEYCATCH_UI);
}

const KEYCATCH_UI : isize = 2;

fn get_catcher() -> isize {
    unsafe{cl::SYSCALL(uiImport_t::UI_KEY_GETCATCHER as intptr_t)}
}

fn set_catcher(catcher : isize) {
    unsafe{cl::SYSCALL(uiImport_t::UI_KEY_SETCATCHER as intptr_t, catcher)};
}

fn _clear_states() {
    unsafe{cl::SYSCALL(uiImport_t::UI_KEY_CLEARSTATES as intptr_t)};
}
