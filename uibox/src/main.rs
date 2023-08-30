slint::include_modules!();

#[macro_use]
extern crate serde_derive;

use chrono::Local;
use env_logger::fmt::Color as LColor;
use log::debug;
use std::env;
use std::io::Write;

mod logic;
mod util;

use logic::{colors, clipboard, message, util as lutil};

pub type CResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> CResult {
    init_logger();
    debug!("{}", "start...");

    let ui = AppWindow::new().unwrap();
    clipboard::init(&ui);
    message::init(&ui);
    lutil::init(&ui);
    colors::init(&ui);
    ui.run().unwrap();

    debug!("{}", "exit...");
    Ok(())
}

fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            let mut level_style = buf.style();
            match record.level() {
                log::Level::Warn | log::Level::Error => {
                    level_style.set_color(LColor::Red).set_bold(true)
                }
                _ => level_style.set_color(LColor::Blue).set_bold(true),
            };

            writeln!(
                buf,
                "[{} {} {} {}] {}",
                ts,
                level_style.value(record.level()),
                record
                    .file()
                    .unwrap_or("None")
                    .split('/')
                    .last()
                    .unwrap_or("None"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
