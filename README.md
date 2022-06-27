# LoopGuard

[![Crate](https://img.shields.io/crates/v/loop-guard.svg)](https://crates.io/crates/loop-guard)
[![Test Status](https://github.com/nbry/loop-guard/actions/workflows/test-status.yml/badge.svg)](https://github.com/nbry/loop-guard/actions/workflows/test-status.yml)

Simple development utility to prevent infinite loops.

## Installation

Add this to your `Cargo.toml`:

```
[dev-dependencies]
loop_guard = "0.1.4"
```

## Usage

```
use::loop_guard::LoopGuard;

fn main () {
    let guard = LoopGuard::new(100);

    loop {
        guard.protect() // This will panic after 100 runs of the loop
    };
}
```

...that's pretty much about it.

# License

LoopGuard is distributed under the terms of the MIT license.

