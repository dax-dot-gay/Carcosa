use chrono::Utc;
use native_model::Model;
use native_db::{ native_db, ToKey };
use native_model::native_model;
use serde::{ Deserialize, Serialize };
use specta::Type;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Type)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct ProjectConfiguration {
    #[primary_key]
    #[specta(skip)]
    #[serde(default = "ProjectConfiguration::default_id")]
    pub id: String,
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub current_tabs: Vec<String>,

    pub created_on: chrono::DateTime<Utc>,
    pub last_access: chrono::DateTime<Utc>,
}

impl ProjectConfiguration {
    pub fn new(name: impl AsRef<str>, description: Option<impl AsRef<str>>) -> Self {
        Self {
            id: "CONFIG".to_string(),
            name: name.as_ref().to_string(),
            description: description.and_then(|d| Some(d.as_ref().to_string())),
            current_tabs: Vec::new(),
            created_on: Utc::now(),
            last_access: Utc::now(),
        }
    }

    pub(self) fn default_id() -> String {
        "CONFIG".to_string()
    }
}
