use std::path::PathBuf;

use tauri::Runtime;
use tokio::fs;

use crate::{carcosa::CarcosaExt, models::ProjectConfiguration};

#[taurpc::ipc_type]
pub struct CreateProjectModel {
    pub path: String,
    pub name: String,

    #[serde(default)]
    pub description: Option<String>
}

#[taurpc::procedures(path = "application")]
pub trait ApplicationApi {
    async fn create_project<R: Runtime>(app_handle: tauri::AppHandle<R>, project: CreateProjectModel) -> Result<ProjectConfiguration, crate::SerializableError>;
    async fn open_project<R: Runtime>(app_handle: tauri::AppHandle<R>, path: String) -> Result<ProjectConfiguration, crate::SerializableError>;
}

#[derive(Clone)]
pub struct ApplicationApiImpl;

#[taurpc::resolvers]
impl ApplicationApi for ApplicationApiImpl {
    async fn create_project<R: Runtime>(self, app_handle: tauri::AppHandle<R>, project: CreateProjectModel) -> Result<ProjectConfiguration, crate::SerializableError> {
        let path = PathBuf::from(project.path.clone());
        let carcosa = app_handle.carcosa();
        if path.exists() {
            if path.is_dir() {
                if path.read_dir()?.next().is_some() {
                    return Err(crate::Error::NonEmptyProjectFolder(project.path).into());
                }
            } else {
                return Err(crate::Error::ExpectedProjectDirectory(project.path).into());
            }
        }

        fs::create_dir_all(path).await?;
        carcosa.set_state("current_project", project.path.clone())?;
        let database = carcosa.current_database()?;
        let txn = database.rw_transaction()?;
        let config = ProjectConfiguration::new(project.name.clone(), project.description.clone());
        txn.insert(config.clone())?;
        txn.commit()?;

        Ok(config)
    }

    async fn open_project<R: Runtime>(self, app_handle: tauri::AppHandle<R>, path: String) -> Result<ProjectConfiguration, crate::SerializableError> {
        let realpath = PathBuf::from(path.clone());
        let carcosa = app_handle.carcosa();
        if !realpath.is_dir() {
            return Err(crate::Error::InvalidProjectSelection(path).into());
        }

        if !realpath.join("project.db").exists() {
            return Err(crate::Error::InvalidProjectSelection(path).into());
        }

        carcosa.set_state("current_project", path.clone())?;
        let db = carcosa.current_database()?;
        let txn = db.r_transaction()?;
        if let Some(config) = txn.get().primary::<ProjectConfiguration>("CONFIG")? {
            Ok(config)
        } else {
            Err(crate::Error::CorruptedProject(path, "Missing project configuration in database.".into()).into())
        }
    }
}
