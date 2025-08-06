use tauri::{AppHandle, Runtime};
use taurpc::Router;

use crate::api::application::{ApplicationApi, ApplicationEventTrigger};

pub mod application;

pub fn router<R: Runtime>() -> Router<R> {
    Router::new()
        .export_config(
            specta_typescript::Typescript
                ::default()
                .header("// My header\n\n")
                // Make sure prettier is installed before using this.
                // .formatter(specta_typescript::formatter::prettier)
                .bigint(specta_typescript::BigIntExportBehavior::String)
        )
        .merge(application::ApplicationApiImpl.into_handler())
}

#[derive(Clone, Debug)]
pub struct Events<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Events<R> {
    pub fn new(handle: AppHandle<R>) -> Self {
        Self(handle)
    }

    pub fn application(&self) -> ApplicationEventTrigger<R> {
        ApplicationEventTrigger::new(self.0.clone())
    }
}
