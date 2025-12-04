use std::ops::Deref;

use eframe::egui;

#[derive(Default)]
struct App {
    display_image: Option<egui::TextureHandle>,
}

impl App {
    fn new(creation_ctx: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        App {
            display_image: Some(App::create_display_image(&creation_ctx.egui_ctx)),
        }
    }

    fn create_display_image(ctx: &egui::Context) -> egui::TextureHandle {
        let width = 64;
        let height = 32;

        // let size = width * height;
        let mut pixels = Vec::new();

        for index in 0..(width * height) {
            let brightness = (index as f32 / (width as f32 * height as f32) * 255.0) as u8;
            pixels.push(brightness);
        }

        let image = egui::ColorImage::from_gray([width, height], &pixels);

        ctx.load_texture("display", image, egui::TextureOptions::LINEAR)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        egui::SidePanel::left("left")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Registers");
                ui.separator();
                ui.label("TODO!");
            });

        egui::SidePanel::right("right")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Instructions");
                ui.separator();
                ui.label("TODO!");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Display");
            ui.separator();

            // TODO: Create texture once before displaying
            let texture = egui::load::SizedTexture::new(
                self.display_image.as_ref().unwrap().id(),
                [32.0, 64.0],
            );
            ui.image(egui::ImageSource::Texture(texture));
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
