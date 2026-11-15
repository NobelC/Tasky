pub mod calculate_status;
pub mod convert_date_nvd_utc;
pub mod format_date;
use super::project::{Project, StatusProject};
use super::task::{Task, TimeStatusTask};

#[derive(Debug, Default)]
pub struct ProjectManager {
    project: Vec<Project>,
    task: Vec<Task>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            project: Vec::new(),
            task: Vec::new(),
        }
    }
    //============ Getter's collection =================
    pub fn get_collection_project_read(&self) -> &[Project] {
        &self.project
    }
    pub fn get_collection_task_read(&self) -> &[Task] {
        &self.task
    }
    // =================== Setter's Information ==================
    pub fn setter_new_project(&mut self, new_data: (String, String, String)) {
        let mut new_project = Project::default();
        new_project.set_project_name("Hol".to_string());
        //Parse date (DD-MM-YYYY)
    }
    pub fn setter_new_task(&mut self, new_task: Task) {
        self.task.push(new_task);
    }
}
