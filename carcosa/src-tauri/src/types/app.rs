use std::{path::PathBuf, sync::Arc};

use getset::{CloneGetters, WithSetters};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::types::ProjectSettings;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Type, Default)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ActiveProject {
    #[default]
    None,
    Local {
        path: PathBuf,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, Type, CloneGetters, WithSetters)]
#[getset(get_clone = "pub", set = "pub", set_with = "pub")]
pub struct ApplicationState {
    active_project: ActiveProject,
    project_settings: Option<ProjectSettings>,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            active_project: ActiveProject::default(),
            project_settings: None,
        }
    }
}

pub type ApplicationStateWrapper = Arc<RwLock<ApplicationState>>;
