pub mod first_follow;
pub mod grammar;
pub mod print;
pub mod table;
pub mod validation;

use std::error::Error;

use grammar::{Grammar, Production};
use table::ParsingTable;
fn main() -> Result<(), Box<dyn Error>> {
    let mut grammar = Grammar::new("S");

    // Example grammar:
    // S -> AaB
    // A -> bA | ε
    // B -> cB | ε
    grammar.add_production("S", vec!["A", "a", "B"]);
    grammar.add_production("A", vec!["b", "A"]);
    grammar.add_production("A", vec!["ε"]);
    grammar.add_production("B", vec!["c", "B"]);
    grammar.add_production("B", vec!["ε"]);

    grammar.print_input_grammar();
    grammar.print_first_set();
    grammar.print_follow_set();
    grammar.print_parsing_table();
    grammar.print_is_ll1();

    // Let's test with a non-LL(1) grammar
    println!("\nTesting a non-LL(1) grammar:");
    let mut non_ll1_grammar = Grammar::new("S");

    // // S -> A | B
    // // A -> a
    // // B -> a
    non_ll1_grammar.add_production("S", vec!["A"]);
    non_ll1_grammar.add_production("S", vec!["B"]);
    non_ll1_grammar.add_production("A", vec!["a"]);
    non_ll1_grammar.add_production("B", vec!["a"]);

    non_ll1_grammar.print_input_grammar();
    non_ll1_grammar.print_first_set();
    non_ll1_grammar.print_follow_set();
    non_ll1_grammar.print_parsing_table();
    non_ll1_grammar.print_is_ll1();

    Ok(())
}
