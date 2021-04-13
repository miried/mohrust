mod label;
mod button;

use crate::ui_println;

use crate::menu::Draw;


#[derive(Debug)]
pub struct Widget {
    active : bool,
    widget : WidgetType,
}

#[derive(Debug)]
enum WidgetType {
    Label(label::Label),
    Button(button::Button),
}

impl Draw for Widget {
    fn draw(&self) {
        match &self.widget {
            WidgetType::Label(l) => l.draw(),
            WidgetType::Button(b) => b.draw(),
        }
    }
}

impl Widget {

    pub fn mouse_click(&self) {
        if !self.active {
            return;
        }

        match &self.widget {
            WidgetType::Button(b) => b.mouse_click(),
            _ => {},
        }
    }

    pub fn mouse_move(&self, _x : i32, _y : i32) {

    }

    pub fn parse<'a, T: Iterator<Item = &'a str>>( commands : &mut T ) -> Option<Self> {
        let w =  match commands.next() {
            Some("Label") => Some(WidgetType::Label(label::Label::parse(commands))),
            Some("Button") => Some(WidgetType::Button(button::Button::parse(commands))),
            Some(r) => {ui_println!("Unknown resource type: {}", r); None},
            None => None,
        };

        w.map(|t|
            Widget {
                active : false,
                widget : t,
            }
        )
    }
}
