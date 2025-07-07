# Polkadot-Africa-Rust-Bootcamp

## Week 1: Balance Pallet

This project implements a simple balance management pallet in Rust, inspired by Substrate's balances module. It is designed as an educational exercise for the Polkadot Africa Rust Bootcamp.

### Features
- **Account Balances:** Store and query balances for accounts (identified by `String`).
- **Set Balance:** Directly set the balance of any account.
- **Transfer with Fee:** Transfer tokens from one account to another, deducting a transaction fee from the sender.
- **Overflow and Underflow Safety:** Uses checked arithmetic to prevent overflows and underflows.
- **Unit Tests:** Includes tests for initialization, balance setting, and transfers (including error cases).

### How It Works
- Balances are stored in a `BTreeMap<String, u128>`.
- The `transfer` function requires the sender to have enough balance for both the transfer amount and the transaction fee.
- If the sender does not have enough for `amount + fee`, the transfer fails.
- The receiver only receives the `amount` (not the fee).

### Example Usage
```rust
let mut pallet = Pallet::new();
pallet.set_balance(&"alice".to_string(), 100);
let result = pallet.transfer("alice".to_string(), "bob".to_string(), 50, 2);
assert_eq!(result, Ok(()));
assert_eq!(pallet.balance(&"alice".to_string()), 48); // 100 - 50 - 2
assert_eq!(pallet.balance(&"bob".to_string()), 50);
```

### Running the Tests
To run the included unit tests, use:
```bash
cargo test
```
This will compile the project and run all tests in `src/balances.rs`.

### File Structure
- `src/balances.rs`: Main logic for the balance pallet and its tests.
- `src/main.rs`: (Empty or for future use)
- `Cargo.toml`: Project configuration.

---

