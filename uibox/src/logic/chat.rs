use super::data::DownloadImgConfig;
use crate::slint_generatedAppWindow::{AppWindow, ChatItem, Logic, Store};
use crate::util::http as uhttp;
use crate::util::translator::tr;
use crate::{config, CResult};
use async_openai::{
    config::OpenAIConfig, types::CreateImageRequestArgs, types::ImageSize, types::ResponseFormat,
    Client,
};
use log::warn;
use native_dialog::FileDialog;
use slint::{ComponentHandle, Model, ModelRc, VecModel, Weak};
use std::fs;
use std::path::Path;
use tokio::task;
use uuid::Uuid;

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
                for item in ui.global::<Store>().get_chat_datas().iter() {
                    if item.uuid != uuid {
                        continue;
                    }

                    match item
                        .imgs_path
                        .as_any()
                        .downcast_ref::<VecModel<slint::SharedString>>()
                        .expect("We know we set a VecModel earlier")
                        .row_data(index as usize)
                    {
                        Some(path) => match fs::copy(path.as_str(), file) {
                            Err(e) => {
                                ui.global::<Logic>().invoke_show_message(
                                    slint::format!("{}{:?}", tr("保存失败"), e),
                                    "warning".into(),
                                );
                            }
                            _ => {
                                ui.global::<Logic>()
                                    .invoke_show_message(tr("保存成功").into(), "success".into());
                            }
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

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_send_question_text(move |question| {
        if question.is_empty() {
            return;
        }

        let ui = ui_handle.unwrap();
        let uuid = Uuid::new_v4().to_string();

        ui.global::<Store>()
            .get_chat_datas()
            .as_any()
            .downcast_ref::<VecModel<ChatItem>>()
            .expect("We know we set a VecModel earlier")
            .push(ChatItem {
                uuid: uuid.as_str().into(),
                utext: question.as_str().into(),
                ..Default::default()
            });

        ui.global::<Logic>()
            .invoke_show_message(tr("正在下载...").into(), "info".into());

        let ui_handle = ui.as_weak();
        let dii = DownloadImgConfig {
            uuid,
            question: question.to_string(),
        };
        task::spawn(async move {
            match download_images(ui_handle, dii).await {
                Err(e) => {
                    warn!("{:?}", e);
                }
                _ => {}
            }
        });
    });
}

// Warning: This function will run in other thread
async fn download_images(ui: Weak<AppWindow>, dii: DownloadImgConfig) -> CResult {
    let uuid = dii.uuid;
    let chat_config = config::chat();
    let image_count = chat_config.image_count.parse::<u8>().unwrap_or(1);
    let cache_image_dir = config::cache_image_dir();

    let image_size = match chat_config.image_size.as_str() {
        "256x256" => ImageSize::S256x256,
        "512x512" => ImageSize::S512x512,
        "1024x1024" => ImageSize::S1024x1024,
        _ => ImageSize::S512x512,
    };

    let config = OpenAIConfig::new()
        .with_api_key(chat_config.api_key)
        .with_api_base(chat_config.api_base);

    let client =
        Client::with_config(config).with_http_client(uhttp::client(uhttp::ClientType::OpenAI)?);

    let request = CreateImageRequestArgs::default()
        .prompt(dii.question)
        .n(image_count)
        .size(image_size)
        .response_format(ResponseFormat::Url)
        .user("async-openai")
        .build()?;

    let response = client.images().create(request).await?;
    let paths = response
        .save(cache_image_dir)
        .await?
        .iter()
        .map(|path| format!("{}", path.display()))
        .collect::<Vec<_>>();

    let _ = slint::invoke_from_event_loop(move || {
        let ui = ui.unwrap();

        for (index, mut item) in ui.global::<Store>().get_chat_datas().iter().enumerate() {
            if item.uuid.as_str() == uuid.as_str() {
                let imgs = paths
                    .iter()
                    .map(|path| {
                        slint::Image::load_from_path(Path::new(path))
                            .unwrap_or(slint::Image::default())
                    })
                    .collect::<Vec<_>>();
                item.imgs = ModelRc::new(VecModel::from(imgs));

                let paths = paths
                    .iter()
                    .map(|path| slint::format!("{}", path))
                    .collect::<Vec<_>>();
                item.imgs_path = ModelRc::new(VecModel::from(paths));

                ui.global::<Store>()
                    .get_chat_datas()
                    .as_any()
                    .downcast_ref::<VecModel<ChatItem>>()
                    .expect("We know we set a VecModel earlier")
                    .set_row_data(index, item);

                ui.global::<Logic>()
                    .invoke_show_message(tr("下载成功").into(), "success".into());

                return;
            }
        }
    });

    Ok(())
}
