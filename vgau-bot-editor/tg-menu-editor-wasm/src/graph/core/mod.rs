use eframe::egui;

use crate::graph::models::{Node, NodeType, Connection};
use crate::graph::history::{HistoryManager, GraphSnapshot};
use crate::graph::rendering::GraphRenderer;
use crate::graph::operations::GraphOperations;

/// The main node graph that manages nodes and connections
pub struct SimpleNodeGraph {
    /// All nodes in the graph
    nodes: Vec<Node>,
    /// All connections between nodes
    connections: Vec<Connection>,
    /// Currently active (selected) node
    pub active_node: Option<usize>,
    /// Offset for panning the graph
    drag_offset: egui::Vec2,
    /// Zoom level of the graph
    zoom: f32,
    /// Node that has been copied to clipboard
    clipboard: Option<Node>,
    /// Port that is currently being connected (node_id, port_id, is_input)
    connecting_port: Option<(usize, String, bool)>,
    /// Whether a context menu is currently open
    context_menu_open: bool,
    /// Node that is currently being edited in a parameters dialog
    editing_node: Option<usize>,
    /// History manager for undo/redo operations
    history_manager: HistoryManager,
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
            editing_node: None,
            history_manager: HistoryManager::new(30), // Default history limit of 30 snapshots
        }
    }
}

impl SimpleNodeGraph {
    /// Create a new graph with sample nodes
    pub fn new() -> Self {
        let mut graph = Self::default();
        graph.initialize_sample_nodes();
        
        // Create initial history snapshot
        graph.initialize_history();
        
        graph
    }

    /// Initialize sample nodes for a new graph
    fn initialize_sample_nodes(&mut self) {
        // Create initial menu node
        let main_menu_id = self.add_menu_item(
            egui::pos2(100.0, 100.0),
            "Главное меню".to_string()
        );
        
        // Create submenu node
        let submenu_id = self.add_menu_item(
            egui::pos2(400.0, 100.0),
            "Подменю: О университете".to_string()
        );
        
        // Connect main menu to submenu
        self.connect_nodes(
            main_menu_id, "sub_menu",
            submenu_id, "parent_menu"
        );
        
        // Create a FAQ item
        self.add_faq_item(
            egui::pos2(100.0, 300.0),
            "Часто задаваемый вопрос".to_string()
        );
    }

    /// Initialize history system with the current state
    fn initialize_history(&mut self) {
        let snapshot = self.create_snapshot();
        self.history_manager.initialize(snapshot);
    }

    /// Add a menu item node at the specified position
    pub fn add_menu_item(&mut self, position: egui::Pos2, title: String) -> usize {
        let id = self.next_node_id();
        let node = GraphOperations::create_menu_item(id, title, position);
        
        self.nodes.push(node);
        self.save_state();
        
        id
    }

    /// Add an FAQ item node at the specified position
    pub fn add_faq_item(&mut self, position: egui::Pos2, title: String) -> usize {
        let id = self.next_node_id();
        let node = GraphOperations::create_faq_item(id, title, position);
        
        self.nodes.push(node);
        self.save_state();
        
        id
    }

    /// Connect two nodes if the connection is valid
    pub fn connect_nodes(&mut self, from_node: usize, from_port: &str, to_node: usize, to_port: &str) -> bool {
        // Don't connect a node to itself
        if from_node == to_node {
            return false;
        }
        
        // Check if the nodes exist
        let from_node_data = match self.nodes.iter().find(|n| n.id() == from_node) {
            Some(node) => node,
            None => return false,
        };
        
        let to_node_data = match self.nodes.iter().find(|n| n.id() == to_node) {
            Some(node) => node,
            None => return false,
        };
        
        // Check if the connection is valid
        if !GraphOperations::is_valid_connection(from_node_data, from_port, to_node_data, to_port) {
            return false;
        }
        
        // Check if the connection already exists
        if self.connections.iter().any(|c| {
            c.from_node() == from_node && c.to_node() == to_node &&
            c.from_port() == from_port && c.to_port() == to_port
        }) {
            return false;
        }
        
        // Create the connection
        let connection = GraphOperations::create_connection(
            from_node,
            from_port.to_string(),
            to_node,
            to_port.to_string()
        );
        
        self.connections.push(connection);
        self.save_state();
        
        true
    }

