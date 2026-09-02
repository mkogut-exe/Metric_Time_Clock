# Metric Time Clock

A Rust implementation of a simple clock that displays the current time in **metric time**.

Metric time is a decimal-based way of measuring the day, where:

- 1 day = 10 metric hours
- 1 metric hour = 100 metric minutes
- 1 metric minute = 100 metric seconds

This project converts the current local time into a metric-style `HH:MM:SS` display.

## Features

- Reads the current time and converts it to metric time
- Supports a configurable UTC offset
- Prints the metric time to the terminal
- Includes reusable helper functions for formatting and conversion

## Example

```text
Metric Time: 03:42:18
```

## Getting started

### Prerequisites

- Rust toolchain installed

### Run

```bash
cargo run
```

## Project structure

- `src/main.rs` — program entry point
- `src/metric_clock.rs` — metric time conversion logic

## Notes

The project uses the `chrono` crate for working with local time.

## Reference

- [Metric time on Wikipedia](https://en.wikipedia.org/wiki/Metric_time)
