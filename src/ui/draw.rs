use super::urc::{Menu};
use crate::widget::{Draw, WidgetType};

fn draw_resource( r : &WidgetType ) {

    match r {
        WidgetType::Label(l) => l.draw(),
        _ => {}
    }
}

pub fn draw_menu(menu : &Menu) {

    menu.resources.iter().for_each(draw_resource);
}
