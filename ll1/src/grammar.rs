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

    pub fn is_valid_symbol(symbol: &str) -> bool {
        if symbol == "ε" {
            return true;
        }
        symbol.chars().all(|c| c.is_lowercase()) || symbol.chars().all(|c| c.is_uppercase())
    }

    // Function to read the grammar from a string
    pub fn from_string(input: &str, start_symbol: &str) -> Result<Self, Box<dyn Error>> {
        if !start_symbol.chars().all(|c| c.is_uppercase()) {
            return Err("Start symbol must be uppercase (non-terminal)".into());
        }

        let mut grammar = Grammar::new(start_symbol);

        for (line_num, line) in input.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue; // Skip empty lines
            }

            // Split the production by the '->' symbol
            let parts: Vec<&str> = line.split("->").collect();
            if parts.len() != 2 {
                continue; // Skip invalid lines
            }

            let non_terminal = parts[0].trim();
            
            // Validate non-terminal is uppercase
            if !non_terminal.chars().all(|c| c.is_uppercase()) {
                return Err(format!(
                    "Error on line {}: Non-terminal '{}' must be uppercase",
                    line_num + 1,
                    non_terminal
                )
                .into());
            }

            let right_side = parts[1].trim();
            let alternatives: Vec<&str> = right_side.split('|').map(|s| s.trim()).collect();

            // Add each alternative as a separate production
            for alternative in alternatives {
                let derivation: Vec<&str> = alternative.split_whitespace().collect();

                if !derivation.is_empty() {
                    // Validate the derivation
                    for symbol in &derivation {
                        if !Grammar::is_valid_symbol(symbol) {
                            return Err(format!(
                                "Error on line {}: Invalid symbol '{}' in derivation",
                                line_num + 1,
                                symbol
                            )
                            .into());
                        }
                    }
                    grammar.add_production(non_terminal, derivation);
                }
            }
        }

        Ok(grammar)
    }

    /// Function to read the grammar from a file
    /// utilizes from_string
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut content = String::new();
        for line in reader.lines() {
            content.push_str(&line?);
            content.push('\n');
        }

        // Extract the start symbol
        let start_symbol = content.lines().next().ok_or("Empty file")?;

        // Ensure the start symbol is uppercase
        if !start_symbol.chars().all(|c| c.is_uppercase()) {
            return Err("Start symbol must be uppercase (non-terminal)".into());
        }

        // Use from_string to parse the grammar from the string content
        Grammar::from_string(&content, start_symbol)
    }
}
