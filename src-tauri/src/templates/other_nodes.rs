use serde::{ Deserialize, Serialize };
use specta::Type;
use spire_enum::prelude::{delegate_impl, delegated_enum};

use crate::templates::{ types::Parent, Identifier, TemplateNode };

macro_rules! impl_node {
    ($node:ty, $kind:literal) => {
        impl TemplateNode for $node {
            fn id(&self) -> Identifier {
                self.id.clone()
            }

            fn node_kind(&self) -> String {
                String::from($kind)
            }

            fn node_category(&self) -> super::NodeCategory {
                super::NodeCategory::Container
            }

            fn parent(&self) -> Parent {
                self.parent.clone()
            }

            fn next(&self) -> Option<Identifier> {
                self.next.clone()
            }

            fn previous(&self) -> Option<Identifier> {
                self.previous.clone()
            }
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Text {
    pub id: Identifier,
    pub parent: Parent,
    pub previous: Option<Identifier>,
    pub next: Option<Identifier>,
    pub content: String,
}

impl_node!(Text, "text");

#[derive(Serialize, Deserialize, Clone, Debug, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum AlertLevel {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Alert {
    pub id: Identifier,

    pub parent: Parent,
    pub previous: Option<Identifier>,
    pub next: Option<Identifier>,
    pub content: String,

    #[serde(default)]
    pub level: AlertLevel,

    #[serde(default)]
    pub title: Option<String>,
}

impl_node!(Alert, "alert");

#[delegated_enum(
    impl_conversions
)]
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "node_kind")]
pub enum OtherNode {
    Text(Text),
    Alert(Alert),
}

#[delegate_impl]
impl TemplateNode for OtherNode {
    fn id(&self) -> Identifier;
    fn node_kind(&self) -> String;
    fn node_category(&self) -> super::NodeCategory;
    fn parent(&self) -> Parent;
    fn previous(&self) -> Option<Identifier>;
    fn next(&self) -> Option<Identifier>;
}
