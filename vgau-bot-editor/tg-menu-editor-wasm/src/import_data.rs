use eframe::egui;
use crate::graph::SimpleNodeGraph;
use serde_json::Value;

/// Utility functions to import bot menu data from JSON
pub struct DataImporter;

impl DataImporter {
    /// Import bot menu structure from JSON data
    pub fn import_menu_data(graph: &mut SimpleNodeGraph, json_data: &str) -> Result<(), String> {
        // Parse JSON
        let data: Value = serde_json::from_str(json_data)
            .map_err(|e| format!("Error parsing JSON: {}", e))?;
        
        // Clear current graph (optional)
        // We choose not to clear here to allow importing into existing graphs
        
        // Import title if any
        let title = data.get("title")
            .and_then(|t| t.as_str())
            .unwrap_or("Telegram Bot Menu");
        
        // Create root menu node
        let main_menu_position = egui::pos2(100.0, 100.0);
        let main_menu_id = graph.add_menu_item(main_menu_position, title.to_string());
        
        // Import main menu items
        if let Some(menu_array) = data.get("main_menu").and_then(|m| m.as_array()) {
            Self::import_menu_items(graph, menu_array, main_menu_id, main_menu_position)?;
        }
        
        // Import FAQ items
        if let Some(faq_array) = data.get("faq").and_then(|f| f.as_array()) {
            Self::import_faq_items(graph, faq_array)?;
        }
        
        Ok(())
    }
    
    /// Import menu items from JSON array
    fn import_menu_items(
        graph: &mut SimpleNodeGraph, 
        items: &[Value], 
        parent_id: usize, 
        parent_pos: egui::Pos2
    ) -> Result<(), String> {
        let spacing_x = 300.0;
        let spacing_y = 120.0;
        
        for (i, item) in items.iter().enumerate() {
            // Calculate position for this menu item
            let position = egui::pos2(
                parent_pos.x + spacing_x,
                parent_pos.y + i as f32 * spacing_y
            );
            
            // Get menu item values
            let text = item.get("text")
                .and_then(|t| t.as_str())
                .unwrap_or("Unnamed Menu Item")
                .to_string();
                
            let callback_data = item.get("callback_data")
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .to_string();
                
            let description = item.get("description")
                .and_then(|d| d.as_str())
                .unwrap_or("")
                .to_string();
                
            let url = item.get("url")
                .and_then(|u| u.as_str())
                .unwrap_or("")
                .to_string();
                
            let text_content = item.get("text_content")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();
                
            // Create menu item node
            let item_id = graph.add_menu_item(position, text.clone());
            
            // Update node parameters
            if let Some(node) = graph.get_node_mut(item_id) {
                if let Some(param) = node.find_param_mut("callback_data") {
                    param.set_text_value(callback_data);
                }
                
                if let Some(param) = node.find_param_mut("description") {
                    param.set_text_value(description);
                }
                
                if let Some(param) = node.find_param_mut("url") {
                    param.set_text_value(url);
                }
                
                if let Some(param) = node.find_param_mut("text_content") {
                    param.set_text_value(text_content);
                }
                
                // Handle custom data
                if let Some(data_obj) = item.get("data") {
                    if let Some(param) = node.find_param_mut("data") {
                        param.set_text_value(data_obj.to_string());
                    } else {
                        // Add data parameter if it doesn't exist
                        node.add_data_parameter(data_obj.to_string());
                    }
                }
            }
            
            // Connect to parent
            graph.connect_nodes(parent_id, "sub_menu", item_id, "parent_menu");
            
            // Process submenu if exists
            if let Some(submenu) = item.get("submenu").and_then(|s| s.as_array()) {
                Self::import_menu_items(graph, submenu, item_id, position)?;
            }
            
            // Process documents if exists
            if let Some(docs) = item.get("documents").and_then(|d| d.as_array()) {
                Self::import_documents(graph, docs, item_id, position)?;
            }
        }
        
        Ok(())
    }
    
    /// Import documents for a menu item
    fn import_documents(
        graph: &mut SimpleNodeGraph,
        docs: &[Value],
        parent_id: usize,
        parent_pos: egui::Pos2
    ) -> Result<(), String> {
        let doc_spacing_x = 300.0;
        let doc_spacing_y = 80.0;
        
        for (i, doc) in docs.iter().enumerate() {
            // Calculate position for this document
            let position = egui::pos2(
                parent_pos.x + doc_spacing_x,
                parent_pos.y + i as f32 * doc_spacing_y
            );
            
            // Get document values
            let text = doc.get("text")
                .and_then(|t| t.as_str())
                .unwrap_or("Unnamed Document")
                .to_string();
                
            let callback_data = doc.get("callback_data")
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .to_string();
                
            let url = doc.get("url")
                .and_then(|u| u.as_str())
                .unwrap_or("")
                .to_string();
                
            // Create document node
            let doc_id = graph.add_document(position, text.clone())?;
            
            // Update node parameters
            if let Some(node) = graph.get_node_mut(doc_id) {
                if !callback_data.is_empty() {
                    if let Some(param) = node.find_param_mut("callback_data") {
                        param.set_text_value(callback_data);
                    }
                }
                
                if let Some(param) = node.find_param_mut("url") {
                    param.set_text_value(url);
                }
            }
            
            // Connect to parent
            graph.connect_nodes(parent_id, "documents", doc_id, "parent_menu");
        }
        
        Ok(())
    }
    
    /// Import FAQ items
    fn import_faq_items(
        graph: &mut SimpleNodeGraph,
        faqs: &[Value]
    ) -> Result<(), String> {
        let faq_start_x = 100.0;
        let faq_start_y = 500.0;
        let faq_spacing_y = 100.0;
        
        for (i, faq) in faqs.iter().enumerate() {
            // Calculate position for this FAQ item
            let position = egui::pos2(
                faq_start_x,
                faq_start_y + i as f32 * faq_spacing_y
            );
            
            // Get FAQ values
            let question = faq.get("question")
                .and_then(|q| q.as_str())
                .unwrap_or("Unnamed Question")
                .to_string();
                
            let answer = faq.get("answer")
                .and_then(|a| a.as_str())
                .unwrap_or("")
                .to_string();
                
            let tags = faq.get("tags")
                .and_then(|t| t.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
                
            // Create FAQ item node
            let faq_id = graph.add_faq_item(position, question.clone());
            
            // Update node parameters
            if let Some(node) = graph.get_node_mut(faq_id) {
                if let Some(param) = node.find_param_mut("answer") {
                    param.set_text_value(answer);
                }
                
                if let Some(param) = node.find_param_mut("tags") {
                    param.set_text_value(tags);
                }
            }
        }
        
        Ok(())
    }
} 