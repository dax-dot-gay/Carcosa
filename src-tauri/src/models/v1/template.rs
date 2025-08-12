use std::collections::HashMap;

use native_db::{ native_db, ToKey };
use native_model::native_model;
use serde::{ Deserialize, Serialize };
use specta::Type;
use native_model::Model;

use crate::templates::{ types::PackageId, Identifier, Node, TemplateLayout };

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[native_model(id = 2, version = 1, with = crate::models::MsgPack)]
#[native_db]
pub struct Template {
    #[primary_key]
    #[serde(default)]
    pub id: Identifier,

    #[secondary_key(unique)]
    pub friendly_id: String,

    #[secondary_key(unique)]
    pub name: String,

    #[secondary_key]
    pub package: PackageId,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub nodes: HashMap<String, Node>,
    pub layout: TemplateLayout,
}
