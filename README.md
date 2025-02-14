# LL(1)

## Overview

This project is a simple LL(1) parser that supports error recovery using panic mode. It reads a given context-free grammar (CFG) and parses input based on LL(1) parsing techniques. If parsing errors occur, the parser attempts to recover gracefully.

## Input grammar

The parser accepts a grammar definition from either a file or a string.

### 1.From File

You can load a grammar from a file by providing the file path:

```rust
// from file (file path)
let grammar = Grammar::from_file("src/input.txt")?;
```

Example: `input.txt`

```text
S
S -> A a B
A -> b A | ε
B -> c B | ε
```

### 2.From String

Alternatively, you can define a grammar inline as a string:

```rust
// from string (Grammar_string , Start_symbol)
let grammar = Grammar::from_string("A -> B", "A")?;
```

## Running the project

To run the parser, execute the following command:

```bash
cargo run
```

After starting, you can provide text input that conforms to the grammar. The parser will attempt to parse the input and will handle errors using panic mode error recovery.

### Error Handling

- The parser will detect syntax errors and attempt to recover using panic mode.

- If the parser encounters an unrecognized token, it will skip to a known synchronization point.

## collaborators

- Amirhossein Edadi
- Amin Sheikh Azimi
- Amin OwrangPour
