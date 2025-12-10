**Factorial (Rust)**

A tiny Rust command-line program that calculates the factorial of a non-negative integer read from standard input.

**Overview**:

- **Language**: Rust
- **Location**: `src/main.rs`
- **Purpose**: Read a number from stdin and print its factorial value (u128).

**Features**:

- **Simple CLI**: Prompts for a number and prints the factorial.
- **u128 arithmetic**: Supports factorials up to the u128 range (use cautiously for large inputs).

**Build & Run**:

- **Build**: `cargo build --release`
- **Run (interactive)**: run and type a number when prompted.

Example (interactive):

```bash
cargo run
# then type e.g. 5 and press Enter
```

Example (non-interactive):

```bash
printf "5\n" | cargo run --quiet
```

**Expected output** (for input `5`):

```
Factorial sequence:
Enter a number to calculate its factorial value:
mul is 120
```

**Notes & Limitations**:

- The program reads a single integer and computes the factorial using a simple loop. It uses `u128` for intermediate results; very large inputs will overflow or be slow.
- The input is validated as an unsigned integer; non-numeric input prints an error and exits.

**Contributing**:

- Small project — feel free to open issues or PRs to add features (better CLI args, big integer support, tests).
