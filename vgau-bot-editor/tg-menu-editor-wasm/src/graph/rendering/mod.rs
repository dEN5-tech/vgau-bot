use eframe::egui;
use crate::graph::models::{Node, Connection};

/// Handles rendering of nodes, connections, and other UI elements in the graph
pub struct GraphRenderer;

impl GraphRenderer {
    /// Draw a connection between two points with a bezier curve
    pub fn draw_bezier_connection(
        ui: &mut egui::Ui,
        from: egui::Pos2,
        to: egui::Pos2,
        color: egui::Color32
    ) {
        // Get the current zoom level from the graph
        let zoom = ui.memory(|mem| mem.data.get_temp::<f32>(egui::Id::new("graph_zoom")).unwrap_or(1.0));
        
        // Determine if this is a vertical connection (top to bottom)
        let is_vertical = (to.y - from.y).abs() > (to.x - from.x).abs();
        
        // Calculate control points based on connection direction
        let control_distance = if is_vertical {
            // For vertical connections (like menu hierarchies)
            (to.y - from.y).abs() * 0.5
        } else {
            // For horizontal connections
            (to.x - from.x).abs() * 0.5
        };
        
        // Ensure minimum control distance
        let control_distance = control_distance.max(50.0 * zoom);
        
        // Create control points
        let from_cp = if is_vertical {
            egui::pos2(from.x, from.y + control_distance)
        } else {
            egui::pos2(from.x + control_distance, from.y)
        };
        
        let to_cp = if is_vertical {
            egui::pos2(to.x, to.y - control_distance)
        } else {
            egui::pos2(to.x - control_distance, to.y)
        };
        
        // Draw the bezier curve with scaled stroke width
        ui.painter().add(egui::Shape::CubicBezier(
            egui::epaint::CubicBezierShape::from_points_stroke(
                [from, from_cp, to_cp, to],
                false,
                egui::Color32::TRANSPARENT,
                egui::Stroke::new(2.0 * zoom, color),
            )
        ));
    }
    
    /// Draw a grid in the background
    pub fn draw_grid(ui: &mut egui::Ui, rect: egui::Rect, zoom: f32) {
        let grid_color_major = egui::Color32::from_gray(60);
        let grid_color_minor = egui::Color32::from_gray(40);
        
        let grid_spacing_major = 100.0 * zoom;
        let grid_spacing_minor = 25.0 * zoom;
        
        // Minor grid lines
        let mut x = rect.min.x - (rect.min.x % grid_spacing_minor);
        while x < rect.max.x {
            let line = [
                egui::pos2(x, rect.min.y),
                egui::pos2(x, rect.max.y),
            ];
            ui.painter().add(egui::Shape::line_segment(
                line,
                egui::Stroke::new(1.0, grid_color_minor),
            ));
            x += grid_spacing_minor;
        }
        
        let mut y = rect.min.y - (rect.min.y % grid_spacing_minor);
        while y < rect.max.y {
            let line = [
                egui::pos2(rect.min.x, y),
                egui::pos2(rect.max.x, y),
            ];
            ui.painter().add(egui::Shape::line_segment(
                line,
                egui::Stroke::new(1.0, grid_color_minor),
            ));
            y += grid_spacing_minor;
        }
        
        // Major grid lines
        let mut x = rect.min.x - (rect.min.x % grid_spacing_major);
        while x < rect.max.x {
            let line = [
                egui::pos2(x, rect.min.y),
                egui::pos2(x, rect.max.y),
            ];
            ui.painter().add(egui::Shape::line_segment(
                line,
                egui::Stroke::new(1.0, grid_color_major),
            ));
            x += grid_spacing_major;
        }
        
        let mut y = rect.min.y - (rect.min.y % grid_spacing_major);
        while y < rect.max.y {
            let line = [
                egui::pos2(rect.min.x, y),
                egui::pos2(rect.max.x, y),
            ];
            ui.painter().add(egui::Shape::line_segment(
                line,
                egui::Stroke::new(1.0, grid_color_major),
            ));
            y += grid_spacing_major;
        }
    }
    
