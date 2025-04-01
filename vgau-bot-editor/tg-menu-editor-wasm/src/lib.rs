use eframe::egui;
use std::collections::{HashMap, HashSet};

// ======== Data Models ========

/// Represents a node type in the telegram bot menu editor
#[derive(Clone, Copy, PartialEq, Debug)]
enum NodeType {
    MainMenu,
    Submenu,
    Button,
    TextContent,
    Image,
    Document,
    Link,
    FAQ,
    Contacts,
}

impl NodeType {
    fn get_color(&self) -> egui::Color32 {
        match self {
            // Higher contrast, more distinct colors for better visibility
            NodeType::MainMenu => egui::Color32::from_rgb(50, 100, 200),    // Stronger blue
            NodeType::Submenu => egui::Color32::from_rgb(70, 130, 220),     // Bright blue
            NodeType::Button => egui::Color32::from_rgb(60, 160, 60),       // Darker green
            NodeType::TextContent => egui::Color32::from_rgb(40, 140, 40),  // Strong green
            NodeType::Image => egui::Color32::from_rgb(200, 60, 60),        // Strong red
            NodeType::Document => egui::Color32::from_rgb(200, 180, 40),    // Strong yellow
            NodeType::Link => egui::Color32::from_rgb(120, 80, 200),        // Purple
            NodeType::FAQ => egui::Color32::from_rgb(230, 120, 30),         // Orange
            NodeType::Contacts => egui::Color32::from_rgb(100, 50, 180),    // Violet
        }
    }
    
    // Add a text color method to ensure readable text on the node backgrounds
    fn get_text_color(&self) -> egui::Color32 {
        match self {
            // Use white text on darker backgrounds, black on lighter ones
            NodeType::Document => egui::Color32::from_rgb(0, 0, 0),  // Black text on yellow
            _ => egui::Color32::WHITE,  // White text on other colors
        }
    }
    
    fn get_outputs(&self) -> Vec<String> {
        match self {
            NodeType::MainMenu | NodeType::Submenu => vec!["Items".to_string()],
            NodeType::Button => vec!["Action".to_string()],
            NodeType::FAQ => vec!["Question".to_string(), "Answer".to_string()],
            _ => vec!["Output".to_string()],
        }
    }
    
    fn get_inputs(&self) -> Vec<String> {
        match self {
            NodeType::MainMenu => vec![],
            NodeType::Submenu | NodeType::Button => vec!["Parent".to_string()],
            _ => vec!["Input".to_string()],
        }
    }
    
    fn get_description(&self) -> &str {
        match self {
            NodeType::MainMenu => "Главное меню бота - начальная точка для всех кнопок",
            NodeType::Submenu => "Подменю содержит набор связанных кнопок",
            NodeType::Button => "Кнопка для взаимодействия пользователя с ботом",
            NodeType::TextContent => "Текстовое сообщение, отправляемое ботом",
            NodeType::Image => "Изображение, отправляемое ботом",
            NodeType::Document => "Документ или файл для скачивания",
            NodeType::Link => "Ссылка на веб-страницу",
            NodeType::FAQ => "Часто задаваемый вопрос с ответом",
            NodeType::Contacts => "Контактная информация",
        }
    }
}

/// Represents a node in the workflow editor
pub struct Node {
    id: usize,
    title: String,
    pos: egui::Pos2,
    size: egui::Vec2,
    inputs: Vec<String>,
    outputs: Vec<String>,
    color: egui::Color32,
    node_type: NodeType,
    properties: HashMap<String, String>,
}

/// Represents a connection between nodes
pub struct Connection {
    from_node: usize,
    from_slot: usize,
    to_node: usize,
    to_slot: usize,
}

// ======== Canvas System ========

/// Handles canvas rendering and interactions
struct CanvasSystem {
    offset: egui::Vec2,
    scale: f32,
    dragging: bool,
}

impl CanvasSystem {
    fn new() -> Self {
        Self {
            offset: egui::Vec2::ZERO,
            scale: 1.0,
            dragging: false,
        }
    }
    
    fn handle_interactions(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui.allocate_response(
            ui.available_size(),
            egui::Sense::click_and_drag()
        );
        
        if response.dragged_by(egui::PointerButton::Middle) {
            self.offset += response.drag_delta();
        }
        
        let scroll_delta = ui.input(|i| i.scroll_delta.y);
        if scroll_delta != 0.0 {
            let zoom_factor = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
            self.scale *= zoom_factor;
            self.scale = self.scale.clamp(0.1, 5.0);
        }
        
        response
    }
    
    fn draw_grid(&self, ui: &mut egui::Ui) {
        let canvas_rect = ui.max_rect();
        let grid_size = 25.0 * self.scale; // Larger grid
        
        let start_x = (self.offset.x % grid_size - grid_size) % grid_size;
        let start_y = (self.offset.y % grid_size - grid_size) % grid_size;
        
        let painter = ui.painter();
        
        // Draw a darker background for the canvas
        painter.rect_filled(
            canvas_rect,
            0.0,
            egui::Color32::from_rgb(40, 40, 50),
        );
        
        // Draw finer grid lines with better contrast
        let minor_grid_color = egui::Color32::from_rgb(55, 55, 65);
        let major_grid_color = egui::Color32::from_rgb(75, 75, 95);
        
        // Draw minor grid lines
        for i in 0..((canvas_rect.width() / grid_size) as i32 + 2) {
            let x = canvas_rect.left() + start_x + i as f32 * grid_size;
            
            // Highlight every 4th line as a major grid line
            let color = if i % 4 == 0 { major_grid_color } else { minor_grid_color };
            let thickness = if i % 4 == 0 { 1.5 } else { 0.8 };
            
            painter.line_segment(
                [egui::pos2(x, canvas_rect.top()), egui::pos2(x, canvas_rect.bottom())],
                egui::Stroke::new(thickness, color),
            );
        }
        
        for j in 0..((canvas_rect.height() / grid_size) as i32 + 2) {
            let y = canvas_rect.top() + start_y + j as f32 * grid_size;
            
            // Highlight every 4th line as a major grid line
            let color = if j % 4 == 0 { major_grid_color } else { minor_grid_color };
            let thickness = if j % 4 == 0 { 1.5 } else { 0.8 };
            
            painter.line_segment(
                [egui::pos2(canvas_rect.left(), y), egui::pos2(canvas_rect.right(), y)],
                egui::Stroke::new(thickness, color),
            );
        }
    }
    
