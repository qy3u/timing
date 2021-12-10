use std::time::Instant;

pub struct Timing {
    name: String,
    start: Instant,
}

impl Timing {
    pub fn start(name: &str) -> Self {
        println!("{} start.", name);
        Self {
            name: name.into(),
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) {
        println!("{} done.elapsed: {:?}", self.name, self.start.elapsed());
    }
}
