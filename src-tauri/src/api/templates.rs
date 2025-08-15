use convert_case::{ Case, Casing };
use serde::{ Deserialize, Serialize };
use specta::Type;
use tauri::{ AppHandle, Runtime };

use crate::{
    api::ApiResult,
    carcosa::CarcosaExt,
    models::{ keys::TemplateKey, Node, Template },
    templates::{
        types::{ PackageId, TemplateMetadata },
        Identifier,
        LayoutKind, NodeDesc
    },
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
    ) -> ApiResult<Vec<TemplateMetadata>>;
    async fn package_templates<R: Runtime>(
        app_handle: AppHandle<R>,
        pkg: PackageId
    ) -> ApiResult<Vec<TemplateMetadata>>;
    async fn create_template<R: Runtime>(
        app_handle: AppHandle<R>,
        model: CreateTemplateModel
    ) -> ApiResult<Template>;
    async fn create_node<R: Runtime>(app_handle: AppHandle<R>, template: String, node: NodeDesc) -> ApiResult<Node>;

    #[taurpc(event)]
    async fn created_template(template: TemplateMetadata);
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
        let db = app_handle.carcosa().current_database()?;
        let txn = db.r_transaction()?;
        match txn.get().primary(Identifier::from(id)) {
            Ok(r) => Ok(r),
            Err(e) => {
                println!("{e:?}");
                Err(e)?
            }
        }
    }
    async fn get_template_by_friendly_id<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        id: String
    ) -> ApiResult<Option<Template>> {
        let db = app_handle.carcosa().current_database()?;
        let txn = db.r_transaction()?;
        Ok(txn.get().secondary(TemplateKey::friendly_id, id)?)
    }
    async fn create_template<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        model: CreateTemplateModel
    ) -> ApiResult<Template> {
        let CreateTemplateModel { icon, name, description, layout, inherit } = model;
        let friendly_id: String = name
            .clone()
            .to_case(Case::Snake)
            .chars()
            .filter_map(|c| (
                if c.is_alphanumeric() || c == '_' || c == '.' || c == '-' {
                    Some(c)
                } else {
                    None
                }
            ))
            .collect();
        let new_template = Template {
            id: Identifier::new(),
            friendly_id,
            name,
            package: PackageId::project(),
            icon,
            description,
            layout,
            inherit
        };
        let db = app_handle.carcosa().current_database()?;
        let txn = db.rw_transaction()?;
        txn.insert(new_template.clone())?;
        txn.commit()?;

        app_handle.carcosa().events().templates().created_template(new_template.clone().into())?;
        Ok(new_template)
    }

    async fn all_templates<R: Runtime>(
        self,
        app_handle: AppHandle<R>
    ) -> ApiResult<Vec<TemplateMetadata>> {
        let db = app_handle.carcosa().current_database()?;
        let txn = db.r_transaction()?;
        let results: Vec<Template> = txn
            .scan()
            .primary()?
            .all()?
            .filter_map(|r| if let Ok(v) = r { Some(v) } else { None })
            .collect();

        Ok(
            results
                .into_iter()
                .map(|v| TemplateMetadata::from(v))
                .collect()
        )
    }
    async fn package_templates<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        pkg: PackageId
    ) -> ApiResult<Vec<TemplateMetadata>> {
        let db = app_handle.carcosa().current_database()?;
        let txn = db.r_transaction()?;
        let results: Vec<Template> = txn
            .scan()
            .secondary(TemplateKey::package)?
            .range(pkg.clone()..=pkg.clone())?
            .filter_map(|r| if let Ok(v) = r { Some(v) } else { None })
            .collect();

        Ok(
            results
                .into_iter()
                .map(|v| TemplateMetadata::from(v))
                .collect()
        )
    }

    async fn create_node<R: Runtime>(self, app_handle: AppHandle<R>, template: String, node: NodeDesc) -> ApiResult<Node> {
        Err(crate::SerializableError::NoActiveProject)
    }
}
