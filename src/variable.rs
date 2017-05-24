#[derive(Debug, Clone)]
pub struct Variable {
    key: String,
    value: String, 
}

impl Variable {
    pub fn new(key: &str, value: &str) -> Variable {
        Variable {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }
    pub fn get_key(&self) -> String {
        format!("{}", &self.key)
    }
    pub fn get_value(&self) -> String {
        format!("{}", &self.value)
    }
}
