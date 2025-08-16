use std::{ path::PathBuf, sync::Arc, time::Duration };
use native_db::{ Builder, Database };
use parking_lot::{ ArcRwLockUpgradableReadGuard, RawRwLock, RwLock };
use serde::de::DeserializeOwned;
use strum::IntoEnumIterator;
use tauri::{ AppHandle, Manager, Runtime };
use tauri_plugin_store::{ Store, StoreExt };

use crate::{ api::Events, models::MODELS, types::state::{ State, StateKey, StateValue } };

#[derive(Clone)]
pub struct Application<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Application<R> {
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
            .default("color_scheme", "dark".to_string())
            .default("sidebar_width", 300u64)
            .default("resource_manager_sidebar_width", 250u64)
            .build()
            .expect("Failed to retrieve application state store.")
    }

    pub fn set_state(&self, value: StateValue) -> crate::Result<()> {
        let val = value.value()?;
        self.app_state().set(value.key_name(), val);
        self.events().application().updated_state(self.full_state())?;
        Ok(())
    }

    pub fn get_state(&self, key: StateKey) -> crate::Result<Option<StateValue>> {
        if let Some(val) = self.app_state().get(key.key_name()) {
            Ok(Some(StateValue::wrap(key, val)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_state_as<V: DeserializeOwned>(&self, key: StateKey) -> crate::Result<Option<V>> {
        if let Some(value) = self.get_state(key)? { Ok(Some(value.resolve()?)) } else { Ok(None) }
    }

    pub fn get_state_or(&self, key: StateKey, fallback: StateValue) -> StateValue {
        self.get_state(key).unwrap_or(Some(fallback.clone())).unwrap_or(fallback)
    }

    pub fn get_state_as_or<V: DeserializeOwned>(&self, key: StateKey, fallback: V) -> V {
        if let Ok(opt_val) = self.get_state_as::<V>(key) {
            if let Some(val) = opt_val { val } else { fallback }
        } else {
            fallback
        }
    }

    pub fn full_state(&self) -> State {
        State::from_iter(StateKey::iter().filter_map(|key| self.get_state(key).unwrap_or(None)))
    }

    pub fn current_project_directory(&self) -> Option<PathBuf> {
        self.get_state_as_or(StateKey::CurrentProject, None)
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
