use std::collections::HashMap;

use super::{Grammar, ParsingTable};

impl Grammar {
    /// Method to print the input grammar
    pub fn print_input_grammar(&self) {
        println!("📚 Grammar:");

        let mut productions_by_nt: HashMap<String, Vec<String>> = HashMap::new();

        // Group productions by non-terminal
        for production in &self.productions {
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
    }

    /// Method to print the FIRST sets
    pub fn print_first_set(&self) {
        // Compute FIRST sets
        let first_sets = self.compute_first_sets();

        // Print FIRST sets without terminals
        println!("\n🔍 FIRST Sets:");
        for (symbol, first_set) in first_sets {
            // Check if the symbol is a non-terminal (i.e., it starts with an uppercase letter)
            if symbol.chars().next().unwrap().is_uppercase() {
                println!("FIRST({}) = {:?}", symbol, first_set);
            }
        }
    }

    /// Method to print the FOLLOW sets
    pub fn print_follow_set(&self) {
        // Compute FIRST sets
        let first_sets = self.compute_first_sets();

        // Compute FOLLOW sets
        let follow_sets = self.compute_follow_sets(&first_sets);

        // Display FOLLOW sets
        println!("\n🔍 FOLLOW Sets:");
        for (symbol, set) in follow_sets {
            println!("FOLLOW({}) = {:?}", symbol, set);
        }
    }

    /// Print the Parsing Table
    pub fn print_parsing_table(&self) {
        match ParsingTable::build(self) {
            Ok(table) => println!("\nLL(1) Parsing Table:\n{}", table),
            Err(e) => println!("\n❌ Error: {}", e),
        }
    }

    /// Print if the grammar is LL(1) or not
    pub fn print_is_ll1(&self) {
        if self.is_ll1() {
            println!("\n✅ Grammar is LL(1)");
        } else {
            println!("\n❌ Grammar is not LL(1)");
        }
    }
}