    fn canvas_to_screen_pos(&self, pos: egui::Pos2) -> egui::Pos2 {
        pos + self.offset
    }
    
    fn screen_to_canvas_pos(&self, pos: egui::Pos2) -> egui::Pos2 {
        pos - self.offset
    }
}

// ======== Node System ========

/// Handles node creation, rendering and management
pub struct NodeSystem {
    nodes: Vec<Node>,
    next_id: usize,
    selected_nodes: HashSet<usize>,
    hovered_node: Option<usize>,
    dragging_node: Option<usize>,
    drag_start_pos: Option<egui::Pos2>,
    last_clicked_node: Option<usize>,
    connection_system: ConnectionSystem,
}

impl NodeSystem {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 0,
            selected_nodes: HashSet::new(),
            hovered_node: None,
            dragging_node: None,
            drag_start_pos: None,
            last_clicked_node: None,
            connection_system: ConnectionSystem::new(),
        }
    }
    
    fn create_node(&mut self, title: &str, pos: egui::Pos2, node_type: NodeType) -> usize {
        let node = Node {
            id: self.next_id,
            title: title.to_string(),
            pos,
            size: egui::vec2(200.0, 100.0),
            inputs: node_type.get_inputs(),
            outputs: node_type.get_outputs(),
            color: node_type.get_color(),
            node_type,
            properties: HashMap::new(),
        };
        
        self.nodes.push(node);
        self.next_id += 1;
        self.next_id - 1
    }
    
    fn customize_node(&mut self, node_id: usize, properties: HashMap<String, String>) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            for (key, value) in properties {
                node.properties.insert(key, value);
            }
        }
    }

    fn update_interactions(&mut self, ui: &mut egui::Ui, canvas: &CanvasSystem) {
        let input = ui.input(|i| i.clone());
        
        // Handle node dragging
        if let Some(node_id) = self.dragging_node {
            if input.pointer.primary_down() {
                if let (Some(drag_start), Some(pointer_pos)) = (self.drag_start_pos, input.pointer.interact_pos()) {
                    let delta = pointer_pos - drag_start;
                    
                    // Move all selected nodes
                    if self.selected_nodes.contains(&node_id) {
                        for &selected_id in &self.selected_nodes {
                            if let Some(node) = self.nodes.iter_mut().find(|n| n.id == selected_id) {
                                node.pos += delta * (1.0 / canvas.scale);
                            }
                        }
                    } else {
                        // Or just the dragged node if it's not selected
                        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                            node.pos += delta * (1.0 / canvas.scale);
                        }
                    }
                    
                    // Update drag start position
                    self.drag_start_pos = input.pointer.interact_pos();
                }
            } else {
                // Mouse released, stop dragging
                self.dragging_node = None;
                self.drag_start_pos = None;
            }
        }
    }
    
    fn draw_nodes(&mut self, ui: &mut egui::Ui, canvas: &CanvasSystem) {
        let mut hovered_port: Option<(usize, usize, bool)> = None; // (node_id, slot_index, is_input)
        let mut connecting_from: Option<(usize, usize)> = None; // (node_id, slot_index)
        
        // Draw nodes
        for node in &self.nodes {
        let node_rect = egui::Rect::from_min_size(
            canvas.canvas_to_screen_pos(node.pos),
            node.size * canvas.scale,
        );
        
        let painter = ui.painter();
        
            // Draw node background
            let bg_color = if self.hovered_node == Some(node.id) {
                node.color.linear_multiply(1.3)
            } else if self.selected_nodes.contains(&node.id) {
                node.color.linear_multiply(1.2)
        } else {
            node.color
        };
        
        painter.rect_filled(
            node_rect,
                10.0,
            bg_color,
        );
        
            // Draw node border
        painter.rect_stroke(
            node_rect,
            10.0,
                egui::Stroke::new(2.5 * canvas.scale, bg_color.linear_multiply(0.7)),
        );
        
            // Draw title
        let text_color = node.node_type.get_text_color();
        painter.text(
            node_rect.min + egui::vec2(10.0, 22.0) * canvas.scale,
            egui::Align2::LEFT_TOP,
                &node.title,
            egui::FontId::proportional(18.0 * canvas.scale),
            text_color,
        );
        
            // Draw input slots
        for (i, input) in node.inputs.iter().enumerate() {
            let input_pos = node_rect.min + egui::vec2(0.0, 45.0 + i as f32 * 25.0) * canvas.scale;
                let input_rect = egui::Rect::from_center_size(
                    input_pos,
                    egui::vec2(16.0, 16.0) * canvas.scale,
                );
                
                // Check if mouse is hovering over this input
                if ui.rect_contains_pointer(input_rect) {
                    hovered_port = Some((node.id, i, true));
                }
                
                // Draw input slot with appropriate color
                let slot_color = if let Some((from_id, from_slot)) = connecting_from {
                    if let Some(from_node) = self.nodes.iter().find(|n| n.id == from_id) {
                        if self.connection_system.validate_connection(from_node, from_slot, node, i) {
                            egui::Color32::from_rgb(100, 200, 100) // Valid connection
                        } else {
                            egui::Color32::from_rgb(200, 100, 100) // Invalid connection
                        }
                    } else {
                        egui::Color32::from_rgb(100, 200, 100)
                    }
                } else {
                    egui::Color32::from_rgb(100, 200, 100)
                };
                
            painter.circle_filled(
                input_pos,
                    8.0 * canvas.scale,
                    slot_color,
            );
            
            painter.circle_stroke(
                input_pos,
                8.0 * canvas.scale,
                egui::Stroke::new(1.5 * canvas.scale, egui::Color32::BLACK),
            );
            
                // Draw input label
            painter.text(
                input_pos + egui::vec2(12.0, 0.0) * canvas.scale,
                egui::Align2::LEFT_CENTER,
                input,
                    egui::FontId::proportional(14.0 * canvas.scale),
                text_color,
            );
        }
        
            // Draw output slots
        for (i, output) in node.outputs.iter().enumerate() {
            let output_pos = node_rect.right_top() + egui::vec2(0.0, 45.0 + i as f32 * 25.0) * canvas.scale;
                let output_rect = egui::Rect::from_center_size(
                    output_pos,
                    egui::vec2(16.0, 16.0) * canvas.scale,
                );
                
                // Check if mouse is hovering over this output
                if ui.rect_contains_pointer(output_rect) {
                    hovered_port = Some((node.id, i, false));
                }
                
                // Draw output slot
            painter.circle_filled(
                output_pos,
                    8.0 * canvas.scale,
                    egui::Color32::from_rgb(200, 100, 100),
            );
            
            painter.circle_stroke(
                output_pos,
                8.0 * canvas.scale,
                egui::Stroke::new(1.5 * canvas.scale, egui::Color32::BLACK),
            );
            
                // Draw output label
            painter.text(
                    output_pos - egui::vec2(12.0, 0.0) * canvas.scale,
                egui::Align2::RIGHT_CENTER,
                output,
                    egui::FontId::proportional(14.0 * canvas.scale),
                text_color,
            );
        }
        
            // Handle interactions
            let response = ui.interact(
                node_rect,
                egui::Id::new(format!("node_{}", node.id)),
                egui::Sense::click_and_drag(),
            );
            
            if response.clicked() {
                self.selected_nodes.clear();
                self.selected_nodes.insert(node.id);
                self.last_clicked_node = Some(node.id);
            }
            
            if response.dragged() {
                self.dragging_node = Some(node.id);
            }
        }
        
        // Handle port interactions
        if let Some((node_id, slot_idx, is_input)) = hovered_port {
            if ui.input(|i| i.pointer.primary_pressed()) {
                if is_input {
                    // Start connection from output to this input
                    connecting_from = None;
                } else {
                    // Start connection from this output
                    connecting_from = Some((node_id, slot_idx));
                }
            } else if ui.input(|i| i.pointer.primary_released()) {
                if let Some((from_id, from_slot)) = connecting_from {
                    if is_input && from_id != node_id { // Prevent self-connections
                        // Complete connection
                        if let Some(from_node) = self.nodes.iter().find(|n| n.id == from_id) {
                            if let Some(to_node) = self.nodes.iter().find(|n| n.id == node_id) {
                                if self.connection_system.validate_connection(from_node, from_slot, to_node, slot_idx) {
                                    self.connection_system.create_connection(from_id, from_slot, node_id, slot_idx);
                                }
                            }
                        }
                    }
                }
                connecting_from = None;
            }
            
            // Handle connection deletion with right click
            if ui.input(|i| i.pointer.secondary_clicked()) {
                // First collect all connections to remove
                let connections_to_remove: Vec<(usize, usize, usize, usize)> = self.connection_system
                    .get_connections_for_node(node_id)
                    .iter()
                    .filter(|conn| {
                        (is_input && conn.to_node == node_id && conn.to_slot == slot_idx) ||
                        (!is_input && conn.from_node == node_id && conn.from_slot == slot_idx)
                    })
                    .map(|conn| (conn.from_node, conn.from_slot, conn.to_node, conn.to_slot))
                    .collect();

                // Then remove them one by one
                for (from_node, from_slot, to_node, to_slot) in connections_to_remove {
                    self.connection_system.remove_connection(from_node, from_slot, to_node, to_slot);
                }
            }
        }
        
        // Draw pending connection
        if let Some((from_id, from_slot)) = connecting_from {
            if let Some(from_node) = self.nodes.iter().find(|n| n.id == from_id) {
                let start = canvas.canvas_to_screen_pos(
                    from_node.pos + egui::vec2(from_node.size.x, 45.0 + from_slot as f32 * 25.0)
                );
                
                let end = ui.input(|i| i.pointer.interact_pos()).unwrap_or(start);
                
                // Determine connection color based on hover state
                let connection_color = if let Some((to_id, to_slot, is_input)) = hovered_port {
                    if is_input && from_id != to_id { // Prevent self-connections
                        if let Some(to_node) = self.nodes.iter().find(|n| n.id == to_id) {
                            if self.connection_system.validate_connection(from_node, from_slot, to_node, to_slot) {
                                egui::Color32::from_rgb(100, 200, 100) // Valid connection
                            } else {
                                egui::Color32::from_rgb(200, 100, 100) // Invalid connection
                            }
                        } else {
                            egui::Color32::from_rgb(200, 200, 200) // Neutral color
                        }
                    } else {
                        egui::Color32::from_rgb(200, 100, 100) // Can't connect to output or self
                    }
                } else {
                    egui::Color32::from_rgb(200, 200, 200) // Not hovering over any port
                };
                
                let painter = ui.painter();
                let ctrl_dist = (end.x - start.x).abs() * 0.5;
                
                painter.add(egui::Shape::CubicBezier(
                    egui::epaint::CubicBezierShape::from_points_stroke(
                        [
                            start,
                            start + egui::vec2(ctrl_dist, 0.0),
                            end - egui::vec2(ctrl_dist, 0.0),
                            end
                        ],
                        false,
                        egui::Color32::TRANSPARENT,
                        egui::Stroke::new(3.0 * canvas.scale, connection_color),
                    )
                ));
            }
        }
    }
    
    fn get_selected_node_count(&self) -> usize {
        self.selected_nodes.len()
    }
    
    fn get_selected_node_ids(&self) -> Vec<usize> {
        self.selected_nodes.iter().copied().collect()
    }
    
    fn clear_selection(&mut self) {
        self.selected_nodes.clear();
        self.last_clicked_node = None;
    }
    
    fn select_all_nodes(&mut self) {
        for node in &self.nodes {
            self.selected_nodes.insert(node.id);
        }
    }
    
    fn delete_selected_nodes(&mut self) -> Vec<usize> {
        let selected = self.get_selected_node_ids();
        self.nodes.retain(|node| !self.selected_nodes.contains(&node.id));
        self.selected_nodes.clear();
        self.last_clicked_node = None;
        selected
    }
}

