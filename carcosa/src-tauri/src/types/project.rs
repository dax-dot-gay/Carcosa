use std::{collections::HashMap, fs, path::Path};

use getset::{CloneGetters, WithSetters};
use serde::{Deserialize, Serialize};

use crate::types::{NetworkIdentity, PeerIdentity};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectCollaborator {
    pub identity: PeerIdentity,
    pub name: String,

    #[serde(default)]
    pub can_edit: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, CloneGetters, WithSetters)]
#[getset(get_clone = "pub")]
pub struct ProjectSettings {
    #[getset(set_with = "pub")]
    name: String,

    #[serde(default)]
    #[getset(set_with = "pub")]
    identity: NetworkIdentity,

    #[serde(default)]
    collaborators: HashMap<PeerIdentity, ProjectCollaborator>,
}

impl ProjectSettings {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            identity: NetworkIdentity::generate(),
            collaborators: HashMap::new(),
        }
    }

    pub fn load(project: impl AsRef<Path>) -> crate::Result<Self> {
        let project = project.as_ref().join("project.json");
        let settings_content = fs::read_to_string(project)?;
        Ok(serde_json::from_str::<Self>(&settings_content)?)
    }

    pub fn with_collaborator(mut self, collaborator: ProjectCollaborator) -> Self {
        let _ = self
            .collaborators
            .insert(collaborator.identity.clone(), collaborator);
        self
    }

    pub fn remove_collaborator(mut self, collaborator: PeerIdentity) -> Self {
        let _ = self.collaborators.remove(&collaborator);
        self
    }

    pub fn collaborator(&self, collaborator: PeerIdentity) -> Option<ProjectCollaborator> {
        self.collaborators.get(&collaborator).cloned()
    }

    pub fn save(self, project: impl AsRef<Path>) -> crate::Result<Self> {
        let project = project.as_ref().join("project.json");
        let settings_content = serde_json::to_string_pretty(&self.clone())?;
        fs::write(project, settings_content)?;
        Ok(self)
    }
}
