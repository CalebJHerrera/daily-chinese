use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Vocab {
    pub simplified: String,
    pub forms: Vec<Form>,
}

#[derive(Debug, Deserialize)]
pub struct Form {
    pub traditional: String,
    pub transcriptions: Transcription,
    pub meanings: Vec<String>,
    pub classifiers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Transcription {
    pub pinyin: String,
}
