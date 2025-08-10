use std::collections::HashMap;

use native_db::{ native_db, ToKey };
use native_model::native_model;
use serde::{ Deserialize, Serialize };
use specta::Type;
use native_model::Model;

use crate::templates::{ Identifier, Node, TemplateLayout };

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[native_model(id = 2, version = 1)]
#[native_db]
pub struct Template {
    #[primary_key]
    #[serde(default)]
    pub id: Identifier,

    #[secondary_key]
    pub friendly_id: String,

    #[secondary_key]
    pub name: String,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub nodes: HashMap<String, Node>,
    pub layout: TemplateLayout,
}
