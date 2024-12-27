use std::collections::VecDeque;
use std::io::{self, Write};

use super::{Grammar, ParsingTable};

/// Represents the LL(1) parser
pub struct Parser {
    grammar: Grammar,
    parsing_table: ParsingTable,
    input: Vec<char>,
}

impl Parser {
    /// Creates a new Parser instance from a Grammar
    pub fn new(grammar: Grammar) -> Result<Self, String> {
        let parsing_table = ParsingTable::build(&grammar)?;
        Ok(Parser {
            grammar,
            parsing_table,
            input: Vec::new(),
        })
    }

    /// Set the input string to be parsed (convert to Vec<char>)
    pub fn set_input(&mut self, input: String) {
        self.input = input.chars().collect(); // Convert input string to Vec<char>
    }

    /// Get the current input as a string
    pub fn get_input(&self) -> String {
        self.input.iter().collect() // Convert Vec<char> back to String
    }

    /// Fetch input from the user via stdin
    pub fn set_input_io(&mut self) {
        print!("Please enter the input string: \n>");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Remove any trailing newline characters
        let input = input.trim();

        // Set the input using the set_input method
        self.set_input(input.to_string());
    }

    /// Parses the input using the LL(1) parsing algorithm
    pub fn parse(&mut self) -> Result<(), String> {
        // Initialize the stack with the end marker and start symbol
        let mut stack: VecDeque<String> = VecDeque::new();
        stack.push_back("$".to_string()); // End marker
        stack.push_back(self.grammar.start_symbol.clone());

        // Add end marker to input if not already present
        if self.input.last() != Some(&'$') {
            self.input.push('$');
        }

        let mut input_pos = 0;

        while !stack.is_empty() {
            let current_input = if input_pos < self.input.len() {
                self.input[input_pos].to_string()
            } else {
                return Err("Unexpected end of input".to_string());
            };

            self.print_state(&stack, input_pos);

            let top = stack
                .pop_back()
                .ok_or("Stack unexpectedly empty".to_string())?;

            if self.grammar.terminals.contains(&top) || top == "$" {
                if top == current_input {
                    input_pos += 1;
                } else {
                    return Err(format!(
                        "Terminal mismatch: expected {}, found {}",
                        top, current_input
                    ));
                }
            } else if self.grammar.non_terminals.contains(&top) {
                let production = self
                    .parsing_table
                    .table
                    .get(&(top.clone(), current_input.clone()))
                    .ok_or(format!(
                        "No production found for ({}, {})",
                        top, current_input
                    ))?;

                for symbol in production.iter().rev() {
                    if symbol != "ε" {
                        stack.push_back(symbol.clone());
                    }
                }
            } else {
                return Err(format!("Invalid symbol on stack: {}", top));
            }
        }

        if input_pos == self.input.len() {
            Ok(())
        } else {
            Err(format!(
                "Input not fully processed. Remaining: {}",
                self.input.iter().skip(input_pos).collect::<String>()
            ))
        }
    }

    /// Helper method to print the current parsing state
    fn print_state(&self, stack: &VecDeque<String>, input_pos: usize) {
        println!("Stack: {:?}", stack);
        println!(
            "Input remaining: {}",
            self.input.iter().skip(input_pos).collect::<String>()
        );
        println!("---");
    }
}
