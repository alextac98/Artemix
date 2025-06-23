// Only Web is currently supported
#[cfg(not(target_arch = "wasm32"))]
compile_error!("This crate is intended to compile only for the web (wasm32 target).");

use crate::homepage::Homepage;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Tab {
    Home,
    Plotter,
    About,
}

impl Default for Tab {
    fn default() -> Self { Self::Home }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ArtemixApp {
    name: String,

    #[serde(skip)]
    homepage: Homepage,

    #[serde(skip)]
    plotter_app: crate::plotter::Plotter,

    active_tab: Tab,
}

impl Default for ArtemixApp {
    fn default() -> Self {
        Self {
            name: "Artemix".to_string(),
            homepage: Homepage::default(),
            plotter_app: crate::plotter::Plotter::new(),
            active_tab: Tab::default(),
        }
    }
}

impl ArtemixApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        return Self {
            ..Default::default()
        };
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(&self.name);
            // ui.add(egui::widgets::Spacer::default().expand_width());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                ui.separator();
            });
        });
    }
}

impl eframe::App for ArtemixApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.top_bar(ui);
                ui.separator();
                ui.selectable_value(&mut self.active_tab, Tab::Home, "Home");
                ui.selectable_value(&mut self.active_tab, Tab::Plotter, "Plotter");
                ui.selectable_value(&mut self.active_tab, Tab::About, "About");
            });
        });

        match self.active_tab {
            Tab::Home => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.homepage.ui(ui);
                });
            }
            Tab::Plotter => {
                self.plotter_app.ui(ctx, _frame);
            }
            Tab::About => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("About");
                    ui.label("Artemix version 0.1.0");
                });
            }
        }
    }
}
