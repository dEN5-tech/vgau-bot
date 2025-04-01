use eframe::egui;
use crate::graph::SimpleNodeGraph;

pub struct StatusBar {
    workflow_id: String,
    dirty: bool,
    status_message: Option<(String, f32)>,
}

impl StatusBar {
    pub fn new(workflow_id: String, dirty: bool, status_message: Option<(String, f32)>) -> Self {
        Self {
            workflow_id,
            dirty,
            status_message,
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context, node_graph: &SimpleNodeGraph, ui_ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar")
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(31, 41, 55)) // bg-gray-800
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Status message with Tailwind-like styling
                    if let Some((message, time_left)) = &mut self.status_message {
                        ui.add(egui::Label::new(egui::RichText::new(message.as_str())
                            .color(egui::Color32::from_rgb(156, 163, 175)))); // gray-400
                        
                        // Update timer
                        *time_left -= ui_ctx.input(|i| i.predicted_dt);
                        if *time_left <= 0.0 {
                            self.status_message = None;
                        }
                    } else {
                        let status_text = if self.dirty {
                            "Несохраненные изменения"
                        } else {
                            "Готов к работе"
                        };
                        ui.add(egui::Label::new(egui::RichText::new(status_text)
                            .color(egui::Color32::from_rgb(156, 163, 175)))); // gray-400
                    }
                    
                    // Show undo/redo status in the middle
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                        let can_undo = node_graph.can_undo();
                        let can_redo = node_graph.can_redo();
                        
                        let undo_text = if can_undo { "⟲ Отменить (Ctrl+Z)" } else { "⟲ Отменить" };
                        let redo_text = if can_redo { "⟳ Вернуть (Ctrl+Y)" } else { "⟳ Вернуть" };
                        
                        let undo_color = if can_undo { 
                            egui::Color32::from_rgb(156, 220, 254) // Light blue
                        } else {
                            egui::Color32::from_rgb(100, 100, 100) // Disabled gray
                        };
                        
                        let redo_color = if can_redo {
                            egui::Color32::from_rgb(156, 220, 254) // Light blue
                        } else {
                            egui::Color32::from_rgb(100, 100, 100) // Disabled gray
                        };
                        
                        ui.add(egui::Label::new(egui::RichText::new(undo_text).color(undo_color)));
                        ui.add_space(10.0);
                        ui.add(egui::Label::new(egui::RichText::new(redo_text).color(redo_color)));
                    });
                    
                    // Right-aligned content
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(egui::Label::new(egui::RichText::new("ВГАУ Бот Редактор v1.0")
                            .color(egui::Color32::from_rgb(156, 163, 175)))); // gray-400
                        
                        // Display workflow ID with shortened format
                        let id_short = if self.workflow_id.len() > 8 {
                            format!("ID: {}...", &self.workflow_id[0..8])
                        } else {
                            format!("ID: {}", self.workflow_id)
                        };
                        ui.add(egui::Label::new(egui::RichText::new(id_short)
                            .color(egui::Color32::from_rgb(156, 163, 175)))); // gray-400
                    });
                });
            });
    }
    
    pub fn update_status(&mut self, message: &str, duration: f32) {
        self.status_message = Some((message.to_string(), duration));
    }
    
    pub fn update_status_if_none(&mut self, message: &str, duration: f32) {
        if self.status_message.is_none() {
            self.status_message = Some((message.to_string(), duration));
        }
    }
    
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
    
    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
    
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
} 