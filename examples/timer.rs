use enigo::EnigoSettings;
use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    env_logger::init();
    thread::sleep(Duration::from_secs(2));
    let mut enigo = Enigo::new(&EnigoSettings::default()).unwrap();

    let now = Instant::now();

    // write text
    enigo.key_sequence("Hello World! ❤️");

    let time = now.elapsed();
    println!("{time:?}");

    // select all
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Unicode('a'));
    enigo.key_up(Key::Control);
}
