use std::str::SplitWhitespace;

use crate::widget;
use crate::ui_println;

#[derive(Debug)]
#[derive(Default)]
pub struct Menu {
    name : String,
    width : u32,
    height : u32,
    pub fullscreen : bool,
    pub resources : Vec<widget::WidgetType>,
}

impl Menu {

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
        let resource = 
            match commands.next() {
                Some("Label") => Some(widget::WidgetType::Label(widget::label::Label::parse(commands))),
                Some("Button") => Some(widget::WidgetType::Button),
                Some(r) => {ui_println!("Unknown resource type: {}", r); None},
                None => None,
            };
        
        if let Some(r) = resource {
            self.resources.push(r);
        }
    }

    fn parse_command<'a, T: Iterator<Item = &'a str>>(&mut self, command : &str, args : SplitWhitespace, commands : &mut T) {
        match command {
            "menu"       => self.parse_menu_command(args),
            "fullscreen" => self.parse_fullscreen_command(args),
            "resource"   => self.parse_resource_command(commands),
            _ => ui_println!("Unknown URC command {}", command),
        }
    }
}
