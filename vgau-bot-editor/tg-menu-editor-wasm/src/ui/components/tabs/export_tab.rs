use eframe::egui;
use crate::MenuEditorApp;

pub struct ExportTab;

impl ExportTab {
    pub fn draw(app: &mut MenuEditorApp, ui: &mut egui::Ui) {
        // Style ui for Tailwind-like appearance
        ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);
        
        ui.vertical(|ui| {
            ui.add(egui::Label::new(egui::RichText::new("Экспорт / Импорт данных")
                .size(24.0)
                .color(egui::Color32::from_rgb(209, 213, 219)))); // gray-300
            
            ui.add_space(8.0);
            
            // Workflow metadata editor with tailwind-like styling
            egui::CollapsingHeader::new("Метаданные проекта")
                .default_open(true)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                    
                    let field_label_style = |text: &str| -> egui::RichText {
                        egui::RichText::new(text).color(egui::Color32::from_rgb(209, 213, 219))
                    };
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Автор:")));
                        if ui.text_edit_singleline(&mut app.workflow_metadata.author).changed() {
                            app.dirty = true;
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Описание:")));
                        if ui.text_edit_singleline(&mut app.workflow_metadata.description).changed() {
                            app.dirty = true;
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Теги:")));
                        let mut tags = app.workflow_metadata.tags.join(", ");
                        if ui.text_edit_singleline(&mut tags).changed() {
                            app.workflow_metadata.tags = tags.split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            app.dirty = true;
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Создан:")));
                        ui.label(&app.workflow_metadata.created_at);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Изменен:")));
                        ui.label(&app.workflow_metadata.modified_at);
                    });
                });
            
            ui.add_space(10.0);
            ui.label("Экспортированные данные:");
            
            // Add a scrollable text area for the exported JSON
            ui.add(
                egui::TextEdit::multiline(&mut app.exported_data)
                    .desired_width(f32::INFINITY)
                    .desired_rows(15)
                    .font(egui::TextStyle::Monospace.resolve(ui.style()))
            );
            
            ui.add_space(10.0);
            
            // Styled button
            let button = egui::Button::new(
                egui::RichText::new("Обновить экспорт")
                    .color(egui::Color32::WHITE)
            )
            .fill(egui::Color32::from_rgb(59, 130, 246)) // blue-500
            .rounding(egui::Rounding::same(6.0));
            
            if ui.add(button).clicked() {
                app.export_data();
            }
            
            ui.separator();
            ui.heading("Импорт данных");
            
            // Add a scrollable text area for the imported JSON
            ui.add(
                egui::TextEdit::multiline(&mut app.import_text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(15)
                    .hint_text("Вставьте JSON данные для импорта...")
                    .font(egui::TextStyle::Monospace.resolve(ui.style()))
            );
            
            ui.horizontal(|ui| {
                if ui.button("Импортировать").clicked() {
                    app.import_data();
                }
                
                if ui.button("Очистить").clicked() {
                    app.import_text.clear();
                }
            });
        });
    }
} 