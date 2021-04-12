mod label;
mod button;

use crate::ui_println;

use crate::menu::Draw;


#[derive(Debug)]
pub enum Widget {
    Label(label::Label),
    Button(button::Button),
}

impl Draw for Widget {
    fn draw(&self) {
        match self {
            Widget::Label(l) => l.draw(),
            Widget::Button(b) => b.draw(),
        }
    }
}

impl Widget {
    pub fn parse<'a, T: Iterator<Item = &'a str>>( commands : &mut T ) -> Option<Self> {
        match commands.next() {
            Some("Label") => Some(Widget::Label(label::Label::parse(commands))),
            Some("Button") => Some(Widget::Button(button::Button::parse(commands))),
            Some(r) => {ui_println!("Unknown resource type: {}", r); None},
            None => None,
        }
    }
}