// ======== Connection System ========

/// Handles connections between nodes
pub struct ConnectionSystem {
    connections: Vec<Connection>,
}

impl ConnectionSystem {
    fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }
    
    fn draw_connections(&self, ui: &mut egui::Ui, nodes: &Vec<Node>, canvas: &CanvasSystem) {
        for conn in &self.connections {
            if let (Some(from_node), Some(to_node)) = (
                nodes.iter().find(|n| n.id == conn.from_node),
                nodes.iter().find(|n| n.id == conn.to_node),
            ) {
                self.draw_connection(ui, from_node, conn.from_slot, to_node, conn.to_slot, canvas);
            }
        }
    }
    
    fn draw_connection(&self, ui: &mut egui::Ui, from_node: &Node, from_slot: usize, 
                      to_node: &Node, to_slot: usize, canvas: &CanvasSystem) {
        if from_slot < from_node.outputs.len() && to_slot < to_node.inputs.len() {
            let start = canvas.canvas_to_screen_pos(from_node.pos + egui::vec2(from_node.size.x, 45.0 + from_slot as f32 * 25.0));
            let end = canvas.canvas_to_screen_pos(to_node.pos + egui::vec2(0.0, 45.0 + to_slot as f32 * 25.0));
            
            let painter = ui.painter();
            let ctrl_dist = (end.x - start.x).abs() * 0.5;
            
            // Calculate bezier curve points
            let t = 0.65; // Position along the bezier curve
            let t2 = t + 0.01; // For tangent calculation
            
            // Helper function to calculate bezier point
            let bezier_point = |t: f32| {
                let mt = 1.0 - t;
                let mt2 = mt * mt;
                let mt3 = mt2 * mt;
                let t2 = t * t;
                let t3 = t2 * t;
                
                let p1 = start;
                let p2 = start + egui::vec2(ctrl_dist, 0.0);
                let p3 = end - egui::vec2(ctrl_dist, 0.0);
                let p4 = end;
                
                egui::pos2(
                    mt3 * p1.x + 3.0 * mt2 * t * p2.x + 3.0 * mt * t2 * p3.x + t3 * p4.x,
                    mt3 * p1.y + 3.0 * mt2 * t * p2.y + 3.0 * mt * t2 * p3.y + t3 * p4.y,
                )
            };
            
            let b1 = bezier_point(t);
            let b2 = bezier_point(t2);
            
            // Draw shadow/outline
            painter.add(egui::Shape::CubicBezier(
                egui::epaint::CubicBezierShape::from_points_stroke(
                    [
                        start + egui::vec2(1.0, 1.0),
                        start + egui::vec2(ctrl_dist, 0.0) + egui::vec2(1.0, 1.0),
                        end - egui::vec2(ctrl_dist, 0.0) + egui::vec2(1.0, 1.0),
                        end + egui::vec2(1.0, 1.0)
                    ],
                    false,
                    egui::Color32::TRANSPARENT,
                    egui::Stroke::new(6.0 * canvas.scale, egui::Color32::from_rgb(20, 20, 20)),
                )
            ));
            
            // Draw main connection line
            painter.add(egui::Shape::CubicBezier(
                egui::epaint::CubicBezierShape::from_points_stroke(
                    [
                        start,
                        start + egui::vec2(ctrl_dist, 0.0),
                        end - egui::vec2(ctrl_dist, 0.0),
                        end
                    ],
                    false,
                    egui::Color32::TRANSPARENT,
                    egui::Stroke::new(3.5 * canvas.scale, egui::Color32::from_rgb(250, 250, 60)),
                )
            ));
            
            // Draw glow effect
            painter.add(egui::Shape::CubicBezier(
                egui::epaint::CubicBezierShape::from_points_stroke(
                    [
                        start,
                        start + egui::vec2(ctrl_dist, 0.0),
                        end - egui::vec2(ctrl_dist, 0.0),
                        end
                    ],
                    false,
                    egui::Color32::TRANSPARENT,
                    egui::Stroke::new(2.0 * canvas.scale, egui::Color32::from_rgb(255, 255, 180)),
                )
            ));
            
            // Calculate tangent for arrow
            let tangent = egui::vec2(b2.x - b1.x, b2.y - b1.y).normalized();
            let normal = egui::vec2(-tangent.y, tangent.x);
            
            // Draw arrow head
            let arr_size = 10.0 * canvas.scale;
            let arr_p1 = b1 - tangent * arr_size + normal * arr_size * 0.6;
            let arr_p2 = b1 - tangent * arr_size - normal * arr_size * 0.6;
            
            painter.add(egui::Shape::convex_polygon(
                vec![b1, arr_p1, arr_p2],
                egui::Color32::from_rgb(250, 250, 60),
                egui::Stroke::new(1.0, egui::Color32::from_rgb(20, 20, 20)),
            ));
        }
    }
    
    fn create_connection(&mut self, from_node: usize, from_slot: usize, to_node: usize, to_slot: usize) -> bool {
        // Validate connection
        if from_node == to_node {
            return false; // Can't connect node to itself
        }
        
        // Check for duplicate connections
        if self.connections.iter().any(|conn| {
            conn.from_node == from_node && conn.from_slot == from_slot &&
            conn.to_node == to_node && conn.to_slot == to_slot
        }) {
            return false;
        }
        
        // Add the connection
        let connection = Connection {
            from_node,
            from_slot,
            to_node,
            to_slot,
        };
        
        self.connections.push(connection);
        true
    }
    
    fn remove_connection(&mut self, from_node: usize, from_slot: usize, to_node: usize, to_slot: usize) {
        self.connections.retain(|conn| {
            !(conn.from_node == from_node && conn.from_slot == from_slot &&
              conn.to_node == to_node && conn.to_slot == to_slot)
        });
    }
    
    fn remove_connections_for_nodes(&mut self, nodes: &Vec<usize>) {
        self.connections.retain(|conn| {
            !nodes.contains(&conn.from_node) && !nodes.contains(&conn.to_node)
        });
    }
    
    fn get_connections_for_node(&self, node_id: usize) -> Vec<&Connection> {
        self.connections.iter()
            .filter(|conn| conn.from_node == node_id || conn.to_node == node_id)
            .collect()
    }
    
    fn validate_connection(&self, from_node: &Node, from_slot: usize, to_node: &Node, to_slot: usize) -> bool {
        // Basic validation
        if from_slot >= from_node.outputs.len() || to_slot >= to_node.inputs.len() {
            return false;
        }
        
        // Validate based on node types
        match (from_node.node_type, to_node.node_type) {
            // Main menu can only connect to submenus and buttons
            (NodeType::MainMenu, NodeType::Submenu) |
            (NodeType::MainMenu, NodeType::Button) => true,
            
            // Submenu can only connect to buttons
            (NodeType::Submenu, NodeType::Button) => true,
            
            // Button can connect to content nodes
            (NodeType::Button, NodeType::TextContent) |
            (NodeType::Button, NodeType::Image) |
            (NodeType::Button, NodeType::Document) |
            (NodeType::Button, NodeType::Link) |
            (NodeType::Button, NodeType::FAQ) |
            (NodeType::Button, NodeType::Contacts) => true,
            
            // All other combinations are invalid
            _ => false,
        }
    }
}

