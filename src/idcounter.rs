#[derive(Clone)]
pub struct IdCounter {
    id: i32,
}

impl IdCounter {
    pub fn new() -> IdCounter {
        IdCounter {
            id: 0
        }
    }

    pub fn nextId(&mut self) -> i32 {
        self.id += 1;
        return self.id;
    }
}
