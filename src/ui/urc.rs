use pest::Parser;
#[derive(Parser)]
#[grammar = "grammars/urc.pest"]
pub struct URCParser;

use crate::ui_println;

pub fn parse_it( urc_string : &str )  {
    let file_parse = URCParser::parse(Rule::file, urc_string).expect("unsuccessful parse").next().unwrap();
    
    for record in file_parse.into_inner() {
        match record.as_rule() {
            Rule::command => {
                ui_println!("command {}", record);
            }
            Rule::resource => {
                ui_println!("resource {}", record);
            }
            Rule::EOI => (),
            _ => { ui_println!("TODO: {}", record);},
        }
    }
}