// ======== Menu System ========

/// Represents a node creation action returned by the menu
enum NodeAction {
    None,
    CreateMainMenu(egui::Pos2),
    CreateSubmenu(egui::Pos2),
    CreateButton(egui::Pos2),
    CreateTextContent(egui::Pos2),
    CreateImage(egui::Pos2),
    CreateDocument(egui::Pos2), 
    CreateLink(egui::Pos2),
    CreateFAQ(egui::Pos2),
    CreateContacts(egui::Pos2),
    CreateTemplateMenu(egui::Pos2),
}

/// Handles the context menu for node creation
struct MenuSystem {
    show_menu: bool,
    menu_position: egui::Pos2,
    show_help: bool,
}

impl MenuSystem {
    fn new() -> Self {
        Self {
            show_menu: false,
            menu_position: egui::Pos2::ZERO,
            show_help: false,
        }
    }
    
    fn show_menu(&mut self, ctx: &egui::Context, canvas: &CanvasSystem) -> NodeAction {
        if !self.show_menu {
            return NodeAction::None;
        }
        
        let canvas_pos = canvas.screen_to_canvas_pos(self.menu_position);
        let mut show_menu = true;
        let mut action = NodeAction::None;
        
        egui::Window::new("Добавить элемент")
            .title_bar(true)
            .fixed_pos(self.menu_position)
            .auto_sized()
            .open(&mut show_menu)
            .show(ctx, |ui| {
                action = self.render_menu_content(ui, canvas_pos);
            });
        
        if !show_menu || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_menu = false;
        }
        
