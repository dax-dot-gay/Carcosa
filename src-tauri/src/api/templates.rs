use serde::{ Deserialize, Serialize };
use specta::Type;
use tauri::{ AppHandle, Runtime };

use crate::{
    api::ApiResult,
    carcosa::CarcosaExt,
    models::{ Node, Template },
    templates::{ types::PackageId, Identifier, LayoutKind, NodeDesc },
};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct CreateTemplateModel {
    pub icon: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub layout: LayoutKind,

    #[serde(default)]
    pub inherit: Option<Identifier>,
}

#[taurpc::procedures(path = "templates", event_trigger = TemplateEventTrigger)]
pub trait TemplateApi {
    async fn get_template_by_uuid<R: Runtime>(
        app_handle: AppHandle<R>,
        id: String
    ) -> ApiResult<Option<Template>>;
    async fn get_template_by_friendly_id<R: Runtime>(
        app_handle: AppHandle<R>,
        id: String
    ) -> ApiResult<Option<Template>>;
    async fn all_templates<R: Runtime>(
        app_handle: AppHandle<R>
    ) -> ApiResult<Vec<Template>>;
    async fn package_templates<R: Runtime>(
        app_handle: AppHandle<R>,
        pkg: PackageId
    ) -> ApiResult<Vec<Template>>;
    async fn create_template<R: Runtime>(
        app_handle: AppHandle<R>,
        model: CreateTemplateModel
    ) -> ApiResult<Template>;
    async fn create_node<R: Runtime>(
        app_handle: AppHandle<R>,
        template: String,
        node: NodeDesc
    ) -> ApiResult<Node>;

    #[taurpc(event)]
    async fn created_template(template: Template);
}

#[derive(Clone)]
pub struct TemplateApiImpl;

#[taurpc::resolvers]
impl TemplateApi for TemplateApiImpl {
    async fn get_template_by_uuid<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        id: String
    ) -> ApiResult<Option<Template>> {
        Ok(
            app_handle
                .templates()
                .get_by_id(id)?
                .and_then(|t| Some(t.into()))
        )
    }
    async fn get_template_by_friendly_id<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        id: String
    ) -> ApiResult<Option<Template>> {
        Ok(
            app_handle
                .templates()
                .get_by_friendly_id(id)?
                .and_then(|t| Some(t.into()))
        )
    }
    async fn create_template<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        model: CreateTemplateModel
    ) -> ApiResult<Template> {
        let CreateTemplateModel { icon, name, description, layout, inherit } = model;
        let new_template: Template = app_handle
            .templates()
            .create()
            .name(name)
            .maybe_icon(icon)
            .maybe_description(description)
            .layout(layout)
            .maybe_inherit(inherit)
            .call()?
            .into();

        app_handle
            .application()
            .events()
            .templates()
            .created_template(new_template.clone().into())?;
        Ok(new_template)
    }

    async fn all_templates<R: Runtime>(
        self,
        app_handle: AppHandle<R>
    ) -> ApiResult<Vec<Template>> {
        Ok(app_handle.templates().all()?.into_iter().map(|t| t.into()).collect())
    }
    async fn package_templates<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        pkg: PackageId
    ) -> ApiResult<Vec<Template>> {
        Ok(app_handle.templates().all_in_package(pkg)?.into_iter().map(|t| t.into()).collect())
    }

    async fn create_node<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        template: String,
        node: NodeDesc
    ) -> ApiResult<Node> {
        Err(crate::SerializableError::NoActiveProject)
    }
}
