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

## Language
- Rust
