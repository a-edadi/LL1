use super::Grammar;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, ContentArrangement, Table};
use std::collections::HashMap;
use std::fmt;

/// Represents an LL(1) Parsing Table
#[derive(Debug, Clone)]
pub struct ParsingTable {
    pub table: HashMap<(String, String), Vec<String>>,
    non_terminals: Vec<String>,
    terminals: Vec<String>,
}

impl ParsingTable {
    /// Build a Parsing Table from a given Grammar
    pub fn build(grammar: &Grammar) -> Result<Self, String> {
        let first_sets = grammar.compute_first_sets();
        let follow_sets = grammar.compute_follow_sets(&first_sets);

        let mut table: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut conflicts = false;

        // Add $ to terminals for the parsing table
        let mut terminals = grammar.terminals.clone();
        terminals.insert("$".to_string());

        // Prepare sorted vectors for display
        let mut terminals_vec: Vec<String> = terminals.into_iter().collect();
        terminals_vec.sort();

        let mut non_terminals_vec: Vec<String> =
            grammar.non_terminals.clone().into_iter().collect();
        non_terminals_vec.sort();

        // Build the parsing table
        for production in &grammar.productions {
            let nt = &production.non_terminal;
            let first_of_rhs = grammar.compute_first_of_string(&production.derivation, &first_sets);

            for terminal in &first_of_rhs {
                if terminal != "ε" {
                    let key = (nt.clone(), terminal.clone());
                    if table.contains_key(&key) {
                        conflicts = true;
                    }
                    table.insert(key, production.derivation.clone());
                }
            }

            // If ε is in FIRST(α), add production to FOLLOW(A)
            if first_of_rhs.contains("ε") {
                if let Some(follow_set) = follow_sets.get(nt) {
                    for terminal in follow_set {
                        let key = (nt.clone(), terminal.clone());
                        if table.contains_key(&key) {
                            conflicts = true;
                        }
                        table.insert(key, production.derivation.clone());
                    }
                }
            }
        }

        if conflicts {
            return Err("Grammar is not LL(1) - parsing table has conflicts".to_string());
        }

        Ok(Self {
            table,
            non_terminals: non_terminals_vec,
            terminals: terminals_vec,
        })
    }

    /// Display the Parsing Table as a formatted table
    pub fn to_comfy_table(&self) -> Table {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        // Header Row
        let mut header_row = vec![Cell::new("📋").add_attribute(Attribute::Bold)];
        header_row.extend(
            self.terminals
                .iter()
                .map(|t| Cell::new(t.as_str()).add_attribute(Attribute::Bold)),
        );
        table.add_row(header_row);

        // Data Rows
        for nt in &self.non_terminals {
            let mut row = vec![Cell::new(nt.as_str())];
            for terminal in &self.terminals {
                let content = self
                    .table
                    .get(&(nt.clone(), terminal.clone()))
                    .map(|prod| {
                        if prod.is_empty() {
                            "_".to_string()
                        } else {
                            prod.join(" ")
                        }
                    })
                    .unwrap_or_else(|| "_".to_string());
                row.push(Cell::new(content));
            }
            table.add_row(row);
        }

        table
    }
}

impl fmt::Display for ParsingTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table = self.to_comfy_table();
        write!(f, "{}", table)
    }
}
