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
    // Amirhossein Edadi - Amin Owrang Pour - Amin Sheikh Azimi
    let grammar = Grammar::from_file("src/input.txt")?;

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

// sample 1
// accepts: ba bacc a b
// Error recovery Strategy 1: Skip Input   abbdacc
// Error recovery Strategy 2: Pop stack   bbxacc
// Error recovery Strategy 3: Follow sets   bbcacc
/*
S
S -> A a B
A -> b A | ε
B -> c B | ε
*/

// sample 2
// accepts: ixiyi
/*
E
E -> T S
S -> y T S | ε
T -> F B
B -> x F B | ε
F -> i | n E m
*/

// sample 3 not LL(1)
/*
S
S -> n L m | a
L -> S E
E -> m S E | ε
*/
