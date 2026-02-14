use tauri::{Manager, Runtime};

use crate::types::{ActiveProject, ApplicationState, ApplicationStateWrapper, ProjectSettings};

pub trait ApplicationExt<R: Runtime> {
    fn get_app_state(&self) -> ApplicationState;
    fn update_app_state(
        &self,
        updater: impl FnOnce(ApplicationState) -> crate::Result<ApplicationState>,
    ) -> crate::Result<ApplicationState>;
    fn set_active_project(&self, project: ActiveProject) -> crate::Result<ApplicationState>;
}

impl<R: Runtime, T: Manager<R>> ApplicationExt<R> for T {
    fn get_app_state(&self) -> ApplicationState {
        self.state::<ApplicationStateWrapper>().read().clone()
    }

    fn update_app_state(
        &self,
        updater: impl FnOnce(ApplicationState) -> crate::Result<ApplicationState>,
    ) -> crate::Result<ApplicationState> {
        let state_obj = self.state::<ApplicationStateWrapper>();
        let mut current_state = state_obj.write();
        let updated_state = updater(current_state.clone())?;
        *current_state = updated_state.clone();
        Ok(updated_state)
    }

    fn set_active_project(&self, project: ActiveProject) -> crate::Result<ApplicationState> {
        match project {
            ActiveProject::None => self.update_app_state(|state| {
                Ok(state
                    .with_active_project(ActiveProject::None)
                    .with_project_settings(None))
            }),
            ActiveProject::Local { path } => self.update_app_state(|state| {
                let project_settings = ProjectSettings::load(path.clone())?;
                Ok(state
                    .with_active_project(ActiveProject::Local { path })
                    .with_project_settings(Some(project_settings)))
            }),
        }
    }
}
