use crate::slint_generatedAppWindow::{AppWindow, ColorMakerConfig, Logic};
// use log::debug;
use slint::{Color, ComponentHandle};

const COLOR_BAND: [(f32, f32, f32); 6] = [
    (255.0, 0.0, 0.0),
    (255.0, 255.0, 0.0),
    (0.0, 255.0, 0.0),
    (0.0, 255.0, 255.0),
    (0.0, 0.0, 255.0),
    (255.0, 0.0, 255.0),
];

pub fn init(ui: &AppWindow) {
    ui.global::<Logic>().on_hex_value(move |red, green, blue| {
        slint::format!("#{:02X}{:02X}{:02X}", red, green, blue)
    });

    ui.global::<Logic>()
        .on_current_color_band_color(move |height, current_y| {
            if height < current_y || height < 0.0 || current_y < 0.0 {
                return Color::from_rgb_u8(255, 255, 255);
            }

            let band_count = COLOR_BAND.len() - 1;
            let band_len = height / band_count as f32;
            let current_band_index = (current_y / band_len) as usize;
            let current_band_offset = current_y % band_len;

            if current_band_index == band_count {
                return Color::from_rgb_u8(255, 0, 255);
            } else if current_band_index > band_count {
                return Color::from_rgb_u8(255, 255, 255);
            }

            let offset_per = current_band_offset / band_len;
            let (start_red, start_green, start_blue) = COLOR_BAND[current_band_index];
            let (end_red, end_green, end_blue) = COLOR_BAND[current_band_index + 1];
            let (cur_red, cur_green, cur_blue) = (
                (start_red + (end_red - start_red) * offset_per) as u8,
                (start_green + (end_green - start_green) * offset_per) as u8,
                (start_blue + (end_blue - start_blue) * offset_per) as u8,
            );

            Color::from_rgb_u8(cur_red, cur_green, cur_blue)
        });

    ui.global::<Logic>()
        .on_current_display_color(move |height, current_y, band_color| {
            if height < current_y || height < 0.0 || current_y < 0.0 {
                return ColorMakerConfig::default();
            }

            let offset_per = current_y / height;

            if offset_per < 0.5 {
                let offset_per = offset_per * 2.0;
                ColorMakerConfig {
                    red: (255.0 - (255.0 - band_color.red() as f32) * offset_per) as i32,
                    green: (255.0 - (255.0 - band_color.green() as f32) * offset_per) as i32,
                    blue: (255.0 - (255.0 - band_color.blue() as f32) * offset_per) as i32,
                }
            } else {
                let offset_per = 1.0 - (offset_per - 0.5) * 2.0;
                ColorMakerConfig {
                    red: (band_color.red() as f32 * offset_per) as i32,
                    green: (band_color.green() as f32 * offset_per) as i32,
                    blue: (band_color.blue() as f32 * offset_per) as i32,
                }
            }
        });
}
