pub struct ErrorLog {
    errors: Vec<String>,
}

impl ErrorLog {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn write_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    pub fn merge_log(&mut self, log: &Self) {
        self.errors.extend(log.errors.clone());
    }

    pub fn has_errors(&self) -> bool {
        return self.errors.len() == 0;
    }

    pub fn to_string(&self) -> String {
        self.errors.join("\n")
    }
}
