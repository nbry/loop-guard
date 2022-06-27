# LoopGuard

Simple development utility to prevent infinite loops.

## Installation

Add this to your `Cargo.toml`:

```
[dev-dependencies]
loop_guard = "0.1.2"
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

