// Only Web is currently supported
#[cfg(not(target_arch = "wasm32"))]
compile_error!("This crate is intended to compile only for the web (wasm32 target).");

use crate::homepage::Homepage;
#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Tab {
    Home,
    Plotter,
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
        #[cfg_attr(not(target_arch = "wasm32"), allow(unused_mut))]
        let mut app = Self {
            ..Default::default()
        };
        // Deep link: set tab based on URL path
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = window() {
            if let Ok(path) = win.location().pathname() {
                match path.as_str() {
                    "/" => app.active_tab = Tab::Home,
                    "/plot" | "/plot/" => app.active_tab = Tab::Plotter,
                    _ => {}
                }
            }
        }
        app
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
            ui.horizontal(|ui| {
                // App title
                ui.label(&self.name);
                ui.separator();
                // Home tab
                let home_btn = ui.selectable_value(&mut self.active_tab, Tab::Home, "Home");
                if home_btn.clicked() {
                    #[cfg(target_arch = "wasm32")]
                    if let Some(win) = window() {
                        let _ = win.history().unwrap()
                            .push_state_with_url(&JsValue::NULL, "", Some("/")).unwrap();
                    }
                }
                // Plot tab
                let plot_btn = ui.selectable_value(&mut self.active_tab, Tab::Plotter, "Plotter");
                if plot_btn.clicked() {
                    #[cfg(target_arch = "wasm32")]
                    if let Some(win) = window() {
                        let _ = win.history().unwrap()
                            .push_state_with_url(&JsValue::NULL, "", Some("/plot")).unwrap();
                    }
                }
                
                // Theme switch on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                });
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
        }
    }
}
