use eframe::egui;
use crate::graph::models::{Node, NodeType, Connection, PortType, ParameterType, ParameterValue};

/// Provides operations for managing nodes and connections in the graph
pub struct GraphOperations;

impl GraphOperations {
    /// Create a new menu item node
    pub fn create_menu_item(id: usize, title: String, position: egui::Pos2) -> Node {
        let mut node = Node::new(id, title.clone(), position, NodeType::MenuItem);
        
        // Add standard inputs and outputs for menu item nodes
        node.add_input(
            "parent_menu".to_string(),
            "Родительское меню".to_string(),
            PortType::Object
        );
        
        node.add_output(
            "sub_menu".to_string(),
            "Подменю".to_string(),
            PortType::Object
        );
        
        node.add_output(
            "documents".to_string(),
            "Документы".to_string(),
            PortType::Object
        );
        
        // Add standard parameters
        node.add_parameter(
            "name".to_string(),
            "Название".to_string(),
            ParameterType::Text,
            ParameterValue::Text(title)
        );
        
        node.add_parameter(
            "callback_data".to_string(),
            "Callback Data".to_string(),
            ParameterType::Text,
            ParameterValue::Text(String::new())
        );
        
        node.add_parameter(
            "description".to_string(),
            "Описание".to_string(),
            ParameterType::Text,
            ParameterValue::Text(String::new())
        );
        
        node.add_parameter(
            "url".to_string(),
            "URL".to_string(),
            ParameterType::Text,
            ParameterValue::Text(String::new())
        );
        
        node.add_parameter(
            "text_content".to_string(),
            "Текстовое содержимое".to_string(),
            ParameterType::Text,
            ParameterValue::Text(String::new())
        );
        
        node
    }
    
    /// Create a new FAQ item node
    pub fn create_faq_item(id: usize, title: String, position: egui::Pos2) -> Node {
        let mut node = Node::new(id, title.clone(), position, NodeType::FaqItem);
        
        // FAQ items don't need ports as they are not connected in the graph
        
        // Add standard parameters for FAQ items
        node.add_parameter(
            "question".to_string(),
            "Вопрос".to_string(),
            ParameterType::Text,
            ParameterValue::Text(title)
        );
        
        node.add_parameter(
            "answer".to_string(),
            "Ответ".to_string(),
            ParameterType::Text,
            ParameterValue::Text(String::new())
        );
        
        node.add_parameter(
            "tags".to_string(),
            "Теги (через запятую)".to_string(),
            ParameterType::Text,
            ParameterValue::Text(String::new())
        );
        
        node
    }
    
    /// Create a new document node
    pub fn create_document(id: usize, title: String, position: egui::Pos2) -> Node {
        let mut node = Node::new(id, title.clone(), position, NodeType::Document);
        
        // Add standard inputs for document nodes
        node.add_input(
            "parent_menu".to_string(),
            "Родительское меню".to_string(),
            PortType::Object
        );
        
        // Add standard parameters for documents
        node.add_parameter(
            "text".to_string(),
            "Название".to_string(),
            ParameterType::Text,
            ParameterValue::Text(title)
        );
        
        node.add_parameter(
            "callback_data".to_string(),
            "Callback Data".to_string(),
            ParameterType::Text,
            ParameterValue::Text(String::new())
        );
        
        node.add_parameter(
            "url".to_string(),
            "URL".to_string(),
            ParameterType::Text,
            ParameterValue::Text(String::new())
        );
        
        node
    }
    
    /// Check if document creation is supported
    pub fn supports_document_creation() -> bool {
        true
    }
    
    /// Check if a connection between two nodes is valid
    pub fn is_valid_connection(
        from_node: &Node,
        from_port: &str,
        to_node: &Node,
        to_port: &str
    ) -> bool {
        // Get the port types
        let from_port_type = from_node.outputs.iter()
            .find(|p| p.id() == from_port)
            .map(|p| p.port_type());
            
        let to_port_type = to_node.inputs.iter()
            .find(|p| p.id() == to_port)
            .map(|p| p.port_type());
            
        // Check if ports exist and have compatible types
        if let (Some(from_type), Some(to_type)) = (from_port_type, to_port_type) {
            // For now, we just check if types are the same
            from_type == to_type
        } else {
            false
        }
    }
    
    /// Create a connection between two nodes
    pub fn create_connection(
        from_node: usize,
        from_port: String,
        to_node: usize,
        to_port: String
    ) -> Connection {
        Connection::new(from_node, from_port, to_node, to_port)
    }
} 