use eframe::egui;
use std::collections::{HashMap, HashSet};

pub struct MenuEditorApp {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    dragging: Option<usize>,
    connecting_from: Option<(usize, usize)>,
    pan_offset: egui::Vec2,
    zoom: f32,
    next_node_id: usize,
    next_connection_id: usize,
    selected_nodes: HashSet<usize>,
    use_linear_connections: bool,
}

struct Node {
    id: usize,
    title: String,
    pos: egui::Pos2,
    inputs: Vec<NodeSocket>,
    outputs: Vec<NodeSocket>,
    node_type: NodeType,
    size: egui::Vec2,
}

struct NodeSocket {
    id: usize,
    name: String,
    socket_type: SocketType,
}

struct Connection {
    id: usize,
    from_node: usize,
    from_socket: usize,
    to_node: usize,
    to_socket: usize,
    connection_type: ConnectionType,
    control_point1: egui::Vec2,
    control_point2: egui::Vec2,
    dragging_point: Option<usize>,
}

enum NodeType {
    Menu,
    Button,
    Message,
    FAQ,
}

enum SocketType {
    Input,
    Output,
}

enum ConnectionType {
    Bezier,
    Linear,
}

impl MenuEditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Create an initial node setup
        let mut app = Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            dragging: None,
            connecting_from: None,
            pan_offset: egui::Vec2::ZERO,
            zoom: 1.0,
            next_node_id: 0,
            next_connection_id: 0,
            selected_nodes: HashSet::new(),
            use_linear_connections: false,
        };
        
        // Add a sample root menu node
        app.create_menu_node("Главное меню", egui::pos2(300.0, 200.0));
        
        app
    }
    
    fn create_menu_node(&mut self, title: &str, pos: egui::Pos2) -> usize {
        let id = self.next_node_id;
        self.next_node_id += 1;
        
        self.nodes.push(Node {
            id,
            title: title.to_owned(),
            pos,
            inputs: vec![
                NodeSocket {
                    id: 0,
                    name: "Parent".to_owned(),
                    socket_type: SocketType::Input,
                }
            ],
            outputs: vec![
                NodeSocket {
                    id: 0,
                    name: "Children".to_owned(),
                    socket_type: SocketType::Output,
                }
            ],
            node_type: NodeType::Menu,
            size: egui::vec2(180.0, 100.0),
        });
        
        id
    }
    
    fn create_button_node(&mut self, title: &str, pos: egui::Pos2) -> usize {
        let id = self.next_node_id;
        self.next_node_id += 1;
        
        self.nodes.push(Node {
            id,
            title: title.to_owned(),
            pos,
            inputs: vec![
                NodeSocket {
                    id: 0,
                    name: "Parent".to_owned(),
                    socket_type: SocketType::Input,
                }
            ],
            outputs: vec![
                NodeSocket {
                    id: 0,
                    name: "Action".to_owned(),
                    socket_type: SocketType::Output,
                }
            ],
            node_type: NodeType::Button,
            size: egui::vec2(150.0, 80.0),
        });
        
        id
    }
    
    fn add_connection(&mut self, from_node: usize, from_socket: usize, to_node: usize, to_socket: usize) {
        let id = self.next_connection_id;
        self.next_connection_id += 1;
        
        if let (Some(from), Some(to)) = (
            self.nodes.iter().find(|n| n.id == from_node),
            self.nodes.iter().find(|n| n.id == to_node)
        ) {
            let start_pos = from.pos + egui::vec2(
                from.size.x,
                40.0 + from_socket as f32 * 20.0,
            );
            
            let end_pos = to.pos + egui::vec2(
                0.0,
                40.0 + to_socket as f32 * 20.0,
            );
            
            let diff = end_pos - start_pos;
            let control_point1: egui::Vec2;
            let control_point2: egui::Vec2;
            
            if self.use_linear_connections {
                // For linear connections, use midpoints for orthogonal routing
                let mid_x = start_pos.x + (end_pos.x - start_pos.x) / 2.0;
                control_point1 = egui::vec2(mid_x, start_pos.y);
                control_point2 = egui::vec2(mid_x, end_pos.y);
            } else {
                // For bezier curves, use the original control points
                control_point1 = (start_pos + diff * 0.33).to_vec2();
                control_point2 = (start_pos + diff * 0.66).to_vec2();
            }
            
            self.connections.push(Connection {
                id,
                from_node,
                from_socket,
                to_node,
                to_socket,
                connection_type: if self.use_linear_connections {
                    ConnectionType::Linear
                } else {
                    ConnectionType::Bezier
                },
                control_point1,
                control_point2,
                dragging_point: None,
            });
        }
    }
}

