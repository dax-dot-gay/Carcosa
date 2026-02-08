use crate::{types::ProjectSettings, MetaError};

#[taurpc::procedures(path = "projects", export_to = "../src/util/api/bindings.ts")]
pub trait ProjectManagementApi {
    async fn create_project(name: String, path: String) -> crate::MetaResult<String>;
}

#[derive(Clone)]
pub struct ProjectManagementApiImpl;

#[taurpc::resolvers]
impl ProjectManagementApi for ProjectManagementApiImpl {
    async fn create_project(self, name: String, path: String) -> crate::MetaResult<String> {
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
        Ok(name)
    }
}
