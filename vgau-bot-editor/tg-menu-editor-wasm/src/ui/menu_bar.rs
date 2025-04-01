use eframe::egui;
use crate::graph::SimpleNodeGraph;

pub struct MenuBar {
    workflow_id: String,
    dirty: bool,
}

impl MenuBar {
    pub fn new(workflow_id: String, dirty: bool) -> Self {
        Self {
            workflow_id,
            dirty,
        }
    }

    pub fn draw(
        &self, 
        ctx: &egui::Context, 
        node_graph: &mut SimpleNodeGraph,
        status_callback: &mut dyn FnMut(&str, f32),
        tab_callback: &mut dyn FnMut(super::app::Tab),
        export_callback: &mut dyn FnMut(),
        import_from_predefined_callback: &mut dyn FnMut() -> Result<(), String>,
        workflow_metadata: &super::app::WorkflowMetadata,
    ) {
        egui::TopBottomPanel::top("menu_bar")
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(31, 41, 55)) // bg-gray-800
                .shadow(egui::epaint::Shadow::small_light())
            )
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 10.0; // Match Tailwind's spacing
                    ui.visuals_mut().button_frame = false;  // No button frames in menu
                    
                    // File menu
                    ui.menu_button("Файл", |ui| {
                        ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(75, 85, 99); // gray-600
                        
                        if ui.button("Новый").clicked() {
                            // TODO: Add confirmation dialog
                            status_callback("Создан новый проект", 3.0);
                            ui.close_menu();
                        }
                        
                        if ui.button("Экспорт").clicked() {
                            export_callback();
                            tab_callback(super::app::Tab::Export);
                            ui.close_menu();
                        }
                        
                        if ui.button("Импорт").clicked() {
                            tab_callback(super::app::Tab::Export);
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Сохранить как PNG").clicked() {
                            status_callback("Сохранение как PNG пока не реализовано", 3.0);
                            ui.close_menu();
                        }
                        
                        // Add import telegrambot data button
                        ui.separator();
                        if ui.button("Импорт данных Telegram-бота ВГАУ").clicked() {
                            match import_from_predefined_callback() {
                                Ok(_) => {
                                    status_callback("Данные Telegram-бота успешно импортированы", 3.0);
                                }
                                Err(e) => {
                                    status_callback(&format!("Ошибка импорта: {}", e), 3.0);
                                }
                            }
                            ui.close_menu();
                        }
                    });
                    
                    // Edit menu
                    ui.menu_button("Правка", |ui| {
                        let can_undo = node_graph.can_undo();
                        let can_redo = node_graph.can_redo();
                        
                        let undo_text = if can_undo { "Отменить (Ctrl+Z)" } else { "Отменить" };
                        let redo_text = if can_redo { "Вернуть (Ctrl+Y)" } else { "Вернуть" };
                        
                        ui.add_enabled(can_undo, egui::Button::new(undo_text))
                            .clicked()
                            .then(|| {
                                if node_graph.undo() {
                                    status_callback("Отменено последнее действие", 2.0);
                                }
                                ui.close_menu();
                            });
                        
                        ui.add_enabled(can_redo, egui::Button::new(redo_text))
                            .clicked()
                            .then(|| {
                                if node_graph.redo() {
                                    status_callback("Действие возвращено", 2.0);
                                }
                                ui.close_menu();
                            });
                        
                        ui.separator();
                        
                        if ui.button("Копировать узел").clicked() {
                            // TODO: Implement copy
                            status_callback("Копирование узла пока не реализовано", 2.0);
                            ui.close_menu();
                        }
                        
                        if ui.button("Вставить узел").clicked() {
                            // TODO: Implement paste
                            status_callback("Вставка узла пока не реализована", 2.0);
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Удалить выбранный узел").clicked() {
                            if let Some(node_id) = node_graph.active_node {
                                node_graph.delete_node(node_id);
                            } else {
                                status_callback("Не выбран узел для удаления", 2.0);
                            }
                            ui.close_menu();
                        }
                    });
                    
                    // Export menu
                    ui.menu_button("Экспорт", |ui| {
                        if ui.button("Сформировать JSON").clicked() {
                            export_callback();
                            ui.close_menu();
                        }
                        
                        if ui.button("Скопировать в буфер обмена").clicked() {
                            status_callback("Копирование в буфер обмена пока не реализовано", 3.0);
                            ui.close_menu();
                        }
                    });
                    
                    // View menu
                    ui.menu_button("Вид", |ui| {
                        if ui.button("Редактор").clicked() {
                            tab_callback(super::app::Tab::Editor);
                            ui.close_menu();
                        }
                        
                        if ui.button("Экспорт/Импорт").clicked() {
                            tab_callback(super::app::Tab::Export);
                            ui.close_menu();
                        }
                        
                        if ui.button("Настройки").clicked() {
                            tab_callback(super::app::Tab::Settings);
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Сбросить масштаб").clicked() {
                            // TODO: Reset zoom level
                            status_callback("Масштаб сброшен", 2.0);
                            ui.close_menu();
                        }
                        
                        if ui.button("Центрировать граф").clicked() {
                            // TODO: Center graph
                            status_callback("Граф центрирован", 2.0);
                            ui.close_menu();
                        }
                    });
                    
                    // Nodes menu
                    ui.menu_button("Узлы", |ui| {
                        if ui.button("Добавить пункт меню").clicked() {
                            // Add new menu item at center position
                            node_graph.add_menu_item(
                                egui::pos2(300.0, 300.0),
                                "Новый пункт меню".to_string()
                            );
                            ui.close_menu();
                        }
                        
                        if ui.button("Добавить FAQ").clicked() {
                            // Add new FAQ item at center position
                            node_graph.add_faq_item(
                                egui::pos2(300.0, 300.0),
                                "Новый FAQ".to_string()
                            );
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Организовать узлы").clicked() {
                            // TODO: Auto-organize nodes
                            status_callback("Автоматическая организация узлов пока не реализована", 3.0);
                            ui.close_menu();
                        }
                    });
                    
                    // Help menu
                    ui.menu_button("Помощь", |ui| {
                        if ui.button("О редакторе").clicked() {
                            tab_callback(super::app::Tab::Help);
                            ui.close_menu();
                        }
                        
                        if ui.button("Документация").clicked() {
                            status_callback("Открытие документации пока не реализовано", 3.0);
                            ui.close_menu();
                        }
                    });
                    
                    // Display workflow name with Tailwind-like styling
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let workflow_name = format!("{} {}", 
                            workflow_metadata.description,
                            if self.dirty { "*" } else { "" }
                        );
                        ui.add(egui::Label::new(egui::RichText::new(workflow_name)
                            .text_style(egui::TextStyle::Heading)
                            .color(egui::Color32::from_rgb(229, 231, 235)))); // gray-200
                    });
                });
            });
    }
} 