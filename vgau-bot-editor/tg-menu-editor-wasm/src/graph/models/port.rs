use eframe::egui;

#[derive(Clone, Default)]
pub struct Port {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) port_type: PortType,
    pub(crate) color: egui::Color32,
}

#[derive(Clone, Default, PartialEq)]
pub enum PortType {
    #[default]
    String,
    Number,
    Object,
    Action,
}

impl Port {
    /// Create a new port with the given id, label, and port type
    pub fn new(id: String, label: String, port_type: PortType) -> Self {
        let color = match port_type {
            PortType::String => egui::Color32::from_rgb(200, 200, 100),
            PortType::Number => egui::Color32::from_rgb(100, 200, 200),
            PortType::Object => egui::Color32::from_rgb(150, 200, 255),
            PortType::Action => egui::Color32::from_rgb(255, 150, 150),
        };
        
        Self {
            id,
            label,
            port_type,
            color,
        }
    }
    
    /// Get the port id
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get the port label
    pub fn label(&self) -> &str {
        &self.label
    }
    
    /// Get the port type
    pub fn port_type(&self) -> &PortType {
        &self.port_type
    }
} 