        // Show help window if active
        if self.show_help {
            let mut show_help = true;
            egui::Window::new("Помощь по редактору")
                .title_bar(true)
                .auto_sized()
                .open(&mut show_help)
                .show(ctx, |ui| {
                    ui.heading("Как пользоваться редактором меню:");
                    ui.add_space(10.0);
                    
                    ui.label("1. Начните с добавления элемента 'Главное меню'");
                    ui.label("2. Добавляйте подменю и кнопки, соединяя их между собой");
                    ui.label("3. Добавляйте содержимое (текст, изображения и др.) и связывайте с кнопками");
                    ui.add_space(10.0);
                    
                    ui.collapsing("Элементы меню бота", |ui| {
                        ui.label("🔝 Главное меню - начальная точка всего бота");
                        ui.label("📂 Подменю - группирует связанные кнопки");
                        ui.label("🔘 Кнопка - элемент для взаимодействия пользователя с ботом");
                    });
                    
                    ui.collapsing("Содержимое для кнопок", |ui| {
                        ui.label("📝 Текст - текстовое сообщение от бота");
                        ui.label("🖼️ Изображение - картинка, отправляемая ботом");
                        ui.label("📄 Документ - файл для скачивания");
                        ui.label("🔗 Ссылка - переход на веб-страницу");
                        ui.label("❓ FAQ - часто задаваемый вопрос с ответом");
                        ui.label("📞 Контакты - контактная информация");
                    });
                    
                    ui.add_space(10.0);
                    ui.collapsing("Горячие клавиши", |ui| {
                        ui.label("Shift+M - добавить главное меню");
                        ui.label("Shift+S - добавить подменю");
                        ui.label("Shift+B - добавить кнопку");
                        ui.label("Shift+T - добавить текст");
                        ui.label("Ctrl+Space - открыть меню добавления элементов");
                    });
                    
                    ui.add_space(20.0);
                    if ui.button("Закрыть справку").clicked() {
                        self.show_help = false;
                    }
                });
            
            if !show_help {
                self.show_help = false;
            }
        }
        
