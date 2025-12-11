pub struct Payload {
    pub data: Vec<u8>,
}

impl Payload {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