impl eframe::App for MenuEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle panning with middle mouse button
        if ctx.input(|i| i.pointer.middle_down()) {
            let delta = ctx.input(|i| i.pointer.delta());
            self.pan_offset += delta;
        }
        
        // Handle zoom with scroll
        if ctx.input(|i| i.scroll_delta.y != 0.0) {
            let zoom_delta = ctx.input(|i| i.scroll_delta.y) * 0.001;
            self.zoom = (self.zoom + zoom_delta).clamp(0.5, 2.0);
        }
        
        // Main panel for node editor
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Редактор структуры меню телеграм-бота");
            
            // Toolbar
            ui.horizontal(|ui| {
                if ui.button("+ Меню").clicked() {
                    let center = ui.available_rect_before_wrap().center();
                    self.create_menu_node("Новое меню", egui::pos2(center.x, center.y));
                }
                if ui.button("+ Кнопка").clicked() {
                    let center = ui.available_rect_before_wrap().center();
                    self.create_button_node("Новая кнопка", egui::pos2(center.x, center.y));
                }
                if ui.button("Удалить выбранное").clicked() {
                    let selected = self.selected_nodes.clone();
                    self.nodes.retain(|node| !selected.contains(&node.id));
                    self.connections.retain(|conn| {
                        !selected.contains(&conn.from_node) && !selected.contains(&conn.to_node)
                    });
                    self.selected_nodes.clear();
                }
                ui.checkbox(&mut self.use_linear_connections, "Linear Connections");
            });
            
            // Node editor area
            let _canvas_response = egui::Frame::canvas(ui.style())
                .show(ui, |ui| {
                    let canvas_rect = ui.max_rect();
                    
                    // Draw grid background
                    self.draw_grid(ui, canvas_rect);
                    
                    // Draw connections
                    // Fix: Create a temporary vector of connections to avoid multiple mutable borrows
                    let mut connections = std::mem::take(&mut self.connections);
                    for connection in &mut connections {
                        self.draw_connection(ui, connection);
                    }
                    self.connections = connections;
                    
                    // Collect all node interactions first
                    let mut node_interactions = Vec::new();
                    for (idx, node) in self.nodes.iter_mut().enumerate() {
                        let interaction = NodeInteraction::collect(
                            ui,
                            node,
                            idx,
                            &self.pan_offset,
                            self.zoom,
                            &self.selected_nodes,
                            &self.connecting_from,
                        );
                        node_interactions.push(interaction);
                    }
                    
                    // Process all interactions
                    let mut node_to_drag = None;
                    for (idx, interaction) in node_interactions.into_iter().enumerate() {
                        // Handle dragging
                        if interaction.dragged {
                            node_to_drag = Some(idx);
                        }
                        
                        // Handle selection
                        if interaction.clicked {
                            if !ui.input(|i| i.modifiers.ctrl) {
                                self.selected_nodes.clear();
                            }
                            self.selected_nodes.insert(self.nodes[idx].id);
                        }
                        
                        // Handle connections
                        if let Some((is_input, socket_idx)) = interaction.clicked_socket {
                            if is_input {
                                if let Some((from_node, from_socket)) = self.connecting_from.take() {
                                    self.add_connection(from_node, from_socket, self.nodes[idx].id, socket_idx);
                                }
                            } else {
                                self.connecting_from = Some((self.nodes[idx].id, socket_idx));
                            }
                        }
                    }
                    
                    // Handle node dragging
                    if let Some(node_idx) = node_to_drag {
                        self.dragging = Some(node_idx);
                    }
                    
                    if self.dragging.is_some() && !ctx.input(|i| i.pointer.any_down()) {
                        self.dragging = None;
                    }
                    
                    if let Some(node_idx) = self.dragging {
                        let delta = ctx.input(|i| i.pointer.delta());
                        self.nodes[node_idx].pos += delta;
                    }
                    
                    // Handle connecting nodes
                    if self.connecting_from.is_some() && !ctx.input(|i| i.pointer.any_down()) {
                        self.connecting_from = None;
                    }
                });
        });
    }
}

// New struct to collect node interactions
struct NodeInteraction {
    dragged: bool,
    clicked: bool,
    clicked_socket: Option<(bool, usize)>,
}

