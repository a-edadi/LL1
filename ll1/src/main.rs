pub mod first_follow;
pub mod grammar;
pub mod parser;
pub mod print;
pub mod table;
pub mod validation;

use std::error::Error;

use grammar::{Grammar, Production};
use parser::Parser;
use table::ParsingTable;

fn main() -> Result<(), Box<dyn Error>> {
    // let grammar = Grammar::from_file("src/input.txt")?;
    let grammar = Grammar::from_string("A -> B", "A")?;

    grammar.print_input_grammar();
    grammar.print_first_set();
    grammar.print_follow_set();
    grammar.print_parsing_table();
    grammar.print_is_ll1();

    if grammar.is_ll1() {
        let mut parser = match Parser::new(grammar) {
            Ok(parser) => parser,
            Err(e) => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
            }
        };

        parser.set_input_io();

        // Now parse the input using the parser
        match parser.parse() {
            Ok(()) => println!("✅ The input is accepted!"),
            Err(e) => println!("❌ Error: {}", e),
        }
    }

    Ok(())
}
