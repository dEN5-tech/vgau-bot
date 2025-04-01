use eframe::egui;
use super::{Port, Parameter};

#[derive(Clone, Default)]
pub struct Node {
    pub(crate) id: usize,
    pub(crate) title: String,
    pub(crate) position: egui::Pos2,
    pub(crate) size: egui::Vec2,
    pub(crate) color: egui::Color32,
    pub(crate) node_type: NodeType,
    pub(crate) inputs: Vec<Port>,
    pub(crate) outputs: Vec<Port>,
    pub(crate) params: Vec<Parameter>,
}

#[derive(Clone, Default, PartialEq)]
pub enum NodeType {
    #[default]
    MenuItem,
    FaqItem,
    Process,
    Input,
    Output,
    Document,
}

impl Node {
    /// Get the title of the node
    pub fn get_title(&self) -> &str {
        &self.title
    }
    
    /// Get the parameters of the node
    pub fn get_params(&self) -> &Vec<Parameter> {
        &self.params
    }
    
    /// Create a new node with the given id, title, position, and node type
    pub fn new(id: usize, title: String, position: egui::Pos2, node_type: NodeType) -> Self {
        Self {
            id,
            title,
            position,
            size: egui::vec2(180.0, 100.0),
            color: match node_type {
                NodeType::MenuItem => egui::Color32::from_rgb(100, 150, 200),
                NodeType::FaqItem => egui::Color32::from_rgb(200, 150, 100),
                NodeType::Process => egui::Color32::from_rgb(150, 200, 100),
                NodeType::Input => egui::Color32::from_rgb(100, 200, 150),
                NodeType::Output => egui::Color32::from_rgb(200, 100, 150),
                NodeType::Document => egui::Color32::from_rgb(100, 100, 200),
            },
            node_type,
            inputs: Vec::new(),
            outputs: Vec::new(),
            params: Vec::new(),
        }
    }
    
    /// Add an input port to the node
    pub fn add_input(&mut self, id: String, label: String, port_type: super::PortType) {
        self.inputs.push(Port::new(id, label, port_type));
    }
    
    /// Add an output port to the node
    pub fn add_output(&mut self, id: String, label: String, port_type: super::PortType) {
        self.outputs.push(Port::new(id, label, port_type));
    }
    
    /// Add a parameter to the node
    pub fn add_parameter(&mut self, id: String, label: String, param_type: super::ParameterType, value: super::ParameterValue) {
        self.params.push(Parameter::new(id, label, param_type, value));
    }
    
    /// Get the node id
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// Find a parameter by its ID
    pub fn find_param(&self, id: &str) -> Option<&Parameter> {
        self.params.iter().find(|p| p.id() == id)
    }
    
    /// Find a parameter by its ID and get a mutable reference
    pub fn find_param_mut(&mut self, id: &str) -> Option<&mut Parameter> {
        self.params.iter_mut().find(|p| p.id() == id)
    }
    
    /// Add a data parameter to the node (for custom JSON data)
    pub fn add_data_parameter(&mut self, data: String) {
        self.add_parameter(
            "data".to_string(),
            "Custom Data".to_string(), 
            super::ParameterType::Text,
            super::ParameterValue::Text(data)
        );
    }
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::MenuItem => "MenuItem",
            NodeType::FaqItem => "FaqItem",
            NodeType::Process => "Process",
            NodeType::Input => "Input",
            NodeType::Output => "Output",
            NodeType::Document => "document",
        }
    }
} 