pub struct Timer {
    name: String,
    start: std::time::Instant,
}

impl Drop for Timer {
    fn drop(&mut self) {
        let end = std::time::Instant::now();
        println!("Name: {} Duration: {:?}", self.name, end.duration_since(self.start));
    }
}
impl Timer {
    pub fn new(name: String) -> Timer {
        Timer {
            name,
            start: std::time::Instant::now(),
        }
    }
}