    /// Draw a node with its inputs and outputs
    pub fn draw_node(
        ui: &mut egui::Ui,
        node: &Node,
        is_active: bool,
        response: &mut Option<egui::Response>,
    ) {
        // Get the current zoom level from the graph
        let zoom = ui.memory(|mem| mem.data.get_temp::<f32>(egui::Id::new("graph_zoom")).unwrap_or(1.0));
        
        // Scale node size and position based on zoom
        let scaled_size = node.size * zoom;
        let scaled_position = egui::pos2(
            node.position.x * zoom,
            node.position.y * zoom
        );
        
        // Calculate node rectangle with scaled values
        let node_rect = egui::Rect::from_min_size(scaled_position, scaled_size);
        
        // Decide colors based on node type and active state
        let bg_color = if is_active {
            // Brighter version of the node color
            let mut color = node.color;
            color = egui::Color32::from_rgba_premultiplied(
                color.r().saturating_add(40),
                color.g().saturating_add(40),
                color.b().saturating_add(40),
                color.a(),
            );
            color
        } else {
            node.color
        };
        
        let frame_color = if is_active {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(180)
        };
        
        // Draw node background
        ui.painter().add(egui::Shape::rect_filled(
            node_rect,
            8.0 * zoom, // Scale corner radius with zoom
            bg_color,
        ));
        
        // Draw node frame
        ui.painter().add(egui::Shape::rect_stroke(
            node_rect,
            8.0 * zoom, // Scale corner radius with zoom
            egui::Stroke::new(2.0 * zoom, frame_color), // Scale stroke width with zoom
        ));
        
        // Draw node title
        let title_height = 24.0 * zoom;
        let title_rect = egui::Rect::from_min_size(
            node_rect.min,
            egui::vec2(node_rect.width(), title_height),
        );
        
        ui.painter().add(egui::Shape::rect_filled(
            title_rect,
            egui::Rounding {
                nw: 8.0 * zoom,
                ne: 8.0 * zoom,
                sw: 0.0,
                se: 0.0,
            },
            egui::Color32::from_rgba_premultiplied(
                0, 0, 0, 100
            ),
        ));
        
        // Node title text
        let title_pos = title_rect.center();
        ui.painter().text(
            title_pos,
            egui::Align2::CENTER_CENTER,
            node.get_title(),
            egui::FontId::proportional(14.0 * zoom), // Scale font size with zoom
            egui::Color32::WHITE,
        );
        
        // Draw input ports
        let port_radius = 6.0 * zoom;
        for (i, port) in node.inputs.iter().enumerate() {
            if node.inputs.len() > 0 {
                let port_spacing = node_rect.width() / (node.inputs.len() as f32 + 1.0);
                let port_x = node_rect.min.x + port_spacing * (i as f32 + 1.0);
                let port_y = node_rect.min.y;
                
                // Draw port circle
                ui.painter().add(egui::Shape::circle_filled(
                    egui::pos2(port_x, port_y),
                    port_radius,
                    port.color,
                ));
                
                // Draw port label
                ui.painter().text(
                    egui::pos2(port_x, port_y - 15.0 * zoom),
                    egui::Align2::CENTER_CENTER,
                    &port.label,
                    egui::FontId::proportional(10.0 * zoom), // Scale font size with zoom
                    egui::Color32::WHITE,
                );
                
                // Add interaction for port (for future use)
                let port_rect = egui::Rect::from_center_size(
                    egui::pos2(port_x, port_y),
                    egui::vec2(port_radius * 2.0, port_radius * 2.0),
                );
                
                let port_id = ui.id().with((node.id(), "input", port.id()));
                ui.interact(port_rect, port_id, egui::Sense::click());
            }
        }
        
        // Draw output ports
        for (i, port) in node.outputs.iter().enumerate() {
            if node.outputs.len() > 0 {
                let port_spacing = node_rect.width() / (node.outputs.len() as f32 + 1.0);
                let port_x = node_rect.min.x + port_spacing * (i as f32 + 1.0);
                let port_y = node_rect.max.y;
                
                // Draw port circle
                ui.painter().add(egui::Shape::circle_filled(
                    egui::pos2(port_x, port_y),
                    port_radius,
                    port.color,
                ));
                
                // Draw port label
                ui.painter().text(
                    egui::pos2(port_x, port_y + 15.0 * zoom),
                    egui::Align2::CENTER_CENTER,
                    &port.label,
                    egui::FontId::proportional(10.0 * zoom), // Scale font size with zoom
                    egui::Color32::WHITE,
                );
                
                // Add interaction for port (for future use)
                let port_rect = egui::Rect::from_center_size(
                    egui::pos2(port_x, port_y),
                    egui::vec2(port_radius * 2.0, port_radius * 2.0),
                );
                
                let port_id = ui.id().with((node.id(), "output", port.id()));
                ui.interact(port_rect, port_id, egui::Sense::click());
            }
        }
        
        // Add interactive response for the whole node
        let node_sense = egui::Sense::click_and_drag();
        *response = Some(ui.interact(node_rect, ui.id().with(node.id()), node_sense));
    }
    
