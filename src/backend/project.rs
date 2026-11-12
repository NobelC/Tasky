use crate::utilities::utils_backend;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum StatusProject {
    Onhold,
    Planned,
    Postponed,
    Completed,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Project {
    id: uuid::Uuid,
    name_project: String,
    author_project: String,
    creation_time: DateTime<Utc>,
    deadline_date: DateTime<Utc>,
    status: StatusProject,
}

#[allow(dead_code)]
impl Project {
    //The `new` method can only be accessed by the parent module,
    //preventing the creation of projects outside the structured data
    pub fn new(
        name: &str,
        author: &str,
        status_project: StatusProject,
        deadline: DateTime<Utc>,
    ) -> Project {
        Project {
            id: uuid::Uuid::new_v4(),
            name_project: name.to_string(),
            author_project: author.to_string(),
            status: status_project,
            creation_time: Utc::now(),
            deadline_date: deadline,
        }
    }

    //======================= Setters's Methods ========================
    //Setter methods are only visible to the parent method's functions
    //to prevent uncontrolled direct modification of project information.
    pub(super) fn set_project_name(&mut self, new_name: String) {
        self.name_project = new_name;
    }
    pub(super) fn set_project_author(&mut self, new_author: String) {
        self.author_project = new_author;
    }
    pub(super) fn set_new_deadline(&mut self, new_deadline: DateTime<Utc>) {
        self.deadline_date = new_deadline;
    }
    pub(super) fn set_new_status(&mut self, new_status: StatusProject) {
        self.status = new_status;
    }

    //======================= Getter's Methods =========================
    //Getter methods are public; they return raw data on their own,
    //and the parent module is responsible for formatting it into a readable output.
    pub fn get_project_name(&self) -> &str {
        &self.name_project
    }
    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }
    pub fn get_project_owner(&self) -> uuid::Uuid {
        self.id
    }
    pub fn get_task_name(&self) -> &str {
        &self.name_project
    }
    pub fn get_creation_date(&self) -> DateTime<Utc> {
        self.creation_time
    }
    pub fn get_deadline(&self) -> DateTime<Utc> {
        self.deadline_date
    }
    pub fn get_status_project(&self) -> StatusProject {
        self.status
    }
    //======================== Format Getter's Methods ================================
    pub fn get_format_date(&self, date: utils_backend::TypeDate) -> String {
        match date {
            utils_backend::TypeDate::Creation => utils_backend::format_date(self.creation_time),
            utils_backend::TypeDate::Deadline => utils_backend::format_date(self.deadline_date),
        }
    }
    pub fn get_status_format(status: StatusProject) -> String {
        match status {
            StatusProject::Onhold => "On-hold".to_string(),
            StatusProject::Planned => "Planned".to_string(),
            StatusProject::Postponed => "Postponed".to_string(),
            StatusProject::Completed => "Completed".to_string(),
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            creation_time: Utc::now(),
            author_project: String::new(),
            name_project: String::new(),
            deadline_date: Utc::now(),
            status: StatusProject::Planned,
        }
    }
}
