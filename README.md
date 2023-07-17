# Learning Rust with Advent of Code

This repository contains my attempts at learning Rust through solving Advent of Code 2022 problems.

## Running Tests

```bash
cargo test
```

## Downloading Problems

First, install [Advent of Code CLI](https://github.com/scarvalhojr/aoc-cli/):

```bash
cargo install aoc-cli
```

Download a single day:
```bash
day=01 && (mkdir -p "src/day${day}" && cd $_ && aoc d --overwrite --day "$day" --input-file "input.txt" --puzzle-file "README.md")
```

Download all days:
```bash
for day in {01..25}; do (mkdir -p "src/day${day}" && cd $_ && aoc d --overwrite --day "$day" --input-file "input.txt" --puzzle-file "README.md"); done
```
