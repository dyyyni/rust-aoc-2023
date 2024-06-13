# Advent of Code 2023 in Rust

This repository contains my solutions for the Advent of Code 2023 challenges written in Rust.

## Getting started

Ensure that you have Rust installed on your system.

## Project Structure

The project is organized into multiple modules, each corresponding to a day of the Advent of Code challenge:

- `day<n>`  : Solutions for Day `<n>`
- `utils`   : Utility functions used across different days
- `main`    : Simple command line interface to run different solutions

## Running the Code

To run the code, use the following command format:

```sh
cargo run -- <day> <part>
```
## Adding New Solutions

1. Create a new module file in the `src` directory (e.g. day04.rs).
2. Implement the functions `run_part1` and `run_part2` as you go.
3. Update the `main` function in `src/main.rs` to include the new solution and its parts in the match statement.