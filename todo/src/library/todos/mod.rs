pub mod todos {
    pub struct Note {
        title: String,
        description: String,
        read: bool,
    }
    impl Note {
        pub fn new(t: String, d: String, r: bool) -> Self {
            Self {
                title: t,
                description: d,
                read: r,
            }
        }
        pub fn get_title(&self) -> &str {
            &self.title
        }
        pub fn get_description(&self) -> &str {
            &self.description
        }
        pub fn get_read(&self) -> &bool {
            &self.read
        }
        pub fn set_read(&mut self, r: bool) {
            self.read = r;
        }
        pub fn info(&self) -> String {
            return format!(
                "title:\"{}\", description:\"{}\", read:{}",
                &self.get_title(),
                &self.get_description(),
                &self.get_read()
            );
        }
    }
}
