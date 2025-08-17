use moka::future::Cache;
use tauri::{AppHandle, Runtime};
use taurpc::Router;

use crate::{api::{application::{ApplicationApi, ApplicationEventTrigger, ApplicationIconsApi}, templates::{TemplateApi, TemplateEventTrigger, NodeApi, NodeEventTrigger}}, SerializableError};

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
        .merge(application::ApplicationIconsApiImpl(Cache::new(10000)).into_handler())
        .merge(templates::TemplateApiImpl.into_handler())
        .merge(templates::NodeApiImpl.into_handler())
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

    pub fn templates(&self) -> TemplateEventTrigger<R> {
        TemplateEventTrigger::new(self.0.clone())
    }

    pub fn nodes(&self) -> NodeEventTrigger<R> {
        NodeEventTrigger::new(self.0.clone())
    }
}

pub type ApiResult<T> = Result<T, SerializableError>;
