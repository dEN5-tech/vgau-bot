// This file provides backward compatibility with the old structure
// It re-exports the new modular structure

// Re-export everything from the new graph module
pub use crate::graph::*;

use eframe::egui;
use std::collections::VecDeque;

// Create a history snapshot type
#[derive(Clone)]
struct GraphSnapshot {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    active_node: Option<usize>,
    drag_offset: egui::Vec2,
    zoom: f32,
}

#[derive(Clone, Default)]
pub struct Node {
    id: usize,
    title: String,
    position: egui::Pos2,
    size: egui::Vec2,
    color: egui::Color32,
    node_type: NodeType,
    inputs: Vec<Port>,
    outputs: Vec<Port>,
    params: Vec<Parameter>,
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

#[derive(Clone, Default)]
pub struct Port {
    id: String,
    label: String,
    port_type: PortType,
    color: egui::Color32,
}

#[derive(Clone, Default, PartialEq)]
pub enum PortType {
    #[default]
    String,
    Number,
    Object,
    Action,
}

#[derive(Clone)]
pub struct Parameter {
    id: String,
    label: String,
    param_type: ParameterType,
    value: ParameterValue,
}

#[derive(Clone, PartialEq)]
pub enum ParameterType {
    Text,
    Number,
    Boolean,
    Select,
}

#[derive(Clone)]
pub enum ParameterValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    Select(String, Vec<String>),
}

#[derive(Clone)]
pub struct Connection {
    from_node: usize,
    to_node: usize,
    from_port: String,
    to_port: String,
}

pub struct SimpleNodeGraph {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    pub active_node: Option<usize>,
    drag_offset: egui::Vec2,
    zoom: f32,
    clipboard: Option<Node>,
    connecting_port: Option<(usize, String, bool)>, // (node_id, port_id, is_input)
    context_menu_open: bool,
    editing_node: Option<usize>, // New field to track which node is being edited
    // History system
    history: VecDeque<GraphSnapshot>,
    redo_stack: VecDeque<GraphSnapshot>,
    history_size_limit: usize,
    history_action_in_progress: bool,
}

impl Default for SimpleNodeGraph {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            active_node: None,
            drag_offset: egui::Vec2::ZERO,
            zoom: 1.0,
            clipboard: None,
            connecting_port: None,
            context_menu_open: false,
            editing_node: None, // Initialize as None
            // Initialize history system
            history: VecDeque::new(),
            redo_stack: VecDeque::new(),
            history_size_limit: 30, // Default history limit
            history_action_in_progress: false,
        }
    }
}

impl SimpleNodeGraph {
    pub fn new() -> Self {
        let mut graph = Self::default();
        graph.initialize_sample_nodes();
        
        // Create initial history snapshot
        graph.initialize_history();
        
        graph
    }

    fn initialize_sample_nodes(&mut self) {
        // Create initial menu node
        self.nodes.push(Node {
            id: 0,
            title: "Главное меню".to_string(),
            position: egui::pos2(100.0, 100.0),
            size: egui::vec2(180.0, 100.0),
            color: egui::Color32::from_rgb(100, 150, 200),
            node_type: NodeType::MenuItem,
            inputs: Vec::new(),
            outputs: vec![
                Port {
                    id: "sub_menu".to_string(),
                    label: "Подменю".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(150, 200, 255),
                }
            ],
            params: vec![
                Parameter {
                    id: "name".to_string(),
                    label: "Название".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("Главное меню".to_string()),
                },
                Parameter {
                    id: "welcome_text".to_string(),
                    label: "Текст приветствия".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("Добро пожаловать в бот ВГАУ!".to_string()),
                },
            ],
        });

        // Create submenu node
        self.nodes.push(Node {
            id: 1,
            title: "Подменю: О университете".to_string(),
            position: egui::pos2(400.0, 100.0),
            size: egui::vec2(180.0, 120.0),
            color: egui::Color32::from_rgb(100, 200, 150),
            node_type: NodeType::MenuItem,
            inputs: vec![
                Port {
                    id: "parent_menu".to_string(),
                    label: "Родительское меню".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(150, 200, 255),
                }
            ],
            outputs: vec![
                Port {
                    id: "sub_menu".to_string(),
                    label: "Подменю".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(150, 200, 255),
                },
                Port {
                    id: "faq".to_string(),
                    label: "FAQ".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(255, 200, 150),
                }
            ],
            params: vec![
                Parameter {
                    id: "name".to_string(),
                    label: "Название".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("О университете".to_string()),
                },
                Parameter {
                    id: "description".to_string(),
                    label: "Описание".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("Информация о ВГАУ".to_string()),
                },
            ],
        });

        // Create FAQ node
        self.nodes.push(Node {
            id: 2,
            title: "FAQ: Часто задаваемые вопросы".to_string(),
            position: egui::pos2(700.0, 100.0),
            size: egui::vec2(200.0, 150.0),
            color: egui::Color32::from_rgb(200, 150, 100),
            node_type: NodeType::FaqItem,
            inputs: vec![
                Port {
                    id: "parent_menu".to_string(),
                    label: "Родительское меню".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(150, 200, 255),
                }
            ],
            outputs: Vec::new(),
            params: vec![
                Parameter {
                    id: "question".to_string(),
                    label: "Вопрос".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("Как поступить в ВГАУ?".to_string()),
                },
                Parameter {
                    id: "answer".to_string(),
                    label: "Ответ".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("Для поступления необходимо подать документы в приемную комиссию...".to_string()),
                },
                Parameter {
                    id: "tag".to_string(),
                    label: "Теги".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("поступление, абитуриент".to_string()),
                },
            ],
        });

        // Add connections
        self.connections.push(Connection {
            from_node: 0,
            to_node: 1,
            from_port: "sub_menu".to_string(),
            to_port: "parent_menu".to_string(),
        });

        self.connections.push(Connection {
            from_node: 1,
            to_node: 2,
            from_port: "faq".to_string(),
            to_port: "parent_menu".to_string(),
        });
    }

    // Initialize history with the current state
    fn initialize_history(&mut self) {
        // Create the initial snapshot
        let snapshot = self.create_snapshot();
        
        // Add to history
        self.history.push_back(snapshot);
    }

