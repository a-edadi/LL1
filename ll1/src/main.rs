pub mod error;
pub mod first_follow;
pub mod grammar;
pub mod table;

use error::Ll1Error;
use std::collections::HashMap;
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

    // not ll1
    // grammar.add_production("S", vec!["A", "a"]);
    // grammar.add_production("S", vec!["A", "b"]);
    // grammar.add_production("A", vec!["ε"]);
    // grammar.add_production("A", vec!["c"]);

    // Display the input grammar
    println!("📚 Grammar:");
    let mut productions_by_nt: HashMap<String, Vec<String>> = HashMap::new();

    // Group productions by non-terminal
    for production in &grammar.productions {
        let entry = productions_by_nt
            .entry(production.non_terminal.clone())
            .or_insert_with(Vec::new);
        entry.push(production.derivation.join(" "));
    }

    // Print productions with pipe for alternatives
    for (nt, productions) in productions_by_nt {
        let formatted = productions.join(" | ");
        println!("{} → {}", nt, formatted);
    }

    // Compute FIRST and FOLLOW sets
    let first_sets = grammar.compute_first_sets();
    let follow_sets = grammar.compute_follow_sets(&first_sets);

    // Print FIRST sets without terminals
    println!("\n🔍 FIRST Sets:");
    for (symbol, first_set) in first_sets {
        // Check if the symbol is a non-terminal (i.e., it starts with an uppercase letter)
        if symbol.chars().next().unwrap().is_uppercase() {
            println!("FIRST({}) = {:?}", symbol, first_set);
        }
    }

    // Display FOLLOW sets
    println!("\n🔍 FOLLOW Sets:");
    for (symbol, set) in &follow_sets {
        println!("FOLLOW({}) = {:?}", symbol, set);
    }

    match grammar.is_ll1() {
        Ok(parsing_table) => {
            // If the grammar is LL(1), print the parsing table
            println!("{}", parsing_table);
        }
        Err(e) => {
            // If there was an error (e.g., not LL(1)), print the error
            println!("Error: {}", e);
        }
    }

    // S → aS | aA
    // A → a
    // non_ll1_grammar.add_production("S", vec!["a", "S"]);
    // non_ll1_grammar.add_production("S", vec!["a", "A"]);
    // non_ll1_grammar.add_production("A", vec!["a"]);

    // S -> AB | AC
    // A -> a
    // B -> b
    // C -> b

    // Let's test with a non-LL(1) grammar
    // println!("\nTesting a non-LL(1) grammar:");
    // let mut non_ll1_grammar = Grammar::new("S");

    // non_ll1_grammar.add_production("S", vec!["A"]);
    // non_ll1_grammar.add_production("S", vec!["B"]);
    // non_ll1_grammar.add_production("A", vec!["a"]);
    // non_ll1_grammar.add_production("B", vec!["a"]);

    // match non_ll1_grammar.is_ll1() {
    //     Ok(parsing_table) => {
    //         // If the grammar is LL(1), print the parsing table
    //         println!("{}", parsing_table);
    //     }
    //     Err(e) => {
    //         // If there was an error (e.g., not LL(1)), print the error
    //         println!("Error: {}", e);
    //     }
    // }
    Ok(())
}
