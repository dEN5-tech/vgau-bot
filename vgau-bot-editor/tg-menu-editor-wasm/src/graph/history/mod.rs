use eframe::egui;
use std::collections::VecDeque;
use crate::graph::models::{Node, Connection};

/// Represents a snapshot of the graph state for undo/redo operations
#[derive(Clone)]
pub struct GraphSnapshot {
    pub(crate) nodes: Vec<Node>,
    pub(crate) connections: Vec<Connection>,
    pub(crate) active_node: Option<usize>,
    pub(crate) drag_offset: egui::Vec2,
    pub(crate) zoom: f32,
}

impl GraphSnapshot {
    /// Create a new snapshot of the graph state
    pub fn new(
        nodes: Vec<Node>,
        connections: Vec<Connection>,
        active_node: Option<usize>,
        drag_offset: egui::Vec2,
        zoom: f32,
    ) -> Self {
        Self {
            nodes,
            connections,
            active_node,
            drag_offset,
            zoom,
        }
    }
}

/// Manages the history of graph states for undo/redo operations
pub struct HistoryManager {
    /// History of snapshots for undo
    history: VecDeque<GraphSnapshot>,
    /// History of snapshots for redo
    redo_stack: VecDeque<GraphSnapshot>,
    /// Maximum number of snapshots to keep in history
    history_size_limit: usize,
    /// Flag to indicate if a history action is in progress
    action_in_progress: bool,
}

impl HistoryManager {
    /// Create a new history manager with the given size limit
    pub fn new(size_limit: usize) -> Self {
        Self {
            history: VecDeque::new(),
            redo_stack: VecDeque::new(),
            history_size_limit: size_limit,
            action_in_progress: false,
        }
    }
    
    /// Add a snapshot to the history
    pub fn add_snapshot(&mut self, snapshot: GraphSnapshot) {
        if self.action_in_progress {
            return;
        }
        
        // Clear redo stack when new action is performed
        self.redo_stack.clear();
        
        // Add the new snapshot to history
        self.history.push_back(snapshot);
        
        // Trim history if it exceeds the size limit
        if self.history.len() > self.history_size_limit {
            self.history.pop_front();
        }
    }
    
    /// Undo the last action and return the previous snapshot
    pub fn undo(&mut self) -> Option<GraphSnapshot> {
        if self.action_in_progress || self.history.is_empty() {
            return None;
        }
        
        self.action_in_progress = true;
        
        // Get the current state from history
        if let Some(current) = self.history.pop_back() {
            // Add it to redo stack
            self.redo_stack.push_back(current);
            
            // Return the previous state (now the latest in history)
            let result = self.history.back().cloned();
            
            self.action_in_progress = false;
            return result;
        }
        
        self.action_in_progress = false;
        None
    }
    
    /// Redo the last undone action and return the next snapshot
    pub fn redo(&mut self) -> Option<GraphSnapshot> {
        if self.action_in_progress || self.redo_stack.is_empty() {
            return None;
        }
        
        self.action_in_progress = true;
        
        // Get the next state from redo stack
        if let Some(next) = self.redo_stack.pop_back() {
            // Add it to history
            self.history.push_back(next.clone());
            
            // Trim history if needed
            if self.history.len() > self.history_size_limit {
                self.history.pop_front();
            }
            
            self.action_in_progress = false;
            return Some(next);
        }
        
        self.action_in_progress = false;
        None
    }
    
    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.history.len() > 1
    }
    
    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
    
    /// Initialize the history with the first snapshot
    pub fn initialize(&mut self, snapshot: GraphSnapshot) {
        self.history.clear();
        self.redo_stack.clear();
        self.history.push_back(snapshot);
        self.action_in_progress = false;
    }
} 