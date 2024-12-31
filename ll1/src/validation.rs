use std::collections::HashMap;

use super::{Grammar, ParsingTable, Production};

impl Grammar {
    /// Validates if the grammar is LL(1) using both FIRST/FOLLOW sets and Parsing Table.
    pub fn is_ll1(&self) -> bool {
        if self.is_ll1_first_follow() && self.is_ll1_parsing_table() {
            true
        } else {
            false
        }
    }

    /// Check if it is LL(1) using the ParseTable
    /// The parse table is built then
    /// if there are conflicts    -> it is not LL(1)
    /// if there are no conflicts -> it is LL(1)
    pub fn is_ll1_parsing_table(&self) -> bool {
        match ParsingTable::build(self) {
            Ok(_table) => true,
            Err(_e) => false,
        }
    }

    // Pseudocode for is_ll1 using only first follow sets without Parsing Table
    /*
        For each non-terminal (NT):
            For every pair of productions (P1, P2):
                If FIRST(P1) ∩ FIRST(P2) ≠ ∅:
                    Return false
                If ε ∈ FIRST(P1) and FIRST(P2) ∩ FOLLOW(NT) ≠ ∅:
                    Return false
                If ε ∈ FIRST(P2) and FIRST(P1) ∩ FOLLOW(NT) ≠ ∅:
                    Return false
        Return true
    */
    pub fn is_ll1_first_follow(&self) -> bool {
        let first_sets = self.compute_first_sets();
        let follow_sets = self.compute_follow_sets(&first_sets);

        // Group productions by their non-terminal symbols.
        let mut productions_by_nt: HashMap<String, Vec<&Production>> = HashMap::new();
        for production in &self.productions {
            productions_by_nt
                .entry(production.non_terminal.clone())
                .or_insert_with(Vec::new)
                .push(production);
        }

        // Iterate through each non-terminal and its associated productions.
        for (nt, productions) in &productions_by_nt {
            // Compare every pair of productions for the same non-terminal.
            for i in 0..productions.len() {
                let first_i = self.compute_first_of_string(&productions[i].derivation, &first_sets);

                for j in (i + 1)..productions.len() {
                    let first_j =
                        self.compute_first_of_string(&productions[j].derivation, &first_sets);

                    // --- Rule 1: FIRST sets must not overlap ---
                    // Ensure that the FIRST sets of two different productions are disjoint.
                    // If they overlap, the grammar is ambiguous and not LL(1).
                    if !first_i.is_disjoint(&first_j) {
                        return false;
                    }

                    // --- Rule 2: Handling ε (Empty String) ---
                    // If the FIRST set of one production contains ε:
                    // Ensure that the FOLLOW set of the non-terminal does not overlap with
                    // the FIRST set of the other production.

                    // Check if ε is in FIRST(i) and FOLLOW(NT) ∩ FIRST(j) ≠ ∅
                    if first_i.contains("ε") {
                        if let Some(follow) = follow_sets.get(nt) {
                            if !first_j.is_disjoint(follow) {
                                return false;
                            }
                        }
                    }

                    // Check if ε is in FIRST(j) and FOLLOW(NT) ∩ FIRST(i) ≠ ∅
                    if first_j.contains("ε") {
                        if let Some(follow) = follow_sets.get(nt) {
                            if !first_i.is_disjoint(follow) {
                                return false;
                            }
                        }
                    }
                }
            }
        }

        // If no violations of LL(1) conditions were detected, return true.
        true
    }
}
