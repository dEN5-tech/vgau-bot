// Parameter types and values for nodes

#[derive(Clone)]
pub struct Parameter {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) param_type: ParameterType,
    pub(crate) value: ParameterValue,
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

impl Parameter {
    /// Create a new parameter with the given id, label, type, and value
    pub fn new(id: String, label: String, param_type: ParameterType, value: ParameterValue) -> Self {
        Self {
            id,
            label,
            param_type,
            value,
        }
    }
    
    /// Get the parameter id which also serves as its kind/type identifier
    pub fn kind(&self) -> &str {
        &self.id
    }
    
    /// Get the text value of the parameter, regardless of its actual type
    pub fn get_text(&self) -> String {
        match &self.value {
            ParameterValue::Text(text) => text.clone(),
            ParameterValue::Number(num) => num.to_string(),
            ParameterValue::Boolean(b) => b.to_string(),
            ParameterValue::Select(selected, _) => selected.clone(),
        }
    }
    
    /// Set the text value of the parameter
    pub fn set_text(&mut self, text: String) {
        match &mut self.value {
            ParameterValue::Text(ref mut value) => *value = text,
            ParameterValue::Number(ref mut value) => {
                if let Ok(num) = text.parse::<f64>() {
                    *value = num;
                }
            },
            ParameterValue::Boolean(ref mut value) => {
                if let Ok(b) = text.parse::<bool>() {
                    *value = b;
                }
            },
            ParameterValue::Select(ref mut selected, _) => *selected = text,
        }
    }
    
    /// Get the label of the parameter
    pub fn label(&self) -> &str {
        &self.label
    }
    
    /// Get the type of the parameter
    pub fn param_type(&self) -> &ParameterType {
        &self.param_type
    }
    
    /// Get a reference to the value of the parameter
    pub fn value(&self) -> &ParameterValue {
        &self.value
    }
    
    /// Get the parameter ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Set parameter value as text
    pub fn set_text_value(&mut self, value: String) {
        match &mut self.value {
            ParameterValue::Text(text) => *text = value,
            ParameterValue::Number(_) => {
                if let Ok(num) = value.parse::<f64>() {
                    self.value = ParameterValue::Number(num);
                } else {
                    // If it's not a valid number, treat as text
                    self.value = ParameterValue::Text(value);
                }
            },
            ParameterValue::Boolean(_) => {
                if let Ok(b) = value.parse::<bool>() {
                    self.value = ParameterValue::Boolean(b);
                } else {
                    // If it's not a valid boolean, treat as text
                    self.value = ParameterValue::Text(value);
                }
            },
            ParameterValue::Select(selected, options) => {
                if options.contains(&value) {
                    *selected = value;
                } else {
                    // If not in options, just update the text
                    self.value = ParameterValue::Text(value);
                }
            },
        }
    }
} 