/// Utility for preventing infinite loops
///
/// Use as a guard rail inside a `loop` or `while` block
///
/// Meant to be used as a development dependency
pub struct LoopGuard {
    pub max_ticks: i32,
    pub count: i32,
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
    pub fn set_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
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
#[should_panic(expected = "Max number of ticks reached")]
fn loop_surpasses_max_ticks_with_guard() {
    // Arrange
    let mut guard = LoopGuard::new(10);

    // Act
    for _i in 0..11 {
        guard.protect();
    }
}

#[test]
fn loop_within_max_count_of_guard() {
    // Arrange
    let mut guard = LoopGuard::new(10);

    // Act - does not panic
    for _i in 0..10 {
        guard.protect();
    }
}

#[test]
#[should_panic(expected = "Infinite Loop 2: Electric Boogaloo")]
fn infinite_loop_with_guard_and_custom_message() {
    // Arrange
    let mut guard = LoopGuard::new(10).set_message("Infinite Loop 2: Electric Boogaloo");

    // Act
    loop {
        guard.protect();
    }
}
