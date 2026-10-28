mod enums;

use iced::Point;
use crate::enums::{WindowMessage, InteractMessage};

enum Message {
    UI(WindowMessage),
    Interact(InteractMessage),
}

fn main() {
    println!("Totally necessary rewrite, bear with me real quick");
}
