use eframe::egui;
use crate::MenuEditorApp;

pub struct EditorTab;

impl EditorTab {
    pub fn draw(app: &mut MenuEditorApp, ui: &mut egui::Ui) {
        // Style for the content area
        ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0); // Match Tailwind spacing
        
        // Add top toolbar with tailwind-like styling
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(55, 65, 81)) // bg-gray-700
            .rounding(egui::Rounding::same(6.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(egui::RichText::new("Масштаб:")
                        .color(egui::Color32::from_rgb(209, 213, 219)))); // gray-300
                    
                    // Button styling
                    let button_style = |ui: &mut egui::Ui| {
                        ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::from_rgb(75, 85, 99); // gray-600
                        ui.visuals_mut().widgets.active.bg_fill = egui::Color32::from_rgb(59, 130, 246); // blue-500
                        ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::from_rgb(96, 165, 250); // blue-400
                    };
                    
                    // Apply button style
                    button_style(ui);
                    
                    if ui.button("➖").clicked() {
                        // TODO: Zoom out 
                        app.show_status("Уменьшение масштаба пока не реализовано", 2.0);
                    }
                    
                    if ui.button("🔍").clicked() {
                        // Reset zoom
                        app.show_status("Сброс масштаба пока не реализован", 2.0);
                    }
                    
                    if ui.button("➕").clicked() {
                        // TODO: Zoom in
                        app.show_status("Увеличение масштаба пока не реализовано", 2.0);
                    }
                    
                    ui.separator();
                    
                    // More buttons with same styling
                    if ui.button("📋 Копировать").clicked() {
                        // TODO: Copy selected node
                        app.show_status("Копирование узла пока не реализовано", 2.0);
                    }
                    
                    if ui.button("📋 Вставить").clicked() {
                        // TODO: Paste node
                        app.show_status("Вставка узла пока не реализована", 2.0);
                    }
                    
                    if ui.button("🗑️ Удалить").clicked() {
                        // TODO: Delete selected node
                        app.show_status("Удаление узла пока не реализовано", 2.0);
                    }
                });
            });
        
        ui.add_space(8.0); // Tailwind-like spacing
        
        // Create a frame for the node graph area
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(31, 41, 55)) // bg-gray-800 
            .rounding(egui::Rounding::same(8.0))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(55, 65, 81))) // gray-700 border
            .inner_margin(egui::style::Margin::same(12.0))
            .show(ui, |ui| {
                // Draw the node graph
                app.node_graph.draw(ui);
            });
        
        // Mark as dirty if changes were made
        // This would need proper change tracking in the node graph
        // For now, this is a placeholder
        if ui.input(|i| i.pointer.primary_released()) {
            app.dirty = true;
        }
    }
} 