# Punching Table

A simple command-line arithmetic quiz written in Rust.

## Run

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo run
```

## Gameplay

The game generates random:

* Addition
* Subtraction
* Multiplication
* Division with no fractional answers

Division questions are generated so the result is always a whole number.

Example:

```text
### Punching Table ###

What is 8 * 7
= 56
Correct!

Wanna continue? [y/n]
```

## Built With

* Rust
* Cargo
* `rand`
