pub const MIN_COUNCIL_MEMBERS: usize = 10;

pub struct Council {
    pub members: usize,
}

impl Council {
    pub fn new(members: usize) -> Self {
        Self { members }
    }

    pub fn has_quorum(&self) -> bool {
        self.members >= MIN_COUNCIL_MEMBERS
    }
}
