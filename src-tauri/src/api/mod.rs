use tauri::Runtime;
use taurpc::Router;

use crate::api::application::ApplicationApi;

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
