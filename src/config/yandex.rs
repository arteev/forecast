use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigYandex {
    pub api_key: String,
    // широта
    pub lat: f64,
    // долгота
    pub lon: f64,
    // язык ответа
    pub lang: Option<String>,
}