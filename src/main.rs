mod cn_dictionary;
use iced::widget::{button, text, column, Column, row};
use crate::cn_dictionary::{query_dictionary, ChineseWord};
use jieba_rs::Jieba;

#[derive(Debug, Clone)]
enum Message {
    Randomize,
    Previous,
    Next
}

#[derive(Default)]
struct App {
    words: Vec<ChineseWord>,
    jieba: Jieba,
    page: i8
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Randomize => {
                self.words = query_dictionary(&self.jieba);
                self.page = 0;
            },
            Message::Previous => {
                if self.page < self.words.len() as i8 {
                    self.page -= 1;
                }
            },
            Message::Next => {
                if self.page >= 0 {
                    self.page += 1;
                }
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            text("Hi"),
            button("Randomize").on_press(Message::Randomize),
            row![
                button("Prev").on_press(Message::Previous),
                button("Next").on_press(Message::Next)
            ]
        ]
    }

    fn new() -> Self {
        let jieba = Jieba::new();
        let words = query_dictionary(&jieba);

        Self {
            words,
            jieba,
            page: 0
        }
    }
}

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .font(include_bytes!("../assets/noto_sans_sc/NotoSansSC-Regular.ttf"))
        .title("Test")
        .run()
}
