#[derive(Debug, Clone)]
pub struct Task {
    name: String,
    depends: String,
    command: String,
    params: String,
    line: usize,
}

impl Task {
    pub fn new(name: &str, depends: &str, 
    command: &str, params: &str, line: usize) -> Task {
        Task {
            name: name.to_owned(),
            depends: depends.to_owned(),
            command: command.to_owned(),
            params: params.to_owned(),
            line: line,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_depends(&self) -> &str {
        &self.depends
    }
    pub fn get_command(&self) -> &str {
        &self.command
    }
    pub fn get_params(&self) -> &str {
        &self.params
    }
    pub fn get_line(&self) -> usize {
        self.line
    }
}
