pub struct Account {
    pub active: bool,
}

impl Account {
    pub fn new() -> Self {
        Self { active: true }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
