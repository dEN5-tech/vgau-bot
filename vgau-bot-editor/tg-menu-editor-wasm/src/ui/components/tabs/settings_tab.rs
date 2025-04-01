use eframe::egui;
use crate::MenuEditorApp;

pub struct SettingsTab;

impl SettingsTab {
    pub fn draw(app: &mut MenuEditorApp, ui: &mut egui::Ui) {
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
                    app.show_status("Уменьшение размера шрифта пока не реализовано", 2.0);
                }
                ui.label("100%");
                if ui.button("+").clicked() {
                    app.show_status("Увеличение размера шрифта пока не реализовано", 2.0);
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
                app.show_status("Сброс настроек пока не реализован", 2.0);
            }
            
            if ui.button("Очистить кэш").clicked() {
                app.show_status("Очистка кэша пока не реализована", 2.0);
            }
        });
    }
} 