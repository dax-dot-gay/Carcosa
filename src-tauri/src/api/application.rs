use std::{collections::HashMap, io::Read, path::PathBuf};
use convert_case::{Case, Casing};
use tauri::{Manager, Runtime};
use tokio::fs;

use crate::{
    carcosa::CarcosaExt,
    models::ProjectConfiguration,
    types::state::{ State, StateKey, StateValue }, SerializableError,
};

#[taurpc::ipc_type]
pub struct CreateProjectModel {
    pub path: String,
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,
}

#[taurpc::procedures(path = "application", event_trigger = ApplicationEventTrigger)]
pub trait ApplicationApi {
    async fn create_project<R: Runtime>(
        app_handle: tauri::AppHandle<R>,
        project: CreateProjectModel
    ) -> Result<ProjectConfiguration, crate::SerializableError>;
    async fn open_project<R: Runtime>(
        app_handle: tauri::AppHandle<R>,
        path: String
    ) -> Result<ProjectConfiguration, crate::SerializableError>;
    async fn exit_project<R: Runtime>(
        app_handle: tauri::AppHandle<R>
    ) -> Result<(), crate::SerializableError>;
    async fn project_config<R: Runtime>(
        app_handle: tauri::AppHandle<R>
    ) -> Result<ProjectConfiguration, crate::SerializableError>;
    async fn project_directory<R: Runtime>(app_handle: tauri::AppHandle<R>) -> Option<String>;
    async fn get_state<R: Runtime>(
        app_handle: tauri::AppHandle<R>,
        key: StateKey
    ) -> Result<Option<StateValue>, crate::SerializableError>;
    async fn set_state<R: Runtime>(
        app_handle: tauri::AppHandle<R>,
        value: StateValue
    ) -> Result<(), crate::SerializableError>;
    async fn full_state<R: Runtime>(app_handle: tauri::AppHandle<R>) -> State;

    #[taurpc(event)]
    async fn opened_project(path: String, config: ProjectConfiguration);

    #[taurpc(event)]
    async fn closed_project();

    #[taurpc(event)]
    async fn updated_state(new_state: State);
}

#[derive(Clone)]
pub struct ApplicationApiImpl;

#[taurpc::resolvers]
impl ApplicationApi for ApplicationApiImpl {
    async fn create_project<R: Runtime>(
        self,
        app_handle: tauri::AppHandle<R>,
        project: CreateProjectModel
    ) -> Result<ProjectConfiguration, crate::SerializableError> {
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
        carcosa.set_state(StateValue::CurrentProject(Some(project.path.clone())))?;
        let database = carcosa.current_database()?;
        let txn = database.rw_transaction()?;
        let config = ProjectConfiguration::new(project.name.clone(), project.description.clone());
        txn.insert(config.clone())?;
        txn.commit()?;
        app_handle
            .carcosa()
            .events()
            .application()
            .opened_project(project.path.clone(), config.clone())?;

        Ok(config)
    }

    async fn open_project<R: Runtime>(
        self,
        app_handle: tauri::AppHandle<R>,
        path: String
    ) -> Result<ProjectConfiguration, crate::SerializableError> {
        let realpath = PathBuf::from(path.clone());
        let carcosa = app_handle.carcosa();
        if !realpath.is_dir() {
            return Err(crate::Error::InvalidProjectSelection(path).into());
        }

        if !realpath.join("project.db").exists() {
            return Err(crate::Error::InvalidProjectSelection(path).into());
        }

        carcosa.set_state(StateValue::CurrentProject(Some(path.clone())))?;
        let db = carcosa.current_database()?;
        let txn = db.r_transaction()?;
        if let Some(config) = txn.get().primary::<ProjectConfiguration>("CONFIG")? {
            app_handle
                .carcosa()
                .events()
                .application()
                .opened_project(path.clone(), config.clone())?;
            Ok(config)
        } else {
            Err(
                crate::Error
                    ::CorruptedProject(path, "Missing project configuration in database.".into())
                    .into()
            )
        }
    }

    async fn exit_project<R: Runtime>(
        self,
        app_handle: tauri::AppHandle<R>
    ) -> Result<(), crate::SerializableError> {
        app_handle.carcosa().set_state(StateValue::CurrentProject(None))?;
        app_handle.carcosa().events().application().closed_project()?;
        Ok(())
    }

    async fn project_config<R: Runtime>(
        self,
        app_handle: tauri::AppHandle<R>
    ) -> Result<ProjectConfiguration, crate::SerializableError> {
        let db = app_handle.carcosa().current_database()?;
        let txn = db.r_transaction()?;
        if let Some(config) = txn.get().primary::<ProjectConfiguration>("CONFIG")? {
            Ok(config)
        } else {
            Err(
                crate::Error
                    ::CorruptedProject(
                        app_handle
                            .carcosa()
                            .current_project_directory()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        "Missing project configuration in database.".into()
                    )
                    .into()
            )
        }
    }