    // Add a new method to create a menu item node
    pub fn add_menu_item(&mut self, position: egui::Pos2, title: String) -> usize {
        // Save state before making changes
        self.save_state();

        let id = self.next_node_id();
        self.nodes.push(Node {
            id,
            title,
            position,
            size: egui::vec2(180.0, 100.0),
            color: egui::Color32::from_rgb(100, 150, 200),
            node_type: NodeType::MenuItem,
            inputs: vec![
                Port {
                    id: "parent_menu".to_string(),
                    label: "Родительское меню".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(150, 200, 255),
                }
            ],
            outputs: vec![
                Port {
                    id: "sub_menu".to_string(),
                    label: "Подменю".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(150, 200, 255),
                },
                Port {
                    id: "faq".to_string(),
                    label: "FAQ".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(255, 200, 150),
                }
            ],
            params: vec![
                Parameter {
                    id: "name".to_string(),
                    label: "Название".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("Новое меню".to_string()),
                },
                Parameter {
                    id: "description".to_string(),
                    label: "Описание".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("".to_string()),
                },
            ],
        });
        id
    }

    // Add a new method to create a FAQ item node
    pub fn add_faq_item(&mut self, position: egui::Pos2, title: String) -> usize {
        // Save state before making changes
        self.save_state();

        let id = self.next_node_id();
        self.nodes.push(Node {
            id,
            title,
            position,
            size: egui::vec2(200.0, 150.0),
            color: egui::Color32::from_rgb(200, 150, 100),
            node_type: NodeType::FaqItem,
            inputs: vec![
                Port {
                    id: "parent_menu".to_string(),
                    label: "Родительское меню".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(150, 200, 255),
                }
            ],
            outputs: Vec::new(),
            params: vec![
                Parameter {
                    id: "question".to_string(),
                    label: "Вопрос".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("".to_string()),
                },
                Parameter {
                    id: "answer".to_string(),
                    label: "Ответ".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("".to_string()),
                },
                Parameter {
                    id: "tag".to_string(),
                    label: "Теги".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("".to_string()),
                },
            ],
        });
        id
    }

    // Add a method to connect nodes
    pub fn connect_nodes(&mut self, from_node: usize, from_port: &str, to_node: usize, to_port: &str) -> bool {
        // Check if connection already exists
        for conn in &self.connections {
            if conn.from_node == from_node && conn.to_node == to_node 
               && conn.from_port == from_port && conn.to_port == to_port {
                return false;
            }
        }

        // Save state before making changes
        self.save_state();

        // Add new connection
        self.connections.push(Connection {
            from_node,
            to_node,
            from_port: from_port.to_string(),
            to_port: to_port.to_string(),
        });
        true
    }

    // Add a method to delete a node
    pub fn delete_node(&mut self, node_id: usize) {
        // Save state before making changes
        self.save_state();

        // Remove connections to/from this node
        self.connections.retain(|conn| conn.from_node != node_id && conn.to_node != node_id);
        
        // Remove the node
        self.nodes.retain(|node| node.id != node_id);
        
        // Reset active node if it was the deleted one
        if self.active_node == Some(node_id) {
            self.active_node = None;
        }
    }

    // Add a method to delete a connection
    pub fn delete_connection(&mut self, from_node: usize, from_port: &str, to_node: usize, to_port: &str) {
        // Save state before making changes
        self.save_state();

        self.connections.retain(|conn| 
            !(conn.from_node == from_node && conn.to_node == to_node 
              && conn.from_port == from_port && conn.to_port == to_port)
        );
    }

    // Helper method to get next node ID
    fn next_node_id(&self) -> usize {
        self.nodes.iter().map(|node| node.id).max().unwrap_or(0) + 1
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        let available_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());
        
        // Handle keyboard shortcuts
        let mut should_undo = false;
        let mut should_redo = false;
        let mut should_delete = false;
        let mut node_to_delete = None;
        
        ui.input(|i| {
            // Undo: Ctrl+Z
            if i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && !i.modifiers.shift {
                should_undo = true;
            }
            
            // Redo: Ctrl+Y or Ctrl+Shift+Z
            if (i.key_pressed(egui::Key::Y) && i.modifiers.ctrl) || 
               (i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && i.modifiers.shift) {
                should_redo = true;
            }
            
            // Delete: Del key for selected node
            if i.key_pressed(egui::Key::Delete) && self.active_node.is_some() {
                should_delete = true;
                node_to_delete = self.active_node;
            }
        });
        
        // Handle actions outside the loop to avoid borrow issues
        if should_undo {
            if self.undo() {
                ui.ctx().request_repaint();
            }
        }
        
        if should_redo {
            if self.redo() {
                ui.ctx().request_repaint();
            }
        }
        
        if should_delete && node_to_delete.is_some() {
            self.delete_node(node_to_delete.unwrap());
            ui.ctx().request_repaint();
        }
        
        // Handle zooming with scroll
        let mut should_save_zoom_state = false;
        ui.input(|i| {
            if i.scroll_delta.y != 0.0 && i.modifiers.ctrl {
                // Save current state before zooming
                if !self.history_action_in_progress {
                    should_save_zoom_state = true;
                    self.history_action_in_progress = true;
                }
                
                self.zoom *= if i.scroll_delta.y > 0.0 { 1.1 } else { 0.9 };
                self.zoom = self.zoom.clamp(0.5, 3.0);
            } else if self.history_action_in_progress && i.scroll_delta.y == 0.0 {
                // End of continuous action like zooming
                self.history_action_in_progress = false;
            }
        });
        
        if should_save_zoom_state {
            self.save_state();
        }
        
        // Detect canvas drag
        if response.dragged() && !ui.input(|i| i.modifiers.alt) {
            self.drag_offset += response.drag_delta();
        }

        // For graph background
        response.context_menu(|ui| {
            // Set context menu flag to track open/close state
            self.context_menu_open = true;
            
            // Clear active node when clicking on background
            self.active_node = None;
            
            ui.set_min_width(150.0);
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 8.0);
            ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 55, 72);
            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(59, 130, 246);
            ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgb(29, 78, 216);
            
            // Get the graph position for new nodes
            let mouse_pos = ui.input(|i| i.pointer.interact_pos().unwrap_or_default());
            let graph_pos = mouse_pos - self.drag_offset;
            