        action
    }
    
    fn render_menu_content(&self, ui: &mut egui::Ui, canvas_pos: egui::Pos2) -> NodeAction {
        ui.set_min_width(280.0);
        let mut action = NodeAction::None;
        
        // Help button with icon
        if ui.add(
            Self::create_menu_button("❓ Показать справку", "")
                .min_size([280.0, 32.0].into())
                .fill(egui::Color32::from_rgb(45, 45, 55))
        ).clicked() {
            ui.close_menu();
            return NodeAction::None;
        }
        
        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);
        
        // Menu structure section
        ui.collapsing(
            egui::RichText::new("Структура меню")
                .strong()
                .size(16.0)
                .color(egui::Color32::from_rgb(150, 150, 230)),
            |ui| {
                egui::Grid::new("menu_structure_grid")
                    .num_columns(3)
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        // Main Menu button
                        if ui.add(Self::create_menu_button("🔝 Главное меню", "Shift+M"))
                        .on_hover_text("Начальная точка для всего меню бота")
                        .clicked() {
                        action = NodeAction::CreateMainMenu(canvas_pos);
                        ui.close_menu();
                    }
                        ui.label(egui::RichText::new("(Shift+M)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                        if ui.add(Self::create_menu_button("Шаблон", "")).clicked() {
                        action = NodeAction::CreateTemplateMenu(canvas_pos);
                        ui.close_menu();
                    }
                    ui.end_row();
                    
                        // Submenu button
                        if ui.add(Self::create_menu_button("📂 Подменю", "Shift+S"))
                        .on_hover_text("Группа связанных кнопок")
                        .clicked() {
                        action = NodeAction::CreateSubmenu(canvas_pos);
                        ui.close_menu();
                    }
                        ui.label(egui::RichText::new("(Shift+S)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                    ui.label("");
                    ui.end_row();
                    
                        // Button button
                        if ui.add(Self::create_menu_button("🔘 Кнопка", "Shift+B"))
                        .on_hover_text("Элемент для взаимодействия")
                        .clicked() {
                        action = NodeAction::CreateButton(canvas_pos);
                        ui.close_menu();
                    }
                        ui.label(egui::RichText::new("(Shift+B)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                    ui.label("");
                    ui.end_row();
                });
            }
        );

        ui.add_space(4.0);
        
        // Content nodes section
        ui.collapsing(
            egui::RichText::new("Содержимое")
                .strong()
                .size(16.0)
                .color(egui::Color32::from_rgb(150, 230, 150)),
            |ui| {
                let layout = egui::Layout::top_down_justified(egui::Align::Center);
                ui.with_layout(layout, |ui| {
                    ui.add_space(4.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(Self::create_menu_button("📝 Текст", "Shift+T"))
                            .on_hover_text("Текстовое сообщение от бота")
                            .clicked() {
                            action = NodeAction::CreateTextContent(canvas_pos);
                            ui.close_menu();
                        }
                        ui.label(egui::RichText::new("(Shift+T)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                        
                        ui.add_space(10.0);
                        
                        if ui.add(Self::create_menu_button("🖼️ Изображение", "Shift+I"))
                            .on_hover_text("Изображение, отправляемое ботом")
                            .clicked() {
                            action = NodeAction::CreateImage(canvas_pos);
                            ui.close_menu();
                        }
                        ui.label(egui::RichText::new("(Shift+I)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                    });

                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        if ui.add(Self::create_menu_button("📄 Документ", "Shift+F"))
                            .on_hover_text("Документ для скачивания")
                            .clicked() {
                            action = NodeAction::CreateDocument(canvas_pos);
                            ui.close_menu();
                        }
                        ui.label(egui::RichText::new("(Shift+F)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                        
                        ui.add_space(10.0);
                        
                        if ui.add(Self::create_menu_button("🔗 Ссылка", "Shift+L"))
                            .on_hover_text("Ссылка на веб-страницу")
                            .clicked() {
                            action = NodeAction::CreateLink(canvas_pos);
                            ui.close_menu();
                        }
                        ui.label(egui::RichText::new("(Shift+L)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                    });
                });
            }
        );

        ui.add_space(4.0);
        
        // Special nodes section
        ui.collapsing(
            egui::RichText::new("Специальные элементы")
                .strong()
                .size(16.0)
                .color(egui::Color32::from_rgb(230, 150, 150)),
            |ui| {
                ui.add_space(4.0);
                
                ui.horizontal(|ui| {
                    if ui.add(Self::create_menu_button("❓ FAQ", "Shift+Q"))
                        .on_hover_text("Часто задаваемый вопрос с ответом")
                        .clicked() {
                        action = NodeAction::CreateFAQ(canvas_pos);
                        ui.close_menu();
                    }
                    ui.label(egui::RichText::new("(Shift+Q)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                });

                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    if ui.add(Self::create_menu_button("📞 Контакты", "Shift+C"))
                        .on_hover_text("Контактная информация")
                        .clicked() {
                        action = NodeAction::CreateContacts(canvas_pos);
                        ui.close_menu();
                    }
                    ui.label(egui::RichText::new("(Shift+C)").size(14.0).color(egui::Color32::from_rgb(170, 170, 170)));
                });
            }
        );
        
        action
    }

    // Helper function to create consistently styled menu buttons
    fn create_menu_button(text: &str, _shortcut: &str) -> egui::Button {
        let mut button = egui::Button::new(
            egui::RichText::new(text)
                .size(15.0)
                .color(egui::Color32::from_rgb(230, 230, 230))
        );

        button = button
            .min_size([120.0, 32.0].into())
            .fill(egui::Color32::from_rgb(60, 60, 70))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 120)))
            .rounding(egui::Rounding::same(6.0));

        button
    }
}

// ======== NodeFactory Trait ========

/// Factory trait for creating different types of nodes
trait NodeFactory {
    fn create_menu_button_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_submenu_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_text_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_image_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_file_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_link_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_condition_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_action_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_faq_node(&mut self, pos: egui::Pos2) -> usize;
    fn create_custom_node(&mut self, pos: egui::Pos2) -> usize;
}

// ======== Workflow Editor App ========

/// Main application that ties all systems together
pub struct WorkflowEditorApp {
    canvas: CanvasSystem,
    node_system: NodeSystem,
    connection_system: ConnectionSystem,
    menu_system: MenuSystem,
}

impl NodeFactory for WorkflowEditorApp {
    fn create_menu_button_node(&mut self, pos: egui::Pos2) -> usize {
        self.node_system.create_node("Menu Button", pos, NodeType::MainMenu)
    }
    
    fn create_submenu_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("Submenu", pos, NodeType::Submenu);
        self.node_system.customize_node(id, HashMap::from([
            ("callback_data".to_string(), "submenu".to_string()),
        ]));
        id
    }
    
    fn create_text_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("Text", pos, NodeType::TextContent);
        self.node_system.customize_node(id, HashMap::from([
            ("text".to_string(), "Содержимое текста...".to_string()),
        ]));
        id
    }
    
    fn create_image_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("Image", pos, NodeType::Image);
        self.node_system.customize_node(id, HashMap::from([
            ("url".to_string(), "https://example.com/image.jpg".to_string()),
        ]));
        id
    }
    
    fn create_file_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("File", pos, NodeType::Document);
        self.node_system.customize_node(id, HashMap::from([
            ("url".to_string(), "https://example.com/document.pdf".to_string()),
        ]));
        id
    }
    
    fn create_link_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("Link", pos, NodeType::Link);
        id
    }
    
    fn create_condition_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("Condition", pos, NodeType::FAQ);
        self.node_system.customize_node(id, HashMap::from([
            ("question".to_string(), "Часто задаваемый вопрос?".to_string()),
            ("answer".to_string(), "Ответ на вопрос...".to_string()),
        ]));
        id
    }
    
    fn create_action_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("Action", pos, NodeType::Button);
        self.node_system.customize_node(id, HashMap::from([
            ("callback_data".to_string(), "button".to_string()),
        ]));
        id
    }
    
    fn create_faq_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("FAQ", pos, NodeType::FAQ);
        id
    }
    
    fn create_custom_node(&mut self, pos: egui::Pos2) -> usize {
        let id = self.node_system.create_node("Custom", pos, NodeType::Contacts);
        self.node_system.customize_node(id, HashMap::from([
            ("address".to_string(), "Адрес организации".to_string()),
            ("phone".to_string(), "+7 (XXX) XXX-XX-XX".to_string()),
            ("email".to_string(), "example@example.com".to_string()),
        ]));
        id
    }
}

impl WorkflowEditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            canvas: CanvasSystem::new(),
            node_system: NodeSystem::new(),
            connection_system: ConnectionSystem::new(),
            menu_system: MenuSystem::new(),
        }
    }
    
    fn handle_node_action(&mut self, action: NodeAction) {
        match action {
            NodeAction::None => {},
            NodeAction::CreateMainMenu(pos) => {
                let id = self.node_system.create_node("Главное меню", pos, NodeType::MainMenu);
                self.node_system.customize_node(id, HashMap::from([
                    ("callback_data".to_string(), "main_menu".to_string()),
                ]));
            },
            NodeAction::CreateSubmenu(pos) => {
                let id = self.node_system.create_node("Подменю", pos, NodeType::Submenu);
                self.node_system.customize_node(id, HashMap::from([
                    ("callback_data".to_string(), "submenu".to_string()),
                ]));
            },
            NodeAction::CreateButton(pos) => {
                let id = self.node_system.create_node("Кнопка", pos, NodeType::Button);
                self.node_system.customize_node(id, HashMap::from([
                    ("callback_data".to_string(), "button".to_string()),
                ]));
            },
            NodeAction::CreateTextContent(pos) => {
                let id = self.node_system.create_node("Текст", pos, NodeType::TextContent);
                self.node_system.customize_node(id, HashMap::from([
                    ("text".to_string(), "Содержимое текста...".to_string()),
                ]));
            },
            NodeAction::CreateImage(pos) => {
                let id = self.node_system.create_node("Изображение", pos, NodeType::Image);
                self.node_system.customize_node(id, HashMap::from([
                    ("url".to_string(), "https://example.com/image.jpg".to_string()),
                ]));
            },
            NodeAction::CreateDocument(pos) => {
                let id = self.node_system.create_node("Документ", pos, NodeType::Document);
                self.node_system.customize_node(id, HashMap::from([
                    ("url".to_string(), "https://example.com/document.pdf".to_string()),
                ]));
            },
            NodeAction::CreateLink(pos) => {
                let id = self.node_system.create_node("Ссылка", pos, NodeType::Link);
                self.node_system.customize_node(id, HashMap::from([
                    ("url".to_string(), "https://example.com".to_string()),
                ]));
            },
            NodeAction::CreateFAQ(pos) => {
                let id = self.node_system.create_node("FAQ", pos, NodeType::FAQ);
                self.node_system.customize_node(id, HashMap::from([
                    ("question".to_string(), "Часто задаваемый вопрос?".to_string()),
                    ("answer".to_string(), "Ответ на вопрос...".to_string()),
                ]));
            },
            NodeAction::CreateContacts(pos) => {
                let id = self.node_system.create_node("Контакты", pos, NodeType::Contacts);
                self.node_system.customize_node(id, HashMap::from([
                    ("address".to_string(), "Адрес организации".to_string()),
                    ("phone".to_string(), "+7 (XXX) XXX-XX-XX".to_string()),
                    ("email".to_string(), "example@example.com".to_string()),
                ]));
            },
            NodeAction::CreateTemplateMenu(pos) => {
                self.create_template_menu(pos);
            },
        }
    }
    
    /// Creates a template menu structure at the given position
    fn create_template_menu(&mut self, pos: egui::Pos2) {
        // Create main menu
        let main_menu_id = self.node_system.create_node("Главное меню бота", pos, NodeType::MainMenu);
        self.node_system.customize_node(main_menu_id, HashMap::from([
            ("title".to_string(), "Меню бота".to_string()),
        ]));
        
        // Create submenu
        let submenu_id = self.node_system.create_node("Раздел меню", 
                               pos + egui::vec2(250.0, -100.0), NodeType::Submenu);
        self.node_system.customize_node(submenu_id, HashMap::from([
            ("callback_data".to_string(), "submenu_1".to_string()),
        ]));
        
        // Create a button
        let button_id = self.node_system.create_node("Кнопка",
                              pos + egui::vec2(250.0, 50.0), NodeType::Button);
        self.node_system.customize_node(button_id, HashMap::from([
            ("callback_data".to_string(), "button_1".to_string()),
        ]));
        
        // Create text content
        let text_id = self.node_system.create_node("Текстовое сообщение",
                             pos + egui::vec2(500.0, 50.0), NodeType::TextContent);
        self.node_system.customize_node(text_id, HashMap::from([
            ("text".to_string(), "Пример текстового сообщения от бота".to_string()),
        ]));
        
        // Create connections
        self.connection_system.create_connection(main_menu_id, 0, submenu_id, 0);
        self.connection_system.create_connection(main_menu_id, 0, button_id, 0);
        self.connection_system.create_connection(button_id, 0, text_id, 0);
    }
    
    /// Handle keyboard shortcuts for node creation
    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        let has_shift = ctx.input(|i| i.modifiers.shift);
        
        if has_shift {
            if let Some(pos) = ctx.pointer_interact_pos() {
                let canvas_pos = self.canvas.screen_to_canvas_pos(pos);
                
                if ctx.input(|i| i.key_pressed(egui::Key::M)) {
                    self.handle_node_action(NodeAction::CreateMainMenu(canvas_pos));
                } else if ctx.input(|i| i.key_pressed(egui::Key::S)) {
                    self.handle_node_action(NodeAction::CreateSubmenu(canvas_pos));
                } else if ctx.input(|i| i.key_pressed(egui::Key::B)) { // B for Button instead of M
                    self.handle_node_action(NodeAction::CreateButton(canvas_pos));
                } else if ctx.input(|i| i.key_pressed(egui::Key::T)) {
                    self.handle_node_action(NodeAction::CreateTextContent(canvas_pos));
                } else if ctx.input(|i| i.key_pressed(egui::Key::I)) {
                    self.handle_node_action(NodeAction::CreateImage(canvas_pos));
                } else if ctx.input(|i| i.key_pressed(egui::Key::F)) {
                    self.handle_node_action(NodeAction::CreateDocument(canvas_pos));
                } else if ctx.input(|i| i.key_pressed(egui::Key::L)) {
                    self.handle_node_action(NodeAction::CreateLink(canvas_pos));
                } else if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
                    self.handle_node_action(NodeAction::CreateFAQ(canvas_pos));
                } else if ctx.input(|i| i.key_pressed(egui::Key::C)) {
                    self.handle_node_action(NodeAction::CreateContacts(canvas_pos));
                }
            }
        }
        
        // Show help with F1 or Ctrl+H
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) || 
           (ctx.input(|i| i.modifiers.ctrl) && ctx.input(|i| i.key_pressed(egui::Key::H))) {
            self.menu_system.show_help = true;
        }
    }
}

