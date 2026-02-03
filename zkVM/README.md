# Range Proof zkVM Tutorial

## What You'll Build

A ~20-line zero-knowledge range proof program that proves a secret value lies within a specified range without revealing the exact value.

**Example use case:** Prove you're 18+ without revealing your exact age!

## Objectives

- Write a Risc0 zkVM guest program
- Data flows between host and guest
- Zero-knowledge property in action
- Proofs are generated and verified

## Step 1: Understanding the Guest Program

**Filed being modified:** `methods/guest/src/main.rs`

The guest program runs inside the zkVM and performs the private computation. Here's what you'll build:

```rust
use risc0_zkvm::guest::env;

fn main() {
    // Read private inputs from host
    let secret_value: u32 = env::read();
    let min_range: u32 = env::read();
    let max_range: u32 = env::read();

    // Perform range check (proven but inputs stay private)
    let is_in_range = secret_value >= min_range && secret_value <= max_range;

    // Commit public outputs to journal
    env::commit(&min_range);
    env::commit(&max_range);
    env::commit(&is_in_range);
}
```

**Key concepts:**
- `env::read()` - Reads data from the host (in order)
- The computation happens inside the zkVM (private)
- `env::commit()` - Writes results to the public journal
- **The secret_value never appears in the journal!**

**Your task:** Replace the TODO template with this range check logic (~17 lines total)

