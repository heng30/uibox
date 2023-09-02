use super::colorsdata::ADG3_RGB_JSON;
use crate::slint_generatedAppWindow::{AppWindow, ColorItem, Store};
use log::warn;
use slint::{ComponentHandle, RgbaColor, VecModel};
use std::rc::Rc;

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

// impl Into<RgbaColor<f32>> for Color {
//     fn into(self) -> RgbaColor<f32> {
//         RgbaColor {
//             red: self.red,
//             green: self.green,
//             blue: self.blue,
//             alpha: self.alpha,
//         }
//     }
// }
//
//
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
    let items = VecModel::default();
    match serde_json::from_str::<ColorsConfig>(ADG3_RGB_JSON) {
        Ok(config) => {
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
            ui.global::<Store>().set_colors(Rc::new(items).into());
        }
        Err(e) => {
            warn!("{:?}", e);
        }
    };
}
