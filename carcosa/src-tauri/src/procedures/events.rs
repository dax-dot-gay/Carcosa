use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{Manager, Runtime};
use uuid::Uuid;

use crate::types::ActiveProject;

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum AppEvent {
    ActivatedProject { project: ActiveProject },
}

#[taurpc::procedures(event_trigger = AppEventTrigger)]
pub trait AppEventApi {
    #[taurpc(event)]
    async fn app_event(id: String, event: AppEvent);
}

#[derive(Clone)]
pub struct AppEventApiImpl;

#[taurpc::resolvers]
impl AppEventApi for AppEventApiImpl {}

pub trait AppEventExt<R: Runtime> {
    fn emit_event(&self, event: AppEvent) -> crate::Result<()>;
}

impl<R: Runtime, T: Manager<R>> AppEventExt<R> for T {
    fn emit_event(&self, event: AppEvent) -> crate::Result<()> {
        let trigger = AppEventTrigger::new(self.app_handle().clone());
        trigger.app_event(Uuid::now_v7().to_string(), event)?;
        Ok(())
    }
}
