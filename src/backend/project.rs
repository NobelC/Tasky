use std::os::unix::raw::uid_t;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum StatusTag {
    Onhold,
    Planned,
    Postponed,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Project {
    id: uuid::Uuid,
    name: String,
    author: String,
    creation_time: DateTime<Utc>,
    status: StatusTag,
}

impl Project {
    fn calculate_creation_date_time() -> DateTime<Utc> {
        Utc::now()
    }
    fn create_uuid() -> uuid::Uuid {
        uuid::Uuid::new_v4()
    }

    pub fn new(name: &str, author: &str) -> Project {
        let new_project = Project {
            id: Self::create_uuid(),
            name: name.to_string(),
            author: author.to_string(),
            status: StatusTag::Planned,
            creation_time: Self::calculate_creation_date_time(),
        };
        new_project
    }
    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }
}
