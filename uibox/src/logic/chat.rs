use crate::slint_generatedAppWindow::{AppWindow, ChatItem, Logic, Store};
use crate::util::translator::tr;
use native_dialog::FileDialog;
use slint::{ComponentHandle, Model, VecModel};
use std::fs;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_delete_chat_item(move |uuid| {
        let ui = ui_handle.unwrap();

        for (index, item) in ui.global::<Store>().get_chat_datas().iter().enumerate() {
            if item.uuid == uuid {
                ui.global::<Store>()
                    .get_chat_datas()
                    .as_any()
                    .downcast_ref::<VecModel<ChatItem>>()
                    .expect("We know we set a VecModel earlier")
                    .remove(index);

                ui.global::<Logic>()
                    .invoke_show_message(tr("删除成功").into(), "success".into());
                return;
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_save_chat_image(move |uuid, index| {
        let ui = ui_handle.unwrap();

        match FileDialog::new()
            .set_location("~")
            .set_filename(&format!("{}-{}.png", uuid.as_str(), index))
            .show_save_single_file()
        {
            Ok(Some(file)) => {
                log::debug!("{:?}", file);

                for item in ui.global::<Store>().get_chat_datas().iter() {
                    if item.uuid != uuid {
                        continue;
                    }

                    match item
                        .imgs_base64
                        .as_any()
                        .downcast_ref::<VecModel<slint::SharedString>>()
                        .expect("We know we set a VecModel earlier")
                        .row_data(index as usize)
                    {
                        Some(img_data) => match fs::write(file, img_data) {
                            Err(e) => {
                                ui.global::<Logic>().invoke_show_message(
                                    slint::format!("{}{:?}", tr("保存失败"), e),
                                    "warning".into(),
                                );
                            }
                            _ => {}
                        },
                        _ => {
                            ui.global::<Logic>()
                                .invoke_show_message(tr("保存失败").into(), "warning".into());
                        }
                    }
                    break;
                }
            }
            Err(e) => {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}{:?}", tr("保存失败"), e),
                    "warning".into(),
                );
            }
            _ => {}
        };
    });
}
