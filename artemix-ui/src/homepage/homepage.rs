use egui::Ui;

pub struct Homepage {
    url: String,

    // #[cfg_attr(feature = "serde", serde(skip))]
    // promise: Option<Promise<ehttp::Result<Resource>>>,
}

impl Default for Homepage {
    fn default() -> Self {
        Self {
            url: "https://raw.githubusercontent.com/emilk/egui/master/README.md".to_owned(),
        }
    }
}

impl Homepage {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Hello Homepage!");
    }
}

impl eframe::App for Homepage {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::bottom("http_bottom").show(ctx, |ui| {
        //     let layout = egui::Layout::top_down(egui::Align::Center).with_main_justify(true);
        //     ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
        //         ui.add(egui_demo_lib::egui_github_link_file!())
        //     })
        // });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Homepage")
        });
    }
}