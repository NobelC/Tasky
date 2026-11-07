#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    title: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            title: "Tasky".to_string(),
        }
    }

    pub fn get_state(&mut self) -> bool {
        self.exit
    }
    pub fn get_title(&mut self) -> String {
        self.title.clone()
    }
    pub fn set_new_state(&mut self) {
        self.exit = !self.exit;
    }
    pub fn set_new_title(&mut self) {
        self.title = "TASKY".to_string();
    }
}