    /// Delete a node and all its connections
    pub fn delete_node(&mut self, node_id: usize) {
        // Remove the node
        self.nodes.retain(|node| node.id() != node_id);
        
        // Remove all connections to/from the node
        self.connections.retain(|conn| {
            conn.from_node() != node_id && conn.to_node() != node_id
        });
        
        // Update active node if the deleted node was active
        if let Some(active) = self.active_node {
            if active == node_id {
                self.active_node = None;
            }
        }
        
        self.save_state();
    }

    /// Delete a connection between two nodes
    pub fn delete_connection(&mut self, from_node: usize, from_port: &str, to_node: usize, to_port: &str) {
        self.connections.retain(|conn| {
            !(conn.from_node() == from_node && 
              conn.to_node() == to_node && 
              conn.from_port() == from_port && 
              conn.to_port() == to_port)
        });
        
        self.save_state();
    }

    /// Get the next available node ID
    fn next_node_id(&self) -> usize {
        self.nodes.iter().map(|n| n.id()).max().unwrap_or(0) + 1
    }

    /// Draw the graph
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        // Calculate available space
        let available_rect = ui.available_rect_before_wrap();
        
        // Draw grid in the background
        GraphRenderer::draw_grid(ui, available_rect, self.zoom);
        
        // Store zoom level in UI memory for node rendering
        ui.memory_mut(|mem| mem.data.insert_temp(egui::Id::new("graph_zoom"), self.zoom));
        
