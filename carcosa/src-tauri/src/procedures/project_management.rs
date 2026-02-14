use std::path::PathBuf;

use tauri::{AppHandle, Runtime};

use crate::{
    extensions::{ApplicationExt, DatabasesExt},
    types::{ActiveProject, ProjectSettings},
    MetaError,
};

#[taurpc::procedures(path = "projects", export_to = "../src/util/api/bindings.ts")]
pub trait ProjectManagementApi {
    async fn create_project<R: Runtime>(
        app_handle: AppHandle<R>,
        name: String,
        path: String,
    ) -> crate::MetaResult<String>;
    async fn open_local_project<R: Runtime>(
        app_handle: AppHandle<R>,
        path: String,
    ) -> crate::MetaResult<ProjectSettings>;
    async fn current_project<R: Runtime>(
        app_handle: AppHandle<R>,
    ) -> crate::MetaResult<Option<(ActiveProject, ProjectSettings)>>;
}

#[derive(Clone)]
pub struct ProjectManagementApiImpl;

#[taurpc::resolvers]
impl ProjectManagementApi for ProjectManagementApiImpl {
    async fn create_project<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        name: String,
        path: String,
    ) -> crate::MetaResult<String> {
        if name.contains("/") || name.contains("\\") {
            return Err(crate::Error::validation(name, "Name contains illegal characters").into());
        }
        if name.len() > 256 {
            return Err(crate::Error::validation(
                name,
                "Name is longer than the maximum of 256 characters",
            )
            .into());
        }
        let project_path = std::path::PathBuf::from(path.clone());
        let target_path = if project_path.exists() {
            if project_path.is_dir() {
                if project_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    == name.clone()
                    && project_path.read_dir()?.count() == 0
                {
                    Ok(project_path.clone())
                } else {
                    tokio::fs::create_dir_all(project_path.join(name.clone())).await?;
                    Ok(project_path.join(name.clone()))
                }
            } else {
                Err(MetaError::operation(
                    "project_path_not_dir",
                    format!("The selected path ({path}) is not a directory."),
                ))
            }
        } else {
            tokio::fs::create_dir_all(project_path.clone()).await?;
            Ok(project_path.clone())
        }?;

        ProjectSettings::new(name.clone()).save(target_path.clone())?;
        app_handle.clear_databases();
        app_handle
            .set_active_project(ActiveProject::Local {
                path: target_path.clone(),
            })
            .map_err(MetaError::from)?;
        Ok(name)
    }

    async fn open_local_project<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        path: String,
    ) -> crate::MetaResult<ProjectSettings> {
        let state = app_handle.set_active_project(ActiveProject::Local {
            path: PathBuf::from(path),
        })?;
        Ok(state.project_settings().unwrap())
    }
    async fn current_project<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
    ) -> crate::MetaResult<Option<(ActiveProject, ProjectSettings)>> {
        let state = app_handle.get_app_state();
        match state.active_project() {
            ActiveProject::None => Ok(None),
            ActiveProject::Local { path } => Ok(Some((
                ActiveProject::Local { path },
                state.project_settings().unwrap(),
            ))),
        }
    }
}
