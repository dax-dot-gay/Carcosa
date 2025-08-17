use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use specta::Type;
use tauri::{ AppHandle, Runtime };

use crate::{
    api::ApiResult,
    carcosa::CarcosaExt,
    models::{ Node, Template },
    templates::{ types::{ PackageId, Parent }, Identifier, LayoutKind, NodeDesc, TemplateNode },
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

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case")]
pub enum NodePlacement {
    After(Identifier),
    Before(Identifier),
    Into(Parent),
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
    async fn all_templates<R: Runtime>(app_handle: AppHandle<R>) -> ApiResult<Vec<Template>>;
    async fn package_templates<R: Runtime>(
        app_handle: AppHandle<R>,
        pkg: PackageId
    ) -> ApiResult<Vec<Template>>;
    async fn create_template<R: Runtime>(
        app_handle: AppHandle<R>,
        model: CreateTemplateModel
    ) -> ApiResult<Template>;

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

    async fn all_templates<R: Runtime>(self, app_handle: AppHandle<R>) -> ApiResult<Vec<Template>> {
        Ok(
            app_handle
                .templates()
                .all()?
                .into_iter()
                .map(|t| t.into())
                .collect()
        )
    }
    async fn package_templates<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        pkg: PackageId
    ) -> ApiResult<Vec<Template>> {
        Ok(
            app_handle
                .templates()
                .all_in_package(pkg)?
                .into_iter()
                .map(|t| t.into())
                .collect()
        )
    }
}

#[taurpc::procedures(path = "templates.nodes", event_trigger = NodeEventTrigger)]
pub trait NodeApi {
    #[taurpc(event)]
    async fn node_created(template: Identifier, node: Node);

    #[taurpc(event)]
    async fn node_updated(template: Identifier, node: Node);

    #[taurpc(event)]
    async fn node_moved(template: Identifier, node: Node);

    #[taurpc(event)]
    async fn node_removed(template: Identifier, id: Identifier);

    async fn create_node<R: Runtime>(
        app_handle: AppHandle<R>,
        template: Identifier,
        placement: NodePlacement,
        node: NodeDesc
    ) -> ApiResult<Node>;

    async fn update_node<R: Runtime>(
        app_handle: AppHandle<R>,
        id: Identifier,
        node: NodeDesc
    ) -> ApiResult<Node>;
    async fn delete_node<R: Runtime>(app_handle: AppHandle<R>, id: Identifier) -> ApiResult<()>;
    async fn move_node<R: Runtime>(
        app_handle: AppHandle<R>,
        id: Identifier,
        placement: NodePlacement
    ) -> ApiResult<Node>;

    async fn get_nodes<R: Runtime>(
        app_handle: AppHandle<R>,
        template: Identifier
    ) -> ApiResult<HashMap<Identifier, Node>>;
    async fn get_children<R: Runtime>(
        app_handle: AppHandle<R>,
        template: Identifier,
        parent: Parent
    ) -> ApiResult<Vec<Node>>;
}

#[derive(Clone, Debug)]
pub struct NodeApiImpl;

#[taurpc::resolvers]
impl NodeApi for NodeApiImpl {
    async fn create_node<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        template: Identifier,
        placement: NodePlacement,
        node: NodeDesc
    ) -> ApiResult<Node> {
        let template = app_handle
            .templates()
            .get_by_id(template.clone())?
            .ok_or(crate::Error::not_found("template", template.clone()))?;
        let created = (match placement {
            NodePlacement::After(identifier) => template.create_node_after(node, identifier),
            NodePlacement::Before(identifier) => template.create_node_before(node, identifier),
            NodePlacement::Into(parent) => template.create_node_into(node, parent),
        })?.node();

        app_handle.events().nodes().node_created(created.template.clone(), created.clone())?;
        Ok(created)
    }

    async fn update_node<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        id: Identifier,
        node: NodeDesc
    ) -> ApiResult<Node> {
        let mut current = app_handle
            .template_nodes()
            .get(id.clone())?
            .ok_or(crate::Error::not_found("node", id.clone()))?;
        if node.node_category() != current.desc().node_category() {
            return Err(crate::Error::immutable("node_category").into());
        }

        if node.node_kind() != current.desc().node_kind() {
            return Err(crate::Error::immutable("node_kind").into());
        }

        current.update(node)?;
        let result = current.node();
        app_handle.events().nodes().node_updated(result.template.clone(), result.clone())?;
        Ok(result)
    }
    async fn delete_node<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        id: Identifier
    ) -> ApiResult<()> {
        let current = app_handle
            .template_nodes()
            .get(id.clone())?
            .ok_or(crate::Error::not_found("node", id.clone()))?;
        let template_id = current.node().template;
        current.delete()?;
        app_handle.events().nodes().node_removed(template_id, id)?;
        Ok(())
    }
    async fn move_node<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        id: Identifier,
        placement: NodePlacement
    ) -> ApiResult<Node> {
        let mut current = app_handle
            .template_nodes()
            .get(id.clone())?
            .ok_or(crate::Error::not_found("node", id.clone()))?;
        (match placement {
            NodePlacement::After(identifier) => current.move_after(identifier),
            NodePlacement::Before(identifier) => current.move_before(identifier),
            NodePlacement::Into(parent) => current.move_into(parent),
        })?;
        app_handle.events().nodes().node_moved(current.node().template, current.node())?;
        Ok(current.node())
    }

    async fn get_nodes<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        template: Identifier
    ) -> ApiResult<HashMap<Identifier, Node>> {
        Ok(
            app_handle
                .template_nodes()
                .in_template(template)?
                .into_iter()
                .map(|(key, val)| (key, Into::<Node>::into(val)))
                .collect()
        )
    }

    async fn get_children<R: Runtime>(
        self,
        app_handle: AppHandle<R>,
        template: Identifier,
        parent: Parent
    ) -> ApiResult<Vec<Node>> {
        Ok(
            app_handle
                .template_nodes()
                .get_children(template, parent)?
                .into_iter()
                .map(|v| Into::<Node>::into(v))
                .collect()
        )
    }
}