    async fn project_directory<R: Runtime>(
        self,
        app_handle: tauri::AppHandle<R>
    ) -> Option<String> {
        app_handle
            .carcosa()
            .current_project_directory()
            .and_then(|v| Some(v.to_string_lossy().to_string()))
    }

    async fn get_state<R: Runtime>(
        self,
        app_handle: tauri::AppHandle<R>,
        key: StateKey
    ) -> Result<Option<StateValue>, crate::SerializableError> {
        Ok(app_handle.carcosa().get_state(key)?)
    }
    async fn set_state<R: Runtime>(
        self,
        app_handle: tauri::AppHandle<R>,
        value: StateValue
    ) -> Result<(), crate::SerializableError> {
        let _ = app_handle.carcosa().set_state(value)?;
        app_handle.carcosa().events().application().updated_state(app_handle.carcosa().full_state())?;
        Ok(())
    }

    async fn full_state<R: Runtime>(self, app_handle: tauri::AppHandle<R>) -> State {
        app_handle.carcosa().full_state()
    }
}

#[taurpc::procedures(path = "application.icons")]
pub trait ApplicationIconsApi {
    async fn icon_categories<R: Runtime>(app_handle: tauri::AppHandle<R>) -> Result<Vec<String>, SerializableError>;
    async fn icons_in_category<R: Runtime>(app_handle: tauri::AppHandle<R>, category: String) -> Result<Option<Vec<String>>, SerializableError>;
    async fn icon<R: Runtime>(app_handle: tauri::AppHandle<R>, icon: String) -> Result<Option<String>, SerializableError>;
    async fn icons<R: Runtime>(app_handle: tauri::AppHandle<R>, icons: Vec<String>) -> Result<HashMap<String, Option<String>>, SerializableError>;
}

#[derive(Clone)]
pub struct ApplicationIconsApiImpl;

#[taurpc::resolvers]
impl ApplicationIconsApi for ApplicationIconsApiImpl {
    async fn icon_categories<R: Runtime>(self, app_handle: tauri::AppHandle<R>) -> Result<Vec<String>, SerializableError> {
        let icon_info_path = app_handle.path().resolve("resources/icons.json", tauri::path::BaseDirectory::Resource)?;
        let json_data: HashMap<String, Vec<String>> = serde_json::from_str(&fs::read_to_string(icon_info_path).await?)?;

        Ok(json_data.into_keys().collect())
    }
    async fn icons_in_category<R: Runtime>(self, app_handle: tauri::AppHandle<R>, category: String) -> Result<Option<Vec<String>>, SerializableError> {
        let icon_info_path = app_handle.path().resolve("resources/icons.json", tauri::path::BaseDirectory::Resource)?;
        let json_data: HashMap<String, Vec<String>> = serde_json::from_str(&fs::read_to_string(icon_info_path).await?)?;

        Ok(json_data.get(&category.to_case(Case::Snake)).and_then(|v| Some(v.clone())))
    }
    async fn icon<R: Runtime>(self, app_handle: tauri::AppHandle<R>, icon: String) -> Result<Option<String>, SerializableError> {
        let icon_data_path = app_handle.path().resolve("resources/icons.zip", tauri::path::BaseDirectory::Resource)?;
        let file = fs::File::open(icon_data_path).await?.into_std().await;
        let mut archive = zip::ZipArchive::new(file)?;
        let extract_result = archive.by_name(&format!("{}.json", icon.to_case(Case::Snake)));
        if let Ok(mut found) = extract_result {
            let mut output = String::new();
            found.read_to_string(&mut output)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
    async fn icons<R: Runtime>(self, app_handle: tauri::AppHandle<R>, icons: Vec<String>) -> Result<HashMap<String, Option<String>>, SerializableError> {
        let icon_data_path = app_handle.path().resolve("resources/icons.zip", tauri::path::BaseDirectory::Resource)?;
        let file = fs::File::open(icon_data_path).await?.into_std().await;
        let mut archive = zip::ZipArchive::new(file)?;
        let mut mapping = HashMap::new();
        for icon in icons {
            let extract_result = archive.by_name(&format!("{}.json", icon.clone().to_case(Case::Snake)));
            if let Ok(mut found) = extract_result {
                let mut output = String::new();
                found.read_to_string(&mut output)?;
                let _ = mapping.insert(icon, Some(output));
            } else {
                let _ = mapping.insert(icon, None);
            }
        }

        Ok(mapping)
    }
}