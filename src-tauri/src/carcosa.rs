use std::{ path::PathBuf, sync::Arc, time::Duration };
use native_db::{ Builder, Database };
use parking_lot::{ ArcRwLockUpgradableReadGuard, RawRwLock, RwLock };
use serde::{ de::DeserializeOwned, Serialize };
use serde_json::{ from_value, to_value };
use tauri::{ AppHandle, Manager, Runtime };
use tauri_plugin_store::{ Store, StoreExt };

use crate::{api::Events, models::MODELS};

pub trait CarcosaExt<R: Runtime> {
    fn carcosa(&self) -> Carcosa<R>;
}

#[derive(Clone)]
pub struct Carcosa<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Carcosa<R> {
    pub fn new(handle: AppHandle<R>) -> Self {
        Self(handle)
    }

    pub fn app_handle(&self) -> &AppHandle<R> {
        &self.0
    }

    pub fn app_state(&self) -> Arc<Store<R>> {
        self.app_handle()
            .store_builder("state.json")
            .auto_save(Duration::from_secs_f64(0.2))
            .default("current_project", None::<String>)
            .build()
            .expect("Failed to retrieve application state store.")
    }

    pub fn set_state<V: Serialize>(&self, key: impl Into<String>, value: V) -> crate::Result<()> {
        let value = to_value(value)?;
        self.app_state().set(key, value);
        Ok(())
    }

    pub fn get_state<V: DeserializeOwned>(
        &self,
        key: impl Into<String>
    ) -> crate::Result<Option<V>> {
        if let Some(val) = self.app_state().get(key.into()) {
            Ok(Some(from_value::<V>(val)?))
        } else {
            Ok(None)
        }
    }

    pub fn current_project_directory(&self) -> Option<PathBuf> {
        self.get_state::<Option<PathBuf>>("current_project").unwrap_or(None).unwrap_or(None)
    }

    pub fn current_database(
        &self
    ) -> crate::Result<ArcRwLockUpgradableReadGuard<RawRwLock, Database<'static>>> {
        let active_db = self.app_handle().state::<RwLock<Option<Arc<RwLock<Database<'static>>>>>>();
        let mut internal_option = active_db.upgradable_read();
        if let Some(path) = self.current_project_directory() {
            Ok(
                (
                    if internal_option.is_none() {
                        internal_option.with_upgraded(|opt| {
                            let created = opt.insert(
                                Arc::new(
                                    RwLock::new(
                                        Builder::new().create(&MODELS, path.join("project.db"))?
                                    )
                                )
                            );
                            Ok::<_, crate::Error>(created.clone())
                        })?
                    } else {
                        internal_option.clone().unwrap()
                    }
                ).upgradable_read_arc()
            )
        } else {
            if internal_option.is_some() {
                let _ = internal_option.with_upgraded(|opt| {
                    let _ = opt.take();
                });
            }

            Err(crate::Error::NoActiveProject)
        }
    }

    pub fn events(&self) -> Events<R> {
        Events::new(self.app_handle().clone())
    }
}

impl<R: Runtime, T: Manager<R>> CarcosaExt<R> for T {
    fn carcosa(&self) -> Carcosa<R> {
        Carcosa::new(self.app_handle().clone())
    }
}
