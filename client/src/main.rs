//! Client binary: selective sync, revocation enforcement

fn main() {
    println!("Client starting...");
    core::init();
    crypto::init();
    governance::init();
}
