use native_db::{ native_db, ToKey };
use native_model::native_model;
use serde::{ Deserialize, Serialize };
use specta::Type;
use native_model::Model;

use crate::templates::{ types::{PackageId, Parent}, Identifier, LayoutKind, NodeDesc, TemplateNode };

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[native_model(id = 2, version = 1, with = crate::models::MsgPack)]
#[native_db]
pub struct Template {
    #[primary_key]
    #[specta(optional = false, default = false)]
    pub id: Identifier,

    #[secondary_key(unique)]
    pub friendly_id: String,

    #[secondary_key]
    pub name: String,

    #[secondary_key]
    pub package: PackageId,

    #[secondary_key]
    pub inherit: Option<Identifier>,

    pub icon: Option<String>,
    pub description: Option<String>,
    pub layout: LayoutKind
}

impl Template {
    pub fn link_node(&self, node: NodeDesc) -> Node {
        Node::create(self.id.clone(), node)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[native_model(id = 3, version = 1, with = crate::models::MsgPack)]
#[native_db]
pub struct Node {
    #[primary_key]
    #[specta(optional = false, default = false)]
    pub id: Identifier,

    #[secondary_key]
    pub template: Identifier,

    #[secondary_key]
    pub parent: Parent,

    #[secondary_key]
    pub previous: Option<Identifier>,

    #[secondary_key]
    pub next: Option<Identifier>,

    pub node: NodeDesc
}

impl Node {
    pub fn create(template: Identifier, node: NodeDesc) -> Self {
        Self { id: node.id(), template, parent: node.parent(), previous: node.previous(), next: node.next(), node }
    }
}
