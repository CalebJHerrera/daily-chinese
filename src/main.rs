use random_word::Lang;

fn main(){
    let raw_word = random_word::get(Lang::Zh);
    let word = raw_word
        .split_whitespace()
        .nth(1)
        .unwrap();

    let result = chinese_dictionary::query_by_simplified(word);
    for entry in result {
        let simplified = entry.simplified();
        println!("Simplified: {}", simplified);

        let pinyin = entry.pinyin().marks();
        println!("Pinyin: {}", pinyin);

        print!("Meanings: ");
        for eng in entry.english() {
            let meaning = eng.gloss().value();
            print!("| {} |", meaning);
        }
    }
}
