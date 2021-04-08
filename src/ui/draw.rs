use super::urc::{Menu, Resource};
//use crate::client as cl;

fn draw_resource( r : &Resource ) {

    match r {
        Resource::Label(l) => l.shader.draw(l.rect[0], l.rect[1], l.rect[2], l.rect[3]),
        _ => {}
    }
}

pub fn draw_menu(menu : &Menu) {

    menu.resources.iter().for_each(draw_resource);
}
