use eframe::egui;

#[derive(Default)]
struct App {
    display_image: Option<egui::TextureHandle>,
}

impl App {
    const DISPLAY_WIDTH: u32 = 32;
    const DISPLAY_HEIGHT: u32 = 64;

    const DISPLAY_PIXELS: u32 = App::DISPLAY_WIDTH * App::DISPLAY_HEIGHT;

    const PRESENT_SCALE: u32 = 8;

    const PRESENT_WIDTH: u32 = App::DISPLAY_WIDTH * App::PRESENT_SCALE;
    const PRESENT_HEIGHT: u32 = App::DISPLAY_HEIGHT * App::PRESENT_SCALE;

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
        let mut pixels = Vec::new();

        for index in 0..App::DISPLAY_PIXELS {
            let brightness = (index as f32
                / (App::DISPLAY_WIDTH as f32 * App::DISPLAY_HEIGHT as f32)
                * 255.0) as u8;
            pixels.push(brightness);
        }

        let image = egui::ColorImage::from_gray(
            [App::DISPLAY_WIDTH as usize, App::DISPLAY_HEIGHT as usize],
            &pixels,
        );

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
                [App::PRESENT_WIDTH as f32, App::PRESENT_HEIGHT as f32],
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
