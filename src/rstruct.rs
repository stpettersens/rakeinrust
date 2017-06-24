#[derive(Debug, Clone)]
pub struct Struct {
    name: String,
    fields: Vec<String>,
    values: Vec<String>,
}

impl Struct {
    pub fn new(name: &str, fields: Vec<String>) -> Struct {
        Struct {
            name: name.to_owned(),
            fields: fields,
            values: Vec::new(),
        }
    }
    /*pub fn append_field(&mut self, field: &str) {
        self.fields.push(field.to_owned());
    }*/
    pub fn set_values(&mut self, values: Vec<String>) {
        self.values = values;
    }

    /* Corresponding to Ruby API for a Struct: */
    pub fn to_json(&self) -> String {
        let mut json: Vec<String> = Vec::new();
        for f in &self.fields {
            for v in &self.values {
                json.push(format!("{}: {}", f, v));
            }
        }
        json.insert(0, "{".to_owned());
        json.push("}".to_owned());
        json.join("")
    }
}
