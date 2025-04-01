use eframe::egui;

pub fn apply_tailwind_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Rounded corners like in Tailwind
    style.visuals.window_rounding = egui::Rounding::same(8.0);
    style.visuals.window_shadow.extrusion = 8.0;
    style.visuals.menu_rounding = egui::Rounding::same(6.0);
    
    // Colors similar to Tailwind blue shades
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(59, 130, 246); // blue-500
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(96, 165, 250); // blue-400
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 58, 138); // blue-900
    
    // Apply the style
    ctx.set_style(style);
}

pub fn create_button(text: &str, color: egui::Color32) -> egui::Button {
    egui::Button::new(
        egui::RichText::new(text)
            .color(egui::Color32::WHITE)
    )
    .fill(color)
    .rounding(egui::Rounding::same(6.0))
}

pub fn create_label(text: &str, color: egui::Color32) -> egui::Label {
    egui::Label::new(
        egui::RichText::new(text)
            .color(color)
    )
}

pub fn create_heading(text: &str) -> egui::Label {
    egui::Label::new(
        egui::RichText::new(text)
            .size(24.0)
            .color(egui::Color32::from_rgb(209, 213, 219)) // gray-300
    )
}

pub fn create_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(31, 41, 55)) // bg-gray-800
        .rounding(egui::Rounding::same(8.0))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(55, 65, 81))) // gray-700 border
        .inner_margin(egui::style::Margin::same(12.0))
}

pub fn create_collapsing_header(text: &str) -> egui::CollapsingHeader {
    egui::CollapsingHeader::new(text)
        .default_open(true)
        .frame(create_frame())
}

pub fn create_text_edit() -> egui::TextEdit {
    egui::TextEdit::multiline(&mut String::new())
        .desired_width(f32::INFINITY)
        .desired_rows(15)
        .font(egui::TextStyle::Monospace)
}

pub fn create_separator() -> egui::Separator {
    egui::Separator::default()
        .spacing(8.0)
}

pub fn create_space() -> egui::Space {
    egui::Space::new(egui::Vec2::new(8.0, 8.0))
} 