use random_word::Lang;
use jieba_rs::Jieba;

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
    pub part_of_speech: Vec<String>,
    pub is_variant: bool,
}

fn get_word() -> String {
    let raw_word = random_word::get(Lang::Zh);
    let word = raw_word.split_whitespace().nth(1).unwrap();

    word.to_string()
}

fn tag_word(jieba: &Jieba, word: &str, meanings: &Vec<String>) -> Vec<String> {
    // From meanings
    if meanings.iter().any(|v| v.contains("idiom") || v.contains("set phrase")) {
        return vec!["Idiom / Phrase".to_string()];
    } else if meanings.iter().any(|v| v.starts_with("to ") && !v.starts_with("to the")) {
        return vec!["Verb".to_string()];
    }

    // Structure checks
    let chars: Vec<char> = word.chars().collect();
    if chars.len() == 3 && (chars[1] == chars[2]) {
        return vec!["Adjective".to_string()];
    } else if chars.len() == 4 && (chars[0] == chars[2]) && (chars[1] == chars[3]) {
        return vec!["Idiom / Phrase".to_string()];
    }

    let raw_tags = jieba.tag(&word, true);
    let mut tags = Vec::new();
    for item in raw_tags {
        let raw_tag = item.tag.to_string();
        tags.push(map_jb_tags(&raw_tag));
    }
    if tags.is_empty() {
        vec!["x".to_string()]
    } else {
        tags.dedup();
        tags
    }
}

fn map_jb_tags(tag: &str) -> String {
    match tag {
        "i" | "l" | "nl" | "vl" | "al" => "Idiom / Phrase".to_string(),
        "n" | "nr" | "ns" | "nt" | "nz" | "ng" => "Noun".to_string(),
        "v" | "vd" | "vn" | "vg" => "Verb".to_string(),
        "a" | "ad" | "an" | "ag" => "Adjective".to_string(),
        "d" => "Adverb".to_string(),
        "r" => "Pronoun".to_string(),
        "m" => "Numeral".to_string(),
        "q" => "Measure Word".to_string(),
        "p" => "Preposition".to_string(),
        "c" => "Conjunction".to_string(),
        "u" => "Particle".to_string(),
        "o" => "Onomatopoeia".to_string(),
        "e" => "Interjection".to_string(),
        _ => "Other".to_string(),
    }
}

fn query_dictionary(jieba: &Jieba) -> Vec<ChineseWord> {
    let result = chinese_dictionary::query_by_simplified("哎呀");
    let mut word = Vec::new();

    for entry in &result {
        let mut daily_word = ChineseWord {
            simplified: String::from(""),
            traditional: String::from(""),
            pinyin: String::from(""),
            meanings: Vec::new(),
            measure_words: Vec::new(),
            part_of_speech: Vec::new(),
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

        daily_word.part_of_speech = tag_word(&jieba, &daily_word.simplified, &daily_word.meanings);
        word.push(daily_word);
    }

    word
}

fn main() {
    let jieba: Jieba = Jieba::new();

    for _i in 0..=0 {
        println!("{:?}", query_dictionary(&jieba));
    }
}
