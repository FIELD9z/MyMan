use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Note,
    Task,
    Event,
    Knowledge,
    File,
}

impl EntityType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Task => "task",
            Self::Event => "event",
            Self::Knowledge => "knowledge",
            Self::File => "file",
        }
    }

    pub fn from_db_value(value: &str) -> Result<Self, EntityTypeParseError> {
        match value {
            "note" => Ok(Self::Note),
            "task" => Ok(Self::Task),
            "event" => Ok(Self::Event),
            "knowledge" => Ok(Self::Knowledge),
            "file" => Ok(Self::File),
            _ => Err(EntityTypeParseError {
                value: value.to_owned(),
            }),
        }
    }
}

impl fmt::Display for EntityType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub struct EntityTypeParseError {
    value: String,
}

impl fmt::Display for EntityTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Invalid entity type: {}", self.value)
    }
}

impl std::error::Error for EntityTypeParseError {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntityRequest {
    pub entity_type: EntityType,
    pub title: String,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntityRequest {
    pub id: String,
    pub title: String,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEntitiesRequest {
    pub entity_type: Option<EntityType>,
    pub tag: Option<String>,
    #[serde(default)]
    pub archived: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchMode {
    #[default]
    And,
    Or,
}

impl SearchMode {
    pub fn operator(self) -> &'static str {
        match self {
            Self::And => " AND ",
            Self::Or => " OR ",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchEntitiesRequest {
    pub query: String,
    #[serde(default)]
    pub search_mode: SearchMode,
    pub entity_type: Option<EntityType>,
    pub tag: Option<String>,
    #[serde(default)]
    pub archived: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    pub entity_type: EntityType,
    pub title: String,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityPage {
    pub items: Vec<Entity>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSummary {
    pub notes: i64,
    pub tasks: i64,
    pub events: i64,
    pub knowledge: i64,
    pub files: i64,
    pub reminders_due_today: i64,
}
