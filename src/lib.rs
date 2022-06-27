/// Utility for preventing infinite loops
///
/// Use as a guard rail inside a `loop` or `while` block
///
/// Meant to be used as a development dependency
pub struct LoopGuard {
    pub max_ticks: i32,
    pub count: i32,
}

impl LoopGuard {
    /// # Create a new LoopGuard
    /// Make sure to create the LoopGuard instance **outside** the loop block
    ///
    /// ## Arguments
    /// * `max_ticks` - The limit of how many times a loop should run
    ///
    /// ```should_panic
    /// use loop_guard::LoopGuard;
    ///
    /// let mut guard = LoopGuard::new(10);
    ///
    /// loop {
    ///     guard.protect() // This panic after 10 loops
    /// }
    /// ```
    pub fn new(max: i32) -> LoopGuard {
        LoopGuard {
            max_ticks: max,
            count: 0,
        }
    }

    /// From within a `while` or `loop` block, panic if the loop iteration
    /// surpasses the LoopGuard's `max_ticks` value.
    pub fn protect(&mut self) {
        self.count += 1;
        if self.count > self.max_ticks {
            panic!("Max number of ticks reached");
        }
    }
}

#[test]
#[should_panic]
fn loop_goes_past_max_ticks() {
    // Arrange
    let mut guard = LoopGuard::new(1000);

    // Act
    loop {
        // Infinite loop - should obviously panic
        guard.protect();
    }
}
