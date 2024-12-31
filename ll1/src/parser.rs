use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Write};

use super::{Grammar, ParsingTable};
pub struct Parser {
    grammar: Grammar,
    parsing_table: ParsingTable,
    input: Vec<char>,
    follow_sets: HashMap<String, HashSet<String>>,
}

impl Parser {
    /// Creates a new Parser instance from a Grammar
    pub fn new(grammar: Grammar) -> Result<Self, String> {
        let first_sets = grammar.compute_first_sets();
        let follow_sets = grammar.compute_follow_sets(&first_sets);
        let parsing_table = ParsingTable::build(&grammar)?;

        Ok(Parser {
            grammar,
            parsing_table,
            input: Vec::new(),
            follow_sets,
        })
    }

    /// Set the input string to be parsed
    pub fn set_input(&mut self, input: String) {
        self.input = input.chars().collect();
    }

    /// Get the current input as a string
    pub fn get_input(&self) -> String {
        self.input.iter().collect()
    }

    /// Takes user input via stdin
    pub fn set_input_io(&mut self) {
        print!("Please enter the input string: \n>");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        self.set_input(input.trim().to_string());
    }

    /// Validates if current stack and input positions are aligned
    fn validate_alignment(&self, stack: &VecDeque<String>, input_pos: usize) -> bool {
        if stack.is_empty() || input_pos >= self.input.len() {
            return false;
        }

        let top = stack.back().unwrap();
        let current_input = self.input[input_pos].to_string();

        // Check if top terminal matches current input
        if self.grammar.terminals.contains(top) {
            return top == &current_input;
        }

        // For non-terminals, check if current input can be derived
        if self.grammar.non_terminals.contains(top) {
            return self
                .parsing_table
                .table
                .contains_key(&(top.clone(), current_input));
        }

        false
    }

    /// Advanced error recovery with validation
    fn recover(
        &self,
        stack: &mut VecDeque<String>,
        input_pos: &mut usize,
        error: &str,
    ) -> Result<bool, String> {
        println!("Error: {}. Attempting recovery...", error);

        let original_pos = *input_pos;
        let mut recovery_successful = false;

        // Try different recovery strategies
        for strategy in 1..=3 {
            match strategy {
                1 => {
                    // Strategy 1: Skip input until synchronization token
                    let mut temp_pos = *input_pos;
                    let temp_stack = stack.clone();

                    while temp_pos < self.input.len() {
                        if self.validate_alignment(&temp_stack, temp_pos) {
                            *input_pos = temp_pos;
                            *stack = temp_stack.clone();
                            recovery_successful = true;
                            println!("Recovered by skipping input to: {}", self.input[temp_pos]);
                            break;
                        }
                        temp_pos += 1;
                    }
                }
                2 => {
                    // Strategy 2: Pop stack symbols until alignment
                    let mut temp_stack = stack.clone();

                    while !temp_stack.is_empty() {
                        if self.validate_alignment(&temp_stack, *input_pos) {
                            *stack = temp_stack.clone();
                            recovery_successful = true;
                            println!("Recovered by popping stack to: {:?}", stack.back().unwrap());
                            break;
                        }
                        temp_stack.pop_back();
                    }
                }
                3 => {
                    // Strategy 3: Use pre-computed sync tokens from FOLLOW sets
                    let sync_tokens = if let Some(top) = stack.back() {
                        self.follow_sets.get(top).cloned().unwrap_or_default()
                    } else {
                        HashSet::new()
                    };

                    let mut temp_pos = *input_pos;
                    while temp_pos < self.input.len() {
                        let current = self.input[temp_pos].to_string();
                        if sync_tokens.contains(&current) {
                            stack.pop_back();
                            *input_pos = temp_pos;
                            recovery_successful = true;
                            break;
                        }
                        temp_pos += 1;
                    }
                }
                _ => unreachable!(),
            }

            if recovery_successful {
                break;
            }
        }

        // Verify recovery was successful
        if recovery_successful {
            if self.validate_alignment(stack, *input_pos) {
                println!("Recovery validation successful");
                return Ok(true);
            } else {
                // Rollback if validation fails
                *input_pos = original_pos;
                println!("Recovery validation failed, rolling back");
                return Ok(false);
            }
        }

        println!("All recovery strategies failed");
        Ok(false)
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let mut stack: VecDeque<String> = VecDeque::new();
        stack.push_back("$".to_string());
        stack.push_back(self.grammar.start_symbol.clone());

        if self.input.last() != Some(&'$') {
            self.input.push('$');
        }

        let mut input_pos = 0;
        let mut error_count = 0;
        const MAX_ERRORS: usize = 10;

        while !stack.is_empty() && input_pos < self.input.len() {
            if error_count >= MAX_ERRORS {
                return Err("Too many errors encountered. Aborting parse.".to_string());
            }

            let current_input = self.input[input_pos].to_string();
            self.print_state(&stack, input_pos);

            let top = stack.pop_back().ok_or("Stack unexpectedly empty")?;

            if self.grammar.terminals.contains(&top) || top == "$" {
                if top == current_input {
                    input_pos += 1;
                } else {
                    error_count += 1;
                    stack.push_back(top.clone());
                    // Comment this if statement to avoid error recovery
                    if !self.recover(
                        &mut stack,
                        &mut input_pos,
                        &format!(
                            "Terminal mismatch: expected {}, found {}",
                            top, current_input
                        ),
                    )? {
                        return Err("Unable to recover from error".to_string());
                    }
                }
            } else if self.grammar.non_terminals.contains(&top) {
                match self
                    .parsing_table
                    .table
                    .get(&(top.clone(), current_input.clone()))
                {
                    Some(production) => {
                        for symbol in production.iter().rev() {
                            if symbol != "ε" {
                                stack.push_back(symbol.clone());
                            }
                        }
                    }
                    None => {
                        error_count += 1;
                        stack.push_back(top.clone());
                        // Comment this if statement to avoid error recovery
                        if !self.recover(
                            &mut stack,
                            &mut input_pos,
                            &format!("No production found for ({}, {})", top, current_input),
                        )? {
                            return Err("Unable to recover from error".to_string());
                        }
                    }
                }
            } else {
                return Err(format!("Invalid symbol on stack: {}", top));
            }
        }

        // Fixed final validation:
        // The parse is successful if we've consumed all meaningful input
        // (except possibly $) and the stack is either empty or only contains the end marker
        if (input_pos == self.input.len()
            || (input_pos == self.input.len() - 1 && self.input[input_pos] == '$'))
            && (stack.is_empty() || (stack.len() == 1 && stack.back() == Some(&"$".to_string())))
        {
            if error_count > 0 {
                println!("Parsing completed with {} error(s) recovered", error_count);
            }
            Ok(())
        } else {
            Err("Parsing failed: incomplete parse or extra input".to_string())
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
