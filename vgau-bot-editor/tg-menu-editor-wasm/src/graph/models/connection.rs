/// Represents a connection between two nodes in the graph
#[derive(Clone)]
pub struct Connection {
    pub(crate) from_node: usize,
    pub(crate) to_node: usize,
    pub(crate) from_port: String,
    pub(crate) to_port: String,
}

impl Connection {
    /// Create a new connection between two nodes
    pub fn new(from_node: usize, from_port: String, to_node: usize, to_port: String) -> Self {
        Self {
            from_node,
            to_node,
            from_port,
            to_port,
        }
    }
    
    /// Get the source node id
    pub fn from_node(&self) -> usize {
        self.from_node
    }
    
    /// Get the target node id
    pub fn to_node(&self) -> usize {
        self.to_node
    }
    
    /// Get the source port id
    pub fn from_port(&self) -> &str {
        &self.from_port
    }
    
    /// Get the target port id
    pub fn to_port(&self) -> &str {
        &self.to_port
    }
} 