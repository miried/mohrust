pub mod label;

use crate::q3ui::Draw;
#[derive(Debug)]
pub enum WidgetType {
    Label(label::Label),
    Button,
}

impl Draw for WidgetType {
    fn draw(&self) {
        match self {
            WidgetType::Label(l) => l.draw(),
            _ => {}
        }
    }
}