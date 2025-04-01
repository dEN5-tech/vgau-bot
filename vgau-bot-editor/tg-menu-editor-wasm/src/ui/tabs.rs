use eframe::egui;
use crate::graph::SimpleNodeGraph;

pub enum Tab {
    Editor,
    Export,
    Settings,
    Help,
}

pub struct Tabs {
    current_tab: Tab,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Editor,
        }
    }

    pub fn draw(
        &mut self,
        ui: &mut egui::Ui,
        node_graph: &mut SimpleNodeGraph,
        status_callback: &mut dyn FnMut(&str, f32),
        export_callback: &mut dyn FnMut(),
        import_callback: &mut dyn FnMut(),
        workflow_metadata: &super::app::WorkflowMetadata,
    ) {
        match self.current_tab {
            Tab::Editor => self.draw_editor_tab(ui, node_graph, status_callback),
            Tab::Export => self.draw_export_tab(ui, export_callback, import_callback, workflow_metadata),
            Tab::Settings => self.draw_settings_tab(ui, status_callback),
            Tab::Help => self.draw_help_tab(ui),
        }
    }

    fn draw_editor_tab(
        &mut self,
        ui: &mut egui::Ui,
        node_graph: &mut SimpleNodeGraph,
        status_callback: &mut dyn FnMut(&str, f32),
    ) {
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
                        status_callback("Уменьшение масштаба пока не реализовано", 2.0);
                    }
                    
                    if ui.button("🔍").clicked() {
                        status_callback("Сброс масштаба пока не реализован", 2.0);
                    }
                    
                    if ui.button("➕").clicked() {
                        status_callback("Увеличение масштаба пока не реализовано", 2.0);
                    }
                    
                    ui.separator();
                    
                    // More buttons with same styling
                    if ui.button("📋 Копировать").clicked() {
                        status_callback("Копирование узла пока не реализовано", 2.0);
                    }
                    
                    if ui.button("📋 Вставить").clicked() {
                        status_callback("Вставка узла пока не реализована", 2.0);
                    }
                    
                    if ui.button("🗑️ Удалить").clicked() {
                        status_callback("Удаление узла пока не реализовано", 2.0);
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
                node_graph.draw(ui);
            });
    }

    fn draw_export_tab(
        &mut self,
        ui: &mut egui::Ui,
        export_callback: &mut dyn FnMut(),
        import_callback: &mut dyn FnMut(),
        workflow_metadata: &super::app::WorkflowMetadata,
    ) {
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
                        ui.label(&workflow_metadata.author);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Описание:")));
                        ui.label(&workflow_metadata.description);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Теги:")));
                        ui.label(&workflow_metadata.tags.join(", "));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Создан:")));
                        ui.label(&workflow_metadata.created_at);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(field_label_style("Изменен:")));
                        ui.label(&workflow_metadata.modified_at);
                    });
                });
            
            ui.add_space(10.0);
            ui.label("Экспортированные данные:");
            
            // Add a scrollable text area for the exported JSON
            ui.add(
                egui::TextEdit::multiline(&mut String::new())
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
                export_callback();
            }
            
            ui.separator();
            ui.heading("Импорт данных");
            
            // Add a scrollable text area for the imported JSON
            ui.add(
                egui::TextEdit::multiline(&mut String::new())
                    .desired_width(f32::INFINITY)
                    .desired_rows(15)
                    .hint_text("Вставьте JSON данные для импорта...")
                    .font(egui::TextStyle::Monospace.resolve(ui.style()))
            );
            
            ui.horizontal(|ui| {
                if ui.button("Импортировать").clicked() {
                    import_callback();
                }
                
                if ui.button("Очистить").clicked() {
                    // TODO: Clear import text
                }
            });
        });
    }

    fn draw_settings_tab(
        &mut self,
        ui: &mut egui::Ui,
        status_callback: &mut dyn FnMut(&str, f32),
    ) {
        ui.heading("Настройки");
        
        ui.add_space(10.0);
        
        ui.collapsing("Внешний вид", |ui| {
            // Theme settings
            ui.horizontal(|ui| {
                ui.label("Тема:");
                ui.selectable_value(&mut String::from("dark"), "dark".to_string(), "Темная");
                ui.selectable_value(&mut String::from("light"), "light".to_string(), "Светлая");
            });
            
            // Font size
            ui.horizontal(|ui| {
                ui.label("Размер шрифта:");
                if ui.button("-").clicked() {
                    status_callback("Уменьшение размера шрифта пока не реализовано", 2.0);
                }
                ui.label("100%");
                if ui.button("+").clicked() {
                    status_callback("Увеличение размера шрифта пока не реализовано", 2.0);
                }
            });
        });
        
        ui.collapsing("Граф", |ui| {
            // Grid settings
            ui.checkbox(&mut bool::default(), "Показывать сетку");
            ui.checkbox(&mut bool::default(), "Привязка к сетке");
            
            // Connection style
            ui.horizontal(|ui| {
                ui.label("Стиль соединений:");
                ui.selectable_value(&mut String::from("bezier"), "bezier".to_string(), "Кривые Безье");
                ui.selectable_value(&mut String::from("straight"), "straight".to_string(), "Прямые линии");
            });
        });
        
        ui.collapsing("Дополнительно", |ui| {
            if ui.button("Сбросить все настройки").clicked() {
                status_callback("Сброс настроек пока не реализован", 2.0);
            }
            
            if ui.button("Очистить кэш").clicked() {
                status_callback("Очистка кэша пока не реализована", 2.0);
            }
        });
    }

    fn draw_help_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("О программе");
        
        ui.add_space(10.0);
        
        ui.label("ВГАУ Бот Редактор v1.0");
        ui.label("Редактор структуры меню и FAQ для телеграм-бота Верхневолжского государственного аграрного университета");
        
        ui.add_space(20.0);
        
        ui.heading("Возможности");
        
        ui.label("• Интуитивный редактор на основе узлов с интерактивными элементами");
        ui.label("• Drag-and-drop интерфейс для создания и соединения компонентов");
        ui.label("• Визуальное редактирование структуры меню телеграм-бота");
        ui.label("• Управление вопросами и ответами (FAQ) через узловые компоненты");
        ui.label("• Экспорт и импорт данных в формате JSON");
        ui.label("• Контекстные меню и панель инструментов для быстрого доступа к функциям");
        ui.label("• Масштабирование и панорамирование рабочей области");
        ui.label("• Подробная настройка параметров бота через свойства узлов");
        ui.label("• История изменений с функциями отмены и повтора действий");
        
        ui.add_space(20.0);
        
        ui.heading("Горячие клавиши");
        
        ui.label("• Ctrl+Z — Отменить последнее действие");
        ui.label("• Ctrl+Y или Ctrl+Shift+Z — Повторить отмененное действие");
        ui.label("• Delete — Удалить выбранный узел");
        ui.label("• Ctrl+колесо мыши — Изменить масштаб");
        ui.label("• Перетаскивание с зажатым ЛКМ — Перемещение узлов");
        ui.label("• Перетаскивание с зажатым СКМ/Alt — Панорамирование рабочей области");
        
        ui.add_space(20.0);
        
        if ui.button("Закрыть").clicked() {
            self.current_tab = Tab::Editor;
        }
    }

    pub fn set_tab(&mut self, tab: Tab) {
        self.current_tab = tab;
    }

    pub fn get_current_tab(&self) -> &Tab {
        &self.current_tab
    }
} 