use std::str::SplitWhitespace;
use crate::client as cl;

use crate::ui_println;

pub mod q3 {
    pub type _Vec2 = [f32; 2];
    pub type _Vec3 = [f32; 3];
    pub type Vec4 = [f32; 4];
}
#[derive(Debug)]
#[derive(Default)]
pub struct Menu {
    name : String,
    width : u32,
    height : u32,
    pub fullscreen : bool,
    pub resources : Vec<Resource>,
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
                Some("Label") => Some(Resource::Label(Label::parse(commands))),
                Some("Button") => Some(Resource::Button),
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

#[derive(Debug, Default)]
pub struct Label {
    name : String,
    pub rect : q3::Vec4,
    pub shader : cl::render::Shader,
}

impl Label {
    pub fn parse<'a, T: Iterator<Item = &'a str>>( commands : &mut T ) -> Label {
        let mut label = Label::default();

        if commands.next() != Some("{") {
            return label
        }
    
        while let Some(cmd_line) = commands.next() {

            if cmd_line == "}" {
                break
            }

            let mut args = cmd_line.split_whitespace();

            if let Some(command) = args.next(){
                label.parse_command(command, args);
            }
        }
        label
    }

    fn parse_command(&mut self, command : &str, args : SplitWhitespace) {
        match command {
            "name"       => self.parse_name_command(args),
            "shader"     => self.parse_shader_command(args),
            "rect"       => self.parse_rect_command(args),
            _ => ui_println!("Unknown URC command {}", command),
        }
    }

    fn parse_name_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.name = s.trim_matches('"').to_owned() }
    }

    fn parse_shader_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.shader = cl::render::Shader::register(s.trim_matches('"')) }
    }

    fn parse_rect_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[0] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[1] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[2] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[3] = x } };
    }
}

#[derive(Debug)]
pub enum Resource {
    Label(Label),
    Button,
}

/*
#[derive(Debug)]
#[derive(PartialEq)]
pub enum UrcProperty {
    Name(String),
    Width(u32),
    Height(u32),
    PosX(u32),
    PosY(u32),
    Fullscreen(bool),
    BgColor(q3::Vec4),
}

impl UrcProperty {
    pub fn is_fullscreen(property : &UrcProperty) -> Option<bool> {
        match *property {
            UrcProperty::Fullscreen(b) => Some(b),
            _ => None
        }
    }
}
#[derive(Debug)]
pub enum UrcResourceType {
    Menu,
    Label,
    Button
}
#[derive(Debug)]
pub struct UrcResource {
    pub res_type   : UrcResourceType,
    pub properties : Vec<UrcProperty>,
    pub resources  : Vec<UrcResource>,
}

impl UrcResource {
    fn new( t : UrcResourceType ) -> Self {
        UrcResource {
            res_type : t,
            properties : Vec::new(),
            resources : Vec::new(),
        }
    }

    fn new_menu() -> Self {
        UrcResource {
            res_type : UrcResourceType::Menu,
            properties : Vec::new(),
            resources : Vec::new(),
        }
    }

    fn name_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        let name = cmd_list.next().unwrap().trim_matches('"').to_owned();
        self.properties.push(UrcProperty::Name(name));
    }

    fn menu_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        let name = cmd_list.next().unwrap().trim_matches('"').to_owned();
        
        let width = cmd_list.next().unwrap().parse().unwrap();
        let height = cmd_list.next().unwrap().parse().unwrap();

        cmd_list.next();
        cmd_list.next();

        self.properties.push(UrcProperty::Name(name));
        self.properties.push(UrcProperty::Width(width));
        self.properties.push(UrcProperty::Height(height));
    }

    fn fullscreen_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        let is_fullscreen : i32 = cmd_list.next().unwrap().parse().unwrap();
        self.properties.push(UrcProperty::Fullscreen(is_fullscreen != 0));
    }

    fn bgcolor_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        let r = cmd_list.next().unwrap().parse().unwrap();
        let g = cmd_list.next().unwrap().parse().unwrap();
        let b = cmd_list.next().unwrap().parse().unwrap();
        let a = cmd_list.next().unwrap().parse().unwrap();
        
        self.properties.push(UrcProperty::BgColor([r, g, b, a]));
    }

    fn rect_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        let x = cmd_list.next().unwrap().parse().unwrap();
        let y = cmd_list.next().unwrap().parse().unwrap();
        let w = cmd_list.next().unwrap().parse().unwrap();
        let h = cmd_list.next().unwrap().parse().unwrap();
        
        self.properties.push(UrcProperty::PosX(x));
        self.properties.push(UrcProperty::PosY(y));
        self.properties.push(UrcProperty::Width(w));
        self.properties.push(UrcProperty::Height(h));
    }

    fn resource_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        let resource_type = cmd_list.next().unwrap();
        if cmd_list.next().unwrap() != "{" {
            return
        }

        let mut resource = match resource_type {
            "Label" => UrcResource::new(UrcResourceType::Label),
            "Button" => UrcResource::new(UrcResourceType::Button),
            r => {ui_println!("Resource type not implemented: {}", r);panic!()}
        };

        resource.parse_resource(cmd_list);

        self.resources.push(resource);
    }

    fn parse_command( &mut self, cur_cmd : &str, cmd_list : &mut SplitWhitespace ) {

        if cur_cmd.starts_with("//") {
            return
        }
    
        match cur_cmd {
            "menu"       => self.menu_cmd(cmd_list),
            "fullscreen" => self.fullscreen_cmd(cmd_list),
            "bgcolor"    => self.bgcolor_cmd(cmd_list),
            "resource"   => self.resource_cmd(cmd_list),
            "name"       => self.name_cmd(cmd_list),
            "rect"       => self.rect_cmd(cmd_list),
            _ => ui_println!("Unknown URC command {}", cur_cmd),
        }
    }

    fn parse_resource( &mut self, commands : &mut SplitWhitespace ) {
        while let Some(cmd) = commands.next() {
            match cmd {
                "}" => break,
                "end." => break,
                _ => self.parse_command(cmd, commands),
            }
        }
    }

    pub fn parse_urc( urc_string : &str ) -> Self {
        let mut commands = urc_string.split_whitespace();
        let mut menu = Self::new_menu();
    
        menu.parse_resource(&mut commands);
    
        menu
    }
}
*/