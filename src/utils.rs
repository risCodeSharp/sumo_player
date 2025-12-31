pub mod texture_util{
    use eframe::egui::{Context, ColorImage, TextureHandle};
    use image;

    pub fn load_cover_texture(ctx: &Context, image: image::DynamicImage) -> TextureHandle {
        let rgba = image.to_rgb8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let pixels = rgba.into_raw();
        
        let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
        // TODO: Make it cache to in HashMap
        ctx.load_texture("cover", color_image, Default::default())
    }
}

pub mod color_util {
    use eframe::egui::Color32;
    
       /// Helper function to blend colors for smooth animations
    pub fn lerp_color(start: Color32, end: Color32, t: f32) -> Color32 {
        Color32::from_rgb(
            (start.r() as f32 + (end.r() as f32 - start.r() as f32) * t) as u8,
            (start.g() as f32 + (end.g() as f32 - start.g() as f32) * t) as u8,
            (start.b() as f32 + (end.b() as f32 - start.b() as f32) * t) as u8,
        )
    }
}

