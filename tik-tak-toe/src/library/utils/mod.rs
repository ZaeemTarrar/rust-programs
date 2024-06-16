pub mod utils {
    use std::thread::sleep;
    use std::time::Duration;
    use std::io::{ Write, stdout };

    /// ## Wait/Sleep
    /// Lets you pause the program for the provided
    /// number of seconds
    /// ```
    /// wait(t: 3);
    /// ```
    pub fn wait(t: u64) {
        sleep(Duration::from_secs(t));
    }

    /// ## Console Clear
    /// Clears out the console entirely.
    /// ```
    /// console_clear();
    /// ```
    pub fn console_clear() {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().expect("Flush Error");
    }
}