            ui.add_space(4.0);
            
            if ui.button(egui::RichText::new("Добавить пункт меню").size(14.0).color(egui::Color32::WHITE)).clicked() {
                self.add_menu_item(graph_pos, "Новый пункт меню".to_string());
                ui.close_menu();
            }
            
            ui.add_space(2.0);
            
            if ui.button(egui::RichText::new("Добавить FAQ").size(14.0).color(egui::Color32::WHITE)).clicked() {
                self.add_faq_item(graph_pos, "Новый FAQ".to_string());
                ui.close_menu();
            }
            
            if let Some(_) = &self.clipboard {
                ui.add_space(2.0);
                
                if ui.button(egui::RichText::new("Вставить узел").size(14.0).color(egui::Color32::WHITE)).clicked() {
                    if let Some(node) = &self.clipboard {
                        let mut new_node = node.clone();
                        new_node.id = self.next_node_id();
                        new_node.position = graph_pos;
                        self.nodes.push(new_node);
                        self.save_state();
                    }
                    ui.close_menu();
                }
            }
            
            ui.add_space(4.0);
        });

        // Draw background with a more modern look
        ui.painter().rect_filled(
            available_rect,
            0.0,
            egui::Color32::from_rgb(30, 30, 40) // Dark background, similar to the React app
        );

        // Draw grid with Tailwind-style colors
        self.draw_grid(ui, available_rect);

        // Draw connections with enhanced styling
        for connection in &self.connections.clone() {
            self.draw_connection(ui, connection);
        }

        // Draw pending connection if there is one
        if let Some((node_id, port_id, is_input)) = &self.connecting_port {
            let start_pos = if let Some(pos) = self.get_port_position(*node_id, port_id, *is_input) {
                pos
            } else {
                egui::Pos2::ZERO
            };
            
            let end_pos = ui.input(|i| i.pointer.interact_pos().unwrap_or(start_pos));
            
            if start_pos != egui::Pos2::ZERO {
                // Use a more vibrant color for pending connections
                let color = egui::Color32::from_rgb(96, 165, 250); // blue-400 from Tailwind
                self.draw_bezier_connection(ui, 
                    if *is_input { end_pos } else { start_pos }, 
                    if *is_input { start_pos } else { end_pos }, 
                    color);
            }
        }

        // Track if we need to create any connections after the loop
        let mut connect_request: Option<(usize, String, usize, String)> = None;

        // Track if any node started being dragged
        let mut any_node_drag_started = false;
        
        // Store a list of responses for processing context menus outside main borrow
        let mut node_responses = Vec::new();

        // Draw nodes and handle interactions
        let mut clicked_node_idx = None;
        let mut node_responses = Vec::new();

        // First pass: Draw nodes and collect responses
        for (i, node) in self.nodes.iter_mut().enumerate() {
            // Apply zoom to the node
            let scaled_pos = egui::pos2(
                node.position.x * self.zoom, 
                node.position.y * self.zoom
            );
            let scaled_size = node.size * self.zoom;
            
            let node_rect = egui::Rect::from_min_size(
                scaled_pos + self.drag_offset,
                scaled_size
            );
            
            let node_response = ui.allocate_rect(node_rect, egui::Sense::click_and_drag());
            
            // Handle node dragging
            if node_response.drag_started() {
                any_node_drag_started = true;
            }
            
            if node_response.dragged() {
                node.position += node_response.drag_delta() / self.zoom;
            }
            
            // Handle selection
            if node_response.clicked() {
                self.active_node = Some(node.id);
                clicked_node_idx = Some(i);
            }

            // Store response for context menu processing later
            node_responses.push((node.id, node_response.clone()));
            
            // Draw the node with enhanced styling
            let is_active = self.active_node == Some(node.id);
            
            // Get node type-specific colors (inspired by Tailwind palette)
            let (bg_color, border_color) = match node.node_type {
                NodeType::MenuItem => (
                    if is_active { egui::Color32::from_rgb(30, 58, 138) } // blue-900 
                    else { egui::Color32::from_rgb(30, 58, 138).gamma_multiply(0.8) }, // slightly darker
                    egui::Color32::from_rgb(59, 130, 246) // blue-500
                ),
                NodeType::FaqItem => (
                    if is_active { egui::Color32::from_rgb(76, 29, 149) } // purple-900
                    else { egui::Color32::from_rgb(76, 29, 149).gamma_multiply(0.8) }, // slightly darker
                    egui::Color32::from_rgb(139, 92, 246) // purple-500
                ),
                NodeType::Process => (
                    if is_active { egui::Color32::from_rgb(20, 83, 45) } // green-900
                    else { egui::Color32::from_rgb(20, 83, 45).gamma_multiply(0.8) }, // slightly darker
                    egui::Color32::from_rgb(34, 197, 94) // green-500
                ),
                NodeType::Input => (
                    if is_active { egui::Color32::from_rgb(120, 53, 15) } // amber-900
                    else { egui::Color32::from_rgb(120, 53, 15).gamma_multiply(0.8) }, // slightly darker
                    egui::Color32::from_rgb(245, 158, 11) // amber-500
                ),
                NodeType::Output => (
                    if is_active { egui::Color32::from_rgb(127, 29, 29) } // red-900
                    else { egui::Color32::from_rgb(127, 29, 29).gamma_multiply(0.8) }, // slightly darker
                    egui::Color32::from_rgb(239, 68, 68) // red-500
                ),
                NodeType::Document => (
                    if is_active { egui::Color32::from_rgb(100, 100, 100) } // gray-900
                    else { egui::Color32::from_rgb(100, 100, 100).gamma_multiply(0.8) }, // slightly darker
                    egui::Color32::from_rgb(150, 150, 150) // gray-500
                ),
            };
            
            // Draw node background with rounded corners
            ui.painter().rect_filled(
                node_rect,
                8.0, // More pronounced rounded corners like in Tailwind
                bg_color
            );
            
            // Draw node border with more prominent active state
            ui.painter().rect_stroke(
                node_rect,
                8.0, // Matching corner radius
                egui::Stroke::new(
                    if is_active { 2.0 } else { 1.0 },
                    if is_active { border_color } else { border_color.gamma_multiply(0.7) }
                )
            );
            
            // Draw node title with improved styling
            let title_rect = egui::Rect::from_min_max(
                node_rect.min,
                egui::pos2(node_rect.max.x, node_rect.min.y + 28.0 * self.zoom) // Slightly taller header
            );
            
            // Title background for better separation
            ui.painter().rect_filled(
                title_rect,
                egui::Rounding { 
                    nw: 8.0, 
                    ne: 8.0,
                    sw: 0.0,
                    se: 0.0,
                },
                border_color.gamma_multiply(0.8) // Slightly darker than the border
            );
            
            // Title text
            ui.painter().text(
                title_rect.center(),
                egui::Align2::CENTER_CENTER,
                &node.title,
                egui::FontId::proportional(14.0 * self.zoom),
                egui::Color32::WHITE
            );

            // Draw horizontal line below title (not needed anymore with background)
            // Instead add a subtle inner shadow effect
            ui.painter().line_segment(
                [
                    egui::pos2(node_rect.min.x + 1.0, node_rect.min.y + 28.0 * self.zoom),
                    egui::pos2(node_rect.max.x - 1.0, node_rect.min.y + 28.0 * self.zoom)
                ],
                egui::Stroke::new(1.0, egui::Color32::from_gray(0).gamma_multiply(0.3))
            );

            // Draw input ports with improved styling
            let mut input_y = node_rect.min.y + 45.0 * self.zoom; // Add more space after title
            for input in &node.inputs {
                // Draw input port
                let port_pos = egui::pos2(node_rect.min.x, input_y);
                let port_radius = 6.0 * self.zoom;
                
                // Port styling based on type 
                let port_color = match input.port_type {
                    PortType::String => egui::Color32::from_rgb(96, 165, 250), // blue-400
                    PortType::Number => egui::Color32::from_rgb(167, 139, 250), // purple-400
                    PortType::Object => egui::Color32::from_rgb(251, 146, 60), // orange-400
                    PortType::Action => egui::Color32::from_rgb(74, 222, 128), // green-400
                };
                
                // Draw port with inner/outer circle for depth
                ui.painter().circle_filled(
                    port_pos,
                    port_radius,
                    egui::Color32::from_gray(20) // Dark background
                );
                
                ui.painter().circle_filled(
                    port_pos,
                    port_radius - 1.5,
                    port_color
                );

                // Port hover effect
                let port_rect = egui::Rect::from_center_size(
                    port_pos, egui::vec2(port_radius * 2.5, port_radius * 2.5)
                );
                let port_response = ui.allocate_rect(port_rect, egui::Sense::click());
                
                if port_response.hovered() {
                    ui.painter().circle_stroke(
                        port_pos,
                        port_radius + 2.0,
                        egui::Stroke::new(1.0, egui::Color32::WHITE)
                    );
                }
                
                // Handle port clicking for connections
                if port_response.clicked() {
                    if let Some((other_node, other_port, is_other_input)) = &self.connecting_port {
                        if !*is_other_input { // Only connect if the other port is an output
                            // Store the request to connect - will be processed after the loop
                            connect_request = Some((*other_node, other_port.clone(), node.id, input.id.clone()));
                        }
                        self.connecting_port = None;
                    } else {
                        // Start a new connection from this input
                        self.connecting_port = Some((node.id, input.id.clone(), true));
                    }
                }

                // Draw port label with improved styling
                ui.painter().text(
                    egui::pos2(port_pos.x + 15.0 * self.zoom, port_pos.y),
                    egui::Align2::LEFT_CENTER,
                    &input.label,
                    egui::FontId::proportional(12.0 * self.zoom),
                    egui::Color32::from_rgb(229, 231, 235) // gray-200 - brighter text
                );

                input_y += 25.0 * self.zoom;
            }

            // Draw output ports with improved styling
            let mut output_y = node_rect.min.y + 45.0 * self.zoom; // Add more space after title
            for output in &node.outputs {
                // Draw output port
                let port_pos = egui::pos2(node_rect.max.x, output_y);
                let port_radius = 6.0 * self.zoom;
                
                // Port styling based on type
                let port_color = match output.port_type {
                    PortType::String => egui::Color32::from_rgb(96, 165, 250), // blue-400
                    PortType::Number => egui::Color32::from_rgb(167, 139, 250), // purple-400
                    PortType::Object => egui::Color32::from_rgb(251, 146, 60), // orange-400
                    PortType::Action => egui::Color32::from_rgb(74, 222, 128), // green-400
                };
                
                // Draw port with inner/outer circle for depth
                ui.painter().circle_filled(
                    port_pos,
                    port_radius,
                    egui::Color32::from_gray(20) // Dark background
                );
                
                ui.painter().circle_filled(
                    port_pos,
                    port_radius - 1.5,
                    port_color
                );

                // Port hover effect
                let port_rect = egui::Rect::from_center_size(
                    port_pos, egui::vec2(port_radius * 2.5, port_radius * 2.5)
                );
                let port_response = ui.allocate_rect(port_rect, egui::Sense::click());
                
                if port_response.hovered() {
                    ui.painter().circle_stroke(
                        port_pos,
                        port_radius + 2.0,
                        egui::Stroke::new(1.0, egui::Color32::WHITE)
                    );
                }
                
                // Handle port clicking for connections
                if port_response.clicked() {
                    if let Some((other_node, other_port, is_other_input)) = &self.connecting_port {
                        if *is_other_input { // Only connect if the other port is an input
                            // Store the request to connect - will be processed after the loop
                            connect_request = Some((node.id, output.id.clone(), *other_node, other_port.clone()));
                        }
                        self.connecting_port = None;
                    } else {
                        // Start a new connection from this output
                        self.connecting_port = Some((node.id, output.id.clone(), false));
                    }
                }

                // Draw port label with improved styling
                ui.painter().text(
                    egui::pos2(port_pos.x - 15.0 * self.zoom, port_pos.y),
                    egui::Align2::RIGHT_CENTER,
                    &output.label,
                    egui::FontId::proportional(12.0 * self.zoom),
                    egui::Color32::from_rgb(229, 231, 235) // gray-200 - brighter text
                );

                output_y += 25.0 * self.zoom;
            }

            // Display parameters if node is active
            if is_active {
                let mut param_y = input_y.max(output_y) + 10.0 * self.zoom;
                
                // Parameters section title
                ui.painter().text(
                    egui::pos2(node_rect.center().x, param_y - 5.0 * self.zoom),
                    egui::Align2::CENTER_CENTER,
                    "Параметры",
                    egui::FontId::proportional(12.0 * self.zoom),
                    egui::Color32::from_rgb(209, 213, 219) // gray-300
                );
                
                param_y += 15.0 * self.zoom;
                
                // Show parameters for active node
                for param in &node.params {
                    let param_text = format!("{}: ", param.label);
                    
                    // Display parameter label
                    ui.painter().text(
                        egui::pos2(node_rect.min.x + 10.0 * self.zoom, param_y),
                        egui::Align2::LEFT_CENTER,
                        &param_text,
                        egui::FontId::proportional(12.0 * self.zoom),
                        egui::Color32::from_rgb(209, 213, 219) // gray-300 - brighter label
                    );
                    
                    // Display parameter value
                    let value_text = match &param.value {
                        ParameterValue::Text(text) => text.clone(),
                        ParameterValue::Number(num) => num.to_string(),
                        ParameterValue::Boolean(b) => b.to_string(),
                        ParameterValue::Select(selected, _) => selected.clone(),
                    };
                    
                    // Add a subtle background for parameter values
                    let value_width = 100.0 * self.zoom; // Fixed width for consistent look
                    let value_height = 18.0 * self.zoom;
                    let value_rect = egui::Rect::from_min_size(
                        egui::pos2(node_rect.max.x - 10.0 * self.zoom - value_width, param_y - value_height/2.0),
                        egui::vec2(value_width, value_height)
                    );
                    
                    ui.painter().rect_filled(
                        value_rect,
                        4.0, // Rounded corners
                        egui::Color32::from_gray(60) // Subtle background
                    );
                    
                    ui.painter().text(
                        value_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        if value_text.chars().count() > 20 {
                            let truncated: String = value_text.chars().take(17).collect();
                            format!("{}...", truncated)
                        } else {
                            value_text
                        },
                        egui::FontId::proportional(12.0 * self.zoom),
                        egui::Color32::WHITE
                    );
                    
                    param_y += 25.0 * self.zoom;
                }
                
                // Update node size if needed to fit all parameters
                let min_height = (param_y - node_rect.min.y + 10.0 * self.zoom) / self.zoom;
                if min_height > node.size.y {
                    node.size.y = min_height;
                }
            }
        }

        // Second pass: handle context menus
        for (node_id, node_response) in node_responses {
            // Handle right-click on node
            node_response.context_menu(|ui| {
                self.active_node = Some(node_id);
                self.context_menu_open = true;
                
                ui.set_min_width(150.0);
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 8.0);
                ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 55, 72);
                ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(59, 130, 246);
                ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgb(29, 78, 216);
                
                ui.add_space(4.0);
                
                if ui.button(egui::RichText::new("Удалить узел").size(14.0).color(egui::Color32::WHITE)).clicked() {
                    // Set the active node to be edited
                    ui.memory_mut(|mem| {
                        mem.data.insert_temp(egui::Id::new("node_to_delete"), Some(node_id));
                    });
                    ui.close_menu();
                }
                
                ui.add_space(2.0);
                
                if ui.button(egui::RichText::new("Копировать узел").size(14.0).color(egui::Color32::WHITE)).clicked() {
                    // Find the node and copy it to clipboard
                    if let Some(node_pos) = self.nodes.iter().position(|n| n.id == node_id) {
                        let node = self.nodes[node_pos].clone();
                        self.clipboard = Some(node);
                    }
                    ui.close_menu();
                }
                
                ui.add_space(2.0);
                
                if ui.button(egui::RichText::new("Редактировать параметры").size(14.0).color(egui::Color32::WHITE)).clicked() {
                    // Set the active node to be edited
                    self.editing_node = Some(node_id);
                    ui.memory_mut(|mem| {
                        mem.data.insert_temp(egui::Id::new("just_started_editing"), true);
                    });
                    ui.close_menu();
                }
                
                ui.add_space(4.0);
            });
        }

        // Check for node deletion after the loops
        if let Some(Some(node_id)) = ui.memory().data.get_temp::<Option<usize>>(egui::Id::new("node_to_delete")) {
            self.delete_node(*node_id);
            ui.memory_mut(|mem| mem.data.remove::<Option<usize>>(egui::Id::new("node_to_delete")));
        }

        // Save state if needed after we're done with node iteration
        if any_node_drag_started && !self.history_action_in_progress {
            self.save_state();
            self.history_action_in_progress = true;
        }
        
        // If we released a drag, end the action
        if response.drag_released() {
            self.history_action_in_progress = false;
        }

        // Process any connection requests that were created during the loop
        if let Some((from_node, from_port, to_node, to_port)) = connect_request {
            self.connect_nodes(from_node, &from_port, to_node, &to_port);
        }

        // Reset pending connection if clicked outside
        if response.clicked() && !self.context_menu_open {
            self.connecting_port = None;
        }
        
        self.context_menu_open = false;

        // Bring clicked node to front
        if let Some(idx) = clicked_node_idx {
            let node = self.nodes.remove(idx);
            self.nodes.push(node);
        }

        // Handle context menus after node drawing to avoid borrow issues
        // Handle editing of a node's parameters if one is selected
        let node_done_editing = self.draw_parameter_editor(ui.ctx());
        
        // Apply the state change outside the borrow context
        if node_done_editing {
            self.editing_node = None;
        }

        // Handle delayed node deletion
        self.process_delayed_deletion(ui.ctx());
    }

    fn draw_connection(&self, ui: &mut egui::Ui, connection: &Connection) {
        // Find the source and target nodes to get positions
        let mut from_pos = egui::Pos2::ZERO;
        let mut to_pos = egui::Pos2::ZERO;
        let mut from_color = egui::Color32::WHITE;
        let mut to_color = egui::Color32::WHITE;
        
        for node in &self.nodes {
            if node.id == connection.from_node {
                // Find the specific output port
                for (i, output) in node.outputs.iter().enumerate() {
                    if output.id == connection.from_port {
                        from_pos = egui::pos2(
                            node.position.x * self.zoom + node.size.x * self.zoom,
                            node.position.y * self.zoom + 45.0 * self.zoom + i as f32 * 25.0 * self.zoom
                        ) + self.drag_offset;
                        
                        // Update color based on port type
                        from_color = match output.port_type {
                            PortType::String => egui::Color32::from_rgb(96, 165, 250), // blue-400
                            PortType::Number => egui::Color32::from_rgb(167, 139, 250), // purple-400
                            PortType::Object => egui::Color32::from_rgb(251, 146, 60), // orange-400
                            PortType::Action => egui::Color32::from_rgb(74, 222, 128), // green-400
                        };
                        break;
                    }
                }
            }
            
            if node.id == connection.to_node {
                // Find the specific input port
                for (i, input) in node.inputs.iter().enumerate() {
                    if input.id == connection.to_port {
                        to_pos = egui::pos2(
                            node.position.x * self.zoom,
                            node.position.y * self.zoom + 45.0 * self.zoom + i as f32 * 25.0 * self.zoom
                        ) + self.drag_offset;
                        
                        // Update color based on port type
                        to_color = match input.port_type {
                            PortType::String => egui::Color32::from_rgb(96, 165, 250), // blue-400
                            PortType::Number => egui::Color32::from_rgb(167, 139, 250), // purple-400
                            PortType::Object => egui::Color32::from_rgb(251, 146, 60), // orange-400
                            PortType::Action => egui::Color32::from_rgb(74, 222, 128), // green-400
                        };
                        break;
                    }
                }
            }
        }
        
        // Draw bezier curve if both ports were found
        if from_pos != egui::Pos2::ZERO && to_pos != egui::Pos2::ZERO {
            // Use gradient color from source to target - fix the type conversion
            let gradient_color = egui::Color32::from_rgba_premultiplied(
                ((from_color.r() as u16 + to_color.r() as u16) / 2) as u8,
                ((from_color.g() as u16 + to_color.g() as u16) / 2) as u8,
                ((from_color.b() as u16 + to_color.b() as u16) / 2) as u8,
                255
            );
            
            self.draw_bezier_connection(ui, from_pos, to_pos, gradient_color);
        }
    }
    
    fn draw_bezier_connection(&self, ui: &mut egui::Ui, from: egui::Pos2, to: egui::Pos2, color: egui::Color32) {
        let distance = (to.x - from.x).abs().max(30.0);
        let control1 = egui::pos2(from.x + distance / 2.0, from.y);
        let control2 = egui::pos2(to.x - distance / 2.0, to.y);
        
        let num_segments = 20; // Increase segments for smoother curves
        let mut last_point = from;
        
        // Draw shadow first for depth effect
        for i in 1..=num_segments {
            let t = i as f32 / num_segments as f32;
            let point = self.cubic_bezier(from, control1, control2, to, t);
            ui.painter().line_segment(
                [last_point, point],
                egui::Stroke::new(3.0, egui::Color32::from_black_alpha(40)) // Shadow
            );
            last_point = point;
        }
        
        // Draw main line with glow effect
        last_point = from;
        for i in 1..=num_segments {
            let t = i as f32 / num_segments as f32;
            let point = self.cubic_bezier(from, control1, control2, to, t);
            ui.painter().line_segment(
                [last_point, point],
                egui::Stroke::new(2.0, color)
            );
            last_point = point;
        }
        
        // Draw small circles at both ends for connection points
        ui.painter().circle_filled(from, 4.0, color);
        ui.painter().circle_filled(to, 4.0, color);
    }

    fn draw_grid(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let grid_size = 20.0;
        // Use a more subtle grid color
        let minor_grid_color = egui::Color32::from_rgb(38, 38, 48); 
        let major_grid_color = egui::Color32::from_rgb(48, 48, 58);
        
        let offset_x = self.drag_offset.x % grid_size;
        let offset_y = self.drag_offset.y % grid_size;
        
        // Minor grid lines 
        for x in (offset_x as i32..(rect.width() as i32)).step_by(grid_size as usize) {
            ui.painter().line_segment(
                [
                    egui::pos2(rect.left() + x as f32, rect.top()),
                    egui::pos2(rect.left() + x as f32, rect.bottom())
                ],
                egui::Stroke::new(1.0, minor_grid_color)
            );
        }
        
        for y in (offset_y as i32..(rect.height() as i32)).step_by(grid_size as usize) {
            ui.painter().line_segment(
                [
                    egui::pos2(rect.left(), rect.top() + y as f32),
                    egui::pos2(rect.right(), rect.top() + y as f32)
                ],
                egui::Stroke::new(1.0, minor_grid_color)
            );
        }
        
        // Major grid lines (every 5 minor lines)
        let major_grid_size = grid_size * 5.0;
        let major_offset_x = self.drag_offset.x % major_grid_size;
        let major_offset_y = self.drag_offset.y % major_grid_size;
        
        for x in (major_offset_x as i32..(rect.width() as i32)).step_by(major_grid_size as usize) {
            ui.painter().line_segment(
                [
                    egui::pos2(rect.left() + x as f32, rect.top()),
                    egui::pos2(rect.left() + x as f32, rect.bottom())
                ],
                egui::Stroke::new(1.0, major_grid_color)
            );
        }
        
        for y in (major_offset_y as i32..(rect.height() as i32)).step_by(major_grid_size as usize) {
            ui.painter().line_segment(
                [
                    egui::pos2(rect.left(), rect.top() + y as f32),
                    egui::pos2(rect.right(), rect.top() + y as f32)
                ],
                egui::Stroke::new(1.0, major_grid_color)
            );
        }
    }

    // Cubic Bezier interpolation
    fn cubic_bezier(&self, p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, p3: egui::Pos2, t: f32) -> egui::Pos2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        egui::pos2(
            p0.x * mt3 + 3.0 * p1.x * mt2 * t + 3.0 * p2.x * mt * t2 + p3.x * t3,
            p0.y * mt3 + 3.0 * p1.y * mt2 * t + 3.0 * p2.y * mt * t2 + p3.y * t3
        )
    }

    // Helper method to get port position
    fn get_port_position(&self, node_id: usize, port_id: &str, is_input: bool) -> Option<egui::Pos2> {
        for node in &self.nodes {
            if node.id == node_id {
                if is_input {
                    for (i, input) in node.inputs.iter().enumerate() {
                        if input.id == port_id {
                            return Some(egui::pos2(
                                node.position.x * self.zoom + self.drag_offset.x,
                                node.position.y * self.zoom + 45.0 * self.zoom + i as f32 * 25.0 * self.zoom + self.drag_offset.y
                            ));
                        }
                    }
                } else {
                    for (i, output) in node.outputs.iter().enumerate() {
                        if output.id == port_id {
                            return Some(egui::pos2(
                                node.position.x * self.zoom + node.size.x * self.zoom + self.drag_offset.x,
                                node.position.y * self.zoom + 45.0 * self.zoom + i as f32 * 25.0 * self.zoom + self.drag_offset.y
                            ));
                        }
                    }
                }
            }
        }
        None
    }

    // Add a new method to handle delayed node deletion
    fn process_delayed_deletion(&mut self, ctx: &egui::Context) {
        if let Some(Some(node_id)) = ctx.memory_mut(|mem| mem.data.get_temp::<Option<usize>>(egui::Id::new("node_to_delete"))) {
            self.delete_node(node_id);
            // Clear the temporary data
            ctx.memory_mut(|mem| mem.data.insert_temp(egui::Id::new("node_to_delete"), Option::<usize>::None));
        }
    }

    // Handle editing of a node's parameters
    fn draw_parameter_editor(&mut self, ctx: &egui::Context) -> bool {
        let mut node_done_editing = false;
        let mut param_changes = false;
        let mut node_index_to_update = None;
        let mut node_title_to_apply = String::new();
        let mut param_values_to_apply: Vec<ParameterValue> = Vec::new();
        let mut should_save_state = false;
        
        if let Some(node_id) = self.editing_node {
            // Find the node to edit
            if let Some(node_index) = self.nodes.iter().position(|node| node.id == node_id) {
                node_index_to_update = Some(node_index);
                
                // Get node data
                let node = &self.nodes[node_index];
                let mut node_title = node.title.clone();
                
                // Prepare mutable copies of parameter values
                let mut param_values: Vec<ParameterValue> = node.params.iter().map(|p| p.value.clone()).collect();
                
                // Draw a modal window
                let screen_rect = ctx.input(|i| i.screen_rect());
                let modal_rect = egui::Rect::from_center_size(
                    screen_rect.center(), 
                    egui::vec2(400.0, 500.0)
                );
                
                let mut should_save = false;
                
                egui::Window::new(format!("Редактирование параметров: {}", node_title))
                    .fixed_rect(modal_rect)
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Параметры узла");
                        });
                        ui.add_space(10.0);
                        
                        // Add a scrolling area for parameters
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // Edit node title
                            ui.horizontal(|ui| {
                                ui.label("Название:");
                                if ui.text_edit_singleline(&mut node_title).changed() {
                                    param_changes = true;
                                }
                            });
                            
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(5.0);
                            
                            // Edit parameters
                            for (param_idx, param) in node.params.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(&param.label);
                                    
                                    match &mut param_values[param_idx] {
                                        ParameterValue::Text(text) => {
                                            if param.label.contains("текст") || param.label == "Ответ" {
                                                // For longer texts, use multiline
                                                if ui.text_edit_multiline(text).changed() {
                                                    param_changes = true;
                                                }
                                            } else {
                                                // For shorter texts like names, use singleline
                                                if ui.text_edit_singleline(text).changed() {
                                                    param_changes = true;
                                                }
                                            }
                                        },
                                        ParameterValue::Number(num) => {
                                            let mut num_text = num.to_string();
                                            if ui.text_edit_singleline(&mut num_text).changed() {
                                                if let Ok(new_num) = num_text.parse::<f64>() {
                                                    *num = new_num;
                                                    param_changes = true;
                                                }
                                            }
                                        },
                                        ParameterValue::Boolean(bool_val) => {
                                            if ui.checkbox(bool_val, "").changed() {
                                                param_changes = true;
                                            }
                                        },
                                        ParameterValue::Select(selected, options) => {
                                            // Use a dropdown with options
                                            let current_text = selected.clone();
                                            egui::ComboBox::from_id_source(&param.id)
                                                .selected_text(current_text)
                                                .show_ui(ui, |ui| {
                                                    for option in options.iter() {
                                                        if ui.selectable_label(selected == option, option.clone()).clicked() {
                                                            *selected = option.clone();
                                                            param_changes = true;
                                                        }
                                                    }
                                                });
                                        },
                                    }
                                });
                                ui.add_space(5.0);
                            }
                        });
                        
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                        
                        // Buttons
                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Сохранить").clicked() {
                                    // Mark for saving changes
                                    should_save = true;
                                    node_done_editing = true;
                                }
                                
                                ui.add_space(10.0);
                                
                                if ui.button("Отмена").clicked() {
                                    // Discard changes
                                    node_done_editing = true;
                                }
                            });
                        });
                    });
                
                // Store values for applying outside the borrow
                if should_save && param_changes {
                    should_save_state = true;
                    node_title_to_apply = node_title;
                    param_values_to_apply = param_values;
                }
            }
        }
        
        // Apply changes outside the borrow
        if should_save_state && node_index_to_update.is_some() {
            let node_index = node_index_to_update.unwrap();
            
            // Save state before making changes
            self.save_state();
            
            // Apply changes to the actual node
            let node = &mut self.nodes[node_index];
            node.title = node_title_to_apply;
            
            // Update parameter values
            for (param_idx, param) in node.params.iter_mut().enumerate() {
                param.value = param_values_to_apply[param_idx].clone();
            }
        }
        
        node_done_editing
    }

    // Create a snapshot of the current state
    fn create_snapshot(&self) -> GraphSnapshot {
        GraphSnapshot {
            nodes: self.nodes.clone(),
            connections: self.connections.clone(),
            active_node: self.active_node,
            drag_offset: self.drag_offset,
            zoom: self.zoom,
        }
    }

    // Save the current state to history
    pub fn save_state(&mut self) {
        // Don't save if we're in the middle of an action that will create its own history
        if self.history_action_in_progress {
            return;
        }

        // Create a snapshot of the current state
        let snapshot = self.create_snapshot();
        
        // Add to history
        self.history.push_back(snapshot);
        
        // Clear redo stack since we've created a new history branch
        self.redo_stack.clear();
        
        // Limit history size
        while self.history.len() > self.history_size_limit {
            self.history.pop_front();
        }
    }

    // Restore a state from a snapshot
    fn restore_from_snapshot(&mut self, snapshot: GraphSnapshot) {
        self.nodes = snapshot.nodes;
        self.connections = snapshot.connections;
        self.active_node = snapshot.active_node;
        self.drag_offset = snapshot.drag_offset;
        self.zoom = snapshot.zoom;
    }

    // Undo the last action
    pub fn undo(&mut self) -> bool {
        // Save current state to redo stack before undoing
        if let Some(current_snapshot) = self.history.pop_back() {
            // First check if we have any history left
            if self.history.is_empty() {
                // Put back the current snapshot since there's nothing to undo to
                self.history.push_back(current_snapshot);
                return false;
            }

            // Save current state to redo stack
            self.redo_stack.push_back(current_snapshot);
            
            // Get the previous state
            if let Some(previous_snapshot) = self.history.back().cloned() {
                self.history_action_in_progress = true;
                
                // Restore the previous state
                self.restore_from_snapshot(previous_snapshot);
                
                self.history_action_in_progress = false;
                return true;
            }
        }
        
        false
    }

    // Redo the last undone action
    pub fn redo(&mut self) -> bool {
        if let Some(redo_snapshot) = self.redo_stack.pop_back() {
            self.history_action_in_progress = true;
            
            // Save current state to history
            let current_snapshot = self.create_snapshot();
            self.history.push_back(current_snapshot);
            
            // Restore the redo state
            self.restore_from_snapshot(redo_snapshot);
            
            self.history_action_in_progress = false;
            return true;
        }
        
        false
    }

    // Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.history.len() > 1 // Need at least 2 items to undo (current + previous)
    }

    // Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    // Add method to open parameter edit dialog for a node
    pub fn edit_node_parameters(&mut self, node_id: usize, ctx: &egui::Context) {
        self.editing_node = Some(node_id);
        ctx.memory_mut(|mem| mem.data.insert_temp(egui::Id::new("just_started_editing"), true));
    }

    // Get root menu nodes (nodes with no input connections that are menu items)
    pub fn get_root_menu_nodes(&self) -> Vec<usize> {
        self.nodes.iter()
            .enumerate()
            .filter(|(id, node)| {
                // Check if node is a menu item type
                let node_type = node.node_type.as_str();
                node_type == "menu_item" && 
                // Check if node has no input connections (is a root node)
                !self.connections.iter().any(|conn| conn.to_node == *id)
            })
            .map(|(id, _)| id)
            .collect()
    }

    // Get FAQ nodes
    pub fn get_faq_nodes(&self) -> Vec<usize> {
        self.nodes.iter()
            .enumerate()
            .filter(|(_, node)| node.node_type.as_str() == "faq_item")
            .map(|(id, _)| id)
            .collect()
    }

    // Get child menu nodes for a given parent node
    pub fn get_child_menu_nodes(&self, parent_id: usize) -> Vec<usize> {
        self.connections.iter()
            .filter(|conn| conn.from_node == parent_id)
            .map(|conn| conn.to_node)
            .filter(|&to_id| {
                // Ensure the child is a menu item
                if let Some(node) = self.nodes.get(to_id) {
                    node.node_type.as_str() == "menu_item"
                } else {
                    false
                }
            })
            .collect()
    }

    // Get document nodes connected to a menu item
    pub fn get_documents_for_node(&self, menu_id: usize) -> Vec<usize> {
        self.connections.iter()
            .filter(|conn| conn.from_node == menu_id)
            .map(|conn| conn.to_node)
            .filter(|&to_id| {
                // Ensure the child is a document
                if let Some(node) = self.nodes.get(to_id) {
                    node.node_type.as_str() == "document"
                } else {
                    false
                }
            })
            .collect()
    }

    // Get node data by id
    pub fn get_node_data(&self, node_id: usize) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == node_id)
    }

    // Add a method to create a document node
    pub fn add_document(&mut self, position: egui::Pos2, title: String) -> Result<usize, String> {
        // Save state before making changes
        self.save_state();

        let id = self.next_node_id();
        self.nodes.push(Node {
            id,
            title,
            position,
            size: egui::vec2(180.0, 100.0),
            color: egui::Color32::from_rgb(150, 150, 150),
            node_type: NodeType::Document,
            inputs: vec![
                Port {
                    id: "parent_menu".to_string(),
                    label: "Родительское меню".to_string(),
                    port_type: PortType::Object,
                    color: egui::Color32::from_rgb(150, 200, 255),
                }
            ],
            outputs: vec![],
            params: vec![
                Parameter {
                    id: "text".to_string(),
                    label: "Название".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text(title.clone()),
                },
                Parameter {
                    id: "callback_data".to_string(),
                    label: "Callback Data".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("".to_string()),
                },
                Parameter {
                    id: "url".to_string(),
                    label: "URL".to_string(),
                    param_type: ParameterType::Text,
                    value: ParameterValue::Text("".to_string()),
                },
            ],
        });
        
        Ok(id)
    }

    // Add a static method to check if document creation is supported
    pub fn supports_document_creation() -> bool {
        true
    }
}

impl NodeType {
    // Get string representation of node type
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::MenuItem => "menu_item",
            NodeType::FaqItem => "faq_item",
            NodeType::Process => "process",
            NodeType::Input => "input",
            NodeType::Output => "output",
            NodeType::Document => "document",
        }
    }
}

// For Parameter implementation
impl Parameter {
    // Get text value from parameter
    pub fn get_text(&self) -> String {
        match &self.value {
            ParameterValue::Text(text) => text.clone(),
            ParameterValue::Number(num) => num.to_string(),
            ParameterValue::Boolean(b) => b.to_string(),
            ParameterValue::Select(selected, _) => selected.clone(),
        }
    }

    // Get the parameter kind/id
    pub fn kind(&self) -> &str {
        &self.id
    }
}

// Add these methods to the Node impl
impl Node {
    pub fn get_title(&self) -> &str {
        &self.title
    }
    
    pub fn get_params(&self) -> &Vec<Parameter> {
        &self.params
    }
} 