impl eframe::App for WorkflowEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set global style for better accessibility
        {
            let mut style = (*ctx.style()).clone();
            
            // Increase font sizes for better readability
            style.text_styles = [
                (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Heading, egui::FontId::new(22.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Monospace, egui::FontId::new(16.0, egui::FontFamily::Monospace)),
                (egui::TextStyle::Small, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            ].into();
            
            // Increase spacing and padding for easier targets
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
            style.spacing.button_padding = egui::vec2(12.0, 6.0);
            style.spacing.window_margin = egui::style::Margin::same(10.0);
            style.spacing.menu_margin = egui::style::Margin::same(8.0);
            
            // Enhanced UI contrast and colors
            style.visuals.override_text_color = Some(egui::Color32::from_rgb(230, 230, 230)); // Brighter default text
            style.visuals.window_fill = egui::Color32::from_rgb(38, 38, 45);
            style.visuals.panel_fill = egui::Color32::from_rgb(48, 48, 56);
            style.visuals.faint_bg_color = egui::Color32::from_rgb(40, 40, 48);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(30, 30, 38);
            
            // Customize widget colors
            style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(55, 55, 65);
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(65, 65, 75);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(85, 85, 105);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(95, 95, 120);
            
            // Thicker strokes with better contrast
            style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(130, 130, 150));
            style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(150, 150, 170));
            style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(180, 180, 210));
            style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 200, 230));
            
            // Improved selection highlighting
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(75, 110, 175);
            style.visuals.selection.stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(120, 150, 210));
            
            // Adjust window and popup rounding for a more modern look
            style.visuals.window_rounding = egui::Rounding::same(6.0);
            style.visuals.popup_shadow.extrusion = 8.0;
            
            ctx.set_style(style);
        }
        
        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);
        
        // Check for Ctrl+Space to open context menu
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Space)) {
            if let Some(pos) = ctx.pointer_interact_pos() {
                self.menu_system.show_menu = true;
                self.menu_system.menu_position = pos;
            }
        }
        
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.heading(
                    egui::RichText::new("Редактор меню телеграм-бота")
                        .size(20.0)
                        .color(egui::Color32::from_rgb(220, 220, 255))
                        .strong()
                );
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);
                
                if ui.add(
                    egui::Button::new(
                        egui::RichText::new("➕ Добавить элемент")
                            .size(16.0)
                            .color(egui::Color32::from_rgb(230, 230, 230))
                    )
                    .min_size([160.0, 32.0].into())
                    .fill(egui::Color32::from_rgb(60, 60, 70))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 120)))
                    .rounding(egui::Rounding::same(6.0))
                ).clicked() {
                    if let Some(pos) = ctx.pointer_interact_pos() {
                        self.menu_system.show_menu = true;
                        self.menu_system.menu_position = pos;
                    }
                }
                
                ui.add_space(8.0);
                
                if ui.add(
                    egui::Button::new(
                        egui::RichText::new("❓ Справка")
                            .size(16.0)
                            .color(egui::Color32::from_rgb(230, 230, 230))
                    )
                    .min_size([120.0, 32.0].into())
                    .fill(egui::Color32::from_rgb(60, 60, 70))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 120)))
                    .rounding(egui::Rounding::same(6.0))
                ).clicked() {
                    self.menu_system.show_help = true;
                }
                
                let selected_count = self.node_system.get_selected_node_count();
                if selected_count > 0 {
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);
                    ui.label(
                        egui::RichText::new(format!("Выбрано: {}", selected_count))
                            .size(16.0)
                            .color(egui::Color32::from_rgb(200, 200, 255))
                    );
                    ui.add_space(8.0);
                    
                    if ui.add(
                        egui::Button::new(
                            egui::RichText::new("🗑 Удалить")
                                .size(16.0)
                                .color(egui::Color32::from_rgb(255, 200, 200))
                        )
                        .min_size([120.0, 32.0].into())
                        .fill(egui::Color32::from_rgb(70, 50, 50))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(120, 100, 100)))
                        .rounding(egui::Rounding::same(6.0))
                    ).clicked() {
                        let deleted_nodes = self.node_system.delete_selected_nodes();
                        // Also remove connections involving these nodes
                        self.connection_system.remove_connections_for_nodes(&deleted_nodes);
                    }
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    if ui.add(
                        egui::Button::new(
                            egui::RichText::new("📥 Импорт JSON")
                                .size(16.0)
                                .color(egui::Color32::from_rgb(230, 230, 230))
                        )
                        .min_size([140.0, 32.0].into())
                        .fill(egui::Color32::from_rgb(60, 60, 70))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 120)))
                        .rounding(egui::Rounding::same(6.0))
                    ).clicked() {
                        // JSON import functionality would go here
                    }
                    
                    ui.add_space(8.0);
                    if ui.add(
                        egui::Button::new(
                            egui::RichText::new("📤 Экспорт JSON")
                                .size(16.0)
                                .color(egui::Color32::from_rgb(230, 230, 230))
                        )
                        .min_size([140.0, 32.0].into())
                        .fill(egui::Color32::from_rgb(60, 60, 70))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 120)))
                        .rounding(egui::Rounding::same(6.0))
                    ).clicked() {
                        // JSON export functionality would go here
                    }
                    ui.add_space(8.0);
                });
            });
            ui.add_space(4.0);
        });
        
        // Status bar
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Узлов: {}", self.node_system.nodes.len()));
                ui.separator();
                ui.label(format!("Соединений: {}", self.connection_system.connections.len()));
                ui.separator();
                
                // Display zoom level
                ui.label(format!("Масштаб: {}%", (self.canvas.scale * 100.0) as i32));
                
                // Show selection status if nodes are selected
                let selected_count = self.node_system.get_selected_node_count();
                if selected_count > 0 {
                    ui.separator();
                    ui.label(format!("Выбрано: {}", selected_count));
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Ctrl+клик: мульти-выбор   Shift+клик: диапазон   Del: удалить");
                });
            });
        });
        
        // Main canvas area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Handle canvas interactions
            let response = self.canvas.handle_interactions(ui);
            
            // Process key presses
            if ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) {
                let deleted_nodes = self.node_system.delete_selected_nodes();
                self.connection_system.remove_connections_for_nodes(&deleted_nodes);
            }
            
            if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::A)) {
                self.node_system.select_all_nodes();
            }
            
            // Handle node dragging
            self.node_system.update_interactions(ui, &self.canvas);
            
            // Draw elements in the correct order:
            // 1. Grid (background)
            self.canvas.draw_grid(ui);
            
            // 2. Connections (behind nodes)
            self.connection_system.draw_connections(ui, &self.node_system.nodes, &self.canvas);
            
            // 3. Nodes (top layer)
            self.node_system.draw_nodes(ui, &self.canvas);
            
            // Check for right-click to open menu
            if response.secondary_clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.menu_system.show_menu = true;
                    self.menu_system.menu_position = pos;
                }
            }
        });
        
        // Show context menu if active and process any actions
        let action = self.menu_system.show_menu(ctx, &self.canvas);
        if matches!(action, NodeAction::None) && self.menu_system.show_help {
            // Special case - show help was clicked
            self.menu_system.show_help = true;
        } else {
            self.handle_node_action(action);
        }
    }
}

// ======== WebAssembly Entry Point ========

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_app(canvas_id: &str) -> Result<(), JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();
    let canvas_id = canvas_id.to_owned(); // Clone the string to avoid lifetime issues
    
    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start(
                &canvas_id,
                web_options,
                Box::new(|cc| Box::new(WorkflowEditorApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}
