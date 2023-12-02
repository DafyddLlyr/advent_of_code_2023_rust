# Advent of Code 2023 - Rust Solutions

Welcome to my Advent of Code 2023 repository! This project is a collection of daily solutions to the Advent of Code challenges, each implemented in Rust. As I tackle the challenges, my primary goal is to learn and improve my Rust programming skills. Feel free to explore the solutions and provide feedback or suggestions.

## Table of Contents

- [Advent of Code 2023 - Rust Solutions](#advent-of-code-2023---rust-solutions)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Project Structure](#project-structure)
  - [Usage](#usage)

## Introduction

[Advent of Code](https://adventofcode.com/) is an annual coding event where participants solve a series of programming challenges throughout the month of December. This repository serves as my personal learning journey, where I implement solutions to each day's challenge using Rust. Rust's focus on performance, safety, and concurrency makes it an ideal language for honing programming skills.

## Project Structure

Each day's challenge is organized into a separate Cargo project. I'm following the project structure suggested in [Chris Biscardi's](https://github.com/ChristopherBiscardi) excellent YouTube video ["How to set up Rust for Advent of Code"](https://www.youtube.com/watch?v=fEQv-cqzbPg).

```
advent_of_code_2023_rust/
│
├── day_01/
│   ├── src/
│   │   ├── bin/
│   │   │   ├── part1.rs
│   │   │   ├── part2.rs
├── day_02/
│   ├── src/
│   │   ├── bin/
│   │   │   ├── part1.rs
│   │   │   ├── part2.rs
├── ...

```

## Usage

Navigate to the specific day's directory and run the corresponding Cargo project:

```bash
cd day_01
cargo run --bin part1 # Solution to part 1
cargo run --bin part2 # Solution to part 2
cargo test # Daily tests
```

Replace `day_01` with the desired day's directory.