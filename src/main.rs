#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::Duration;

use log::LevelFilter;
use eframe::NativeOptions;
use cirrus_egui::v1::{config_manager::ConfigManager, styling::Styling, notifier::Notifier};
use cirrus_theming::v1::{colour::Colour, theme::Theme};

use clap::Parser;

use egui_notify::ToastLevel;
use env_logger::Builder;

use config::config::Config;
use app::ExampleApp;

mod app;
mod achievements;
mod about_window;
mod config;
mod settings;

static APP_NAME: &str = "egui-desktop-app-example";
static TEMPLATE_CONFIG_TOML_STRING: &str = include_str!("../assets/config.template.toml");

/// Example egui desktop app at cloudy-org.
#[derive(Parser, Debug)]
#[clap(author = "Author Name")]
#[command(version, about, long_about = None)]
struct Args {}

fn main() -> eframe::Result {
    Builder::from_default_env()
        .filter_level(LevelFilter::Warn)
        .filter_module("zbus", LevelFilter::Off)
        .filter_module("sctk", LevelFilter::Off)
        .filter_module("winit", LevelFilter::Off)
        .filter_module("tracing", LevelFilter::Off)
        .parse_default_env()
        .init();

    let notifier = Notifier::new();

    let config_manager: ConfigManager<Config> = match ConfigManager::new(APP_NAME, TEMPLATE_CONFIG_TOML_STRING) {
        Ok(config_manager) => config_manager,
        Err(error) => {
            notifier.toast(
                format!(
                    "Error occurred initializing {}'s config file! \
                    Falling back to default config! Error: {}", APP_NAME, error.human_message()
                ),
                ToastLevel::Error,
                |toast| {
                    toast.duration(Some(Duration::from_secs(10)));
                }
            );

            ConfigManager::default()
        }
    };

    let theme = Theme::new(Some(Colour::from_hex(0xe05f78)));

    eframe::run_native(
        "Nijika Clicker",
        NativeOptions::default(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Styling::new(&theme)
                .set_all()
                .apply(&cc.egui_ctx);

            Ok(Box::new(ExampleApp::new(theme, notifier, config_manager)))
        }),
    )
}
