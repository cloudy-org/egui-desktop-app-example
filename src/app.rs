use std::time::Duration;

use cirrus_theming::v1::theme::Theme;
use eframe::{egui::{self, Context}};

use cirrus_egui::v1::{config_manager::ConfigManager, notifier::Notifier, widgets::settings::Settings, ui_utils::center_multi::ui_multiple_centered_double_render};
use egui::{CursorIcon, Key, RichText, Vec2, Sense};
use egui_notify::ToastLevel;

use crate::{about_window::AboutWindow, achievements, settings::SettingsMenu, config::config::Config};

pub static NIJIKA: egui::ImageSource = egui::include_image!("../assets/nijika.png");

pub struct ExampleApp {
    theme: Theme,
    notifier: Notifier,

    about_window: AboutWindow<'static>,
    settings_menu: SettingsMenu,

    config_manager: ConfigManager<Config>,

    show_about: bool,
    show_settings: bool,

    clicked: i128
}

impl ExampleApp {
    pub fn new(theme: Theme, notifier: Notifier, config_manager: ConfigManager<Config>) -> Self {
        let about_window = AboutWindow::new();
        let settings_menu = SettingsMenu::new();

        Self {
            theme,
            about_window,
            notifier,

            config_manager,
            settings_menu,

            show_about: false,
            show_settings: false,

            clicked: 0
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        Settings::handle_input(
            &ctx,
            &mut self.config_manager,
            &mut self.notifier,
            &mut self.show_settings
        );

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::A)) {
            self.show_about = !self.show_about;
        }

        self.notifier.update(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_about {
                self.about_window.show(ui);
            }

            if self.show_settings {
                self.config_manager.update(ctx, &mut self.notifier);

                self.settings_menu.show(
                    ui,
                    &self.theme,
                    &mut self.config_manager.config
                );

                return;
            }

            ui.centered_and_justified(|ui| {
                let clicker = ui.add(
                    egui::Image::new(NIJIKA.clone())
                        .max_height(250.0)
                        .sense(Sense::click())
                ).on_hover_cursor(CursorIcon::PointingHand);

                if clicker.clicked() {
                    self.clicked = self.clicked + 1;
                    if let Some(str) = achievements::handle(self.clicked, self.config_manager.config.game.achievements.enable) {
                        self.notifier.toast(
                            str,
                            ToastLevel::Success,
                            |toast| {
                                toast.duration(Some(Duration::from_secs(10)));
                            }
                        );
                    }
                }
            });
        });

        egui::TopBottomPanel::bottom("click_amount").show(ctx, |ui| {
            ui.label(RichText::new(format!("Clicks: {}", self.clicked)));
        });
    }
}
