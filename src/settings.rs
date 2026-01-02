use cirrus_config::config_key_path;
use cirrus_egui::v1::widgets::settings::{Settings, section::{Section, SectionDisplayInfo, SectionOverrides}};
use cirrus_theming::v1::theme::Theme;
use egui::Ui;

use crate::{TEMPLATE_CONFIG_TOML_STRING, config::config::Config};

pub struct SettingsMenu {}

impl SettingsMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&self, ui: &mut Ui, theme: &Theme, config: &mut Config) {
        let mut settings = Settings::new(TEMPLATE_CONFIG_TOML_STRING, &ui);

        settings.add_section(
            Section::new(
                config_key_path!(config.game.achievements.enable),
                &mut config.game.achievements.enable,
                SectionOverrides::default(),
                SectionDisplayInfo {
                    name: Some("Enable/disable achievements".into()),
                    ..Default::default()
                }
            )
        );

        settings.show_ui(ui, &theme);
    }
}
