mod config;
mod input;

fn main() {
    config::loadconfig::load_config();

    // Blink twice to show we started successfullly
    input::buzz::blink(2, None);

    // Test button-reactions
    input::buzz::open_device();
}
