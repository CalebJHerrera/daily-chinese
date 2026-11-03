#[derive(serde::Serialize, serde::Deserialize, Debug)] // allow the JSONification of the entries.
pub struct Entry {
    pub traditional: String,
    pub simplified: String,
    pub pinyin: String,
    pub definitions: Vec<String>,
}