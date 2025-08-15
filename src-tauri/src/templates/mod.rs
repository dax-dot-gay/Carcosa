pub mod types;
pub mod fields;
pub mod containers;
pub mod other_nodes;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use specta::Type;
use spire_enum::prelude::{delegate_impl, delegated_enum};
pub use types::{ FromValue, ValueType, NodeCategory, Identifier };

use crate::templates::{containers::ContainerNode, fields::FieldNode, other_nodes::OtherNode, types::Parent};

pub trait TemplateNode {
    fn id(&self) -> Identifier;
    fn node_kind(&self) -> String;
    fn node_category(&self) -> NodeCategory;
    fn parent(&self) -> Parent;
    fn previous(&self) -> Option<Identifier>;
    fn next(&self) -> Option<Identifier>;
}

pub trait TemplateContainer: TemplateNode {
    fn collections(&self) -> Vec<String>;
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

#[delegated_enum(
    impl_conversions
)]
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "node_category")]
pub enum NodeDesc {
    Other(OtherNode),
    Container(ContainerNode),
    Field(FieldNode)
}

#[delegate_impl]
impl TemplateNode for NodeDesc {
    fn id(&self) -> Identifier;
    fn node_kind(&self) -> String;
    fn node_category(&self) -> NodeCategory;
    fn parent(&self) -> Parent;
    fn previous(&self) -> Option<Identifier>;
    fn next(&self) -> Option<Identifier>;
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case")]
pub enum LayoutKind {
    Form,
    RichDocument,
    InteractableMap,
    Calendar,
    Timeline
}
