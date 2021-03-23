use std::str::SplitWhitespace;

//use crate::ui_println;

enum ResourceType {
    Label,
    Button,
}
enum BorderStyle {
    None,
}
struct UrcMenu {
    name : String,
    width : u32,
    height : u32,
    unknown_prop1 : bool,
    unknown_prop2 : i32,
    bgcolor : [f32; 4],
    _borderstyle : BorderStyle,
    _fullscreen : bool,
    _virtualres : bool,
    resources : Vec<ResourceType>,
}

impl Default for UrcMenu {
    fn default() -> Self {
        UrcMenu {
            name : String::new(),
            width : 0,
            height : 0,
            unknown_prop1 : false,
            unknown_prop2 : 0,
            bgcolor : [0.0, 0.0, 0.0, 0.0],
            _borderstyle : BorderStyle::None,
            _fullscreen : true,
            _virtualres : true,
            resources : Vec::new(),
        }
    }
}

impl UrcMenu {
    fn menu_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        self.name = cmd_list.next().unwrap().trim_matches('"').to_owned();
        self.width = cmd_list.next().unwrap().parse().unwrap();
        self.height = cmd_list.next().unwrap().parse().unwrap();
        self.unknown_prop1 = {cmd_list.next(); false};
        self.unknown_prop2 = cmd_list.next().unwrap().parse().unwrap();
    }

    fn bgcolor_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        let r = cmd_list.next().unwrap().parse().unwrap();
        let g = cmd_list.next().unwrap().parse().unwrap();
        let b = cmd_list.next().unwrap().parse().unwrap();
        let a = cmd_list.next().unwrap().parse().unwrap();
        self.bgcolor = [r, g, b, a];
    }

    fn resource_cmd( &mut self, cmd_list : &mut SplitWhitespace ) {
        let resource_type = cmd_list.next().unwrap();
        if cmd_list.next().unwrap() != "{" {
            return
        }

        match resource_type {
            "Label" => self.resources.push(ResourceType::Label),
            "Button" => self.resources.push(ResourceType::Button),
            r => println!("Resource type not implemented: {}", r)
        }
    }
}

fn parse_command( cur_cmd : &str, cmd_list : &mut SplitWhitespace, menu : &mut UrcMenu ) {

    if cur_cmd.starts_with("//") {
        return
    }

    match cur_cmd {
        "menu" => menu.menu_cmd(cmd_list),
        "bgcolor" => menu.bgcolor_cmd(cmd_list),
        "resource" => menu.resource_cmd(cmd_list),
        _ => println!("unknown command {}", cur_cmd),
    }
}

pub fn parse_urc( urc_string : &str )  {

    let mut commands = urc_string.split_whitespace();

    let mut menu = UrcMenu::default();

    while let Some(cmd) = commands.next() {
        //println!("{}", command);
        parse_command(cmd, &mut commands, &mut menu);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    //#[should_panic]
    fn internal() {
        let testline = "menu \"main\" 639 479 NONE 0\nbgcolor 1 1 1 1\n//vidmode 3\n\n\nresource\nLabel\n{\nname \"Default\"\nrect 480 0 256 512\n}\n";
        assert_eq!((), super::parse_urc(testline));
    }
}
