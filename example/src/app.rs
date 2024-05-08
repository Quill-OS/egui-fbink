use eframe::{App, Frame};
use egui::{self, menu, CentralPanel, Context, ProgressBar, TopBottomPanel};
use egui_fbink::handle_component_update;
use log::debug;

pub struct TemplateApp {
    label: String,
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let Self { label, value } = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /*
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // cl
                    }
                })
            });
        });
        */

        /*
        epi::egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(
                epi::egui::Layout::bottom_up(epi::egui::Align::Center),
                |ui| {
                    ui.add(epi::egui::Hyperlink::new("https://github.com/emilk/egui/"));
                },
            );
        });
         */
        CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            if false {
                ui.heading("This is a heading");
                ui.hyperlink("https://this_is_a_link");
            }
            ui.checkbox(&mut true, "This is a checkbox");
            if ui.add(egui::Button::new("This is a button")).clicked() {}
            ui.add(egui::Slider::new(&mut 50.0, 0.0..=100.0).text("This is a slider"));
            egui::ComboBox::from_label("This is a combo box")
                .selected_text(format!("Combo box indeed"))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut "Nothing here", "Hey", "First");
                    ui.selectable_value(&mut "Woah", "Bye", "Second");
                    ui.selectable_value(&mut "No way", "who is there", "Third");
                });
            ui.radio_value(&mut "Nope", "First", "This is a radio button");
            ui.add(ProgressBar::new(0.5).text("This is a progress bar"));
       });

        /*
        if false {
            epi::egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
        */
    }
}
