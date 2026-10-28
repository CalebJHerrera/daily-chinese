use random_zh::{RandomZhOptions, random_zh};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::path::Path;

#[derive(serde::Serialize)] // allow the JSONification of the entries.
struct Entry {
    traditional: String,
    simplified: String,
    pinyin: String,
    definitions: Vec<String>,
}

fn to_pinyin(sharky: &str) -> String {
    fn is_vowel(fishy: char) -> bool {
        match fishy {
            'a' | 'e' | 'i' | 'o' | 'u' | 'ü' => true,
            _ => false,
        }
    }

    fn refine(yellowtail: &str) -> String {
        let remora = yellowtail.replace('v', "ü").replace("u:", "ü");

        let mut sliced_remora = remora.chars().collect::<Vec<char>>();

        // Tone
        let mackerel = match sliced_remora.last().copied() {
            Some(c @ '1'..='5') => {
                sliced_remora.pop();
                c.to_digit(10).unwrap() as u8
            }
            _ => 0,
        };

        if mackerel == 0 || mackerel == 5 {
            return sliced_remora.into_iter().collect();
        }

        // Decide which fishy slice gets the tone.
        let salmon = sliced_remora
            .iter()
            .position(|&c| c == 'a')
            .or_else(|| sliced_remora.iter().position(|&c| c == 'e'))
            .or_else(|| sliced_remora.iter().position(|&c| c == 'o'))
            .or_else(|| sliced_remora.iter().rposition(|&c| is_vowel(c)));

        match salmon {
            Some(i) => {
                sliced_remora[i] = mark(sliced_remora[i], mackerel);
                sliced_remora.into_iter().collect()
            }
            None => sliced_remora.into_iter().collect(),
        }
    }

    // Absolute HELL
    fn mark(tuna: char, tone: u8) -> char {
        match (tuna, tone) {
            ('a', 1) => 'ā',
            ('a', 2) => 'á',
            ('a', 3) => 'ǎ',
            ('a', 4) => 'à',
            ('e', 1) => 'ē',
            ('e', 2) => 'é',
            ('e', 3) => 'ě',
            ('e', 4) => 'è',
            ('i', 1) => 'ī',
            ('i', 2) => 'í',
            ('i', 3) => 'ǐ',
            ('i', 4) => 'ì',
            ('o', 1) => 'ō',
            ('o', 2) => 'ó',
            ('o', 3) => 'ǒ',
            ('o', 4) => 'ò',
            ('u', 1) => 'ū',
            ('u', 2) => 'ú',
            ('u', 3) => 'ǔ',
            ('u', 4) => 'ù',
            ('ü', 1) => 'ǖ',
            ('ü', 2) => 'ǘ',
            ('ü', 3) => 'ǚ',
            ('ü', 4) => 'ǜ',
            _ => tuna,
        }
    }

    // Begin the real operation after a ton of setup
    sharky
        .split_whitespace()
        .map(refine)
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get file from root folder (daily-chinese) reliably
    let root_dir = env!("CARGO_MANIFEST_DIR");
    let cc_path = Path::new(root_dir).join("assets/cedict/cedict_ts.u8");

    let file = File::open(cc_path)?;

    // Get random zh level 1 vocab (ALL OF IT!!)
    let lv1_vocab = random_zh(RandomZhOptions {
        count: Some(88888),
        level_range: Some((1, 1)),
        stroke_count_range: Some((1, 36)),
        allow_duplicates: false,
    });

    let mut export_dict: HashMap<char, Vec<Entry>> = HashMap::new();
    let parsed = cedict::parse_reader(file);

    // Iterate over every dictionary entry, get the definitions for each,
    // filter them, add to the dict that will be exported to JSON.
    for entry in parsed {
        let simple = entry.simplified();

        if simple.chars().count() > 1 {
            continue; // random_zh only does single characters, no phrases allowed.
        }

        let singled_out = simple.chars().next().unwrap();

        if !lv1_vocab.contains(&singled_out) {
            continue; // not in vocab, remove the extra bloat.
        }

        let defs = entry
            .definitions()
            .filter(|e| !e.starts_with("variant of"))
            .map(String::from)
            .collect::<Vec<String>>();

        if defs.is_empty() {
            continue;
        }

        export_dict.entry(singled_out).or_default().push(Entry {
            traditional: entry.traditional().to_string(),
            simplified: simple.to_string(),
            pinyin: to_pinyin(entry.pinyin()),
            definitions: defs,
        });
    }

    // Create file and write to it directly.
    let out_location = Path::new(root_dir).join("assets/trim.json");
    let out = File::create(&out_location)?;

    // Sort by shadowing export_dict into a BTree - somehow sorts itself I guess
    let export_dict: BTreeMap<char, Vec<Entry>> = export_dict.into_iter().collect();
    serde_json::to_writer_pretty(out, &export_dict)?;

    let size = std::fs::metadata(&out_location)?.len() as f32 / (1024f32 * 1024f32);
    println!("Wrote {}MB to {}.", size, out_location.display());

    Ok(()) // Return OK instead of 0... How far we have come.
}
