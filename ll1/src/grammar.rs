use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Production {
    pub non_terminal: String,
    pub derivation: Vec<String>,
}

#[derive(Debug)]
pub struct Grammar {
    pub productions: Vec<Production>,
    pub terminals: HashSet<String>,
    pub non_terminals: HashSet<String>,
    pub start_symbol: String,
}

impl Grammar {
    pub fn new(start_symbol: &str) -> Self {
        Grammar {
            productions: Vec::new(),
            terminals: HashSet::new(),
            non_terminals: HashSet::new(),
            start_symbol: start_symbol.to_string(),
        }
    }

    pub fn add_production(&mut self, non_terminal: &str, derivation: Vec<&str>) {
        self.non_terminals.insert(non_terminal.to_string());
        let derivation: Vec<String> = derivation.iter().map(|s| s.to_string()).collect();

        for symbol in &derivation {
            if symbol.chars().next().unwrap().is_uppercase() {
                self.non_terminals.insert(symbol.clone());
            } else if symbol != "ε" {
                self.terminals.insert(symbol.clone());
            }
        }

        self.productions.push(Production {
            non_terminal: non_terminal.to_string(),
            derivation,
        });
    }
}
