pub mod utils {
    use std::thread::sleep;
    use std::time::Duration;
    use std::io::{ Write, stdout };

    pub fn wait(t: u64) {
        sleep(Duration::from_secs(t));
    }

    pub fn console_clear() {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().expect("Flush Error");
    }
}
