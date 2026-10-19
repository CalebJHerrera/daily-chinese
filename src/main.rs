mod cn_dictionary;

use crate::cn_dictionary::{ChineseWord, query_dictionary};
use iced::widget::{button, column, container, row, text, Column, Row};
use iced::{Alignment, Point, Program};
use iced::border::width;
use jieba_rs::Jieba;

#[derive(Debug, Clone)]
enum Message {
    Randomize,
    Previous,
    Next,
}

#[derive(Default)]
struct App {
    words: Vec<ChineseWord>,
    jieba: Jieba,
    page: i8,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Randomize => {
                self.words = query_dictionary(&self.jieba);
                self.page = 0;
            }
            Message::Previous => {
                if self.page > 0 {
                    self.page -= 1;
                }
            }
            Message::Next => {
                if self.page < (self.words.len() - 1) as i8 {
                    self.page += 1;
                }
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        // Non entry dependents
        let buttons = row![
            button("<").on_press(Message::Previous),
            button("Random").on_press(Message::Randomize),
            button(">").on_press(Message::Next),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        if let Some(entry) = self.words.get(self.page as usize) {
            // Pinyin-character alignment element
            let pinyin_split: Vec<&str> = entry.pinyin.split_whitespace().collect();
            let traditional_split: Vec<char> = entry.traditional.chars().collect();
            let simplified_split: Vec<char> = entry.simplified.chars().collect();

            let mut final_row = Row::new().spacing(5);

            for i in 0..pinyin_split.len() {
                let p = pinyin_split.get(i).copied().unwrap_or_default();
                let s = simplified_split.get(i).copied().unwrap_or_default();
                let t = traditional_split.get(i).copied().unwrap_or_default();

                let char_col = format!("{s}{t}");

                let word_col = column![
                    text(p).size(16),
                    text(char_col).size(36),
                ]
                    .align_x(Alignment::Center)
                    .width(iced::Length::Fixed(55f32));

                final_row = final_row.push(word_col);
            }

            // List of 5 top meanings
            let total_meanings = entry.meanings.len();
            let mut meanings_col = column![].align_x(Alignment::Start);
            meanings_col = meanings_col.push("Meanings:");

            // Push empty lines for no vert movement
            for i in 0..5 {
                if let Some(meaning) = entry.meanings.get(i) {
                    let t = text(format!("- {}", meaning)).size(16);
                    meanings_col = meanings_col.push(t);
                } else {
                    let t = text(" ").size(16);
                    meanings_col = meanings_col.push(t);
                }
            }

            if total_meanings > 5 {
                let more_text = text(format!("...{} meanings omitted", total_meanings - 5))
                    .size(14)
                    .color([0.7, 0.7, 0.7]);

                meanings_col = meanings_col.push(more_text);
            } else {
                let t = text(" ").size(14);
                meanings_col = meanings_col.push(t);
            }

            let meanings_container = container(meanings_col)
                .width(iced::Length::Fixed(450.0));

            // Entry dependents
            let dict_page = column![final_row.padding(2), meanings_container,]
                .align_x(Alignment::Center)
                .width(iced::Length::Fixed(500f32))
                .height(iced::Length::Fixed(250f32));

            column![
                dict_page,
                text(format!(
                    "page {} of {}",
                    (self.page + 1),
                    (self.words.len())
                ))
                .size(16),
                buttons
            ]
            .align_x(Alignment::Center)
        } else {
            column![text("Something went wrong").size(40), buttons]
        }
    }

    fn new() -> Self {
        let jieba = Jieba::new();
        let words = query_dictionary(&jieba);

        Self {
            words,
            jieba,
            page: 0,
        }
    }
}

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .font(include_bytes!(
            "../assets/noto_sans_sc/NotoSansSC-Regular.ttf"
        ))
        .title("Test")
        .resizable(false)
        .window_size(iced::Size::new(500f32, 300f32))
        .position(iced::window::Position::SpecificWith(
            |window_size, monitor_size| Point {
                x: 0.0,
                y: monitor_size.height - window_size.height,
            },
        ))
        .theme(iced::Theme::Nord)
        .run()
}
