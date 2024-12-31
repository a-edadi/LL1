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

impl Production {
    /// Creates a new Production from a non-terminal and its derivation
    pub fn new(non_terminal: &str, derivation: Vec<&str>) -> Self {
        Self {
            non_terminal: non_terminal.to_string(),
            derivation: derivation.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Grammar {
    /// Creates a new Grammar with the specified start symbol
    pub fn new(start_symbol: &str) -> Self {
        Grammar {
            productions: Vec::new(),
            terminals: HashSet::new(),
            non_terminals: HashSet::new(),
            start_symbol: start_symbol.to_string(),
        }
    }

    /// Adds a production rule to the grammar
    pub fn add_production(&mut self, non_terminal: &str, derivation: Vec<&str>) {
        self.non_terminals.insert(non_terminal.to_string());

        let production = Production::new(non_terminal, derivation.clone());
        self.update_symbols(&derivation);
        self.productions.push(production);
    }

    /// Updates the terminal and non-terminal sets based on the derivation
    fn update_symbols(&mut self, derivation: &[&str]) {
        for symbol in derivation {
            if Self::is_non_terminal(symbol) {
                self.non_terminals.insert(symbol.to_string());
            } else if *symbol != "ε" {
                self.terminals.insert(symbol.to_string());
            }
        }
    }

    /// Creates a Grammar from a string representation
    pub fn from_string(input: &str, start_symbol: &str) -> Result<Self, Box<dyn Error>> {
        Self::validate_start_symbol(start_symbol)?;
        let mut grammar = Grammar::new(start_symbol);

        for (line_num, line) in input.lines().enumerate() {
            if let Some((non_terminal, alternatives)) = Self::parse_production_line(line) {
                Self::validate_non_terminal(&non_terminal, line_num)?;

                for alternative in alternatives {
                    let derivation = Self::parse_derivation(&alternative)?;
                    Self::validate_derivation(&derivation, line_num)?;
                    if !derivation.is_empty() {
                        grammar.add_production(&non_terminal, derivation);
                    }
                }
            }
        }

        Ok(grammar)
    }

    /// Creates a Grammar from a file
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<dyn Error>> {
        let content = Self::read_file_content(file_path)?;
        let start_symbol = Self::extract_start_symbol(&content)?;
        Grammar::from_string(&content, &start_symbol)
    }
}

/// Validators  and helpers
impl Grammar {
    fn parse_production_line(line: &str) -> Option<(String, Vec<&str>)> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let parts: Vec<&str> = line.split("->").collect();
        if parts.len() != 2 {
            return None;
        }

        let non_terminal = parts[0].trim().to_string();
        let alternatives = parts[1].trim().split('|').map(|s| s.trim()).collect();
        Some((non_terminal, alternatives))
    }

    fn parse_derivation(alternative: &str) -> Result<Vec<&str>, Box<dyn Error>> {
        Ok(alternative.split_whitespace().collect())
    }

    fn read_file_content<P: AsRef<Path>>(file_path: P) -> Result<String, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut content = String::new();
        for line in reader.lines() {
            content.push_str(&line?);
            content.push('\n');
        }
        Ok(content)
    }

    fn extract_start_symbol(content: &str) -> Result<String, Box<dyn Error>> {
        let start_symbol = content.lines().next().ok_or("Empty file")?;
        if !Self::is_non_terminal(start_symbol) {
            return Err("Start symbol must be uppercase (non-terminal)".into());
        }
        Ok(start_symbol.to_string())
    }

    /*

    Validators

    */

    /// Checks if a symbol is valid (either terminal, non-terminal, or epsilon)
    pub fn is_valid_symbol(symbol: &str) -> bool {
        symbol == "ε" || Self::is_terminal(symbol) || Self::is_non_terminal(symbol)
    }

    fn is_terminal(symbol: &str) -> bool {
        symbol.chars().all(|c| c.is_lowercase())
    }

    fn is_non_terminal(symbol: &str) -> bool {
        symbol.chars().all(|c| c.is_uppercase())
    }

    fn validate_start_symbol(start_symbol: &str) -> Result<(), Box<dyn Error>> {
        if !Self::is_non_terminal(start_symbol) {
            return Err("Start symbol must be uppercase (non-terminal)".into());
        }
        Ok(())
    }

    fn validate_non_terminal(non_terminal: &str, line_num: usize) -> Result<(), Box<dyn Error>> {
        if !Self::is_non_terminal(non_terminal) {
            return Err(format!(
                "Error on line {}: Non-terminal '{}' must be uppercase",
                line_num + 1,
                non_terminal
            )
            .into());
        }
        Ok(())
    }

    fn validate_derivation(derivation: &[&str], line_num: usize) -> Result<(), Box<dyn Error>> {
        for symbol in derivation {
            if !Self::is_valid_symbol(symbol) {
                return Err(format!(
                    "Error on line {}: Invalid symbol '{}' in derivation",
                    line_num + 1,
                    symbol
                )
                .into());
            }
        }
        Ok(())
    }
}
