#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::env;

use log::LevelFilter;
use eframe::NativeOptions;
use cirrus_egui::v1::styling::Styling;
use cirrus_theming::v1::Theme;
use clap::{arg, command, Parser};

use app::ExampleApp;

mod app;
mod windows;

/// Example Egui desktop app at cloudy-org.
#[derive(Parser, Debug)]
#[clap(author = "Author Name")]
#[command(version, about, long_about = None)]
struct Args {
    /// Valid themes at the moment: dark, light
    #[arg(short, long)]
    theme: Option<String>,
}

fn main() -> eframe::Result {
    match env::var("RUST_LOG").is_ok() {
        true => {
            env_logger::init();
        },
        false => {
            env_logger::builder()
                .filter_level(LevelFilter::Warn)
                .init();
        },
    }

    let cli_args = Args::parse();

    let theme_string = cli_args.theme;

    let is_dark = match theme_string {
        Some(string) => {
            match string.as_str() {
                "light" => false,
                "dark" => true,
                _ => {
                    log::warn!(
                        "'{}' is not a valid theme. Pass either 'dark' or 'light'.", string
                    );

                    true
                }
            }
        },
        _ => true
    };

    let theme = Theme::new(
        is_dark,
        vec![],
        None
    );

    eframe::run_native(
        "Example App",
        NativeOptions::default(),
        Box::new(|cc| {
            // Required to add support for images.
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Styling::new(&theme, None)
                .set_all()
                .apply(&cc.egui_ctx);

            Ok(Box::new(ExampleApp::new(theme)))
        }),
    )
}