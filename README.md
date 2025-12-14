# Change Dispenser Algorithm

## Description
This project implements a simple change-dispensing algorithm using fixed denominations:


The algorithm returns the minimum number of notes or coins required to give exact change for a given amount.

---

## How It Works
- Uses a greedy approach
- Starts with the largest denomination
- Subtracts values until the remaining amount is zero
- Stops if exact change cannot be produced

---

## Usage
- Input: total change amount
- Output: list or count of denominations used

---

## Notes
- Amount must be divisible by 5
- Assumes sufficient denominations are available

---

## Future Improvements

- **Inventory Awareness**
  Track available count per denomination and refuse change if exact denominations are unavailable.

- **Error Handling**
  Return a `Result` instead of panicking when exact change cannot be made.

- **Configurable Denominations**
  Load supported denominations dynamically instead of hardcoding values.

- **Embedded Integration**
  Interface with real coin and bill dispensers using GPIO control.

- **Transaction Logging**
  Log all change-dispensing operations for auditing and debugging.

- **Timeout & Retry Logic**
  Detect dispenser failures and retry safely.

- **Unit Testing**
  Add test cases for edge conditions such as zero amount and insufficient change.

- **Power-Failure Recovery**
  Persist dispenser state to handle unexpected shutdowns.

- **Multi-Currency Support**
  Extend the algorithm to support different currencies and denomination sets.


## Language
- Rust
