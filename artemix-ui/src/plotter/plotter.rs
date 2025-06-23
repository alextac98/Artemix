use egui::{Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use crate::plotter::sidebar::Sidebar;

type Tab = String;

// To define the contents and properties of individual tabs, we implement the `TabViewer`
// trait. Only three things are mandatory: the `Tab` associated type, and the `ui` and
// `title` methods. There are more methods in `TabViewer` which you can also override.
struct MyTabViewer;

impl TabViewer for MyTabViewer {
    // This associated type is used to attach some data to each tab.
    type Tab = Tab;

    // Returns the current `tab`'s title.
    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.as_str().into()
    }

    // Defines the contents of a given `tab`.
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }
}

pub struct Plotter {
    dock_state: DockState<Tab>,
    sidebar: Sidebar,
}

impl Plotter {
    pub fn new() -> Self {
        let tabs = ["tab1"]
            .map(str::to_string)
            .into_iter()
            .collect();
        return Self {
            dock_state: DockState::new(tabs),
            sidebar: Sidebar::new(),
        };
    }
    pub fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let side_panel_frame = egui::Frame::default().inner_margin(8);
        
        // Calculate max width as percentage of screen width
        let screen_width = ctx.screen_rect().width();
        
        egui::SidePanel::left("SidePanel")
            .frame(side_panel_frame)
            .resizable(true)
            .default_width(screen_width *  0.2)
            .min_width(screen_width * 0.1)
            .max_width(screen_width * 0.8)
            .show(ctx, |ui| {
                self.sidebar.ui(ui);
            });

        let central_frame = egui::Frame::default().inner_margin(0);
        egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
            let mut dock_style = Style::from_egui(ui.style());
            dock_style.tab_bar.fill_tab_bar = true;

            DockArea::new(&mut self.dock_state)
                .style(dock_style)
                .show_add_buttons(true)
                .show_leaf_collapse_buttons(false)
                .show_leaf_close_all_buttons(false)
                .show_inside(ui, &mut MyTabViewer);
        });
    }
}

impl eframe::App for Plotter {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui(ctx, frame);
    }
}
