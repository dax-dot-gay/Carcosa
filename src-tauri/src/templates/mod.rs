pub mod types;
pub mod fields;
pub mod containers;
pub mod other_nodes;

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
#[serde(rename_all = "snake_case")]
pub enum PredefinedLayout {
    RichDocument,
    InteractableMap,
    Calendar,
    Timeline
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "layout")]
pub enum TemplateLayout {
    Predefined {
        header_root_children: Vec<Identifier>,
        layout_type: PredefinedLayout,
    },
    Form {
        inherit: Option<Identifier>,
        root_children: Vec<Identifier>
    }
}
