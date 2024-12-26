use std::collections::{HashMap, HashSet};

use super::{Grammar, Production};

impl Grammar {
    pub fn compute_first_sets(&self) -> HashMap<String, HashSet<String>> {
        let mut first_sets: HashMap<String, HashSet<String>> = HashMap::new();

        for terminal in &self.terminals {
            let mut set = HashSet::new();
            set.insert(terminal.clone());
            first_sets.insert(terminal.clone(), set);
        }

        for non_terminal in &self.non_terminals {
            first_sets.insert(non_terminal.clone(), HashSet::new());
        }

        let mut changed = true;
        while changed {
            changed = false;

            for production in &self.productions {
                let nt = &production.non_terminal;

                if production.derivation[0] == "ε" {
                    if first_sets.get_mut(nt).unwrap().insert("ε".to_string()) {
                        changed = true;
                    }
                    continue;
                }

                let mut all_nullable = true;
                let mut first_set = HashSet::new();

                for symbol in &production.derivation {
                    if let Some(symbol_first) = first_sets.get(symbol) {
                        for terminal in symbol_first {
                            if terminal != "ε" {
                                first_set.insert(terminal.clone());
                            }
                        }

                        if !symbol_first.contains("ε") {
                            all_nullable = false;
                            break;
                        }
                    }
                }

                if all_nullable {
                    first_set.insert("ε".to_string());
                }

                if let Some(first_set_entry) = first_sets.get_mut(nt) {
                    for item in first_set {
                        if first_set_entry.insert(item) {
                            changed = true;
                        }
                    }
                }
            }
        }

        first_sets
    }

    pub fn compute_follow_sets(
        &self,
        first_sets: &HashMap<String, HashSet<String>>,
    ) -> HashMap<String, HashSet<String>> {
        let mut follow_sets: HashMap<String, HashSet<String>> = HashMap::new();

        // Initialize FOLLOW sets
        for non_terminal in &self.non_terminals {
            follow_sets.insert(non_terminal.clone(), HashSet::new());
        }

        // Add $ to follow set of start symbol
        follow_sets
            .get_mut(&self.start_symbol)
            .unwrap()
            .insert("$".to_string());

        let mut changed = true;
        while changed {
            changed = false;
            let mut updates: Vec<(String, String)> = Vec::new();

            for production in &self.productions {
                let nt = &production.non_terminal;

                for i in 0..production.derivation.len() {
                    let current = &production.derivation[i];

                    if self.non_terminals.contains(current) {
                        let mut first_of_rest = HashSet::new();
                        let mut all_nullable = true;

                        // Compute FIRST of everything that follows
                        for j in (i + 1)..production.derivation.len() {
                            let symbol = &production.derivation[j];
                            if let Some(symbol_first) = first_sets.get(symbol) {
                                for terminal in symbol_first {
                                    if terminal != "ε" {
                                        first_of_rest.insert(terminal.clone());
                                    }
                                }

                                if !symbol_first.contains("ε") {
                                    all_nullable = false;
                                    break;
                                }
                            }
                        }

                        // Collect updates instead of modifying follow_sets directly
                        for terminal in &first_of_rest {
                            updates.push((current.clone(), terminal.clone()));
                        }

                        if all_nullable || i == production.derivation.len() - 1 {
                            if let Some(follow_of_lhs) = follow_sets.get(nt) {
                                for terminal in follow_of_lhs {
                                    updates.push((current.clone(), terminal.clone()));
                                }
                            }
                        }
                    }
                }
            }

            // Apply all updates at once
            for (non_terminal, terminal) in updates {
                if let Some(follow_set) = follow_sets.get_mut(&non_terminal) {
                    if follow_set.insert(terminal) {
                        changed = true;
                    }
                }
            }
        }

        follow_sets
    }

    pub fn compute_first_of_string(
        &self,
        string: &[String],
        first_sets: &HashMap<String, HashSet<String>>,
    ) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut all_nullable = true;

        for symbol in string {
            if let Some(symbol_first) = first_sets.get(symbol) {
                for terminal in symbol_first {
                    if terminal != "ε" {
                        result.insert(terminal.clone());
                    }
                }

                if !symbol_first.contains("ε") {
                    all_nullable = false;
                    break;
                }
            }
        }

        if all_nullable {
            result.insert("ε".to_string());
        }

        result
    }

    // Pseudocode for is_ll1
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

    pub fn is_ll1(&self) -> bool {
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
