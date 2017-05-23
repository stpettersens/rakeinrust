#[derive(Debug)]
pub struct Task {
    name: String,
    command: String,
    params: String,
}

impl Task {
    pub fn new(name: &str, command: &str, params: &str) -> Task {
        Task {
            name: name.to_owned(),
            command: command.to_owned(),
            params: params.to_owned(),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_command(&self) -> &str {
        &self.command
    }
    pub fn get_params(&self) -> &str {
        &self.params
    }
}
