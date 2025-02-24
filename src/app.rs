use egui::Sense;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // for hot reload set the following in chrome dev tools:
        // application->service workers->update on reload: true
        // application->storage->unregister service worker: false
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.heading("Electricity").highlight();

                // draw a circle outline
                // todo: extract this into a function so that we can enable ui interaction to spawn circles
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(16.0), Sense::hover());
                let rect = response.rect;
                let c = rect.center();
                let r = rect.width() / 2.0 - 1.0;
                painter.circle_stroke(c, r, egui::Stroke::new(1.0, egui::Color32::WHITE));

                ui.horizontal(|ui| {
                    ui.label("we fixed it: ");
                    ui.text_edit_singleline(&mut self.label);
                });

                ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                if ui.button("Increment").clicked() {
                    self.value += 1.0;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });
    }
}
