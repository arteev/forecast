use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigYandex {
    pub api_key: String,
    // широта
    pub lat: String,
    // долгота
    pub lon: String,
    // язык ответа
    pub lang: Option<String>,
    // user-agent
    pub user_agent: String,
}