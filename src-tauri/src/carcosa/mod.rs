pub mod application;
pub mod templates;

use tauri::{ Manager, Runtime };

pub use application::Application;
pub use templates::{ Templates, TemplateInterface, Nodes, NodeInterface };

pub trait CarcosaExt<R: Runtime> {
    fn application(&self) -> Application<R>;
    fn templates(&self) -> Templates<R>;
    fn template_nodes(&self) -> Nodes<R>;
}

impl<R: Runtime, T: Manager<R>> CarcosaExt<R> for T {
    fn application(&self) -> Application<R> {
        Application::new(self.app_handle().clone())
    }
    fn templates(&self) -> Templates<R> {
        Templates::new(self.app_handle().clone())
    }
    fn template_nodes(&self) -> Nodes<R> {
        self.templates().nodes()
    }
}
