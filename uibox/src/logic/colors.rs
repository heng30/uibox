use super::colorsdata::{ADG3_RGB_JSON, DSP_JSON};
use crate::slint_generatedAppWindow::{AppWindow, ColorItem, Store};
use log::warn;
use slint::{ComponentHandle, ModelRc, RgbaColor, VecModel};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct ColorsConfig {
    pub colors: Vec<Color>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct ColorDSP {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct ColorsDSPConfig {
    pub colors: Vec<ColorDSP>,
}

impl From<Color> for RgbaColor<f32> {
    fn from(val: Color) -> Self {
        RgbaColor {
            red: val.red,
            green: val.green,
            blue: val.blue,
            alpha: val.alpha,
        }
    }
}

pub fn init(ui: &AppWindow) {
    match serde_json::from_str::<ColorsConfig>(ADG3_RGB_JSON) {
        Ok(config) => {
            let items = VecModel::default();
            for color in config.colors.into_iter() {
                let name = format!(
                    "#{:02X}{:02X}{:02X}{:02X}",
                    (color.red * 255.0).round() as u8,
                    (color.green * 255.0).round() as u8,
                    (color.blue * 255.0).round() as u8,
                    (color.alpha * 255.0).round() as u8,
                );
                items.push(ColorItem {
                    name: name.as_str().into(),
                    hex: name.into(),
                    value: {
                        let color: RgbaColor<f32> = color.into();
                        color.into()
                    },
                });
            }
            ui.global::<Store>().set_colors_adg3(ModelRc::new(items));
        }
        Err(e) => {
            warn!("{:?}", e);
        }
    };

    match serde_json::from_str::<ColorsDSPConfig>(DSP_JSON) {
        Ok(config) => {
            let items = VecModel::default();
            for color in config.colors.into_iter() {
                items.push(ColorItem {
                    name: color.name.as_str().into(),
                    hex: color.value.to_uppercase().into(),
                    value: {
                        let (r, g, b, a) = hex_to_rgba(&color.value);
                        slint::Color::from_argb_u8(a, r, g, b)
                    },
                });
            }
            ui.global::<Store>().set_colors_dsp(ModelRc::new(items));
        }
        Err(e) => {
            warn!("{:?}", e);
        }
    };

    ui.global::<Store>()
        .set_colors(ui.global::<Store>().get_colors_adg3());
}

fn hex_to_rgba(hex: &str) -> (u8, u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0_u8);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0_u8);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0_u8);
    let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(0_u8);
    (r, g, b, a)
}