impl NodeInteraction {
    fn collect(
        ui: &mut egui::Ui,
        node: &mut Node,
        _idx: usize,
        pan_offset: &egui::Vec2,
        zoom: f32,
        selected_nodes: &HashSet<usize>,
        connecting_from: &Option<(usize, usize)>,
    ) -> Self {
        let painter = ui.painter();
        
        // Apply pan and zoom transformations
        let pos = node.pos + *pan_offset;
        let size = node.size * zoom;
        
        let node_rect = egui::Rect::from_min_size(pos, size);
        
        // Node background
        let mut node_color = match node.node_type {
            NodeType::Menu => egui::Color32::from_rgb(70, 130, 180),
            NodeType::Button => egui::Color32::from_rgb(60, 179, 113),
            NodeType::Message => egui::Color32::from_rgb(205, 92, 92),
            NodeType::FAQ => egui::Color32::from_rgb(147, 112, 219),
        };
        
        // Highlight selected nodes
        if selected_nodes.contains(&node.id) {
            node_color = node_color.linear_multiply(1.2);
        }
        
        painter.rect(
            node_rect,
            4.0,
            node_color,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
        
        // Node title
        painter.text(
            node_rect.min + egui::vec2(10.0, 20.0),
            egui::Align2::LEFT_TOP,
            &node.title,
            egui::FontId::proportional(16.0 * zoom),
            egui::Color32::WHITE,
        );
        
        let mut clicked_socket = None;
        
        // Draw input sockets
        for (socket_idx, socket) in node.inputs.iter().enumerate() {
            let socket_pos = node_rect.min + egui::vec2(0.0, 40.0 + socket_idx as f32 * 20.0 * zoom);
            painter.circle(
                socket_pos,
                6.0 * zoom,
                egui::Color32::DARK_GRAY,
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            );
            
            painter.text(
                socket_pos + egui::vec2(10.0, 0.0),
                egui::Align2::LEFT_CENTER,
                &socket.name,
                egui::FontId::proportional(14.0 * zoom),
                egui::Color32::WHITE,
            );
            
            let socket_rect = egui::Rect::from_center_size(socket_pos, egui::vec2(12.0, 12.0) * zoom);
            if ui.interact(socket_rect, ui.id().with("socket_in").with(node.id).with(socket_idx), egui::Sense::click()).clicked() {
                clicked_socket = Some((true, socket_idx));
            }
        }
        
        // Draw output sockets
        for (socket_idx, socket) in node.outputs.iter().enumerate() {
            let socket_pos = node_rect.right_top() + egui::vec2(0.0, 40.0 + socket_idx as f32 * 20.0 * zoom);
            painter.circle(
                socket_pos,
                6.0 * zoom,
                egui::Color32::DARK_GRAY,
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            );
            
            painter.text(
                socket_pos + egui::vec2(-10.0, 0.0),
                egui::Align2::RIGHT_CENTER,
                &socket.name,
                egui::FontId::proportional(14.0 * zoom),
                egui::Color32::WHITE,
            );
            
            let socket_rect = egui::Rect::from_center_size(socket_pos, egui::vec2(12.0, 12.0) * zoom);
            if ui.interact(socket_rect, ui.id().with("socket_out").with(node.id).with(socket_idx), egui::Sense::click()).clicked() {
                clicked_socket = Some((false, socket_idx));
            }
            
            // Draw in-progress connection
            if let Some((from_node, from_socket)) = connecting_from {
                if *from_node == node.id && *from_socket == socket_idx {
                    if let Some(mouse_pos) = ui.ctx().pointer_latest_pos() {
                        painter.line_segment(
                            [socket_pos, mouse_pos],
                            egui::Stroke::new(2.0, egui::Color32::YELLOW),
                        );
                    }
                }
            }
        }
        
        // Node interaction
        let response = ui.interact(node_rect, ui.id().with(node.id), egui::Sense::click_and_drag());
        
        Self {
            dragged: response.dragged(),
            clicked: response.clicked(),
            clicked_socket,
        }
    }
}

impl MenuEditorApp {
    fn draw_grid(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter();
        
        // Apply pan and zoom
        let grid_size = 20.0 * self.zoom;
        let offset_x = self.pan_offset.x % grid_size;
        let offset_y = self.pan_offset.y % grid_size;
        
        // Draw grid lines
        let grid_color = ui.style().visuals.widgets.noninteractive.bg_stroke.color.linear_multiply(0.4);
        
        for i in 0..((rect.width() / grid_size) as i32 + 2) {
            let x = rect.left() + offset_x + i as f32 * grid_size;
            painter.line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                egui::Stroke::new(1.0, grid_color),
            );
        }
        
