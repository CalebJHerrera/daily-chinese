use iced::Point;

pub enum WindowMessage {
    Show,
    Hide,
    Quit,
    DragStart,
    DragEnd(Point),
}

pub enum InteractMessage {
    
}