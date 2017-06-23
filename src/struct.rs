#[derive(Debug, Clone)]
pub struct Struct {
    fields: Vec<&str>,
    values: Vec<&str>,
}

impl Struct {
    pub fn new(fields: Vec<&str>, values: Vec<&str>) -> Struct {
        Struct {
            fields: fields,
            values: values,
        }
    }
    pub fn to_json(&self) -> &str {
        let mut json: Vec<&str> = Vec::new();
        for f in fields {
            for v in values {
                json.push(&format!("{}:{}", f, v));
            }
        }
        json.insert(0, "{\n");
        json.push("}\n");
        json.join("")
    }
}