        for i in 0..((rect.height() / grid_size) as i32 + 2) {
            let y = rect.top() + offset_y + i as f32 * grid_size;
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                egui::Stroke::new(1.0, grid_color),
            );
        }
    }
    
    fn draw_connection(&self, ui: &mut egui::Ui, connection: &mut Connection) {
        let painter = ui.painter();
        
        if let (Some(from), Some(to)) = (
            self.nodes.iter().find(|n| n.id == connection.from_node),
            self.nodes.iter().find(|n| n.id == connection.to_node)
        ) {
            let start_pos = from.pos + self.pan_offset + egui::vec2(
                from.size.x,
                40.0 + connection.from_socket as f32 * 20.0 * self.zoom,
            );
            
            let end_pos = to.pos + self.pan_offset + egui::vec2(
                0.0,
                40.0 + connection.to_socket as f32 * 20.0 * self.zoom,
            );
            
            match connection.connection_type {
                ConnectionType::Linear => {
                    // Draw orthogonal path with three segments
                    let cp1 = egui::pos2(
                        connection.control_point1.x + self.pan_offset.x,
                        connection.control_point1.y + self.pan_offset.y,
                    );
                    let cp2 = egui::pos2(
                        connection.control_point2.x + self.pan_offset.x,
                        connection.control_point2.y + self.pan_offset.y,
                    );
                    
                    // Draw the three segments
                    painter.line_segment(
                        [start_pos, cp1],
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                    painter.line_segment(
                        [cp1, cp2],
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                    painter.line_segment(
                        [cp2, end_pos],
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                    
                    // Draw control points for adjusting the path
                    let control_point_radius = 4.0 * self.zoom;
                    painter.circle_filled(cp1, control_point_radius, egui::Color32::YELLOW);
                    painter.circle_filled(cp2, control_point_radius, egui::Color32::YELLOW);
                    
                    // Handle control point dragging
                    let cp1_rect = egui::Rect::from_center_size(
                        cp1,
                        egui::vec2(control_point_radius * 2.0, control_point_radius * 2.0),
                    );
                    let cp2_rect = egui::Rect::from_center_size(
                        cp2,
                        egui::vec2(control_point_radius * 2.0, control_point_radius * 2.0),
                    );
                    
                    if ui.interact(cp1_rect, ui.id().with("cp1").with(connection.id), egui::Sense::drag()).dragged() {
                        let delta = ui.input(|i| i.pointer.delta());
                        connection.control_point1.x += delta.x; // Only allow horizontal movement
                    }
                    
                    if ui.interact(cp2_rect, ui.id().with("cp2").with(connection.id), egui::Sense::drag()).dragged() {
                        let delta = ui.input(|i| i.pointer.delta());
                        connection.control_point1.x += delta.x; // Move both points together
                        connection.control_point2.x = connection.control_point1.x;
                    }
                }
                ConnectionType::Bezier => {
                    // Original bezier curve drawing code
                    let cp1 = egui::pos2(
                        connection.control_point1.x + self.pan_offset.x,
                        connection.control_point1.y + self.pan_offset.y,
                    );
                    let cp2 = egui::pos2(
                        connection.control_point2.x + self.pan_offset.x,
                        connection.control_point2.y + self.pan_offset.y,
                    );
                    
                    painter.add(egui::Shape::CubicBezier(egui::epaint::CubicBezierShape::from_points_stroke(
                        [start_pos, cp1, cp2, end_pos],
                        false,
                        egui::Color32::TRANSPARENT,
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    )));
                    
                    // Draw control points and handles
                    let control_point_radius = 4.0 * self.zoom;
                    
                    // Draw lines to control points
                    painter.line_segment(
                        [start_pos, cp1],
                        egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 100)),
                    );
                    painter.line_segment(
                        [end_pos, cp2],
                        egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 100)),
                    );
                    
                    // Draw and handle interaction with control points
                    let cp1_rect = egui::Rect::from_center_size(
                        cp1,
                        egui::vec2(control_point_radius * 2.0, control_point_radius * 2.0),
                    );
                    let cp2_rect = egui::Rect::from_center_size(
                        cp2,
                        egui::vec2(control_point_radius * 2.0, control_point_radius * 2.0),
                    );
                    
                    // Draw control points
                    painter.circle_filled(cp1, control_point_radius, egui::Color32::YELLOW);
                    painter.circle_filled(cp2, control_point_radius, egui::Color32::YELLOW);
                    
                    // Handle control point dragging
                    let cp1_response = ui.interact(cp1_rect, ui.id().with("cp1").with(connection.id), egui::Sense::drag());
                    let cp2_response = ui.interact(cp2_rect, ui.id().with("cp2").with(connection.id), egui::Sense::drag());
                    
                    if cp1_response.dragged() {
                        connection.dragging_point = Some(0);
                        let delta = ui.input(|i| i.pointer.delta());
                        connection.control_point1 += delta;
                    } else if cp2_response.dragged() {
                        connection.dragging_point = Some(1);
                        let delta = ui.input(|i| i.pointer.delta());
                        connection.control_point2 += delta;
                    }
                    
                    // Reset dragging state when mouse is released
                    if !ui.input(|i| i.pointer.any_down()) {
                        connection.dragging_point = None;
                    }
                }
            }
        }
    }
}

// This is the entry-point for all the web-assembly stuff
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
                Box::new(|cc| Box::new(MenuEditorApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}
