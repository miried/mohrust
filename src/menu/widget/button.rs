use crate::client as cl;
use std::str::SplitWhitespace;
use crate::ui_println;

use crate::menu::Draw;
#[derive(Debug, Default)]
pub struct Button {
    name : String,
    rect : [i32; 4],
    shader : cl::render::Shader,
    hover_shader : cl::render::Shader,
    stuffcommand : String,
    hovercommand : Option<String>,
    clicksound : Option<String>,
    enabled_cvar : Option<String>,
}


impl Draw for Button {
    fn draw(&self) {
        self.enabled_cvar.as_ref().map(|s| {
            if cl::cvar::value_float(&s) == 0.0 {return}
        });

        self.shader.draw(self.rect[0], self.rect[1], self.rect[2], self.rect[3]);
    }
}

impl Button {

    pub fn mouse_click(&self) {
        cl::util::cmd_execute(&self.stuffcommand)
    }

    pub fn parse<'a, T: Iterator<Item = &'a str>>( commands : &mut T ) -> Self {
        let mut label = Self::default();

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
            "hovershader"     => self.parse_hovershader_command(args),
            "rect"       => self.parse_rect_command(args),
            "stuffcommand" => self.parse_stuffcommand_command(args),
            "hovercommand" => self.parse_hovercommand_command(args),
            "enabledcvar" => self.parse_enabledcvar_command(args),
            "clicksound" => self.parse_clicksound_command(args),
            "fgcolor" | "bgcolor" | "borderstyle" => {}, // TODO
            _ => ui_println!("Unknown URC command {}", command),
        }
    }

    fn parse_name_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.name = s.trim_matches('"').to_owned() }
    }

    fn parse_enabledcvar_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() {
            let s = s.trim_matches('"').to_owned();
            self.enabled_cvar=Some(s)
        }
    }

    fn parse_hovercommand_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { let s = s.trim_matches('"').to_owned(); self.hovercommand=Some(s) }
    }

    fn parse_shader_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.shader = cl::render::Shader::register(s.trim_matches('"')) }
    }

    fn parse_hovershader_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.hover_shader = cl::render::Shader::register(s.trim_matches('"')) }
    }

    fn parse_stuffcommand_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.stuffcommand = s.trim_matches('"').to_owned() }
    }

    fn parse_clicksound_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() {
            let string = s.trim_matches('"').to_owned();
            self.clicksound = Some(string)
        }
    }

    fn parse_rect_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[0] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[1] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[2] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[3] = x } };
    }
}
