use chinese_dictionary::query;

fn main() {
    let results = query("今天").unwrap();
    println!("{:?}", results[0].simplified());
}
