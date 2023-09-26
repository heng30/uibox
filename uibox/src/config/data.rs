#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub working_dir: String,

    #[serde(skip)]
    pub config_path: String,

    #[serde(skip)]
    pub cache_dir: String,

    #[serde(skip)]
    pub cache_image_dir: String,

    pub ui: UI,
    pub chat: Chat,
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
pub struct Chat {
    pub api_key: String,
    pub api_base: String,
    pub image_size: String,
    pub image_count: String,
    pub request_timeout: u64,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            api_base: "https://api.openai.com/v1".to_string(),
            image_size: "512x512".to_string(),
            image_count: "1".to_string(),
            request_timeout: 60,
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
