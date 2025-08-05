use tauri::Runtime;
use taurpc::Router;

use crate::api::application::ApplicationApi;

pub mod application;

pub fn router<R: Runtime>() -> Router<R> {
    Router::new().merge(application::ApplicationApiImpl.into_handler())
}
