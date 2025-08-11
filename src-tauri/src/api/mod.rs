use tauri::{AppHandle, Runtime};
use taurpc::Router;

use crate::api::{application::{ApplicationApi, ApplicationEventTrigger, ApplicationIconsApi}, templates::TemplateApi};

pub mod application;
pub mod templates;

pub fn router<R: Runtime>() -> Router<R> {
    Router::new()
        .export_config(
            specta_typescript::Typescript
                ::default()
                .bigint(specta_typescript::BigIntExportBehavior::Number)
                .formatter(specta_typescript::formatter::prettier)
        )
        .merge(application::ApplicationApiImpl.into_handler())
        .merge(templates::TemplateApiImpl.into_handler())
        .merge(application::ApplicationIconsApiImpl.into_handler())
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
