use eframe::egui;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::graph::SimpleNodeGraph;
use crate::import_data::DataImporter;
use crate::ui::components::*;

mod menu_bar;
mod status_bar;
mod tabs;
mod utils;

use menu_bar::MenuBar;
use status_bar::StatusBar;
use tabs::{Tab, Tabs};
use utils::apply_tailwind_style;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct WorkflowMetadata {
    pub created_at: String,
    pub modified_at: String,
    pub author: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MenuItem {
    pub text: String,
    pub callback_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submenu: Option<Vec<MenuItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<Document>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Document {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FaqItem {
    pub question: String,
    pub answer: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct BotMenuConfig {
    pub title: String,
    pub main_menu: Vec<MenuItem>,
    pub faq: Vec<FaqItem>,
    #[serde(skip)]
    pub workflow_metadata: WorkflowMetadata,
}

/// Your app state
pub struct MenuEditorApp {
    // Node graph for menu editing
    pub node_graph: SimpleNodeGraph,
    // Current tab
    pub current_tab: Tab,
    // Exported data as string
    pub exported_data: String,
    // Import text field
    pub import_text: String,
    // Status message
    pub status_message: Option<(String, f32)>, // (message, time_remaining)
    // Current workflow ID
    pub workflow_id: String,
    // Workflow metadata
    pub workflow_metadata: WorkflowMetadata,
    // Dirty flag - indicates unsaved changes
    pub dirty: bool,
    tabs: Tabs,
    menu_bar: MenuBar,
    status_bar: StatusBar,
}

impl Default for MenuEditorApp {
    fn default() -> Self {
        let id = Uuid::new_v4().to_string();
        
        // Get current date/time as ISO string
        let now = js_sys::Date::new_0().to_iso_string().as_string()
            .unwrap_or_else(|| "Unknown date".to_string());
        
        Self {
            node_graph: SimpleNodeGraph::new(),
            current_tab: Tab::Editor,
            exported_data: String::new(),
            import_text: String::new(),
            status_message: None,
            workflow_id: id,
            workflow_metadata: WorkflowMetadata {
                created_at: now.clone(),
                modified_at: now,
                author: "User".to_string(),
                description: "New ВГАУ Bot Menu".to_string(),
                tags: vec!["menu".to_string(), "bot".to_string()],
            },
            dirty: false,
            tabs: Tabs::new(),
            menu_bar: MenuBar::new(id.clone(), false),
            status_bar: StatusBar::new(id.clone(), false, None),
        }
    }
}

impl MenuEditorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply custom styles to match React/Tailwind aesthetic
        let ctx = &cc.egui_ctx;
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
        
        Default::default()
    }

    pub fn export_data(&mut self) {
        // Update modified timestamp
        let now = js_sys::Date::new_0().to_iso_string().as_string()
            .unwrap_or_else(|| "Unknown date".to_string());
        self.workflow_metadata.modified_at = now;
        
        // Create a new BotMenuConfig structure with default title
        let mut config = BotMenuConfig {
            title: "Данные для кнопок для телеграмм бота Верхневолжского ГАУ".to_string(),
            main_menu: Vec::new(),
            faq: Vec::new(),
            workflow_metadata: self.workflow_metadata.clone(),
        };
        
        // Convert node_graph to BotMenuConfig structure
        self.build_menu_structure(&mut config);
        
        // Convert to JSON and store in exported_data
        match serde_json::to_string_pretty(&config) {
            Ok(json) => {
                self.exported_data = json;
                self.status_bar.update_status("Данные успешно экспортированы", 3.0);
                self.dirty = false;
            },
            Err(e) => {
                self.exported_data = format!("Ошибка экспорта: {}", e);
                self.status_bar.update_status("Ошибка при экспорте данных", 3.0);
            }
        }
    }
    
    // Function to build menu structure from node graph
    fn build_menu_structure(&self, config: &mut BotMenuConfig) {
        // Find all root menu items (nodes without input connections)
        let root_nodes = self.node_graph.get_root_menu_nodes();
        
        // Convert root nodes to menu items
        for node_id in root_nodes {
            if let Some(menu_item) = self.build_menu_item(node_id) {
                config.main_menu.push(menu_item);
            }
        }
        
        // Find all FAQ items
        let faq_nodes = self.node_graph.get_faq_nodes();
        
        // Convert FAQ nodes to FAQ items
        for node_id in faq_nodes {
            if let Some(faq_item) = self.build_faq_item(node_id) {
                config.faq.push(faq_item);
            }
        }
    }
    
    // Recursively build menu items
    fn build_menu_item(&self, node_id: usize) -> Option<MenuItem> {
        let node_data = self.node_graph.get_node_data(node_id)?;
        
        // Create menu item from node data
        let mut menu_item = MenuItem {
            text: node_data.get_title().to_string(),
            callback_data: self.generate_callback_data(node_data.get_title()),
            description: None,
            url: None,
            submenu: None,
            documents: None,
            data: None,
            text_content: None,
        };
        
        // Parse parameters from node
        for param in node_data.get_params() {
            match param.kind() {
                "name" => {
                    if !param.get_text().is_empty() {
                        menu_item.text = param.get_text();
                    }
                },
                "callback_data" => {
                    if !param.get_text().is_empty() {
                        menu_item.callback_data = param.get_text();
                    }
                },
                "description" => {
                    let desc = param.get_text();
                    if !desc.is_empty() {
                        menu_item.description = Some(desc);
                    }
                },
                "url" => {
                    let url = param.get_text();
                    if !url.is_empty() {
                        menu_item.url = Some(url);
                    }
                },
                "text_content" => {
                    let content = param.get_text();
                    if !content.is_empty() {
                        menu_item.text_content = Some(content);
                    }
                },
                "data" => {
                    let data = param.get_text();
                    if !data.is_empty() {
                        if let Ok(json) = serde_json::from_str(&data) {
                            menu_item.data = Some(json);
                        }
                    }
                },
                _ => {}
            }
        }
        
        // Find child nodes (submenu items)
        let children = self.node_graph.get_child_menu_nodes(node_id);
        if !children.is_empty() {
            let mut submenu = Vec::new();
            for child_id in children {
                if let Some(child_item) = self.build_menu_item(child_id) {
                    submenu.push(child_item);
                }
            }
            
            if !submenu.is_empty() {
                menu_item.submenu = Some(submenu);
            }
        }
        
        // Find document nodes
        let documents = self.node_graph.get_documents_for_node(node_id);
        if !documents.is_empty() {
            let mut doc_items = Vec::new();
            for doc_id in documents {
                if let Some(doc_data) = self.node_graph.get_node_data(doc_id) {
                    let mut doc = Document {
                        text: doc_data.get_title().to_string(),
                        callback_data: None,
                        url: String::new(),
                    };
                    
                    for param in doc_data.get_params() {
                        match param.kind() {
                            "text" => {
                                if !param.get_text().is_empty() {
                                    doc.text = param.get_text();
                                }
                            },
                            "callback_data" => {
                                let cb = param.get_text();
                                if !cb.is_empty() {
                                    doc.callback_data = Some(cb);
                                }
                            },
                            "url" => {
                                doc.url = param.get_text();
                            },
                            _ => {}
                        }
                    }
                    
                    // Only add if has URL
                    if !doc.url.is_empty() {
                        doc_items.push(doc);
                    }
                }
            }
            
            if !doc_items.is_empty() {
                menu_item.documents = Some(doc_items);
            }
        }
        
        Some(menu_item)
    }
    
    // Build FAQ items
    fn build_faq_item(&self, node_id: usize) -> Option<FaqItem> {
        let node_data = self.node_graph.get_node_data(node_id)?;
        
        // Create FAQ item
        let mut faq_item = FaqItem {
            question: node_data.get_title().to_string(),
            answer: String::new(),
            tags: Vec::new(),
        };
        
        // Parse parameters
        for param in node_data.get_params() {
            match param.kind() {
                "question" => {
                    if !param.get_text().is_empty() {
                        faq_item.question = param.get_text();
                    }
                },
                "answer" => {
                    faq_item.answer = param.get_text();
                },
                "tag" | "tags" => {
                    let tags_text = param.get_text();
                    if !tags_text.is_empty() {
                        faq_item.tags = tags_text.split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                },
                _ => {}
            }
        }
        
        // Only return if has non-empty answer
        if !faq_item.answer.is_empty() {
            Some(faq_item)
        } else {
            None
        }
    }
    
    // Generate a valid callback_data from title
    fn generate_callback_data(&self, title: &str) -> String {
        // Replace spaces with underscore, remove special characters, convert to lowercase
        let mut callback = title.to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
            .replace(' ', "_");
        
        // Limit length
        if callback.len() > 64 {
            callback = callback.chars().take(64).collect();
        }
        
        // Ensure it's not empty
        if callback.is_empty() {
            callback = format!("item_{}", rand::random::<u16>());
        }
        
        callback
    }
    
    pub fn import_data(&mut self) {
        // Try to parse the import text as JSON
        match serde_json::from_str::<BotMenuConfig>(&self.import_text) {
            Ok(config) => {
                // Update workflow metadata
                self.workflow_metadata = config.workflow_metadata;
                
                // TODO: Convert BotMenuConfig to node_graph
                // This is a simplified placeholder implementation
                
                self.current_tab = Tab::Editor;
                self.status_bar.update_status("Данные успешно импортированы", 3.0);
                self.dirty = false;
            },
            Err(e) => {
                self.status_bar.update_status(&format!("Ошибка импорта: {}", e), 3.0);
            }
        }
    }
    
    pub fn show_status(&mut self, message: &str, duration: f32) {
        self.status_message = Some((message.to_string(), duration));
    }

    // Update clipboard implementation
    pub fn copy_to_clipboard(&self, text: &str) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let window = web_sys::window().ok_or("No window found")?;
            let navigator = window.navigator();
            
            // Use clipboard API if available
            let clipboard = navigator.clipboard();
            if !clipboard.is_undefined() {
                let promise = clipboard.write_text(text);
                // Fire and forget - we can't await in synchronous code
                let _ = wasm_bindgen_futures::JsFuture::from(promise);
                return Ok(());
            }
            
            // Fallback method if clipboard API is not available
            let document = window.document().ok_or("No document found")?;
            
            // Create a temporary textarea
            let textarea: web_sys::HtmlTextAreaElement = document
                .create_element("textarea")
                .map_err(|_| "Failed to create textarea".to_string())?
                .dyn_into()
                .map_err(|_| "Failed to convert to textarea".to_string())?;
            
            // Set textarea properties
            textarea.set_value(text);
            textarea.style().set_property("position", "fixed").unwrap();
            textarea.style().set_property("top", "0").unwrap();
            textarea.style().set_property("left", "0").unwrap();
            textarea.style().set_property("width", "2em").unwrap();
            textarea.style().set_property("height", "2em").unwrap();
            textarea.style().set_property("padding", "0").unwrap();
            textarea.style().set_property("border", "none").unwrap();
            textarea.style().set_property("outline", "none").unwrap();
            textarea.style().set_property("box-shadow", "none").unwrap();
            textarea.style().set_property("background", "transparent").unwrap();
            
            // Append to document
            let body = document.body().ok_or("No body found".to_string())?;
            body.append_child(&textarea).map_err(|_| "Failed to append textarea".to_string())?;
            
            // Select the text
            textarea.select();
            
            // Try to execute copy command using JS
            let result = js_sys::Reflect::get(
                &document,
                &"execCommand".into()
            )
            .map_err(|_| "No execCommand method found".to_string())?;
            
            let exec_command = js_sys::Function::from(result);
            let success = exec_command
                .call1(&document, &"copy".into())
                .map_err(|_| "Failed to execute copy command".to_string())?;
                
            let success = js_sys::Boolean::from(success).value_of();
            
            // Clean up
            body.remove_child(&textarea).map_err(|_| "Failed to remove textarea".to_string())?;
            
            if success {
                Ok(())
            } else {
                Err("Copy command failed".to_string())
            }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // For non-WASM targets, we don't implement clipboard access
            Err("Clipboard not supported in this environment".to_string())
        }
    }

    /// Import menu data from JSON string
    pub fn import_from_json(&mut self, json_data: &str) -> Result<(), String> {
        DataImporter::import_menu_data(&mut self.node_graph, json_data)
    }

    /// Import predefined telegram bot data
    pub fn import_predefined_bot_data(&mut self) -> Result<(), String> {
        let telegram_bot_data = include_str!("../../static/default_bot_data.json");
        self.import_from_json(telegram_bot_data)
    }
}

impl eframe::App for MenuEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set global visuals to match React/Tailwind dark mode
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.panel_fill = egui::Color32::from_rgb(17, 24, 39); // bg-gray-900
        ctx.set_style(style);
        
        // Handle global keyboard shortcuts
        ctx.input(|i| {
            // Only process keyboard shortcuts when in Editor tab
            if self.current_tab == Tab::Editor {
                // Undo: Ctrl+Z
                if i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && !i.modifiers.shift {
                    if self.node_graph.can_undo() {
                        if self.node_graph.undo() {
                            self.status_bar.update_status("Отменено последнее действие", 2.0);
                            self.dirty = true;
                        } else {
                            self.status_bar.update_status("Нет действий для отмены", 2.0);
                        }
                    } else {
                        self.status_bar.update_status("Нет действий для отмены", 2.0);
                    }
                }
                
                // Redo: Ctrl+Y or Ctrl+Shift+Z
                if (i.key_pressed(egui::Key::Y) && i.modifiers.ctrl) || 
                   (i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && i.modifiers.shift) {
                    if self.node_graph.can_redo() {
                        if self.node_graph.redo() {
                            self.status_bar.update_status("Действие возвращено", 2.0);
                            self.dirty = true;
                        } else {
                            self.status_bar.update_status("Нет действий для возврата", 2.0);
                        }
                    } else {
                        self.status_bar.update_status("Нет действий для возврата", 2.0);
                    }
                }
            }
        });
        
        // Draw the menu bar
        self.menu_bar.draw(
            ctx,
            &mut self.node_graph,
            &mut |msg, duration| self.status_bar.update_status(msg, duration),
            &mut |tab| self.tabs.set_tab(tab),
            &mut || self.export_data(),
            &mut || self.import_predefined_bot_data(),
            &self.workflow_metadata,
        );
        
        // Main content with styling similar to App.jsx
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(17, 24, 39))) // bg-gray-900
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0); // Match Tailwind spacing
                
                // Draw the current tab
                self.tabs.draw(
                    ui,
                    &mut self.node_graph,
                    &mut |msg, duration| self.status_bar.update_status(msg, duration),
                    &mut || self.export_data(),
                    &mut || self.import_data(),
                    &self.workflow_metadata,
                );
            });
        
        // Draw the status bar
        self.status_bar.draw(ctx, &self.node_graph, ctx);
    }
} 