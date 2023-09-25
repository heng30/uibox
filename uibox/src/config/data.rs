#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub working_dir: String,

    #[serde(skip)]
    pub config_path: String,

    pub ui: UI,

    pub socks5: Socks5,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UI {
    pub font_size: u32,
    pub font_family: String,
    pub language: String,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            font_size: 20,
            font_family: "SystemDefault".to_string(),
            language: "cn".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Socks5 {
    pub openai: bool,
    pub url: String,
    pub port: u16,
}

impl Default for Socks5 {
    fn default() -> Self {
        Self {
            openai: false,
            url: "127.0.0.1".to_string(),
            port: 1080,
        }
    }
}

