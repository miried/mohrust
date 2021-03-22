use pest::Parser;
#[derive(Parser)]
#[grammar = "grammars/urc.pest"]
pub struct URCParser;

//use crate::ui_println;

pub fn parse_it( urc_string : &str )  {
    let file_parse = URCParser::parse(Rule::file, urc_string).expect("unsuccessful parse").next().unwrap();
    
    println!("rec: {}", file_parse);
    for record in file_parse.into_inner() {
        match record.as_rule() {
            Rule::command => {
                println!("command {}", record);
            }
            Rule::resource => {
                println!("resource {}", record);
            }
            Rule::name => {
                println!("name {}", record.into_inner());
            }
            Rule::EOI => (),
            _ => { println!("TODO: {}", record);},
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    //#[should_panic]
    fn internal() {
        let testline = "menu \"main\" 639 479 NONE 0\nbgcolor 1 1 1 1\n";
        assert_eq!((), super::parse_it(testline));
    }
}
