# LoopGuard

[![Crate](https://img.shields.io/crates/v/loop-guard.svg)](https://crates.io/crates/loop-guard)
[![Test Status](https://github.com/nbry/loop-guard/actions/workflows/test-status.yml/badge.svg)](https://github.com/nbry/loop-guard/actions/workflows/test-status.yml)

Simple development utility to prevent infinite loops.

Honestly, I made this crate solely to practice with cargo

## Installation

Add this to your `Cargo.toml`:

```
[dependencies]
loop_guard = "1.0.0"
```

## Usage

```rust
use loop_guard::LoopGuard;

fn main() {
    // This LoopGuard instance will prevent a loop from running more than 50 times.
    let mut guard = LoopGuard::new(50);

    // However, this for loop won't cause a panic, because it only loops 10 times.
    for _i in 0..10 {
        guard.protect()
    }

    // There isn't a way to "reset" a guard instance
    // If you need another guarded loop, create a new instance of LoopGuard
    // We'll make a new instance, this time with a custom panic message

    let mut guard_2 = LoopGuard::new(100).set_panic_message("Oh no! This failed!");

    // This (infinite) loop is protected by guard_2, and will panic after 100 runs
    loop {
        guard_2.protect();
    }
}
```

Run this code:
```
$ cargo run
```

Result:
```
thread 'main' panicked at 'Oh no! This failed!', ...
```

...that's pretty much about it.

# License

LoopGuard is distributed under the terms of the MIT license.

