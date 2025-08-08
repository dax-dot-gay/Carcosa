pub mod types;
pub mod fields;
pub mod containers;
pub mod other_nodes;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use specta::Type;
pub use types::{ FromValue, ValueType, NodeCategory, Identifier };

use crate::templates::{containers::ContainerNode, fields::FieldNode, other_nodes::OtherNode};

pub trait TemplateNode {
    fn id(&self) -> Identifier;
    fn node_kind(&self) -> String;
    fn node_category(&self) -> NodeCategory;
    fn parent(&self) -> Option<Identifier>;
}

pub trait TemplateContainer: TemplateNode {
    fn children(&self) -> Vec<Identifier>;
}

pub trait TemplateField: TemplateNode {
    fn key(&self) -> String;
    fn value_type(&self) -> ValueType;
    fn default_value(&self) -> Value;
    fn label(&self) -> Option<String> {
        None
    }
    fn icon(&self) -> Option<String> {
        None
    }
    fn help_text(&self) -> Option<String> {
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "node_category")]
pub enum Node {
    Other(OtherNode),
    Container(ContainerNode),
    Field(FieldNode)
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "layout")]
pub enum TemplateLayout {
    Document {
        header_nodes: Vec<Identifier>,
        root_node: Identifier
    },
    Form {
        ordered_children: Vec<Identifier>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Template {
    pub id: Identifier,
    pub friendly_id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub nodes: HashMap<String, Node>,
    pub layout: TemplateLayout
}
