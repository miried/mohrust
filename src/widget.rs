pub mod label;

pub trait Draw {
    fn draw(&self);
}
#[derive(Debug)]
pub enum WidgetType {
    Label(label::Label),
    Button,
}
