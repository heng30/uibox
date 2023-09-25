use crate::slint_generatedAppWindow::{AppWindow, FontItem, Logic, Store};
use font_kit::source::SystemSource;
use log::warn;
use slint::{ComponentHandle, Model, SharedString, SortModel, VecModel};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use tokio::task;

pub fn init(ui: &AppWindow) {
    show_all_fonts(ui);

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_search_font(move |search_text| {
        let ui = ui_handle.unwrap();

        if search_text.is_empty() {
            ui.global::<Store>()
                .set_fonts(ui.global::<Store>().get_fonts_cache());
            return;
        }

        let items = VecModel::default();
        for item in ui.global::<Store>().get_fonts_cache().iter() {
            if item.postscript_name.contains(search_text.as_str()) {
                items.push(item);
            }
        }

        ui.global::<Store>().set_fonts(Rc::new(items).into());
    });
}

fn show_all_fonts(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    task::spawn(async move {
        match SystemSource::new().all_fonts() {
            Ok(fonts) => {
                let mut map: HashMap<_, (HashSet<_>, _)> = HashMap::new();
                for font in fonts {
                    if let Ok(font) = font.load() {
                        let properties = font.properties();

                        if font.postscript_name().is_none() {
                            continue;
                        }

                        let pname = font
                            .postscript_name()
                            .unwrap()
                            .split('-')
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap()
                            .split('_')
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap()
                            .to_string();

                        match map.get_mut(&pname) {
                            Some(item) => {
                                item.0.insert(properties.style.to_string());
                            }
                            _ => {
                                let mut set = HashSet::new();
                                set.insert(properties.style.to_string());
                                map.insert(
                                    pname.clone(),
                                    (
                                        set,
                                        (
                                            pname,
                                            font.full_name(),
                                            font.family_name(),
                                            properties.weight.0,
                                            properties.stretch.0,
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                }

                let _ = slint::invoke_from_event_loop(move || {
                    let items = VecModel::default();
                    for item in map.into_values() {
                        let styles: VecModel<SharedString> = VecModel::default();
                        for style in item.0.iter() {
                            styles.push(style.into());
                        }
                        let styles =
                            SortModel::new(styles, |a, b| a.to_lowercase().cmp(&b.to_lowercase()));

                        items.push(FontItem {
                            postscript_name: item.1 .0.into(),
                            name: item.1 .1.into(),
                            family: item.1 .2.into(),
                            weight: item.1 .3,
                            stretch: item.1 .4,
                            styles: Rc::new(styles).into(),
                        });
                    }

                    let items = SortModel::new(items, |a, b| {
                        a.postscript_name
                            .to_lowercase()
                            .cmp(&b.postscript_name.to_lowercase())
                    });

                    let ui = ui_handle.unwrap();
                    ui.global::<Store>().set_fonts_cache(Rc::new(items).into());
                    ui.global::<Store>()
                        .set_fonts(ui.global::<Store>().get_fonts_cache());
                });
            }
            Err(e) => {
                warn!("{:?}", e);
            }
        }
    });
}
