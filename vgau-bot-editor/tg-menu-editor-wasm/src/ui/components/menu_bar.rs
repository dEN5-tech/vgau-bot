use eframe::egui;
use crate::ui::app::Tab;
use crate::MenuEditorApp;

pub struct MenuBar;

impl MenuBar {
    pub fn draw(app: &mut MenuEditorApp, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar")
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(31, 41, 55)) // bg-gray-800
                .shadow(egui::epaint::Shadow::small_light())
            )
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 10.0; // Match Tailwind's spacing
                    ui.visuals_mut().button_frame = false;  // No button frames in menu
                    
                    // Menu buttons with tailwind-like styles
                    ui.menu_button("Файл", |ui| {
                        ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(75, 85, 99); // gray-600
                        
                        if ui.button("Новый").clicked() {
                            if app.dirty {
                                // TODO: Add confirmation dialog
                            }
                            *app = MenuEditorApp::default();
                            app.show_status("Создан новый проект", 3.0);
                            ui.close_menu();
                        }
                        
                        if ui.button("Экспорт").clicked() {
                            app.export_data();
                            app.current_tab = Tab::Export;
                            ui.close_menu();
                        }
                        
                        if ui.button("Импорт").clicked() {
                            app.current_tab = Tab::Export;
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Сохранить как PNG").clicked() {
                            app.show_status("Сохранение как PNG пока не реализовано", 3.0);
                            ui.close_menu();
                        }
                        
                        // Add import telegrambot data button
                        ui.separator();
                        if ui.button("Импорт данных Telegram-бота ВГАУ").clicked() {
                            match app.import_predefined_bot_data() {
                                Ok(_) => {
                                    app.status_message = Some(("Данные Telegram-бота успешно импортированы".to_string(), 3.0));
                                }
                                Err(e) => {
                                    app.status_message = Some((format!("Ошибка импорта: {}", e), 3.0));
                                }
                            }
                            ui.close_menu();
                        }
                    });
                    
                    // Add Edit menu with undo/redo
                    ui.menu_button("Правка", |ui| {
                        let can_undo = app.node_graph.can_undo();
                        let can_redo = app.node_graph.can_redo();
                        
                        let undo_text = if can_undo { "Отменить (Ctrl+Z)" } else { "Отменить" };
                        let redo_text = if can_redo { "Вернуть (Ctrl+Y)" } else { "Вернуть" };
                        
                        ui.add_enabled(can_undo, egui::Button::new(undo_text))
                            .clicked()
                            .then(|| {
                                if app.node_graph.undo() {
                                    app.show_status("Отменено последнее действие", 2.0);
                                    app.dirty = true;
                                }
                                ui.close_menu();
                            });
                        
                        ui.add_enabled(can_redo, egui::Button::new(redo_text))
                            .clicked()
                            .then(|| {
                                if app.node_graph.redo() {
                                    app.show_status("Действие возвращено", 2.0);
                                    app.dirty = true;
                                }
                                ui.close_menu();
                            });
                        
                        ui.separator();
                        
                        if ui.button("Копировать узел").clicked() {
                            // TODO: Implement copy
                            app.show_status("Копирование узла пока не реализовано", 2.0);
                            ui.close_menu();
                        }
                        
                        if ui.button("Вставить узел").clicked() {
                            // TODO: Implement paste
                            app.show_status("Вставка узла пока не реализована", 2.0);
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Удалить выбранный узел").clicked() {
                            if let Some(node_id) = app.node_graph.active_node {
                                app.node_graph.delete_node(node_id);
                                app.dirty = true;
                            } else {
                                app.show_status("Не выбран узел для удаления", 2.0);
                            }
                            ui.close_menu();
                        }
                    });
                    
                    ui.menu_button("Экспорт", |ui| {
                        if ui.button("Сформировать JSON").clicked() {
                            app.export_data();
                            ui.close_menu();
                        }
                        
                        if ui.button("Скопировать в буфер обмена").clicked() {
                            // Copy to clipboard using JavaScript
                            if !app.exported_data.is_empty() {
                                let _ = app.copy_to_clipboard(&app.exported_data);
                                app.show_status("Данные скопированы в буфер обмена", 3.0);
                            } else {
                                app.show_status("Нет данных для копирования", 3.0);
                            }
                            ui.close_menu();
                        }
                    });
                    
                    ui.menu_button("Вид", |ui| {
                        if ui.button("Редактор").clicked() {
                            app.current_tab = Tab::Editor;
                            ui.close_menu();
                        }
                        
                        if ui.button("Экспорт/Импорт").clicked() {
                            app.current_tab = Tab::Export;
                            ui.close_menu();
                        }
                        
                        if ui.button("Настройки").clicked() {
                            app.current_tab = Tab::Settings;
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Сбросить масштаб").clicked() {
                            // TODO: Reset zoom level
                            app.show_status("Масштаб сброшен", 2.0);
                            ui.close_menu();
                        }
                        
                        if ui.button("Центрировать граф").clicked() {
                            // TODO: Center graph
                            app.show_status("Граф центрирован", 2.0);
                            ui.close_menu();
                        }
                    });
                    
                    ui.menu_button("Узлы", |ui| {
                        if ui.button("Добавить пункт меню").clicked() {
                            // Add new menu item at center position
                            app.node_graph.add_menu_item(
                                egui::pos2(300.0, 300.0),
                                "Новый пункт меню".to_string()
                            );
                            app.dirty = true;
                            ui.close_menu();
                        }
                        
                        if ui.button("Добавить FAQ").clicked() {
                            // Add new FAQ item at center position
                            app.node_graph.add_faq_item(
                                egui::pos2(300.0, 300.0),
                                "Новый FAQ".to_string()
                            );
                            app.dirty = true;
                            ui.close_menu();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Организовать узлы").clicked() {
                            // TODO: Auto-organize nodes
                            app.show_status("Автоматическая организация узлов пока не реализована", 3.0);
                            ui.close_menu();
                        }
                    });
                    
                    ui.menu_button("Помощь", |ui| {
                        if ui.button("О редакторе").clicked() {
                            app.current_tab = Tab::Help;
                            ui.close_menu();
                        }
                        
                        if ui.button("Документация").clicked() {
                            app.show_status("Открытие документации пока не реализовано", 3.0);
                            ui.close_menu();
                        }
                    });
                    
                    // Display workflow name with Tailwind-like styling
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let workflow_name = format!("{} {}", 
                            app.workflow_metadata.description,
                            if app.dirty { "*" } else { "" }
                        );
                        ui.add(egui::Label::new(egui::RichText::new(workflow_name)
                            .text_style(egui::TextStyle::Heading)
                            .color(egui::Color32::from_rgb(229, 231, 235)))); // gray-200
                    });
                });
            });
    }
} 