use crate::client as cl;
use std::str::SplitWhitespace;
use crate::ui_println;

use crate::menu::Draw;
#[derive(Debug, Default)]
pub struct Label {
    name : String,
    rect : [i32; 4],
    shader : cl::render::Shader,
    enabled_cvar : Option<String>,
    linkcvartoshader : bool,
    linkcvar : String,
}


impl Draw for Label {
    fn draw(&self) {
        if self.enabled_cvar.is_some() {
            // TODO: check the cvar here. For now, we just don't draw.
            return
        }

        if self.linkcvartoshader {
            // TODO: we need to improve the cvar code so that we don't register the shader all the time.
            //let s = cl::cvar::_variable_string_buffer(&self.linkcvar);
            //cl::render::Shader::register(&s).draw(self.rect[0], self.rect[1], self.rect[2], self.rect[3]);
        }
        else {
            self.shader.draw(self.rect[0], self.rect[1], self.rect[2], self.rect[3]);
        }
    }
}

impl Label {
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
            "rect"       => self.parse_rect_command(args),
            "enabledcvar" => self.parse_enabledcvar_command(args),
            "linkcvartoshader" => self.linkcvartoshader = true,
            "linkcvar" => self.parse_linkcvar_command(args),
            "fgcolor" | "bgcolor" | "borderstyle" => {}, // TODO
            _ => ui_println!("Unknown URC command {}", command),
        }
    }

    fn parse_name_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.name = s.trim_matches('"').to_owned() }
    }

    fn parse_enabledcvar_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { let s = s.trim_matches('"').to_owned(); self.enabled_cvar=Some(s) }
    }

    fn parse_shader_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.shader = cl::render::Shader::register(s.trim_matches('"')) }
    }

    fn parse_linkcvar_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { self.linkcvar = s.trim_matches('"').to_owned() }
    }

    fn parse_rect_command(&mut self, mut args : SplitWhitespace) {
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[0] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[1] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[2] = x } };
        if let Some(s) = args.next() { if let Ok(x) = s.parse() { self.rect[3] = x } };
    }
}
