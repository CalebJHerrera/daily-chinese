use random_word::Lang;

#[derive(Debug, Clone)]
struct MeasureWord {
    pub simplified: String,
    pub traditional: String,
}

#[derive(Debug, Clone)]
struct ChineseWord {
    pub simplified: String,
    pub traditional: String,
    pub pinyin: String,
    pub meanings: Vec<String>,
    pub measure_words: Vec<MeasureWord>,
    pub is_variant: bool,
}

fn get_word() -> String {
    let raw_word = random_word::get(Lang::Zh);
    let word = raw_word.split_whitespace().nth(1).unwrap();

    word.to_string()
}

fn query_dictionary() -> Vec<ChineseWord> {
    let result = chinese_dictionary::query_by_simplified(&get_word());
    let mut word = Vec::new();

    for entry in &result {
        let mut daily_word = ChineseWord {
            simplified: String::from(""),
            traditional: String::from(""),
            pinyin: String::from(""),
            meanings: Vec::new(),
            measure_words: Vec::new(),
            is_variant: false,
        };

        let simplified = entry.simplified();
        let traditional = entry.traditional();
        let pinyin = entry.pinyin().marks();

        let mut meanings = Vec::new();
        let mut measure_words = Vec::new();

        let mut defined_variant = false;

        for thing in entry.english() {
            let meaning = thing.gloss().value();
            if meaning.contains("variant of ") {
                defined_variant = true;
            }

            meanings.push(String::from(meaning));
        }

        for word in entry.measure_words() {
            let mw_simplified = word.value().simplified();
            let mw_traditional = word.value().traditional();

            let mw_struct = MeasureWord {
                simplified: String::from(mw_simplified),
                traditional: String::from(mw_traditional),
            };

            measure_words.push(mw_struct);
        }

        daily_word.simplified = String::from(simplified);
        daily_word.traditional = String::from(traditional);
        daily_word.pinyin = String::from(pinyin);
        daily_word.meanings = meanings;
        daily_word.measure_words = measure_words;
        daily_word.is_variant = defined_variant;

        word.push(daily_word);
    }

    word
}

fn main() {
    for _i in 0..=0 {
        println!("{:?}", query_dictionary());
    }
}
