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