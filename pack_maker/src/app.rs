use eframe::{egui::{self, Align2, Color32, Pos2, Visuals, Widget}, glow };
use native_dialog::{FileDialog, MessageDialog, MessageType};

pub struct PackApp {
    headers: Vec<String>,
    pack_name: String,
    pack_id: u8,
    is_dark: bool,
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
            pack_name: String::from("Custom Song Pack"),
            pack_id: 4,
            is_dark: false,
        }
    }
}

impl PackApp {
    fn show_song_grid(&self, ui: &mut egui::Ui) {
        egui::Grid::new("song_grid_header")
            .min_col_width(100.0)
            .min_row_height(12.0)
            .show(ui, |ui| {
                // Header
                for h in &self.headers {
                    ui.label(h);
                }
                ui.end_row();
            });
        ui.separator();

        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                egui::Grid::new("song_grid")
                    .striped(true)
                    .min_col_width(100.0)
                    .min_row_height(12.0)
                    .show(ui, |ui| {
                        /*// Header
                        for h in &self.headers {
                            ui.label(h);
                        }
                        ui.end_row();*/

                        // Songs
                        for i in 0..100 {
                            ui.label("col 1");
                            ui.label("col 2");
                            ui.label("col 3");
                            //ui.label("col 4");

                            //let prog_text = egui::WidgetText::RichText(egui::RichText::new("ERROR").strong().color(egui::epaint::Color32::RED));

                            egui::widgets::ProgressBar::new((i % 10) as f32 / 10.0)
                                .show_percentage()
                                //.text(prog_text)
                                .ui(ui);

                            //ui.label("col 5");

                            ui.button("Remove");

                            /*ui.painter().text(
                                Pos2::new(ui.min_rect().left() + 100.0, ui.min_rect().top() + 100.0),
                                Align2::CENTER_CENTER,
                                "test",
                                egui::TextStyle::Heading,
                                ui.visuals().text_color());*/

                            ui.end_row();
                        }
                    });
                });
    }
}

impl eframe::App for PackApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //ctx.request_repaint();

        // Set dark mode once
        if !self.is_dark {
            ctx.set_visuals(Visuals::dark());
            self.is_dark = true;
        }

        for f in ctx.input().raw.dropped_files.iter() {
            if let Some(path) = &f.path {
                println!("Dropped: {:?}", path);
            }
        }

        for f in ctx.input().raw.hovered_files.iter() {
            if let Some(path) = &f.path {
                println!("Hovered: {:?}", path);
            }
        }

        // Toolbar menu
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Exit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        // Main content
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.pack_name);

                    ui.label("Id:");
                    ui.add(egui::DragValue::new(&mut self.pack_id)
                        .clamp_range(4..=99i32)
                        .speed(1.0)
                        .fixed_decimals(0));

                        if ui.button("Add Songs").clicked() {
                            let path = FileDialog::new()
                                //.set_location("~/Desktop")
                                .show_open_single_dir()
                                .unwrap();
                        }

                        ui.button("Build");
                });

                ui.separator();

                self.show_song_grid(ui);
            });
    }

    fn on_exit(&mut self, _gl: Option<&glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        egui::Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }
}