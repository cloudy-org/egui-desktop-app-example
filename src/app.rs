use cirrus_theming::v1::Theme;
use eframe::{egui::{self, Context}};
use egui::RichText;

use crate::windows::about::AboutWindow;

pub struct ExampleApp<'a> {
    theme: Theme,
    about_box: AboutWindow<'a>
}

impl<'a> ExampleApp<'_> {
    pub fn new(theme: Theme) -> Self {
        let about_box = AboutWindow::new();

        Self {
            theme,
            about_box
        }
    }
}

impl eframe::App for ExampleApp<'_> {

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.about_box.handle_input(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.about_box.update(ctx);

            ui.centered_and_justified(|ui| {
                ui.heading(RichText::new("The forecast is Cloudy!").size(50.0));
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show_separator_line(false).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(format!("Is the theme dark? Answer: {}!", self.theme.is_dark)).monospace());
                ui.label("Press 'A' on your keyboard to open the About window!");
            });
        });
    }
}