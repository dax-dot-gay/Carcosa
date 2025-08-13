use std::collections::HashMap;

use native_db::{ native_db, ToKey };
use native_model::native_model;
use serde::{ Deserialize, Serialize };
use specta::Type;
use native_model::Model;

use crate::templates::{ types::PackageId, Identifier, Node, LayoutKind };

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[native_model(id = 2, version = 1, with = crate::models::MsgPack)]
#[native_db]
pub struct Template {
    #[primary_key]
    #[specta(optional = false, default = false)]
    pub id: Identifier,

    #[secondary_key(unique)]
    pub friendly_id: String,

    #[secondary_key(unique)]
    pub name: String,

    #[secondary_key]
    pub package: PackageId,

    #[secondary_key]
    pub inherit: Option<Identifier>,

    pub icon: Option<String>,
    pub description: Option<String>,
    pub nodes: HashMap<String, Node>,
    pub root_children: Vec<String>,
    pub layout: LayoutKind
}
