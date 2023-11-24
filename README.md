# Advent of Code 2023

[![Advent of Code](https://github.com/rikharink/adventofcode/actions/workflows/aoc.yml/badge.svg)](https://github.com/rikharink/adventofcode/actions/workflows/aoc.yml)

## Run

```sh
cargo run
cargo run day01
cargo run all
```

## Run tests

```sh
cargo test
cargo test day01
```

## Run benchmarks

```sh
cargo install cargo-criterion
cargo criterion
cargo criterion day01
```

## Add template for day

```sh
# add templates up to $NUMBER_OF_DAYS
# eg cargo run --bin setup 25 for all the days
# if it is december it will take the current day by default
cargo run --bin setup $NUMBER_OF_DAYS
```

## See benchmarks

[benchmark page](https://rikharink.github.io/adventofcode/index.html)
