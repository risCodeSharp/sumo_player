use eframe::egui::{self, Color32, Painter, vec2, Pos2, Rect, Sense, Shape, Stroke, debug_text::print, response};



// Constants for consistent styling
const BUTTON_COLOR: Color32 = Color32::from_rgb(35, 39, 48);
const BUTTON_WIDGET_COLOR: Color32 = Color32::from_rgb(0, 122, 255);
const HOVER_BORDER_COLOR: Color32 = Color32::from_rgb(0, 122, 255);

/// Helper function to blend colors for smooth animations
fn lerp_color(start: Color32, end: Color32, t: f32) -> Color32 {
    Color32::from_rgb(
        (start.r() as f32 + (end.r() as f32 - start.r() as f32) * t) as u8,
        (start.g() as f32 + (end.g() as f32 - start.g() as f32) * t) as u8,
        (start.b() as f32 + (end.b() as f32 - start.b() as f32) * t) as u8,
    )
}

pub fn play_button(ui: &mut egui::Ui) -> egui::Response {
    let desired_size = vec2(60.0, 25.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let how_hovered = ui.ctx().animate_bool_with_time(response.id, response.hovered(), 0.1);

        // --- Background ---
        let bg_color = if response.is_pointer_button_down_on() {
            Color32::from_rgb(45, 50, 60)
        } else {
            let hover_bg = Color32::from_rgb(50, 55, 65);
            lerp_color(BUTTON_COLOR, hover_bg, how_hovered)
        };
        painter.rect_filled(rect, 4.0, bg_color);

        if how_hovered > 0.0 {
            let stroke_color = Color32::from_rgba_unmultiplied(
                HOVER_BORDER_COLOR.r(),
                HOVER_BORDER_COLOR.g(),
                HOVER_BORDER_COLOR.b(),
                (how_hovered * 100.0) as u8,
            );
            painter.rect_stroke(rect, 4.0, (1.0, stroke_color), egui::StrokeKind::Inside);
        }

        // --- Play Icon (Triangle) ---
        let icon_size = 10.0;
        let cx = painter.round_to_pixel_center(rect.center().x) + 1.0; // +1 for visual balance
        let cy = painter.round_to_pixel_center(rect.center().y);
        let half_side = icon_size / 2.0;

        let points = vec![
            Pos2::new(cx - half_side, cy - half_side),
            Pos2::new(cx - half_side, cy + half_side),
            Pos2::new(cx + half_side, cy),
        ];

        let icon_color = if response.hovered() { Color32::from_rgb(60, 160, 255) } else { BUTTON_WIDGET_COLOR };
        painter.add(Shape::convex_polygon(points, icon_color, Stroke::NONE));
    }

    if response.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    response
}

pub fn pause_button(ui: &mut egui::Ui) -> egui::Response {
    let desired_size = vec2(60.0, 25.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let how_hovered = ui.ctx().animate_bool_with_time(response.id, response.hovered(), 0.1);

        // --- Background ---
        let bg_color = if response.is_pointer_button_down_on() {
            Color32::from_rgb(45, 50, 60)
        } else {
            let hover_bg = Color32::from_rgb(50, 55, 65);
            lerp_color(BUTTON_COLOR, hover_bg, how_hovered)
        };
        painter.rect_filled(rect, 4.0, bg_color);

        if how_hovered > 0.0 {
            let stroke_color = Color32::from_rgba_unmultiplied(
                HOVER_BORDER_COLOR.r(),
                HOVER_BORDER_COLOR.g(),
                HOVER_BORDER_COLOR.b(),
                (how_hovered * 100.0) as u8,
            );
            painter.rect_stroke(rect, 4.0, (1.0, stroke_color), egui::StrokeKind::Inside);
        }

        // --- Pause Icon (Two Bars) ---
        let icon_height = 10.0;
        let bar_width = 3.0;
        let gap = 3.0;
        
        let cx = painter.round_to_pixel_center(rect.center().x);
        let cy = painter.round_to_pixel_center(rect.center().y);

        let top_y = cy - (icon_height / 2.0);
        let left_bar_x = cx - (gap / 2.0) - bar_width;
        let right_bar_x = cx + (gap / 2.0);

        let icon_color = if response.hovered() { Color32::from_rgb(60, 160, 255) } else { BUTTON_WIDGET_COLOR };

        painter.rect_filled(Rect::from_min_size(Pos2::new(left_bar_x, top_y), vec2(bar_width, icon_height)), 1.0, icon_color);
        painter.rect_filled(Rect::from_min_size(Pos2::new(right_bar_x, top_y), vec2(bar_width, icon_height)), 1.0, icon_color);
    }

    if response.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    response
}

pub fn stop_button(ui: &mut egui::Ui) -> egui::Response {
    let desired_size = vec2(60.0, 25.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let how_hovered = ui.ctx().animate_bool_with_time(response.id, response.hovered(), 0.1);

        // --- Background ---
        let bg_color = if response.is_pointer_button_down_on() {
            Color32::from_rgb(45, 50, 60)
        } else {
            let hover_bg = Color32::from_rgb(50, 55, 65);
            lerp_color(BUTTON_COLOR, hover_bg, how_hovered)
        };
        painter.rect_filled(rect, 4.0, bg_color);

        if how_hovered > 0.0 {
            let stroke_color = Color32::from_rgba_unmultiplied(
                HOVER_BORDER_COLOR.r(),
                HOVER_BORDER_COLOR.g(),
                HOVER_BORDER_COLOR.b(),
                (how_hovered * 100.0) as u8,
            );
            painter.rect_stroke(rect, 4.0, (1.0, stroke_color), egui::StrokeKind::Inside);
        }

        // --- Stop Icon (Square) ---
        let icon_size = 10.0;
        let cx = painter.round_to_pixel_center(rect.center().x);
        let cy = painter.round_to_pixel_center(rect.center().y);
        let icon_rect = Rect::from_center_size(Pos2::new(cx, cy), vec2(icon_size, icon_size));

        let icon_color = if response.hovered() { Color32::from_rgb(60, 160, 255) } else { BUTTON_WIDGET_COLOR };
        painter.rect_filled(icon_rect, 1.0, icon_color);
    }

    if response.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
    response
}