use crate::utilities::utils_backend;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum TimeStatusTask {
    Overdue,
    AboutToExpire,
    OnTime,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Task {
    id: uuid::Uuid,
    project_owner: uuid::Uuid,
    task_name: String,
    creation_date: DateTime<Utc>,
    deadline: DateTime<Utc>,
    time_spent: u32,
    status: TimeStatusTask,
}

impl Task {
    //The `new` method can only be accessed by the parent module,
    //preventing the creation of task outside the structured data
    pub(super) fn new(
        id_project: uuid::Uuid,
        name: &str,
        dead_time: DateTime<Utc>,
        actual_status_time: TimeStatusTask,
    ) -> Task {
        Self {
            id: Self::create_id(),
            project_owner: id_project,
            task_name: name.to_string(),
            creation_date: Self::creation_date(),
            deadline: dead_time,
            time_spent: 0,
            status: actual_status_time,
        }
    }
    fn create_id() -> uuid::Uuid {
        uuid::Uuid::new_v4()
    }
    fn creation_date() -> DateTime<Utc> {
        Utc::now()
    }

    //======================= Setters's Methods ========================
    //Setter methods are only visible to the parent method's functions
    //to prevent uncontrolled direct modification of task information.
    pub(super) fn set_project_owner(&mut self, id: uuid::Uuid) {
        self.project_owner = id;
    }
    pub(super) fn set_task_name(&mut self, new_name: &str) {
        self.task_name = new_name.to_string();
    }
    pub(super) fn set_new_deadline(&mut self, new_deadline: DateTime<Utc>) {
        self.deadline = new_deadline;
    }
    pub(super) fn set_new_status(&mut self, new_status: TimeStatusTask) {
        self.status = new_status;
    }

    //======================= Getter's Methods =========================
    //Getter methods are public; they return raw data on their own,
    //and the parent module is responsible for formatting it into a readable output.
    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }
    pub fn get_project_owner(&self) -> uuid::Uuid {
        self.project_owner
    }
    pub fn get_task_name(&self) -> &str {
        &self.task_name
    }
    pub fn get_creation_date(&self) -> DateTime<Utc> {
        self.creation_date
    }
    pub fn get_deadline(&self) -> DateTime<Utc> {
        self.deadline
    }
    pub fn get_time_spent(&self) -> u32 {
        self.time_spent
    }
    pub fn get_status_time(&self) -> TimeStatusTask {
        self.status
    }
    //=================== Format Getter's Methods =========================
    pub fn get_format_date(&self, date: utils_backend::TypeDate) -> String {
        match date {
            utils_backend::TypeDate::Creation => utils_backend::format_date(self.creation_date),
            utils_backend::TypeDate::Deadline => utils_backend::format_date(self.deadline),
        }
    }
}
