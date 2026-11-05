use std::time::Duration;
use slint::{SharedString, Timer, TimerMode};
use chinese_popup::Vocab;

slint::include_modules!();

fn next_card<'a>(current: &mut i32, deck: &'a Vec<Vocab>) -> &'a Vocab {
    if *current as usize >= (deck.len() - 1) {
        *current = 0;
    } else {
        *current += 1;
    }

    deck.get(*current as usize).unwrap()
}

fn main() -> Result<(), slint::PlatformError> {
    let deck: Vec<Vocab> = serde_json::from_str(
        include_str!("../assets/hsk/inclusive-new-4.json")
    ).unwrap();
    let mut index = rand::random_range(0..=deck.len()) as i32;

    let app = MainWindow::new()?;
    let app_weak = app.as_weak();

    app.on_hide_function(move || {
        if let Some(weak) = app_weak.upgrade() {
            weak
                .window()
                .hide()
                .expect("Error hiding window");
        }
    });

    let timer = Timer::default();
    timer.start(
        TimerMode::Repeated,
        Duration::from_secs(10),
        move || {
            let t = next_card(&mut index, &deck);

            app.set_pinyin(SharedString::from(&t.forms.first().unwrap().transcriptions.pinyin));
            app.set_simplified(SharedString::from(&t.simplified));
            app.set_traditional(SharedString::from(
                &t.forms
                    .first()
                    .unwrap()
                    .traditional
            ));

            app.show().unwrap()
        }
    );

    slint::run_event_loop_until_quit()?;

    Ok({
        println!("Hi");
    })
}
