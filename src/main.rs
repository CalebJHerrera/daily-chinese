use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use daily_chinese::Entry;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let root = env!("CARGO_MANIFEST_DIR");
    let dict_path = Path::new(root).join("assets/trim.json");
    let dict_file = File::open(dict_path);

    let dict: HashMap<char, Vec<Entry>> = serde_json::from_reader(dict_file.unwrap()).unwrap();
    println!("{:#?}", dict);

    let app = MainWindow::new()?;
    let app_weak = app.as_weak();


    app.on_hide_function(move || {
        let weak = app_weak.unwrap();
        weak.window().hide().unwrap();
    });

    app.show()?;
    slint::run_event_loop_until_quit()?;

    Ok(())
}
