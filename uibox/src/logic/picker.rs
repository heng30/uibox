use crate::slint_generatedAppWindow::{AppWindow, ColorPickerConfig, Logic};
use crate::util::translator::tr;
use image::{ColorType, ImageBuffer, Rgba};
use native_dialog::FileDialog;
use slint::{ComponentHandle, Rgba8Pixel, SharedPixelBuffer};
use std::sync::Mutex;

lazy_static! {
    static ref IMAGE_BUFFER: Mutex<Option<ImageBuffer<Rgba<u8>, Vec<u8>>>> = Mutex::new(None);
}

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_load_image(move || {
        let ui = ui_handle.unwrap();

        let path = FileDialog::new()
            .set_location("~")
            .add_filter("Image Files", &["png", "PNG"])
            .show_open_single_file();

        let image_path = match path {
            Ok(file) => match file {
                Some(file) => file,
                _ => return slint::Image::default(),
            },
            Err(e) => {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}{:?}", tr("打开文件失败！"), e),
                    "warning".into(),
                );

                return slint::Image::default();
            }
        };

        match image::open(image_path) {
            Err(e) => {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}{:?}", tr("打开文件失败！"), e),
                    "warning".into(),
                );
                slint::Image::default()
            }
            Ok(img) => match img.color() {
                ColorType::Rgb8 | ColorType::Rgba8 => {
                    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = img.to_rgba8();
                    let img_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                        buffer.as_raw(),
                        buffer.width(),
                        buffer.height(),
                    );

                    {
                        *IMAGE_BUFFER.lock().unwrap() = Some(buffer);
                    }
                    slint::Image::from_rgba8(img_buffer)
                }
                _ => {
                    ui.global::<Logic>()
                        .invoke_show_message(tr("图片格式非法！").into(), "warning".into());
                    slint::Image::default()
                }
            },
        }
    });

    ui.global::<Logic>().on_image_point_color(move |x, y| {
        let buffer = IMAGE_BUFFER.lock().unwrap();
        if buffer.is_none() {
            return ColorPickerConfig::default();
        }

        let buffer = buffer.as_ref().unwrap();

        let (x, y) = (x as u32, y as u32);
        if x > buffer.width() || y > buffer.height() {
            return ColorPickerConfig::default();
        }

        let pixel_color: Rgba<u8> = *buffer.get_pixel(x, y);

        // println!(
        //     "Pixel color at ({}, {}): R={}, G={}, B={}, A={}",
        //     x, y, pixel_color[0], pixel_color[1], pixel_color[2], pixel_color[3]
        // );

        ColorPickerConfig {
            red: pixel_color[0] as i32,
            green: pixel_color[1] as i32,
            blue: pixel_color[2] as i32,
            alpha: pixel_color[3] as i32,
            hex: slint::format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                pixel_color[0],
                pixel_color[1],
                pixel_color[2],
                pixel_color[3],
            ),
        }
    });
}
