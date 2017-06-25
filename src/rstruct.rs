#[derive(Debug, Clone)]
pub struct Struct {
    variable: String,
    name: String,
    fields: Vec<String>,
    values: Vec<String>,
}

impl Struct {
    pub fn new(name: &str, fields: Vec<String>) -> Struct {
        Struct {
            variable: String::new(),
            name: name.to_owned(),
            fields: fields,
            values: Vec::new(),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_variable(&mut self, variable: &str) {
        self.variable = variable.to_owned();
    }
    pub fn get_variable(&self) -> &str {
        &self.variable
    }
    pub fn set_values(&mut self, values: Vec<String>) {
        self.values = values;
    }

    /* Corresponding to Ruby API for a Struct: */
    pub fn to_json(&self) -> String {
        let mut json: Vec<String> = vec!["{".to_owned()];
        let mut i: usize = 0;
        for f in &self.fields {
            let mut comma = ",";
            if i == &self.fields.len() - 1 {
                comma = "";
            }
            json.push(format!("\"{}\":{}{}",
            &f[1..], &self.values[i], comma));
            i += 1;
        }
        json.push("}".to_owned());
        json.join("")
    }
}