        // For graph background - capture the response for both drag and context menu
        let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());
        
        // Show a grab cursor when hovering to indicate panning is available
        if response.hovered() && !self.context_menu_open && ui.input(|i| !i.pointer.any_down()) {
            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
        }
        
        // Handle middle-button dragging for panning
        if ui.input(|i| i.pointer.middle_down()) {
            let delta = response.drag_delta();
            // Update drag offset for panning the graph
            self.drag_offset += delta;
            
            // Apply pan offset to all nodes
            for node in &mut self.nodes {
                node.position += delta;
            }
            
            // Show a grabbing cursor during panning
            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
        } else if ui.input(|i| i.pointer.button_released(egui::PointerButton::Middle)) {
            // Save the state after panning is completed
            self.save_state();
        }
        
        // Handle zooming with mouse wheel
        let scroll_delta = ui.input(|i| i.scroll_delta.y);
        if scroll_delta != 0.0 {
            // Calculate zoom factor based on scroll delta
            let zoom_factor = if scroll_delta > 0.0 {
                1.1 // Zoom in
            } else {
                0.9 // Zoom out
            };
            
            // Get mouse position before zoom
            let mouse_pos = ui.input(|i| i.pointer.interact_pos()).unwrap_or(egui::pos2(0.0, 0.0));
            
            // Update zoom level
            let old_zoom = self.zoom;
            self.zoom = (self.zoom * zoom_factor).clamp(0.1, 5.0); // Limit zoom range
            
            // Calculate how much the zoom changed
            let zoom_change = self.zoom / old_zoom;
            
            // Adjust node positions to zoom around mouse cursor
            for node in &mut self.nodes {
                // Calculate vector from mouse to node
                let node_to_mouse = mouse_pos - node.position;
                // Scale this vector by the zoom change
                let scaled_vector = node_to_mouse * (1.0 - zoom_change);
                // Apply the scaled vector to the node position
                node.position += scaled_vector;
            }
            
            // Save state after zooming
            self.save_state();
        }
        
        // Handle context menu
        response.context_menu(|ui| {
            // First ensure the context menu flag is set
            self.context_menu_open = true;
            
            ui.set_min_width(150.0);
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 8.0);
            ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 55, 72);
            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(59, 130, 246);
            ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgb(29, 78, 216);
            
            // Make the active node None when clicking on the background
            self.active_node = None;
            
            ui.add_space(4.0);
            
            if ui.button(egui::RichText::new("Добавить пункт меню").size(14.0).color(egui::Color32::WHITE)).clicked() {
                let pointer_pos = ui.input(|i| i.pointer.interact_pos()).unwrap_or(egui::pos2(100.0, 100.0));
                self.add_menu_item(pointer_pos, "Новый пункт меню".to_string());
                ui.close_menu();
            }
            
            ui.add_space(2.0);
            
            if ui.button(egui::RichText::new("Добавить FAQ").size(14.0).color(egui::Color32::WHITE)).clicked() {
                let pointer_pos = ui.input(|i| i.pointer.interact_pos()).unwrap_or(egui::pos2(100.0, 100.0));
                self.add_faq_item(pointer_pos, "Новый FAQ".to_string());
                ui.close_menu();
            }
            
            if let Some(node) = &self.clipboard {
                ui.add_space(2.0);
                
                if ui.button(egui::RichText::new("Вставить узел").size(14.0).color(egui::Color32::WHITE)).clicked() {
                    let pointer_pos = ui.input(|i| i.pointer.interact_pos()).unwrap_or(egui::pos2(100.0, 100.0));
                    
                    let mut new_node = node.clone();
                    new_node.id = self.next_node_id();
                    new_node.position = pointer_pos;
                    
                    self.nodes.push(new_node);
                    self.save_state();
                    
                    ui.close_menu();
                }
            }
            
            ui.add_space(4.0);
        });
        
        // Draw connections
        for connection in &self.connections {
            GraphRenderer::draw_connection(ui, connection, &self.nodes);
        }
        
        // Draw pending connection if any
        if let Some((from_node_id, ref from_port, is_input)) = self.connecting_port {
            // Find the node
            if let Some(from_node) = self.nodes.iter().find(|n| n.id() == from_node_id) {
                // Get port position
                if let Some(from_pos) = GraphRenderer::get_port_position(from_node, from_port, is_input) {
                    // Get current mouse position for the end point
                    let to_pos = ui.input(|i| i.pointer.interact_pos()).unwrap_or(from_pos);
                    
                    // Default color for pending connection
                    let mut connection_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 128);
                    
                    // Check if hovering over a compatible port
                    let mut _hovering_compatible_port = false;
                    
                    // Find the port we're from
                    let from_port_data = if is_input {
                        from_node.inputs.iter().find(|p| p.id() == from_port)
                    } else {
                        from_node.outputs.iter().find(|p| p.id() == from_port)
                    };
                    
                    // Check if mouse is over any node
                    for node in &self.nodes {
                        if node.id() == from_node_id {
                            continue; // Skip the node we're connecting from
                        }
                        
                        // Get node rect
                        let node_rect = egui::Rect::from_min_size(node.position, node.size);
                        
                        // If mouse is near this node, check its ports
                        if node_rect.distance_to_pos(to_pos) < 50.0 {
                            // Get potential target ports (inputs if we're from an output and vice versa)
                            let target_ports = if is_input {
                                // We're dragging from an input, so look for outputs
                                &node.outputs
                            } else {
                                // We're dragging from an output, so look for inputs
                                &node.inputs
                            };
                            
                            // Check each potential port
                            for port in target_ports {
                                let port_pos = GraphRenderer::get_port_position(
                                    node, 
                                    port.id(), 
                                    !is_input // opposite of what we're dragging from
                                );
                                
                                if let Some(port_pos) = port_pos {
                                    // If mouse is near this port
                                    if (port_pos - to_pos).length() < 20.0 {
                                        // Check if port types are compatible
                                        if let Some(from_port_data) = from_port_data {
                                            if from_port_data.port_type() == port.port_type() {
                                                // Compatible port - green connection
                                                connection_color = egui::Color32::from_rgba_unmultiplied(100, 255, 100, 200);
                                                _hovering_compatible_port = true;
                                            } else {
                                                // Incompatible port type - red connection
                                                connection_color = egui::Color32::from_rgba_unmultiplied(255, 100, 100, 200);
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    
                    // Draw pending connection with appropriate color
                    GraphRenderer::draw_bezier_connection(
                        ui,
                        if is_input { to_pos } else { from_pos },
                        if is_input { from_pos } else { to_pos },
                        connection_color
                    );
                }
            }
        }
        
        // Store nodes to be moved
        let mut nodes_to_move = Vec::new();
        let mut nodes_to_handle_context = Vec::new();
        
        // Create a clone of nodes to avoid borrowing issues
        let nodes_clone = self.nodes.clone();
        
        // Draw nodes
        for node in &nodes_clone {
            let is_active = self.active_node.map_or(false, |id| id == node.id());
            let mut response = None;
            
            GraphRenderer::draw_node(ui, node, is_active, &mut response);
            
            // Handle node interaction
            if let Some(response) = response {
                if response.clicked() {
                    self.active_node = Some(node.id());
                }
                
                if response.dragged() {
                    // Move the node
                    if let Some(active_id) = self.active_node {
                        if active_id == node.id() {
                            let delta = response.drag_delta();
                            nodes_to_move.push((node.id(), delta));
                        }
                    }
                }
                
                // Store node id and response for context menu handling after loop
                nodes_to_handle_context.push((node.id(), response));
            }
        }
        
        // Apply node movements
        for (node_id, delta) in nodes_to_move {
            if let Some(node) = self.nodes.iter_mut().find(|n| n.id() == node_id) {
                node.position += delta;
            }
        }
        
        // Handle context menus for nodes outside the borrow
        for (node_id, response) in nodes_to_handle_context {
            response.context_menu(|ui| {
                self.context_menu_open = true;
                self.active_node = Some(node_id);
                
                ui.set_min_width(150.0);
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 8.0);
                ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 55, 72);
                ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(59, 130, 246);
                ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgb(29, 78, 216);
                
                ui.add_space(4.0);
                
                if ui.button(egui::RichText::new("Удалить узел").size(14.0).color(egui::Color32::WHITE)).clicked() {
                    // Store for later deletion to avoid borrow issues
                    ui.memory_mut(|mem| mem.data.insert_temp(egui::Id::new("node_to_delete"), node_id));
                    ui.close_menu();
                }
                
                ui.add_space(2.0);
                
                if ui.button(egui::RichText::new("Копировать узел").size(14.0).color(egui::Color32::WHITE)).clicked() {
                    // Lookup in original collection and clone
                    if let Some(found_node) = self.nodes.iter().find(|n| n.id() == node_id) {
                        self.clipboard = Some(found_node.clone());
                    }
                    ui.close_menu();
                }
                
                ui.add_space(2.0);
                
                if ui.button(egui::RichText::new("Редактировать параметры").size(14.0).color(egui::Color32::WHITE)).clicked() {
                    self.editing_node = Some(node_id);
                    ui.close_menu();
                }
                
                ui.add_space(4.0);
            });
        }
        
        // Check if any node needs to be deleted (from context menu action)
        if let Some(node_id) = ui.memory(|mem| mem.data.get_temp::<usize>(egui::Id::new("node_to_delete"))) {
            self.delete_node(node_id);
            ui.memory_mut(|mem| mem.data.remove::<usize>(egui::Id::new("node_to_delete")));
        }
        
        // Handle keyboard shortcuts
        ui.input(|i| {
            // Delete selected node with Delete key
            if i.key_pressed(egui::Key::Delete) {
                if let Some(node_id) = self.active_node {
                    self.delete_node(node_id);
                }
            }
        });
        
        // Reset context menu flag at the end of the draw call
        self.context_menu_open = false;
    }

    /// Create a snapshot of the current graph state
    fn create_snapshot(&self) -> GraphSnapshot {
        GraphSnapshot::new(
            self.nodes.clone(),
            self.connections.clone(),
            self.active_node,
            self.drag_offset,
            self.zoom
        )
    }

    /// Save the current state to history
    pub fn save_state(&mut self) {
        let snapshot = self.create_snapshot();
        self.history_manager.add_snapshot(snapshot);
    }

    /// Restore the graph from a snapshot
    fn restore_from_snapshot(&mut self, snapshot: GraphSnapshot) {
        self.nodes = snapshot.nodes;
        self.connections = snapshot.connections;
        self.active_node = snapshot.active_node;
        self.drag_offset = snapshot.drag_offset;
        self.zoom = snapshot.zoom;
    }

    /// Undo the last action
    pub fn undo(&mut self) -> bool {
        if let Some(snapshot) = self.history_manager.undo() {
            self.restore_from_snapshot(snapshot);
            true
        } else {
            false
        }
    }

    /// Redo the last undone action
    pub fn redo(&mut self) -> bool {
        if let Some(snapshot) = self.history_manager.redo() {
            self.restore_from_snapshot(snapshot);
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.history_manager.can_undo()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.history_manager.can_redo()
    }

    /// Get all root menu nodes (no parent connection)
    pub fn get_root_menu_nodes(&self) -> Vec<usize> {
        self.nodes.iter()
            .filter(|node| node.node_type == NodeType::MenuItem)
            .filter(|node| {
                // Check if this node is not connected as a child to any other node
                !self.connections.iter().any(|conn| {
                    conn.to_node() == node.id() && conn.to_port() == "parent_menu"
                })
            })
            .map(|node| node.id())
            .collect()
    }

    /// Get all FAQ nodes
    pub fn get_faq_nodes(&self) -> Vec<usize> {
        self.nodes.iter()
            .filter(|node| node.node_type == NodeType::FaqItem)
            .map(|node| node.id())
            .collect()
    }

    /// Get all child menu nodes for a parent node
    pub fn get_child_menu_nodes(&self, parent_id: usize) -> Vec<usize> {
        // Find all connections from parent's sub_menu output
        self.connections.iter()
            .filter(|conn| conn.from_node() == parent_id && conn.from_port() == "sub_menu")
            .filter_map(|conn| {
                // Find the connected node and check if it's a menu item
                let node_id = conn.to_node();
                if self.nodes.iter().any(|node| node.id() == node_id && node.node_type == NodeType::MenuItem) {
                    Some(node_id)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all document nodes connected to a menu node
    pub fn get_documents_for_node(&self, menu_id: usize) -> Vec<usize> {
        // Find all connections from menu's documents output
        self.connections.iter()
            .filter(|conn| conn.from_node() == menu_id && conn.from_port() == "documents")
            .map(|conn| conn.to_node())
            .collect()
    }

    /// Get the data for a node
    pub fn get_node_data(&self, node_id: usize) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id() == node_id)
    }

    /// Get a mutable reference to a node by its ID
    pub fn get_node_mut(&mut self, node_id: usize) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.id() == node_id)
    }

    /// Add a document node at the specified position
    pub fn add_document(&mut self, position: egui::Pos2, title: String) -> Result<usize, String> {
        // Check if we have document creation functionality
        if GraphOperations::supports_document_creation() {
            let id = self.next_node_id();
            let node = GraphOperations::create_document(id, title, position);
            
            self.nodes.push(node);
            self.save_state();
            
            Ok(id)
        } else {
            Err("Document node creation not supported in this version".to_string())
        }
    }
}