use eframe::{egui::{self, Color32}, epi};

pub struct PackApp {
    headers: Vec<String>,
}

impl Default for PackApp {
    fn default() -> Self {
        Self {
            headers: vec![
                String::from("id"),
                String::from("artist"),
                String::from("title"),
                String::from("progress"),
                String::from("options"),
            ],
        }
    }
}

impl PackApp {
    fn show_song_grid(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::auto_sized()
            .show(ui, |ui| {
                egui::Grid::new("song_grid")
                    .striped(true)
                    .min_col_width(50.0)
                    .min_row_height(12.0)
                    .show(ui, |ui| {
                        // Header
                        for h in &self.headers {
                            ui.label(h);
                        }
                        ui.end_row();

                        // Songs
                        for _ in 0..100 {
                            ui.label("col 1");
                            ui.label("col 2");
                            ui.label("col 3");

                            ui.end_row();
                        }
                    });
                });
    }
}

impl epi::App for PackApp {
    fn name(&self) -> &str {
        "Pack Maker for GP"
    }

    fn load(&mut self, _storage: &dyn epi::Storage) {}

    fn save(&mut self, _storage: &mut dyn epi::Storage) {}

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // Toolbar menu
        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Exit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        // Main content
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.show_song_grid(ui);
            });
    }

    fn setup(&mut self, _ctx: &egui::CtxRef) {}

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn on_exit(&mut self) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        egui::Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }
}