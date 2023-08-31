#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub working_dir: String,

    #[serde(skip)]
    pub config_path: String,

    pub ui: UI,
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
