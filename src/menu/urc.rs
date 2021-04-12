use std::str::SplitWhitespace;

use super::widget;
use crate::ui_println;

use crate::menu::Draw;

#[derive(Debug)]
#[derive(Default)]
pub struct Menu {
    name : String,
    width : u32,
    height : u32,
    fullscreen : bool,
    resources : Vec<widget::Widget>,
}

impl Draw for Menu {
    fn draw(&self) {
        self.resources.iter().for_each(|r|r.draw());
    }
}

impl Menu {

    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn parse( urc_string : &str ) -> Self {
        let mut commands =
            urc_string
            .split(&['\r','\n'][..])
            .filter(|s| !s.is_empty())
            .filter(|s| !s.starts_with("//"));

        let mut menu = Self::default();
    
        while let Some(cmd_line) = commands.next() {

            let mut args = cmd_line.split_whitespace();

            if let Some(command) = args.next(){
                menu.parse_command(command, args, &mut commands);
            }
        }
    
        menu
    }

    fn parse_menu_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.name = s.trim_matches('"').to_owned() }
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.width = x } }
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.height = x } }
    }

    fn parse_fullscreen_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { if let Ok(x) = s.parse::<i32>() { self.fullscreen = x != 0 } }
    }

    fn parse_resource_command<'a, T: Iterator<Item = &'a str>>(&mut self, commands : &mut T) {
        let widget = widget::Widget::parse(commands);
        
        if let Some(r) = widget {
            self.resources.push(r);
        }
    }

    fn parse_command<'a, T: Iterator<Item = &'a str>>(&mut self, command : &str, args : SplitWhitespace, commands : &mut T) {
        match command {
            "menu"       => self.parse_menu_command(args),
            "fullscreen" => self.parse_fullscreen_command(args),
            "resource"   => self.parse_resource_command(commands),
            "bgfill" | "bgcolor" | "borderstyle" | "virtualres" | "end." => {}, // TODO
            _ => ui_println!("Unknown URC command {}", command),
        }
    }
}
