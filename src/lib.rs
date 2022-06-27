/// Utility for preventing infinite loops
///
/// Use as a guard rail inside a loop block
pub struct LoopGuard {
    max_ticks: i32,
    count: i32,
    message: String,
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
    ///     guard.protect() // This will panic after 10 loops
    /// }
    /// ```
    pub fn new(max: i32) -> LoopGuard {
        LoopGuard {
            max_ticks: max,
            count: 0,
            message: String::from("Max number of ticks reached"),
        }
    }

    /// Set a custom panic message for a LoopGuard instance
    pub fn set_panic_message(mut self, message: &str) -> Self {
        self.message = String::from(message);
        self
    }

    /// From within a `while` or `loop` block, panic if the loop iteration
    /// surpasses the LoopGuard's `max_ticks` value.
    pub fn protect(&mut self) {
        self.count += 1;
        if self.count > self.max_ticks {
            panic!("{}", self.message);
        }
    }
}

#[test]
#[should_panic(expected = "Max number of ticks reached")]
fn infinite_loop_with_guard() {
    // Arrange
    let mut guard = LoopGuard::new(1000);

    // Act
    loop {
        // Infinite loop - should obviously panic
        guard.protect();
    }
}

#[test]
#[should_panic(expected = "Infinite Loop 2: Electric Boogaloo")]
fn infinite_loop_with_guard_with_custom_message() {
    // Arrange
    let mut guard = LoopGuard::new(10).set_panic_message("Infinite Loop 2: Electric Boogaloo");

    // Act
    loop {
        guard.protect();
    }
}

#[test]
#[should_panic(expected = "Max number of ticks reached")]
fn for_loop_surpasses_max_ticks() {
    // Arrange
    let mut guard = LoopGuard::new(10);

    // Act
    for _i in 0..11 {
        guard.protect();
    }
}

#[test]
fn for_loop_does_not_surpass_max_ticks() {
    // Arrange
    let mut guard = LoopGuard::new(10);

    // Act - Does not panic, since loop is within 10 ticks
    for _i in 0..10 {
        guard.protect();
    }
}
