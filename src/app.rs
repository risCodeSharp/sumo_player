use crate::components;
use crate::services::MusicService;
use crate::ui::MusicPathEntryUI;
use eframe::egui::{self, TextureHandle};
pub struct MusicPlayer {
    music_service: MusicService,
    music_path_entry_ui: MusicPathEntryUI,
    cover_texture: Option<TextureHandle>,
}

impl MusicPlayer {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            music_path_entry_ui: MusicPathEntryUI::new(),
            music_service: MusicService::new(),
            cover_texture: None,
        }
    }
}

impl eframe::App for MusicPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.music_path_entry_ui.show(ui);
            self.music_path_entry_ui.on_submit(
                ctx,
                &mut self.music_service,
                &mut self.cover_texture,
            );

            if self.music_service.is_music_loaded() {
                ui.label(format!(
                    "song name: {}",
                    self.music_service.music_file.name()
                ));
                ui.horizontal(|ui| {
                    if components::play_button(ui).clicked() {
                        self.music_service.resume();
                    }

                    if components::pause_button(ui).clicked() {
                        self.music_service.pause();
                    }
                    
                    if components::stop_button(ui).clicked() {
                        self.music_service.stop();
                    }
                });
            }
        });
    }
}