    /// Calculate the position of a port on a node
    pub fn get_port_position(
        node: &Node,
        port_id: &str,
        is_input: bool
    ) -> Option<egui::Pos2> {
        let node_rect = egui::Rect::from_min_size(node.position, node.size);
        
        let ports = if is_input { &node.inputs } else { &node.outputs };
        let port_index = ports.iter().position(|p| p.id() == port_id)?;
        let port_count = ports.len();
        
        if port_count == 0 {
            return None;
        }
        
        // Calculate port position
        let port_spacing = node_rect.width() / (port_count as f32 + 1.0);
        let port_y = if is_input {
            node_rect.min.y // Top of node
        } else {
            node_rect.max.y // Bottom of node
        };
        
        let port_x = node_rect.min.x + port_spacing * (port_index as f32 + 1.0);
        
        Some(egui::pos2(port_x, port_y))
    }
    
    /// Draw a connection between two nodes
    pub fn draw_connection(
        ui: &mut egui::Ui,
        connection: &Connection,
        nodes: &[Node],
    ) {
        // Get the current zoom level from the graph
        let zoom = ui.memory(|mem| mem.data.get_temp::<f32>(egui::Id::new("graph_zoom")).unwrap_or(1.0));
        
        // Find the nodes
        let from_node = nodes.iter().find(|n| n.id() == connection.from_node());
        let to_node = nodes.iter().find(|n| n.id() == connection.to_node());
        
        if let (Some(from_node), Some(to_node)) = (from_node, to_node) {
            // Calculate port positions
            let from_pos = Self::get_port_position(from_node, connection.from_port(), false);
            let to_pos = Self::get_port_position(to_node, connection.to_port(), true);
            
            if let (Some(from_pos), Some(to_pos)) = (from_pos, to_pos) {
                // Scale positions by zoom
                let from_pos = egui::pos2(
                    from_pos.x * zoom,
                    from_pos.y * zoom
                );
                let to_pos = egui::pos2(
                    to_pos.x * zoom,
                    to_pos.y * zoom
                );
                
                // Determine color based on port types
                let from_port = from_node.outputs.iter().find(|p| p.id() == connection.from_port());
                let color = if let Some(port) = from_port {
                    match port.port_type() {
                        crate::graph::models::PortType::String => egui::Color32::from_rgb(200, 200, 100),
                        crate::graph::models::PortType::Number => egui::Color32::from_rgb(100, 200, 200),
                        crate::graph::models::PortType::Object => egui::Color32::from_rgb(150, 200, 255),
                        crate::graph::models::PortType::Action => egui::Color32::from_rgb(255, 150, 150),
                    }
                } else {
                    egui::Color32::WHITE
                };
                
                // Draw bezier connection with scaled stroke width
                Self::draw_bezier_connection(ui, from_pos, to_pos, color);
            }
        }
    }
} 