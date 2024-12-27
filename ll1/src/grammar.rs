use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();

        // Read and validate start symbol from first line
        let start_symbol = lines.next().ok_or("Empty file")??;
        if !start_symbol.chars().all(|c| c.is_uppercase()) {
            return Err("Start symbol must be uppercase (non-terminal)".into());
        }
        let mut grammar = Grammar::new(&start_symbol);

        // Read productions
        for (line_num, line) in lines.enumerate() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split("->").collect();
            if parts.len() != 2 {
                continue; // Skip invalid lines
            }

            let non_terminal = parts[0].trim();
            // Validate non-terminal is uppercase
            if !non_terminal.chars().all(|c| c.is_uppercase()) {
                return Err(format!(
                    "Error on line {}: Non-terminal '{}' must be uppercase",
                    line_num + 2, // +2 because we start counting from 1 and already read the start symbol
                    non_terminal
                )
                .into());
            }

            let right_side = parts[1].trim();

            // Split by pipe to handle alternative productions
            let alternatives: Vec<&str> = right_side.split('|').map(|s| s.trim()).collect();

            // Add each alternative as a separate production
            for alternative in alternatives {
                let derivation: Vec<&str> = alternative.split_whitespace().collect();

                if !derivation.is_empty() {
                    // Validate each symbol in the derivation
                    for symbol in &derivation {
                        if *symbol != "ε" {
                            // Skip validation for epsilon
                            let is_valid = if symbol.chars().all(|c| c.is_uppercase()) {
                                true // Non-terminal
                            } else if symbol.chars().all(|c| c.is_lowercase()) {
                                true // Terminal
                            } else {
                                false // Mixed case or invalid
                            };

                            if !is_valid {
                                return Err(format!(
                                    "Error on line {}: Symbol '{}' must be either all uppercase (non-terminal) or all lowercase (terminal)",
                                    line_num + 2,
                                    symbol
                                ).into());
                            }
                        }
                    }
                    grammar.add_production(non_terminal, derivation);
                }
            }
        }

        Ok(grammar)
    }
}
