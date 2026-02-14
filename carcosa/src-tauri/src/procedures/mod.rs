use tauri::{Runtime, ipc::Invoke};
use taurpc::Router;

use crate::procedures::{events::AppEventApi, project_management::ProjectManagementApi};

pub mod project_management;
pub mod events;
pub use events::{AppEvent, AppEventExt};

pub fn handler<R: Runtime>() -> impl Fn(Invoke<R>) -> bool {
    let router = Router::<R>::new()
        .merge(project_management::ProjectManagementApiImpl.into_handler())
        .merge(events::AppEventApiImpl.into_handler());
    router.into_handler()
}